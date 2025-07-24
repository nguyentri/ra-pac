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
// Generated from SVD 1.30.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:47 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Control Register"]
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10usize))
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

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12usize))
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

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13usize))
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

    #[doc = "Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P008Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P008Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P008PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P008PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "Port 008 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p008pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P008PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P008PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(35usize),
            )
        }
    }

    #[doc = "Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x34usize))
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x36usize))
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x37usize))
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x48usize))
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
    #[inline(always)]
    pub const fn p108pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p109pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4ausize))
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
    #[inline(always)]
    pub const fn p108pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p109pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4busize))
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
    #[inline(always)]
    pub const fn p108pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p109pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67usize),
            )
        }
    }

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW>,
        4,
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

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW>,
        4,
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

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW>,
        4,
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
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x94usize))
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
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x96usize))
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
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x97usize))
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
        2,
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

    #[doc = "Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p2pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW>,
        2,
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

    #[doc = "Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p2pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW>,
        2,
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

    #[doc = "Port 30%s Pin Function Select Register"]
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

    #[doc = "Port 30%s Pin Function Select Register"]
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

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11cusize))
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11eusize))
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
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11fusize))
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

    #[doc = "Port 4%s Pin Function Select Register"]
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

    #[doc = "Port 4%s Pin Function Select Register"]
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

    #[doc = "Port 500 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P500Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P500Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "Port 500 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P500PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P500PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(322usize),
            )
        }
    }

    #[doc = "Port 500 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p500pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P500PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P500PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(323usize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x238usize))
        }
    }
    #[inline(always)]
    pub const fn p8014pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p8015pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x23ausize))
        }
    }
    #[inline(always)]
    pub const fn p8014pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn p8015pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23eusize),
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x23busize))
        }
    }
    #[inline(always)]
    pub const fn p8014pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23busize),
            )
        }
    }
    #[inline(always)]
    pub const fn p8015pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P80PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23fusize),
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

    #[doc = "Write-Protect Register for Secure"]
    #[inline(always)]
    pub const fn pwprs(&self) -> &'static crate::common::Reg<self::Pwprs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwprs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1285usize),
            )
        }
    }

    #[doc = "RI3C Slope Control Register"]
    #[inline(always)]
    pub const fn pfi3c(&self) -> &'static crate::common::Reg<self::Pfi3C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pfi3C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1292usize),
            )
        }
    }

    #[doc = "Port Security Attribution register"]
    #[inline(always)]
    pub const fn psar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Psar_SPEC, crate::common::RW>,
        6,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x510usize))
        }
    }
    #[inline(always)]
    pub const fn p0sar(&self) -> &'static crate::common::Reg<self::Psar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p1sar(&self) -> &'static crate::common::Reg<self::Psar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x512usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p2sar(&self) -> &'static crate::common::Reg<self::Psar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x514usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p3sar(&self) -> &'static crate::common::Reg<self::Psar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x516usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p4sar(&self) -> &'static crate::common::Reg<self::Psar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x518usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p5sar(&self) -> &'static crate::common::Reg<self::Psar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51ausize),
            )
        }
    }

    #[doc = "Port Security Attribution register"]
    #[inline(always)]
    pub const fn p8sar(&self) -> &'static crate::common::Reg<self::P8Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P8Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1312usize),
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p00pfs::Dscr,
        p00pfs::Dscr,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p00pfs::Dscr,
            p00pfs::Dscr,
            P00Pfs_SPEC,
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
        p00pfs::Eofr,
        p00pfs::Eofr,
        P00Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p00pfs::Eofr,
            p00pfs::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p00pfs_ha::Dscr,
        p00pfs_ha::Dscr,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p00pfs_ha::Dscr,
            p00pfs_ha::Dscr,
            P00PfsHa_SPEC,
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
        p00pfs_ha::Eofr,
        p00pfs_ha::Eofr,
        P00PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p00pfs_ha::Eofr,
            p00pfs_ha::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
pub struct P008Pfs_SPEC;
impl crate::sealed::RegSpec for P008Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 008 Pin Function Select Register"]
pub type P008Pfs = crate::RegValueT<P008Pfs_SPEC>;

impl P008Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p008pfs::Podr,
        p008pfs::Podr,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p008pfs::Podr,
            p008pfs::Podr,
            P008Pfs_SPEC,
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
        p008pfs::Pidr,
        p008pfs::Pidr,
        P008Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p008pfs::Pidr,
            p008pfs::Pidr,
            P008Pfs_SPEC,
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
        p008pfs::Pdr,
        p008pfs::Pdr,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p008pfs::Pdr,
            p008pfs::Pdr,
            P008Pfs_SPEC,
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
        p008pfs::Pcr,
        p008pfs::Pcr,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p008pfs::Pcr,
            p008pfs::Pcr,
            P008Pfs_SPEC,
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
        p008pfs::Ncodr,
        p008pfs::Ncodr,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p008pfs::Ncodr,
            p008pfs::Ncodr,
            P008Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p008pfs::Dscr,
        p008pfs::Dscr,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p008pfs::Dscr,
            p008pfs::Dscr,
            P008Pfs_SPEC,
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
        p008pfs::Eofr,
        p008pfs::Eofr,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p008pfs::Eofr,
            p008pfs::Eofr,
            P008Pfs_SPEC,
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
        p008pfs::Isel,
        p008pfs::Isel,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p008pfs::Isel,
            p008pfs::Isel,
            P008Pfs_SPEC,
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
        p008pfs::Asel,
        p008pfs::Asel,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p008pfs::Asel,
            p008pfs::Asel,
            P008Pfs_SPEC,
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
        p008pfs::Pmr,
        p008pfs::Pmr,
        P008Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p008pfs::Pmr,
            p008pfs::Pmr,
            P008Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P008Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P008Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P008Pfs {
    #[inline(always)]
    fn default() -> P008Pfs {
        <crate::RegValueT<P008Pfs_SPEC> as RegisterValue<_>>::new(66576)
    }
}
pub mod p008pfs {

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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
pub struct P008PfsHa_SPEC;
impl crate::sealed::RegSpec for P008PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 008 Pin Function Select Register"]
pub type P008PfsHa = crate::RegValueT<P008PfsHa_SPEC>;

impl P008PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p008pfs_ha::Podr,
        p008pfs_ha::Podr,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p008pfs_ha::Podr,
            p008pfs_ha::Podr,
            P008PfsHa_SPEC,
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
        p008pfs_ha::Pidr,
        p008pfs_ha::Pidr,
        P008PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p008pfs_ha::Pidr,
            p008pfs_ha::Pidr,
            P008PfsHa_SPEC,
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
        p008pfs_ha::Pdr,
        p008pfs_ha::Pdr,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p008pfs_ha::Pdr,
            p008pfs_ha::Pdr,
            P008PfsHa_SPEC,
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
        p008pfs_ha::Pcr,
        p008pfs_ha::Pcr,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p008pfs_ha::Pcr,
            p008pfs_ha::Pcr,
            P008PfsHa_SPEC,
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
        p008pfs_ha::Ncodr,
        p008pfs_ha::Ncodr,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p008pfs_ha::Ncodr,
            p008pfs_ha::Ncodr,
            P008PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p008pfs_ha::Dscr,
        p008pfs_ha::Dscr,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p008pfs_ha::Dscr,
            p008pfs_ha::Dscr,
            P008PfsHa_SPEC,
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
        p008pfs_ha::Eofr,
        p008pfs_ha::Eofr,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p008pfs_ha::Eofr,
            p008pfs_ha::Eofr,
            P008PfsHa_SPEC,
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
        p008pfs_ha::Isel,
        p008pfs_ha::Isel,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p008pfs_ha::Isel,
            p008pfs_ha::Isel,
            P008PfsHa_SPEC,
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
        p008pfs_ha::Asel,
        p008pfs_ha::Asel,
        P008PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p008pfs_ha::Asel,
            p008pfs_ha::Asel,
            P008PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P008PfsHa {
    #[inline(always)]
    fn default() -> P008PfsHa {
        <crate::RegValueT<P008PfsHa_SPEC> as RegisterValue<_>>::new(1040)
    }
}
pub mod p008pfs_ha {

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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
pub struct P008PfsBy_SPEC;
impl crate::sealed::RegSpec for P008PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 008 Pin Function Select Register"]
pub type P008PfsBy = crate::RegValueT<P008PfsBy_SPEC>;

impl P008PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p008pfs_by::Podr,
        p008pfs_by::Podr,
        P008PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p008pfs_by::Podr,
            p008pfs_by::Podr,
            P008PfsBy_SPEC,
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
        p008pfs_by::Pidr,
        p008pfs_by::Pidr,
        P008PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p008pfs_by::Pidr,
            p008pfs_by::Pidr,
            P008PfsBy_SPEC,
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
        p008pfs_by::Pdr,
        p008pfs_by::Pdr,
        P008PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p008pfs_by::Pdr,
            p008pfs_by::Pdr,
            P008PfsBy_SPEC,
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
        p008pfs_by::Pcr,
        p008pfs_by::Pcr,
        P008PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p008pfs_by::Pcr,
            p008pfs_by::Pcr,
            P008PfsBy_SPEC,
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
        p008pfs_by::Ncodr,
        p008pfs_by::Ncodr,
        P008PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p008pfs_by::Ncodr,
            p008pfs_by::Ncodr,
            P008PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P008PfsBy {
    #[inline(always)]
    fn default() -> P008PfsBy {
        <crate::RegValueT<P008PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p008pfs_by {

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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p0pfs::Dscr,
        p0pfs::Dscr,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p0pfs::Dscr,
            p0pfs::Dscr,
            P0Pfs_SPEC,
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
        p0pfs::Eofr,
        p0pfs::Eofr,
        P0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p0pfs::Eofr,
            p0pfs::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p0pfs_ha::Dscr,
        p0pfs_ha::Dscr,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p0pfs_ha::Dscr,
            p0pfs_ha::Dscr,
            P0PfsHa_SPEC,
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
        p0pfs_ha::Eofr,
        p0pfs_ha::Eofr,
        P0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p0pfs_ha::Eofr,
            p0pfs_ha::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p10pfs::Dscr,
        p10pfs::Dscr,
        P10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p10pfs::Dscr,
            p10pfs::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p10pfs_ha::Dscr,
        p10pfs_ha::Dscr,
        P10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p10pfs_ha::Dscr,
            p10pfs_ha::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p1pfs::Dscr,
        p1pfs::Dscr,
        P1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p1pfs::Dscr,
            p1pfs::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p1pfs_ha::Dscr,
        p1pfs_ha::Dscr,
        P1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p1pfs_ha::Dscr,
            p1pfs_ha::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p200pfs::Dscr,
        p200pfs::Dscr,
        P200Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p200pfs::Dscr,
            p200pfs::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p200pfs_ha::Dscr,
        p200pfs_ha::Dscr,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p200pfs_ha::Dscr,
            p200pfs_ha::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p20pfs::Dscr,
        p20pfs::Dscr,
        P20Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p20pfs::Dscr,
            p20pfs::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p20pfs_ha::Dscr,
        p20pfs_ha::Dscr,
        P20PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p20pfs_ha::Dscr,
            p20pfs_ha::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p2pfs::Dscr,
        p2pfs::Dscr,
        P2Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p2pfs::Dscr,
            p2pfs::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p2pfs_ha::Dscr,
        p2pfs_ha::Dscr,
        P2PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p2pfs_ha::Dscr,
            p2pfs_ha::Dscr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p300pfs::Dscr,
        p300pfs::Dscr,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p300pfs::Dscr,
            p300pfs::Dscr,
            P300Pfs_SPEC,
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
        p300pfs::Eofr,
        p300pfs::Eofr,
        P300Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p300pfs::Eofr,
            p300pfs::Eofr,
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
        <crate::RegValueT<P300Pfs_SPEC> as RegisterValue<_>>::new(65552)
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p300pfs_ha::Dscr,
        p300pfs_ha::Dscr,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p300pfs_ha::Dscr,
            p300pfs_ha::Dscr,
            P300PfsHa_SPEC,
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
        p300pfs_ha::Eofr,
        p300pfs_ha::Eofr,
        P300PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p300pfs_ha::Eofr,
            p300pfs_ha::Eofr,
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
        <crate::RegValueT<P300PfsHa_SPEC> as RegisterValue<_>>::new(16)
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
        <crate::RegValueT<P300PfsBy_SPEC> as RegisterValue<_>>::new(16)
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p30pfs::Dscr,
        p30pfs::Dscr,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p30pfs::Dscr,
            p30pfs::Dscr,
            P30Pfs_SPEC,
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
        p30pfs::Eofr,
        p30pfs::Eofr,
        P30Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p30pfs::Eofr,
            p30pfs::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p30pfs_ha::Dscr,
        p30pfs_ha::Dscr,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p30pfs_ha::Dscr,
            p30pfs_ha::Dscr,
            P30PfsHa_SPEC,
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
        p30pfs_ha::Eofr,
        p30pfs_ha::Eofr,
        P30PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p30pfs_ha::Eofr,
            p30pfs_ha::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p40pfs::Dscr,
        p40pfs::Dscr,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p40pfs::Dscr,
            p40pfs::Dscr,
            P40Pfs_SPEC,
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
        p40pfs::Eofr,
        p40pfs::Eofr,
        P40Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p40pfs::Eofr,
            p40pfs::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p40pfs_ha::Dscr,
        p40pfs_ha::Dscr,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p40pfs_ha::Dscr,
            p40pfs_ha::Dscr,
            P40PfsHa_SPEC,
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
        p40pfs_ha::Eofr,
        p40pfs_ha::Eofr,
        P40PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p40pfs_ha::Eofr,
            p40pfs_ha::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p4pfs::Dscr,
        p4pfs::Dscr,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p4pfs::Dscr,
            p4pfs::Dscr,
            P4Pfs_SPEC,
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
        p4pfs::Eofr,
        p4pfs::Eofr,
        P4Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p4pfs::Eofr,
            p4pfs::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p4pfs_ha::Dscr,
        p4pfs_ha::Dscr,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p4pfs_ha::Dscr,
            p4pfs_ha::Dscr,
            P4PfsHa_SPEC,
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
        p4pfs_ha::Eofr,
        p4pfs_ha::Eofr,
        P4PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p4pfs_ha::Eofr,
            p4pfs_ha::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
pub struct P500Pfs_SPEC;
impl crate::sealed::RegSpec for P500Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 500 Pin Function Select Register"]
pub type P500Pfs = crate::RegValueT<P500Pfs_SPEC>;

impl P500Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p500pfs::Podr,
        p500pfs::Podr,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p500pfs::Podr,
            p500pfs::Podr,
            P500Pfs_SPEC,
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
        p500pfs::Pidr,
        p500pfs::Pidr,
        P500Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p500pfs::Pidr,
            p500pfs::Pidr,
            P500Pfs_SPEC,
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
        p500pfs::Pdr,
        p500pfs::Pdr,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p500pfs::Pdr,
            p500pfs::Pdr,
            P500Pfs_SPEC,
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
        p500pfs::Pcr,
        p500pfs::Pcr,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p500pfs::Pcr,
            p500pfs::Pcr,
            P500Pfs_SPEC,
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
        p500pfs::Ncodr,
        p500pfs::Ncodr,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p500pfs::Ncodr,
            p500pfs::Ncodr,
            P500Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p500pfs::Dscr,
        p500pfs::Dscr,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p500pfs::Dscr,
            p500pfs::Dscr,
            P500Pfs_SPEC,
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
        p500pfs::Eofr,
        p500pfs::Eofr,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p500pfs::Eofr,
            p500pfs::Eofr,
            P500Pfs_SPEC,
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
        p500pfs::Isel,
        p500pfs::Isel,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p500pfs::Isel,
            p500pfs::Isel,
            P500Pfs_SPEC,
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
        p500pfs::Asel,
        p500pfs::Asel,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p500pfs::Asel,
            p500pfs::Asel,
            P500Pfs_SPEC,
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
        p500pfs::Pmr,
        p500pfs::Pmr,
        P500Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p500pfs::Pmr,
            p500pfs::Pmr,
            P500Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P500Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P500Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P500Pfs {
    #[inline(always)]
    fn default() -> P500Pfs {
        <crate::RegValueT<P500Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p500pfs {

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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
pub struct P500PfsHa_SPEC;
impl crate::sealed::RegSpec for P500PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 500 Pin Function Select Register"]
pub type P500PfsHa = crate::RegValueT<P500PfsHa_SPEC>;

impl P500PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p500pfs_ha::Podr,
        p500pfs_ha::Podr,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p500pfs_ha::Podr,
            p500pfs_ha::Podr,
            P500PfsHa_SPEC,
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
        p500pfs_ha::Pidr,
        p500pfs_ha::Pidr,
        P500PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p500pfs_ha::Pidr,
            p500pfs_ha::Pidr,
            P500PfsHa_SPEC,
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
        p500pfs_ha::Pdr,
        p500pfs_ha::Pdr,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p500pfs_ha::Pdr,
            p500pfs_ha::Pdr,
            P500PfsHa_SPEC,
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
        p500pfs_ha::Pcr,
        p500pfs_ha::Pcr,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p500pfs_ha::Pcr,
            p500pfs_ha::Pcr,
            P500PfsHa_SPEC,
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
        p500pfs_ha::Ncodr,
        p500pfs_ha::Ncodr,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p500pfs_ha::Ncodr,
            p500pfs_ha::Ncodr,
            P500PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p500pfs_ha::Dscr,
        p500pfs_ha::Dscr,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p500pfs_ha::Dscr,
            p500pfs_ha::Dscr,
            P500PfsHa_SPEC,
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
        p500pfs_ha::Eofr,
        p500pfs_ha::Eofr,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p500pfs_ha::Eofr,
            p500pfs_ha::Eofr,
            P500PfsHa_SPEC,
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
        p500pfs_ha::Isel,
        p500pfs_ha::Isel,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p500pfs_ha::Isel,
            p500pfs_ha::Isel,
            P500PfsHa_SPEC,
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
        p500pfs_ha::Asel,
        p500pfs_ha::Asel,
        P500PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p500pfs_ha::Asel,
            p500pfs_ha::Asel,
            P500PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P500PfsHa {
    #[inline(always)]
    fn default() -> P500PfsHa {
        <crate::RegValueT<P500PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p500pfs_ha {

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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
pub struct P500PfsBy_SPEC;
impl crate::sealed::RegSpec for P500PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 500 Pin Function Select Register"]
pub type P500PfsBy = crate::RegValueT<P500PfsBy_SPEC>;

impl P500PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p500pfs_by::Podr,
        p500pfs_by::Podr,
        P500PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p500pfs_by::Podr,
            p500pfs_by::Podr,
            P500PfsBy_SPEC,
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
        p500pfs_by::Pidr,
        p500pfs_by::Pidr,
        P500PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p500pfs_by::Pidr,
            p500pfs_by::Pidr,
            P500PfsBy_SPEC,
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
        p500pfs_by::Pdr,
        p500pfs_by::Pdr,
        P500PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p500pfs_by::Pdr,
            p500pfs_by::Pdr,
            P500PfsBy_SPEC,
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
        p500pfs_by::Pcr,
        p500pfs_by::Pcr,
        P500PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p500pfs_by::Pcr,
            p500pfs_by::Pcr,
            P500PfsBy_SPEC,
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
        p500pfs_by::Ncodr,
        p500pfs_by::Ncodr,
        P500PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p500pfs_by::Ncodr,
            p500pfs_by::Ncodr,
            P500PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P500PfsBy {
    #[inline(always)]
    fn default() -> P500PfsBy {
        <crate::RegValueT<P500PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p500pfs_by {

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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p80pfs::Dscr,
        p80pfs::Dscr,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p80pfs::Dscr,
            p80pfs::Dscr,
            P80Pfs_SPEC,
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
        p80pfs::Eofr,
        p80pfs::Eofr,
        P80Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p80pfs::Eofr,
            p80pfs::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p80pfs_ha::Dscr,
        p80pfs_ha::Dscr,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p80pfs_ha::Dscr,
            p80pfs_ha::Dscr,
            P80PfsHa_SPEC,
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
        p80pfs_ha::Eofr,
        p80pfs_ha::Eofr,
        P80PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p80pfs_ha::Eofr,
            p80pfs_ha::Eofr,
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);

        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
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
pub struct Pwprs_SPEC;
impl crate::sealed::RegSpec for Pwprs_SPEC {
    type DataType = u8;
}

#[doc = "Write-Protect Register for Secure"]
pub type Pwprs = crate::RegValueT<Pwprs_SPEC>;

impl Pwprs {
    #[doc = "PmnPFS Register Write Enable"]
    #[inline(always)]
    pub fn pfswe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pwprs::Pfswe,
        pwprs::Pfswe,
        Pwprs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pwprs::Pfswe,
            pwprs::Pfswe,
            Pwprs_SPEC,
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
        pwprs::B0Wi,
        pwprs::B0Wi,
        Pwprs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pwprs::B0Wi,
            pwprs::B0Wi,
            Pwprs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pwprs {
    #[inline(always)]
    fn default() -> Pwprs {
        <crate::RegValueT<Pwprs_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod pwprs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfswe_SPEC;
    pub type Pfswe = crate::EnumBitfieldStruct<u8, Pfswe_SPEC>;
    impl Pfswe {
        #[doc = "Disable writes to the PmnPFS register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to the PmnPFS register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B0Wi_SPEC;
    pub type B0Wi = crate::EnumBitfieldStruct<u8, B0Wi_SPEC>;
    impl B0Wi {
        #[doc = "Enable writes the PFSWE bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable writes to the PFSWE bit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi3C_SPEC;
impl crate::sealed::RegSpec for Pfi3C_SPEC {
    type DataType = u8;
}

#[doc = "RI3C Slope Control Register"]
pub type Pfi3C = crate::RegValueT<Pfi3C_SPEC>;

impl Pfi3C {
    #[doc = "I3C mode slope control bit"]
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
pub struct Psar_SPEC;
impl crate::sealed::RegSpec for Psar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribution register"]
pub type Psar = crate::RegValueT<Psar_SPEC>;

impl Psar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        psar::Pmnsa,
        psar::Pmnsa,
        Psar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            psar::Pmnsa,
            psar::Pmnsa,
            Psar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psar {
    #[inline(always)]
    fn default() -> Psar {
        <crate::RegValueT<Psar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod psar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmnsa_SPEC;
    pub type Pmnsa = crate::EnumBitfieldStruct<u8, Pmnsa_SPEC>;
    impl Pmnsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8Sar_SPEC;
impl crate::sealed::RegSpec for P8Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port Security Attribution register"]
pub type P8Sar = crate::RegValueT<P8Sar_SPEC>;

impl P8Sar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        p8sar::Pmnsa,
        p8sar::Pmnsa,
        P8Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            p8sar::Pmnsa,
            p8sar::Pmnsa,
            P8Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P8Sar {
    #[inline(always)]
    fn default() -> P8Sar {
        <crate::RegValueT<P8Sar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod p8sar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmnsa_SPEC;
    pub type Pmnsa = crate::EnumBitfieldStruct<u8, Pmnsa_SPEC>;
    impl Pmnsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
