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
#[doc = r"Pmn Pin Function Control Register"]
unsafe impl ::core::marker::Send for super::Pfs {}
unsafe impl ::core::marker::Sync for super::Pfs {}
impl super::Pfs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P000Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P000PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P000PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }
    #[inline(always)]
    pub const fn p008pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p009pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }
    #[inline(always)]
    pub const fn p008pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p009pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }
    #[inline(always)]
    pub const fn p008pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p009pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }

    #[doc = "P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x28usize))
        }
    }
    #[inline(always)]
    pub const fn p010pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p011pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p012pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p013pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p014pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p015pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }

    #[doc = "P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x28usize))
        }
    }
    #[inline(always)]
    pub const fn p010pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p011pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p012pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p013pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p014pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p015pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }

    #[doc = "P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x28usize))
        }
    }
    #[inline(always)]
    pub const fn p010pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p011pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p012pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p013pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p014pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p015pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }

    #[doc = "P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P100Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P100Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P100PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P100PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P100PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P100PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x44usize))
        }
    }
    #[inline(always)]
    pub const fn p101pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p102pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p103pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p104pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p105pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p106pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p107pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x44usize))
        }
    }
    #[inline(always)]
    pub const fn p101pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p102pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p103pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p104pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p105pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p106pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p107pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x44usize))
        }
    }
    #[inline(always)]
    pub const fn p101pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p102pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p103pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p104pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p105pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p106pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p107pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }

    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P108Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P108PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P108PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P109Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P109PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P109PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P110Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P110PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P110PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6cusize))
        }
    }
    #[inline(always)]
    pub const fn p111pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p112pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p113pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p114pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p115pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6cusize))
        }
    }
    #[inline(always)]
    pub const fn p111pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p112pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p113pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p114pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p115pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6cusize))
        }
    }
    #[inline(always)]
    pub const fn p111pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p112pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p113pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p114pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p115pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }

    #[doc = "P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P200Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P200PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P200PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P201Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P201PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P201PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }
    #[inline(always)]
    pub const fn p202pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p203pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p204pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p205pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p206pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p207pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p208pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p209pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }
    #[inline(always)]
    pub const fn p202pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p203pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p204pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p205pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p206pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p207pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p208pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p209pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }
    #[inline(always)]
    pub const fn p202pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p203pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p204pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p205pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p206pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p207pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p208pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p209pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa8usize))
        }
    }
    #[inline(always)]
    pub const fn p210pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p211pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p212pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p213pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p214pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p215pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa8usize))
        }
    }
    #[inline(always)]
    pub const fn p210pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p211pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p212pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p213pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p214pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p215pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa8usize))
        }
    }
    #[inline(always)]
    pub const fn p210pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p211pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p212pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p213pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p214pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p215pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }

    #[doc = "P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P300Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P300Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P300PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P300PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P300PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P300PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc4usize))
        }
    }
    #[inline(always)]
    pub const fn p301pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p302pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p303pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p304pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p305pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p306pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p307pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p308pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p309pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc4usize))
        }
    }
    #[inline(always)]
    pub const fn p301pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p302pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p303pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p304pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p305pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p306pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p307pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p308pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p309pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc4usize))
        }
    }
    #[inline(always)]
    pub const fn p301pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p302pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p303pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p304pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p305pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p306pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p307pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p308pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p309pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }

    #[doc = "P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p3pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe8usize))
        }
    }
    #[inline(always)]
    pub const fn p310pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p311pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p312pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p313pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p314pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p315pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfcusize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p3pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe8usize))
        }
    }
    #[inline(always)]
    pub const fn p310pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p311pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p312pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p313pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p314pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p315pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfcusize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p3pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe8usize))
        }
    }
    #[inline(always)]
    pub const fn p310pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p311pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p312pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p313pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p314pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p315pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfcusize),
            )
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn p400pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p401pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p402pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p403pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p404pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p405pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p406pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p407pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p408pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p409pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn p400pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p401pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p402pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p403pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p404pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p405pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p406pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p407pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p408pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p409pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn p400pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p401pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p402pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p403pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p404pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p405pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p406pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p407pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p408pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p409pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }
    #[inline(always)]
    pub const fn p410pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p411pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p412pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p413pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p414pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p415pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }
    #[inline(always)]
    pub const fn p410pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p411pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p412pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p413pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p414pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p415pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }
    #[inline(always)]
    pub const fn p410pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p411pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p412pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p413pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p414pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p415pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x160usize))
        }
    }
    #[inline(always)]
    pub const fn p508pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p509pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x160usize))
        }
    }
    #[inline(always)]
    pub const fn p508pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p509pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x160usize))
        }
    }
    #[inline(always)]
    pub const fn p508pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p509pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }

    #[doc = "P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p5pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x168usize))
        }
    }
    #[inline(always)]
    pub const fn p510pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p511pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p512pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p513pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p514pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p515pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17cusize),
            )
        }
    }

    #[doc = "P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p5pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x168usize))
        }
    }
    #[inline(always)]
    pub const fn p510pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p511pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p512pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p513pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p514pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p515pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17cusize),
            )
        }
    }

    #[doc = "P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p5pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x168usize))
        }
    }
    #[inline(always)]
    pub const fn p510pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p511pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p512pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p513pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p514pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p515pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17cusize),
            )
        }
    }

    #[doc = "P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p60pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a0usize))
        }
    }
    #[inline(always)]
    pub const fn p608pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P60Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p609pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P60Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }

    #[doc = "P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p60pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a0usize))
        }
    }
    #[inline(always)]
    pub const fn p608pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p609pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }

    #[doc = "P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p60pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a0usize))
        }
    }
    #[inline(always)]
    pub const fn p608pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p609pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }

    #[doc = "P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p6pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a8usize))
        }
    }
    #[inline(always)]
    pub const fn p610pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p611pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p612pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p613pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p614pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p615pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bcusize),
            )
        }
    }

    #[doc = "P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p6pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a8usize))
        }
    }
    #[inline(always)]
    pub const fn p610pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p611pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p612pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p613pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p614pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p615pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bcusize),
            )
        }
    }

    #[doc = "P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p6pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a8usize))
        }
    }
    #[inline(always)]
    pub const fn p610pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p611pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p612pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p613pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p614pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p615pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bcusize),
            )
        }
    }

    #[doc = "P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p70pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }
    #[inline(always)]
    pub const fn p700pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p701pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p702pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p703pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p704pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p705pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p706pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p707pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p708pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p709pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }

    #[doc = "P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p70pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }
    #[inline(always)]
    pub const fn p700pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p701pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p702pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p703pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p704pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p705pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p706pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p707pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p708pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p709pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }

    #[doc = "P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p70pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }
    #[inline(always)]
    pub const fn p700pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p701pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p702pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p703pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p704pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p705pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p706pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p707pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p708pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p709pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P70PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }

    #[doc = "P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p7pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e8usize))
        }
    }
    #[inline(always)]
    pub const fn p710pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p711pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p712pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p713pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p714pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p715pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fcusize),
            )
        }
    }

    #[doc = "P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p7pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e8usize))
        }
    }
    #[inline(always)]
    pub const fn p710pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p711pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p712pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p713pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p714pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p715pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fcusize),
            )
        }
    }

    #[doc = "P7%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p7pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e8usize))
        }
    }
    #[inline(always)]
    pub const fn p710pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p711pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p712pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p713pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p714pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p715pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fcusize),
            )
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn p800pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p801pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p802pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p803pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p804pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p805pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p806pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p807pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p808pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p809pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn p800pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p801pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p802pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p803pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p804pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p805pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p806pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p807pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p808pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p809pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn p800pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p801pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p802pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p803pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p804pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p805pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p806pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p807pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p808pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p809pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }

    #[doc = "P8%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p8pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x228usize))
        }
    }
    #[inline(always)]
    pub const fn p810pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p811pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p812pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p813pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p814pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p815pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
            )
        }
    }

    #[doc = "P8%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p8pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x228usize))
        }
    }
    #[inline(always)]
    pub const fn p810pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p811pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p812pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p813pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p814pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p815pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
            )
        }
    }

    #[doc = "P8%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p8pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x228usize))
        }
    }
    #[inline(always)]
    pub const fn p810pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p811pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p812pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p813pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p814pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p815pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
            )
        }
    }

    #[doc = "P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p90pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }
    #[inline(always)]
    pub const fn p900pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p901pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p902pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p903pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p904pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p905pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p906pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p907pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x25cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p908pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p909pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }

    #[doc = "P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p90pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }
    #[inline(always)]
    pub const fn p900pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p901pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p902pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p903pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p904pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p905pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p906pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p907pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x25cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p908pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p909pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }

    #[doc = "P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p90pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }
    #[inline(always)]
    pub const fn p900pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p901pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p902pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p903pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p904pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p905pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p906pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p907pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x25cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p908pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p909pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P90PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }

    #[doc = "P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p9pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x268usize))
        }
    }
    #[inline(always)]
    pub const fn p910pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p911pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p912pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p913pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p914pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p915pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27cusize),
            )
        }
    }

    #[doc = "P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p9pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x268usize))
        }
    }
    #[inline(always)]
    pub const fn p910pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p911pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p912pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p913pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p914pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p915pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27cusize),
            )
        }
    }

    #[doc = "P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p9pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x268usize))
        }
    }
    #[inline(always)]
    pub const fn p910pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p911pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p912pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p913pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p914pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p915pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27cusize),
            )
        }
    }

    #[doc = "PA0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }
    #[inline(always)]
    pub const fn pa00pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa01pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa02pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa03pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa04pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa05pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa06pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa07pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa08pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa09pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }

    #[doc = "PA0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }
    #[inline(always)]
    pub const fn pa00pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa01pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa02pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa03pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa04pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa05pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa06pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa07pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa08pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa09pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }

    #[doc = "PA0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }
    #[inline(always)]
    pub const fn pa00pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa01pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa02pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa03pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa04pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa05pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa06pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa07pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa08pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa09pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }

    #[doc = "PA%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn papfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Papfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a8usize))
        }
    }
    #[inline(always)]
    pub const fn pa10pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Papfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Papfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa11pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Papfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Papfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa12pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Papfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Papfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa13pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Papfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Papfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa14pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Papfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Papfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa15pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Papfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Papfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }

    #[doc = "PA%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn papfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a8usize))
        }
    }
    #[inline(always)]
    pub const fn pa10pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa11pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa12pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa13pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa14pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa15pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }

    #[doc = "PA%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn papfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a8usize))
        }
    }
    #[inline(always)]
    pub const fn pa10pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa11pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa12pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa13pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa14pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa15pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PapfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }

    #[doc = "PB0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pb0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }
    #[inline(always)]
    pub const fn pb00pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb01pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb02pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb03pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb04pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb05pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb06pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb07pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb08pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb09pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }

    #[doc = "PB0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pb0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }
    #[inline(always)]
    pub const fn pb00pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb01pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb02pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb03pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb04pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb05pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb06pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb07pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb08pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb09pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }

    #[doc = "PB0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pb0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }
    #[inline(always)]
    pub const fn pb00pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb01pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb02pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb03pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb04pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb05pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb06pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb07pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb08pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb09pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }

    #[doc = "PB%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pbpfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pbpfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2e8usize))
        }
    }
    #[inline(always)]
    pub const fn pb10pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pbpfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbpfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb11pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pbpfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbpfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb12pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pbpfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbpfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb13pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pbpfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbpfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb14pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pbpfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbpfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb15pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pbpfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbpfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }

    #[doc = "PB%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pbpfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::PbpfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2e8usize))
        }
    }
    #[inline(always)]
    pub const fn pb10pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb11pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb12pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb13pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb14pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb15pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }

    #[doc = "PB%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pbpfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::PbpfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2e8usize))
        }
    }
    #[inline(always)]
    pub const fn pb10pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb11pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb12pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb13pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb14pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb15pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::PbpfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PbpfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }

    #[doc = "Ethernet Control Register"]
    #[inline(always)]
    pub const fn pfenet(
        &self,
    ) -> &'static crate::common::Reg<self::Pfenet_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pfenet_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[doc = "NonSecure Write-Protect Register"]
    #[inline(always)]
    pub const fn pwpr_ns(
        &self,
    ) -> &'static crate::common::Reg<self::PwprNs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PwprNs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1292usize),
            )
        }
    }

    #[doc = "Secure Write Protect Register"]
    #[inline(always)]
    pub const fn pwpr_s(&self) -> &'static crate::common::Reg<self::PwprS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PwprS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[doc = "I3C Control Register"]
    #[inline(always)]
    pub const fn pfi3c(&self) -> &'static crate::common::Reg<self::Pfi3C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pfi3C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1312usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 0"]
    #[inline(always)]
    pub const fn p0sar(&self) -> &'static crate::common::Reg<self::P0Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1328usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p1sar(&self) -> &'static crate::common::Reg<self::P1Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1332usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p2sar(&self) -> &'static crate::common::Reg<self::P2Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1336usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p3sar(&self) -> &'static crate::common::Reg<self::P3Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1340usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p4sar(&self) -> &'static crate::common::Reg<self::P4Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1344usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p5sar(&self) -> &'static crate::common::Reg<self::P5Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P5Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1348usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p6sar(&self) -> &'static crate::common::Reg<self::P6Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P6Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1352usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p7sar(&self) -> &'static crate::common::Reg<self::P7Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P7Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1356usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p8sar(&self) -> &'static crate::common::Reg<self::P8Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1360usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn p9sar(&self) -> &'static crate::common::Reg<self::P9Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1364usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn pasar(&self) -> &'static crate::common::Reg<self::Pasar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pasar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1368usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn pbsar(&self) -> &'static crate::common::Reg<self::Pbsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1372usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn pcsar(&self) -> &'static crate::common::Reg<self::Pcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1376usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn pdsar(&self) -> &'static crate::common::Reg<self::Pdsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1380usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn pesar(&self) -> &'static crate::common::Reg<self::Pesar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pesar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1384usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn pfsar(&self) -> &'static crate::common::Reg<self::Pfsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pfsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1388usize),
            )
        }
    }

    #[doc = "Port Security Attribute register 1"]
    #[inline(always)]
    pub const fn pgsar(&self) -> &'static crate::common::Reg<self::Pgsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1392usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000Pfs_SPEC;
impl crate::sealed::RegSpec for P000Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P000 Pin Function Control Register"]
pub type P000Pfs = crate::RegValueT<P000Pfs_SPEC>;

impl P000Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p000pfs::Podr,
        p000pfs::Podr,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p000pfs::Podr,
            p000pfs::Podr,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p000pfs::Pidr,
        p000pfs::Pidr,
        P000Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p000pfs::Pidr,
            p000pfs::Pidr,
            P000Pfs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p000pfs::Pdr,
        p000pfs::Pdr,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p000pfs::Pdr,
            p000pfs::Pdr,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p000pfs::Pcr,
        p000pfs::Pcr,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p000pfs::Pcr,
            p000pfs::Pcr,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p000pfs::Ncodr,
        p000pfs::Ncodr,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p000pfs::Ncodr,
            p000pfs::Ncodr,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p000pfs::Dscr,
        p000pfs::Dscr,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p000pfs::Dscr,
            p000pfs::Dscr,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p000pfs::Isel,
        p000pfs::Isel,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p000pfs::Isel,
            p000pfs::Isel,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p000pfs::Asel,
        p000pfs::Asel,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p000pfs::Asel,
            p000pfs::Asel,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        p000pfs::Pmr,
        p000pfs::Pmr,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p000pfs::Pmr,
            p000pfs::Pmr,
            P000Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P000Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, P000Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P000Pfs {
    #[inline(always)]
    fn default() -> P000Pfs {
        <crate::RegValueT<P000Pfs_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p000pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000PfsHa_SPEC;
impl crate::sealed::RegSpec for P000PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P000 Pin Function Control Register"]
pub type P000PfsHa = crate::RegValueT<P000PfsHa_SPEC>;

impl P000PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p000pfs_ha::Podr,
        p000pfs_ha::Podr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p000pfs_ha::Podr,
            p000pfs_ha::Podr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p000pfs_ha::Pidr,
        p000pfs_ha::Pidr,
        P000PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p000pfs_ha::Pidr,
            p000pfs_ha::Pidr,
            P000PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p000pfs_ha::Pdr,
        p000pfs_ha::Pdr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p000pfs_ha::Pdr,
            p000pfs_ha::Pdr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p000pfs_ha::Pcr,
        p000pfs_ha::Pcr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p000pfs_ha::Pcr,
            p000pfs_ha::Pcr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p000pfs_ha::Ncodr,
        p000pfs_ha::Ncodr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p000pfs_ha::Ncodr,
            p000pfs_ha::Ncodr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p000pfs_ha::Dscr,
        p000pfs_ha::Dscr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p000pfs_ha::Dscr,
            p000pfs_ha::Dscr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, P000PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,P000PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p000pfs_ha::Isel,
        p000pfs_ha::Isel,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p000pfs_ha::Isel,
            p000pfs_ha::Isel,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p000pfs_ha::Asel,
        p000pfs_ha::Asel,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p000pfs_ha::Asel,
            p000pfs_ha::Asel,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P000PfsHa {
    #[inline(always)]
    fn default() -> P000PfsHa {
        <crate::RegValueT<P000PfsHa_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p000pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000PfsBy_SPEC;
impl crate::sealed::RegSpec for P000PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P000 Pin Function Control Register"]
pub type P000PfsBy = crate::RegValueT<P000PfsBy_SPEC>;

impl P000PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p000pfs_by::Podr,
        p000pfs_by::Podr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p000pfs_by::Podr,
            p000pfs_by::Podr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p000pfs_by::Pidr,
        p000pfs_by::Pidr,
        P000PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p000pfs_by::Pidr,
            p000pfs_by::Pidr,
            P000PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p000pfs_by::Pdr,
        p000pfs_by::Pdr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p000pfs_by::Pdr,
            p000pfs_by::Pdr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p000pfs_by::Pcr,
        p000pfs_by::Pcr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p000pfs_by::Pcr,
            p000pfs_by::Pcr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p000pfs_by::Ncodr,
        p000pfs_by::Ncodr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p000pfs_by::Ncodr,
            p000pfs_by::Ncodr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, P000PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,P000PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P000PfsBy {
    #[inline(always)]
    fn default() -> P000PfsBy {
        <crate::RegValueT<P000PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p000pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00Pfs_SPEC;
impl crate::sealed::RegSpec for P00Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P00%s Pin Function Control Register"]
pub type P00Pfs = crate::RegValueT<P00Pfs_SPEC>;

impl NoBitfieldReg<P00Pfs_SPEC> for P00Pfs {}
impl ::core::default::Default for P00Pfs {
    #[inline(always)]
    fn default() -> P00Pfs {
        <crate::RegValueT<P00Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsHa_SPEC;
impl crate::sealed::RegSpec for P00PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P00%s Pin Function Control Register"]
pub type P00PfsHa = crate::RegValueT<P00PfsHa_SPEC>;

impl NoBitfieldReg<P00PfsHa_SPEC> for P00PfsHa {}
impl ::core::default::Default for P00PfsHa {
    #[inline(always)]
    fn default() -> P00PfsHa {
        <crate::RegValueT<P00PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsBy_SPEC;
impl crate::sealed::RegSpec for P00PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P00%s Pin Function Control Register"]
pub type P00PfsBy = crate::RegValueT<P00PfsBy_SPEC>;

impl NoBitfieldReg<P00PfsBy_SPEC> for P00PfsBy {}
impl ::core::default::Default for P00PfsBy {
    #[inline(always)]
    fn default() -> P00PfsBy {
        <crate::RegValueT<P00PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0Pfs_SPEC;
impl crate::sealed::RegSpec for P0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P0%s Pin Function Control Register"]
pub type P0Pfs = crate::RegValueT<P0Pfs_SPEC>;

impl NoBitfieldReg<P0Pfs_SPEC> for P0Pfs {}
impl ::core::default::Default for P0Pfs {
    #[inline(always)]
    fn default() -> P0Pfs {
        <crate::RegValueT<P0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsHa_SPEC;
impl crate::sealed::RegSpec for P0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P0%s Pin Function Control Register"]
pub type P0PfsHa = crate::RegValueT<P0PfsHa_SPEC>;

impl NoBitfieldReg<P0PfsHa_SPEC> for P0PfsHa {}
impl ::core::default::Default for P0PfsHa {
    #[inline(always)]
    fn default() -> P0PfsHa {
        <crate::RegValueT<P0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsBy_SPEC;
impl crate::sealed::RegSpec for P0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P0%s Pin Function Control Register"]
pub type P0PfsBy = crate::RegValueT<P0PfsBy_SPEC>;

impl NoBitfieldReg<P0PfsBy_SPEC> for P0PfsBy {}
impl ::core::default::Default for P0PfsBy {
    #[inline(always)]
    fn default() -> P0PfsBy {
        <crate::RegValueT<P0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P100Pfs_SPEC;
impl crate::sealed::RegSpec for P100Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P100 Pin Function Control Register"]
pub type P100Pfs = crate::RegValueT<P100Pfs_SPEC>;

impl P100Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p100pfs::Podr,
        p100pfs::Podr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p100pfs::Podr,
            p100pfs::Podr,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p100pfs::Pidr,
        p100pfs::Pidr,
        P100Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p100pfs::Pidr,
            p100pfs::Pidr,
            P100Pfs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p100pfs::Pdr,
        p100pfs::Pdr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p100pfs::Pdr,
            p100pfs::Pdr,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p100pfs::Pcr,
        p100pfs::Pcr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p100pfs::Pcr,
            p100pfs::Pcr,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p100pfs::Ncodr,
        p100pfs::Ncodr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p100pfs::Ncodr,
            p100pfs::Ncodr,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p100pfs::Dscr,
        p100pfs::Dscr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p100pfs::Dscr,
            p100pfs::Dscr,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p100pfs::Eofr,
        p100pfs::Eofr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p100pfs::Eofr,
            p100pfs::Eofr,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p100pfs::Isel,
        p100pfs::Isel,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p100pfs::Isel,
            p100pfs::Isel,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p100pfs::Asel,
        p100pfs::Asel,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p100pfs::Asel,
            p100pfs::Asel,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        p100pfs::Pmr,
        p100pfs::Pmr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p100pfs::Pmr,
            p100pfs::Pmr,
            P100Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P100Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, P100Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P100Pfs {
    #[inline(always)]
    fn default() -> P100Pfs {
        <crate::RegValueT<P100Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P100PfsHa_SPEC;
impl crate::sealed::RegSpec for P100PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P100 Pin Function Control Register"]
pub type P100PfsHa = crate::RegValueT<P100PfsHa_SPEC>;

impl P100PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p100pfs_ha::Podr,
        p100pfs_ha::Podr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p100pfs_ha::Podr,
            p100pfs_ha::Podr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p100pfs_ha::Pidr,
        p100pfs_ha::Pidr,
        P100PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p100pfs_ha::Pidr,
            p100pfs_ha::Pidr,
            P100PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p100pfs_ha::Pdr,
        p100pfs_ha::Pdr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p100pfs_ha::Pdr,
            p100pfs_ha::Pdr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p100pfs_ha::Pcr,
        p100pfs_ha::Pcr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p100pfs_ha::Pcr,
            p100pfs_ha::Pcr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p100pfs_ha::Ncodr,
        p100pfs_ha::Ncodr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p100pfs_ha::Ncodr,
            p100pfs_ha::Ncodr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, P100PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,P100PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p100pfs_ha::Dscr,
        p100pfs_ha::Dscr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p100pfs_ha::Dscr,
            p100pfs_ha::Dscr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p100pfs_ha::Eofr,
        p100pfs_ha::Eofr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p100pfs_ha::Eofr,
            p100pfs_ha::Eofr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p100pfs_ha::Isel,
        p100pfs_ha::Isel,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p100pfs_ha::Isel,
            p100pfs_ha::Isel,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p100pfs_ha::Asel,
        p100pfs_ha::Asel,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p100pfs_ha::Asel,
            p100pfs_ha::Asel,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P100PfsHa {
    #[inline(always)]
    fn default() -> P100PfsHa {
        <crate::RegValueT<P100PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P100PfsBy_SPEC;
impl crate::sealed::RegSpec for P100PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P100 Pin Function Control Register"]
pub type P100PfsBy = crate::RegValueT<P100PfsBy_SPEC>;

impl P100PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p100pfs_by::Podr,
        p100pfs_by::Podr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p100pfs_by::Podr,
            p100pfs_by::Podr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p100pfs_by::Pidr,
        p100pfs_by::Pidr,
        P100PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p100pfs_by::Pidr,
            p100pfs_by::Pidr,
            P100PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p100pfs_by::Pdr,
        p100pfs_by::Pdr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p100pfs_by::Pdr,
            p100pfs_by::Pdr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p100pfs_by::Pcr,
        p100pfs_by::Pcr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p100pfs_by::Pcr,
            p100pfs_by::Pcr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p100pfs_by::Ncodr,
        p100pfs_by::Ncodr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p100pfs_by::Ncodr,
            p100pfs_by::Ncodr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, P100PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,P100PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P100PfsBy {
    #[inline(always)]
    fn default() -> P100PfsBy {
        <crate::RegValueT<P100PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10Pfs_SPEC;
impl crate::sealed::RegSpec for P10Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P10%s Pin Function Control Register"]
pub type P10Pfs = crate::RegValueT<P10Pfs_SPEC>;

impl NoBitfieldReg<P10Pfs_SPEC> for P10Pfs {}
impl ::core::default::Default for P10Pfs {
    #[inline(always)]
    fn default() -> P10Pfs {
        <crate::RegValueT<P10Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsHa_SPEC;
impl crate::sealed::RegSpec for P10PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P10%s Pin Function Control Register"]
pub type P10PfsHa = crate::RegValueT<P10PfsHa_SPEC>;

impl NoBitfieldReg<P10PfsHa_SPEC> for P10PfsHa {}
impl ::core::default::Default for P10PfsHa {
    #[inline(always)]
    fn default() -> P10PfsHa {
        <crate::RegValueT<P10PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsBy_SPEC;
impl crate::sealed::RegSpec for P10PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P10%s Pin Function Control Register"]
pub type P10PfsBy = crate::RegValueT<P10PfsBy_SPEC>;

impl NoBitfieldReg<P10PfsBy_SPEC> for P10PfsBy {}
impl ::core::default::Default for P10PfsBy {
    #[inline(always)]
    fn default() -> P10PfsBy {
        <crate::RegValueT<P10PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108Pfs_SPEC;
impl crate::sealed::RegSpec for P108Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P108 Pin Function Control Register"]
pub type P108Pfs = crate::RegValueT<P108Pfs_SPEC>;

impl P108Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p108pfs::Podr,
        p108pfs::Podr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p108pfs::Podr,
            p108pfs::Podr,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p108pfs::Pidr,
        p108pfs::Pidr,
        P108Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p108pfs::Pidr,
            p108pfs::Pidr,
            P108Pfs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p108pfs::Pdr,
        p108pfs::Pdr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p108pfs::Pdr,
            p108pfs::Pdr,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p108pfs::Pcr,
        p108pfs::Pcr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p108pfs::Pcr,
            p108pfs::Pcr,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p108pfs::Ncodr,
        p108pfs::Ncodr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p108pfs::Ncodr,
            p108pfs::Ncodr,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p108pfs::Dscr,
        p108pfs::Dscr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p108pfs::Dscr,
            p108pfs::Dscr,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p108pfs::Eofr,
        p108pfs::Eofr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p108pfs::Eofr,
            p108pfs::Eofr,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p108pfs::Isel,
        p108pfs::Isel,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p108pfs::Isel,
            p108pfs::Isel,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p108pfs::Asel,
        p108pfs::Asel,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p108pfs::Asel,
            p108pfs::Asel,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        p108pfs::Pmr,
        p108pfs::Pmr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p108pfs::Pmr,
            p108pfs::Pmr,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P108Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, P108Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P108Pfs {
    #[inline(always)]
    fn default() -> P108Pfs {
        <crate::RegValueT<P108Pfs_SPEC> as RegisterValue<_>>::new(66576)
    }
}
pub mod p108pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108PfsHa_SPEC;
impl crate::sealed::RegSpec for P108PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P108 Pin Function Control Register"]
pub type P108PfsHa = crate::RegValueT<P108PfsHa_SPEC>;

impl P108PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p108pfs_ha::Podr,
        p108pfs_ha::Podr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p108pfs_ha::Podr,
            p108pfs_ha::Podr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p108pfs_ha::Pidr,
        p108pfs_ha::Pidr,
        P108PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p108pfs_ha::Pidr,
            p108pfs_ha::Pidr,
            P108PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p108pfs_ha::Pdr,
        p108pfs_ha::Pdr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p108pfs_ha::Pdr,
            p108pfs_ha::Pdr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p108pfs_ha::Pcr,
        p108pfs_ha::Pcr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p108pfs_ha::Pcr,
            p108pfs_ha::Pcr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p108pfs_ha::Ncodr,
        p108pfs_ha::Ncodr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p108pfs_ha::Ncodr,
            p108pfs_ha::Ncodr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, P108PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,P108PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p108pfs_ha::Dscr,
        p108pfs_ha::Dscr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p108pfs_ha::Dscr,
            p108pfs_ha::Dscr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p108pfs_ha::Eofr,
        p108pfs_ha::Eofr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p108pfs_ha::Eofr,
            p108pfs_ha::Eofr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p108pfs_ha::Isel,
        p108pfs_ha::Isel,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p108pfs_ha::Isel,
            p108pfs_ha::Isel,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p108pfs_ha::Asel,
        p108pfs_ha::Asel,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p108pfs_ha::Asel,
            p108pfs_ha::Asel,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P108PfsHa {
    #[inline(always)]
    fn default() -> P108PfsHa {
        <crate::RegValueT<P108PfsHa_SPEC> as RegisterValue<_>>::new(1040)
    }
}
pub mod p108pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108PfsBy_SPEC;
impl crate::sealed::RegSpec for P108PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P108 Pin Function Control Register"]
pub type P108PfsBy = crate::RegValueT<P108PfsBy_SPEC>;

impl P108PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p108pfs_by::Podr,
        p108pfs_by::Podr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p108pfs_by::Podr,
            p108pfs_by::Podr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p108pfs_by::Pidr,
        p108pfs_by::Pidr,
        P108PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p108pfs_by::Pidr,
            p108pfs_by::Pidr,
            P108PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p108pfs_by::Pdr,
        p108pfs_by::Pdr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p108pfs_by::Pdr,
            p108pfs_by::Pdr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p108pfs_by::Pcr,
        p108pfs_by::Pcr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p108pfs_by::Pcr,
            p108pfs_by::Pcr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p108pfs_by::Ncodr,
        p108pfs_by::Ncodr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p108pfs_by::Ncodr,
            p108pfs_by::Ncodr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, P108PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,P108PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P108PfsBy {
    #[inline(always)]
    fn default() -> P108PfsBy {
        <crate::RegValueT<P108PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p108pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109Pfs_SPEC;
impl crate::sealed::RegSpec for P109Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P109 Pin Function Control Register"]
pub type P109Pfs = crate::RegValueT<P109Pfs_SPEC>;

impl NoBitfieldReg<P109Pfs_SPEC> for P109Pfs {}
impl ::core::default::Default for P109Pfs {
    #[inline(always)]
    fn default() -> P109Pfs {
        <crate::RegValueT<P109Pfs_SPEC> as RegisterValue<_>>::new(66576)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109PfsHa_SPEC;
impl crate::sealed::RegSpec for P109PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P109 Pin Function Control Register"]
pub type P109PfsHa = crate::RegValueT<P109PfsHa_SPEC>;

impl NoBitfieldReg<P109PfsHa_SPEC> for P109PfsHa {}
impl ::core::default::Default for P109PfsHa {
    #[inline(always)]
    fn default() -> P109PfsHa {
        <crate::RegValueT<P109PfsHa_SPEC> as RegisterValue<_>>::new(1040)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109PfsBy_SPEC;
impl crate::sealed::RegSpec for P109PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P109 Pin Function Control Register"]
pub type P109PfsBy = crate::RegValueT<P109PfsBy_SPEC>;

impl NoBitfieldReg<P109PfsBy_SPEC> for P109PfsBy {}
impl ::core::default::Default for P109PfsBy {
    #[inline(always)]
    fn default() -> P109PfsBy {
        <crate::RegValueT<P109PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110Pfs_SPEC;
impl crate::sealed::RegSpec for P110Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P110 Pin Function Control Register"]
pub type P110Pfs = crate::RegValueT<P110Pfs_SPEC>;

impl P110Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p110pfs::Podr,
        p110pfs::Podr,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p110pfs::Podr,
            p110pfs::Podr,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p110pfs::Pidr,
        p110pfs::Pidr,
        P110Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p110pfs::Pidr,
            p110pfs::Pidr,
            P110Pfs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p110pfs::Pdr,
        p110pfs::Pdr,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p110pfs::Pdr,
            p110pfs::Pdr,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p110pfs::Pcr,
        p110pfs::Pcr,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p110pfs::Pcr,
            p110pfs::Pcr,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p110pfs::Ncodr,
        p110pfs::Ncodr,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p110pfs::Ncodr,
            p110pfs::Ncodr,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p110pfs::Dscr,
        p110pfs::Dscr,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p110pfs::Dscr,
            p110pfs::Dscr,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p110pfs::Eofr,
        p110pfs::Eofr,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p110pfs::Eofr,
            p110pfs::Eofr,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p110pfs::Isel,
        p110pfs::Isel,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p110pfs::Isel,
            p110pfs::Isel,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p110pfs::Asel,
        p110pfs::Asel,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p110pfs::Asel,
            p110pfs::Asel,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        p110pfs::Pmr,
        p110pfs::Pmr,
        P110Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p110pfs::Pmr,
            p110pfs::Pmr,
            P110Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P110Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, P110Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P110Pfs {
    #[inline(always)]
    fn default() -> P110Pfs {
        <crate::RegValueT<P110Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}
pub mod p110pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110PfsHa_SPEC;
impl crate::sealed::RegSpec for P110PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P110 Pin Function Control Register"]
pub type P110PfsHa = crate::RegValueT<P110PfsHa_SPEC>;

impl P110PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p110pfs_ha::Podr,
        p110pfs_ha::Podr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p110pfs_ha::Podr,
            p110pfs_ha::Podr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p110pfs_ha::Pidr,
        p110pfs_ha::Pidr,
        P110PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p110pfs_ha::Pidr,
            p110pfs_ha::Pidr,
            P110PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p110pfs_ha::Pdr,
        p110pfs_ha::Pdr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p110pfs_ha::Pdr,
            p110pfs_ha::Pdr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p110pfs_ha::Pcr,
        p110pfs_ha::Pcr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p110pfs_ha::Pcr,
            p110pfs_ha::Pcr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p110pfs_ha::Ncodr,
        p110pfs_ha::Ncodr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p110pfs_ha::Ncodr,
            p110pfs_ha::Ncodr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, P110PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,P110PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p110pfs_ha::Dscr,
        p110pfs_ha::Dscr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p110pfs_ha::Dscr,
            p110pfs_ha::Dscr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p110pfs_ha::Eofr,
        p110pfs_ha::Eofr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p110pfs_ha::Eofr,
            p110pfs_ha::Eofr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p110pfs_ha::Isel,
        p110pfs_ha::Isel,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p110pfs_ha::Isel,
            p110pfs_ha::Isel,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p110pfs_ha::Asel,
        p110pfs_ha::Asel,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p110pfs_ha::Asel,
            p110pfs_ha::Asel,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P110PfsHa {
    #[inline(always)]
    fn default() -> P110PfsHa {
        <crate::RegValueT<P110PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p110pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110PfsBy_SPEC;
impl crate::sealed::RegSpec for P110PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P110 Pin Function Control Register"]
pub type P110PfsBy = crate::RegValueT<P110PfsBy_SPEC>;

impl P110PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p110pfs_by::Podr,
        p110pfs_by::Podr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p110pfs_by::Podr,
            p110pfs_by::Podr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p110pfs_by::Pidr,
        p110pfs_by::Pidr,
        P110PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p110pfs_by::Pidr,
            p110pfs_by::Pidr,
            P110PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p110pfs_by::Pdr,
        p110pfs_by::Pdr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p110pfs_by::Pdr,
            p110pfs_by::Pdr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p110pfs_by::Pcr,
        p110pfs_by::Pcr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p110pfs_by::Pcr,
            p110pfs_by::Pcr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p110pfs_by::Ncodr,
        p110pfs_by::Ncodr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p110pfs_by::Ncodr,
            p110pfs_by::Ncodr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, P110PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,P110PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P110PfsBy {
    #[inline(always)]
    fn default() -> P110PfsBy {
        <crate::RegValueT<P110PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p110pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1Pfs_SPEC;
impl crate::sealed::RegSpec for P1Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P1%s Pin Function Control Register"]
pub type P1Pfs = crate::RegValueT<P1Pfs_SPEC>;

impl NoBitfieldReg<P1Pfs_SPEC> for P1Pfs {}
impl ::core::default::Default for P1Pfs {
    #[inline(always)]
    fn default() -> P1Pfs {
        <crate::RegValueT<P1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsHa_SPEC;
impl crate::sealed::RegSpec for P1PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P1%s Pin Function Control Register"]
pub type P1PfsHa = crate::RegValueT<P1PfsHa_SPEC>;

impl NoBitfieldReg<P1PfsHa_SPEC> for P1PfsHa {}
impl ::core::default::Default for P1PfsHa {
    #[inline(always)]
    fn default() -> P1PfsHa {
        <crate::RegValueT<P1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsBy_SPEC;
impl crate::sealed::RegSpec for P1PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P1%s Pin Function Control Register"]
pub type P1PfsBy = crate::RegValueT<P1PfsBy_SPEC>;

impl NoBitfieldReg<P1PfsBy_SPEC> for P1PfsBy {}
impl ::core::default::Default for P1PfsBy {
    #[inline(always)]
    fn default() -> P1PfsBy {
        <crate::RegValueT<P1PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200Pfs_SPEC;
impl crate::sealed::RegSpec for P200Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P200 Pin Function Control Register"]
pub type P200Pfs = crate::RegValueT<P200Pfs_SPEC>;

impl NoBitfieldReg<P200Pfs_SPEC> for P200Pfs {}
impl ::core::default::Default for P200Pfs {
    #[inline(always)]
    fn default() -> P200Pfs {
        <crate::RegValueT<P200Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsHa_SPEC;
impl crate::sealed::RegSpec for P200PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P200 Pin Function Control Register"]
pub type P200PfsHa = crate::RegValueT<P200PfsHa_SPEC>;

impl NoBitfieldReg<P200PfsHa_SPEC> for P200PfsHa {}
impl ::core::default::Default for P200PfsHa {
    #[inline(always)]
    fn default() -> P200PfsHa {
        <crate::RegValueT<P200PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsBy_SPEC;
impl crate::sealed::RegSpec for P200PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P200 Pin Function Control Register"]
pub type P200PfsBy = crate::RegValueT<P200PfsBy_SPEC>;

impl NoBitfieldReg<P200PfsBy_SPEC> for P200PfsBy {}
impl ::core::default::Default for P200PfsBy {
    #[inline(always)]
    fn default() -> P200PfsBy {
        <crate::RegValueT<P200PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201Pfs_SPEC;
impl crate::sealed::RegSpec for P201Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P201 Pin Function Control Register"]
pub type P201Pfs = crate::RegValueT<P201Pfs_SPEC>;

impl P201Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p201pfs::Podr,
        p201pfs::Podr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p201pfs::Podr,
            p201pfs::Podr,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p201pfs::Pidr,
        p201pfs::Pidr,
        P201Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p201pfs::Pidr,
            p201pfs::Pidr,
            P201Pfs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p201pfs::Pdr,
        p201pfs::Pdr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p201pfs::Pdr,
            p201pfs::Pdr,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p201pfs::Pcr,
        p201pfs::Pcr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p201pfs::Pcr,
            p201pfs::Pcr,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p201pfs::Ncodr,
        p201pfs::Ncodr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p201pfs::Ncodr,
            p201pfs::Ncodr,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p201pfs::Dscr,
        p201pfs::Dscr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p201pfs::Dscr,
            p201pfs::Dscr,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p201pfs::Eofr,
        p201pfs::Eofr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p201pfs::Eofr,
            p201pfs::Eofr,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p201pfs::Isel,
        p201pfs::Isel,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p201pfs::Isel,
            p201pfs::Isel,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p201pfs::Asel,
        p201pfs::Asel,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p201pfs::Asel,
            p201pfs::Asel,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        p201pfs::Pmr,
        p201pfs::Pmr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p201pfs::Pmr,
            p201pfs::Pmr,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P201Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, P201Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P201Pfs {
    #[inline(always)]
    fn default() -> P201Pfs {
        <crate::RegValueT<P201Pfs_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsHa_SPEC;
impl crate::sealed::RegSpec for P201PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P201 Pin Function Control Register"]
pub type P201PfsHa = crate::RegValueT<P201PfsHa_SPEC>;

impl P201PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p201pfs_ha::Podr,
        p201pfs_ha::Podr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p201pfs_ha::Podr,
            p201pfs_ha::Podr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p201pfs_ha::Pidr,
        p201pfs_ha::Pidr,
        P201PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p201pfs_ha::Pidr,
            p201pfs_ha::Pidr,
            P201PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p201pfs_ha::Pdr,
        p201pfs_ha::Pdr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p201pfs_ha::Pdr,
            p201pfs_ha::Pdr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p201pfs_ha::Pcr,
        p201pfs_ha::Pcr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p201pfs_ha::Pcr,
            p201pfs_ha::Pcr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p201pfs_ha::Ncodr,
        p201pfs_ha::Ncodr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p201pfs_ha::Ncodr,
            p201pfs_ha::Ncodr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, u8, P201PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x7,1,0,u8,u8,P201PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p201pfs_ha::Dscr,
        p201pfs_ha::Dscr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p201pfs_ha::Dscr,
            p201pfs_ha::Dscr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p201pfs_ha::Eofr,
        p201pfs_ha::Eofr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p201pfs_ha::Eofr,
            p201pfs_ha::Eofr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p201pfs_ha::Isel,
        p201pfs_ha::Isel,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p201pfs_ha::Isel,
            p201pfs_ha::Isel,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p201pfs_ha::Asel,
        p201pfs_ha::Asel,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p201pfs_ha::Asel,
            p201pfs_ha::Asel,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P201PfsHa {
    #[inline(always)]
    fn default() -> P201PfsHa {
        <crate::RegValueT<P201PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);

        #[doc = "Extra high drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "No need to care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsBy_SPEC;
impl crate::sealed::RegSpec for P201PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P201 Pin Function Control Register"]
pub type P201PfsBy = crate::RegValueT<P201PfsBy_SPEC>;

impl P201PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p201pfs_by::Podr,
        p201pfs_by::Podr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p201pfs_by::Podr,
            p201pfs_by::Podr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p201pfs_by::Pidr,
        p201pfs_by::Pidr,
        P201PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p201pfs_by::Pidr,
            p201pfs_by::Pidr,
            P201PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p201pfs_by::Pdr,
        p201pfs_by::Pdr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p201pfs_by::Pdr,
            p201pfs_by::Pdr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p201pfs_by::Pcr,
        p201pfs_by::Pcr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p201pfs_by::Pcr,
            p201pfs_by::Pcr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p201pfs_by::Ncodr,
        p201pfs_by::Ncodr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p201pfs_by::Ncodr,
            p201pfs_by::Ncodr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, P201PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,P201PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P201PfsBy {
    #[inline(always)]
    fn default() -> P201PfsBy {
        <crate::RegValueT<P201PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20Pfs_SPEC;
impl crate::sealed::RegSpec for P20Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P20%s Pin Function Control Register"]
pub type P20Pfs = crate::RegValueT<P20Pfs_SPEC>;

impl NoBitfieldReg<P20Pfs_SPEC> for P20Pfs {}
impl ::core::default::Default for P20Pfs {
    #[inline(always)]
    fn default() -> P20Pfs {
        <crate::RegValueT<P20Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsHa_SPEC;
impl crate::sealed::RegSpec for P20PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P20%s Pin Function Control Register"]
pub type P20PfsHa = crate::RegValueT<P20PfsHa_SPEC>;

impl NoBitfieldReg<P20PfsHa_SPEC> for P20PfsHa {}
impl ::core::default::Default for P20PfsHa {
    #[inline(always)]
    fn default() -> P20PfsHa {
        <crate::RegValueT<P20PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsBy_SPEC;
impl crate::sealed::RegSpec for P20PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P20%s Pin Function Control Register"]
pub type P20PfsBy = crate::RegValueT<P20PfsBy_SPEC>;

impl NoBitfieldReg<P20PfsBy_SPEC> for P20PfsBy {}
impl ::core::default::Default for P20PfsBy {
    #[inline(always)]
    fn default() -> P20PfsBy {
        <crate::RegValueT<P20PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2Pfs_SPEC;
impl crate::sealed::RegSpec for P2Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P2%s Pin Function Control Register"]
pub type P2Pfs = crate::RegValueT<P2Pfs_SPEC>;

impl NoBitfieldReg<P2Pfs_SPEC> for P2Pfs {}
impl ::core::default::Default for P2Pfs {
    #[inline(always)]
    fn default() -> P2Pfs {
        <crate::RegValueT<P2Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsHa_SPEC;
impl crate::sealed::RegSpec for P2PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P2%s Pin Function Control Register"]
pub type P2PfsHa = crate::RegValueT<P2PfsHa_SPEC>;

impl NoBitfieldReg<P2PfsHa_SPEC> for P2PfsHa {}
impl ::core::default::Default for P2PfsHa {
    #[inline(always)]
    fn default() -> P2PfsHa {
        <crate::RegValueT<P2PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsBy_SPEC;
impl crate::sealed::RegSpec for P2PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P2%s Pin Function Control Register"]
pub type P2PfsBy = crate::RegValueT<P2PfsBy_SPEC>;

impl NoBitfieldReg<P2PfsBy_SPEC> for P2PfsBy {}
impl ::core::default::Default for P2PfsBy {
    #[inline(always)]
    fn default() -> P2PfsBy {
        <crate::RegValueT<P2PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300Pfs_SPEC;
impl crate::sealed::RegSpec for P300Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P300 Pin Function Control Register"]
pub type P300Pfs = crate::RegValueT<P300Pfs_SPEC>;

impl NoBitfieldReg<P300Pfs_SPEC> for P300Pfs {}
impl ::core::default::Default for P300Pfs {
    #[inline(always)]
    fn default() -> P300Pfs {
        <crate::RegValueT<P300Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300PfsHa_SPEC;
impl crate::sealed::RegSpec for P300PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P300 Pin Function Control Register"]
pub type P300PfsHa = crate::RegValueT<P300PfsHa_SPEC>;

impl NoBitfieldReg<P300PfsHa_SPEC> for P300PfsHa {}
impl ::core::default::Default for P300PfsHa {
    #[inline(always)]
    fn default() -> P300PfsHa {
        <crate::RegValueT<P300PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300PfsBy_SPEC;
impl crate::sealed::RegSpec for P300PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P300 Pin Function Control Register"]
pub type P300PfsBy = crate::RegValueT<P300PfsBy_SPEC>;

impl NoBitfieldReg<P300PfsBy_SPEC> for P300PfsBy {}
impl ::core::default::Default for P300PfsBy {
    #[inline(always)]
    fn default() -> P300PfsBy {
        <crate::RegValueT<P300PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30Pfs_SPEC;
impl crate::sealed::RegSpec for P30Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P30%s Pin Function Control Register"]
pub type P30Pfs = crate::RegValueT<P30Pfs_SPEC>;

impl NoBitfieldReg<P30Pfs_SPEC> for P30Pfs {}
impl ::core::default::Default for P30Pfs {
    #[inline(always)]
    fn default() -> P30Pfs {
        <crate::RegValueT<P30Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsHa_SPEC;
impl crate::sealed::RegSpec for P30PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P30%s Pin Function Control Register"]
pub type P30PfsHa = crate::RegValueT<P30PfsHa_SPEC>;

impl NoBitfieldReg<P30PfsHa_SPEC> for P30PfsHa {}
impl ::core::default::Default for P30PfsHa {
    #[inline(always)]
    fn default() -> P30PfsHa {
        <crate::RegValueT<P30PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsBy_SPEC;
impl crate::sealed::RegSpec for P30PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P30%s Pin Function Control Register"]
pub type P30PfsBy = crate::RegValueT<P30PfsBy_SPEC>;

impl NoBitfieldReg<P30PfsBy_SPEC> for P30PfsBy {}
impl ::core::default::Default for P30PfsBy {
    #[inline(always)]
    fn default() -> P30PfsBy {
        <crate::RegValueT<P30PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3Pfs_SPEC;
impl crate::sealed::RegSpec for P3Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P3%s Pin Function Control Register"]
pub type P3Pfs = crate::RegValueT<P3Pfs_SPEC>;

impl NoBitfieldReg<P3Pfs_SPEC> for P3Pfs {}
impl ::core::default::Default for P3Pfs {
    #[inline(always)]
    fn default() -> P3Pfs {
        <crate::RegValueT<P3Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PfsHa_SPEC;
impl crate::sealed::RegSpec for P3PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P30%s Pin Function Control Register"]
pub type P3PfsHa = crate::RegValueT<P3PfsHa_SPEC>;

impl NoBitfieldReg<P3PfsHa_SPEC> for P3PfsHa {}
impl ::core::default::Default for P3PfsHa {
    #[inline(always)]
    fn default() -> P3PfsHa {
        <crate::RegValueT<P3PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PfsBy_SPEC;
impl crate::sealed::RegSpec for P3PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P30%s Pin Function Control Register"]
pub type P3PfsBy = crate::RegValueT<P3PfsBy_SPEC>;

impl NoBitfieldReg<P3PfsBy_SPEC> for P3PfsBy {}
impl ::core::default::Default for P3PfsBy {
    #[inline(always)]
    fn default() -> P3PfsBy {
        <crate::RegValueT<P3PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40Pfs_SPEC;
impl crate::sealed::RegSpec for P40Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P40%s Pin Function Control Register"]
pub type P40Pfs = crate::RegValueT<P40Pfs_SPEC>;

impl NoBitfieldReg<P40Pfs_SPEC> for P40Pfs {}
impl ::core::default::Default for P40Pfs {
    #[inline(always)]
    fn default() -> P40Pfs {
        <crate::RegValueT<P40Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsHa_SPEC;
impl crate::sealed::RegSpec for P40PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P40%s Pin Function Control Register"]
pub type P40PfsHa = crate::RegValueT<P40PfsHa_SPEC>;

impl NoBitfieldReg<P40PfsHa_SPEC> for P40PfsHa {}
impl ::core::default::Default for P40PfsHa {
    #[inline(always)]
    fn default() -> P40PfsHa {
        <crate::RegValueT<P40PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsBy_SPEC;
impl crate::sealed::RegSpec for P40PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P40%s Pin Function Control Register"]
pub type P40PfsBy = crate::RegValueT<P40PfsBy_SPEC>;

impl NoBitfieldReg<P40PfsBy_SPEC> for P40PfsBy {}
impl ::core::default::Default for P40PfsBy {
    #[inline(always)]
    fn default() -> P40PfsBy {
        <crate::RegValueT<P40PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4Pfs_SPEC;
impl crate::sealed::RegSpec for P4Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P4%s Pin Function Control Register"]
pub type P4Pfs = crate::RegValueT<P4Pfs_SPEC>;

impl NoBitfieldReg<P4Pfs_SPEC> for P4Pfs {}
impl ::core::default::Default for P4Pfs {
    #[inline(always)]
    fn default() -> P4Pfs {
        <crate::RegValueT<P4Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsHa_SPEC;
impl crate::sealed::RegSpec for P4PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P4%s Pin Function Control Register"]
pub type P4PfsHa = crate::RegValueT<P4PfsHa_SPEC>;

impl NoBitfieldReg<P4PfsHa_SPEC> for P4PfsHa {}
impl ::core::default::Default for P4PfsHa {
    #[inline(always)]
    fn default() -> P4PfsHa {
        <crate::RegValueT<P4PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsBy_SPEC;
impl crate::sealed::RegSpec for P4PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P4%s Pin Function Control Register"]
pub type P4PfsBy = crate::RegValueT<P4PfsBy_SPEC>;

impl NoBitfieldReg<P4PfsBy_SPEC> for P4PfsBy {}
impl ::core::default::Default for P4PfsBy {
    #[inline(always)]
    fn default() -> P4PfsBy {
        <crate::RegValueT<P4PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50Pfs_SPEC;
impl crate::sealed::RegSpec for P50Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P50%s Pin Function Control Register"]
pub type P50Pfs = crate::RegValueT<P50Pfs_SPEC>;

impl NoBitfieldReg<P50Pfs_SPEC> for P50Pfs {}
impl ::core::default::Default for P50Pfs {
    #[inline(always)]
    fn default() -> P50Pfs {
        <crate::RegValueT<P50Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsHa_SPEC;
impl crate::sealed::RegSpec for P50PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P50%s Pin Function Control Register"]
pub type P50PfsHa = crate::RegValueT<P50PfsHa_SPEC>;

impl NoBitfieldReg<P50PfsHa_SPEC> for P50PfsHa {}
impl ::core::default::Default for P50PfsHa {
    #[inline(always)]
    fn default() -> P50PfsHa {
        <crate::RegValueT<P50PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsBy_SPEC;
impl crate::sealed::RegSpec for P50PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P50%s Pin Function Control Register"]
pub type P50PfsBy = crate::RegValueT<P50PfsBy_SPEC>;

impl NoBitfieldReg<P50PfsBy_SPEC> for P50PfsBy {}
impl ::core::default::Default for P50PfsBy {
    #[inline(always)]
    fn default() -> P50PfsBy {
        <crate::RegValueT<P50PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5Pfs_SPEC;
impl crate::sealed::RegSpec for P5Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P5%s Pin Function Control Register"]
pub type P5Pfs = crate::RegValueT<P5Pfs_SPEC>;

impl NoBitfieldReg<P5Pfs_SPEC> for P5Pfs {}
impl ::core::default::Default for P5Pfs {
    #[inline(always)]
    fn default() -> P5Pfs {
        <crate::RegValueT<P5Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5PfsHa_SPEC;
impl crate::sealed::RegSpec for P5PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P5%s Pin Function Control Register"]
pub type P5PfsHa = crate::RegValueT<P5PfsHa_SPEC>;

impl NoBitfieldReg<P5PfsHa_SPEC> for P5PfsHa {}
impl ::core::default::Default for P5PfsHa {
    #[inline(always)]
    fn default() -> P5PfsHa {
        <crate::RegValueT<P5PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5PfsBy_SPEC;
impl crate::sealed::RegSpec for P5PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P5%s Pin Function Control Register"]
pub type P5PfsBy = crate::RegValueT<P5PfsBy_SPEC>;

impl NoBitfieldReg<P5PfsBy_SPEC> for P5PfsBy {}
impl ::core::default::Default for P5PfsBy {
    #[inline(always)]
    fn default() -> P5PfsBy {
        <crate::RegValueT<P5PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60Pfs_SPEC;
impl crate::sealed::RegSpec for P60Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P60%s Pin Function Control Register"]
pub type P60Pfs = crate::RegValueT<P60Pfs_SPEC>;

impl NoBitfieldReg<P60Pfs_SPEC> for P60Pfs {}
impl ::core::default::Default for P60Pfs {
    #[inline(always)]
    fn default() -> P60Pfs {
        <crate::RegValueT<P60Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsHa_SPEC;
impl crate::sealed::RegSpec for P60PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P60%s Pin Function Control Register"]
pub type P60PfsHa = crate::RegValueT<P60PfsHa_SPEC>;

impl NoBitfieldReg<P60PfsHa_SPEC> for P60PfsHa {}
impl ::core::default::Default for P60PfsHa {
    #[inline(always)]
    fn default() -> P60PfsHa {
        <crate::RegValueT<P60PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsBy_SPEC;
impl crate::sealed::RegSpec for P60PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P60%s Pin Function Control Register"]
pub type P60PfsBy = crate::RegValueT<P60PfsBy_SPEC>;

impl NoBitfieldReg<P60PfsBy_SPEC> for P60PfsBy {}
impl ::core::default::Default for P60PfsBy {
    #[inline(always)]
    fn default() -> P60PfsBy {
        <crate::RegValueT<P60PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6Pfs_SPEC;
impl crate::sealed::RegSpec for P6Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P6%s Pin Function Control Register"]
pub type P6Pfs = crate::RegValueT<P6Pfs_SPEC>;

impl NoBitfieldReg<P6Pfs_SPEC> for P6Pfs {}
impl ::core::default::Default for P6Pfs {
    #[inline(always)]
    fn default() -> P6Pfs {
        <crate::RegValueT<P6Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6PfsHa_SPEC;
impl crate::sealed::RegSpec for P6PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P6%s Pin Function Control Register"]
pub type P6PfsHa = crate::RegValueT<P6PfsHa_SPEC>;

impl NoBitfieldReg<P6PfsHa_SPEC> for P6PfsHa {}
impl ::core::default::Default for P6PfsHa {
    #[inline(always)]
    fn default() -> P6PfsHa {
        <crate::RegValueT<P6PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6PfsBy_SPEC;
impl crate::sealed::RegSpec for P6PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P6%s Pin Function Control Register"]
pub type P6PfsBy = crate::RegValueT<P6PfsBy_SPEC>;

impl NoBitfieldReg<P6PfsBy_SPEC> for P6PfsBy {}
impl ::core::default::Default for P6PfsBy {
    #[inline(always)]
    fn default() -> P6PfsBy {
        <crate::RegValueT<P6PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70Pfs_SPEC;
impl crate::sealed::RegSpec for P70Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P70%s Pin Function Control Register"]
pub type P70Pfs = crate::RegValueT<P70Pfs_SPEC>;

impl NoBitfieldReg<P70Pfs_SPEC> for P70Pfs {}
impl ::core::default::Default for P70Pfs {
    #[inline(always)]
    fn default() -> P70Pfs {
        <crate::RegValueT<P70Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70PfsHa_SPEC;
impl crate::sealed::RegSpec for P70PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P70%s Pin Function Control Register"]
pub type P70PfsHa = crate::RegValueT<P70PfsHa_SPEC>;

impl NoBitfieldReg<P70PfsHa_SPEC> for P70PfsHa {}
impl ::core::default::Default for P70PfsHa {
    #[inline(always)]
    fn default() -> P70PfsHa {
        <crate::RegValueT<P70PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70PfsBy_SPEC;
impl crate::sealed::RegSpec for P70PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P70%s Pin Function Control Register"]
pub type P70PfsBy = crate::RegValueT<P70PfsBy_SPEC>;

impl NoBitfieldReg<P70PfsBy_SPEC> for P70PfsBy {}
impl ::core::default::Default for P70PfsBy {
    #[inline(always)]
    fn default() -> P70PfsBy {
        <crate::RegValueT<P70PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7Pfs_SPEC;
impl crate::sealed::RegSpec for P7Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P7%s Pin Function Control Register"]
pub type P7Pfs = crate::RegValueT<P7Pfs_SPEC>;

impl NoBitfieldReg<P7Pfs_SPEC> for P7Pfs {}
impl ::core::default::Default for P7Pfs {
    #[inline(always)]
    fn default() -> P7Pfs {
        <crate::RegValueT<P7Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7PfsHa_SPEC;
impl crate::sealed::RegSpec for P7PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P7%s Pin Function Control Register"]
pub type P7PfsHa = crate::RegValueT<P7PfsHa_SPEC>;

impl NoBitfieldReg<P7PfsHa_SPEC> for P7PfsHa {}
impl ::core::default::Default for P7PfsHa {
    #[inline(always)]
    fn default() -> P7PfsHa {
        <crate::RegValueT<P7PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7PfsBy_SPEC;
impl crate::sealed::RegSpec for P7PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P7%s Pin Function Control Register"]
pub type P7PfsBy = crate::RegValueT<P7PfsBy_SPEC>;

impl NoBitfieldReg<P7PfsBy_SPEC> for P7PfsBy {}
impl ::core::default::Default for P7PfsBy {
    #[inline(always)]
    fn default() -> P7PfsBy {
        <crate::RegValueT<P7PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80Pfs_SPEC;
impl crate::sealed::RegSpec for P80Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P80%s Pin Function Control Register"]
pub type P80Pfs = crate::RegValueT<P80Pfs_SPEC>;

impl NoBitfieldReg<P80Pfs_SPEC> for P80Pfs {}
impl ::core::default::Default for P80Pfs {
    #[inline(always)]
    fn default() -> P80Pfs {
        <crate::RegValueT<P80Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsHa_SPEC;
impl crate::sealed::RegSpec for P80PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P80%s Pin Function Control Register"]
pub type P80PfsHa = crate::RegValueT<P80PfsHa_SPEC>;

impl NoBitfieldReg<P80PfsHa_SPEC> for P80PfsHa {}
impl ::core::default::Default for P80PfsHa {
    #[inline(always)]
    fn default() -> P80PfsHa {
        <crate::RegValueT<P80PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsBy_SPEC;
impl crate::sealed::RegSpec for P80PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P80%s Pin Function Control Register"]
pub type P80PfsBy = crate::RegValueT<P80PfsBy_SPEC>;

impl NoBitfieldReg<P80PfsBy_SPEC> for P80PfsBy {}
impl ::core::default::Default for P80PfsBy {
    #[inline(always)]
    fn default() -> P80PfsBy {
        <crate::RegValueT<P80PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8Pfs_SPEC;
impl crate::sealed::RegSpec for P8Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P8%s Pin Function Control Register"]
pub type P8Pfs = crate::RegValueT<P8Pfs_SPEC>;

impl NoBitfieldReg<P8Pfs_SPEC> for P8Pfs {}
impl ::core::default::Default for P8Pfs {
    #[inline(always)]
    fn default() -> P8Pfs {
        <crate::RegValueT<P8Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8PfsHa_SPEC;
impl crate::sealed::RegSpec for P8PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P8%s Pin Function Control Register"]
pub type P8PfsHa = crate::RegValueT<P8PfsHa_SPEC>;

impl NoBitfieldReg<P8PfsHa_SPEC> for P8PfsHa {}
impl ::core::default::Default for P8PfsHa {
    #[inline(always)]
    fn default() -> P8PfsHa {
        <crate::RegValueT<P8PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8PfsBy_SPEC;
impl crate::sealed::RegSpec for P8PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P8%s Pin Function Control Register"]
pub type P8PfsBy = crate::RegValueT<P8PfsBy_SPEC>;

impl NoBitfieldReg<P8PfsBy_SPEC> for P8PfsBy {}
impl ::core::default::Default for P8PfsBy {
    #[inline(always)]
    fn default() -> P8PfsBy {
        <crate::RegValueT<P8PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90Pfs_SPEC;
impl crate::sealed::RegSpec for P90Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P90%s Pin Function Control Register"]
pub type P90Pfs = crate::RegValueT<P90Pfs_SPEC>;

impl NoBitfieldReg<P90Pfs_SPEC> for P90Pfs {}
impl ::core::default::Default for P90Pfs {
    #[inline(always)]
    fn default() -> P90Pfs {
        <crate::RegValueT<P90Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90PfsHa_SPEC;
impl crate::sealed::RegSpec for P90PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P90%s Pin Function Control Register"]
pub type P90PfsHa = crate::RegValueT<P90PfsHa_SPEC>;

impl NoBitfieldReg<P90PfsHa_SPEC> for P90PfsHa {}
impl ::core::default::Default for P90PfsHa {
    #[inline(always)]
    fn default() -> P90PfsHa {
        <crate::RegValueT<P90PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90PfsBy_SPEC;
impl crate::sealed::RegSpec for P90PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P90%s Pin Function Control Register"]
pub type P90PfsBy = crate::RegValueT<P90PfsBy_SPEC>;

impl NoBitfieldReg<P90PfsBy_SPEC> for P90PfsBy {}
impl ::core::default::Default for P90PfsBy {
    #[inline(always)]
    fn default() -> P90PfsBy {
        <crate::RegValueT<P90PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9Pfs_SPEC;
impl crate::sealed::RegSpec for P9Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P9%s Pin Function Control Register"]
pub type P9Pfs = crate::RegValueT<P9Pfs_SPEC>;

impl NoBitfieldReg<P9Pfs_SPEC> for P9Pfs {}
impl ::core::default::Default for P9Pfs {
    #[inline(always)]
    fn default() -> P9Pfs {
        <crate::RegValueT<P9Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9PfsHa_SPEC;
impl crate::sealed::RegSpec for P9PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P9%s Pin Function Control Register"]
pub type P9PfsHa = crate::RegValueT<P9PfsHa_SPEC>;

impl NoBitfieldReg<P9PfsHa_SPEC> for P9PfsHa {}
impl ::core::default::Default for P9PfsHa {
    #[inline(always)]
    fn default() -> P9PfsHa {
        <crate::RegValueT<P9PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9PfsBy_SPEC;
impl crate::sealed::RegSpec for P9PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P9%s Pin Function Control Register"]
pub type P9PfsBy = crate::RegValueT<P9PfsBy_SPEC>;

impl NoBitfieldReg<P9PfsBy_SPEC> for P9PfsBy {}
impl ::core::default::Default for P9PfsBy {
    #[inline(always)]
    fn default() -> P9PfsBy {
        <crate::RegValueT<P9PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0Pfs_SPEC;
impl crate::sealed::RegSpec for Pa0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "PA0%s Pin Function Control Register"]
pub type Pa0Pfs = crate::RegValueT<Pa0Pfs_SPEC>;

impl NoBitfieldReg<Pa0Pfs_SPEC> for Pa0Pfs {}
impl ::core::default::Default for Pa0Pfs {
    #[inline(always)]
    fn default() -> Pa0Pfs {
        <crate::RegValueT<Pa0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pa0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "PA0%s Pin Function Control Register"]
pub type Pa0PfsHa = crate::RegValueT<Pa0PfsHa_SPEC>;

impl NoBitfieldReg<Pa0PfsHa_SPEC> for Pa0PfsHa {}
impl ::core::default::Default for Pa0PfsHa {
    #[inline(always)]
    fn default() -> Pa0PfsHa {
        <crate::RegValueT<Pa0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pa0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "PA0%s Pin Function Control Register"]
pub type Pa0PfsBy = crate::RegValueT<Pa0PfsBy_SPEC>;

impl NoBitfieldReg<Pa0PfsBy_SPEC> for Pa0PfsBy {}
impl ::core::default::Default for Pa0PfsBy {
    #[inline(always)]
    fn default() -> Pa0PfsBy {
        <crate::RegValueT<Pa0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Papfs_SPEC;
impl crate::sealed::RegSpec for Papfs_SPEC {
    type DataType = u32;
}

#[doc = "PA%s Pin Function Control Register"]
pub type Papfs = crate::RegValueT<Papfs_SPEC>;

impl NoBitfieldReg<Papfs_SPEC> for Papfs {}
impl ::core::default::Default for Papfs {
    #[inline(always)]
    fn default() -> Papfs {
        <crate::RegValueT<Papfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PapfsHa_SPEC;
impl crate::sealed::RegSpec for PapfsHa_SPEC {
    type DataType = u16;
}

#[doc = "PA%s Pin Function Control Register"]
pub type PapfsHa = crate::RegValueT<PapfsHa_SPEC>;

impl NoBitfieldReg<PapfsHa_SPEC> for PapfsHa {}
impl ::core::default::Default for PapfsHa {
    #[inline(always)]
    fn default() -> PapfsHa {
        <crate::RegValueT<PapfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PapfsBy_SPEC;
impl crate::sealed::RegSpec for PapfsBy_SPEC {
    type DataType = u8;
}

#[doc = "PA%s Pin Function Control Register"]
pub type PapfsBy = crate::RegValueT<PapfsBy_SPEC>;

impl NoBitfieldReg<PapfsBy_SPEC> for PapfsBy {}
impl ::core::default::Default for PapfsBy {
    #[inline(always)]
    fn default() -> PapfsBy {
        <crate::RegValueT<PapfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0Pfs_SPEC;
impl crate::sealed::RegSpec for Pb0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "PB0%s Pin Function Control Register"]
pub type Pb0Pfs = crate::RegValueT<Pb0Pfs_SPEC>;

impl NoBitfieldReg<Pb0Pfs_SPEC> for Pb0Pfs {}
impl ::core::default::Default for Pb0Pfs {
    #[inline(always)]
    fn default() -> Pb0Pfs {
        <crate::RegValueT<Pb0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "PB0%s Pin Function Control Register"]
pub type Pb0PfsHa = crate::RegValueT<Pb0PfsHa_SPEC>;

impl NoBitfieldReg<Pb0PfsHa_SPEC> for Pb0PfsHa {}
impl ::core::default::Default for Pb0PfsHa {
    #[inline(always)]
    fn default() -> Pb0PfsHa {
        <crate::RegValueT<Pb0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "PB0%s Pin Function Control Register"]
pub type Pb0PfsBy = crate::RegValueT<Pb0PfsBy_SPEC>;

impl NoBitfieldReg<Pb0PfsBy_SPEC> for Pb0PfsBy {}
impl ::core::default::Default for Pb0PfsBy {
    #[inline(always)]
    fn default() -> Pb0PfsBy {
        <crate::RegValueT<Pb0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pbpfs_SPEC;
impl crate::sealed::RegSpec for Pbpfs_SPEC {
    type DataType = u32;
}

#[doc = "PB%s Pin Function Control Register"]
pub type Pbpfs = crate::RegValueT<Pbpfs_SPEC>;

impl NoBitfieldReg<Pbpfs_SPEC> for Pbpfs {}
impl ::core::default::Default for Pbpfs {
    #[inline(always)]
    fn default() -> Pbpfs {
        <crate::RegValueT<Pbpfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PbpfsHa_SPEC;
impl crate::sealed::RegSpec for PbpfsHa_SPEC {
    type DataType = u16;
}

#[doc = "PB%s Pin Function Control Register"]
pub type PbpfsHa = crate::RegValueT<PbpfsHa_SPEC>;

impl NoBitfieldReg<PbpfsHa_SPEC> for PbpfsHa {}
impl ::core::default::Default for PbpfsHa {
    #[inline(always)]
    fn default() -> PbpfsHa {
        <crate::RegValueT<PbpfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PbpfsBy_SPEC;
impl crate::sealed::RegSpec for PbpfsBy_SPEC {
    type DataType = u8;
}

#[doc = "PB%s Pin Function Control Register"]
pub type PbpfsBy = crate::RegValueT<PbpfsBy_SPEC>;

impl NoBitfieldReg<PbpfsBy_SPEC> for PbpfsBy {}
impl ::core::default::Default for PbpfsBy {
    #[inline(always)]
    fn default() -> PbpfsBy {
        <crate::RegValueT<PbpfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfenet_SPEC;
impl crate::sealed::RegSpec for Pfenet_SPEC {
    type DataType = u8;
}

#[doc = "Ethernet Control Register"]
pub type Pfenet = crate::RegValueT<Pfenet_SPEC>;

impl Pfenet {
    #[doc = "Ethernet Mode Setting ch0"]
    #[inline(always)]
    pub fn phymode0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pfenet::Phymode0,
        pfenet::Phymode0,
        Pfenet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pfenet::Phymode0,
            pfenet::Phymode0,
            Pfenet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Pfenet_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Pfenet_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pfenet {
    #[inline(always)]
    fn default() -> Pfenet {
        <crate::RegValueT<Pfenet_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pfenet {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phymode0_SPEC;
    pub type Phymode0 = crate::EnumBitfieldStruct<u8, Phymode0_SPEC>;
    impl Phymode0 {
        #[doc = "RMII mode (ETHERC channel 0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "MII mode (ETHERC channel 0)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwprNs_SPEC;
impl crate::sealed::RegSpec for PwprNs_SPEC {
    type DataType = u8;
}

#[doc = "NonSecure Write-Protect Register"]
pub type PwprNs = crate::RegValueT<PwprNs_SPEC>;

impl PwprNs {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, PwprNs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,PwprNs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PFS Register Write Enable"]
    #[inline(always)]
    pub fn pfswe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pwpr_ns::Pfswe,
        pwpr_ns::Pfswe,
        PwprNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pwpr_ns::Pfswe,
            pwpr_ns::Pfswe,
            PwprNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFSWE Bit Write Disable"]
    #[inline(always)]
    pub fn b0wi(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pwpr_ns::B0Wi,
        pwpr_ns::B0Wi,
        PwprNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pwpr_ns::B0Wi,
            pwpr_ns::B0Wi,
            PwprNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PwprNs {
    #[inline(always)]
    fn default() -> PwprNs {
        <crate::RegValueT<PwprNs_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod pwpr_ns {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfswe_SPEC;
    pub type Pfswe = crate::EnumBitfieldStruct<u8, Pfswe_SPEC>;
    impl Pfswe {
        #[doc = "Writing to the PFS register is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the PFS register is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B0Wi_SPEC;
    pub type B0Wi = crate::EnumBitfieldStruct<u8, B0Wi_SPEC>;
    impl B0Wi {
        #[doc = "Writing to the PFSWE bit is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the PFSWE bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwprS_SPEC;
impl crate::sealed::RegSpec for PwprS_SPEC {
    type DataType = u8;
}

#[doc = "Secure Write Protect Register"]
pub type PwprS = crate::RegValueT<PwprS_SPEC>;

impl PwprS {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, PwprS_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,PwprS_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PmnPFS Register Write Enable"]
    #[inline(always)]
    pub fn pfswe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pwpr_s::Pfswe,
        pwpr_s::Pfswe,
        PwprS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pwpr_s::Pfswe,
            pwpr_s::Pfswe,
            PwprS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFSWE Bit Write Disable"]
    #[inline(always)]
    pub fn b0wi(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pwpr_s::B0Wi,
        pwpr_s::B0Wi,
        PwprS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pwpr_s::B0Wi,
            pwpr_s::B0Wi,
            PwprS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PwprS {
    #[inline(always)]
    fn default() -> PwprS {
        <crate::RegValueT<PwprS_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod pwpr_s {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfswe_SPEC;
    pub type Pfswe = crate::EnumBitfieldStruct<u8, Pfswe_SPEC>;
    impl Pfswe {
        #[doc = "Disable writes to the PmnPFS register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to the PmnPFS register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B0Wi_SPEC;
    pub type B0Wi = crate::EnumBitfieldStruct<u8, B0Wi_SPEC>;
    impl B0Wi {
        #[doc = "Enable writes the PFSWE bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable writes to the PFSWE bit."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi3C_SPEC;
impl crate::sealed::RegSpec for Pfi3C_SPEC {
    type DataType = u8;
}

#[doc = "I3C Control Register"]
pub type Pfi3C = crate::RegValueT<Pfi3C_SPEC>;

impl Pfi3C {
    #[doc = "RI3C I3C Mode slope control setting bit"]
    #[inline(always)]
    pub fn i3cslope0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pfi3c::I3Cslope0,
        pfi3c::I3Cslope0,
        Pfi3C_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pfi3c::I3Cslope0,
            pfi3c::I3Cslope0,
            Pfi3C_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Pfi3C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Pfi3C_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pfi3C {
    #[inline(always)]
    fn default() -> Pfi3C {
        <crate::RegValueT<Pfi3C_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pfi3c {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Cslope0_SPEC;
    pub type I3Cslope0 = crate::EnumBitfieldStruct<u8, I3Cslope0_SPEC>;
    impl I3Cslope0 {
        #[doc = "I3C mode slope control disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "I3C mode slope control enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0Sar_SPEC;
impl crate::sealed::RegSpec for P0Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 0"]
pub type P0Sar = crate::RegValueT<P0Sar_SPEC>;

impl P0Sar {
    #[doc = "P000 Security Attribute"]
    #[inline(always)]
    pub fn p000sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p0sar::P000Sa,
        p0sar::P000Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p0sar::P000Sa,
            p0sar::P000Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P001 Security Attribute"]
    #[inline(always)]
    pub fn p001sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p0sar::P001Sa,
        p0sar::P001Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p0sar::P001Sa,
            p0sar::P001Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P002 Security Attribute"]
    #[inline(always)]
    pub fn p002sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p0sar::P002Sa,
        p0sar::P002Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p0sar::P002Sa,
            p0sar::P002Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P003 Security Attribute"]
    #[inline(always)]
    pub fn p003sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p0sar::P003Sa,
        p0sar::P003Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p0sar::P003Sa,
            p0sar::P003Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P004 Security Attribute"]
    #[inline(always)]
    pub fn p004sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p0sar::P004Sa,
        p0sar::P004Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p0sar::P004Sa,
            p0sar::P004Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P005 Security Attribute"]
    #[inline(always)]
    pub fn p005sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p0sar::P005Sa,
        p0sar::P005Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p0sar::P005Sa,
            p0sar::P005Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P006 Security Attribute"]
    #[inline(always)]
    pub fn p006sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p0sar::P006Sa,
        p0sar::P006Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p0sar::P006Sa,
            p0sar::P006Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P007 Security Attribute"]
    #[inline(always)]
    pub fn p007sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p0sar::P007Sa,
        p0sar::P007Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p0sar::P007Sa,
            p0sar::P007Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P008 Security Attribute"]
    #[inline(always)]
    pub fn p008sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p0sar::P008Sa,
        p0sar::P008Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p0sar::P008Sa,
            p0sar::P008Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P009 Security Attribute"]
    #[inline(always)]
    pub fn p009sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p0sar::P009Sa,
        p0sar::P009Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p0sar::P009Sa,
            p0sar::P009Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P010 Security Attribute"]
    #[inline(always)]
    pub fn p010sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p0sar::P010Sa,
        p0sar::P010Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p0sar::P010Sa,
            p0sar::P010Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P011 Security Attribute"]
    #[inline(always)]
    pub fn p011sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p0sar::P011Sa,
        p0sar::P011Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p0sar::P011Sa,
            p0sar::P011Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P012 Security Attribute"]
    #[inline(always)]
    pub fn p012sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p0sar::P012Sa,
        p0sar::P012Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p0sar::P012Sa,
            p0sar::P012Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P013 Security Attribute"]
    #[inline(always)]
    pub fn p013sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p0sar::P013Sa,
        p0sar::P013Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p0sar::P013Sa,
            p0sar::P013Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P014 Security Attribute"]
    #[inline(always)]
    pub fn p014sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p0sar::P014Sa,
        p0sar::P014Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p0sar::P014Sa,
            p0sar::P014Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P015 Security Attribute"]
    #[inline(always)]
    pub fn p015sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p0sar::P015Sa,
        p0sar::P015Sa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p0sar::P015Sa,
            p0sar::P015Sa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0Sar {
    #[inline(always)]
    fn default() -> P0Sar {
        <crate::RegValueT<P0Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p0sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P000Sa_SPEC;
    pub type P000Sa = crate::EnumBitfieldStruct<u8, P000Sa_SPEC>;
    impl P000Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P001Sa_SPEC;
    pub type P001Sa = crate::EnumBitfieldStruct<u8, P001Sa_SPEC>;
    impl P001Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P002Sa_SPEC;
    pub type P002Sa = crate::EnumBitfieldStruct<u8, P002Sa_SPEC>;
    impl P002Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P003Sa_SPEC;
    pub type P003Sa = crate::EnumBitfieldStruct<u8, P003Sa_SPEC>;
    impl P003Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P004Sa_SPEC;
    pub type P004Sa = crate::EnumBitfieldStruct<u8, P004Sa_SPEC>;
    impl P004Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P005Sa_SPEC;
    pub type P005Sa = crate::EnumBitfieldStruct<u8, P005Sa_SPEC>;
    impl P005Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P006Sa_SPEC;
    pub type P006Sa = crate::EnumBitfieldStruct<u8, P006Sa_SPEC>;
    impl P006Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P007Sa_SPEC;
    pub type P007Sa = crate::EnumBitfieldStruct<u8, P007Sa_SPEC>;
    impl P007Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P008Sa_SPEC;
    pub type P008Sa = crate::EnumBitfieldStruct<u8, P008Sa_SPEC>;
    impl P008Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P009Sa_SPEC;
    pub type P009Sa = crate::EnumBitfieldStruct<u8, P009Sa_SPEC>;
    impl P009Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P010Sa_SPEC;
    pub type P010Sa = crate::EnumBitfieldStruct<u8, P010Sa_SPEC>;
    impl P010Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P011Sa_SPEC;
    pub type P011Sa = crate::EnumBitfieldStruct<u8, P011Sa_SPEC>;
    impl P011Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P012Sa_SPEC;
    pub type P012Sa = crate::EnumBitfieldStruct<u8, P012Sa_SPEC>;
    impl P012Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P013Sa_SPEC;
    pub type P013Sa = crate::EnumBitfieldStruct<u8, P013Sa_SPEC>;
    impl P013Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P014Sa_SPEC;
    pub type P014Sa = crate::EnumBitfieldStruct<u8, P014Sa_SPEC>;
    impl P014Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P015Sa_SPEC;
    pub type P015Sa = crate::EnumBitfieldStruct<u8, P015Sa_SPEC>;
    impl P015Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1Sar_SPEC;
impl crate::sealed::RegSpec for P1Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P1Sar = crate::RegValueT<P1Sar_SPEC>;

impl P1Sar {
    #[doc = "P100 Security Attribute"]
    #[inline(always)]
    pub fn p100sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p1sar::P100Sa,
        p1sar::P100Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p1sar::P100Sa,
            p1sar::P100Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P101 Security Attribute"]
    #[inline(always)]
    pub fn p101sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p1sar::P101Sa,
        p1sar::P101Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p1sar::P101Sa,
            p1sar::P101Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P102 Security Attribute"]
    #[inline(always)]
    pub fn p102sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p1sar::P102Sa,
        p1sar::P102Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p1sar::P102Sa,
            p1sar::P102Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P103 Security Attribute"]
    #[inline(always)]
    pub fn p103sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p1sar::P103Sa,
        p1sar::P103Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p1sar::P103Sa,
            p1sar::P103Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P104 Security Attribute"]
    #[inline(always)]
    pub fn p104sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p1sar::P104Sa,
        p1sar::P104Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p1sar::P104Sa,
            p1sar::P104Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P105 Security Attribute"]
    #[inline(always)]
    pub fn p105sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p1sar::P105Sa,
        p1sar::P105Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p1sar::P105Sa,
            p1sar::P105Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P106 Security Attribute"]
    #[inline(always)]
    pub fn p106sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p1sar::P106Sa,
        p1sar::P106Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p1sar::P106Sa,
            p1sar::P106Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P107 Security Attribute"]
    #[inline(always)]
    pub fn p107sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p1sar::P107Sa,
        p1sar::P107Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p1sar::P107Sa,
            p1sar::P107Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P108 Security Attribute"]
    #[inline(always)]
    pub fn p108sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p1sar::P108Sa,
        p1sar::P108Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p1sar::P108Sa,
            p1sar::P108Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P109 Security Attribute"]
    #[inline(always)]
    pub fn p109sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p1sar::P109Sa,
        p1sar::P109Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p1sar::P109Sa,
            p1sar::P109Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P110 Security Attribute"]
    #[inline(always)]
    pub fn p110sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p1sar::P110Sa,
        p1sar::P110Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p1sar::P110Sa,
            p1sar::P110Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P111 Security Attribute"]
    #[inline(always)]
    pub fn p111sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p1sar::P111Sa,
        p1sar::P111Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p1sar::P111Sa,
            p1sar::P111Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P112 Security Attribute"]
    #[inline(always)]
    pub fn p112sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p1sar::P112Sa,
        p1sar::P112Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p1sar::P112Sa,
            p1sar::P112Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P113 Security Attribute"]
    #[inline(always)]
    pub fn p113sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p1sar::P113Sa,
        p1sar::P113Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p1sar::P113Sa,
            p1sar::P113Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P114 Security Attribute"]
    #[inline(always)]
    pub fn p114sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p1sar::P114Sa,
        p1sar::P114Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p1sar::P114Sa,
            p1sar::P114Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P115 Security Attribute"]
    #[inline(always)]
    pub fn p115sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p1sar::P115Sa,
        p1sar::P115Sa,
        P1Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p1sar::P115Sa,
            p1sar::P115Sa,
            P1Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1Sar {
    #[inline(always)]
    fn default() -> P1Sar {
        <crate::RegValueT<P1Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p1sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P100Sa_SPEC;
    pub type P100Sa = crate::EnumBitfieldStruct<u8, P100Sa_SPEC>;
    impl P100Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P101Sa_SPEC;
    pub type P101Sa = crate::EnumBitfieldStruct<u8, P101Sa_SPEC>;
    impl P101Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P102Sa_SPEC;
    pub type P102Sa = crate::EnumBitfieldStruct<u8, P102Sa_SPEC>;
    impl P102Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P103Sa_SPEC;
    pub type P103Sa = crate::EnumBitfieldStruct<u8, P103Sa_SPEC>;
    impl P103Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P104Sa_SPEC;
    pub type P104Sa = crate::EnumBitfieldStruct<u8, P104Sa_SPEC>;
    impl P104Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P105Sa_SPEC;
    pub type P105Sa = crate::EnumBitfieldStruct<u8, P105Sa_SPEC>;
    impl P105Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P106Sa_SPEC;
    pub type P106Sa = crate::EnumBitfieldStruct<u8, P106Sa_SPEC>;
    impl P106Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P107Sa_SPEC;
    pub type P107Sa = crate::EnumBitfieldStruct<u8, P107Sa_SPEC>;
    impl P107Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P108Sa_SPEC;
    pub type P108Sa = crate::EnumBitfieldStruct<u8, P108Sa_SPEC>;
    impl P108Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P109Sa_SPEC;
    pub type P109Sa = crate::EnumBitfieldStruct<u8, P109Sa_SPEC>;
    impl P109Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P110Sa_SPEC;
    pub type P110Sa = crate::EnumBitfieldStruct<u8, P110Sa_SPEC>;
    impl P110Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P111Sa_SPEC;
    pub type P111Sa = crate::EnumBitfieldStruct<u8, P111Sa_SPEC>;
    impl P111Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P112Sa_SPEC;
    pub type P112Sa = crate::EnumBitfieldStruct<u8, P112Sa_SPEC>;
    impl P112Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P113Sa_SPEC;
    pub type P113Sa = crate::EnumBitfieldStruct<u8, P113Sa_SPEC>;
    impl P113Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P114Sa_SPEC;
    pub type P114Sa = crate::EnumBitfieldStruct<u8, P114Sa_SPEC>;
    impl P114Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P115Sa_SPEC;
    pub type P115Sa = crate::EnumBitfieldStruct<u8, P115Sa_SPEC>;
    impl P115Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2Sar_SPEC;
impl crate::sealed::RegSpec for P2Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P2Sar = crate::RegValueT<P2Sar_SPEC>;

impl P2Sar {
    #[doc = "P200 Security Attribute"]
    #[inline(always)]
    pub fn p200sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p2sar::P200Sa,
        p2sar::P200Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p2sar::P200Sa,
            p2sar::P200Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P201 Security Attribute"]
    #[inline(always)]
    pub fn p201sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p2sar::P201Sa,
        p2sar::P201Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p2sar::P201Sa,
            p2sar::P201Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P202 Security Attribute"]
    #[inline(always)]
    pub fn p202sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p2sar::P202Sa,
        p2sar::P202Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p2sar::P202Sa,
            p2sar::P202Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P203 Security Attribute"]
    #[inline(always)]
    pub fn p203sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p2sar::P203Sa,
        p2sar::P203Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p2sar::P203Sa,
            p2sar::P203Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P204 Security Attribute"]
    #[inline(always)]
    pub fn p204sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p2sar::P204Sa,
        p2sar::P204Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p2sar::P204Sa,
            p2sar::P204Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P205 Security Attribute"]
    #[inline(always)]
    pub fn p205sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p2sar::P205Sa,
        p2sar::P205Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p2sar::P205Sa,
            p2sar::P205Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P206 Security Attribute"]
    #[inline(always)]
    pub fn p206sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p2sar::P206Sa,
        p2sar::P206Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p2sar::P206Sa,
            p2sar::P206Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P207 Security Attribute"]
    #[inline(always)]
    pub fn p207sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p2sar::P207Sa,
        p2sar::P207Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p2sar::P207Sa,
            p2sar::P207Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P208 Security Attribute"]
    #[inline(always)]
    pub fn p208sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p2sar::P208Sa,
        p2sar::P208Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p2sar::P208Sa,
            p2sar::P208Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P209 Security Attribute"]
    #[inline(always)]
    pub fn p209sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p2sar::P209Sa,
        p2sar::P209Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p2sar::P209Sa,
            p2sar::P209Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P210 Security Attribute"]
    #[inline(always)]
    pub fn p210sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p2sar::P210Sa,
        p2sar::P210Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p2sar::P210Sa,
            p2sar::P210Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P211 Security Attribute"]
    #[inline(always)]
    pub fn p211sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p2sar::P211Sa,
        p2sar::P211Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p2sar::P211Sa,
            p2sar::P211Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P212 Security Attribute"]
    #[inline(always)]
    pub fn p212sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p2sar::P212Sa,
        p2sar::P212Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p2sar::P212Sa,
            p2sar::P212Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P213 Security Attribute"]
    #[inline(always)]
    pub fn p213sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p2sar::P213Sa,
        p2sar::P213Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p2sar::P213Sa,
            p2sar::P213Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P214 Security Attribute"]
    #[inline(always)]
    pub fn p214sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p2sar::P214Sa,
        p2sar::P214Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p2sar::P214Sa,
            p2sar::P214Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P215 Security Attribute"]
    #[inline(always)]
    pub fn p215sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p2sar::P215Sa,
        p2sar::P215Sa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p2sar::P215Sa,
            p2sar::P215Sa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2Sar {
    #[inline(always)]
    fn default() -> P2Sar {
        <crate::RegValueT<P2Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p2sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P200Sa_SPEC;
    pub type P200Sa = crate::EnumBitfieldStruct<u8, P200Sa_SPEC>;
    impl P200Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P201Sa_SPEC;
    pub type P201Sa = crate::EnumBitfieldStruct<u8, P201Sa_SPEC>;
    impl P201Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P202Sa_SPEC;
    pub type P202Sa = crate::EnumBitfieldStruct<u8, P202Sa_SPEC>;
    impl P202Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P203Sa_SPEC;
    pub type P203Sa = crate::EnumBitfieldStruct<u8, P203Sa_SPEC>;
    impl P203Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P204Sa_SPEC;
    pub type P204Sa = crate::EnumBitfieldStruct<u8, P204Sa_SPEC>;
    impl P204Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P205Sa_SPEC;
    pub type P205Sa = crate::EnumBitfieldStruct<u8, P205Sa_SPEC>;
    impl P205Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P206Sa_SPEC;
    pub type P206Sa = crate::EnumBitfieldStruct<u8, P206Sa_SPEC>;
    impl P206Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P207Sa_SPEC;
    pub type P207Sa = crate::EnumBitfieldStruct<u8, P207Sa_SPEC>;
    impl P207Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P208Sa_SPEC;
    pub type P208Sa = crate::EnumBitfieldStruct<u8, P208Sa_SPEC>;
    impl P208Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P209Sa_SPEC;
    pub type P209Sa = crate::EnumBitfieldStruct<u8, P209Sa_SPEC>;
    impl P209Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P210Sa_SPEC;
    pub type P210Sa = crate::EnumBitfieldStruct<u8, P210Sa_SPEC>;
    impl P210Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P211Sa_SPEC;
    pub type P211Sa = crate::EnumBitfieldStruct<u8, P211Sa_SPEC>;
    impl P211Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P212Sa_SPEC;
    pub type P212Sa = crate::EnumBitfieldStruct<u8, P212Sa_SPEC>;
    impl P212Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P213Sa_SPEC;
    pub type P213Sa = crate::EnumBitfieldStruct<u8, P213Sa_SPEC>;
    impl P213Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P214Sa_SPEC;
    pub type P214Sa = crate::EnumBitfieldStruct<u8, P214Sa_SPEC>;
    impl P214Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P215Sa_SPEC;
    pub type P215Sa = crate::EnumBitfieldStruct<u8, P215Sa_SPEC>;
    impl P215Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3Sar_SPEC;
impl crate::sealed::RegSpec for P3Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P3Sar = crate::RegValueT<P3Sar_SPEC>;

impl P3Sar {
    #[doc = "P300 Security Attribute"]
    #[inline(always)]
    pub fn p300sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p3sar::P300Sa,
        p3sar::P300Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p3sar::P300Sa,
            p3sar::P300Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P301 Security Attribute"]
    #[inline(always)]
    pub fn p301sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p3sar::P301Sa,
        p3sar::P301Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p3sar::P301Sa,
            p3sar::P301Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P302 Security Attribute"]
    #[inline(always)]
    pub fn p302sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p3sar::P302Sa,
        p3sar::P302Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p3sar::P302Sa,
            p3sar::P302Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P303 Security Attribute"]
    #[inline(always)]
    pub fn p303sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p3sar::P303Sa,
        p3sar::P303Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p3sar::P303Sa,
            p3sar::P303Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P304 Security Attribute"]
    #[inline(always)]
    pub fn p304sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p3sar::P304Sa,
        p3sar::P304Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p3sar::P304Sa,
            p3sar::P304Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P305 Security Attribute"]
    #[inline(always)]
    pub fn p305sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p3sar::P305Sa,
        p3sar::P305Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p3sar::P305Sa,
            p3sar::P305Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P306 Security Attribute"]
    #[inline(always)]
    pub fn p306sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p3sar::P306Sa,
        p3sar::P306Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p3sar::P306Sa,
            p3sar::P306Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P307 Security Attribute"]
    #[inline(always)]
    pub fn p307sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p3sar::P307Sa,
        p3sar::P307Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p3sar::P307Sa,
            p3sar::P307Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P308 Security Attribute"]
    #[inline(always)]
    pub fn p308sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p3sar::P308Sa,
        p3sar::P308Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p3sar::P308Sa,
            p3sar::P308Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P309 Security Attribute"]
    #[inline(always)]
    pub fn p309sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p3sar::P309Sa,
        p3sar::P309Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p3sar::P309Sa,
            p3sar::P309Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P310 Security Attribute"]
    #[inline(always)]
    pub fn p310sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p3sar::P310Sa,
        p3sar::P310Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p3sar::P310Sa,
            p3sar::P310Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P311 Security Attribute"]
    #[inline(always)]
    pub fn p311sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p3sar::P311Sa,
        p3sar::P311Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p3sar::P311Sa,
            p3sar::P311Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P312 Security Attribute"]
    #[inline(always)]
    pub fn p312sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p3sar::P312Sa,
        p3sar::P312Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p3sar::P312Sa,
            p3sar::P312Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P313 Security Attribute"]
    #[inline(always)]
    pub fn p313sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p3sar::P313Sa,
        p3sar::P313Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p3sar::P313Sa,
            p3sar::P313Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P314 Security Attribute"]
    #[inline(always)]
    pub fn p314sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p3sar::P314Sa,
        p3sar::P314Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p3sar::P314Sa,
            p3sar::P314Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P315 Security Attribute"]
    #[inline(always)]
    pub fn p315sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p3sar::P315Sa,
        p3sar::P315Sa,
        P3Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p3sar::P315Sa,
            p3sar::P315Sa,
            P3Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P3Sar {
    #[inline(always)]
    fn default() -> P3Sar {
        <crate::RegValueT<P3Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p3sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P300Sa_SPEC;
    pub type P300Sa = crate::EnumBitfieldStruct<u8, P300Sa_SPEC>;
    impl P300Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P301Sa_SPEC;
    pub type P301Sa = crate::EnumBitfieldStruct<u8, P301Sa_SPEC>;
    impl P301Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P302Sa_SPEC;
    pub type P302Sa = crate::EnumBitfieldStruct<u8, P302Sa_SPEC>;
    impl P302Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P303Sa_SPEC;
    pub type P303Sa = crate::EnumBitfieldStruct<u8, P303Sa_SPEC>;
    impl P303Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P304Sa_SPEC;
    pub type P304Sa = crate::EnumBitfieldStruct<u8, P304Sa_SPEC>;
    impl P304Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P305Sa_SPEC;
    pub type P305Sa = crate::EnumBitfieldStruct<u8, P305Sa_SPEC>;
    impl P305Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P306Sa_SPEC;
    pub type P306Sa = crate::EnumBitfieldStruct<u8, P306Sa_SPEC>;
    impl P306Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P307Sa_SPEC;
    pub type P307Sa = crate::EnumBitfieldStruct<u8, P307Sa_SPEC>;
    impl P307Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P308Sa_SPEC;
    pub type P308Sa = crate::EnumBitfieldStruct<u8, P308Sa_SPEC>;
    impl P308Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P309Sa_SPEC;
    pub type P309Sa = crate::EnumBitfieldStruct<u8, P309Sa_SPEC>;
    impl P309Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P310Sa_SPEC;
    pub type P310Sa = crate::EnumBitfieldStruct<u8, P310Sa_SPEC>;
    impl P310Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P311Sa_SPEC;
    pub type P311Sa = crate::EnumBitfieldStruct<u8, P311Sa_SPEC>;
    impl P311Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P312Sa_SPEC;
    pub type P312Sa = crate::EnumBitfieldStruct<u8, P312Sa_SPEC>;
    impl P312Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P313Sa_SPEC;
    pub type P313Sa = crate::EnumBitfieldStruct<u8, P313Sa_SPEC>;
    impl P313Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P314Sa_SPEC;
    pub type P314Sa = crate::EnumBitfieldStruct<u8, P314Sa_SPEC>;
    impl P314Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P315Sa_SPEC;
    pub type P315Sa = crate::EnumBitfieldStruct<u8, P315Sa_SPEC>;
    impl P315Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4Sar_SPEC;
impl crate::sealed::RegSpec for P4Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P4Sar = crate::RegValueT<P4Sar_SPEC>;

impl P4Sar {
    #[doc = "P400 Security Attribute"]
    #[inline(always)]
    pub fn p400sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p4sar::P400Sa,
        p4sar::P400Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p4sar::P400Sa,
            p4sar::P400Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P401 Security Attribute"]
    #[inline(always)]
    pub fn p401sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p4sar::P401Sa,
        p4sar::P401Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p4sar::P401Sa,
            p4sar::P401Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P402 Security Attribute"]
    #[inline(always)]
    pub fn p402sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p4sar::P402Sa,
        p4sar::P402Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p4sar::P402Sa,
            p4sar::P402Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P403 Security Attribute"]
    #[inline(always)]
    pub fn p403sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p4sar::P403Sa,
        p4sar::P403Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p4sar::P403Sa,
            p4sar::P403Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P404 Security Attribute"]
    #[inline(always)]
    pub fn p404sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p4sar::P404Sa,
        p4sar::P404Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p4sar::P404Sa,
            p4sar::P404Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P405 Security Attribute"]
    #[inline(always)]
    pub fn p405sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p4sar::P405Sa,
        p4sar::P405Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p4sar::P405Sa,
            p4sar::P405Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P406 Security Attribute"]
    #[inline(always)]
    pub fn p406sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p4sar::P406Sa,
        p4sar::P406Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p4sar::P406Sa,
            p4sar::P406Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P407 Security Attribute"]
    #[inline(always)]
    pub fn p407sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p4sar::P407Sa,
        p4sar::P407Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p4sar::P407Sa,
            p4sar::P407Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P408 Security Attribute"]
    #[inline(always)]
    pub fn p408sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p4sar::P408Sa,
        p4sar::P408Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p4sar::P408Sa,
            p4sar::P408Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P409 Security Attribute"]
    #[inline(always)]
    pub fn p409sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p4sar::P409Sa,
        p4sar::P409Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p4sar::P409Sa,
            p4sar::P409Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P410 Security Attribute"]
    #[inline(always)]
    pub fn p410sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p4sar::P410Sa,
        p4sar::P410Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p4sar::P410Sa,
            p4sar::P410Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P411 Security Attribute"]
    #[inline(always)]
    pub fn p411sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p4sar::P411Sa,
        p4sar::P411Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p4sar::P411Sa,
            p4sar::P411Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P412 Security Attribute"]
    #[inline(always)]
    pub fn p412sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p4sar::P412Sa,
        p4sar::P412Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p4sar::P412Sa,
            p4sar::P412Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P413 Security Attribute"]
    #[inline(always)]
    pub fn p413sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p4sar::P413Sa,
        p4sar::P413Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p4sar::P413Sa,
            p4sar::P413Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P414 Security Attribute"]
    #[inline(always)]
    pub fn p414sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p4sar::P414Sa,
        p4sar::P414Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p4sar::P414Sa,
            p4sar::P414Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P415 Security Attribute"]
    #[inline(always)]
    pub fn p415sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p4sar::P415Sa,
        p4sar::P415Sa,
        P4Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p4sar::P415Sa,
            p4sar::P415Sa,
            P4Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P4Sar {
    #[inline(always)]
    fn default() -> P4Sar {
        <crate::RegValueT<P4Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p4sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P400Sa_SPEC;
    pub type P400Sa = crate::EnumBitfieldStruct<u8, P400Sa_SPEC>;
    impl P400Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P401Sa_SPEC;
    pub type P401Sa = crate::EnumBitfieldStruct<u8, P401Sa_SPEC>;
    impl P401Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P402Sa_SPEC;
    pub type P402Sa = crate::EnumBitfieldStruct<u8, P402Sa_SPEC>;
    impl P402Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P403Sa_SPEC;
    pub type P403Sa = crate::EnumBitfieldStruct<u8, P403Sa_SPEC>;
    impl P403Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P404Sa_SPEC;
    pub type P404Sa = crate::EnumBitfieldStruct<u8, P404Sa_SPEC>;
    impl P404Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P405Sa_SPEC;
    pub type P405Sa = crate::EnumBitfieldStruct<u8, P405Sa_SPEC>;
    impl P405Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P406Sa_SPEC;
    pub type P406Sa = crate::EnumBitfieldStruct<u8, P406Sa_SPEC>;
    impl P406Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P407Sa_SPEC;
    pub type P407Sa = crate::EnumBitfieldStruct<u8, P407Sa_SPEC>;
    impl P407Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P408Sa_SPEC;
    pub type P408Sa = crate::EnumBitfieldStruct<u8, P408Sa_SPEC>;
    impl P408Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P409Sa_SPEC;
    pub type P409Sa = crate::EnumBitfieldStruct<u8, P409Sa_SPEC>;
    impl P409Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P410Sa_SPEC;
    pub type P410Sa = crate::EnumBitfieldStruct<u8, P410Sa_SPEC>;
    impl P410Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P411Sa_SPEC;
    pub type P411Sa = crate::EnumBitfieldStruct<u8, P411Sa_SPEC>;
    impl P411Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P412Sa_SPEC;
    pub type P412Sa = crate::EnumBitfieldStruct<u8, P412Sa_SPEC>;
    impl P412Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P413Sa_SPEC;
    pub type P413Sa = crate::EnumBitfieldStruct<u8, P413Sa_SPEC>;
    impl P413Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P414Sa_SPEC;
    pub type P414Sa = crate::EnumBitfieldStruct<u8, P414Sa_SPEC>;
    impl P414Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P415Sa_SPEC;
    pub type P415Sa = crate::EnumBitfieldStruct<u8, P415Sa_SPEC>;
    impl P415Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5Sar_SPEC;
impl crate::sealed::RegSpec for P5Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P5Sar = crate::RegValueT<P5Sar_SPEC>;

impl P5Sar {
    #[doc = "P500 Security Attribute"]
    #[inline(always)]
    pub fn p500sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p5sar::P500Sa,
        p5sar::P500Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p5sar::P500Sa,
            p5sar::P500Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P501 Security Attribute"]
    #[inline(always)]
    pub fn p501sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p5sar::P501Sa,
        p5sar::P501Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p5sar::P501Sa,
            p5sar::P501Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P502 Security Attribute"]
    #[inline(always)]
    pub fn p502sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p5sar::P502Sa,
        p5sar::P502Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p5sar::P502Sa,
            p5sar::P502Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P503 Security Attribute"]
    #[inline(always)]
    pub fn p503sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p5sar::P503Sa,
        p5sar::P503Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p5sar::P503Sa,
            p5sar::P503Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P504 Security Attribute"]
    #[inline(always)]
    pub fn p504sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p5sar::P504Sa,
        p5sar::P504Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p5sar::P504Sa,
            p5sar::P504Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P505 Security Attribute"]
    #[inline(always)]
    pub fn p505sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p5sar::P505Sa,
        p5sar::P505Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p5sar::P505Sa,
            p5sar::P505Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P506 Security Attribute"]
    #[inline(always)]
    pub fn p506sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p5sar::P506Sa,
        p5sar::P506Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p5sar::P506Sa,
            p5sar::P506Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P507 Security Attribute"]
    #[inline(always)]
    pub fn p507sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p5sar::P507Sa,
        p5sar::P507Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p5sar::P507Sa,
            p5sar::P507Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P508 Security Attribute"]
    #[inline(always)]
    pub fn p508sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p5sar::P508Sa,
        p5sar::P508Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p5sar::P508Sa,
            p5sar::P508Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P509 Security Attribute"]
    #[inline(always)]
    pub fn p509sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p5sar::P509Sa,
        p5sar::P509Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p5sar::P509Sa,
            p5sar::P509Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P510 Security Attribute"]
    #[inline(always)]
    pub fn p510sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p5sar::P510Sa,
        p5sar::P510Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p5sar::P510Sa,
            p5sar::P510Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P511 Security Attribute"]
    #[inline(always)]
    pub fn p511sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p5sar::P511Sa,
        p5sar::P511Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p5sar::P511Sa,
            p5sar::P511Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P512 Security Attribute"]
    #[inline(always)]
    pub fn p512sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p5sar::P512Sa,
        p5sar::P512Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p5sar::P512Sa,
            p5sar::P512Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P513 Security Attribute"]
    #[inline(always)]
    pub fn p513sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p5sar::P513Sa,
        p5sar::P513Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p5sar::P513Sa,
            p5sar::P513Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P514 Security Attribute"]
    #[inline(always)]
    pub fn p514sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p5sar::P514Sa,
        p5sar::P514Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p5sar::P514Sa,
            p5sar::P514Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P515 Security Attribute"]
    #[inline(always)]
    pub fn p515sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p5sar::P515Sa,
        p5sar::P515Sa,
        P5Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p5sar::P515Sa,
            p5sar::P515Sa,
            P5Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P5Sar {
    #[inline(always)]
    fn default() -> P5Sar {
        <crate::RegValueT<P5Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p5sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P500Sa_SPEC;
    pub type P500Sa = crate::EnumBitfieldStruct<u8, P500Sa_SPEC>;
    impl P500Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P501Sa_SPEC;
    pub type P501Sa = crate::EnumBitfieldStruct<u8, P501Sa_SPEC>;
    impl P501Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P502Sa_SPEC;
    pub type P502Sa = crate::EnumBitfieldStruct<u8, P502Sa_SPEC>;
    impl P502Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P503Sa_SPEC;
    pub type P503Sa = crate::EnumBitfieldStruct<u8, P503Sa_SPEC>;
    impl P503Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P504Sa_SPEC;
    pub type P504Sa = crate::EnumBitfieldStruct<u8, P504Sa_SPEC>;
    impl P504Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P505Sa_SPEC;
    pub type P505Sa = crate::EnumBitfieldStruct<u8, P505Sa_SPEC>;
    impl P505Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P506Sa_SPEC;
    pub type P506Sa = crate::EnumBitfieldStruct<u8, P506Sa_SPEC>;
    impl P506Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P507Sa_SPEC;
    pub type P507Sa = crate::EnumBitfieldStruct<u8, P507Sa_SPEC>;
    impl P507Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P508Sa_SPEC;
    pub type P508Sa = crate::EnumBitfieldStruct<u8, P508Sa_SPEC>;
    impl P508Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P509Sa_SPEC;
    pub type P509Sa = crate::EnumBitfieldStruct<u8, P509Sa_SPEC>;
    impl P509Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P510Sa_SPEC;
    pub type P510Sa = crate::EnumBitfieldStruct<u8, P510Sa_SPEC>;
    impl P510Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P511Sa_SPEC;
    pub type P511Sa = crate::EnumBitfieldStruct<u8, P511Sa_SPEC>;
    impl P511Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P512Sa_SPEC;
    pub type P512Sa = crate::EnumBitfieldStruct<u8, P512Sa_SPEC>;
    impl P512Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P513Sa_SPEC;
    pub type P513Sa = crate::EnumBitfieldStruct<u8, P513Sa_SPEC>;
    impl P513Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P514Sa_SPEC;
    pub type P514Sa = crate::EnumBitfieldStruct<u8, P514Sa_SPEC>;
    impl P514Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P515Sa_SPEC;
    pub type P515Sa = crate::EnumBitfieldStruct<u8, P515Sa_SPEC>;
    impl P515Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6Sar_SPEC;
impl crate::sealed::RegSpec for P6Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P6Sar = crate::RegValueT<P6Sar_SPEC>;

impl P6Sar {
    #[doc = "P600 Security Attribute"]
    #[inline(always)]
    pub fn p600sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p6sar::P600Sa,
        p6sar::P600Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p6sar::P600Sa,
            p6sar::P600Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P601 Security Attribute"]
    #[inline(always)]
    pub fn p601sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p6sar::P601Sa,
        p6sar::P601Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p6sar::P601Sa,
            p6sar::P601Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P602 Security Attribute"]
    #[inline(always)]
    pub fn p602sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p6sar::P602Sa,
        p6sar::P602Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p6sar::P602Sa,
            p6sar::P602Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P603 Security Attribute"]
    #[inline(always)]
    pub fn p603sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p6sar::P603Sa,
        p6sar::P603Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p6sar::P603Sa,
            p6sar::P603Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P604 Security Attribute"]
    #[inline(always)]
    pub fn p604sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p6sar::P604Sa,
        p6sar::P604Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p6sar::P604Sa,
            p6sar::P604Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P605 Security Attribute"]
    #[inline(always)]
    pub fn p605sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p6sar::P605Sa,
        p6sar::P605Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p6sar::P605Sa,
            p6sar::P605Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P606 Security Attribute"]
    #[inline(always)]
    pub fn p606sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p6sar::P606Sa,
        p6sar::P606Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p6sar::P606Sa,
            p6sar::P606Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P607 Security Attribute"]
    #[inline(always)]
    pub fn p607sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p6sar::P607Sa,
        p6sar::P607Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p6sar::P607Sa,
            p6sar::P607Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P608 Security Attribute"]
    #[inline(always)]
    pub fn p608sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p6sar::P608Sa,
        p6sar::P608Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p6sar::P608Sa,
            p6sar::P608Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P609 Security Attribute"]
    #[inline(always)]
    pub fn p609sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p6sar::P609Sa,
        p6sar::P609Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p6sar::P609Sa,
            p6sar::P609Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P610 Security Attribute"]
    #[inline(always)]
    pub fn p610sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p6sar::P610Sa,
        p6sar::P610Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p6sar::P610Sa,
            p6sar::P610Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P611 Security Attribute"]
    #[inline(always)]
    pub fn p611sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p6sar::P611Sa,
        p6sar::P611Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p6sar::P611Sa,
            p6sar::P611Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P612 Security Attribute"]
    #[inline(always)]
    pub fn p612sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p6sar::P612Sa,
        p6sar::P612Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p6sar::P612Sa,
            p6sar::P612Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P613 Security Attribute"]
    #[inline(always)]
    pub fn p613sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p6sar::P613Sa,
        p6sar::P613Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p6sar::P613Sa,
            p6sar::P613Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P614 Security Attribute"]
    #[inline(always)]
    pub fn p614sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p6sar::P614Sa,
        p6sar::P614Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p6sar::P614Sa,
            p6sar::P614Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P615 Security Attribute"]
    #[inline(always)]
    pub fn p615sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p6sar::P615Sa,
        p6sar::P615Sa,
        P6Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p6sar::P615Sa,
            p6sar::P615Sa,
            P6Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P6Sar {
    #[inline(always)]
    fn default() -> P6Sar {
        <crate::RegValueT<P6Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p6sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P600Sa_SPEC;
    pub type P600Sa = crate::EnumBitfieldStruct<u8, P600Sa_SPEC>;
    impl P600Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P601Sa_SPEC;
    pub type P601Sa = crate::EnumBitfieldStruct<u8, P601Sa_SPEC>;
    impl P601Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P602Sa_SPEC;
    pub type P602Sa = crate::EnumBitfieldStruct<u8, P602Sa_SPEC>;
    impl P602Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P603Sa_SPEC;
    pub type P603Sa = crate::EnumBitfieldStruct<u8, P603Sa_SPEC>;
    impl P603Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P604Sa_SPEC;
    pub type P604Sa = crate::EnumBitfieldStruct<u8, P604Sa_SPEC>;
    impl P604Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P605Sa_SPEC;
    pub type P605Sa = crate::EnumBitfieldStruct<u8, P605Sa_SPEC>;
    impl P605Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P606Sa_SPEC;
    pub type P606Sa = crate::EnumBitfieldStruct<u8, P606Sa_SPEC>;
    impl P606Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P607Sa_SPEC;
    pub type P607Sa = crate::EnumBitfieldStruct<u8, P607Sa_SPEC>;
    impl P607Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P608Sa_SPEC;
    pub type P608Sa = crate::EnumBitfieldStruct<u8, P608Sa_SPEC>;
    impl P608Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P609Sa_SPEC;
    pub type P609Sa = crate::EnumBitfieldStruct<u8, P609Sa_SPEC>;
    impl P609Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P610Sa_SPEC;
    pub type P610Sa = crate::EnumBitfieldStruct<u8, P610Sa_SPEC>;
    impl P610Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P611Sa_SPEC;
    pub type P611Sa = crate::EnumBitfieldStruct<u8, P611Sa_SPEC>;
    impl P611Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P612Sa_SPEC;
    pub type P612Sa = crate::EnumBitfieldStruct<u8, P612Sa_SPEC>;
    impl P612Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P613Sa_SPEC;
    pub type P613Sa = crate::EnumBitfieldStruct<u8, P613Sa_SPEC>;
    impl P613Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P614Sa_SPEC;
    pub type P614Sa = crate::EnumBitfieldStruct<u8, P614Sa_SPEC>;
    impl P614Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P615Sa_SPEC;
    pub type P615Sa = crate::EnumBitfieldStruct<u8, P615Sa_SPEC>;
    impl P615Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7Sar_SPEC;
impl crate::sealed::RegSpec for P7Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P7Sar = crate::RegValueT<P7Sar_SPEC>;

impl P7Sar {
    #[doc = "P700 Security Attribute"]
    #[inline(always)]
    pub fn p700sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p7sar::P700Sa,
        p7sar::P700Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p7sar::P700Sa,
            p7sar::P700Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P701 Security Attribute"]
    #[inline(always)]
    pub fn p701sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p7sar::P701Sa,
        p7sar::P701Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p7sar::P701Sa,
            p7sar::P701Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P702 Security Attribute"]
    #[inline(always)]
    pub fn p702sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p7sar::P702Sa,
        p7sar::P702Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p7sar::P702Sa,
            p7sar::P702Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P703 Security Attribute"]
    #[inline(always)]
    pub fn p703sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p7sar::P703Sa,
        p7sar::P703Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p7sar::P703Sa,
            p7sar::P703Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P704 Security Attribute"]
    #[inline(always)]
    pub fn p704sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p7sar::P704Sa,
        p7sar::P704Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p7sar::P704Sa,
            p7sar::P704Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P705 Security Attribute"]
    #[inline(always)]
    pub fn p705sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p7sar::P705Sa,
        p7sar::P705Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p7sar::P705Sa,
            p7sar::P705Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P706 Security Attribute"]
    #[inline(always)]
    pub fn p706sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p7sar::P706Sa,
        p7sar::P706Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p7sar::P706Sa,
            p7sar::P706Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P707 Security Attribute"]
    #[inline(always)]
    pub fn p707sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p7sar::P707Sa,
        p7sar::P707Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p7sar::P707Sa,
            p7sar::P707Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P708 Security Attribute"]
    #[inline(always)]
    pub fn p708sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p7sar::P708Sa,
        p7sar::P708Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p7sar::P708Sa,
            p7sar::P708Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P709 Security Attribute"]
    #[inline(always)]
    pub fn p709sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p7sar::P709Sa,
        p7sar::P709Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p7sar::P709Sa,
            p7sar::P709Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P710 Security Attribute"]
    #[inline(always)]
    pub fn p710sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p7sar::P710Sa,
        p7sar::P710Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p7sar::P710Sa,
            p7sar::P710Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P711 Security Attribute"]
    #[inline(always)]
    pub fn p711sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p7sar::P711Sa,
        p7sar::P711Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p7sar::P711Sa,
            p7sar::P711Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P712 Security Attribute"]
    #[inline(always)]
    pub fn p712sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p7sar::P712Sa,
        p7sar::P712Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p7sar::P712Sa,
            p7sar::P712Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P713 Security Attribute"]
    #[inline(always)]
    pub fn p713sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p7sar::P713Sa,
        p7sar::P713Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p7sar::P713Sa,
            p7sar::P713Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P714 Security Attribute"]
    #[inline(always)]
    pub fn p714sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p7sar::P714Sa,
        p7sar::P714Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p7sar::P714Sa,
            p7sar::P714Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P715 Security Attribute"]
    #[inline(always)]
    pub fn p715sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p7sar::P715Sa,
        p7sar::P715Sa,
        P7Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p7sar::P715Sa,
            p7sar::P715Sa,
            P7Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P7Sar {
    #[inline(always)]
    fn default() -> P7Sar {
        <crate::RegValueT<P7Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p7sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P700Sa_SPEC;
    pub type P700Sa = crate::EnumBitfieldStruct<u8, P700Sa_SPEC>;
    impl P700Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P701Sa_SPEC;
    pub type P701Sa = crate::EnumBitfieldStruct<u8, P701Sa_SPEC>;
    impl P701Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P702Sa_SPEC;
    pub type P702Sa = crate::EnumBitfieldStruct<u8, P702Sa_SPEC>;
    impl P702Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P703Sa_SPEC;
    pub type P703Sa = crate::EnumBitfieldStruct<u8, P703Sa_SPEC>;
    impl P703Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P704Sa_SPEC;
    pub type P704Sa = crate::EnumBitfieldStruct<u8, P704Sa_SPEC>;
    impl P704Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P705Sa_SPEC;
    pub type P705Sa = crate::EnumBitfieldStruct<u8, P705Sa_SPEC>;
    impl P705Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P706Sa_SPEC;
    pub type P706Sa = crate::EnumBitfieldStruct<u8, P706Sa_SPEC>;
    impl P706Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P707Sa_SPEC;
    pub type P707Sa = crate::EnumBitfieldStruct<u8, P707Sa_SPEC>;
    impl P707Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P708Sa_SPEC;
    pub type P708Sa = crate::EnumBitfieldStruct<u8, P708Sa_SPEC>;
    impl P708Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P709Sa_SPEC;
    pub type P709Sa = crate::EnumBitfieldStruct<u8, P709Sa_SPEC>;
    impl P709Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P710Sa_SPEC;
    pub type P710Sa = crate::EnumBitfieldStruct<u8, P710Sa_SPEC>;
    impl P710Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P711Sa_SPEC;
    pub type P711Sa = crate::EnumBitfieldStruct<u8, P711Sa_SPEC>;
    impl P711Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P712Sa_SPEC;
    pub type P712Sa = crate::EnumBitfieldStruct<u8, P712Sa_SPEC>;
    impl P712Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P713Sa_SPEC;
    pub type P713Sa = crate::EnumBitfieldStruct<u8, P713Sa_SPEC>;
    impl P713Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P714Sa_SPEC;
    pub type P714Sa = crate::EnumBitfieldStruct<u8, P714Sa_SPEC>;
    impl P714Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P715Sa_SPEC;
    pub type P715Sa = crate::EnumBitfieldStruct<u8, P715Sa_SPEC>;
    impl P715Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8Sar_SPEC;
impl crate::sealed::RegSpec for P8Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P8Sar = crate::RegValueT<P8Sar_SPEC>;

impl P8Sar {
    #[doc = "P800 Security Attribute"]
    #[inline(always)]
    pub fn p800sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p8sar::P800Sa,
        p8sar::P800Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p8sar::P800Sa,
            p8sar::P800Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P801 Security Attribute"]
    #[inline(always)]
    pub fn p801sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p8sar::P801Sa,
        p8sar::P801Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p8sar::P801Sa,
            p8sar::P801Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P802 Security Attribute"]
    #[inline(always)]
    pub fn p802sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p8sar::P802Sa,
        p8sar::P802Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p8sar::P802Sa,
            p8sar::P802Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P803 Security Attribute"]
    #[inline(always)]
    pub fn p803sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p8sar::P803Sa,
        p8sar::P803Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p8sar::P803Sa,
            p8sar::P803Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P804 Security Attribute"]
    #[inline(always)]
    pub fn p804sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p8sar::P804Sa,
        p8sar::P804Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p8sar::P804Sa,
            p8sar::P804Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P805 Security Attribute"]
    #[inline(always)]
    pub fn p805sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p8sar::P805Sa,
        p8sar::P805Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p8sar::P805Sa,
            p8sar::P805Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P806 Security Attribute"]
    #[inline(always)]
    pub fn p806sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p8sar::P806Sa,
        p8sar::P806Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p8sar::P806Sa,
            p8sar::P806Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P807 Security Attribute"]
    #[inline(always)]
    pub fn p807sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p8sar::P807Sa,
        p8sar::P807Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p8sar::P807Sa,
            p8sar::P807Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P808 Security Attribute"]
    #[inline(always)]
    pub fn p808sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p8sar::P808Sa,
        p8sar::P808Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p8sar::P808Sa,
            p8sar::P808Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P809 Security Attribute"]
    #[inline(always)]
    pub fn p809sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p8sar::P809Sa,
        p8sar::P809Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p8sar::P809Sa,
            p8sar::P809Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P810 Security Attribute"]
    #[inline(always)]
    pub fn p810sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p8sar::P810Sa,
        p8sar::P810Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p8sar::P810Sa,
            p8sar::P810Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P811 Security Attribute"]
    #[inline(always)]
    pub fn p811sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p8sar::P811Sa,
        p8sar::P811Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p8sar::P811Sa,
            p8sar::P811Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P812 Security Attribute"]
    #[inline(always)]
    pub fn p812sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p8sar::P812Sa,
        p8sar::P812Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p8sar::P812Sa,
            p8sar::P812Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P813 Security Attribute"]
    #[inline(always)]
    pub fn p813sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p8sar::P813Sa,
        p8sar::P813Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p8sar::P813Sa,
            p8sar::P813Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P814 Security Attribute"]
    #[inline(always)]
    pub fn p814sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p8sar::P814Sa,
        p8sar::P814Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p8sar::P814Sa,
            p8sar::P814Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P815 Security Attribute"]
    #[inline(always)]
    pub fn p815sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p8sar::P815Sa,
        p8sar::P815Sa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p8sar::P815Sa,
            p8sar::P815Sa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P8Sar {
    #[inline(always)]
    fn default() -> P8Sar {
        <crate::RegValueT<P8Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p8sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P800Sa_SPEC;
    pub type P800Sa = crate::EnumBitfieldStruct<u8, P800Sa_SPEC>;
    impl P800Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P801Sa_SPEC;
    pub type P801Sa = crate::EnumBitfieldStruct<u8, P801Sa_SPEC>;
    impl P801Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P802Sa_SPEC;
    pub type P802Sa = crate::EnumBitfieldStruct<u8, P802Sa_SPEC>;
    impl P802Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P803Sa_SPEC;
    pub type P803Sa = crate::EnumBitfieldStruct<u8, P803Sa_SPEC>;
    impl P803Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P804Sa_SPEC;
    pub type P804Sa = crate::EnumBitfieldStruct<u8, P804Sa_SPEC>;
    impl P804Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P805Sa_SPEC;
    pub type P805Sa = crate::EnumBitfieldStruct<u8, P805Sa_SPEC>;
    impl P805Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P806Sa_SPEC;
    pub type P806Sa = crate::EnumBitfieldStruct<u8, P806Sa_SPEC>;
    impl P806Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P807Sa_SPEC;
    pub type P807Sa = crate::EnumBitfieldStruct<u8, P807Sa_SPEC>;
    impl P807Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P808Sa_SPEC;
    pub type P808Sa = crate::EnumBitfieldStruct<u8, P808Sa_SPEC>;
    impl P808Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P809Sa_SPEC;
    pub type P809Sa = crate::EnumBitfieldStruct<u8, P809Sa_SPEC>;
    impl P809Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P810Sa_SPEC;
    pub type P810Sa = crate::EnumBitfieldStruct<u8, P810Sa_SPEC>;
    impl P810Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P811Sa_SPEC;
    pub type P811Sa = crate::EnumBitfieldStruct<u8, P811Sa_SPEC>;
    impl P811Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P812Sa_SPEC;
    pub type P812Sa = crate::EnumBitfieldStruct<u8, P812Sa_SPEC>;
    impl P812Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P813Sa_SPEC;
    pub type P813Sa = crate::EnumBitfieldStruct<u8, P813Sa_SPEC>;
    impl P813Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P814Sa_SPEC;
    pub type P814Sa = crate::EnumBitfieldStruct<u8, P814Sa_SPEC>;
    impl P814Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P815Sa_SPEC;
    pub type P815Sa = crate::EnumBitfieldStruct<u8, P815Sa_SPEC>;
    impl P815Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9Sar_SPEC;
impl crate::sealed::RegSpec for P9Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type P9Sar = crate::RegValueT<P9Sar_SPEC>;

impl P9Sar {
    #[doc = "P900 Security Attribute"]
    #[inline(always)]
    pub fn p900sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p9sar::P900Sa,
        p9sar::P900Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p9sar::P900Sa,
            p9sar::P900Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P901 Security Attribute"]
    #[inline(always)]
    pub fn p901sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p9sar::P901Sa,
        p9sar::P901Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p9sar::P901Sa,
            p9sar::P901Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P902 Security Attribute"]
    #[inline(always)]
    pub fn p902sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p9sar::P902Sa,
        p9sar::P902Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p9sar::P902Sa,
            p9sar::P902Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P903 Security Attribute"]
    #[inline(always)]
    pub fn p903sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        p9sar::P903Sa,
        p9sar::P903Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            p9sar::P903Sa,
            p9sar::P903Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P904 Security Attribute"]
    #[inline(always)]
    pub fn p904sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p9sar::P904Sa,
        p9sar::P904Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p9sar::P904Sa,
            p9sar::P904Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P905 Security Attribute"]
    #[inline(always)]
    pub fn p905sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        p9sar::P905Sa,
        p9sar::P905Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            p9sar::P905Sa,
            p9sar::P905Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P906 Security Attribute"]
    #[inline(always)]
    pub fn p906sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p9sar::P906Sa,
        p9sar::P906Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p9sar::P906Sa,
            p9sar::P906Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P907 Security Attribute"]
    #[inline(always)]
    pub fn p907sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        p9sar::P907Sa,
        p9sar::P907Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            p9sar::P907Sa,
            p9sar::P907Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P908 Security Attribute"]
    #[inline(always)]
    pub fn p908sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        p9sar::P908Sa,
        p9sar::P908Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            p9sar::P908Sa,
            p9sar::P908Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P909 Security Attribute"]
    #[inline(always)]
    pub fn p909sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        p9sar::P909Sa,
        p9sar::P909Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            p9sar::P909Sa,
            p9sar::P909Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P910 Security Attribute"]
    #[inline(always)]
    pub fn p910sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p9sar::P910Sa,
        p9sar::P910Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p9sar::P910Sa,
            p9sar::P910Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P911 Security Attribute"]
    #[inline(always)]
    pub fn p911sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p9sar::P911Sa,
        p9sar::P911Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p9sar::P911Sa,
            p9sar::P911Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P912 Security Attribute"]
    #[inline(always)]
    pub fn p912sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p9sar::P912Sa,
        p9sar::P912Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p9sar::P912Sa,
            p9sar::P912Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P913 Security Attribute"]
    #[inline(always)]
    pub fn p913sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p9sar::P913Sa,
        p9sar::P913Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p9sar::P913Sa,
            p9sar::P913Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P914 Security Attribute"]
    #[inline(always)]
    pub fn p914sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p9sar::P914Sa,
        p9sar::P914Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p9sar::P914Sa,
            p9sar::P914Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P915 Security Attribute"]
    #[inline(always)]
    pub fn p915sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p9sar::P915Sa,
        p9sar::P915Sa,
        P9Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p9sar::P915Sa,
            p9sar::P915Sa,
            P9Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P9Sar {
    #[inline(always)]
    fn default() -> P9Sar {
        <crate::RegValueT<P9Sar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p9sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P900Sa_SPEC;
    pub type P900Sa = crate::EnumBitfieldStruct<u8, P900Sa_SPEC>;
    impl P900Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P901Sa_SPEC;
    pub type P901Sa = crate::EnumBitfieldStruct<u8, P901Sa_SPEC>;
    impl P901Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P902Sa_SPEC;
    pub type P902Sa = crate::EnumBitfieldStruct<u8, P902Sa_SPEC>;
    impl P902Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P903Sa_SPEC;
    pub type P903Sa = crate::EnumBitfieldStruct<u8, P903Sa_SPEC>;
    impl P903Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P904Sa_SPEC;
    pub type P904Sa = crate::EnumBitfieldStruct<u8, P904Sa_SPEC>;
    impl P904Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P905Sa_SPEC;
    pub type P905Sa = crate::EnumBitfieldStruct<u8, P905Sa_SPEC>;
    impl P905Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P906Sa_SPEC;
    pub type P906Sa = crate::EnumBitfieldStruct<u8, P906Sa_SPEC>;
    impl P906Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P907Sa_SPEC;
    pub type P907Sa = crate::EnumBitfieldStruct<u8, P907Sa_SPEC>;
    impl P907Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P908Sa_SPEC;
    pub type P908Sa = crate::EnumBitfieldStruct<u8, P908Sa_SPEC>;
    impl P908Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P909Sa_SPEC;
    pub type P909Sa = crate::EnumBitfieldStruct<u8, P909Sa_SPEC>;
    impl P909Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P910Sa_SPEC;
    pub type P910Sa = crate::EnumBitfieldStruct<u8, P910Sa_SPEC>;
    impl P910Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P911Sa_SPEC;
    pub type P911Sa = crate::EnumBitfieldStruct<u8, P911Sa_SPEC>;
    impl P911Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P912Sa_SPEC;
    pub type P912Sa = crate::EnumBitfieldStruct<u8, P912Sa_SPEC>;
    impl P912Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P913Sa_SPEC;
    pub type P913Sa = crate::EnumBitfieldStruct<u8, P913Sa_SPEC>;
    impl P913Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P914Sa_SPEC;
    pub type P914Sa = crate::EnumBitfieldStruct<u8, P914Sa_SPEC>;
    impl P914Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P915Sa_SPEC;
    pub type P915Sa = crate::EnumBitfieldStruct<u8, P915Sa_SPEC>;
    impl P915Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pasar_SPEC;
impl crate::sealed::RegSpec for Pasar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type Pasar = crate::RegValueT<Pasar_SPEC>;

impl Pasar {
    #[doc = "PA00 Security Attribute"]
    #[inline(always)]
    pub fn pa00sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pasar::Pa00Sa,
        pasar::Pa00Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pasar::Pa00Sa,
            pasar::Pa00Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA01 Security Attribute"]
    #[inline(always)]
    pub fn pa01sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pasar::Pa01Sa,
        pasar::Pa01Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pasar::Pa01Sa,
            pasar::Pa01Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA02 Security Attribute"]
    #[inline(always)]
    pub fn pa02sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pasar::Pa02Sa,
        pasar::Pa02Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pasar::Pa02Sa,
            pasar::Pa02Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA03 Security Attribute"]
    #[inline(always)]
    pub fn pa03sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pasar::Pa03Sa,
        pasar::Pa03Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pasar::Pa03Sa,
            pasar::Pa03Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA04 Security Attribute"]
    #[inline(always)]
    pub fn pa04sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pasar::Pa04Sa,
        pasar::Pa04Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pasar::Pa04Sa,
            pasar::Pa04Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA05 Security Attribute"]
    #[inline(always)]
    pub fn pa05sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pasar::Pa05Sa,
        pasar::Pa05Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pasar::Pa05Sa,
            pasar::Pa05Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA06 Security Attribute"]
    #[inline(always)]
    pub fn pa06sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pasar::Pa06Sa,
        pasar::Pa06Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pasar::Pa06Sa,
            pasar::Pa06Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA07 Security Attribute"]
    #[inline(always)]
    pub fn pa07sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pasar::Pa07Sa,
        pasar::Pa07Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pasar::Pa07Sa,
            pasar::Pa07Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA08 Security Attribute"]
    #[inline(always)]
    pub fn pa08sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pasar::Pa08Sa,
        pasar::Pa08Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pasar::Pa08Sa,
            pasar::Pa08Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA09 Security Attribute"]
    #[inline(always)]
    pub fn pa09sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pasar::Pa09Sa,
        pasar::Pa09Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pasar::Pa09Sa,
            pasar::Pa09Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA10 Security Attribute"]
    #[inline(always)]
    pub fn pa10sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pasar::Pa10Sa,
        pasar::Pa10Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pasar::Pa10Sa,
            pasar::Pa10Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA11 Security Attribute"]
    #[inline(always)]
    pub fn pa11sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pasar::Pa11Sa,
        pasar::Pa11Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pasar::Pa11Sa,
            pasar::Pa11Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA12 Security Attribute"]
    #[inline(always)]
    pub fn pa12sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pasar::Pa12Sa,
        pasar::Pa12Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pasar::Pa12Sa,
            pasar::Pa12Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA13 Security Attribute"]
    #[inline(always)]
    pub fn pa13sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pasar::Pa13Sa,
        pasar::Pa13Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pasar::Pa13Sa,
            pasar::Pa13Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA14 Security Attribute"]
    #[inline(always)]
    pub fn pa14sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pasar::Pa14Sa,
        pasar::Pa14Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pasar::Pa14Sa,
            pasar::Pa14Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PA15 Security Attribute"]
    #[inline(always)]
    pub fn pa15sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pasar::Pa15Sa,
        pasar::Pa15Sa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pasar::Pa15Sa,
            pasar::Pa15Sa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pasar {
    #[inline(always)]
    fn default() -> Pasar {
        <crate::RegValueT<Pasar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pasar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa00Sa_SPEC;
    pub type Pa00Sa = crate::EnumBitfieldStruct<u8, Pa00Sa_SPEC>;
    impl Pa00Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa01Sa_SPEC;
    pub type Pa01Sa = crate::EnumBitfieldStruct<u8, Pa01Sa_SPEC>;
    impl Pa01Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa02Sa_SPEC;
    pub type Pa02Sa = crate::EnumBitfieldStruct<u8, Pa02Sa_SPEC>;
    impl Pa02Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa03Sa_SPEC;
    pub type Pa03Sa = crate::EnumBitfieldStruct<u8, Pa03Sa_SPEC>;
    impl Pa03Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa04Sa_SPEC;
    pub type Pa04Sa = crate::EnumBitfieldStruct<u8, Pa04Sa_SPEC>;
    impl Pa04Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa05Sa_SPEC;
    pub type Pa05Sa = crate::EnumBitfieldStruct<u8, Pa05Sa_SPEC>;
    impl Pa05Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa06Sa_SPEC;
    pub type Pa06Sa = crate::EnumBitfieldStruct<u8, Pa06Sa_SPEC>;
    impl Pa06Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa07Sa_SPEC;
    pub type Pa07Sa = crate::EnumBitfieldStruct<u8, Pa07Sa_SPEC>;
    impl Pa07Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa08Sa_SPEC;
    pub type Pa08Sa = crate::EnumBitfieldStruct<u8, Pa08Sa_SPEC>;
    impl Pa08Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa09Sa_SPEC;
    pub type Pa09Sa = crate::EnumBitfieldStruct<u8, Pa09Sa_SPEC>;
    impl Pa09Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa10Sa_SPEC;
    pub type Pa10Sa = crate::EnumBitfieldStruct<u8, Pa10Sa_SPEC>;
    impl Pa10Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa11Sa_SPEC;
    pub type Pa11Sa = crate::EnumBitfieldStruct<u8, Pa11Sa_SPEC>;
    impl Pa11Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa12Sa_SPEC;
    pub type Pa12Sa = crate::EnumBitfieldStruct<u8, Pa12Sa_SPEC>;
    impl Pa12Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa13Sa_SPEC;
    pub type Pa13Sa = crate::EnumBitfieldStruct<u8, Pa13Sa_SPEC>;
    impl Pa13Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa14Sa_SPEC;
    pub type Pa14Sa = crate::EnumBitfieldStruct<u8, Pa14Sa_SPEC>;
    impl Pa14Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pa15Sa_SPEC;
    pub type Pa15Sa = crate::EnumBitfieldStruct<u8, Pa15Sa_SPEC>;
    impl Pa15Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pbsar_SPEC;
impl crate::sealed::RegSpec for Pbsar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type Pbsar = crate::RegValueT<Pbsar_SPEC>;

impl Pbsar {
    #[doc = "PB00 Security Attribute"]
    #[inline(always)]
    pub fn pb00sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pbsar::Pb00Sa,
        pbsar::Pb00Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pbsar::Pb00Sa,
            pbsar::Pb00Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB01 Security Attribute"]
    #[inline(always)]
    pub fn pb01sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pbsar::Pb01Sa,
        pbsar::Pb01Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pbsar::Pb01Sa,
            pbsar::Pb01Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB02 Security Attribute"]
    #[inline(always)]
    pub fn pb02sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pbsar::Pb02Sa,
        pbsar::Pb02Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pbsar::Pb02Sa,
            pbsar::Pb02Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB03 Security Attribute"]
    #[inline(always)]
    pub fn pb03sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pbsar::Pb03Sa,
        pbsar::Pb03Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pbsar::Pb03Sa,
            pbsar::Pb03Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB04 Security Attribute"]
    #[inline(always)]
    pub fn pb04sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pbsar::Pb04Sa,
        pbsar::Pb04Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pbsar::Pb04Sa,
            pbsar::Pb04Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB05 Security Attribute"]
    #[inline(always)]
    pub fn pb05sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pbsar::Pb05Sa,
        pbsar::Pb05Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pbsar::Pb05Sa,
            pbsar::Pb05Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB06 Security Attribute"]
    #[inline(always)]
    pub fn pb06sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pbsar::Pb06Sa,
        pbsar::Pb06Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pbsar::Pb06Sa,
            pbsar::Pb06Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB07 Security Attribute"]
    #[inline(always)]
    pub fn pb07sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pbsar::Pb07Sa,
        pbsar::Pb07Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pbsar::Pb07Sa,
            pbsar::Pb07Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB08 Security Attribute"]
    #[inline(always)]
    pub fn pb08sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pbsar::Pb08Sa,
        pbsar::Pb08Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pbsar::Pb08Sa,
            pbsar::Pb08Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB09 Security Attribute"]
    #[inline(always)]
    pub fn pb09sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pbsar::Pb09Sa,
        pbsar::Pb09Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pbsar::Pb09Sa,
            pbsar::Pb09Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB10 Security Attribute"]
    #[inline(always)]
    pub fn pb10sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pbsar::Pb10Sa,
        pbsar::Pb10Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pbsar::Pb10Sa,
            pbsar::Pb10Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB11 Security Attribute"]
    #[inline(always)]
    pub fn pb11sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pbsar::Pb11Sa,
        pbsar::Pb11Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pbsar::Pb11Sa,
            pbsar::Pb11Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB12 Security Attribute"]
    #[inline(always)]
    pub fn pb12sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pbsar::Pb12Sa,
        pbsar::Pb12Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pbsar::Pb12Sa,
            pbsar::Pb12Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB13 Security Attribute"]
    #[inline(always)]
    pub fn pb13sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pbsar::Pb13Sa,
        pbsar::Pb13Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pbsar::Pb13Sa,
            pbsar::Pb13Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB14 Security Attribute"]
    #[inline(always)]
    pub fn pb14sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pbsar::Pb14Sa,
        pbsar::Pb14Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pbsar::Pb14Sa,
            pbsar::Pb14Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB15 Security Attribute"]
    #[inline(always)]
    pub fn pb15sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pbsar::Pb15Sa,
        pbsar::Pb15Sa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pbsar::Pb15Sa,
            pbsar::Pb15Sa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pbsar {
    #[inline(always)]
    fn default() -> Pbsar {
        <crate::RegValueT<Pbsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pbsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb00Sa_SPEC;
    pub type Pb00Sa = crate::EnumBitfieldStruct<u8, Pb00Sa_SPEC>;
    impl Pb00Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb01Sa_SPEC;
    pub type Pb01Sa = crate::EnumBitfieldStruct<u8, Pb01Sa_SPEC>;
    impl Pb01Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb02Sa_SPEC;
    pub type Pb02Sa = crate::EnumBitfieldStruct<u8, Pb02Sa_SPEC>;
    impl Pb02Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb03Sa_SPEC;
    pub type Pb03Sa = crate::EnumBitfieldStruct<u8, Pb03Sa_SPEC>;
    impl Pb03Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb04Sa_SPEC;
    pub type Pb04Sa = crate::EnumBitfieldStruct<u8, Pb04Sa_SPEC>;
    impl Pb04Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb05Sa_SPEC;
    pub type Pb05Sa = crate::EnumBitfieldStruct<u8, Pb05Sa_SPEC>;
    impl Pb05Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb06Sa_SPEC;
    pub type Pb06Sa = crate::EnumBitfieldStruct<u8, Pb06Sa_SPEC>;
    impl Pb06Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb07Sa_SPEC;
    pub type Pb07Sa = crate::EnumBitfieldStruct<u8, Pb07Sa_SPEC>;
    impl Pb07Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb08Sa_SPEC;
    pub type Pb08Sa = crate::EnumBitfieldStruct<u8, Pb08Sa_SPEC>;
    impl Pb08Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb09Sa_SPEC;
    pub type Pb09Sa = crate::EnumBitfieldStruct<u8, Pb09Sa_SPEC>;
    impl Pb09Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb10Sa_SPEC;
    pub type Pb10Sa = crate::EnumBitfieldStruct<u8, Pb10Sa_SPEC>;
    impl Pb10Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb11Sa_SPEC;
    pub type Pb11Sa = crate::EnumBitfieldStruct<u8, Pb11Sa_SPEC>;
    impl Pb11Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb12Sa_SPEC;
    pub type Pb12Sa = crate::EnumBitfieldStruct<u8, Pb12Sa_SPEC>;
    impl Pb12Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb13Sa_SPEC;
    pub type Pb13Sa = crate::EnumBitfieldStruct<u8, Pb13Sa_SPEC>;
    impl Pb13Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb14Sa_SPEC;
    pub type Pb14Sa = crate::EnumBitfieldStruct<u8, Pb14Sa_SPEC>;
    impl Pb14Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb15Sa_SPEC;
    pub type Pb15Sa = crate::EnumBitfieldStruct<u8, Pb15Sa_SPEC>;
    impl Pb15Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsar_SPEC;
impl crate::sealed::RegSpec for Pcsar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type Pcsar = crate::RegValueT<Pcsar_SPEC>;

impl Pcsar {
    #[doc = "PB00 Security Attribute"]
    #[inline(always)]
    pub fn pb00sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pcsar::Pb00Sa,
        pcsar::Pb00Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pcsar::Pb00Sa,
            pcsar::Pb00Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB01 Security Attribute"]
    #[inline(always)]
    pub fn pb01sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pcsar::Pb01Sa,
        pcsar::Pb01Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pcsar::Pb01Sa,
            pcsar::Pb01Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB02 Security Attribute"]
    #[inline(always)]
    pub fn pb02sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pcsar::Pb02Sa,
        pcsar::Pb02Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pcsar::Pb02Sa,
            pcsar::Pb02Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB03 Security Attribute"]
    #[inline(always)]
    pub fn pb03sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pcsar::Pb03Sa,
        pcsar::Pb03Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pcsar::Pb03Sa,
            pcsar::Pb03Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB04 Security Attribute"]
    #[inline(always)]
    pub fn pb04sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pcsar::Pb04Sa,
        pcsar::Pb04Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pcsar::Pb04Sa,
            pcsar::Pb04Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB05 Security Attribute"]
    #[inline(always)]
    pub fn pb05sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pcsar::Pb05Sa,
        pcsar::Pb05Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pcsar::Pb05Sa,
            pcsar::Pb05Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB06 Security Attribute"]
    #[inline(always)]
    pub fn pb06sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pcsar::Pb06Sa,
        pcsar::Pb06Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pcsar::Pb06Sa,
            pcsar::Pb06Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB07 Security Attribute"]
    #[inline(always)]
    pub fn pb07sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pcsar::Pb07Sa,
        pcsar::Pb07Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pcsar::Pb07Sa,
            pcsar::Pb07Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB08 Security Attribute"]
    #[inline(always)]
    pub fn pb08sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pcsar::Pb08Sa,
        pcsar::Pb08Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pcsar::Pb08Sa,
            pcsar::Pb08Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB09 Security Attribute"]
    #[inline(always)]
    pub fn pb09sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pcsar::Pb09Sa,
        pcsar::Pb09Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pcsar::Pb09Sa,
            pcsar::Pb09Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB10 Security Attribute"]
    #[inline(always)]
    pub fn pb10sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pcsar::Pb10Sa,
        pcsar::Pb10Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pcsar::Pb10Sa,
            pcsar::Pb10Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB11 Security Attribute"]
    #[inline(always)]
    pub fn pb11sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pcsar::Pb11Sa,
        pcsar::Pb11Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pcsar::Pb11Sa,
            pcsar::Pb11Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB12 Security Attribute"]
    #[inline(always)]
    pub fn pb12sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pcsar::Pb12Sa,
        pcsar::Pb12Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pcsar::Pb12Sa,
            pcsar::Pb12Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB13 Security Attribute"]
    #[inline(always)]
    pub fn pb13sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pcsar::Pb13Sa,
        pcsar::Pb13Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pcsar::Pb13Sa,
            pcsar::Pb13Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB14 Security Attribute"]
    #[inline(always)]
    pub fn pb14sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pcsar::Pb14Sa,
        pcsar::Pb14Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pcsar::Pb14Sa,
            pcsar::Pb14Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PB15 Security Attribute"]
    #[inline(always)]
    pub fn pb15sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pcsar::Pb15Sa,
        pcsar::Pb15Sa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pcsar::Pb15Sa,
            pcsar::Pb15Sa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcsar {
    #[inline(always)]
    fn default() -> Pcsar {
        <crate::RegValueT<Pcsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb00Sa_SPEC;
    pub type Pb00Sa = crate::EnumBitfieldStruct<u8, Pb00Sa_SPEC>;
    impl Pb00Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb01Sa_SPEC;
    pub type Pb01Sa = crate::EnumBitfieldStruct<u8, Pb01Sa_SPEC>;
    impl Pb01Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb02Sa_SPEC;
    pub type Pb02Sa = crate::EnumBitfieldStruct<u8, Pb02Sa_SPEC>;
    impl Pb02Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb03Sa_SPEC;
    pub type Pb03Sa = crate::EnumBitfieldStruct<u8, Pb03Sa_SPEC>;
    impl Pb03Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb04Sa_SPEC;
    pub type Pb04Sa = crate::EnumBitfieldStruct<u8, Pb04Sa_SPEC>;
    impl Pb04Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb05Sa_SPEC;
    pub type Pb05Sa = crate::EnumBitfieldStruct<u8, Pb05Sa_SPEC>;
    impl Pb05Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb06Sa_SPEC;
    pub type Pb06Sa = crate::EnumBitfieldStruct<u8, Pb06Sa_SPEC>;
    impl Pb06Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb07Sa_SPEC;
    pub type Pb07Sa = crate::EnumBitfieldStruct<u8, Pb07Sa_SPEC>;
    impl Pb07Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb08Sa_SPEC;
    pub type Pb08Sa = crate::EnumBitfieldStruct<u8, Pb08Sa_SPEC>;
    impl Pb08Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb09Sa_SPEC;
    pub type Pb09Sa = crate::EnumBitfieldStruct<u8, Pb09Sa_SPEC>;
    impl Pb09Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb10Sa_SPEC;
    pub type Pb10Sa = crate::EnumBitfieldStruct<u8, Pb10Sa_SPEC>;
    impl Pb10Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb11Sa_SPEC;
    pub type Pb11Sa = crate::EnumBitfieldStruct<u8, Pb11Sa_SPEC>;
    impl Pb11Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb12Sa_SPEC;
    pub type Pb12Sa = crate::EnumBitfieldStruct<u8, Pb12Sa_SPEC>;
    impl Pb12Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb13Sa_SPEC;
    pub type Pb13Sa = crate::EnumBitfieldStruct<u8, Pb13Sa_SPEC>;
    impl Pb13Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb14Sa_SPEC;
    pub type Pb14Sa = crate::EnumBitfieldStruct<u8, Pb14Sa_SPEC>;
    impl Pb14Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pb15Sa_SPEC;
    pub type Pb15Sa = crate::EnumBitfieldStruct<u8, Pb15Sa_SPEC>;
    impl Pb15Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsar_SPEC;
impl crate::sealed::RegSpec for Pdsar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type Pdsar = crate::RegValueT<Pdsar_SPEC>;

impl Pdsar {
    #[doc = "PC00 Security Attribute"]
    #[inline(always)]
    pub fn pc00sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pdsar::Pc00Sa,
        pdsar::Pc00Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pdsar::Pc00Sa,
            pdsar::Pc00Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC01 Security Attribute"]
    #[inline(always)]
    pub fn pc01sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pdsar::Pc01Sa,
        pdsar::Pc01Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pdsar::Pc01Sa,
            pdsar::Pc01Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC02 Security Attribute"]
    #[inline(always)]
    pub fn pc02sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pdsar::Pc02Sa,
        pdsar::Pc02Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pdsar::Pc02Sa,
            pdsar::Pc02Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC03 Security Attribute"]
    #[inline(always)]
    pub fn pc03sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pdsar::Pc03Sa,
        pdsar::Pc03Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pdsar::Pc03Sa,
            pdsar::Pc03Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC04 Security Attribute"]
    #[inline(always)]
    pub fn pc04sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pdsar::Pc04Sa,
        pdsar::Pc04Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pdsar::Pc04Sa,
            pdsar::Pc04Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC05 Security Attribute"]
    #[inline(always)]
    pub fn pc05sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pdsar::Pc05Sa,
        pdsar::Pc05Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pdsar::Pc05Sa,
            pdsar::Pc05Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC06 Security Attribute"]
    #[inline(always)]
    pub fn pc06sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pdsar::Pc06Sa,
        pdsar::Pc06Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pdsar::Pc06Sa,
            pdsar::Pc06Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC07 Security Attribute"]
    #[inline(always)]
    pub fn pc07sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pdsar::Pc07Sa,
        pdsar::Pc07Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pdsar::Pc07Sa,
            pdsar::Pc07Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC08 Security Attribute"]
    #[inline(always)]
    pub fn pc08sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pdsar::Pc08Sa,
        pdsar::Pc08Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pdsar::Pc08Sa,
            pdsar::Pc08Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC09 Security Attribute"]
    #[inline(always)]
    pub fn pc09sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pdsar::Pc09Sa,
        pdsar::Pc09Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pdsar::Pc09Sa,
            pdsar::Pc09Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC10 Security Attribute"]
    #[inline(always)]
    pub fn pc10sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pdsar::Pc10Sa,
        pdsar::Pc10Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pdsar::Pc10Sa,
            pdsar::Pc10Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC11 Security Attribute"]
    #[inline(always)]
    pub fn pc11sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pdsar::Pc11Sa,
        pdsar::Pc11Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pdsar::Pc11Sa,
            pdsar::Pc11Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC12 Security Attribute"]
    #[inline(always)]
    pub fn pc12sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pdsar::Pc12Sa,
        pdsar::Pc12Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pdsar::Pc12Sa,
            pdsar::Pc12Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC13 Security Attribute"]
    #[inline(always)]
    pub fn pc13sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pdsar::Pc13Sa,
        pdsar::Pc13Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pdsar::Pc13Sa,
            pdsar::Pc13Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC14 Security Attribute"]
    #[inline(always)]
    pub fn pc14sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pdsar::Pc14Sa,
        pdsar::Pc14Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pdsar::Pc14Sa,
            pdsar::Pc14Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PC15 Security Attribute"]
    #[inline(always)]
    pub fn pc15sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pdsar::Pc15Sa,
        pdsar::Pc15Sa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pdsar::Pc15Sa,
            pdsar::Pc15Sa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdsar {
    #[inline(always)]
    fn default() -> Pdsar {
        <crate::RegValueT<Pdsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc00Sa_SPEC;
    pub type Pc00Sa = crate::EnumBitfieldStruct<u8, Pc00Sa_SPEC>;
    impl Pc00Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc01Sa_SPEC;
    pub type Pc01Sa = crate::EnumBitfieldStruct<u8, Pc01Sa_SPEC>;
    impl Pc01Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc02Sa_SPEC;
    pub type Pc02Sa = crate::EnumBitfieldStruct<u8, Pc02Sa_SPEC>;
    impl Pc02Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc03Sa_SPEC;
    pub type Pc03Sa = crate::EnumBitfieldStruct<u8, Pc03Sa_SPEC>;
    impl Pc03Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc04Sa_SPEC;
    pub type Pc04Sa = crate::EnumBitfieldStruct<u8, Pc04Sa_SPEC>;
    impl Pc04Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc05Sa_SPEC;
    pub type Pc05Sa = crate::EnumBitfieldStruct<u8, Pc05Sa_SPEC>;
    impl Pc05Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc06Sa_SPEC;
    pub type Pc06Sa = crate::EnumBitfieldStruct<u8, Pc06Sa_SPEC>;
    impl Pc06Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc07Sa_SPEC;
    pub type Pc07Sa = crate::EnumBitfieldStruct<u8, Pc07Sa_SPEC>;
    impl Pc07Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc08Sa_SPEC;
    pub type Pc08Sa = crate::EnumBitfieldStruct<u8, Pc08Sa_SPEC>;
    impl Pc08Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc09Sa_SPEC;
    pub type Pc09Sa = crate::EnumBitfieldStruct<u8, Pc09Sa_SPEC>;
    impl Pc09Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc10Sa_SPEC;
    pub type Pc10Sa = crate::EnumBitfieldStruct<u8, Pc10Sa_SPEC>;
    impl Pc10Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc11Sa_SPEC;
    pub type Pc11Sa = crate::EnumBitfieldStruct<u8, Pc11Sa_SPEC>;
    impl Pc11Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc12Sa_SPEC;
    pub type Pc12Sa = crate::EnumBitfieldStruct<u8, Pc12Sa_SPEC>;
    impl Pc12Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc13Sa_SPEC;
    pub type Pc13Sa = crate::EnumBitfieldStruct<u8, Pc13Sa_SPEC>;
    impl Pc13Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc14Sa_SPEC;
    pub type Pc14Sa = crate::EnumBitfieldStruct<u8, Pc14Sa_SPEC>;
    impl Pc14Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pc15Sa_SPEC;
    pub type Pc15Sa = crate::EnumBitfieldStruct<u8, Pc15Sa_SPEC>;
    impl Pc15Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pesar_SPEC;
impl crate::sealed::RegSpec for Pesar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type Pesar = crate::RegValueT<Pesar_SPEC>;

impl Pesar {
    #[doc = "PD00 Security Attribute"]
    #[inline(always)]
    pub fn pd00sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pesar::Pd00Sa,
        pesar::Pd00Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pesar::Pd00Sa,
            pesar::Pd00Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD01 Security Attribute"]
    #[inline(always)]
    pub fn pd01sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pesar::Pd01Sa,
        pesar::Pd01Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pesar::Pd01Sa,
            pesar::Pd01Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD02 Security Attribute"]
    #[inline(always)]
    pub fn pd02sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pesar::Pd02Sa,
        pesar::Pd02Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pesar::Pd02Sa,
            pesar::Pd02Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD03 Security Attribute"]
    #[inline(always)]
    pub fn pd03sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pesar::Pd03Sa,
        pesar::Pd03Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pesar::Pd03Sa,
            pesar::Pd03Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD04 Security Attribute"]
    #[inline(always)]
    pub fn pd04sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pesar::Pd04Sa,
        pesar::Pd04Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pesar::Pd04Sa,
            pesar::Pd04Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD05 Security Attribute"]
    #[inline(always)]
    pub fn pd05sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pesar::Pd05Sa,
        pesar::Pd05Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pesar::Pd05Sa,
            pesar::Pd05Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD06 Security Attribute"]
    #[inline(always)]
    pub fn pd06sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pesar::Pd06Sa,
        pesar::Pd06Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pesar::Pd06Sa,
            pesar::Pd06Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD07 Security Attribute"]
    #[inline(always)]
    pub fn pd07sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pesar::Pd07Sa,
        pesar::Pd07Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pesar::Pd07Sa,
            pesar::Pd07Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD08 Security Attribute"]
    #[inline(always)]
    pub fn pd08sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pesar::Pd08Sa,
        pesar::Pd08Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pesar::Pd08Sa,
            pesar::Pd08Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD09 Security Attribute"]
    #[inline(always)]
    pub fn pd09sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pesar::Pd09Sa,
        pesar::Pd09Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pesar::Pd09Sa,
            pesar::Pd09Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD10 Security Attribute"]
    #[inline(always)]
    pub fn pd10sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pesar::Pd10Sa,
        pesar::Pd10Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pesar::Pd10Sa,
            pesar::Pd10Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD11 Security Attribute"]
    #[inline(always)]
    pub fn pd11sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pesar::Pd11Sa,
        pesar::Pd11Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pesar::Pd11Sa,
            pesar::Pd11Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD12 Security Attribute"]
    #[inline(always)]
    pub fn pd12sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pesar::Pd12Sa,
        pesar::Pd12Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pesar::Pd12Sa,
            pesar::Pd12Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD13 Security Attribute"]
    #[inline(always)]
    pub fn pd13sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pesar::Pd13Sa,
        pesar::Pd13Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pesar::Pd13Sa,
            pesar::Pd13Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD14 Security Attribute"]
    #[inline(always)]
    pub fn pd14sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pesar::Pd14Sa,
        pesar::Pd14Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pesar::Pd14Sa,
            pesar::Pd14Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PD15 Security Attribute"]
    #[inline(always)]
    pub fn pd15sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pesar::Pd15Sa,
        pesar::Pd15Sa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pesar::Pd15Sa,
            pesar::Pd15Sa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pesar {
    #[inline(always)]
    fn default() -> Pesar {
        <crate::RegValueT<Pesar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pesar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd00Sa_SPEC;
    pub type Pd00Sa = crate::EnumBitfieldStruct<u8, Pd00Sa_SPEC>;
    impl Pd00Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd01Sa_SPEC;
    pub type Pd01Sa = crate::EnumBitfieldStruct<u8, Pd01Sa_SPEC>;
    impl Pd01Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd02Sa_SPEC;
    pub type Pd02Sa = crate::EnumBitfieldStruct<u8, Pd02Sa_SPEC>;
    impl Pd02Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd03Sa_SPEC;
    pub type Pd03Sa = crate::EnumBitfieldStruct<u8, Pd03Sa_SPEC>;
    impl Pd03Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd04Sa_SPEC;
    pub type Pd04Sa = crate::EnumBitfieldStruct<u8, Pd04Sa_SPEC>;
    impl Pd04Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd05Sa_SPEC;
    pub type Pd05Sa = crate::EnumBitfieldStruct<u8, Pd05Sa_SPEC>;
    impl Pd05Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd06Sa_SPEC;
    pub type Pd06Sa = crate::EnumBitfieldStruct<u8, Pd06Sa_SPEC>;
    impl Pd06Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd07Sa_SPEC;
    pub type Pd07Sa = crate::EnumBitfieldStruct<u8, Pd07Sa_SPEC>;
    impl Pd07Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd08Sa_SPEC;
    pub type Pd08Sa = crate::EnumBitfieldStruct<u8, Pd08Sa_SPEC>;
    impl Pd08Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd09Sa_SPEC;
    pub type Pd09Sa = crate::EnumBitfieldStruct<u8, Pd09Sa_SPEC>;
    impl Pd09Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd10Sa_SPEC;
    pub type Pd10Sa = crate::EnumBitfieldStruct<u8, Pd10Sa_SPEC>;
    impl Pd10Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd11Sa_SPEC;
    pub type Pd11Sa = crate::EnumBitfieldStruct<u8, Pd11Sa_SPEC>;
    impl Pd11Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd12Sa_SPEC;
    pub type Pd12Sa = crate::EnumBitfieldStruct<u8, Pd12Sa_SPEC>;
    impl Pd12Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd13Sa_SPEC;
    pub type Pd13Sa = crate::EnumBitfieldStruct<u8, Pd13Sa_SPEC>;
    impl Pd13Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd14Sa_SPEC;
    pub type Pd14Sa = crate::EnumBitfieldStruct<u8, Pd14Sa_SPEC>;
    impl Pd14Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd15Sa_SPEC;
    pub type Pd15Sa = crate::EnumBitfieldStruct<u8, Pd15Sa_SPEC>;
    impl Pd15Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfsar_SPEC;
impl crate::sealed::RegSpec for Pfsar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type Pfsar = crate::RegValueT<Pfsar_SPEC>;

impl Pfsar {
    #[doc = "PE00 Security Attribute"]
    #[inline(always)]
    pub fn pe00sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pfsar::Pe00Sa,
        pfsar::Pe00Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pfsar::Pe00Sa,
            pfsar::Pe00Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE01 Security Attribute"]
    #[inline(always)]
    pub fn pe01sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pfsar::Pe01Sa,
        pfsar::Pe01Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pfsar::Pe01Sa,
            pfsar::Pe01Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE02 Security Attribute"]
    #[inline(always)]
    pub fn pe02sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pfsar::Pe02Sa,
        pfsar::Pe02Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pfsar::Pe02Sa,
            pfsar::Pe02Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE03 Security Attribute"]
    #[inline(always)]
    pub fn pe03sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pfsar::Pe03Sa,
        pfsar::Pe03Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pfsar::Pe03Sa,
            pfsar::Pe03Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE04 Security Attribute"]
    #[inline(always)]
    pub fn pe04sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pfsar::Pe04Sa,
        pfsar::Pe04Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pfsar::Pe04Sa,
            pfsar::Pe04Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE05 Security Attribute"]
    #[inline(always)]
    pub fn pe05sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pfsar::Pe05Sa,
        pfsar::Pe05Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pfsar::Pe05Sa,
            pfsar::Pe05Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE06 Security Attribute"]
    #[inline(always)]
    pub fn pe06sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pfsar::Pe06Sa,
        pfsar::Pe06Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pfsar::Pe06Sa,
            pfsar::Pe06Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE07 Security Attribute"]
    #[inline(always)]
    pub fn pe07sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pfsar::Pe07Sa,
        pfsar::Pe07Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pfsar::Pe07Sa,
            pfsar::Pe07Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE08 Security Attribute"]
    #[inline(always)]
    pub fn pe08sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pfsar::Pe08Sa,
        pfsar::Pe08Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pfsar::Pe08Sa,
            pfsar::Pe08Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE09 Security Attribute"]
    #[inline(always)]
    pub fn pe09sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pfsar::Pe09Sa,
        pfsar::Pe09Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pfsar::Pe09Sa,
            pfsar::Pe09Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE10 Security Attribute"]
    #[inline(always)]
    pub fn pe10sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pfsar::Pe10Sa,
        pfsar::Pe10Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pfsar::Pe10Sa,
            pfsar::Pe10Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE11 Security Attribute"]
    #[inline(always)]
    pub fn pe11sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pfsar::Pe11Sa,
        pfsar::Pe11Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pfsar::Pe11Sa,
            pfsar::Pe11Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE12 Security Attribute"]
    #[inline(always)]
    pub fn pe12sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pfsar::Pe12Sa,
        pfsar::Pe12Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pfsar::Pe12Sa,
            pfsar::Pe12Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE13 Security Attribute"]
    #[inline(always)]
    pub fn pe13sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pfsar::Pe13Sa,
        pfsar::Pe13Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pfsar::Pe13Sa,
            pfsar::Pe13Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE14 Security Attribute"]
    #[inline(always)]
    pub fn pe14sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pfsar::Pe14Sa,
        pfsar::Pe14Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pfsar::Pe14Sa,
            pfsar::Pe14Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PE15 Security Attribute"]
    #[inline(always)]
    pub fn pe15sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pfsar::Pe15Sa,
        pfsar::Pe15Sa,
        Pfsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pfsar::Pe15Sa,
            pfsar::Pe15Sa,
            Pfsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pfsar {
    #[inline(always)]
    fn default() -> Pfsar {
        <crate::RegValueT<Pfsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pfsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe00Sa_SPEC;
    pub type Pe00Sa = crate::EnumBitfieldStruct<u8, Pe00Sa_SPEC>;
    impl Pe00Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe01Sa_SPEC;
    pub type Pe01Sa = crate::EnumBitfieldStruct<u8, Pe01Sa_SPEC>;
    impl Pe01Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe02Sa_SPEC;
    pub type Pe02Sa = crate::EnumBitfieldStruct<u8, Pe02Sa_SPEC>;
    impl Pe02Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe03Sa_SPEC;
    pub type Pe03Sa = crate::EnumBitfieldStruct<u8, Pe03Sa_SPEC>;
    impl Pe03Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe04Sa_SPEC;
    pub type Pe04Sa = crate::EnumBitfieldStruct<u8, Pe04Sa_SPEC>;
    impl Pe04Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe05Sa_SPEC;
    pub type Pe05Sa = crate::EnumBitfieldStruct<u8, Pe05Sa_SPEC>;
    impl Pe05Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe06Sa_SPEC;
    pub type Pe06Sa = crate::EnumBitfieldStruct<u8, Pe06Sa_SPEC>;
    impl Pe06Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe07Sa_SPEC;
    pub type Pe07Sa = crate::EnumBitfieldStruct<u8, Pe07Sa_SPEC>;
    impl Pe07Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe08Sa_SPEC;
    pub type Pe08Sa = crate::EnumBitfieldStruct<u8, Pe08Sa_SPEC>;
    impl Pe08Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe09Sa_SPEC;
    pub type Pe09Sa = crate::EnumBitfieldStruct<u8, Pe09Sa_SPEC>;
    impl Pe09Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe10Sa_SPEC;
    pub type Pe10Sa = crate::EnumBitfieldStruct<u8, Pe10Sa_SPEC>;
    impl Pe10Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe11Sa_SPEC;
    pub type Pe11Sa = crate::EnumBitfieldStruct<u8, Pe11Sa_SPEC>;
    impl Pe11Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe12Sa_SPEC;
    pub type Pe12Sa = crate::EnumBitfieldStruct<u8, Pe12Sa_SPEC>;
    impl Pe12Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe13Sa_SPEC;
    pub type Pe13Sa = crate::EnumBitfieldStruct<u8, Pe13Sa_SPEC>;
    impl Pe13Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe14Sa_SPEC;
    pub type Pe14Sa = crate::EnumBitfieldStruct<u8, Pe14Sa_SPEC>;
    impl Pe14Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe15Sa_SPEC;
    pub type Pe15Sa = crate::EnumBitfieldStruct<u8, Pe15Sa_SPEC>;
    impl Pe15Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgsar_SPEC;
impl crate::sealed::RegSpec for Pgsar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribute register 1"]
pub type Pgsar = crate::RegValueT<Pgsar_SPEC>;

impl Pgsar {
    #[doc = "PG00 Security Attribute"]
    #[inline(always)]
    pub fn pg00sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pgsar::Pg00Sa,
        pgsar::Pg00Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pgsar::Pg00Sa,
            pgsar::Pg00Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG01 Security Attribute"]
    #[inline(always)]
    pub fn pg01sa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pgsar::Pg01Sa,
        pgsar::Pg01Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pgsar::Pg01Sa,
            pgsar::Pg01Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG02 Security Attribute"]
    #[inline(always)]
    pub fn pg02sa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pgsar::Pg02Sa,
        pgsar::Pg02Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pgsar::Pg02Sa,
            pgsar::Pg02Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG03 Security Attribute"]
    #[inline(always)]
    pub fn pg03sa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pgsar::Pg03Sa,
        pgsar::Pg03Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pgsar::Pg03Sa,
            pgsar::Pg03Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG04 Security Attribute"]
    #[inline(always)]
    pub fn pg04sa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pgsar::Pg04Sa,
        pgsar::Pg04Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pgsar::Pg04Sa,
            pgsar::Pg04Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG05 Security Attribute"]
    #[inline(always)]
    pub fn pg05sa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pgsar::Pg05Sa,
        pgsar::Pg05Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pgsar::Pg05Sa,
            pgsar::Pg05Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG06 Security Attribute"]
    #[inline(always)]
    pub fn pg06sa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pgsar::Pg06Sa,
        pgsar::Pg06Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pgsar::Pg06Sa,
            pgsar::Pg06Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG07 Security Attribute"]
    #[inline(always)]
    pub fn pg07sa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pgsar::Pg07Sa,
        pgsar::Pg07Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pgsar::Pg07Sa,
            pgsar::Pg07Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG08 Security Attribute"]
    #[inline(always)]
    pub fn pg08sa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pgsar::Pg08Sa,
        pgsar::Pg08Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pgsar::Pg08Sa,
            pgsar::Pg08Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG09 Security Attribute"]
    #[inline(always)]
    pub fn pg09sa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pgsar::Pg09Sa,
        pgsar::Pg09Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pgsar::Pg09Sa,
            pgsar::Pg09Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG10 Security Attribute"]
    #[inline(always)]
    pub fn pg10sa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pgsar::Pg10Sa,
        pgsar::Pg10Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pgsar::Pg10Sa,
            pgsar::Pg10Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG11 Security Attribute"]
    #[inline(always)]
    pub fn pg11sa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pgsar::Pg11Sa,
        pgsar::Pg11Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pgsar::Pg11Sa,
            pgsar::Pg11Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG12 Security Attribute"]
    #[inline(always)]
    pub fn pg12sa(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pgsar::Pg12Sa,
        pgsar::Pg12Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pgsar::Pg12Sa,
            pgsar::Pg12Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG13 Security Attribute"]
    #[inline(always)]
    pub fn pg13sa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pgsar::Pg13Sa,
        pgsar::Pg13Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pgsar::Pg13Sa,
            pgsar::Pg13Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG14 Security Attribute"]
    #[inline(always)]
    pub fn pg14sa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pgsar::Pg14Sa,
        pgsar::Pg14Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pgsar::Pg14Sa,
            pgsar::Pg14Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PG15 Security Attribute"]
    #[inline(always)]
    pub fn pg15sa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pgsar::Pg15Sa,
        pgsar::Pg15Sa,
        Pgsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pgsar::Pg15Sa,
            pgsar::Pg15Sa,
            Pgsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pgsar {
    #[inline(always)]
    fn default() -> Pgsar {
        <crate::RegValueT<Pgsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pgsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg00Sa_SPEC;
    pub type Pg00Sa = crate::EnumBitfieldStruct<u8, Pg00Sa_SPEC>;
    impl Pg00Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg01Sa_SPEC;
    pub type Pg01Sa = crate::EnumBitfieldStruct<u8, Pg01Sa_SPEC>;
    impl Pg01Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg02Sa_SPEC;
    pub type Pg02Sa = crate::EnumBitfieldStruct<u8, Pg02Sa_SPEC>;
    impl Pg02Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg03Sa_SPEC;
    pub type Pg03Sa = crate::EnumBitfieldStruct<u8, Pg03Sa_SPEC>;
    impl Pg03Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg04Sa_SPEC;
    pub type Pg04Sa = crate::EnumBitfieldStruct<u8, Pg04Sa_SPEC>;
    impl Pg04Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg05Sa_SPEC;
    pub type Pg05Sa = crate::EnumBitfieldStruct<u8, Pg05Sa_SPEC>;
    impl Pg05Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg06Sa_SPEC;
    pub type Pg06Sa = crate::EnumBitfieldStruct<u8, Pg06Sa_SPEC>;
    impl Pg06Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg07Sa_SPEC;
    pub type Pg07Sa = crate::EnumBitfieldStruct<u8, Pg07Sa_SPEC>;
    impl Pg07Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg08Sa_SPEC;
    pub type Pg08Sa = crate::EnumBitfieldStruct<u8, Pg08Sa_SPEC>;
    impl Pg08Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg09Sa_SPEC;
    pub type Pg09Sa = crate::EnumBitfieldStruct<u8, Pg09Sa_SPEC>;
    impl Pg09Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg10Sa_SPEC;
    pub type Pg10Sa = crate::EnumBitfieldStruct<u8, Pg10Sa_SPEC>;
    impl Pg10Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg11Sa_SPEC;
    pub type Pg11Sa = crate::EnumBitfieldStruct<u8, Pg11Sa_SPEC>;
    impl Pg11Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg12Sa_SPEC;
    pub type Pg12Sa = crate::EnumBitfieldStruct<u8, Pg12Sa_SPEC>;
    impl Pg12Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg13Sa_SPEC;
    pub type Pg13Sa = crate::EnumBitfieldStruct<u8, Pg13Sa_SPEC>;
    impl Pg13Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg14Sa_SPEC;
    pub type Pg14Sa = crate::EnumBitfieldStruct<u8, Pg14Sa_SPEC>;
    impl Pg14Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pg15Sa_SPEC;
    pub type Pg15Sa = crate::EnumBitfieldStruct<u8, Pg15Sa_SPEC>;
    impl Pg15Sa {
        #[doc = "Security Attribution is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Security Attribution is 1."]
        pub const _1: Self = Self::new(1);
    }
}
