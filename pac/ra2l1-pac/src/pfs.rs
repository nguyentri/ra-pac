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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:47:04 +0000

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

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn p000pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
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

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2usize))
        }
    }
    #[inline(always)]
    pub const fn p000pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2usize),
            )
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

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3usize))
        }
    }
    #[inline(always)]
    pub const fn p000pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3usize),
            )
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

    #[doc = "Port 0%s Pin Function Select Register"]
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

    #[doc = "Port 0%s Pin Function Select Register"]
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

    #[doc = "Port 0%s Pin Function Select Register"]
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

    #[doc = "Port 10%s Pin Function Select Register"]
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

    #[doc = "Port 10%s Pin Function Select Register"]
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

    #[doc = "Port 10%s Pin Function Select Register"]
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

    #[doc = "Port 108 Pin Function Select Register"]
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

    #[doc = "Port 108 Pin Function Select Register"]
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

    #[doc = "Port 108 Pin Function Select Register"]
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

    #[doc = "Port 109 Pin Function Select Register"]
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

    #[doc = "Port 109 Pin Function Select Register"]
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

    #[doc = "Port 109 Pin Function Select Register"]
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

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x68usize))
        }
    }
    #[inline(always)]
    pub const fn p110pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
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

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6ausize))
        }
    }
    #[inline(always)]
    pub const fn p110pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ausize),
            )
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

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6busize))
        }
    }
    #[inline(always)]
    pub const fn p110pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6busize),
            )
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

    #[doc = "Port 200 Pin Function Select Register"]
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

    #[doc = "Port 200 Pin Function Select Register"]
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

    #[doc = "Port 200 Pin Function Select Register"]
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

    #[doc = "Port 201 Pin Function Select Register"]
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

    #[doc = "Port 201 Pin Function Select Register"]
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

    #[doc = "Port 201 Pin Function Select Register"]
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

    #[doc = "Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW>,
        7,
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

    #[doc = "Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW>,
        7,
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
    #[inline(always)]
    pub const fn p207pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p208pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa2usize),
            )
        }
    }

    #[doc = "Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW>,
        7,
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
    #[inline(always)]
    pub const fn p207pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn p208pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa3usize),
            )
        }
    }

    #[doc = "Port 2%s Pin Function Select Register"]
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

    #[doc = "Port 2%s Pin Function Select Register"]
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

    #[doc = "Port 2%s Pin Function Select Register"]
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

    #[doc = "Port 300 Pin Function Select Register"]
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

    #[doc = "Port 300 Pin Function Select Register"]
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

    #[doc = "Port 300 Pin Function Select Register"]
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

    #[doc = "Port 30%s Pin Function Select Register"]
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

    #[doc = "Port 30%s Pin Function Select Register"]
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

    #[doc = "Port 30%s Pin Function Select Register"]
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

    #[doc = "Port 40%s Pin Function Select Register"]
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

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW>,
        10,
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
    #[inline(always)]
    pub const fn p408pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x122usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p409pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x126usize),
            )
        }
    }

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW>,
        10,
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
    #[inline(always)]
    pub const fn p408pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x123usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p409pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P40PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x127usize),
            )
        }
    }

    #[doc = "Port 4%s Pin Function Select Register"]
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

    #[doc = "Port 4%s Pin Function Select Register"]
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

    #[doc = "Port 4%s Pin Function Select Register"]
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

    #[doc = "Port 50%s Pin Function Select Register"]
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

    #[doc = "Port 50%s Pin Function Select Register"]
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

    #[doc = "Port 50%s Pin Function Select Register"]
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

    #[doc = "Port 60%s Pin Function Select Register"]
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

    #[doc = "Port 60%s Pin Function Select Register"]
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

    #[doc = "Port 60%s Pin Function Select Register"]
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

    #[doc = "Port 610 Pin Function Select Register"]
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

    #[doc = "Port 610 Pin Function Select Register"]
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

    #[doc = "Port 610 Pin Function Select Register"]
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

    #[doc = "Port 708 Pin Function Select Register"]
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

    #[doc = "Port 708 Pin Function Select Register"]
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

    #[doc = "Port 708 Pin Function Select Register"]
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

    #[doc = "Port 714 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p714pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P714Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P714Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(504usize),
            )
        }
    }

    #[doc = "Port 714 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p714pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P714PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P714PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(506usize),
            )
        }
    }

    #[doc = "Port 714 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p714pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P714PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P714PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(507usize),
            )
        }
    }

    #[doc = "Port 80%s Pin Function Select Register"]
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

    #[doc = "Port 80%s Pin Function Select Register"]
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

    #[doc = "Port 80%s Pin Function Select Register"]
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

    #[doc = "Write-Protect Register"]
    #[inline(always)]
    pub const fn pwpr(&self) -> &'static crate::common::Reg<self::Pwpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1283usize),
            )
        }
    }

    #[doc = "Port Read Wait Control Register"]
    #[inline(always)]
    pub const fn prwcntr(
        &self,
    ) -> &'static crate::common::Reg<self::Prwcntr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prwcntr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1295usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00Pfs_SPEC;
impl crate::sealed::RegSpec for P00Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 00%s Pin Function Select Register"]
pub type P00Pfs = crate::RegValueT<P00Pfs_SPEC>;

impl P00Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p00pfs::Podr,
        p00pfs::Podr,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p00pfs::Podr,
            p00pfs::Podr,
            P00Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p00pfs::Pidr,
        p00pfs::Pidr,
        P00Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p00pfs::Pidr,
            p00pfs::Pidr,
            P00Pfs_SPEC,
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
        p00pfs::Pdr,
        p00pfs::Pdr,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p00pfs::Pdr,
            p00pfs::Pdr,
            P00Pfs_SPEC,
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
        p00pfs::Pcr,
        p00pfs::Pcr,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p00pfs::Pcr,
            p00pfs::Pcr,
            P00Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p00pfs::Ncodr,
        p00pfs::Ncodr,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p00pfs::Ncodr,
            p00pfs::Ncodr,
            P00Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p00pfs::Isel,
        p00pfs::Isel,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p00pfs::Isel,
            p00pfs::Isel,
            P00Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p00pfs::Asel,
        p00pfs::Asel,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p00pfs::Asel,
            p00pfs::Asel,
            P00Pfs_SPEC,
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
        p00pfs::Pmr,
        p00pfs::Pmr,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p00pfs::Pmr,
            p00pfs::Pmr,
            P00Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P00Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P00Pfs {
    #[inline(always)]
    fn default() -> P00Pfs {
        <crate::RegValueT<P00Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p00pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsHa_SPEC;
impl crate::sealed::RegSpec for P00PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 00%s Pin Function Select Register"]
pub type P00PfsHa = crate::RegValueT<P00PfsHa_SPEC>;

impl P00PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p00pfs_ha::Podr,
        p00pfs_ha::Podr,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p00pfs_ha::Podr,
            p00pfs_ha::Podr,
            P00PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p00pfs_ha::Pidr,
        p00pfs_ha::Pidr,
        P00PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p00pfs_ha::Pidr,
            p00pfs_ha::Pidr,
            P00PfsHa_SPEC,
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
        p00pfs_ha::Pdr,
        p00pfs_ha::Pdr,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p00pfs_ha::Pdr,
            p00pfs_ha::Pdr,
            P00PfsHa_SPEC,
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
        p00pfs_ha::Pcr,
        p00pfs_ha::Pcr,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p00pfs_ha::Pcr,
            p00pfs_ha::Pcr,
            P00PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p00pfs_ha::Ncodr,
        p00pfs_ha::Ncodr,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p00pfs_ha::Ncodr,
            p00pfs_ha::Ncodr,
            P00PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p00pfs_ha::Isel,
        p00pfs_ha::Isel,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p00pfs_ha::Isel,
            p00pfs_ha::Isel,
            P00PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p00pfs_ha::Asel,
        p00pfs_ha::Asel,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p00pfs_ha::Asel,
            p00pfs_ha::Asel,
            P00PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P00PfsHa {
    #[inline(always)]
    fn default() -> P00PfsHa {
        <crate::RegValueT<P00PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p00pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsBy_SPEC;
impl crate::sealed::RegSpec for P00PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 00%s Pin Function Select Register"]
pub type P00PfsBy = crate::RegValueT<P00PfsBy_SPEC>;

impl P00PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p00pfs_by::Podr,
        p00pfs_by::Podr,
        P00PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p00pfs_by::Podr,
            p00pfs_by::Podr,
            P00PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p00pfs_by::Pidr,
        p00pfs_by::Pidr,
        P00PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p00pfs_by::Pidr,
            p00pfs_by::Pidr,
            P00PfsBy_SPEC,
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
        p00pfs_by::Pdr,
        p00pfs_by::Pdr,
        P00PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p00pfs_by::Pdr,
            p00pfs_by::Pdr,
            P00PfsBy_SPEC,
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
        p00pfs_by::Pcr,
        p00pfs_by::Pcr,
        P00PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p00pfs_by::Pcr,
            p00pfs_by::Pcr,
            P00PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p00pfs_by::Ncodr,
        p00pfs_by::Ncodr,
        P00PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p00pfs_by::Ncodr,
            p00pfs_by::Ncodr,
            P00PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P00PfsBy {
    #[inline(always)]
    fn default() -> P00PfsBy {
        <crate::RegValueT<P00PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p00pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0Pfs_SPEC;
impl crate::sealed::RegSpec for P0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 0%s Pin Function Select Register"]
pub type P0Pfs = crate::RegValueT<P0Pfs_SPEC>;

impl P0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p0pfs::Podr,
        p0pfs::Podr,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p0pfs::Podr,
            p0pfs::Podr,
            P0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p0pfs::Pidr,
        p0pfs::Pidr,
        P0Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p0pfs::Pidr,
            p0pfs::Pidr,
            P0Pfs_SPEC,
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
        p0pfs::Pdr,
        p0pfs::Pdr,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p0pfs::Pdr,
            p0pfs::Pdr,
            P0Pfs_SPEC,
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
        p0pfs::Pcr,
        p0pfs::Pcr,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p0pfs::Pcr,
            p0pfs::Pcr,
            P0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p0pfs::Ncodr,
        p0pfs::Ncodr,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p0pfs::Ncodr,
            p0pfs::Ncodr,
            P0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p0pfs::Isel,
        p0pfs::Isel,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p0pfs::Isel,
            p0pfs::Isel,
            P0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p0pfs::Asel,
        p0pfs::Asel,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p0pfs::Asel,
            p0pfs::Asel,
            P0Pfs_SPEC,
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
        p0pfs::Pmr,
        p0pfs::Pmr,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p0pfs::Pmr,
            p0pfs::Pmr,
            P0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P0Pfs {
    #[inline(always)]
    fn default() -> P0Pfs {
        <crate::RegValueT<P0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p0pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsHa_SPEC;
impl crate::sealed::RegSpec for P0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 0%s Pin Function Select Register"]
pub type P0PfsHa = crate::RegValueT<P0PfsHa_SPEC>;

impl P0PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p0pfs_ha::Podr,
        p0pfs_ha::Podr,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p0pfs_ha::Podr,
            p0pfs_ha::Podr,
            P0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p0pfs_ha::Pidr,
        p0pfs_ha::Pidr,
        P0PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p0pfs_ha::Pidr,
            p0pfs_ha::Pidr,
            P0PfsHa_SPEC,
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
        p0pfs_ha::Pdr,
        p0pfs_ha::Pdr,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p0pfs_ha::Pdr,
            p0pfs_ha::Pdr,
            P0PfsHa_SPEC,
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
        p0pfs_ha::Pcr,
        p0pfs_ha::Pcr,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p0pfs_ha::Pcr,
            p0pfs_ha::Pcr,
            P0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p0pfs_ha::Ncodr,
        p0pfs_ha::Ncodr,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p0pfs_ha::Ncodr,
            p0pfs_ha::Ncodr,
            P0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p0pfs_ha::Isel,
        p0pfs_ha::Isel,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p0pfs_ha::Isel,
            p0pfs_ha::Isel,
            P0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p0pfs_ha::Asel,
        p0pfs_ha::Asel,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p0pfs_ha::Asel,
            p0pfs_ha::Asel,
            P0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0PfsHa {
    #[inline(always)]
    fn default() -> P0PfsHa {
        <crate::RegValueT<P0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p0pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsBy_SPEC;
impl crate::sealed::RegSpec for P0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 0%s Pin Function Select Register"]
pub type P0PfsBy = crate::RegValueT<P0PfsBy_SPEC>;

impl P0PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p0pfs_by::Podr,
        p0pfs_by::Podr,
        P0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p0pfs_by::Podr,
            p0pfs_by::Podr,
            P0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p0pfs_by::Pidr,
        p0pfs_by::Pidr,
        P0PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p0pfs_by::Pidr,
            p0pfs_by::Pidr,
            P0PfsBy_SPEC,
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
        p0pfs_by::Pdr,
        p0pfs_by::Pdr,
        P0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p0pfs_by::Pdr,
            p0pfs_by::Pdr,
            P0PfsBy_SPEC,
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
        p0pfs_by::Pcr,
        p0pfs_by::Pcr,
        P0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p0pfs_by::Pcr,
            p0pfs_by::Pcr,
            P0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p0pfs_by::Ncodr,
        p0pfs_by::Ncodr,
        P0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p0pfs_by::Ncodr,
            p0pfs_by::Ncodr,
            P0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0PfsBy {
    #[inline(always)]
    fn default() -> P0PfsBy {
        <crate::RegValueT<P0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p0pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10Pfs_SPEC;
impl crate::sealed::RegSpec for P10Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 10%s Pin Function Select Register"]
pub type P10Pfs = crate::RegValueT<P10Pfs_SPEC>;

impl P10Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p10pfs::Podr,
        p10pfs::Podr,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p10pfs::Podr,
            p10pfs::Podr,
            P10Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p10pfs::Pidr,
        p10pfs::Pidr,
        P10Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p10pfs::Pidr,
            p10pfs::Pidr,
            P10Pfs_SPEC,
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
        p10pfs::Pdr,
        p10pfs::Pdr,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p10pfs::Pdr,
            p10pfs::Pdr,
            P10Pfs_SPEC,
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
        p10pfs::Pcr,
        p10pfs::Pcr,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p10pfs::Pcr,
            p10pfs::Pcr,
            P10Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p10pfs::Ncodr,
        p10pfs::Ncodr,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p10pfs::Ncodr,
            p10pfs::Ncodr,
            P10Pfs_SPEC,
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
        p10pfs::Eofr,
        p10pfs::Eofr,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p10pfs::Eofr,
            p10pfs::Eofr,
            P10Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p10pfs::Isel,
        p10pfs::Isel,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p10pfs::Isel,
            p10pfs::Isel,
            P10Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p10pfs::Asel,
        p10pfs::Asel,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p10pfs::Asel,
            p10pfs::Asel,
            P10Pfs_SPEC,
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
        p10pfs::Pmr,
        p10pfs::Pmr,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p10pfs::Pmr,
            p10pfs::Pmr,
            P10Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P10Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P10Pfs {
    #[inline(always)]
    fn default() -> P10Pfs {
        <crate::RegValueT<P10Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p10pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsHa_SPEC;
impl crate::sealed::RegSpec for P10PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 10%s Pin Function Select Register"]
pub type P10PfsHa = crate::RegValueT<P10PfsHa_SPEC>;

impl P10PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p10pfs_ha::Podr,
        p10pfs_ha::Podr,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p10pfs_ha::Podr,
            p10pfs_ha::Podr,
            P10PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p10pfs_ha::Pidr,
        p10pfs_ha::Pidr,
        P10PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p10pfs_ha::Pidr,
            p10pfs_ha::Pidr,
            P10PfsHa_SPEC,
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
        p10pfs_ha::Pdr,
        p10pfs_ha::Pdr,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p10pfs_ha::Pdr,
            p10pfs_ha::Pdr,
            P10PfsHa_SPEC,
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
        p10pfs_ha::Pcr,
        p10pfs_ha::Pcr,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p10pfs_ha::Pcr,
            p10pfs_ha::Pcr,
            P10PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p10pfs_ha::Ncodr,
        p10pfs_ha::Ncodr,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p10pfs_ha::Ncodr,
            p10pfs_ha::Ncodr,
            P10PfsHa_SPEC,
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
        p10pfs_ha::Eofr,
        p10pfs_ha::Eofr,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p10pfs_ha::Eofr,
            p10pfs_ha::Eofr,
            P10PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p10pfs_ha::Isel,
        p10pfs_ha::Isel,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p10pfs_ha::Isel,
            p10pfs_ha::Isel,
            P10PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p10pfs_ha::Asel,
        p10pfs_ha::Asel,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p10pfs_ha::Asel,
            p10pfs_ha::Asel,
            P10PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P10PfsHa {
    #[inline(always)]
    fn default() -> P10PfsHa {
        <crate::RegValueT<P10PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p10pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsBy_SPEC;
impl crate::sealed::RegSpec for P10PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 10%s Pin Function Select Register"]
pub type P10PfsBy = crate::RegValueT<P10PfsBy_SPEC>;

impl P10PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p10pfs_by::Podr,
        p10pfs_by::Podr,
        P10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p10pfs_by::Podr,
            p10pfs_by::Podr,
            P10PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p10pfs_by::Pidr,
        p10pfs_by::Pidr,
        P10PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p10pfs_by::Pidr,
            p10pfs_by::Pidr,
            P10PfsBy_SPEC,
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
        p10pfs_by::Pdr,
        p10pfs_by::Pdr,
        P10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p10pfs_by::Pdr,
            p10pfs_by::Pdr,
            P10PfsBy_SPEC,
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
        p10pfs_by::Pcr,
        p10pfs_by::Pcr,
        P10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p10pfs_by::Pcr,
            p10pfs_by::Pcr,
            P10PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p10pfs_by::Ncodr,
        p10pfs_by::Ncodr,
        P10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p10pfs_by::Ncodr,
            p10pfs_by::Ncodr,
            P10PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P10PfsBy {
    #[inline(always)]
    fn default() -> P10PfsBy {
        <crate::RegValueT<P10PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p10pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108Pfs_SPEC;
impl crate::sealed::RegSpec for P108Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 108 Pin Function Select Register"]
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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

    #[doc = "IRQ Input Enable"]
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

    #[doc = "Analog Input Enable"]
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

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P108Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108PfsHa_SPEC;
impl crate::sealed::RegSpec for P108PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 108 Pin Function Select Register"]
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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

    #[doc = "IRQ Input Enable"]
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

    #[doc = "Analog Input Enable"]
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
        <crate::RegValueT<P108PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p108pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108PfsBy_SPEC;
impl crate::sealed::RegSpec for P108PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 108 Pin Function Select Register"]
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109Pfs_SPEC;
impl crate::sealed::RegSpec for P109Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 109 Pin Function Select Register"]
pub type P109Pfs = crate::RegValueT<P109Pfs_SPEC>;

impl P109Pfs {
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p109pfs::Eofr,
        p109pfs::Eofr,
        P109Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p109pfs::Eofr,
            p109pfs::Eofr,
            P109Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
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

    #[doc = "Analog Input Enable"]
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

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P109Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P109Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P109Pfs {
    #[inline(always)]
    fn default() -> P109Pfs {
        <crate::RegValueT<P109Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p109pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109PfsHa_SPEC;
impl crate::sealed::RegSpec for P109PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 109 Pin Function Select Register"]
pub type P109PfsHa = crate::RegValueT<P109PfsHa_SPEC>;

impl P109PfsHa {
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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

    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        p109pfs_ha::Eofr,
        p109pfs_ha::Eofr,
        P109PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p109pfs_ha::Eofr,
            p109pfs_ha::Eofr,
            P109PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
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

    #[doc = "Analog Input Enable"]
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
}
impl ::core::default::Default for P109PfsHa {
    #[inline(always)]
    fn default() -> P109PfsHa {
        <crate::RegValueT<P109PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p109pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109PfsBy_SPEC;
impl crate::sealed::RegSpec for P109PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 109 Pin Function Select Register"]
pub type P109PfsBy = crate::RegValueT<P109PfsBy_SPEC>;

impl P109PfsBy {
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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
}
impl ::core::default::Default for P109PfsBy {
    #[inline(always)]
    fn default() -> P109PfsBy {
        <crate::RegValueT<P109PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p109pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1Pfs_SPEC;
impl crate::sealed::RegSpec for P1Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 1%s Pin Function Select Register"]
pub type P1Pfs = crate::RegValueT<P1Pfs_SPEC>;

impl P1Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p1pfs::Podr,
        p1pfs::Podr,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p1pfs::Podr,
            p1pfs::Podr,
            P1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p1pfs::Pidr,
        p1pfs::Pidr,
        P1Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p1pfs::Pidr,
            p1pfs::Pidr,
            P1Pfs_SPEC,
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
        p1pfs::Pdr,
        p1pfs::Pdr,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p1pfs::Pdr,
            p1pfs::Pdr,
            P1Pfs_SPEC,
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
        p1pfs::Pcr,
        p1pfs::Pcr,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p1pfs::Pcr,
            p1pfs::Pcr,
            P1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p1pfs::Ncodr,
        p1pfs::Ncodr,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p1pfs::Ncodr,
            p1pfs::Ncodr,
            P1Pfs_SPEC,
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
        p1pfs::Eofr,
        p1pfs::Eofr,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p1pfs::Eofr,
            p1pfs::Eofr,
            P1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p1pfs::Isel,
        p1pfs::Isel,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p1pfs::Isel,
            p1pfs::Isel,
            P1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p1pfs::Asel,
        p1pfs::Asel,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p1pfs::Asel,
            p1pfs::Asel,
            P1Pfs_SPEC,
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
        p1pfs::Pmr,
        p1pfs::Pmr,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p1pfs::Pmr,
            p1pfs::Pmr,
            P1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P1Pfs {
    #[inline(always)]
    fn default() -> P1Pfs {
        <crate::RegValueT<P1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p1pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsHa_SPEC;
impl crate::sealed::RegSpec for P1PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 1%s Pin Function Select Register"]
pub type P1PfsHa = crate::RegValueT<P1PfsHa_SPEC>;

impl P1PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p1pfs_ha::Podr,
        p1pfs_ha::Podr,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p1pfs_ha::Podr,
            p1pfs_ha::Podr,
            P1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p1pfs_ha::Pidr,
        p1pfs_ha::Pidr,
        P1PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p1pfs_ha::Pidr,
            p1pfs_ha::Pidr,
            P1PfsHa_SPEC,
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
        p1pfs_ha::Pdr,
        p1pfs_ha::Pdr,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p1pfs_ha::Pdr,
            p1pfs_ha::Pdr,
            P1PfsHa_SPEC,
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
        p1pfs_ha::Pcr,
        p1pfs_ha::Pcr,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p1pfs_ha::Pcr,
            p1pfs_ha::Pcr,
            P1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p1pfs_ha::Ncodr,
        p1pfs_ha::Ncodr,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p1pfs_ha::Ncodr,
            p1pfs_ha::Ncodr,
            P1PfsHa_SPEC,
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
        p1pfs_ha::Eofr,
        p1pfs_ha::Eofr,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p1pfs_ha::Eofr,
            p1pfs_ha::Eofr,
            P1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p1pfs_ha::Isel,
        p1pfs_ha::Isel,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p1pfs_ha::Isel,
            p1pfs_ha::Isel,
            P1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p1pfs_ha::Asel,
        p1pfs_ha::Asel,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p1pfs_ha::Asel,
            p1pfs_ha::Asel,
            P1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1PfsHa {
    #[inline(always)]
    fn default() -> P1PfsHa {
        <crate::RegValueT<P1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p1pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsBy_SPEC;
impl crate::sealed::RegSpec for P1PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 1%s Pin Function Select Register"]
pub type P1PfsBy = crate::RegValueT<P1PfsBy_SPEC>;

impl P1PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p1pfs_by::Podr,
        p1pfs_by::Podr,
        P1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p1pfs_by::Podr,
            p1pfs_by::Podr,
            P1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p1pfs_by::Pidr,
        p1pfs_by::Pidr,
        P1PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p1pfs_by::Pidr,
            p1pfs_by::Pidr,
            P1PfsBy_SPEC,
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
        p1pfs_by::Pdr,
        p1pfs_by::Pdr,
        P1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p1pfs_by::Pdr,
            p1pfs_by::Pdr,
            P1PfsBy_SPEC,
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
        p1pfs_by::Pcr,
        p1pfs_by::Pcr,
        P1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p1pfs_by::Pcr,
            p1pfs_by::Pcr,
            P1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p1pfs_by::Ncodr,
        p1pfs_by::Ncodr,
        P1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p1pfs_by::Ncodr,
            p1pfs_by::Ncodr,
            P1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1PfsBy {
    #[inline(always)]
    fn default() -> P1PfsBy {
        <crate::RegValueT<P1PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p1pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200Pfs_SPEC;
impl crate::sealed::RegSpec for P200Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 200 Pin Function Select Register"]
pub type P200Pfs = crate::RegValueT<P200Pfs_SPEC>;

impl P200Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p200pfs::Podr,
        p200pfs::Podr,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p200pfs::Podr,
            p200pfs::Podr,
            P200Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p200pfs::Pidr,
        p200pfs::Pidr,
        P200Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p200pfs::Pidr,
            p200pfs::Pidr,
            P200Pfs_SPEC,
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
        p200pfs::Pdr,
        p200pfs::Pdr,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p200pfs::Pdr,
            p200pfs::Pdr,
            P200Pfs_SPEC,
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
        p200pfs::Pcr,
        p200pfs::Pcr,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p200pfs::Pcr,
            p200pfs::Pcr,
            P200Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p200pfs::Ncodr,
        p200pfs::Ncodr,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p200pfs::Ncodr,
            p200pfs::Ncodr,
            P200Pfs_SPEC,
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
        p200pfs::Eofr,
        p200pfs::Eofr,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p200pfs::Eofr,
            p200pfs::Eofr,
            P200Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p200pfs::Isel,
        p200pfs::Isel,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p200pfs::Isel,
            p200pfs::Isel,
            P200Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p200pfs::Asel,
        p200pfs::Asel,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p200pfs::Asel,
            p200pfs::Asel,
            P200Pfs_SPEC,
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
        p200pfs::Pmr,
        p200pfs::Pmr,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p200pfs::Pmr,
            p200pfs::Pmr,
            P200Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P200Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P200Pfs {
    #[inline(always)]
    fn default() -> P200Pfs {
        <crate::RegValueT<P200Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p200pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsHa_SPEC;
impl crate::sealed::RegSpec for P200PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 200 Pin Function Select Register"]
pub type P200PfsHa = crate::RegValueT<P200PfsHa_SPEC>;

impl P200PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p200pfs_ha::Podr,
        p200pfs_ha::Podr,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p200pfs_ha::Podr,
            p200pfs_ha::Podr,
            P200PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p200pfs_ha::Pidr,
        p200pfs_ha::Pidr,
        P200PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p200pfs_ha::Pidr,
            p200pfs_ha::Pidr,
            P200PfsHa_SPEC,
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
        p200pfs_ha::Pdr,
        p200pfs_ha::Pdr,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p200pfs_ha::Pdr,
            p200pfs_ha::Pdr,
            P200PfsHa_SPEC,
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
        p200pfs_ha::Pcr,
        p200pfs_ha::Pcr,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p200pfs_ha::Pcr,
            p200pfs_ha::Pcr,
            P200PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p200pfs_ha::Ncodr,
        p200pfs_ha::Ncodr,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p200pfs_ha::Ncodr,
            p200pfs_ha::Ncodr,
            P200PfsHa_SPEC,
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
        p200pfs_ha::Eofr,
        p200pfs_ha::Eofr,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p200pfs_ha::Eofr,
            p200pfs_ha::Eofr,
            P200PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p200pfs_ha::Isel,
        p200pfs_ha::Isel,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p200pfs_ha::Isel,
            p200pfs_ha::Isel,
            P200PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p200pfs_ha::Asel,
        p200pfs_ha::Asel,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p200pfs_ha::Asel,
            p200pfs_ha::Asel,
            P200PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P200PfsHa {
    #[inline(always)]
    fn default() -> P200PfsHa {
        <crate::RegValueT<P200PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p200pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsBy_SPEC;
impl crate::sealed::RegSpec for P200PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 200 Pin Function Select Register"]
pub type P200PfsBy = crate::RegValueT<P200PfsBy_SPEC>;

impl P200PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p200pfs_by::Podr,
        p200pfs_by::Podr,
        P200PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p200pfs_by::Podr,
            p200pfs_by::Podr,
            P200PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p200pfs_by::Pidr,
        p200pfs_by::Pidr,
        P200PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p200pfs_by::Pidr,
            p200pfs_by::Pidr,
            P200PfsBy_SPEC,
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
        p200pfs_by::Pdr,
        p200pfs_by::Pdr,
        P200PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p200pfs_by::Pdr,
            p200pfs_by::Pdr,
            P200PfsBy_SPEC,
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
        p200pfs_by::Pcr,
        p200pfs_by::Pcr,
        P200PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p200pfs_by::Pcr,
            p200pfs_by::Pcr,
            P200PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p200pfs_by::Ncodr,
        p200pfs_by::Ncodr,
        P200PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p200pfs_by::Ncodr,
            p200pfs_by::Ncodr,
            P200PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P200PfsBy {
    #[inline(always)]
    fn default() -> P200PfsBy {
        <crate::RegValueT<P200PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p200pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201Pfs_SPEC;
impl crate::sealed::RegSpec for P201Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 201 Pin Function Select Register"]
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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

    #[doc = "IRQ Input Enable"]
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

    #[doc = "Analog Input Enable"]
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

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P201Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsHa_SPEC;
impl crate::sealed::RegSpec for P201PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 201 Pin Function Select Register"]
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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

    #[doc = "IRQ Input Enable"]
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

    #[doc = "Analog Input Enable"]
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
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsBy_SPEC;
impl crate::sealed::RegSpec for P201PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 201 Pin Function Select Register"]
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

    #[doc = "Port State"]
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

    #[doc = "N-Channel Open-Drain Control"]
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
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20Pfs_SPEC;
impl crate::sealed::RegSpec for P20Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 20%s Pin Function Select Register"]
pub type P20Pfs = crate::RegValueT<P20Pfs_SPEC>;

impl P20Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p20pfs::Podr,
        p20pfs::Podr,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p20pfs::Podr,
            p20pfs::Podr,
            P20Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p20pfs::Pidr,
        p20pfs::Pidr,
        P20Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p20pfs::Pidr,
            p20pfs::Pidr,
            P20Pfs_SPEC,
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
        p20pfs::Pdr,
        p20pfs::Pdr,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p20pfs::Pdr,
            p20pfs::Pdr,
            P20Pfs_SPEC,
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
        p20pfs::Pcr,
        p20pfs::Pcr,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p20pfs::Pcr,
            p20pfs::Pcr,
            P20Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p20pfs::Ncodr,
        p20pfs::Ncodr,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p20pfs::Ncodr,
            p20pfs::Ncodr,
            P20Pfs_SPEC,
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
        p20pfs::Eofr,
        p20pfs::Eofr,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p20pfs::Eofr,
            p20pfs::Eofr,
            P20Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p20pfs::Isel,
        p20pfs::Isel,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p20pfs::Isel,
            p20pfs::Isel,
            P20Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p20pfs::Asel,
        p20pfs::Asel,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p20pfs::Asel,
            p20pfs::Asel,
            P20Pfs_SPEC,
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
        p20pfs::Pmr,
        p20pfs::Pmr,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p20pfs::Pmr,
            p20pfs::Pmr,
            P20Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P20Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P20Pfs {
    #[inline(always)]
    fn default() -> P20Pfs {
        <crate::RegValueT<P20Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p20pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsHa_SPEC;
impl crate::sealed::RegSpec for P20PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 20%s Pin Function Select Register"]
pub type P20PfsHa = crate::RegValueT<P20PfsHa_SPEC>;

impl P20PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p20pfs_ha::Podr,
        p20pfs_ha::Podr,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p20pfs_ha::Podr,
            p20pfs_ha::Podr,
            P20PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p20pfs_ha::Pidr,
        p20pfs_ha::Pidr,
        P20PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p20pfs_ha::Pidr,
            p20pfs_ha::Pidr,
            P20PfsHa_SPEC,
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
        p20pfs_ha::Pdr,
        p20pfs_ha::Pdr,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p20pfs_ha::Pdr,
            p20pfs_ha::Pdr,
            P20PfsHa_SPEC,
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
        p20pfs_ha::Pcr,
        p20pfs_ha::Pcr,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p20pfs_ha::Pcr,
            p20pfs_ha::Pcr,
            P20PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p20pfs_ha::Ncodr,
        p20pfs_ha::Ncodr,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p20pfs_ha::Ncodr,
            p20pfs_ha::Ncodr,
            P20PfsHa_SPEC,
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
        p20pfs_ha::Eofr,
        p20pfs_ha::Eofr,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p20pfs_ha::Eofr,
            p20pfs_ha::Eofr,
            P20PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p20pfs_ha::Isel,
        p20pfs_ha::Isel,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p20pfs_ha::Isel,
            p20pfs_ha::Isel,
            P20PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p20pfs_ha::Asel,
        p20pfs_ha::Asel,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p20pfs_ha::Asel,
            p20pfs_ha::Asel,
            P20PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P20PfsHa {
    #[inline(always)]
    fn default() -> P20PfsHa {
        <crate::RegValueT<P20PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p20pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsBy_SPEC;
impl crate::sealed::RegSpec for P20PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 20%s Pin Function Select Register"]
pub type P20PfsBy = crate::RegValueT<P20PfsBy_SPEC>;

impl P20PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p20pfs_by::Podr,
        p20pfs_by::Podr,
        P20PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p20pfs_by::Podr,
            p20pfs_by::Podr,
            P20PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p20pfs_by::Pidr,
        p20pfs_by::Pidr,
        P20PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p20pfs_by::Pidr,
            p20pfs_by::Pidr,
            P20PfsBy_SPEC,
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
        p20pfs_by::Pdr,
        p20pfs_by::Pdr,
        P20PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p20pfs_by::Pdr,
            p20pfs_by::Pdr,
            P20PfsBy_SPEC,
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
        p20pfs_by::Pcr,
        p20pfs_by::Pcr,
        P20PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p20pfs_by::Pcr,
            p20pfs_by::Pcr,
            P20PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p20pfs_by::Ncodr,
        p20pfs_by::Ncodr,
        P20PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p20pfs_by::Ncodr,
            p20pfs_by::Ncodr,
            P20PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P20PfsBy {
    #[inline(always)]
    fn default() -> P20PfsBy {
        <crate::RegValueT<P20PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p20pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2Pfs_SPEC;
impl crate::sealed::RegSpec for P2Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 2%s Pin Function Select Register"]
pub type P2Pfs = crate::RegValueT<P2Pfs_SPEC>;

impl P2Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p2pfs::Podr,
        p2pfs::Podr,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p2pfs::Podr,
            p2pfs::Podr,
            P2Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p2pfs::Pidr,
        p2pfs::Pidr,
        P2Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p2pfs::Pidr,
            p2pfs::Pidr,
            P2Pfs_SPEC,
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
        p2pfs::Pdr,
        p2pfs::Pdr,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p2pfs::Pdr,
            p2pfs::Pdr,
            P2Pfs_SPEC,
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
        p2pfs::Pcr,
        p2pfs::Pcr,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p2pfs::Pcr,
            p2pfs::Pcr,
            P2Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p2pfs::Ncodr,
        p2pfs::Ncodr,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p2pfs::Ncodr,
            p2pfs::Ncodr,
            P2Pfs_SPEC,
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
        p2pfs::Eofr,
        p2pfs::Eofr,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p2pfs::Eofr,
            p2pfs::Eofr,
            P2Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p2pfs::Isel,
        p2pfs::Isel,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p2pfs::Isel,
            p2pfs::Isel,
            P2Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p2pfs::Asel,
        p2pfs::Asel,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p2pfs::Asel,
            p2pfs::Asel,
            P2Pfs_SPEC,
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
        p2pfs::Pmr,
        p2pfs::Pmr,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p2pfs::Pmr,
            p2pfs::Pmr,
            P2Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P2Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2Pfs {
    #[inline(always)]
    fn default() -> P2Pfs {
        <crate::RegValueT<P2Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p2pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsHa_SPEC;
impl crate::sealed::RegSpec for P2PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 2%s Pin Function Select Register"]
pub type P2PfsHa = crate::RegValueT<P2PfsHa_SPEC>;

impl P2PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p2pfs_ha::Podr,
        p2pfs_ha::Podr,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p2pfs_ha::Podr,
            p2pfs_ha::Podr,
            P2PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p2pfs_ha::Pidr,
        p2pfs_ha::Pidr,
        P2PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p2pfs_ha::Pidr,
            p2pfs_ha::Pidr,
            P2PfsHa_SPEC,
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
        p2pfs_ha::Pdr,
        p2pfs_ha::Pdr,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p2pfs_ha::Pdr,
            p2pfs_ha::Pdr,
            P2PfsHa_SPEC,
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
        p2pfs_ha::Pcr,
        p2pfs_ha::Pcr,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p2pfs_ha::Pcr,
            p2pfs_ha::Pcr,
            P2PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p2pfs_ha::Ncodr,
        p2pfs_ha::Ncodr,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p2pfs_ha::Ncodr,
            p2pfs_ha::Ncodr,
            P2PfsHa_SPEC,
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
        p2pfs_ha::Eofr,
        p2pfs_ha::Eofr,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p2pfs_ha::Eofr,
            p2pfs_ha::Eofr,
            P2PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p2pfs_ha::Isel,
        p2pfs_ha::Isel,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p2pfs_ha::Isel,
            p2pfs_ha::Isel,
            P2PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p2pfs_ha::Asel,
        p2pfs_ha::Asel,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p2pfs_ha::Asel,
            p2pfs_ha::Asel,
            P2PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2PfsHa {
    #[inline(always)]
    fn default() -> P2PfsHa {
        <crate::RegValueT<P2PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p2pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
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
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsBy_SPEC;
impl crate::sealed::RegSpec for P2PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 2%s Pin Function Select Register"]
pub type P2PfsBy = crate::RegValueT<P2PfsBy_SPEC>;

impl P2PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p2pfs_by::Podr,
        p2pfs_by::Podr,
        P2PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p2pfs_by::Podr,
            p2pfs_by::Podr,
            P2PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p2pfs_by::Pidr,
        p2pfs_by::Pidr,
        P2PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p2pfs_by::Pidr,
            p2pfs_by::Pidr,
            P2PfsBy_SPEC,
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
        p2pfs_by::Pdr,
        p2pfs_by::Pdr,
        P2PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p2pfs_by::Pdr,
            p2pfs_by::Pdr,
            P2PfsBy_SPEC,
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
        p2pfs_by::Pcr,
        p2pfs_by::Pcr,
        P2PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p2pfs_by::Pcr,
            p2pfs_by::Pcr,
            P2PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p2pfs_by::Ncodr,
        p2pfs_by::Ncodr,
        P2PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p2pfs_by::Ncodr,
            p2pfs_by::Ncodr,
            P2PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2PfsBy {
    #[inline(always)]
    fn default() -> P2PfsBy {
        <crate::RegValueT<P2PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p2pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300Pfs_SPEC;
impl crate::sealed::RegSpec for P300Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 300 Pin Function Select Register"]
pub type P300Pfs = crate::RegValueT<P300Pfs_SPEC>;

impl P300Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p300pfs::Podr,
        p300pfs::Podr,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p300pfs::Podr,
            p300pfs::Podr,
            P300Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p300pfs::Pidr,
        p300pfs::Pidr,
        P300Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p300pfs::Pidr,
            p300pfs::Pidr,
            P300Pfs_SPEC,
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
        p300pfs::Pdr,
        p300pfs::Pdr,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p300pfs::Pdr,
            p300pfs::Pdr,
            P300Pfs_SPEC,
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
        p300pfs::Pcr,
        p300pfs::Pcr,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p300pfs::Pcr,
            p300pfs::Pcr,
            P300Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p300pfs::Ncodr,
        p300pfs::Ncodr,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p300pfs::Ncodr,
            p300pfs::Ncodr,
            P300Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p300pfs::Isel,
        p300pfs::Isel,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p300pfs::Isel,
            p300pfs::Isel,
            P300Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p300pfs::Asel,
        p300pfs::Asel,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p300pfs::Asel,
            p300pfs::Asel,
            P300Pfs_SPEC,
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
        p300pfs::Pmr,
        p300pfs::Pmr,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p300pfs::Pmr,
            p300pfs::Pmr,
            P300Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P300Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P300Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P300Pfs {
    #[inline(always)]
    fn default() -> P300Pfs {
        <crate::RegValueT<P300Pfs_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod p300pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300PfsHa_SPEC;
impl crate::sealed::RegSpec for P300PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 300 Pin Function Select Register"]
pub type P300PfsHa = crate::RegValueT<P300PfsHa_SPEC>;

impl P300PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p300pfs_ha::Podr,
        p300pfs_ha::Podr,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p300pfs_ha::Podr,
            p300pfs_ha::Podr,
            P300PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p300pfs_ha::Pidr,
        p300pfs_ha::Pidr,
        P300PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p300pfs_ha::Pidr,
            p300pfs_ha::Pidr,
            P300PfsHa_SPEC,
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
        p300pfs_ha::Pdr,
        p300pfs_ha::Pdr,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p300pfs_ha::Pdr,
            p300pfs_ha::Pdr,
            P300PfsHa_SPEC,
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
        p300pfs_ha::Pcr,
        p300pfs_ha::Pcr,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p300pfs_ha::Pcr,
            p300pfs_ha::Pcr,
            P300PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p300pfs_ha::Ncodr,
        p300pfs_ha::Ncodr,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p300pfs_ha::Ncodr,
            p300pfs_ha::Ncodr,
            P300PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p300pfs_ha::Isel,
        p300pfs_ha::Isel,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p300pfs_ha::Isel,
            p300pfs_ha::Isel,
            P300PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p300pfs_ha::Asel,
        p300pfs_ha::Asel,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p300pfs_ha::Asel,
            p300pfs_ha::Asel,
            P300PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P300PfsHa {
    #[inline(always)]
    fn default() -> P300PfsHa {
        <crate::RegValueT<P300PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p300pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300PfsBy_SPEC;
impl crate::sealed::RegSpec for P300PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 300 Pin Function Select Register"]
pub type P300PfsBy = crate::RegValueT<P300PfsBy_SPEC>;

impl P300PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p300pfs_by::Podr,
        p300pfs_by::Podr,
        P300PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p300pfs_by::Podr,
            p300pfs_by::Podr,
            P300PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p300pfs_by::Pidr,
        p300pfs_by::Pidr,
        P300PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p300pfs_by::Pidr,
            p300pfs_by::Pidr,
            P300PfsBy_SPEC,
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
        p300pfs_by::Pdr,
        p300pfs_by::Pdr,
        P300PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p300pfs_by::Pdr,
            p300pfs_by::Pdr,
            P300PfsBy_SPEC,
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
        p300pfs_by::Pcr,
        p300pfs_by::Pcr,
        P300PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p300pfs_by::Pcr,
            p300pfs_by::Pcr,
            P300PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p300pfs_by::Ncodr,
        p300pfs_by::Ncodr,
        P300PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p300pfs_by::Ncodr,
            p300pfs_by::Ncodr,
            P300PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P300PfsBy {
    #[inline(always)]
    fn default() -> P300PfsBy {
        <crate::RegValueT<P300PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p300pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30Pfs_SPEC;
impl crate::sealed::RegSpec for P30Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 30%s Pin Function Select Register"]
pub type P30Pfs = crate::RegValueT<P30Pfs_SPEC>;

impl P30Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p30pfs::Podr,
        p30pfs::Podr,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p30pfs::Podr,
            p30pfs::Podr,
            P30Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p30pfs::Pidr,
        p30pfs::Pidr,
        P30Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p30pfs::Pidr,
            p30pfs::Pidr,
            P30Pfs_SPEC,
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
        p30pfs::Pdr,
        p30pfs::Pdr,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p30pfs::Pdr,
            p30pfs::Pdr,
            P30Pfs_SPEC,
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
        p30pfs::Pcr,
        p30pfs::Pcr,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p30pfs::Pcr,
            p30pfs::Pcr,
            P30Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p30pfs::Ncodr,
        p30pfs::Ncodr,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p30pfs::Ncodr,
            p30pfs::Ncodr,
            P30Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p30pfs::Isel,
        p30pfs::Isel,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p30pfs::Isel,
            p30pfs::Isel,
            P30Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p30pfs::Asel,
        p30pfs::Asel,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p30pfs::Asel,
            p30pfs::Asel,
            P30Pfs_SPEC,
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
        p30pfs::Pmr,
        p30pfs::Pmr,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p30pfs::Pmr,
            p30pfs::Pmr,
            P30Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P30Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P30Pfs {
    #[inline(always)]
    fn default() -> P30Pfs {
        <crate::RegValueT<P30Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p30pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsHa_SPEC;
impl crate::sealed::RegSpec for P30PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 30%s Pin Function Select Register"]
pub type P30PfsHa = crate::RegValueT<P30PfsHa_SPEC>;

impl P30PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p30pfs_ha::Podr,
        p30pfs_ha::Podr,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p30pfs_ha::Podr,
            p30pfs_ha::Podr,
            P30PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p30pfs_ha::Pidr,
        p30pfs_ha::Pidr,
        P30PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p30pfs_ha::Pidr,
            p30pfs_ha::Pidr,
            P30PfsHa_SPEC,
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
        p30pfs_ha::Pdr,
        p30pfs_ha::Pdr,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p30pfs_ha::Pdr,
            p30pfs_ha::Pdr,
            P30PfsHa_SPEC,
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
        p30pfs_ha::Pcr,
        p30pfs_ha::Pcr,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p30pfs_ha::Pcr,
            p30pfs_ha::Pcr,
            P30PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p30pfs_ha::Ncodr,
        p30pfs_ha::Ncodr,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p30pfs_ha::Ncodr,
            p30pfs_ha::Ncodr,
            P30PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p30pfs_ha::Isel,
        p30pfs_ha::Isel,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p30pfs_ha::Isel,
            p30pfs_ha::Isel,
            P30PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p30pfs_ha::Asel,
        p30pfs_ha::Asel,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p30pfs_ha::Asel,
            p30pfs_ha::Asel,
            P30PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P30PfsHa {
    #[inline(always)]
    fn default() -> P30PfsHa {
        <crate::RegValueT<P30PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p30pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsBy_SPEC;
impl crate::sealed::RegSpec for P30PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 30%s Pin Function Select Register"]
pub type P30PfsBy = crate::RegValueT<P30PfsBy_SPEC>;

impl P30PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p30pfs_by::Podr,
        p30pfs_by::Podr,
        P30PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p30pfs_by::Podr,
            p30pfs_by::Podr,
            P30PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p30pfs_by::Pidr,
        p30pfs_by::Pidr,
        P30PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p30pfs_by::Pidr,
            p30pfs_by::Pidr,
            P30PfsBy_SPEC,
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
        p30pfs_by::Pdr,
        p30pfs_by::Pdr,
        P30PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p30pfs_by::Pdr,
            p30pfs_by::Pdr,
            P30PfsBy_SPEC,
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
        p30pfs_by::Pcr,
        p30pfs_by::Pcr,
        P30PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p30pfs_by::Pcr,
            p30pfs_by::Pcr,
            P30PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p30pfs_by::Ncodr,
        p30pfs_by::Ncodr,
        P30PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p30pfs_by::Ncodr,
            p30pfs_by::Ncodr,
            P30PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P30PfsBy {
    #[inline(always)]
    fn default() -> P30PfsBy {
        <crate::RegValueT<P30PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p30pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40Pfs_SPEC;
impl crate::sealed::RegSpec for P40Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 40%s Pin Function Select Register"]
pub type P40Pfs = crate::RegValueT<P40Pfs_SPEC>;

impl P40Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p40pfs::Podr,
        p40pfs::Podr,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p40pfs::Podr,
            p40pfs::Podr,
            P40Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p40pfs::Pidr,
        p40pfs::Pidr,
        P40Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p40pfs::Pidr,
            p40pfs::Pidr,
            P40Pfs_SPEC,
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
        p40pfs::Pdr,
        p40pfs::Pdr,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p40pfs::Pdr,
            p40pfs::Pdr,
            P40Pfs_SPEC,
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
        p40pfs::Pcr,
        p40pfs::Pcr,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p40pfs::Pcr,
            p40pfs::Pcr,
            P40Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p40pfs::Ncodr,
        p40pfs::Ncodr,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p40pfs::Ncodr,
            p40pfs::Ncodr,
            P40Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p40pfs::Isel,
        p40pfs::Isel,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p40pfs::Isel,
            p40pfs::Isel,
            P40Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p40pfs::Asel,
        p40pfs::Asel,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p40pfs::Asel,
            p40pfs::Asel,
            P40Pfs_SPEC,
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
        p40pfs::Pmr,
        p40pfs::Pmr,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p40pfs::Pmr,
            p40pfs::Pmr,
            P40Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P40Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P40Pfs {
    #[inline(always)]
    fn default() -> P40Pfs {
        <crate::RegValueT<P40Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p40pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsHa_SPEC;
impl crate::sealed::RegSpec for P40PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 40%s Pin Function Select Register"]
pub type P40PfsHa = crate::RegValueT<P40PfsHa_SPEC>;

impl P40PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p40pfs_ha::Podr,
        p40pfs_ha::Podr,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p40pfs_ha::Podr,
            p40pfs_ha::Podr,
            P40PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p40pfs_ha::Pidr,
        p40pfs_ha::Pidr,
        P40PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p40pfs_ha::Pidr,
            p40pfs_ha::Pidr,
            P40PfsHa_SPEC,
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
        p40pfs_ha::Pdr,
        p40pfs_ha::Pdr,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p40pfs_ha::Pdr,
            p40pfs_ha::Pdr,
            P40PfsHa_SPEC,
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
        p40pfs_ha::Pcr,
        p40pfs_ha::Pcr,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p40pfs_ha::Pcr,
            p40pfs_ha::Pcr,
            P40PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p40pfs_ha::Ncodr,
        p40pfs_ha::Ncodr,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p40pfs_ha::Ncodr,
            p40pfs_ha::Ncodr,
            P40PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p40pfs_ha::Isel,
        p40pfs_ha::Isel,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p40pfs_ha::Isel,
            p40pfs_ha::Isel,
            P40PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p40pfs_ha::Asel,
        p40pfs_ha::Asel,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p40pfs_ha::Asel,
            p40pfs_ha::Asel,
            P40PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P40PfsHa {
    #[inline(always)]
    fn default() -> P40PfsHa {
        <crate::RegValueT<P40PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p40pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsBy_SPEC;
impl crate::sealed::RegSpec for P40PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 40%s Pin Function Select Register"]
pub type P40PfsBy = crate::RegValueT<P40PfsBy_SPEC>;

impl P40PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p40pfs_by::Podr,
        p40pfs_by::Podr,
        P40PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p40pfs_by::Podr,
            p40pfs_by::Podr,
            P40PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p40pfs_by::Pidr,
        p40pfs_by::Pidr,
        P40PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p40pfs_by::Pidr,
            p40pfs_by::Pidr,
            P40PfsBy_SPEC,
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
        p40pfs_by::Pdr,
        p40pfs_by::Pdr,
        P40PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p40pfs_by::Pdr,
            p40pfs_by::Pdr,
            P40PfsBy_SPEC,
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
        p40pfs_by::Pcr,
        p40pfs_by::Pcr,
        P40PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p40pfs_by::Pcr,
            p40pfs_by::Pcr,
            P40PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p40pfs_by::Ncodr,
        p40pfs_by::Ncodr,
        P40PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p40pfs_by::Ncodr,
            p40pfs_by::Ncodr,
            P40PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P40PfsBy {
    #[inline(always)]
    fn default() -> P40PfsBy {
        <crate::RegValueT<P40PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p40pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4Pfs_SPEC;
impl crate::sealed::RegSpec for P4Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 4%s Pin Function Select Register"]
pub type P4Pfs = crate::RegValueT<P4Pfs_SPEC>;

impl P4Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p4pfs::Podr,
        p4pfs::Podr,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p4pfs::Podr,
            p4pfs::Podr,
            P4Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p4pfs::Pidr,
        p4pfs::Pidr,
        P4Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p4pfs::Pidr,
            p4pfs::Pidr,
            P4Pfs_SPEC,
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
        p4pfs::Pdr,
        p4pfs::Pdr,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p4pfs::Pdr,
            p4pfs::Pdr,
            P4Pfs_SPEC,
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
        p4pfs::Pcr,
        p4pfs::Pcr,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p4pfs::Pcr,
            p4pfs::Pcr,
            P4Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p4pfs::Ncodr,
        p4pfs::Ncodr,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p4pfs::Ncodr,
            p4pfs::Ncodr,
            P4Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p4pfs::Isel,
        p4pfs::Isel,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p4pfs::Isel,
            p4pfs::Isel,
            P4Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p4pfs::Asel,
        p4pfs::Asel,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p4pfs::Asel,
            p4pfs::Asel,
            P4Pfs_SPEC,
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
        p4pfs::Pmr,
        p4pfs::Pmr,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p4pfs::Pmr,
            p4pfs::Pmr,
            P4Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P4Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P4Pfs {
    #[inline(always)]
    fn default() -> P4Pfs {
        <crate::RegValueT<P4Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p4pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsHa_SPEC;
impl crate::sealed::RegSpec for P4PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 4%s Pin Function Select Register"]
pub type P4PfsHa = crate::RegValueT<P4PfsHa_SPEC>;

impl P4PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p4pfs_ha::Podr,
        p4pfs_ha::Podr,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p4pfs_ha::Podr,
            p4pfs_ha::Podr,
            P4PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p4pfs_ha::Pidr,
        p4pfs_ha::Pidr,
        P4PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p4pfs_ha::Pidr,
            p4pfs_ha::Pidr,
            P4PfsHa_SPEC,
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
        p4pfs_ha::Pdr,
        p4pfs_ha::Pdr,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p4pfs_ha::Pdr,
            p4pfs_ha::Pdr,
            P4PfsHa_SPEC,
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
        p4pfs_ha::Pcr,
        p4pfs_ha::Pcr,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p4pfs_ha::Pcr,
            p4pfs_ha::Pcr,
            P4PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p4pfs_ha::Ncodr,
        p4pfs_ha::Ncodr,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p4pfs_ha::Ncodr,
            p4pfs_ha::Ncodr,
            P4PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p4pfs_ha::Isel,
        p4pfs_ha::Isel,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p4pfs_ha::Isel,
            p4pfs_ha::Isel,
            P4PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p4pfs_ha::Asel,
        p4pfs_ha::Asel,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p4pfs_ha::Asel,
            p4pfs_ha::Asel,
            P4PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P4PfsHa {
    #[inline(always)]
    fn default() -> P4PfsHa {
        <crate::RegValueT<P4PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p4pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsBy_SPEC;
impl crate::sealed::RegSpec for P4PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 4%s Pin Function Select Register"]
pub type P4PfsBy = crate::RegValueT<P4PfsBy_SPEC>;

impl P4PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p4pfs_by::Podr,
        p4pfs_by::Podr,
        P4PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p4pfs_by::Podr,
            p4pfs_by::Podr,
            P4PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p4pfs_by::Pidr,
        p4pfs_by::Pidr,
        P4PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p4pfs_by::Pidr,
            p4pfs_by::Pidr,
            P4PfsBy_SPEC,
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
        p4pfs_by::Pdr,
        p4pfs_by::Pdr,
        P4PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p4pfs_by::Pdr,
            p4pfs_by::Pdr,
            P4PfsBy_SPEC,
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
        p4pfs_by::Pcr,
        p4pfs_by::Pcr,
        P4PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p4pfs_by::Pcr,
            p4pfs_by::Pcr,
            P4PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p4pfs_by::Ncodr,
        p4pfs_by::Ncodr,
        P4PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p4pfs_by::Ncodr,
            p4pfs_by::Ncodr,
            P4PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P4PfsBy {
    #[inline(always)]
    fn default() -> P4PfsBy {
        <crate::RegValueT<P4PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p4pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50Pfs_SPEC;
impl crate::sealed::RegSpec for P50Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 50%s Pin Function Select Register"]
pub type P50Pfs = crate::RegValueT<P50Pfs_SPEC>;

impl P50Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p50pfs::Podr,
        p50pfs::Podr,
        P50Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p50pfs::Podr,
            p50pfs::Podr,
            P50Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p50pfs::Pidr,
        p50pfs::Pidr,
        P50Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p50pfs::Pidr,
            p50pfs::Pidr,
            P50Pfs_SPEC,
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
        p50pfs::Pdr,
        p50pfs::Pdr,
        P50Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p50pfs::Pdr,
            p50pfs::Pdr,
            P50Pfs_SPEC,
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
        p50pfs::Pcr,
        p50pfs::Pcr,
        P50Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p50pfs::Pcr,
            p50pfs::Pcr,
            P50Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p50pfs::Ncodr,
        p50pfs::Ncodr,
        P50Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p50pfs::Ncodr,
            p50pfs::Ncodr,
            P50Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p50pfs::Isel,
        p50pfs::Isel,
        P50Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p50pfs::Isel,
            p50pfs::Isel,
            P50Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p50pfs::Asel,
        p50pfs::Asel,
        P50Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p50pfs::Asel,
            p50pfs::Asel,
            P50Pfs_SPEC,
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
        p50pfs::Pmr,
        p50pfs::Pmr,
        P50Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p50pfs::Pmr,
            p50pfs::Pmr,
            P50Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P50Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P50Pfs {
    #[inline(always)]
    fn default() -> P50Pfs {
        <crate::RegValueT<P50Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p50pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsHa_SPEC;
impl crate::sealed::RegSpec for P50PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 50%s Pin Function Select Register"]
pub type P50PfsHa = crate::RegValueT<P50PfsHa_SPEC>;

impl P50PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p50pfs_ha::Podr,
        p50pfs_ha::Podr,
        P50PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p50pfs_ha::Podr,
            p50pfs_ha::Podr,
            P50PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p50pfs_ha::Pidr,
        p50pfs_ha::Pidr,
        P50PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p50pfs_ha::Pidr,
            p50pfs_ha::Pidr,
            P50PfsHa_SPEC,
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
        p50pfs_ha::Pdr,
        p50pfs_ha::Pdr,
        P50PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p50pfs_ha::Pdr,
            p50pfs_ha::Pdr,
            P50PfsHa_SPEC,
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
        p50pfs_ha::Pcr,
        p50pfs_ha::Pcr,
        P50PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p50pfs_ha::Pcr,
            p50pfs_ha::Pcr,
            P50PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p50pfs_ha::Ncodr,
        p50pfs_ha::Ncodr,
        P50PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p50pfs_ha::Ncodr,
            p50pfs_ha::Ncodr,
            P50PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p50pfs_ha::Isel,
        p50pfs_ha::Isel,
        P50PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p50pfs_ha::Isel,
            p50pfs_ha::Isel,
            P50PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p50pfs_ha::Asel,
        p50pfs_ha::Asel,
        P50PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p50pfs_ha::Asel,
            p50pfs_ha::Asel,
            P50PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P50PfsHa {
    #[inline(always)]
    fn default() -> P50PfsHa {
        <crate::RegValueT<P50PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p50pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsBy_SPEC;
impl crate::sealed::RegSpec for P50PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 50%s Pin Function Select Register"]
pub type P50PfsBy = crate::RegValueT<P50PfsBy_SPEC>;

impl P50PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p50pfs_by::Podr,
        p50pfs_by::Podr,
        P50PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p50pfs_by::Podr,
            p50pfs_by::Podr,
            P50PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p50pfs_by::Pidr,
        p50pfs_by::Pidr,
        P50PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p50pfs_by::Pidr,
            p50pfs_by::Pidr,
            P50PfsBy_SPEC,
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
        p50pfs_by::Pdr,
        p50pfs_by::Pdr,
        P50PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p50pfs_by::Pdr,
            p50pfs_by::Pdr,
            P50PfsBy_SPEC,
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
        p50pfs_by::Pcr,
        p50pfs_by::Pcr,
        P50PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p50pfs_by::Pcr,
            p50pfs_by::Pcr,
            P50PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p50pfs_by::Ncodr,
        p50pfs_by::Ncodr,
        P50PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p50pfs_by::Ncodr,
            p50pfs_by::Ncodr,
            P50PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P50PfsBy {
    #[inline(always)]
    fn default() -> P50PfsBy {
        <crate::RegValueT<P50PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p50pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60Pfs_SPEC;
impl crate::sealed::RegSpec for P60Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 60%s Pin Function Select Register"]
pub type P60Pfs = crate::RegValueT<P60Pfs_SPEC>;

impl P60Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p60pfs::Podr,
        p60pfs::Podr,
        P60Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p60pfs::Podr,
            p60pfs::Podr,
            P60Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p60pfs::Pidr,
        p60pfs::Pidr,
        P60Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p60pfs::Pidr,
            p60pfs::Pidr,
            P60Pfs_SPEC,
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
        p60pfs::Pdr,
        p60pfs::Pdr,
        P60Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p60pfs::Pdr,
            p60pfs::Pdr,
            P60Pfs_SPEC,
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
        p60pfs::Pcr,
        p60pfs::Pcr,
        P60Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p60pfs::Pcr,
            p60pfs::Pcr,
            P60Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p60pfs::Ncodr,
        p60pfs::Ncodr,
        P60Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p60pfs::Ncodr,
            p60pfs::Ncodr,
            P60Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p60pfs::Isel,
        p60pfs::Isel,
        P60Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p60pfs::Isel,
            p60pfs::Isel,
            P60Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p60pfs::Asel,
        p60pfs::Asel,
        P60Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p60pfs::Asel,
            p60pfs::Asel,
            P60Pfs_SPEC,
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
        p60pfs::Pmr,
        p60pfs::Pmr,
        P60Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p60pfs::Pmr,
            p60pfs::Pmr,
            P60Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P60Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P60Pfs {
    #[inline(always)]
    fn default() -> P60Pfs {
        <crate::RegValueT<P60Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p60pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsHa_SPEC;
impl crate::sealed::RegSpec for P60PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 60%s Pin Function Select Register"]
pub type P60PfsHa = crate::RegValueT<P60PfsHa_SPEC>;

impl P60PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p60pfs_ha::Podr,
        p60pfs_ha::Podr,
        P60PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p60pfs_ha::Podr,
            p60pfs_ha::Podr,
            P60PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p60pfs_ha::Pidr,
        p60pfs_ha::Pidr,
        P60PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p60pfs_ha::Pidr,
            p60pfs_ha::Pidr,
            P60PfsHa_SPEC,
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
        p60pfs_ha::Pdr,
        p60pfs_ha::Pdr,
        P60PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p60pfs_ha::Pdr,
            p60pfs_ha::Pdr,
            P60PfsHa_SPEC,
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
        p60pfs_ha::Pcr,
        p60pfs_ha::Pcr,
        P60PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p60pfs_ha::Pcr,
            p60pfs_ha::Pcr,
            P60PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p60pfs_ha::Ncodr,
        p60pfs_ha::Ncodr,
        P60PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p60pfs_ha::Ncodr,
            p60pfs_ha::Ncodr,
            P60PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p60pfs_ha::Isel,
        p60pfs_ha::Isel,
        P60PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p60pfs_ha::Isel,
            p60pfs_ha::Isel,
            P60PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p60pfs_ha::Asel,
        p60pfs_ha::Asel,
        P60PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p60pfs_ha::Asel,
            p60pfs_ha::Asel,
            P60PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P60PfsHa {
    #[inline(always)]
    fn default() -> P60PfsHa {
        <crate::RegValueT<P60PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p60pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsBy_SPEC;
impl crate::sealed::RegSpec for P60PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 60%s Pin Function Select Register"]
pub type P60PfsBy = crate::RegValueT<P60PfsBy_SPEC>;

impl P60PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p60pfs_by::Podr,
        p60pfs_by::Podr,
        P60PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p60pfs_by::Podr,
            p60pfs_by::Podr,
            P60PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p60pfs_by::Pidr,
        p60pfs_by::Pidr,
        P60PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p60pfs_by::Pidr,
            p60pfs_by::Pidr,
            P60PfsBy_SPEC,
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
        p60pfs_by::Pdr,
        p60pfs_by::Pdr,
        P60PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p60pfs_by::Pdr,
            p60pfs_by::Pdr,
            P60PfsBy_SPEC,
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
        p60pfs_by::Pcr,
        p60pfs_by::Pcr,
        P60PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p60pfs_by::Pcr,
            p60pfs_by::Pcr,
            P60PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p60pfs_by::Ncodr,
        p60pfs_by::Ncodr,
        P60PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p60pfs_by::Ncodr,
            p60pfs_by::Ncodr,
            P60PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P60PfsBy {
    #[inline(always)]
    fn default() -> P60PfsBy {
        <crate::RegValueT<P60PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p60pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P610Pfs_SPEC;
impl crate::sealed::RegSpec for P610Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 610 Pin Function Select Register"]
pub type P610Pfs = crate::RegValueT<P610Pfs_SPEC>;

impl P610Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p610pfs::Podr,
        p610pfs::Podr,
        P610Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p610pfs::Podr,
            p610pfs::Podr,
            P610Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p610pfs::Pidr,
        p610pfs::Pidr,
        P610Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p610pfs::Pidr,
            p610pfs::Pidr,
            P610Pfs_SPEC,
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
        p610pfs::Pdr,
        p610pfs::Pdr,
        P610Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p610pfs::Pdr,
            p610pfs::Pdr,
            P610Pfs_SPEC,
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
        p610pfs::Pcr,
        p610pfs::Pcr,
        P610Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p610pfs::Pcr,
            p610pfs::Pcr,
            P610Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p610pfs::Ncodr,
        p610pfs::Ncodr,
        P610Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p610pfs::Ncodr,
            p610pfs::Ncodr,
            P610Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p610pfs::Isel,
        p610pfs::Isel,
        P610Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p610pfs::Isel,
            p610pfs::Isel,
            P610Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p610pfs::Asel,
        p610pfs::Asel,
        P610Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p610pfs::Asel,
            p610pfs::Asel,
            P610Pfs_SPEC,
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
        p610pfs::Pmr,
        p610pfs::Pmr,
        P610Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p610pfs::Pmr,
            p610pfs::Pmr,
            P610Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P610Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P610Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P610Pfs {
    #[inline(always)]
    fn default() -> P610Pfs {
        <crate::RegValueT<P610Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p610pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P610PfsHa_SPEC;
impl crate::sealed::RegSpec for P610PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 610 Pin Function Select Register"]
pub type P610PfsHa = crate::RegValueT<P610PfsHa_SPEC>;

impl P610PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p610pfs_ha::Podr,
        p610pfs_ha::Podr,
        P610PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p610pfs_ha::Podr,
            p610pfs_ha::Podr,
            P610PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p610pfs_ha::Pidr,
        p610pfs_ha::Pidr,
        P610PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p610pfs_ha::Pidr,
            p610pfs_ha::Pidr,
            P610PfsHa_SPEC,
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
        p610pfs_ha::Pdr,
        p610pfs_ha::Pdr,
        P610PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p610pfs_ha::Pdr,
            p610pfs_ha::Pdr,
            P610PfsHa_SPEC,
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
        p610pfs_ha::Pcr,
        p610pfs_ha::Pcr,
        P610PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p610pfs_ha::Pcr,
            p610pfs_ha::Pcr,
            P610PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p610pfs_ha::Ncodr,
        p610pfs_ha::Ncodr,
        P610PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p610pfs_ha::Ncodr,
            p610pfs_ha::Ncodr,
            P610PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p610pfs_ha::Isel,
        p610pfs_ha::Isel,
        P610PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p610pfs_ha::Isel,
            p610pfs_ha::Isel,
            P610PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p610pfs_ha::Asel,
        p610pfs_ha::Asel,
        P610PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p610pfs_ha::Asel,
            p610pfs_ha::Asel,
            P610PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P610PfsHa {
    #[inline(always)]
    fn default() -> P610PfsHa {
        <crate::RegValueT<P610PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p610pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P610PfsBy_SPEC;
impl crate::sealed::RegSpec for P610PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 610 Pin Function Select Register"]
pub type P610PfsBy = crate::RegValueT<P610PfsBy_SPEC>;

impl P610PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p610pfs_by::Podr,
        p610pfs_by::Podr,
        P610PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p610pfs_by::Podr,
            p610pfs_by::Podr,
            P610PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p610pfs_by::Pidr,
        p610pfs_by::Pidr,
        P610PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p610pfs_by::Pidr,
            p610pfs_by::Pidr,
            P610PfsBy_SPEC,
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
        p610pfs_by::Pdr,
        p610pfs_by::Pdr,
        P610PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p610pfs_by::Pdr,
            p610pfs_by::Pdr,
            P610PfsBy_SPEC,
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
        p610pfs_by::Pcr,
        p610pfs_by::Pcr,
        P610PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p610pfs_by::Pcr,
            p610pfs_by::Pcr,
            P610PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p610pfs_by::Ncodr,
        p610pfs_by::Ncodr,
        P610PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p610pfs_by::Ncodr,
            p610pfs_by::Ncodr,
            P610PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P610PfsBy {
    #[inline(always)]
    fn default() -> P610PfsBy {
        <crate::RegValueT<P610PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p610pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P708Pfs_SPEC;
impl crate::sealed::RegSpec for P708Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 708 Pin Function Select Register"]
pub type P708Pfs = crate::RegValueT<P708Pfs_SPEC>;

impl P708Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p708pfs::Podr,
        p708pfs::Podr,
        P708Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p708pfs::Podr,
            p708pfs::Podr,
            P708Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p708pfs::Pidr,
        p708pfs::Pidr,
        P708Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p708pfs::Pidr,
            p708pfs::Pidr,
            P708Pfs_SPEC,
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
        p708pfs::Pdr,
        p708pfs::Pdr,
        P708Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p708pfs::Pdr,
            p708pfs::Pdr,
            P708Pfs_SPEC,
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
        p708pfs::Pcr,
        p708pfs::Pcr,
        P708Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p708pfs::Pcr,
            p708pfs::Pcr,
            P708Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p708pfs::Ncodr,
        p708pfs::Ncodr,
        P708Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p708pfs::Ncodr,
            p708pfs::Ncodr,
            P708Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p708pfs::Isel,
        p708pfs::Isel,
        P708Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p708pfs::Isel,
            p708pfs::Isel,
            P708Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p708pfs::Asel,
        p708pfs::Asel,
        P708Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p708pfs::Asel,
            p708pfs::Asel,
            P708Pfs_SPEC,
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
        p708pfs::Pmr,
        p708pfs::Pmr,
        P708Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p708pfs::Pmr,
            p708pfs::Pmr,
            P708Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P708Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P708Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P708Pfs {
    #[inline(always)]
    fn default() -> P708Pfs {
        <crate::RegValueT<P708Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p708pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P708PfsHa_SPEC;
impl crate::sealed::RegSpec for P708PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 708 Pin Function Select Register"]
pub type P708PfsHa = crate::RegValueT<P708PfsHa_SPEC>;

impl P708PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p708pfs_ha::Podr,
        p708pfs_ha::Podr,
        P708PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p708pfs_ha::Podr,
            p708pfs_ha::Podr,
            P708PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p708pfs_ha::Pidr,
        p708pfs_ha::Pidr,
        P708PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p708pfs_ha::Pidr,
            p708pfs_ha::Pidr,
            P708PfsHa_SPEC,
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
        p708pfs_ha::Pdr,
        p708pfs_ha::Pdr,
        P708PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p708pfs_ha::Pdr,
            p708pfs_ha::Pdr,
            P708PfsHa_SPEC,
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
        p708pfs_ha::Pcr,
        p708pfs_ha::Pcr,
        P708PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p708pfs_ha::Pcr,
            p708pfs_ha::Pcr,
            P708PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p708pfs_ha::Ncodr,
        p708pfs_ha::Ncodr,
        P708PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p708pfs_ha::Ncodr,
            p708pfs_ha::Ncodr,
            P708PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p708pfs_ha::Isel,
        p708pfs_ha::Isel,
        P708PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p708pfs_ha::Isel,
            p708pfs_ha::Isel,
            P708PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p708pfs_ha::Asel,
        p708pfs_ha::Asel,
        P708PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p708pfs_ha::Asel,
            p708pfs_ha::Asel,
            P708PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P708PfsHa {
    #[inline(always)]
    fn default() -> P708PfsHa {
        <crate::RegValueT<P708PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p708pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P708PfsBy_SPEC;
impl crate::sealed::RegSpec for P708PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 708 Pin Function Select Register"]
pub type P708PfsBy = crate::RegValueT<P708PfsBy_SPEC>;

impl P708PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p708pfs_by::Podr,
        p708pfs_by::Podr,
        P708PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p708pfs_by::Podr,
            p708pfs_by::Podr,
            P708PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p708pfs_by::Pidr,
        p708pfs_by::Pidr,
        P708PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p708pfs_by::Pidr,
            p708pfs_by::Pidr,
            P708PfsBy_SPEC,
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
        p708pfs_by::Pdr,
        p708pfs_by::Pdr,
        P708PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p708pfs_by::Pdr,
            p708pfs_by::Pdr,
            P708PfsBy_SPEC,
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
        p708pfs_by::Pcr,
        p708pfs_by::Pcr,
        P708PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p708pfs_by::Pcr,
            p708pfs_by::Pcr,
            P708PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p708pfs_by::Ncodr,
        p708pfs_by::Ncodr,
        P708PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p708pfs_by::Ncodr,
            p708pfs_by::Ncodr,
            P708PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P708PfsBy {
    #[inline(always)]
    fn default() -> P708PfsBy {
        <crate::RegValueT<P708PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p708pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P714Pfs_SPEC;
impl crate::sealed::RegSpec for P714Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 714 Pin Function Select Register"]
pub type P714Pfs = crate::RegValueT<P714Pfs_SPEC>;

impl P714Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p714pfs::Podr,
        p714pfs::Podr,
        P714Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p714pfs::Podr,
            p714pfs::Podr,
            P714Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p714pfs::Pidr,
        p714pfs::Pidr,
        P714Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p714pfs::Pidr,
            p714pfs::Pidr,
            P714Pfs_SPEC,
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
        p714pfs::Pdr,
        p714pfs::Pdr,
        P714Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p714pfs::Pdr,
            p714pfs::Pdr,
            P714Pfs_SPEC,
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
        p714pfs::Pcr,
        p714pfs::Pcr,
        P714Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p714pfs::Pcr,
            p714pfs::Pcr,
            P714Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p714pfs::Ncodr,
        p714pfs::Ncodr,
        P714Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p714pfs::Ncodr,
            p714pfs::Ncodr,
            P714Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p714pfs::Isel,
        p714pfs::Isel,
        P714Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p714pfs::Isel,
            p714pfs::Isel,
            P714Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p714pfs::Asel,
        p714pfs::Asel,
        P714Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p714pfs::Asel,
            p714pfs::Asel,
            P714Pfs_SPEC,
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
        p714pfs::Pmr,
        p714pfs::Pmr,
        P714Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p714pfs::Pmr,
            p714pfs::Pmr,
            P714Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P714Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P714Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P714Pfs {
    #[inline(always)]
    fn default() -> P714Pfs {
        <crate::RegValueT<P714Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p714pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P714PfsHa_SPEC;
impl crate::sealed::RegSpec for P714PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 714 Pin Function Select Register"]
pub type P714PfsHa = crate::RegValueT<P714PfsHa_SPEC>;

impl P714PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p714pfs_ha::Podr,
        p714pfs_ha::Podr,
        P714PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p714pfs_ha::Podr,
            p714pfs_ha::Podr,
            P714PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p714pfs_ha::Pidr,
        p714pfs_ha::Pidr,
        P714PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p714pfs_ha::Pidr,
            p714pfs_ha::Pidr,
            P714PfsHa_SPEC,
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
        p714pfs_ha::Pdr,
        p714pfs_ha::Pdr,
        P714PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p714pfs_ha::Pdr,
            p714pfs_ha::Pdr,
            P714PfsHa_SPEC,
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
        p714pfs_ha::Pcr,
        p714pfs_ha::Pcr,
        P714PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p714pfs_ha::Pcr,
            p714pfs_ha::Pcr,
            P714PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p714pfs_ha::Ncodr,
        p714pfs_ha::Ncodr,
        P714PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p714pfs_ha::Ncodr,
            p714pfs_ha::Ncodr,
            P714PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p714pfs_ha::Isel,
        p714pfs_ha::Isel,
        P714PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p714pfs_ha::Isel,
            p714pfs_ha::Isel,
            P714PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p714pfs_ha::Asel,
        p714pfs_ha::Asel,
        P714PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p714pfs_ha::Asel,
            p714pfs_ha::Asel,
            P714PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P714PfsHa {
    #[inline(always)]
    fn default() -> P714PfsHa {
        <crate::RegValueT<P714PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p714pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P714PfsBy_SPEC;
impl crate::sealed::RegSpec for P714PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 714 Pin Function Select Register"]
pub type P714PfsBy = crate::RegValueT<P714PfsBy_SPEC>;

impl P714PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p714pfs_by::Podr,
        p714pfs_by::Podr,
        P714PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p714pfs_by::Podr,
            p714pfs_by::Podr,
            P714PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p714pfs_by::Pidr,
        p714pfs_by::Pidr,
        P714PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p714pfs_by::Pidr,
            p714pfs_by::Pidr,
            P714PfsBy_SPEC,
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
        p714pfs_by::Pdr,
        p714pfs_by::Pdr,
        P714PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p714pfs_by::Pdr,
            p714pfs_by::Pdr,
            P714PfsBy_SPEC,
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
        p714pfs_by::Pcr,
        p714pfs_by::Pcr,
        P714PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p714pfs_by::Pcr,
            p714pfs_by::Pcr,
            P714PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p714pfs_by::Ncodr,
        p714pfs_by::Ncodr,
        P714PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p714pfs_by::Ncodr,
            p714pfs_by::Ncodr,
            P714PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P714PfsBy {
    #[inline(always)]
    fn default() -> P714PfsBy {
        <crate::RegValueT<P714PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p714pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80Pfs_SPEC;
impl crate::sealed::RegSpec for P80Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 80%s Pin Function Select Register"]
pub type P80Pfs = crate::RegValueT<P80Pfs_SPEC>;

impl P80Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p80pfs::Podr,
        p80pfs::Podr,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p80pfs::Podr,
            p80pfs::Podr,
            P80Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p80pfs::Pidr,
        p80pfs::Pidr,
        P80Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p80pfs::Pidr,
            p80pfs::Pidr,
            P80Pfs_SPEC,
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
        p80pfs::Pdr,
        p80pfs::Pdr,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p80pfs::Pdr,
            p80pfs::Pdr,
            P80Pfs_SPEC,
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
        p80pfs::Pcr,
        p80pfs::Pcr,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p80pfs::Pcr,
            p80pfs::Pcr,
            P80Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p80pfs::Ncodr,
        p80pfs::Ncodr,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p80pfs::Ncodr,
            p80pfs::Ncodr,
            P80Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p80pfs::Isel,
        p80pfs::Isel,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p80pfs::Isel,
            p80pfs::Isel,
            P80Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p80pfs::Asel,
        p80pfs::Asel,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p80pfs::Asel,
            p80pfs::Asel,
            P80Pfs_SPEC,
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
        p80pfs::Pmr,
        p80pfs::Pmr,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p80pfs::Pmr,
            p80pfs::Pmr,
            P80Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P80Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P80Pfs {
    #[inline(always)]
    fn default() -> P80Pfs {
        <crate::RegValueT<P80Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p80pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Use as general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsHa_SPEC;
impl crate::sealed::RegSpec for P80PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 80%s Pin Function Select Register"]
pub type P80PfsHa = crate::RegValueT<P80PfsHa_SPEC>;

impl P80PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p80pfs_ha::Podr,
        p80pfs_ha::Podr,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p80pfs_ha::Podr,
            p80pfs_ha::Podr,
            P80PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p80pfs_ha::Pidr,
        p80pfs_ha::Pidr,
        P80PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p80pfs_ha::Pidr,
            p80pfs_ha::Pidr,
            P80PfsHa_SPEC,
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
        p80pfs_ha::Pdr,
        p80pfs_ha::Pdr,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p80pfs_ha::Pdr,
            p80pfs_ha::Pdr,
            P80PfsHa_SPEC,
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
        p80pfs_ha::Pcr,
        p80pfs_ha::Pcr,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p80pfs_ha::Pcr,
            p80pfs_ha::Pcr,
            P80PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p80pfs_ha::Ncodr,
        p80pfs_ha::Ncodr,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p80pfs_ha::Ncodr,
            p80pfs_ha::Ncodr,
            P80PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p80pfs_ha::Isel,
        p80pfs_ha::Isel,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p80pfs_ha::Isel,
            p80pfs_ha::Isel,
            P80PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p80pfs_ha::Asel,
        p80pfs_ha::Asel,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p80pfs_ha::Asel,
            p80pfs_ha::Asel,
            P80PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P80PfsHa {
    #[inline(always)]
    fn default() -> P80PfsHa {
        <crate::RegValueT<P80PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p80pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Do not use as IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Do not use as analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use as analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsBy_SPEC;
impl crate::sealed::RegSpec for P80PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 80%s Pin Function Select Register"]
pub type P80PfsBy = crate::RegValueT<P80PfsBy_SPEC>;

impl P80PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p80pfs_by::Podr,
        p80pfs_by::Podr,
        P80PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p80pfs_by::Podr,
            p80pfs_by::Podr,
            P80PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p80pfs_by::Pidr,
        p80pfs_by::Pidr,
        P80PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p80pfs_by::Pidr,
            p80pfs_by::Pidr,
            P80PfsBy_SPEC,
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
        p80pfs_by::Pdr,
        p80pfs_by::Pdr,
        P80PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p80pfs_by::Pdr,
            p80pfs_by::Pdr,
            P80PfsBy_SPEC,
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
        p80pfs_by::Pcr,
        p80pfs_by::Pcr,
        P80PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p80pfs_by::Pcr,
            p80pfs_by::Pcr,
            P80PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p80pfs_by::Ncodr,
        p80pfs_by::Ncodr,
        P80PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p80pfs_by::Ncodr,
            p80pfs_by::Ncodr,
            P80PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P80PfsBy {
    #[inline(always)]
    fn default() -> P80PfsBy {
        <crate::RegValueT<P80PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p80pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Output low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "Output CMOS"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output NMOS open-drain"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwpr_SPEC;
impl crate::sealed::RegSpec for Pwpr_SPEC {
    type DataType = u8;
}

#[doc = "Write-Protect Register"]
pub type Pwpr = crate::RegValueT<Pwpr_SPEC>;

impl Pwpr {
    #[doc = "PmnPFS Register Write Enable"]
    #[inline(always)]
    pub fn pfswe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pwpr::Pfswe,
        pwpr::Pfswe,
        Pwpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pwpr::Pfswe,
            pwpr::Pfswe,
            Pwpr_SPEC,
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
        pwpr::B0Wi,
        pwpr::B0Wi,
        Pwpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pwpr::B0Wi,
            pwpr::B0Wi,
            Pwpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwpr {
    #[inline(always)]
    fn default() -> Pwpr {
        <crate::RegValueT<Pwpr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod pwpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfswe_SPEC;
    pub type Pfswe = crate::EnumBitfieldStruct<u8, Pfswe_SPEC>;
    impl Pfswe {
        #[doc = "Writing to the PmnPFS register is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the PmnPFS register is enabled"]
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
pub struct Prwcntr_SPEC;
impl crate::sealed::RegSpec for Prwcntr_SPEC {
    type DataType = u8;
}

#[doc = "Port Read Wait Control Register"]
pub type Prwcntr = crate::RegValueT<Prwcntr_SPEC>;

impl Prwcntr {
    #[doc = "Wait Cycle Control"]
    #[inline(always)]
    pub fn wait(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        prwcntr::Wait,
        prwcntr::Wait,
        Prwcntr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            prwcntr::Wait,
            prwcntr::Wait,
            Prwcntr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Prwcntr {
    #[inline(always)]
    fn default() -> Prwcntr {
        <crate::RegValueT<Prwcntr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod prwcntr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wait_SPEC;
    pub type Wait = crate::EnumBitfieldStruct<u8, Wait_SPEC>;
    impl Wait {
        #[doc = "Setting prohibited"]
        pub const _00: Self = Self::new(0);

        #[doc = "Insert a 1-cycle wait"]
        pub const _01: Self = Self::new(1);

        #[doc = "Insert a 2-cycle wait"]
        pub const _10: Self = Self::new(2);

        #[doc = "Insert a 3-cycle wait"]
        pub const _11: Self = Self::new(3);
    }
}
