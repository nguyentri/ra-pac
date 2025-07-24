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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:49:18 +0000

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

    #[doc = "P00%s Pin Function Control Register"]
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

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P000PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P000PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4usize))
        }
    }
    #[inline(always)]
    pub const fn p001pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p002pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p003pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p004pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p005pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p006pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p007pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
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

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6usize))
        }
    }
    #[inline(always)]
    pub const fn p001pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p002pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p003pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p004pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p005pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p006pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p007pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p008pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22usize),
            )
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x7usize))
        }
    }
    #[inline(always)]
    pub const fn p001pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p002pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p003pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p004pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p005pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p006pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p007pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p008pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2ausize))
        }
    }
    #[inline(always)]
    pub const fn p010pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p011pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p012pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p013pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p014pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p015pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3eusize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2busize))
        }
    }
    #[inline(always)]
    pub const fn p010pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p011pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p012pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p013pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p014pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p015pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3fusize),
            )
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn p100pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
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
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x42usize))
        }
    }
    #[inline(always)]
    pub const fn p100pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p101pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x46usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p102pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p103pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p104pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p105pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p106pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p107pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5eusize),
            )
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x43usize))
        }
    }
    #[inline(always)]
    pub const fn p100pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x43usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p101pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x47usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p102pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p103pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p104pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p105pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p106pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p107pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5fusize),
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
                self._svd2pac_as_ptr().add(98usize),
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
                self._svd2pac_as_ptr().add(99usize),
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
                self._svd2pac_as_ptr().add(102usize),
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
                self._svd2pac_as_ptr().add(103usize),
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
                self._svd2pac_as_ptr().add(106usize),
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
                self._svd2pac_as_ptr().add(107usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6eusize))
        }
    }
    #[inline(always)]
    pub const fn p111pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p112pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x72usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p113pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x76usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p114pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p115pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7eusize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6fusize))
        }
    }
    #[inline(always)]
    pub const fn p111pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p112pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x73usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p113pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x77usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p114pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p115pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7fusize),
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
                self._svd2pac_as_ptr().add(130usize),
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
                self._svd2pac_as_ptr().add(131usize),
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
                self._svd2pac_as_ptr().add(134usize),
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
                self._svd2pac_as_ptr().add(135usize),
            )
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW>,
        5,
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

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8ausize))
        }
    }
    #[inline(always)]
    pub const fn p202pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p203pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p204pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x92usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p205pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x96usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p206pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9ausize),
            )
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8busize))
        }
    }
    #[inline(always)]
    pub const fn p202pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p203pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p204pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x93usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p205pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x97usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p206pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9busize),
            )
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb0usize))
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
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb2usize))
        }
    }
    #[inline(always)]
    pub const fn p212pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p213pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p214pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p215pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbeusize),
            )
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb3usize))
        }
    }
    #[inline(always)]
    pub const fn p212pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p213pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p214pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p215pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbfusize),
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
                self._svd2pac_as_ptr().add(194usize),
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
                self._svd2pac_as_ptr().add(195usize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW>,
        7,
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

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc6usize))
        }
    }
    #[inline(always)]
    pub const fn p301pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p302pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p303pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xceusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p304pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p305pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p306pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p307pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdeusize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc7usize))
        }
    }
    #[inline(always)]
    pub const fn p301pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p302pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p303pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p304pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p305pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p306pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p307pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdfusize),
            )
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW>,
        8,
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

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x102usize))
        }
    }
    #[inline(always)]
    pub const fn p400pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x102usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p401pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x106usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p402pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p403pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p404pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x112usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p405pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x116usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p406pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p407pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11eusize),
            )
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x103usize))
        }
    }
    #[inline(always)]
    pub const fn p400pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x103usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p401pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x107usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p402pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p403pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p404pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x113usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p405pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x117usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p406pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p407pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11fusize),
            )
        }
    }

    #[doc = "P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P408Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P408Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P408PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P408PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(290usize),
            )
        }
    }

    #[doc = "P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P408PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P408PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(291usize),
            )
        }
    }

    #[doc = "P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P409Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P409Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P409PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P409PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(294usize),
            )
        }
    }

    #[doc = "P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P409PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P409PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(295usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12ausize))
        }
    }
    #[inline(always)]
    pub const fn p410pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p411pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p412pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x132usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p413pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x136usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p414pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p415pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13eusize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12busize))
        }
    }
    #[inline(always)]
    pub const fn p410pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p411pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p412pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x133usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p413pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x137usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p414pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p415pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P4PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13fusize),
            )
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }
    #[inline(always)]
    pub const fn p500pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p501pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p502pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p503pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p504pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p505pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x154usize),
            )
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x142usize))
        }
    }
    #[inline(always)]
    pub const fn p500pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x142usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p501pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x146usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p502pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p503pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p504pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x152usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p505pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x156usize),
            )
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x143usize))
        }
    }
    #[inline(always)]
    pub const fn p500pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x143usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p501pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x147usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p502pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p503pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p504pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x153usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p505pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P50PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x157usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a2usize))
        }
    }
    #[inline(always)]
    pub const fn p608pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p609pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a6usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a3usize))
        }
    }
    #[inline(always)]
    pub const fn p608pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p609pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P60PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P60PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a7usize),
            )
        }
    }

    #[doc = "P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P610Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P610Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(424usize),
            )
        }
    }

    #[doc = "P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P610PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P610PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(426usize),
            )
        }
    }

    #[doc = "P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P610PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P610PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(427usize),
            )
        }
    }

    #[doc = "P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P708Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P708Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(480usize),
            )
        }
    }

    #[doc = "P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P708PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P708PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(482usize),
            )
        }
    }

    #[doc = "P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P708PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P708PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(483usize),
            )
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x220usize))
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
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x222usize))
        }
    }
    #[inline(always)]
    pub const fn p808pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x222usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p809pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x226usize),
            )
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x223usize))
        }
    }
    #[inline(always)]
    pub const fn p808pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x223usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p809pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x227usize),
            )
        }
    }

    #[doc = "P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p9pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x278usize))
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
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x27ausize))
        }
    }
    #[inline(always)]
    pub const fn p914pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p915pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27eusize),
            )
        }
    }

    #[doc = "P9%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p9pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x27busize))
        }
    }
    #[inline(always)]
    pub const fn p914pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p915pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P9PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27fusize),
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

#[doc = "P00%s Pin Function Control Register"]
pub type P000Pfs = crate::RegValueT<P000Pfs_SPEC>;

impl P000Pfs {
    #[doc = "Port Function Select\nThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P000Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p000pfs::Dscr,
        p000pfs::Dscr,
        P000Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p000pfs::Dscr,
            p000pfs::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P000Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, P000Pfs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
}
impl ::core::default::Default for P000Pfs {
    #[inline(always)]
    fn default() -> P000Pfs {
        <crate::RegValueT<P000Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p000pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive."]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000PfsHa_SPEC;
impl crate::sealed::RegSpec for P000PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P00%s Pin Function Control Register"]
pub type P000PfsHa = crate::RegValueT<P000PfsHa_SPEC>;

impl P000PfsHa {
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p000pfs_ha::Dscr,
        p000pfs_ha::Dscr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p000pfs_ha::Dscr,
            p000pfs_ha::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P000PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P000PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P000PfsHa {
    #[inline(always)]
    fn default() -> P000PfsHa {
        <crate::RegValueT<P000PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p000pfs_ha {

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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive."]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000PfsBy_SPEC;
impl crate::sealed::RegSpec for P000PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P00%s Pin Function Control Register"]
pub type P000PfsBy = crate::RegValueT<P000PfsBy_SPEC>;

impl P000PfsBy {
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P000PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P000PfsBy_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P000PfsBy {
    #[inline(always)]
    fn default() -> P000PfsBy {
        <crate::RegValueT<P000PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p000pfs_by {

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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
    #[doc = "Port Function Select \nThese bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P108Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Event on Failing"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p108pfs::Eof,
        p108pfs::Eof,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p108pfs::Eof,
            p108pfs::Eof,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p108pfs::Eor,
        p108pfs::Eor,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p108pfs::Eor,
            p108pfs::Eor,
            P108Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p108pfs::Dscr,
        p108pfs::Dscr,
        P108Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p108pfs::Dscr,
            p108pfs::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P108Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, P108Pfs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
}
impl ::core::default::Default for P108Pfs {
    #[inline(always)]
    fn default() -> P108Pfs {
        <crate::RegValueT<P108Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}
pub mod p108pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect failing edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive."]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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

    #[doc = "Event on Failing"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p108pfs_ha::Eof,
        p108pfs_ha::Eof,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p108pfs_ha::Eof,
            p108pfs_ha::Eof,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p108pfs_ha::Eor,
        p108pfs_ha::Eor,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p108pfs_ha::Eor,
            p108pfs_ha::Eor,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p108pfs_ha::Dscr,
        p108pfs_ha::Dscr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p108pfs_ha::Dscr,
            p108pfs_ha::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P108PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P108PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P108PfsHa {
    #[inline(always)]
    fn default() -> P108PfsHa {
        <crate::RegValueT<P108PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p108pfs_ha {

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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect failing edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive."]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P108PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P108PfsBy_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P108PfsBy {
    #[inline(always)]
    fn default() -> P108PfsBy {
        <crate::RegValueT<P108PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p108pfs_by {

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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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

impl P109Pfs {
    #[doc = "Port Function Select \nThese bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P109Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P109Pfs_SPEC,crate::common::RW>::from_register(self,0)
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
        p109pfs::Pmr,
        p109pfs::Pmr,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p109pfs::Pmr,
            p109pfs::Pmr,
            P109Pfs_SPEC,
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
        p109pfs::Asel,
        p109pfs::Asel,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p109pfs::Asel,
            p109pfs::Asel,
            P109Pfs_SPEC,
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
        p109pfs::Isel,
        p109pfs::Isel,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p109pfs::Isel,
            p109pfs::Isel,
            P109Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Failing"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p109pfs::Eof,
        p109pfs::Eof,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p109pfs::Eof,
            p109pfs::Eof,
            P109Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p109pfs::Eor,
        p109pfs::Eor,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p109pfs::Eor,
            p109pfs::Eor,
            P109Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p109pfs::Dscr,
        p109pfs::Dscr,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p109pfs::Dscr,
            p109pfs::Dscr,
            P109Pfs_SPEC,
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
        p109pfs::Ncodr,
        p109pfs::Ncodr,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p109pfs::Ncodr,
            p109pfs::Ncodr,
            P109Pfs_SPEC,
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
        p109pfs::Pcr,
        p109pfs::Pcr,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p109pfs::Pcr,
            p109pfs::Pcr,
            P109Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P109Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, P109Pfs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        p109pfs::Pdr,
        p109pfs::Pdr,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p109pfs::Pdr,
            p109pfs::Pdr,
            P109Pfs_SPEC,
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
        p109pfs::Pidr,
        p109pfs::Pidr,
        P109Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p109pfs::Pidr,
            p109pfs::Pidr,
            P109Pfs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p109pfs::Podr,
        p109pfs::Podr,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p109pfs::Podr,
            p109pfs::Podr,
            P109Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P109Pfs {
    #[inline(always)]
    fn default() -> P109Pfs {
        <crate::RegValueT<P109Pfs_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod p109pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect failing edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive."]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
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

impl P109PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p109pfs_ha::Asel,
        p109pfs_ha::Asel,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p109pfs_ha::Asel,
            p109pfs_ha::Asel,
            P109PfsHa_SPEC,
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
        p109pfs_ha::Isel,
        p109pfs_ha::Isel,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p109pfs_ha::Isel,
            p109pfs_ha::Isel,
            P109PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Failing"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p109pfs_ha::Eof,
        p109pfs_ha::Eof,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p109pfs_ha::Eof,
            p109pfs_ha::Eof,
            P109PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p109pfs_ha::Eor,
        p109pfs_ha::Eor,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p109pfs_ha::Eor,
            p109pfs_ha::Eor,
            P109PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p109pfs_ha::Dscr,
        p109pfs_ha::Dscr,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p109pfs_ha::Dscr,
            p109pfs_ha::Dscr,
            P109PfsHa_SPEC,
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
        p109pfs_ha::Ncodr,
        p109pfs_ha::Ncodr,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p109pfs_ha::Ncodr,
            p109pfs_ha::Ncodr,
            P109PfsHa_SPEC,
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
        p109pfs_ha::Pcr,
        p109pfs_ha::Pcr,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p109pfs_ha::Pcr,
            p109pfs_ha::Pcr,
            P109PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P109PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P109PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
        p109pfs_ha::Pdr,
        p109pfs_ha::Pdr,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p109pfs_ha::Pdr,
            p109pfs_ha::Pdr,
            P109PfsHa_SPEC,
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
        p109pfs_ha::Pidr,
        p109pfs_ha::Pidr,
        P109PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p109pfs_ha::Pidr,
            p109pfs_ha::Pidr,
            P109PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p109pfs_ha::Podr,
        p109pfs_ha::Podr,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p109pfs_ha::Podr,
            p109pfs_ha::Podr,
            P109PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P109PfsHa {
    #[inline(always)]
    fn default() -> P109PfsHa {
        <crate::RegValueT<P109PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p109pfs_ha {

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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect failing edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "No effected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive."]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
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

impl P109PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p109pfs_by::Ncodr,
        p109pfs_by::Ncodr,
        P109PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p109pfs_by::Ncodr,
            p109pfs_by::Ncodr,
            P109PfsBy_SPEC,
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
        p109pfs_by::Pcr,
        p109pfs_by::Pcr,
        P109PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p109pfs_by::Pcr,
            p109pfs_by::Pcr,
            P109PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P109PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P109PfsBy_SPEC,crate::common::RW>::from_register(self,0)
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
        p109pfs_by::Pdr,
        p109pfs_by::Pdr,
        P109PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p109pfs_by::Pdr,
            p109pfs_by::Pdr,
            P109PfsBy_SPEC,
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
        p109pfs_by::Pidr,
        p109pfs_by::Pidr,
        P109PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p109pfs_by::Pidr,
            p109pfs_by::Pidr,
            P109PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p109pfs_by::Podr,
        p109pfs_by::Podr,
        P109PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p109pfs_by::Podr,
            p109pfs_by::Podr,
            P109PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P109PfsBy {
    #[inline(always)]
    fn default() -> P109PfsBy {
        <crate::RegValueT<P109PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p109pfs_by {

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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
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

impl NoBitfieldReg<P110Pfs_SPEC> for P110Pfs {}
impl ::core::default::Default for P110Pfs {
    #[inline(always)]
    fn default() -> P110Pfs {
        <crate::RegValueT<P110Pfs_SPEC> as RegisterValue<_>>::new(65552)
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

impl NoBitfieldReg<P110PfsHa_SPEC> for P110PfsHa {}
impl ::core::default::Default for P110PfsHa {
    #[inline(always)]
    fn default() -> P110PfsHa {
        <crate::RegValueT<P110PfsHa_SPEC> as RegisterValue<_>>::new(16)
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

impl NoBitfieldReg<P110PfsBy_SPEC> for P110PfsBy {}
impl ::core::default::Default for P110PfsBy {
    #[inline(always)]
    fn default() -> P110PfsBy {
        <crate::RegValueT<P110PfsBy_SPEC> as RegisterValue<_>>::new(16)
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
    #[doc = "Port Function Select \nThese bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P201Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p201pfs::Eof,
        p201pfs::Eof,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p201pfs::Eof,
            p201pfs::Eof,
            P201Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p201pfs::Eor,
        p201pfs::Eor,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p201pfs::Eor,
            p201pfs::Eor,
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
        0x1,
        1,
        0,
        p201pfs::Dscr,
        p201pfs::Dscr,
        P201Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p201pfs::Dscr,
            p201pfs::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P201Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, P201Pfs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
}
impl ::core::default::Default for P201Pfs {
    #[inline(always)]
    fn default() -> P201Pfs {
        <crate::RegValueT<P201Pfs_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "High drive"]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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

    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p201pfs_ha::Eof,
        p201pfs_ha::Eof,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p201pfs_ha::Eof,
            p201pfs_ha::Eof,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p201pfs_ha::Eor,
        p201pfs_ha::Eor,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p201pfs_ha::Eor,
            p201pfs_ha::Eor,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p201pfs_ha::Dscr,
        p201pfs_ha::Dscr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p201pfs_ha::Dscr,
            p201pfs_ha::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P201PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P201PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P201PfsHa {
    #[inline(always)]
    fn default() -> P201PfsHa {
        <crate::RegValueT<P201PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs_ha {

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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "High drive"]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P201PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P201PfsBy_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P201PfsBy {
    #[inline(always)]
    fn default() -> P201PfsBy {
        <crate::RegValueT<P201PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs_by {

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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
pub struct P408Pfs_SPEC;
impl crate::sealed::RegSpec for P408Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P408 Pin Function Control Register"]
pub type P408Pfs = crate::RegValueT<P408Pfs_SPEC>;

impl P408Pfs {
    #[doc = "Port Function Select \nThese bits select the peripheral function. For individual pin functions, see the setting table."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P408Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P408Pfs_SPEC,crate::common::RW>::from_register(self,0)
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
        p408pfs::Pmr,
        p408pfs::Pmr,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p408pfs::Pmr,
            p408pfs::Pmr,
            P408Pfs_SPEC,
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
        p408pfs::Asel,
        p408pfs::Asel,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p408pfs::Asel,
            p408pfs::Asel,
            P408Pfs_SPEC,
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
        p408pfs::Isel,
        p408pfs::Isel,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p408pfs::Isel,
            p408pfs::Isel,
            P408Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p408pfs::Eof,
        p408pfs::Eof,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p408pfs::Eof,
            p408pfs::Eof,
            P408Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p408pfs::Eor,
        p408pfs::Eor,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p408pfs::Eor,
            p408pfs::Eor,
            P408Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p408pfs::Dscr1,
        p408pfs::Dscr1,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p408pfs::Dscr1,
            p408pfs::Dscr1,
            P408Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p408pfs::Dscr,
        p408pfs::Dscr,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p408pfs::Dscr,
            p408pfs::Dscr,
            P408Pfs_SPEC,
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
        p408pfs::Ncodr,
        p408pfs::Ncodr,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p408pfs::Ncodr,
            p408pfs::Ncodr,
            P408Pfs_SPEC,
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
        p408pfs::Pcr,
        p408pfs::Pcr,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p408pfs::Pcr,
            p408pfs::Pcr,
            P408Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P408Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, P408Pfs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        p408pfs::Pdr,
        p408pfs::Pdr,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p408pfs::Pdr,
            p408pfs::Pdr,
            P408Pfs_SPEC,
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
        p408pfs::Pidr,
        p408pfs::Pidr,
        P408Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p408pfs::Pidr,
            p408pfs::Pidr,
            P408Pfs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p408pfs::Podr,
        p408pfs::Podr,
        P408Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p408pfs::Podr,
            p408pfs::Podr,
            P408Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P408Pfs {
    #[inline(always)]
    fn default() -> P408Pfs {
        <crate::RegValueT<P408Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p408pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Uses the pin as an I/O port for peripheral functions."]
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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr1_SPEC;
    pub type Dscr1 = crate::EnumBitfieldStruct<u8, Dscr1_SPEC>;
    impl Dscr1 {
        #[doc = "Low drive(DSCR = 0)/Middle drive(DSCR = 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive for IIC Fast-mode(DSCR = 0)/Setting prohibited(DSCR = 1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive(DSCR1 = 0)/Middle drive for llC Fast-mode(DSCR1 = 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive(DSCR1 = 0)/Setting prohibited(DSCR1 = 1)"]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P408PfsHa_SPEC;
impl crate::sealed::RegSpec for P408PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P408 Pin Function Control Register"]
pub type P408PfsHa = crate::RegValueT<P408PfsHa_SPEC>;

impl P408PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p408pfs_ha::Asel,
        p408pfs_ha::Asel,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p408pfs_ha::Asel,
            p408pfs_ha::Asel,
            P408PfsHa_SPEC,
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
        p408pfs_ha::Isel,
        p408pfs_ha::Isel,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p408pfs_ha::Isel,
            p408pfs_ha::Isel,
            P408PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p408pfs_ha::Eof,
        p408pfs_ha::Eof,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p408pfs_ha::Eof,
            p408pfs_ha::Eof,
            P408PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p408pfs_ha::Eor,
        p408pfs_ha::Eor,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p408pfs_ha::Eor,
            p408pfs_ha::Eor,
            P408PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        p408pfs_ha::Dscr1,
        p408pfs_ha::Dscr1,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p408pfs_ha::Dscr1,
            p408pfs_ha::Dscr1,
            P408PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p408pfs_ha::Dscr,
        p408pfs_ha::Dscr,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p408pfs_ha::Dscr,
            p408pfs_ha::Dscr,
            P408PfsHa_SPEC,
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
        p408pfs_ha::Ncodr,
        p408pfs_ha::Ncodr,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p408pfs_ha::Ncodr,
            p408pfs_ha::Ncodr,
            P408PfsHa_SPEC,
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
        p408pfs_ha::Pcr,
        p408pfs_ha::Pcr,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p408pfs_ha::Pcr,
            p408pfs_ha::Pcr,
            P408PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P408PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P408PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
        p408pfs_ha::Pdr,
        p408pfs_ha::Pdr,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p408pfs_ha::Pdr,
            p408pfs_ha::Pdr,
            P408PfsHa_SPEC,
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
        p408pfs_ha::Pidr,
        p408pfs_ha::Pidr,
        P408PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p408pfs_ha::Pidr,
            p408pfs_ha::Pidr,
            P408PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p408pfs_ha::Podr,
        p408pfs_ha::Podr,
        P408PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p408pfs_ha::Podr,
            p408pfs_ha::Podr,
            P408PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P408PfsHa {
    #[inline(always)]
    fn default() -> P408PfsHa {
        <crate::RegValueT<P408PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p408pfs_ha {

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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr1_SPEC;
    pub type Dscr1 = crate::EnumBitfieldStruct<u8, Dscr1_SPEC>;
    impl Dscr1 {
        #[doc = "Low drive(DSCR = 0)/Middle drive(DSCR = 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive for IIC Fast-mode(DSCR = 0)/Setting prohibited(DSCR = 1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive(DSCR1 = 0)/Middle drive for llC Fast-mode(DSCR1 = 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle drive(DSCR1 = 0)/Setting prohibited(DSCR1 = 1)"]
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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P408PfsBy_SPEC;
impl crate::sealed::RegSpec for P408PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P408 Pin Function Control Register"]
pub type P408PfsBy = crate::RegValueT<P408PfsBy_SPEC>;

impl P408PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p408pfs_by::Ncodr,
        p408pfs_by::Ncodr,
        P408PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p408pfs_by::Ncodr,
            p408pfs_by::Ncodr,
            P408PfsBy_SPEC,
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
        p408pfs_by::Pcr,
        p408pfs_by::Pcr,
        P408PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p408pfs_by::Pcr,
            p408pfs_by::Pcr,
            P408PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P408PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P408PfsBy_SPEC,crate::common::RW>::from_register(self,0)
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
        p408pfs_by::Pdr,
        p408pfs_by::Pdr,
        P408PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p408pfs_by::Pdr,
            p408pfs_by::Pdr,
            P408PfsBy_SPEC,
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
        p408pfs_by::Pidr,
        p408pfs_by::Pidr,
        P408PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p408pfs_by::Pidr,
            p408pfs_by::Pidr,
            P408PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p408pfs_by::Podr,
        p408pfs_by::Podr,
        P408PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p408pfs_by::Podr,
            p408pfs_by::Podr,
            P408PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P408PfsBy {
    #[inline(always)]
    fn default() -> P408PfsBy {
        <crate::RegValueT<P408PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p408pfs_by {

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
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an input pull-up."]
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
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);

        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P409Pfs_SPEC;
impl crate::sealed::RegSpec for P409Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P409 Pin Function Control Register"]
pub type P409Pfs = crate::RegValueT<P409Pfs_SPEC>;

impl NoBitfieldReg<P409Pfs_SPEC> for P409Pfs {}
impl ::core::default::Default for P409Pfs {
    #[inline(always)]
    fn default() -> P409Pfs {
        <crate::RegValueT<P409Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P409PfsHa_SPEC;
impl crate::sealed::RegSpec for P409PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P409 Pin Function Control Register"]
pub type P409PfsHa = crate::RegValueT<P409PfsHa_SPEC>;

impl NoBitfieldReg<P409PfsHa_SPEC> for P409PfsHa {}
impl ::core::default::Default for P409PfsHa {
    #[inline(always)]
    fn default() -> P409PfsHa {
        <crate::RegValueT<P409PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P409PfsBy_SPEC;
impl crate::sealed::RegSpec for P409PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P409 Pin Function Control Register"]
pub type P409PfsBy = crate::RegValueT<P409PfsBy_SPEC>;

impl NoBitfieldReg<P409PfsBy_SPEC> for P409PfsBy {}
impl ::core::default::Default for P409PfsBy {
    #[inline(always)]
    fn default() -> P409PfsBy {
        <crate::RegValueT<P409PfsBy_SPEC> as RegisterValue<_>>::new(0)
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
pub struct P610Pfs_SPEC;
impl crate::sealed::RegSpec for P610Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P610 Pin Function Control Register"]
pub type P610Pfs = crate::RegValueT<P610Pfs_SPEC>;

impl NoBitfieldReg<P610Pfs_SPEC> for P610Pfs {}
impl ::core::default::Default for P610Pfs {
    #[inline(always)]
    fn default() -> P610Pfs {
        <crate::RegValueT<P610Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P610PfsHa_SPEC;
impl crate::sealed::RegSpec for P610PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P610 Pin Function Control Register"]
pub type P610PfsHa = crate::RegValueT<P610PfsHa_SPEC>;

impl NoBitfieldReg<P610PfsHa_SPEC> for P610PfsHa {}
impl ::core::default::Default for P610PfsHa {
    #[inline(always)]
    fn default() -> P610PfsHa {
        <crate::RegValueT<P610PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P610PfsBy_SPEC;
impl crate::sealed::RegSpec for P610PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P610 Pin Function Control Register"]
pub type P610PfsBy = crate::RegValueT<P610PfsBy_SPEC>;

impl NoBitfieldReg<P610PfsBy_SPEC> for P610PfsBy {}
impl ::core::default::Default for P610PfsBy {
    #[inline(always)]
    fn default() -> P610PfsBy {
        <crate::RegValueT<P610PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P708Pfs_SPEC;
impl crate::sealed::RegSpec for P708Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P708 Pin Function Control Register"]
pub type P708Pfs = crate::RegValueT<P708Pfs_SPEC>;

impl NoBitfieldReg<P708Pfs_SPEC> for P708Pfs {}
impl ::core::default::Default for P708Pfs {
    #[inline(always)]
    fn default() -> P708Pfs {
        <crate::RegValueT<P708Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P708PfsHa_SPEC;
impl crate::sealed::RegSpec for P708PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P708 Pin Function Control Register"]
pub type P708PfsHa = crate::RegValueT<P708PfsHa_SPEC>;

impl NoBitfieldReg<P708PfsHa_SPEC> for P708PfsHa {}
impl ::core::default::Default for P708PfsHa {
    #[inline(always)]
    fn default() -> P708PfsHa {
        <crate::RegValueT<P708PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P708PfsBy_SPEC;
impl crate::sealed::RegSpec for P708PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P708 Pin Function Control Register"]
pub type P708PfsBy = crate::RegValueT<P708PfsBy_SPEC>;

impl NoBitfieldReg<P708PfsBy_SPEC> for P708PfsBy {}
impl ::core::default::Default for P708PfsBy {
    #[inline(always)]
    fn default() -> P708PfsBy {
        <crate::RegValueT<P708PfsBy_SPEC> as RegisterValue<_>>::new(0)
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
        <crate::RegValueT<P9Pfs_SPEC> as RegisterValue<_>>::new(65536)
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
