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
// Generated from SVD 1.1, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:31 +0000

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
                self._svd2pac_as_ptr().add(2usize),
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
        3,
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

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        3,
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

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        3,
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

    #[doc = "P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30usize))
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
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x32usize))
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
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x33usize))
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
                self._svd2pac_as_ptr().add(66usize),
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
                self._svd2pac_as_ptr().add(67usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x46usize))
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
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x47usize))
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

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW>,
        3,
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

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW>,
        3,
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

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW>,
        3,
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x90usize))
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x92usize))
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x93usize))
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
        4,
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

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW>,
        4,
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

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW>,
        4,
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

    #[doc = "P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P407Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P407Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[doc = "P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P407PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P407PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(286usize),
            )
        }
    }

    #[doc = "P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P407PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P407PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(287usize),
            )
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x120usize))
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
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x122usize))
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

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x123usize))
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

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW>,
        2,
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

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW>,
        2,
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

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW>,
        2,
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

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW>,
        3,
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

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW>,
        3,
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

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW>,
        3,
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

    #[doc = "P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P914Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P914Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(632usize),
            )
        }
    }

    #[doc = "P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P914PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P914PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(634usize),
            )
        }
    }

    #[doc = "P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P914PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P914PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(635usize),
            )
        }
    }

    #[doc = "P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P915Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P915Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(636usize),
            )
        }
    }

    #[doc = "P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P915PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P915PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(638usize),
            )
        }
    }

    #[doc = "P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P915PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P915PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(639usize),
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

    #[doc = "Drive Strength Control Register"]
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
pub struct P000PfsHa_SPEC;
impl crate::sealed::RegSpec for P000PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P000 Pin Function Control Register"]
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

    #[doc = "Drive Strength Control Register"]
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
pub struct P000PfsBy_SPEC;
impl crate::sealed::RegSpec for P000PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P000 Pin Function Control Register"]
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
pub struct P100Pfs_SPEC;
impl crate::sealed::RegSpec for P100Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P100 Pin Function Control Register"]
pub type P100Pfs = crate::RegValueT<P100Pfs_SPEC>;

impl P100Pfs {
    #[doc = "Port Function Select\nThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P100Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p100pfs::Dscr,
        p100pfs::Dscr,
        P100Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p100pfs::Dscr,
            p100pfs::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P100Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, P100Pfs_SPEC, crate::common::RW>::from_register(
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
}
impl ::core::default::Default for P100Pfs {
    #[inline(always)]
    fn default() -> P100Pfs {
        <crate::RegValueT<P100Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs {

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
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edge."]
        pub const _11: Self = Self::new(3);
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
pub struct P100PfsHa_SPEC;
impl crate::sealed::RegSpec for P100PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P100 Pin Function Control Register"]
pub type P100PfsHa = crate::RegValueT<P100PfsHa_SPEC>;

impl P100PfsHa {
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

    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        p100pfs_ha::Dscr,
        p100pfs_ha::Dscr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            p100pfs_ha::Dscr,
            p100pfs_ha::Dscr,
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P100PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P100PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P100PfsHa {
    #[inline(always)]
    fn default() -> P100PfsHa {
        <crate::RegValueT<P100PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs_ha {

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
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);

        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);

        #[doc = "Detect both edge."]
        pub const _11: Self = Self::new(3);
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
pub struct P100PfsBy_SPEC;
impl crate::sealed::RegSpec for P100PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P100 Pin Function Control Register"]
pub type P100PfsBy = crate::RegValueT<P100PfsBy_SPEC>;

impl P100PfsBy {
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P100PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P100PfsBy_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for P100PfsBy {
    #[inline(always)]
    fn default() -> P100PfsBy {
        <crate::RegValueT<P100PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs_by {

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

impl NoBitfieldReg<P108Pfs_SPEC> for P108Pfs {}
impl ::core::default::Default for P108Pfs {
    #[inline(always)]
    fn default() -> P108Pfs {
        <crate::RegValueT<P108Pfs_SPEC> as RegisterValue<_>>::new(65552)
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

impl NoBitfieldReg<P108PfsHa_SPEC> for P108PfsHa {}
impl ::core::default::Default for P108PfsHa {
    #[inline(always)]
    fn default() -> P108PfsHa {
        <crate::RegValueT<P108PfsHa_SPEC> as RegisterValue<_>>::new(16)
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

impl NoBitfieldReg<P108PfsBy_SPEC> for P108PfsBy {}
impl ::core::default::Default for P108PfsBy {
    #[inline(always)]
    fn default() -> P108PfsBy {
        <crate::RegValueT<P108PfsBy_SPEC> as RegisterValue<_>>::new(16)
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
        <crate::RegValueT<P109Pfs_SPEC> as RegisterValue<_>>::new(0)
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
        <crate::RegValueT<P109PfsHa_SPEC> as RegisterValue<_>>::new(0)
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
        <crate::RegValueT<P109PfsBy_SPEC> as RegisterValue<_>>::new(0)
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

impl NoBitfieldReg<P201Pfs_SPEC> for P201Pfs {}
impl ::core::default::Default for P201Pfs {
    #[inline(always)]
    fn default() -> P201Pfs {
        <crate::RegValueT<P201Pfs_SPEC> as RegisterValue<_>>::new(16)
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

impl NoBitfieldReg<P201PfsHa_SPEC> for P201PfsHa {}
impl ::core::default::Default for P201PfsHa {
    #[inline(always)]
    fn default() -> P201PfsHa {
        <crate::RegValueT<P201PfsHa_SPEC> as RegisterValue<_>>::new(16)
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

impl NoBitfieldReg<P201PfsBy_SPEC> for P201PfsBy {}
impl ::core::default::Default for P201PfsBy {
    #[inline(always)]
    fn default() -> P201PfsBy {
        <crate::RegValueT<P201PfsBy_SPEC> as RegisterValue<_>>::new(16)
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
pub struct P407Pfs_SPEC;
impl crate::sealed::RegSpec for P407Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P407 Pin Function Control Register"]
pub type P407Pfs = crate::RegValueT<P407Pfs_SPEC>;

impl P407Pfs {
    #[doc = "Port Function Select\nThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P407Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P407Pfs_SPEC,crate::common::RW>::from_register(self,0)
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
        p407pfs::Pmr,
        p407pfs::Pmr,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p407pfs::Pmr,
            p407pfs::Pmr,
            P407Pfs_SPEC,
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
        p407pfs::Asel,
        p407pfs::Asel,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p407pfs::Asel,
            p407pfs::Asel,
            P407Pfs_SPEC,
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
        p407pfs::Isel,
        p407pfs::Isel,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p407pfs::Isel,
            p407pfs::Isel,
            P407Pfs_SPEC,
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
        p407pfs::Dscr1,
        p407pfs::Dscr1,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p407pfs::Dscr1,
            p407pfs::Dscr1,
            P407Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P407Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, P407Pfs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        p407pfs::Ncodr,
        p407pfs::Ncodr,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p407pfs::Ncodr,
            p407pfs::Ncodr,
            P407Pfs_SPEC,
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
        p407pfs::Pcr,
        p407pfs::Pcr,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p407pfs::Pcr,
            p407pfs::Pcr,
            P407Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P407Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, P407Pfs_SPEC, crate::common::RW>::from_register(
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
        p407pfs::Pdr,
        p407pfs::Pdr,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p407pfs::Pdr,
            p407pfs::Pdr,
            P407Pfs_SPEC,
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
        p407pfs::Pidr,
        p407pfs::Pidr,
        P407Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p407pfs::Pidr,
            p407pfs::Pidr,
            P407Pfs_SPEC,
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
        p407pfs::Podr,
        p407pfs::Podr,
        P407Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p407pfs::Podr,
            p407pfs::Podr,
            P407Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P407Pfs {
    #[inline(always)]
    fn default() -> P407Pfs {
        <crate::RegValueT<P407Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p407pfs {

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
    pub struct Dscr1_SPEC;
    pub type Dscr1 = crate::EnumBitfieldStruct<u8, Dscr1_SPEC>;
    impl Dscr1 {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Middle drive for IIC Fast-mode and SPI"]
        pub const _10: Self = Self::new(0);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(1);
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
pub struct P407PfsHa_SPEC;
impl crate::sealed::RegSpec for P407PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P407 Pin Function Control Register"]
pub type P407PfsHa = crate::RegValueT<P407PfsHa_SPEC>;

impl P407PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p407pfs_ha::Asel,
        p407pfs_ha::Asel,
        P407PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p407pfs_ha::Asel,
            p407pfs_ha::Asel,
            P407PfsHa_SPEC,
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
        p407pfs_ha::Isel,
        p407pfs_ha::Isel,
        P407PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p407pfs_ha::Isel,
            p407pfs_ha::Isel,
            P407PfsHa_SPEC,
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
        p407pfs_ha::Dscr1,
        p407pfs_ha::Dscr1,
        P407PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            p407pfs_ha::Dscr1,
            p407pfs_ha::Dscr1,
            P407PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P407PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P407PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
        p407pfs_ha::Ncodr,
        p407pfs_ha::Ncodr,
        P407PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p407pfs_ha::Ncodr,
            p407pfs_ha::Ncodr,
            P407PfsHa_SPEC,
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
        p407pfs_ha::Pcr,
        p407pfs_ha::Pcr,
        P407PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p407pfs_ha::Pcr,
            p407pfs_ha::Pcr,
            P407PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P407PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P407PfsHa_SPEC,crate::common::RW>::from_register(self,0)
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
        p407pfs_ha::Pdr,
        p407pfs_ha::Pdr,
        P407PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p407pfs_ha::Pdr,
            p407pfs_ha::Pdr,
            P407PfsHa_SPEC,
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
        p407pfs_ha::Pidr,
        p407pfs_ha::Pidr,
        P407PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p407pfs_ha::Pidr,
            p407pfs_ha::Pidr,
            P407PfsHa_SPEC,
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
        p407pfs_ha::Podr,
        p407pfs_ha::Podr,
        P407PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p407pfs_ha::Podr,
            p407pfs_ha::Podr,
            P407PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P407PfsHa {
    #[inline(always)]
    fn default() -> P407PfsHa {
        <crate::RegValueT<P407PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p407pfs_ha {

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
    pub struct Dscr1_SPEC;
    pub type Dscr1 = crate::EnumBitfieldStruct<u8, Dscr1_SPEC>;
    impl Dscr1 {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Middle drive for IIC Fast-mode and SPI"]
        pub const _10: Self = Self::new(0);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(1);
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
pub struct P407PfsBy_SPEC;
impl crate::sealed::RegSpec for P407PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P407 Pin Function Control Register"]
pub type P407PfsBy = crate::RegValueT<P407PfsBy_SPEC>;

impl P407PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p407pfs_by::Ncodr,
        p407pfs_by::Ncodr,
        P407PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p407pfs_by::Ncodr,
            p407pfs_by::Ncodr,
            P407PfsBy_SPEC,
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
        p407pfs_by::Pcr,
        p407pfs_by::Pcr,
        P407PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p407pfs_by::Pcr,
            p407pfs_by::Pcr,
            P407PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, P407PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,P407PfsBy_SPEC,crate::common::RW>::from_register(self,0)
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
        p407pfs_by::Pdr,
        p407pfs_by::Pdr,
        P407PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p407pfs_by::Pdr,
            p407pfs_by::Pdr,
            P407PfsBy_SPEC,
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
        p407pfs_by::Pidr,
        p407pfs_by::Pidr,
        P407PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p407pfs_by::Pidr,
            p407pfs_by::Pidr,
            P407PfsBy_SPEC,
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
        p407pfs_by::Podr,
        p407pfs_by::Podr,
        P407PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p407pfs_by::Podr,
            p407pfs_by::Podr,
            P407PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P407PfsBy {
    #[inline(always)]
    fn default() -> P407PfsBy {
        <crate::RegValueT<P407PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p407pfs_by {

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
pub struct P914Pfs_SPEC;
impl crate::sealed::RegSpec for P914Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P914 Pin Function Control Register"]
pub type P914Pfs = crate::RegValueT<P914Pfs_SPEC>;

impl NoBitfieldReg<P914Pfs_SPEC> for P914Pfs {}
impl ::core::default::Default for P914Pfs {
    #[inline(always)]
    fn default() -> P914Pfs {
        <crate::RegValueT<P914Pfs_SPEC> as RegisterValue<_>>::new(65536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P914PfsHa_SPEC;
impl crate::sealed::RegSpec for P914PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P914 Pin Function Control Register"]
pub type P914PfsHa = crate::RegValueT<P914PfsHa_SPEC>;

impl NoBitfieldReg<P914PfsHa_SPEC> for P914PfsHa {}
impl ::core::default::Default for P914PfsHa {
    #[inline(always)]
    fn default() -> P914PfsHa {
        <crate::RegValueT<P914PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P914PfsBy_SPEC;
impl crate::sealed::RegSpec for P914PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P914 Pin Function Control Register"]
pub type P914PfsBy = crate::RegValueT<P914PfsBy_SPEC>;

impl NoBitfieldReg<P914PfsBy_SPEC> for P914PfsBy {}
impl ::core::default::Default for P914PfsBy {
    #[inline(always)]
    fn default() -> P914PfsBy {
        <crate::RegValueT<P914PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P915Pfs_SPEC;
impl crate::sealed::RegSpec for P915Pfs_SPEC {
    type DataType = u32;
}

#[doc = "P915 Pin Function Control Register"]
pub type P915Pfs = crate::RegValueT<P915Pfs_SPEC>;

impl NoBitfieldReg<P915Pfs_SPEC> for P915Pfs {}
impl ::core::default::Default for P915Pfs {
    #[inline(always)]
    fn default() -> P915Pfs {
        <crate::RegValueT<P915Pfs_SPEC> as RegisterValue<_>>::new(65536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P915PfsHa_SPEC;
impl crate::sealed::RegSpec for P915PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "P915 Pin Function Control Register"]
pub type P915PfsHa = crate::RegValueT<P915PfsHa_SPEC>;

impl NoBitfieldReg<P915PfsHa_SPEC> for P915PfsHa {}
impl ::core::default::Default for P915PfsHa {
    #[inline(always)]
    fn default() -> P915PfsHa {
        <crate::RegValueT<P915PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P915PfsBy_SPEC;
impl crate::sealed::RegSpec for P915PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "P915 Pin Function Control Register"]
pub type P915PfsBy = crate::RegValueT<P915PfsBy_SPEC>;

impl NoBitfieldReg<P915PfsBy_SPEC> for P915PfsBy {}
impl ::core::default::Default for P915PfsBy {
    #[inline(always)]
    fn default() -> P915PfsBy {
        <crate::RegValueT<P915PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
