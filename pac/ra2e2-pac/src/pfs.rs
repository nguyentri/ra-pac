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
// Generated from SVD 1.40.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:37 +0000

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

    #[doc = "Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x38usize))
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
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3ausize))
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
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3busize))
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
        4,
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

    #[doc = "Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW>,
        4,
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

    #[doc = "Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW>,
        4,
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

    #[doc = "Port 1%s Pin Function Select Register"]
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

    #[doc = "Port 1%s Pin Function Select Register"]
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

    #[doc = "Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P205Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P205Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P205PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P205PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(150usize),
            )
        }
    }

    #[doc = "Port 205 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p205pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P205PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P205PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(151usize),
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

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW>,
        2,
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

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW>,
        2,
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

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW>,
        2,
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

    #[doc = "Port 914 Pin Function Select Register"]
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

    #[doc = "Port 914 Pin Function Select Register"]
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

    #[doc = "Port 914 Pin Function Select Register"]
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
pub struct P205Pfs_SPEC;
impl crate::sealed::RegSpec for P205Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 205 Pin Function Select Register"]
pub type P205Pfs = crate::RegValueT<P205Pfs_SPEC>;

impl P205Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p205pfs::Podr,
        p205pfs::Podr,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p205pfs::Podr,
            p205pfs::Podr,
            P205Pfs_SPEC,
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
        p205pfs::Pidr,
        p205pfs::Pidr,
        P205Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p205pfs::Pidr,
            p205pfs::Pidr,
            P205Pfs_SPEC,
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
        p205pfs::Pdr,
        p205pfs::Pdr,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p205pfs::Pdr,
            p205pfs::Pdr,
            P205Pfs_SPEC,
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
        p205pfs::Pcr,
        p205pfs::Pcr,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p205pfs::Pcr,
            p205pfs::Pcr,
            P205Pfs_SPEC,
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
        p205pfs::Ncodr,
        p205pfs::Ncodr,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p205pfs::Ncodr,
            p205pfs::Ncodr,
            P205Pfs_SPEC,
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
        p205pfs::Eofr,
        p205pfs::Eofr,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p205pfs::Eofr,
            p205pfs::Eofr,
            P205Pfs_SPEC,
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
        p205pfs::Isel,
        p205pfs::Isel,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p205pfs::Isel,
            p205pfs::Isel,
            P205Pfs_SPEC,
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
        p205pfs::Asel,
        p205pfs::Asel,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p205pfs::Asel,
            p205pfs::Asel,
            P205Pfs_SPEC,
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
        p205pfs::Pmr,
        p205pfs::Pmr,
        P205Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p205pfs::Pmr,
            p205pfs::Pmr,
            P205Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P205Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P205Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P205Pfs {
    #[inline(always)]
    fn default() -> P205Pfs {
        <crate::RegValueT<P205Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p205pfs {

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
pub struct P205PfsHa_SPEC;
impl crate::sealed::RegSpec for P205PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 205 Pin Function Select Register"]
pub type P205PfsHa = crate::RegValueT<P205PfsHa_SPEC>;

impl P205PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p205pfs_ha::Podr,
        p205pfs_ha::Podr,
        P205PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p205pfs_ha::Podr,
            p205pfs_ha::Podr,
            P205PfsHa_SPEC,
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
        p205pfs_ha::Pidr,
        p205pfs_ha::Pidr,
        P205PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p205pfs_ha::Pidr,
            p205pfs_ha::Pidr,
            P205PfsHa_SPEC,
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
        p205pfs_ha::Pdr,
        p205pfs_ha::Pdr,
        P205PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p205pfs_ha::Pdr,
            p205pfs_ha::Pdr,
            P205PfsHa_SPEC,
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
        p205pfs_ha::Pcr,
        p205pfs_ha::Pcr,
        P205PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p205pfs_ha::Pcr,
            p205pfs_ha::Pcr,
            P205PfsHa_SPEC,
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
        p205pfs_ha::Ncodr,
        p205pfs_ha::Ncodr,
        P205PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p205pfs_ha::Ncodr,
            p205pfs_ha::Ncodr,
            P205PfsHa_SPEC,
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
        p205pfs_ha::Eofr,
        p205pfs_ha::Eofr,
        P205PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            p205pfs_ha::Eofr,
            p205pfs_ha::Eofr,
            P205PfsHa_SPEC,
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
        p205pfs_ha::Isel,
        p205pfs_ha::Isel,
        P205PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p205pfs_ha::Isel,
            p205pfs_ha::Isel,
            P205PfsHa_SPEC,
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
        p205pfs_ha::Asel,
        p205pfs_ha::Asel,
        P205PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p205pfs_ha::Asel,
            p205pfs_ha::Asel,
            P205PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P205PfsHa {
    #[inline(always)]
    fn default() -> P205PfsHa {
        <crate::RegValueT<P205PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p205pfs_ha {

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
pub struct P205PfsBy_SPEC;
impl crate::sealed::RegSpec for P205PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 205 Pin Function Select Register"]
pub type P205PfsBy = crate::RegValueT<P205PfsBy_SPEC>;

impl P205PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p205pfs_by::Podr,
        p205pfs_by::Podr,
        P205PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p205pfs_by::Podr,
            p205pfs_by::Podr,
            P205PfsBy_SPEC,
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
        p205pfs_by::Pidr,
        p205pfs_by::Pidr,
        P205PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p205pfs_by::Pidr,
            p205pfs_by::Pidr,
            P205PfsBy_SPEC,
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
        p205pfs_by::Pdr,
        p205pfs_by::Pdr,
        P205PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p205pfs_by::Pdr,
            p205pfs_by::Pdr,
            P205PfsBy_SPEC,
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
        p205pfs_by::Pcr,
        p205pfs_by::Pcr,
        P205PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p205pfs_by::Pcr,
            p205pfs_by::Pcr,
            P205PfsBy_SPEC,
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
        p205pfs_by::Ncodr,
        p205pfs_by::Ncodr,
        P205PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p205pfs_by::Ncodr,
            p205pfs_by::Ncodr,
            P205PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P205PfsBy {
    #[inline(always)]
    fn default() -> P205PfsBy {
        <crate::RegValueT<P205PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p205pfs_by {

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
pub struct P914Pfs_SPEC;
impl crate::sealed::RegSpec for P914Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 914 Pin Function Select Register"]
pub type P914Pfs = crate::RegValueT<P914Pfs_SPEC>;

impl P914Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p914pfs::Podr,
        p914pfs::Podr,
        P914Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p914pfs::Podr,
            p914pfs::Podr,
            P914Pfs_SPEC,
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
        p914pfs::Pidr,
        p914pfs::Pidr,
        P914Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p914pfs::Pidr,
            p914pfs::Pidr,
            P914Pfs_SPEC,
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
        p914pfs::Pdr,
        p914pfs::Pdr,
        P914Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p914pfs::Pdr,
            p914pfs::Pdr,
            P914Pfs_SPEC,
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
        p914pfs::Pcr,
        p914pfs::Pcr,
        P914Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p914pfs::Pcr,
            p914pfs::Pcr,
            P914Pfs_SPEC,
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
        p914pfs::Ncodr,
        p914pfs::Ncodr,
        P914Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p914pfs::Ncodr,
            p914pfs::Ncodr,
            P914Pfs_SPEC,
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
        p914pfs::Isel,
        p914pfs::Isel,
        P914Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p914pfs::Isel,
            p914pfs::Isel,
            P914Pfs_SPEC,
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
        p914pfs::Asel,
        p914pfs::Asel,
        P914Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p914pfs::Asel,
            p914pfs::Asel,
            P914Pfs_SPEC,
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
        p914pfs::Pmr,
        p914pfs::Pmr,
        P914Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p914pfs::Pmr,
            p914pfs::Pmr,
            P914Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P914Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P914Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P914Pfs {
    #[inline(always)]
    fn default() -> P914Pfs {
        <crate::RegValueT<P914Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p914pfs {

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
pub struct P914PfsHa_SPEC;
impl crate::sealed::RegSpec for P914PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 914 Pin Function Select Register"]
pub type P914PfsHa = crate::RegValueT<P914PfsHa_SPEC>;

impl P914PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p914pfs_ha::Podr,
        p914pfs_ha::Podr,
        P914PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p914pfs_ha::Podr,
            p914pfs_ha::Podr,
            P914PfsHa_SPEC,
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
        p914pfs_ha::Pidr,
        p914pfs_ha::Pidr,
        P914PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p914pfs_ha::Pidr,
            p914pfs_ha::Pidr,
            P914PfsHa_SPEC,
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
        p914pfs_ha::Pdr,
        p914pfs_ha::Pdr,
        P914PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p914pfs_ha::Pdr,
            p914pfs_ha::Pdr,
            P914PfsHa_SPEC,
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
        p914pfs_ha::Pcr,
        p914pfs_ha::Pcr,
        P914PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p914pfs_ha::Pcr,
            p914pfs_ha::Pcr,
            P914PfsHa_SPEC,
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
        p914pfs_ha::Ncodr,
        p914pfs_ha::Ncodr,
        P914PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p914pfs_ha::Ncodr,
            p914pfs_ha::Ncodr,
            P914PfsHa_SPEC,
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
        p914pfs_ha::Isel,
        p914pfs_ha::Isel,
        P914PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p914pfs_ha::Isel,
            p914pfs_ha::Isel,
            P914PfsHa_SPEC,
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
        p914pfs_ha::Asel,
        p914pfs_ha::Asel,
        P914PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p914pfs_ha::Asel,
            p914pfs_ha::Asel,
            P914PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P914PfsHa {
    #[inline(always)]
    fn default() -> P914PfsHa {
        <crate::RegValueT<P914PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p914pfs_ha {

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
pub struct P914PfsBy_SPEC;
impl crate::sealed::RegSpec for P914PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 914 Pin Function Select Register"]
pub type P914PfsBy = crate::RegValueT<P914PfsBy_SPEC>;

impl P914PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p914pfs_by::Podr,
        p914pfs_by::Podr,
        P914PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p914pfs_by::Podr,
            p914pfs_by::Podr,
            P914PfsBy_SPEC,
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
        p914pfs_by::Pidr,
        p914pfs_by::Pidr,
        P914PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p914pfs_by::Pidr,
            p914pfs_by::Pidr,
            P914PfsBy_SPEC,
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
        p914pfs_by::Pdr,
        p914pfs_by::Pdr,
        P914PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p914pfs_by::Pdr,
            p914pfs_by::Pdr,
            P914PfsBy_SPEC,
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
        p914pfs_by::Pcr,
        p914pfs_by::Pcr,
        P914PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p914pfs_by::Pcr,
            p914pfs_by::Pcr,
            P914PfsBy_SPEC,
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
        p914pfs_by::Ncodr,
        p914pfs_by::Ncodr,
        P914PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p914pfs_by::Ncodr,
            p914pfs_by::Ncodr,
            P914PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P914PfsBy {
    #[inline(always)]
    fn default() -> P914PfsBy {
        <crate::RegValueT<P914PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p914pfs_by {

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
