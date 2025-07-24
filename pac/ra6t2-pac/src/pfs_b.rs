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
#[doc = r"Pmn Pin Function Control Register"]
unsafe impl ::core::marker::Send for super::PfsB {}
unsafe impl ::core::marker::Sync for super::PfsB {}
impl super::PfsB {
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
        2,
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

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn p000pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p001pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn p000pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn p001pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }

    #[doc = "Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P002Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P002Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P002PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P002PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Port 002 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p002pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P002PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P002PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
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
                self._svd2pac_as_ptr().add(132usize),
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
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x298usize))
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

    #[doc = "Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x298usize))
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

    #[doc = "Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x298usize))
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

    #[doc = "Port A13 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa13pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa13Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa13Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(692usize),
            )
        }
    }

    #[doc = "Port A13 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa13pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa13PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa13PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(692usize),
            )
        }
    }

    #[doc = "Port A13 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa13pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa13PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa13PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(692usize),
            )
        }
    }

    #[doc = "Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa1Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2b8usize))
        }
    }
    #[inline(always)]
    pub const fn pa14pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa15pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }

    #[doc = "Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa1PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2b8usize))
        }
    }
    #[inline(always)]
    pub const fn pa14pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa15pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }

    #[doc = "Port A1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa1PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2b8usize))
        }
    }
    #[inline(always)]
    pub const fn pa14pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pa15pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }

    #[doc = "Port B02 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb02pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb02Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb02Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(712usize),
            )
        }
    }

    #[doc = "Port B02 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb02pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb02PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb02PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(712usize),
            )
        }
    }

    #[doc = "Port B02 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb02pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb02PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb02PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(712usize),
            )
        }
    }

    #[doc = "Port B03 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb03pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb03Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb03Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(716usize),
            )
        }
    }

    #[doc = "Port B03 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb03pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb03PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb03PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(716usize),
            )
        }
    }

    #[doc = "Port B03 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb03pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb03PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb03PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(716usize),
            )
        }
    }

    #[doc = "Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2d0usize))
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

    #[doc = "Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2d0usize))
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

    #[doc = "Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2d0usize))
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

    #[doc = "Port B10 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb10pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(744usize),
            )
        }
    }

    #[doc = "Port B10 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb10pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(744usize),
            )
        }
    }

    #[doc = "Port B10 Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb10pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(744usize),
            )
        }
    }

    #[doc = "Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb1Pfs_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2f0usize))
        }
    }
    #[inline(always)]
    pub const fn pb12pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb13pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb14pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb15pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }

    #[doc = "Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb1PfsHa_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2f0usize))
        }
    }
    #[inline(always)]
    pub const fn pb12pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb13pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb14pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb15pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }

    #[doc = "Port B1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb1PfsBy_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2f0usize))
        }
    }
    #[inline(always)]
    pub const fn pb12pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb13pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb14pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pb15pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pb1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pb1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }

    #[doc = "Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
    #[inline(always)]
    pub const fn pc00pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc01pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc02pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc03pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc04pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc05pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc06pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc07pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc08pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc09pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }

    #[doc = "Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
    #[inline(always)]
    pub const fn pc00pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc01pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc02pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc03pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc04pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc05pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc06pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc07pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc08pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc09pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }

    #[doc = "Port C0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
    #[inline(always)]
    pub const fn pc00pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc01pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc02pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc03pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc04pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc05pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc06pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc07pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc08pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc09pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }

    #[doc = "Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pc1Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x328usize))
        }
    }
    #[inline(always)]
    pub const fn pc10pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc11pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc12pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc13pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc14pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc15pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }

    #[doc = "Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pc1PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x328usize))
        }
    }
    #[inline(always)]
    pub const fn pc10pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc11pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc12pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc13pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc14pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc15pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }

    #[doc = "Port C1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pc1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pc1PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x328usize))
        }
    }
    #[inline(always)]
    pub const fn pc10pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc11pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc12pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc13pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc14pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pc15pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pc1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pc1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }

    #[doc = "Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x340usize))
        }
    }
    #[inline(always)]
    pub const fn pd00pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd01pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd02pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd03pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd04pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd05pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd06pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd07pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd08pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd09pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }

    #[doc = "Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x340usize))
        }
    }
    #[inline(always)]
    pub const fn pd00pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd01pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd02pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd03pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd04pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd05pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd06pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd07pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd08pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd09pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }

    #[doc = "Port D0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x340usize))
        }
    }
    #[inline(always)]
    pub const fn pd00pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd01pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd02pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd03pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd04pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd05pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd06pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd07pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd08pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd09pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }

    #[doc = "Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pd1Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x368usize))
        }
    }
    #[inline(always)]
    pub const fn pd10pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd11pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd12pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd13pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd14pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd15pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }

    #[doc = "Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pd1PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x368usize))
        }
    }
    #[inline(always)]
    pub const fn pd10pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd11pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd12pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd13pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd14pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd15pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }

    #[doc = "Port D1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pd1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pd1PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x368usize))
        }
    }
    #[inline(always)]
    pub const fn pd10pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd11pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd12pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd13pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd14pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pd15pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pd1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pd1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }

    #[doc = "Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pe0Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3a0usize))
        }
    }
    #[inline(always)]
    pub const fn pe08pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe09pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe0Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe0Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }

    #[doc = "Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pe0PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3a0usize))
        }
    }
    #[inline(always)]
    pub const fn pe08pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe09pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe0PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe0PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }

    #[doc = "Port E0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pe0PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3a0usize))
        }
    }
    #[inline(always)]
    pub const fn pe08pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe09pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe0PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe0PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }

    #[doc = "Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pe1Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3a8usize))
        }
    }
    #[inline(always)]
    pub const fn pe10pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe11pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe12pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe13pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe14pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe15pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3bcusize),
            )
        }
    }

    #[doc = "Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pe1PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3a8usize))
        }
    }
    #[inline(always)]
    pub const fn pe10pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe11pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe12pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe13pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe14pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe15pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3bcusize),
            )
        }
    }

    #[doc = "Port E1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pe1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pe1PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3a8usize))
        }
    }
    #[inline(always)]
    pub const fn pe10pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe11pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe12pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe13pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe14pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pe15pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pe1PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pe1PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3bcusize),
            )
        }
    }

    #[doc = "Write-Protect Register"]
    #[inline(always)]
    pub const fn pwpr(&self) -> &'static crate::common::Reg<self::Pwpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1292usize),
            )
        }
    }

    #[doc = "Write-Protect Register for Secure"]
    #[inline(always)]
    pub const fn pwprs(&self) -> &'static crate::common::Reg<self::Pwprs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pwprs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[doc = "Port 0 Security Attribution register"]
    #[inline(always)]
    pub const fn p0sar(&self) -> &'static crate::common::Reg<self::P0Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1328usize),
            )
        }
    }

    #[doc = "Port 2 Security Attribution register"]
    #[inline(always)]
    pub const fn p2sar(&self) -> &'static crate::common::Reg<self::P2Sar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2Sar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1336usize),
            )
        }
    }

    #[doc = "Port A Security Attribution register"]
    #[inline(always)]
    pub const fn pasar(&self) -> &'static crate::common::Reg<self::Pasar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pasar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1368usize),
            )
        }
    }

    #[doc = "Port B Security Attribution register"]
    #[inline(always)]
    pub const fn pbsar(&self) -> &'static crate::common::Reg<self::Pbsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pbsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1372usize),
            )
        }
    }

    #[doc = "Port C Security Attribution register"]
    #[inline(always)]
    pub const fn pcsar(&self) -> &'static crate::common::Reg<self::Pcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1376usize),
            )
        }
    }

    #[doc = "Port D Security Attribution register"]
    #[inline(always)]
    pub const fn pdsar(&self) -> &'static crate::common::Reg<self::Pdsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1380usize),
            )
        }
    }

    #[doc = "Port E Security Attribution register"]
    #[inline(always)]
    pub const fn pesar(&self) -> &'static crate::common::Reg<self::Pesar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pesar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1384usize),
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

    #[doc = "Pmn State"]
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
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
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

    #[doc = "Pmn State"]
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
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
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

    #[doc = "Pmn State"]
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
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P002Pfs_SPEC;
impl crate::sealed::RegSpec for P002Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port 002 Pin Function Select Register"]
pub type P002Pfs = crate::RegValueT<P002Pfs_SPEC>;

impl P002Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p002pfs::Podr,
        p002pfs::Podr,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p002pfs::Podr,
            p002pfs::Podr,
            P002Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p002pfs::Pidr,
        p002pfs::Pidr,
        P002Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p002pfs::Pidr,
            p002pfs::Pidr,
            P002Pfs_SPEC,
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
        p002pfs::Pdr,
        p002pfs::Pdr,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p002pfs::Pdr,
            p002pfs::Pdr,
            P002Pfs_SPEC,
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
        p002pfs::Pcr,
        p002pfs::Pcr,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p002pfs::Pcr,
            p002pfs::Pcr,
            P002Pfs_SPEC,
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
        p002pfs::Ncodr,
        p002pfs::Ncodr,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p002pfs::Ncodr,
            p002pfs::Ncodr,
            P002Pfs_SPEC,
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
        p002pfs::Dscr,
        p002pfs::Dscr,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p002pfs::Dscr,
            p002pfs::Dscr,
            P002Pfs_SPEC,
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
        p002pfs::Isel,
        p002pfs::Isel,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p002pfs::Isel,
            p002pfs::Isel,
            P002Pfs_SPEC,
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
        p002pfs::Asel,
        p002pfs::Asel,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p002pfs::Asel,
            p002pfs::Asel,
            P002Pfs_SPEC,
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
        p002pfs::Pmr,
        p002pfs::Pmr,
        P002Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            p002pfs::Pmr,
            p002pfs::Pmr,
            P002Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, P002Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,P002Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P002Pfs {
    #[inline(always)]
    fn default() -> P002Pfs {
        <crate::RegValueT<P002Pfs_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p002pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P002PfsHa_SPEC;
impl crate::sealed::RegSpec for P002PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port 002 Pin Function Select Register"]
pub type P002PfsHa = crate::RegValueT<P002PfsHa_SPEC>;

impl P002PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p002pfs_ha::Podr,
        p002pfs_ha::Podr,
        P002PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p002pfs_ha::Podr,
            p002pfs_ha::Podr,
            P002PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p002pfs_ha::Pidr,
        p002pfs_ha::Pidr,
        P002PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p002pfs_ha::Pidr,
            p002pfs_ha::Pidr,
            P002PfsHa_SPEC,
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
        p002pfs_ha::Pdr,
        p002pfs_ha::Pdr,
        P002PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p002pfs_ha::Pdr,
            p002pfs_ha::Pdr,
            P002PfsHa_SPEC,
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
        p002pfs_ha::Pcr,
        p002pfs_ha::Pcr,
        P002PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p002pfs_ha::Pcr,
            p002pfs_ha::Pcr,
            P002PfsHa_SPEC,
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
        p002pfs_ha::Ncodr,
        p002pfs_ha::Ncodr,
        P002PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p002pfs_ha::Ncodr,
            p002pfs_ha::Ncodr,
            P002PfsHa_SPEC,
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
        p002pfs_ha::Dscr,
        p002pfs_ha::Dscr,
        P002PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p002pfs_ha::Dscr,
            p002pfs_ha::Dscr,
            P002PfsHa_SPEC,
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
        p002pfs_ha::Isel,
        p002pfs_ha::Isel,
        P002PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p002pfs_ha::Isel,
            p002pfs_ha::Isel,
            P002PfsHa_SPEC,
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
        p002pfs_ha::Asel,
        p002pfs_ha::Asel,
        P002PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p002pfs_ha::Asel,
            p002pfs_ha::Asel,
            P002PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P002PfsHa {
    #[inline(always)]
    fn default() -> P002PfsHa {
        <crate::RegValueT<P002PfsHa_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p002pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P002PfsBy_SPEC;
impl crate::sealed::RegSpec for P002PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port 002 Pin Function Select Register"]
pub type P002PfsBy = crate::RegValueT<P002PfsBy_SPEC>;

impl P002PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p002pfs_by::Podr,
        p002pfs_by::Podr,
        P002PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p002pfs_by::Podr,
            p002pfs_by::Podr,
            P002PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p002pfs_by::Pidr,
        p002pfs_by::Pidr,
        P002PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p002pfs_by::Pidr,
            p002pfs_by::Pidr,
            P002PfsBy_SPEC,
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
        p002pfs_by::Pdr,
        p002pfs_by::Pdr,
        P002PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p002pfs_by::Pdr,
            p002pfs_by::Pdr,
            P002PfsBy_SPEC,
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
        p002pfs_by::Pcr,
        p002pfs_by::Pcr,
        P002PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p002pfs_by::Pcr,
            p002pfs_by::Pcr,
            P002PfsBy_SPEC,
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
        p002pfs_by::Ncodr,
        p002pfs_by::Ncodr,
        P002PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p002pfs_by::Ncodr,
            p002pfs_by::Ncodr,
            P002PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P002PfsBy {
    #[inline(always)]
    fn default() -> P002PfsBy {
        <crate::RegValueT<P002PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p002pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

    #[doc = "Pmn State"]
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
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
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

    #[doc = "Pmn State"]
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
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
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

    #[doc = "Pmn State"]
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
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);

        #[doc = "High output"]
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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0Pfs_SPEC;
impl crate::sealed::RegSpec for Pa0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port A0%s Pin Function Select Register"]
pub type Pa0Pfs = crate::RegValueT<Pa0Pfs_SPEC>;

impl Pa0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa0pfs::Podr,
        pa0pfs::Podr,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa0pfs::Podr,
            pa0pfs::Podr,
            Pa0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa0pfs::Pidr,
        pa0pfs::Pidr,
        Pa0Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa0pfs::Pidr,
            pa0pfs::Pidr,
            Pa0Pfs_SPEC,
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
        pa0pfs::Pdr,
        pa0pfs::Pdr,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa0pfs::Pdr,
            pa0pfs::Pdr,
            Pa0Pfs_SPEC,
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
        pa0pfs::Pcr,
        pa0pfs::Pcr,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa0pfs::Pcr,
            pa0pfs::Pcr,
            Pa0Pfs_SPEC,
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
        pa0pfs::Ncodr,
        pa0pfs::Ncodr,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa0pfs::Ncodr,
            pa0pfs::Ncodr,
            Pa0Pfs_SPEC,
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
        pa0pfs::Dscr,
        pa0pfs::Dscr,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pa0pfs::Dscr,
            pa0pfs::Dscr,
            Pa0Pfs_SPEC,
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
        pa0pfs::Isel,
        pa0pfs::Isel,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pa0pfs::Isel,
            pa0pfs::Isel,
            Pa0Pfs_SPEC,
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
        pa0pfs::Asel,
        pa0pfs::Asel,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pa0pfs::Asel,
            pa0pfs::Asel,
            Pa0Pfs_SPEC,
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
        pa0pfs::Pmr,
        pa0pfs::Pmr,
        Pa0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pa0pfs::Pmr,
            pa0pfs::Pmr,
            Pa0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pa0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pa0Pfs {
    #[inline(always)]
    fn default() -> Pa0Pfs {
        <crate::RegValueT<Pa0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pa0pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pa0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port A0%s Pin Function Select Register"]
pub type Pa0PfsHa = crate::RegValueT<Pa0PfsHa_SPEC>;

impl Pa0PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa0pfs_ha::Podr,
        pa0pfs_ha::Podr,
        Pa0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa0pfs_ha::Podr,
            pa0pfs_ha::Podr,
            Pa0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa0pfs_ha::Pidr,
        pa0pfs_ha::Pidr,
        Pa0PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa0pfs_ha::Pidr,
            pa0pfs_ha::Pidr,
            Pa0PfsHa_SPEC,
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
        pa0pfs_ha::Pdr,
        pa0pfs_ha::Pdr,
        Pa0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa0pfs_ha::Pdr,
            pa0pfs_ha::Pdr,
            Pa0PfsHa_SPEC,
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
        pa0pfs_ha::Pcr,
        pa0pfs_ha::Pcr,
        Pa0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa0pfs_ha::Pcr,
            pa0pfs_ha::Pcr,
            Pa0PfsHa_SPEC,
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
        pa0pfs_ha::Ncodr,
        pa0pfs_ha::Ncodr,
        Pa0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa0pfs_ha::Ncodr,
            pa0pfs_ha::Ncodr,
            Pa0PfsHa_SPEC,
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
        pa0pfs_ha::Dscr,
        pa0pfs_ha::Dscr,
        Pa0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pa0pfs_ha::Dscr,
            pa0pfs_ha::Dscr,
            Pa0PfsHa_SPEC,
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
        pa0pfs_ha::Isel,
        pa0pfs_ha::Isel,
        Pa0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pa0pfs_ha::Isel,
            pa0pfs_ha::Isel,
            Pa0PfsHa_SPEC,
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
        pa0pfs_ha::Asel,
        pa0pfs_ha::Asel,
        Pa0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pa0pfs_ha::Asel,
            pa0pfs_ha::Asel,
            Pa0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pa0PfsHa {
    #[inline(always)]
    fn default() -> Pa0PfsHa {
        <crate::RegValueT<Pa0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pa0pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pa0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port A0%s Pin Function Select Register"]
pub type Pa0PfsBy = crate::RegValueT<Pa0PfsBy_SPEC>;

impl Pa0PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa0pfs_by::Podr,
        pa0pfs_by::Podr,
        Pa0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa0pfs_by::Podr,
            pa0pfs_by::Podr,
            Pa0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa0pfs_by::Pidr,
        pa0pfs_by::Pidr,
        Pa0PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa0pfs_by::Pidr,
            pa0pfs_by::Pidr,
            Pa0PfsBy_SPEC,
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
        pa0pfs_by::Pdr,
        pa0pfs_by::Pdr,
        Pa0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa0pfs_by::Pdr,
            pa0pfs_by::Pdr,
            Pa0PfsBy_SPEC,
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
        pa0pfs_by::Pcr,
        pa0pfs_by::Pcr,
        Pa0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa0pfs_by::Pcr,
            pa0pfs_by::Pcr,
            Pa0PfsBy_SPEC,
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
        pa0pfs_by::Ncodr,
        pa0pfs_by::Ncodr,
        Pa0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa0pfs_by::Ncodr,
            pa0pfs_by::Ncodr,
            Pa0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pa0PfsBy {
    #[inline(always)]
    fn default() -> Pa0PfsBy {
        <crate::RegValueT<Pa0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pa0pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa13Pfs_SPEC;
impl crate::sealed::RegSpec for Pa13Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port A13 Pin Function Select Register"]
pub type Pa13Pfs = crate::RegValueT<Pa13Pfs_SPEC>;

impl Pa13Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa13pfs::Podr,
        pa13pfs::Podr,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa13pfs::Podr,
            pa13pfs::Podr,
            Pa13Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa13pfs::Pidr,
        pa13pfs::Pidr,
        Pa13Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa13pfs::Pidr,
            pa13pfs::Pidr,
            Pa13Pfs_SPEC,
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
        pa13pfs::Pdr,
        pa13pfs::Pdr,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa13pfs::Pdr,
            pa13pfs::Pdr,
            Pa13Pfs_SPEC,
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
        pa13pfs::Pcr,
        pa13pfs::Pcr,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa13pfs::Pcr,
            pa13pfs::Pcr,
            Pa13Pfs_SPEC,
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
        pa13pfs::Ncodr,
        pa13pfs::Ncodr,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa13pfs::Ncodr,
            pa13pfs::Ncodr,
            Pa13Pfs_SPEC,
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
        pa13pfs::Dscr,
        pa13pfs::Dscr,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pa13pfs::Dscr,
            pa13pfs::Dscr,
            Pa13Pfs_SPEC,
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
        pa13pfs::Isel,
        pa13pfs::Isel,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pa13pfs::Isel,
            pa13pfs::Isel,
            Pa13Pfs_SPEC,
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
        pa13pfs::Asel,
        pa13pfs::Asel,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pa13pfs::Asel,
            pa13pfs::Asel,
            Pa13Pfs_SPEC,
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
        pa13pfs::Pmr,
        pa13pfs::Pmr,
        Pa13Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pa13pfs::Pmr,
            pa13pfs::Pmr,
            Pa13Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pa13Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pa13Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pa13Pfs {
    #[inline(always)]
    fn default() -> Pa13Pfs {
        <crate::RegValueT<Pa13Pfs_SPEC> as RegisterValue<_>>::new(66576)
    }
}
pub mod pa13pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa13PfsHa_SPEC;
impl crate::sealed::RegSpec for Pa13PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port A13 Pin Function Select Register"]
pub type Pa13PfsHa = crate::RegValueT<Pa13PfsHa_SPEC>;

impl Pa13PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa13pfs_ha::Podr,
        pa13pfs_ha::Podr,
        Pa13PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa13pfs_ha::Podr,
            pa13pfs_ha::Podr,
            Pa13PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa13pfs_ha::Pidr,
        pa13pfs_ha::Pidr,
        Pa13PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa13pfs_ha::Pidr,
            pa13pfs_ha::Pidr,
            Pa13PfsHa_SPEC,
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
        pa13pfs_ha::Pdr,
        pa13pfs_ha::Pdr,
        Pa13PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa13pfs_ha::Pdr,
            pa13pfs_ha::Pdr,
            Pa13PfsHa_SPEC,
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
        pa13pfs_ha::Pcr,
        pa13pfs_ha::Pcr,
        Pa13PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa13pfs_ha::Pcr,
            pa13pfs_ha::Pcr,
            Pa13PfsHa_SPEC,
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
        pa13pfs_ha::Ncodr,
        pa13pfs_ha::Ncodr,
        Pa13PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa13pfs_ha::Ncodr,
            pa13pfs_ha::Ncodr,
            Pa13PfsHa_SPEC,
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
        pa13pfs_ha::Dscr,
        pa13pfs_ha::Dscr,
        Pa13PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pa13pfs_ha::Dscr,
            pa13pfs_ha::Dscr,
            Pa13PfsHa_SPEC,
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
        pa13pfs_ha::Isel,
        pa13pfs_ha::Isel,
        Pa13PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pa13pfs_ha::Isel,
            pa13pfs_ha::Isel,
            Pa13PfsHa_SPEC,
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
        pa13pfs_ha::Asel,
        pa13pfs_ha::Asel,
        Pa13PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pa13pfs_ha::Asel,
            pa13pfs_ha::Asel,
            Pa13PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pa13PfsHa {
    #[inline(always)]
    fn default() -> Pa13PfsHa {
        <crate::RegValueT<Pa13PfsHa_SPEC> as RegisterValue<_>>::new(1040)
    }
}
pub mod pa13pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa13PfsBy_SPEC;
impl crate::sealed::RegSpec for Pa13PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port A13 Pin Function Select Register"]
pub type Pa13PfsBy = crate::RegValueT<Pa13PfsBy_SPEC>;

impl Pa13PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa13pfs_by::Podr,
        pa13pfs_by::Podr,
        Pa13PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa13pfs_by::Podr,
            pa13pfs_by::Podr,
            Pa13PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa13pfs_by::Pidr,
        pa13pfs_by::Pidr,
        Pa13PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa13pfs_by::Pidr,
            pa13pfs_by::Pidr,
            Pa13PfsBy_SPEC,
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
        pa13pfs_by::Pdr,
        pa13pfs_by::Pdr,
        Pa13PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa13pfs_by::Pdr,
            pa13pfs_by::Pdr,
            Pa13PfsBy_SPEC,
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
        pa13pfs_by::Pcr,
        pa13pfs_by::Pcr,
        Pa13PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa13pfs_by::Pcr,
            pa13pfs_by::Pcr,
            Pa13PfsBy_SPEC,
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
        pa13pfs_by::Ncodr,
        pa13pfs_by::Ncodr,
        Pa13PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa13pfs_by::Ncodr,
            pa13pfs_by::Ncodr,
            Pa13PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pa13PfsBy {
    #[inline(always)]
    fn default() -> Pa13PfsBy {
        <crate::RegValueT<Pa13PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod pa13pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa1Pfs_SPEC;
impl crate::sealed::RegSpec for Pa1Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port A1%s Pin Function Select Register"]
pub type Pa1Pfs = crate::RegValueT<Pa1Pfs_SPEC>;

impl Pa1Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa1pfs::Podr,
        pa1pfs::Podr,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa1pfs::Podr,
            pa1pfs::Podr,
            Pa1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa1pfs::Pidr,
        pa1pfs::Pidr,
        Pa1Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa1pfs::Pidr,
            pa1pfs::Pidr,
            Pa1Pfs_SPEC,
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
        pa1pfs::Pdr,
        pa1pfs::Pdr,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa1pfs::Pdr,
            pa1pfs::Pdr,
            Pa1Pfs_SPEC,
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
        pa1pfs::Pcr,
        pa1pfs::Pcr,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa1pfs::Pcr,
            pa1pfs::Pcr,
            Pa1Pfs_SPEC,
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
        pa1pfs::Ncodr,
        pa1pfs::Ncodr,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa1pfs::Ncodr,
            pa1pfs::Ncodr,
            Pa1Pfs_SPEC,
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
        pa1pfs::Dscr,
        pa1pfs::Dscr,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pa1pfs::Dscr,
            pa1pfs::Dscr,
            Pa1Pfs_SPEC,
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
        pa1pfs::Isel,
        pa1pfs::Isel,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pa1pfs::Isel,
            pa1pfs::Isel,
            Pa1Pfs_SPEC,
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
        pa1pfs::Asel,
        pa1pfs::Asel,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pa1pfs::Asel,
            pa1pfs::Asel,
            Pa1Pfs_SPEC,
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
        pa1pfs::Pmr,
        pa1pfs::Pmr,
        Pa1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pa1pfs::Pmr,
            pa1pfs::Pmr,
            Pa1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pa1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pa1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pa1Pfs {
    #[inline(always)]
    fn default() -> Pa1Pfs {
        <crate::RegValueT<Pa1Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}
pub mod pa1pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa1PfsHa_SPEC;
impl crate::sealed::RegSpec for Pa1PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port A1%s Pin Function Select Register"]
pub type Pa1PfsHa = crate::RegValueT<Pa1PfsHa_SPEC>;

impl Pa1PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa1pfs_ha::Podr,
        pa1pfs_ha::Podr,
        Pa1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa1pfs_ha::Podr,
            pa1pfs_ha::Podr,
            Pa1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa1pfs_ha::Pidr,
        pa1pfs_ha::Pidr,
        Pa1PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa1pfs_ha::Pidr,
            pa1pfs_ha::Pidr,
            Pa1PfsHa_SPEC,
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
        pa1pfs_ha::Pdr,
        pa1pfs_ha::Pdr,
        Pa1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa1pfs_ha::Pdr,
            pa1pfs_ha::Pdr,
            Pa1PfsHa_SPEC,
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
        pa1pfs_ha::Pcr,
        pa1pfs_ha::Pcr,
        Pa1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa1pfs_ha::Pcr,
            pa1pfs_ha::Pcr,
            Pa1PfsHa_SPEC,
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
        pa1pfs_ha::Ncodr,
        pa1pfs_ha::Ncodr,
        Pa1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa1pfs_ha::Ncodr,
            pa1pfs_ha::Ncodr,
            Pa1PfsHa_SPEC,
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
        pa1pfs_ha::Dscr,
        pa1pfs_ha::Dscr,
        Pa1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pa1pfs_ha::Dscr,
            pa1pfs_ha::Dscr,
            Pa1PfsHa_SPEC,
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
        pa1pfs_ha::Isel,
        pa1pfs_ha::Isel,
        Pa1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pa1pfs_ha::Isel,
            pa1pfs_ha::Isel,
            Pa1PfsHa_SPEC,
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
        pa1pfs_ha::Asel,
        pa1pfs_ha::Asel,
        Pa1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pa1pfs_ha::Asel,
            pa1pfs_ha::Asel,
            Pa1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pa1PfsHa {
    #[inline(always)]
    fn default() -> Pa1PfsHa {
        <crate::RegValueT<Pa1PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod pa1pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
        pub const _10: Self = Self::new(2);

        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa1PfsBy_SPEC;
impl crate::sealed::RegSpec for Pa1PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port A1%s Pin Function Select Register"]
pub type Pa1PfsBy = crate::RegValueT<Pa1PfsBy_SPEC>;

impl Pa1PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pa1pfs_by::Podr,
        pa1pfs_by::Podr,
        Pa1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pa1pfs_by::Podr,
            pa1pfs_by::Podr,
            Pa1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pa1pfs_by::Pidr,
        pa1pfs_by::Pidr,
        Pa1PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pa1pfs_by::Pidr,
            pa1pfs_by::Pidr,
            Pa1PfsBy_SPEC,
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
        pa1pfs_by::Pdr,
        pa1pfs_by::Pdr,
        Pa1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pa1pfs_by::Pdr,
            pa1pfs_by::Pdr,
            Pa1PfsBy_SPEC,
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
        pa1pfs_by::Pcr,
        pa1pfs_by::Pcr,
        Pa1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pa1pfs_by::Pcr,
            pa1pfs_by::Pcr,
            Pa1PfsBy_SPEC,
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
        pa1pfs_by::Ncodr,
        pa1pfs_by::Ncodr,
        Pa1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pa1pfs_by::Ncodr,
            pa1pfs_by::Ncodr,
            Pa1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pa1PfsBy {
    #[inline(always)]
    fn default() -> Pa1PfsBy {
        <crate::RegValueT<Pa1PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod pa1pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb02Pfs_SPEC;
impl crate::sealed::RegSpec for Pb02Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port B02 Pin Function Select Register"]
pub type Pb02Pfs = crate::RegValueT<Pb02Pfs_SPEC>;

impl Pb02Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb02pfs::Podr,
        pb02pfs::Podr,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb02pfs::Podr,
            pb02pfs::Podr,
            Pb02Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb02pfs::Pidr,
        pb02pfs::Pidr,
        Pb02Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb02pfs::Pidr,
            pb02pfs::Pidr,
            Pb02Pfs_SPEC,
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
        pb02pfs::Pdr,
        pb02pfs::Pdr,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb02pfs::Pdr,
            pb02pfs::Pdr,
            Pb02Pfs_SPEC,
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
        pb02pfs::Pcr,
        pb02pfs::Pcr,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb02pfs::Pcr,
            pb02pfs::Pcr,
            Pb02Pfs_SPEC,
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
        pb02pfs::Ncodr,
        pb02pfs::Ncodr,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb02pfs::Ncodr,
            pb02pfs::Ncodr,
            Pb02Pfs_SPEC,
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
        pb02pfs::Dscr,
        pb02pfs::Dscr,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb02pfs::Dscr,
            pb02pfs::Dscr,
            Pb02Pfs_SPEC,
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
        pb02pfs::Eofr,
        pb02pfs::Eofr,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb02pfs::Eofr,
            pb02pfs::Eofr,
            Pb02Pfs_SPEC,
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
        pb02pfs::Isel,
        pb02pfs::Isel,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb02pfs::Isel,
            pb02pfs::Isel,
            Pb02Pfs_SPEC,
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
        pb02pfs::Asel,
        pb02pfs::Asel,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb02pfs::Asel,
            pb02pfs::Asel,
            Pb02Pfs_SPEC,
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
        pb02pfs::Pmr,
        pb02pfs::Pmr,
        Pb02Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pb02pfs::Pmr,
            pb02pfs::Pmr,
            Pb02Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pb02Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pb02Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb02Pfs {
    #[inline(always)]
    fn default() -> Pb02Pfs {
        <crate::RegValueT<Pb02Pfs_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod pb02pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb02PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb02PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port B02 Pin Function Select Register"]
pub type Pb02PfsHa = crate::RegValueT<Pb02PfsHa_SPEC>;

impl Pb02PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb02pfs_ha::Podr,
        pb02pfs_ha::Podr,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb02pfs_ha::Podr,
            pb02pfs_ha::Podr,
            Pb02PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb02pfs_ha::Pidr,
        pb02pfs_ha::Pidr,
        Pb02PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb02pfs_ha::Pidr,
            pb02pfs_ha::Pidr,
            Pb02PfsHa_SPEC,
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
        pb02pfs_ha::Pdr,
        pb02pfs_ha::Pdr,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb02pfs_ha::Pdr,
            pb02pfs_ha::Pdr,
            Pb02PfsHa_SPEC,
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
        pb02pfs_ha::Pcr,
        pb02pfs_ha::Pcr,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb02pfs_ha::Pcr,
            pb02pfs_ha::Pcr,
            Pb02PfsHa_SPEC,
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
        pb02pfs_ha::Ncodr,
        pb02pfs_ha::Ncodr,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb02pfs_ha::Ncodr,
            pb02pfs_ha::Ncodr,
            Pb02PfsHa_SPEC,
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
        pb02pfs_ha::Dscr,
        pb02pfs_ha::Dscr,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb02pfs_ha::Dscr,
            pb02pfs_ha::Dscr,
            Pb02PfsHa_SPEC,
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
        pb02pfs_ha::Eofr,
        pb02pfs_ha::Eofr,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb02pfs_ha::Eofr,
            pb02pfs_ha::Eofr,
            Pb02PfsHa_SPEC,
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
        pb02pfs_ha::Isel,
        pb02pfs_ha::Isel,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb02pfs_ha::Isel,
            pb02pfs_ha::Isel,
            Pb02PfsHa_SPEC,
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
        pb02pfs_ha::Asel,
        pb02pfs_ha::Asel,
        Pb02PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb02pfs_ha::Asel,
            pb02pfs_ha::Asel,
            Pb02PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb02PfsHa {
    #[inline(always)]
    fn default() -> Pb02PfsHa {
        <crate::RegValueT<Pb02PfsHa_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod pb02pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb02PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb02PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port B02 Pin Function Select Register"]
pub type Pb02PfsBy = crate::RegValueT<Pb02PfsBy_SPEC>;

impl Pb02PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb02pfs_by::Podr,
        pb02pfs_by::Podr,
        Pb02PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb02pfs_by::Podr,
            pb02pfs_by::Podr,
            Pb02PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb02pfs_by::Pidr,
        pb02pfs_by::Pidr,
        Pb02PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb02pfs_by::Pidr,
            pb02pfs_by::Pidr,
            Pb02PfsBy_SPEC,
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
        pb02pfs_by::Pdr,
        pb02pfs_by::Pdr,
        Pb02PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb02pfs_by::Pdr,
            pb02pfs_by::Pdr,
            Pb02PfsBy_SPEC,
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
        pb02pfs_by::Pcr,
        pb02pfs_by::Pcr,
        Pb02PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb02pfs_by::Pcr,
            pb02pfs_by::Pcr,
            Pb02PfsBy_SPEC,
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
        pb02pfs_by::Ncodr,
        pb02pfs_by::Ncodr,
        Pb02PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb02pfs_by::Ncodr,
            pb02pfs_by::Ncodr,
            Pb02PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb02PfsBy {
    #[inline(always)]
    fn default() -> Pb02PfsBy {
        <crate::RegValueT<Pb02PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb02pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb03Pfs_SPEC;
impl crate::sealed::RegSpec for Pb03Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port B03 Pin Function Select Register"]
pub type Pb03Pfs = crate::RegValueT<Pb03Pfs_SPEC>;

impl Pb03Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb03pfs::Podr,
        pb03pfs::Podr,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb03pfs::Podr,
            pb03pfs::Podr,
            Pb03Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb03pfs::Pidr,
        pb03pfs::Pidr,
        Pb03Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb03pfs::Pidr,
            pb03pfs::Pidr,
            Pb03Pfs_SPEC,
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
        pb03pfs::Pdr,
        pb03pfs::Pdr,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb03pfs::Pdr,
            pb03pfs::Pdr,
            Pb03Pfs_SPEC,
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
        pb03pfs::Pcr,
        pb03pfs::Pcr,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb03pfs::Pcr,
            pb03pfs::Pcr,
            Pb03Pfs_SPEC,
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
        pb03pfs::Ncodr,
        pb03pfs::Ncodr,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb03pfs::Ncodr,
            pb03pfs::Ncodr,
            Pb03Pfs_SPEC,
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
        pb03pfs::Dscr,
        pb03pfs::Dscr,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb03pfs::Dscr,
            pb03pfs::Dscr,
            Pb03Pfs_SPEC,
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
        pb03pfs::Eofr,
        pb03pfs::Eofr,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb03pfs::Eofr,
            pb03pfs::Eofr,
            Pb03Pfs_SPEC,
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
        pb03pfs::Isel,
        pb03pfs::Isel,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb03pfs::Isel,
            pb03pfs::Isel,
            Pb03Pfs_SPEC,
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
        pb03pfs::Asel,
        pb03pfs::Asel,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb03pfs::Asel,
            pb03pfs::Asel,
            Pb03Pfs_SPEC,
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
        pb03pfs::Pmr,
        pb03pfs::Pmr,
        Pb03Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pb03pfs::Pmr,
            pb03pfs::Pmr,
            Pb03Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pb03Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pb03Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb03Pfs {
    #[inline(always)]
    fn default() -> Pb03Pfs {
        <crate::RegValueT<Pb03Pfs_SPEC> as RegisterValue<_>>::new(66560)
    }
}
pub mod pb03pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb03PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb03PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port B03 Pin Function Select Register"]
pub type Pb03PfsHa = crate::RegValueT<Pb03PfsHa_SPEC>;

impl Pb03PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb03pfs_ha::Podr,
        pb03pfs_ha::Podr,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb03pfs_ha::Podr,
            pb03pfs_ha::Podr,
            Pb03PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb03pfs_ha::Pidr,
        pb03pfs_ha::Pidr,
        Pb03PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb03pfs_ha::Pidr,
            pb03pfs_ha::Pidr,
            Pb03PfsHa_SPEC,
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
        pb03pfs_ha::Pdr,
        pb03pfs_ha::Pdr,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb03pfs_ha::Pdr,
            pb03pfs_ha::Pdr,
            Pb03PfsHa_SPEC,
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
        pb03pfs_ha::Pcr,
        pb03pfs_ha::Pcr,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb03pfs_ha::Pcr,
            pb03pfs_ha::Pcr,
            Pb03PfsHa_SPEC,
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
        pb03pfs_ha::Ncodr,
        pb03pfs_ha::Ncodr,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb03pfs_ha::Ncodr,
            pb03pfs_ha::Ncodr,
            Pb03PfsHa_SPEC,
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
        pb03pfs_ha::Dscr,
        pb03pfs_ha::Dscr,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb03pfs_ha::Dscr,
            pb03pfs_ha::Dscr,
            Pb03PfsHa_SPEC,
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
        pb03pfs_ha::Eofr,
        pb03pfs_ha::Eofr,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb03pfs_ha::Eofr,
            pb03pfs_ha::Eofr,
            Pb03PfsHa_SPEC,
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
        pb03pfs_ha::Isel,
        pb03pfs_ha::Isel,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb03pfs_ha::Isel,
            pb03pfs_ha::Isel,
            Pb03PfsHa_SPEC,
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
        pb03pfs_ha::Asel,
        pb03pfs_ha::Asel,
        Pb03PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb03pfs_ha::Asel,
            pb03pfs_ha::Asel,
            Pb03PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb03PfsHa {
    #[inline(always)]
    fn default() -> Pb03PfsHa {
        <crate::RegValueT<Pb03PfsHa_SPEC> as RegisterValue<_>>::new(1024)
    }
}
pub mod pb03pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb03PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb03PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port B03 Pin Function Select Register"]
pub type Pb03PfsBy = crate::RegValueT<Pb03PfsBy_SPEC>;

impl Pb03PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb03pfs_by::Podr,
        pb03pfs_by::Podr,
        Pb03PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb03pfs_by::Podr,
            pb03pfs_by::Podr,
            Pb03PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb03pfs_by::Pidr,
        pb03pfs_by::Pidr,
        Pb03PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb03pfs_by::Pidr,
            pb03pfs_by::Pidr,
            Pb03PfsBy_SPEC,
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
        pb03pfs_by::Pdr,
        pb03pfs_by::Pdr,
        Pb03PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb03pfs_by::Pdr,
            pb03pfs_by::Pdr,
            Pb03PfsBy_SPEC,
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
        pb03pfs_by::Pcr,
        pb03pfs_by::Pcr,
        Pb03PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb03pfs_by::Pcr,
            pb03pfs_by::Pcr,
            Pb03PfsBy_SPEC,
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
        pb03pfs_by::Ncodr,
        pb03pfs_by::Ncodr,
        Pb03PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb03pfs_by::Ncodr,
            pb03pfs_by::Ncodr,
            Pb03PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb03PfsBy {
    #[inline(always)]
    fn default() -> Pb03PfsBy {
        <crate::RegValueT<Pb03PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb03pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0Pfs_SPEC;
impl crate::sealed::RegSpec for Pb0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port B0%s Pin Function Select Register"]
pub type Pb0Pfs = crate::RegValueT<Pb0Pfs_SPEC>;

impl Pb0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb0pfs::Podr,
        pb0pfs::Podr,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb0pfs::Podr,
            pb0pfs::Podr,
            Pb0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb0pfs::Pidr,
        pb0pfs::Pidr,
        Pb0Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb0pfs::Pidr,
            pb0pfs::Pidr,
            Pb0Pfs_SPEC,
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
        pb0pfs::Pdr,
        pb0pfs::Pdr,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb0pfs::Pdr,
            pb0pfs::Pdr,
            Pb0Pfs_SPEC,
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
        pb0pfs::Pcr,
        pb0pfs::Pcr,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb0pfs::Pcr,
            pb0pfs::Pcr,
            Pb0Pfs_SPEC,
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
        pb0pfs::Ncodr,
        pb0pfs::Ncodr,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb0pfs::Ncodr,
            pb0pfs::Ncodr,
            Pb0Pfs_SPEC,
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
        pb0pfs::Dscr,
        pb0pfs::Dscr,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb0pfs::Dscr,
            pb0pfs::Dscr,
            Pb0Pfs_SPEC,
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
        pb0pfs::Eofr,
        pb0pfs::Eofr,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb0pfs::Eofr,
            pb0pfs::Eofr,
            Pb0Pfs_SPEC,
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
        pb0pfs::Isel,
        pb0pfs::Isel,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb0pfs::Isel,
            pb0pfs::Isel,
            Pb0Pfs_SPEC,
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
        pb0pfs::Asel,
        pb0pfs::Asel,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb0pfs::Asel,
            pb0pfs::Asel,
            Pb0Pfs_SPEC,
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
        pb0pfs::Pmr,
        pb0pfs::Pmr,
        Pb0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pb0pfs::Pmr,
            pb0pfs::Pmr,
            Pb0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pb0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb0Pfs {
    #[inline(always)]
    fn default() -> Pb0Pfs {
        <crate::RegValueT<Pb0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb0pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port B0%s Pin Function Select Register"]
pub type Pb0PfsHa = crate::RegValueT<Pb0PfsHa_SPEC>;

impl Pb0PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb0pfs_ha::Podr,
        pb0pfs_ha::Podr,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb0pfs_ha::Podr,
            pb0pfs_ha::Podr,
            Pb0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb0pfs_ha::Pidr,
        pb0pfs_ha::Pidr,
        Pb0PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb0pfs_ha::Pidr,
            pb0pfs_ha::Pidr,
            Pb0PfsHa_SPEC,
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
        pb0pfs_ha::Pdr,
        pb0pfs_ha::Pdr,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb0pfs_ha::Pdr,
            pb0pfs_ha::Pdr,
            Pb0PfsHa_SPEC,
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
        pb0pfs_ha::Pcr,
        pb0pfs_ha::Pcr,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb0pfs_ha::Pcr,
            pb0pfs_ha::Pcr,
            Pb0PfsHa_SPEC,
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
        pb0pfs_ha::Ncodr,
        pb0pfs_ha::Ncodr,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb0pfs_ha::Ncodr,
            pb0pfs_ha::Ncodr,
            Pb0PfsHa_SPEC,
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
        pb0pfs_ha::Dscr,
        pb0pfs_ha::Dscr,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb0pfs_ha::Dscr,
            pb0pfs_ha::Dscr,
            Pb0PfsHa_SPEC,
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
        pb0pfs_ha::Eofr,
        pb0pfs_ha::Eofr,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb0pfs_ha::Eofr,
            pb0pfs_ha::Eofr,
            Pb0PfsHa_SPEC,
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
        pb0pfs_ha::Isel,
        pb0pfs_ha::Isel,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb0pfs_ha::Isel,
            pb0pfs_ha::Isel,
            Pb0PfsHa_SPEC,
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
        pb0pfs_ha::Asel,
        pb0pfs_ha::Asel,
        Pb0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb0pfs_ha::Asel,
            pb0pfs_ha::Asel,
            Pb0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb0PfsHa {
    #[inline(always)]
    fn default() -> Pb0PfsHa {
        <crate::RegValueT<Pb0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb0pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port B0%s Pin Function Select Register"]
pub type Pb0PfsBy = crate::RegValueT<Pb0PfsBy_SPEC>;

impl Pb0PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb0pfs_by::Podr,
        pb0pfs_by::Podr,
        Pb0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb0pfs_by::Podr,
            pb0pfs_by::Podr,
            Pb0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb0pfs_by::Pidr,
        pb0pfs_by::Pidr,
        Pb0PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb0pfs_by::Pidr,
            pb0pfs_by::Pidr,
            Pb0PfsBy_SPEC,
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
        pb0pfs_by::Pdr,
        pb0pfs_by::Pdr,
        Pb0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb0pfs_by::Pdr,
            pb0pfs_by::Pdr,
            Pb0PfsBy_SPEC,
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
        pb0pfs_by::Pcr,
        pb0pfs_by::Pcr,
        Pb0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb0pfs_by::Pcr,
            pb0pfs_by::Pcr,
            Pb0PfsBy_SPEC,
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
        pb0pfs_by::Ncodr,
        pb0pfs_by::Ncodr,
        Pb0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb0pfs_by::Ncodr,
            pb0pfs_by::Ncodr,
            Pb0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb0PfsBy {
    #[inline(always)]
    fn default() -> Pb0PfsBy {
        <crate::RegValueT<Pb0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb0pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb10Pfs_SPEC;
impl crate::sealed::RegSpec for Pb10Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port B10 Pin Function Select Register"]
pub type Pb10Pfs = crate::RegValueT<Pb10Pfs_SPEC>;

impl Pb10Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb10pfs::Podr,
        pb10pfs::Podr,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb10pfs::Podr,
            pb10pfs::Podr,
            Pb10Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb10pfs::Pidr,
        pb10pfs::Pidr,
        Pb10Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb10pfs::Pidr,
            pb10pfs::Pidr,
            Pb10Pfs_SPEC,
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
        pb10pfs::Pdr,
        pb10pfs::Pdr,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb10pfs::Pdr,
            pb10pfs::Pdr,
            Pb10Pfs_SPEC,
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
        pb10pfs::Pcr,
        pb10pfs::Pcr,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb10pfs::Pcr,
            pb10pfs::Pcr,
            Pb10Pfs_SPEC,
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
        pb10pfs::Ncodr,
        pb10pfs::Ncodr,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb10pfs::Ncodr,
            pb10pfs::Ncodr,
            Pb10Pfs_SPEC,
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
        pb10pfs::Dscr,
        pb10pfs::Dscr,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb10pfs::Dscr,
            pb10pfs::Dscr,
            Pb10Pfs_SPEC,
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
        pb10pfs::Eofr,
        pb10pfs::Eofr,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb10pfs::Eofr,
            pb10pfs::Eofr,
            Pb10Pfs_SPEC,
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
        pb10pfs::Isel,
        pb10pfs::Isel,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb10pfs::Isel,
            pb10pfs::Isel,
            Pb10Pfs_SPEC,
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
        pb10pfs::Asel,
        pb10pfs::Asel,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb10pfs::Asel,
            pb10pfs::Asel,
            Pb10Pfs_SPEC,
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
        pb10pfs::Pmr,
        pb10pfs::Pmr,
        Pb10Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pb10pfs::Pmr,
            pb10pfs::Pmr,
            Pb10Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pb10Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pb10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb10Pfs {
    #[inline(always)]
    fn default() -> Pb10Pfs {
        <crate::RegValueT<Pb10Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb10pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb10PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb10PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port B10 Pin Function Select Register"]
pub type Pb10PfsHa = crate::RegValueT<Pb10PfsHa_SPEC>;

impl Pb10PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb10pfs_ha::Podr,
        pb10pfs_ha::Podr,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb10pfs_ha::Podr,
            pb10pfs_ha::Podr,
            Pb10PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb10pfs_ha::Pidr,
        pb10pfs_ha::Pidr,
        Pb10PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb10pfs_ha::Pidr,
            pb10pfs_ha::Pidr,
            Pb10PfsHa_SPEC,
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
        pb10pfs_ha::Pdr,
        pb10pfs_ha::Pdr,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb10pfs_ha::Pdr,
            pb10pfs_ha::Pdr,
            Pb10PfsHa_SPEC,
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
        pb10pfs_ha::Pcr,
        pb10pfs_ha::Pcr,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb10pfs_ha::Pcr,
            pb10pfs_ha::Pcr,
            Pb10PfsHa_SPEC,
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
        pb10pfs_ha::Ncodr,
        pb10pfs_ha::Ncodr,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb10pfs_ha::Ncodr,
            pb10pfs_ha::Ncodr,
            Pb10PfsHa_SPEC,
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
        pb10pfs_ha::Dscr,
        pb10pfs_ha::Dscr,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb10pfs_ha::Dscr,
            pb10pfs_ha::Dscr,
            Pb10PfsHa_SPEC,
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
        pb10pfs_ha::Eofr,
        pb10pfs_ha::Eofr,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb10pfs_ha::Eofr,
            pb10pfs_ha::Eofr,
            Pb10PfsHa_SPEC,
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
        pb10pfs_ha::Isel,
        pb10pfs_ha::Isel,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb10pfs_ha::Isel,
            pb10pfs_ha::Isel,
            Pb10PfsHa_SPEC,
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
        pb10pfs_ha::Asel,
        pb10pfs_ha::Asel,
        Pb10PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb10pfs_ha::Asel,
            pb10pfs_ha::Asel,
            Pb10PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb10PfsHa {
    #[inline(always)]
    fn default() -> Pb10PfsHa {
        <crate::RegValueT<Pb10PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb10pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb10PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb10PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port B10 Pin Function Select Register"]
pub type Pb10PfsBy = crate::RegValueT<Pb10PfsBy_SPEC>;

impl Pb10PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb10pfs_by::Podr,
        pb10pfs_by::Podr,
        Pb10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb10pfs_by::Podr,
            pb10pfs_by::Podr,
            Pb10PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb10pfs_by::Pidr,
        pb10pfs_by::Pidr,
        Pb10PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb10pfs_by::Pidr,
            pb10pfs_by::Pidr,
            Pb10PfsBy_SPEC,
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
        pb10pfs_by::Pdr,
        pb10pfs_by::Pdr,
        Pb10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb10pfs_by::Pdr,
            pb10pfs_by::Pdr,
            Pb10PfsBy_SPEC,
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
        pb10pfs_by::Pcr,
        pb10pfs_by::Pcr,
        Pb10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb10pfs_by::Pcr,
            pb10pfs_by::Pcr,
            Pb10PfsBy_SPEC,
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
        pb10pfs_by::Ncodr,
        pb10pfs_by::Ncodr,
        Pb10PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb10pfs_by::Ncodr,
            pb10pfs_by::Ncodr,
            Pb10PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb10PfsBy {
    #[inline(always)]
    fn default() -> Pb10PfsBy {
        <crate::RegValueT<Pb10PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb10pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb1Pfs_SPEC;
impl crate::sealed::RegSpec for Pb1Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port B1%s Pin Function Select Register"]
pub type Pb1Pfs = crate::RegValueT<Pb1Pfs_SPEC>;

impl Pb1Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb1pfs::Podr,
        pb1pfs::Podr,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb1pfs::Podr,
            pb1pfs::Podr,
            Pb1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb1pfs::Pidr,
        pb1pfs::Pidr,
        Pb1Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb1pfs::Pidr,
            pb1pfs::Pidr,
            Pb1Pfs_SPEC,
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
        pb1pfs::Pdr,
        pb1pfs::Pdr,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb1pfs::Pdr,
            pb1pfs::Pdr,
            Pb1Pfs_SPEC,
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
        pb1pfs::Pcr,
        pb1pfs::Pcr,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb1pfs::Pcr,
            pb1pfs::Pcr,
            Pb1Pfs_SPEC,
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
        pb1pfs::Ncodr,
        pb1pfs::Ncodr,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb1pfs::Ncodr,
            pb1pfs::Ncodr,
            Pb1Pfs_SPEC,
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
        pb1pfs::Dscr,
        pb1pfs::Dscr,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb1pfs::Dscr,
            pb1pfs::Dscr,
            Pb1Pfs_SPEC,
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
        pb1pfs::Eofr,
        pb1pfs::Eofr,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb1pfs::Eofr,
            pb1pfs::Eofr,
            Pb1Pfs_SPEC,
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
        pb1pfs::Isel,
        pb1pfs::Isel,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb1pfs::Isel,
            pb1pfs::Isel,
            Pb1Pfs_SPEC,
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
        pb1pfs::Asel,
        pb1pfs::Asel,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb1pfs::Asel,
            pb1pfs::Asel,
            Pb1Pfs_SPEC,
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
        pb1pfs::Pmr,
        pb1pfs::Pmr,
        Pb1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pb1pfs::Pmr,
            pb1pfs::Pmr,
            Pb1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pb1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pb1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb1Pfs {
    #[inline(always)]
    fn default() -> Pb1Pfs {
        <crate::RegValueT<Pb1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb1pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb1PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb1PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port B1%s Pin Function Select Register"]
pub type Pb1PfsHa = crate::RegValueT<Pb1PfsHa_SPEC>;

impl Pb1PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb1pfs_ha::Podr,
        pb1pfs_ha::Podr,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb1pfs_ha::Podr,
            pb1pfs_ha::Podr,
            Pb1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb1pfs_ha::Pidr,
        pb1pfs_ha::Pidr,
        Pb1PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb1pfs_ha::Pidr,
            pb1pfs_ha::Pidr,
            Pb1PfsHa_SPEC,
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
        pb1pfs_ha::Pdr,
        pb1pfs_ha::Pdr,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb1pfs_ha::Pdr,
            pb1pfs_ha::Pdr,
            Pb1PfsHa_SPEC,
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
        pb1pfs_ha::Pcr,
        pb1pfs_ha::Pcr,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb1pfs_ha::Pcr,
            pb1pfs_ha::Pcr,
            Pb1PfsHa_SPEC,
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
        pb1pfs_ha::Ncodr,
        pb1pfs_ha::Ncodr,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb1pfs_ha::Ncodr,
            pb1pfs_ha::Ncodr,
            Pb1PfsHa_SPEC,
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
        pb1pfs_ha::Dscr,
        pb1pfs_ha::Dscr,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pb1pfs_ha::Dscr,
            pb1pfs_ha::Dscr,
            Pb1PfsHa_SPEC,
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
        pb1pfs_ha::Eofr,
        pb1pfs_ha::Eofr,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pb1pfs_ha::Eofr,
            pb1pfs_ha::Eofr,
            Pb1PfsHa_SPEC,
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
        pb1pfs_ha::Isel,
        pb1pfs_ha::Isel,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pb1pfs_ha::Isel,
            pb1pfs_ha::Isel,
            Pb1PfsHa_SPEC,
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
        pb1pfs_ha::Asel,
        pb1pfs_ha::Asel,
        Pb1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pb1pfs_ha::Asel,
            pb1pfs_ha::Asel,
            Pb1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb1PfsHa {
    #[inline(always)]
    fn default() -> Pb1PfsHa {
        <crate::RegValueT<Pb1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb1pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb1PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb1PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port B1%s Pin Function Select Register"]
pub type Pb1PfsBy = crate::RegValueT<Pb1PfsBy_SPEC>;

impl Pb1PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pb1pfs_by::Podr,
        pb1pfs_by::Podr,
        Pb1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pb1pfs_by::Podr,
            pb1pfs_by::Podr,
            Pb1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pb1pfs_by::Pidr,
        pb1pfs_by::Pidr,
        Pb1PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pb1pfs_by::Pidr,
            pb1pfs_by::Pidr,
            Pb1PfsBy_SPEC,
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
        pb1pfs_by::Pdr,
        pb1pfs_by::Pdr,
        Pb1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pb1pfs_by::Pdr,
            pb1pfs_by::Pdr,
            Pb1PfsBy_SPEC,
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
        pb1pfs_by::Pcr,
        pb1pfs_by::Pcr,
        Pb1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pb1pfs_by::Pcr,
            pb1pfs_by::Pcr,
            Pb1PfsBy_SPEC,
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
        pb1pfs_by::Ncodr,
        pb1pfs_by::Ncodr,
        Pb1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pb1pfs_by::Ncodr,
            pb1pfs_by::Ncodr,
            Pb1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pb1PfsBy {
    #[inline(always)]
    fn default() -> Pb1PfsBy {
        <crate::RegValueT<Pb1PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb1pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc0Pfs_SPEC;
impl crate::sealed::RegSpec for Pc0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port C0%s Pin Function Select Register"]
pub type Pc0Pfs = crate::RegValueT<Pc0Pfs_SPEC>;

impl Pc0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pc0pfs::Podr,
        pc0pfs::Podr,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pc0pfs::Podr,
            pc0pfs::Podr,
            Pc0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pc0pfs::Pidr,
        pc0pfs::Pidr,
        Pc0Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pc0pfs::Pidr,
            pc0pfs::Pidr,
            Pc0Pfs_SPEC,
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
        pc0pfs::Pdr,
        pc0pfs::Pdr,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pc0pfs::Pdr,
            pc0pfs::Pdr,
            Pc0Pfs_SPEC,
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
        pc0pfs::Pcr,
        pc0pfs::Pcr,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pc0pfs::Pcr,
            pc0pfs::Pcr,
            Pc0Pfs_SPEC,
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
        pc0pfs::Ncodr,
        pc0pfs::Ncodr,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pc0pfs::Ncodr,
            pc0pfs::Ncodr,
            Pc0Pfs_SPEC,
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
        pc0pfs::Dscr,
        pc0pfs::Dscr,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pc0pfs::Dscr,
            pc0pfs::Dscr,
            Pc0Pfs_SPEC,
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
        pc0pfs::Eofr,
        pc0pfs::Eofr,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pc0pfs::Eofr,
            pc0pfs::Eofr,
            Pc0Pfs_SPEC,
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
        pc0pfs::Isel,
        pc0pfs::Isel,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pc0pfs::Isel,
            pc0pfs::Isel,
            Pc0Pfs_SPEC,
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
        pc0pfs::Asel,
        pc0pfs::Asel,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pc0pfs::Asel,
            pc0pfs::Asel,
            Pc0Pfs_SPEC,
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
        pc0pfs::Pmr,
        pc0pfs::Pmr,
        Pc0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pc0pfs::Pmr,
            pc0pfs::Pmr,
            Pc0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pc0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pc0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pc0Pfs {
    #[inline(always)]
    fn default() -> Pc0Pfs {
        <crate::RegValueT<Pc0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pc0pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pc0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port C0%s Pin Function Select Register"]
pub type Pc0PfsHa = crate::RegValueT<Pc0PfsHa_SPEC>;

impl Pc0PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pc0pfs_ha::Podr,
        pc0pfs_ha::Podr,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pc0pfs_ha::Podr,
            pc0pfs_ha::Podr,
            Pc0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pc0pfs_ha::Pidr,
        pc0pfs_ha::Pidr,
        Pc0PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pc0pfs_ha::Pidr,
            pc0pfs_ha::Pidr,
            Pc0PfsHa_SPEC,
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
        pc0pfs_ha::Pdr,
        pc0pfs_ha::Pdr,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pc0pfs_ha::Pdr,
            pc0pfs_ha::Pdr,
            Pc0PfsHa_SPEC,
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
        pc0pfs_ha::Pcr,
        pc0pfs_ha::Pcr,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pc0pfs_ha::Pcr,
            pc0pfs_ha::Pcr,
            Pc0PfsHa_SPEC,
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
        pc0pfs_ha::Ncodr,
        pc0pfs_ha::Ncodr,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pc0pfs_ha::Ncodr,
            pc0pfs_ha::Ncodr,
            Pc0PfsHa_SPEC,
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
        pc0pfs_ha::Dscr,
        pc0pfs_ha::Dscr,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pc0pfs_ha::Dscr,
            pc0pfs_ha::Dscr,
            Pc0PfsHa_SPEC,
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
        pc0pfs_ha::Eofr,
        pc0pfs_ha::Eofr,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pc0pfs_ha::Eofr,
            pc0pfs_ha::Eofr,
            Pc0PfsHa_SPEC,
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
        pc0pfs_ha::Isel,
        pc0pfs_ha::Isel,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pc0pfs_ha::Isel,
            pc0pfs_ha::Isel,
            Pc0PfsHa_SPEC,
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
        pc0pfs_ha::Asel,
        pc0pfs_ha::Asel,
        Pc0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pc0pfs_ha::Asel,
            pc0pfs_ha::Asel,
            Pc0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pc0PfsHa {
    #[inline(always)]
    fn default() -> Pc0PfsHa {
        <crate::RegValueT<Pc0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pc0pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pc0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port C0%s Pin Function Select Register"]
pub type Pc0PfsBy = crate::RegValueT<Pc0PfsBy_SPEC>;

impl Pc0PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pc0pfs_by::Podr,
        pc0pfs_by::Podr,
        Pc0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pc0pfs_by::Podr,
            pc0pfs_by::Podr,
            Pc0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pc0pfs_by::Pidr,
        pc0pfs_by::Pidr,
        Pc0PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pc0pfs_by::Pidr,
            pc0pfs_by::Pidr,
            Pc0PfsBy_SPEC,
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
        pc0pfs_by::Pdr,
        pc0pfs_by::Pdr,
        Pc0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pc0pfs_by::Pdr,
            pc0pfs_by::Pdr,
            Pc0PfsBy_SPEC,
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
        pc0pfs_by::Pcr,
        pc0pfs_by::Pcr,
        Pc0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pc0pfs_by::Pcr,
            pc0pfs_by::Pcr,
            Pc0PfsBy_SPEC,
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
        pc0pfs_by::Ncodr,
        pc0pfs_by::Ncodr,
        Pc0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pc0pfs_by::Ncodr,
            pc0pfs_by::Ncodr,
            Pc0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pc0PfsBy {
    #[inline(always)]
    fn default() -> Pc0PfsBy {
        <crate::RegValueT<Pc0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pc0pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc1Pfs_SPEC;
impl crate::sealed::RegSpec for Pc1Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port C1%s Pin Function Select Register"]
pub type Pc1Pfs = crate::RegValueT<Pc1Pfs_SPEC>;

impl Pc1Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pc1pfs::Podr,
        pc1pfs::Podr,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pc1pfs::Podr,
            pc1pfs::Podr,
            Pc1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pc1pfs::Pidr,
        pc1pfs::Pidr,
        Pc1Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pc1pfs::Pidr,
            pc1pfs::Pidr,
            Pc1Pfs_SPEC,
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
        pc1pfs::Pdr,
        pc1pfs::Pdr,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pc1pfs::Pdr,
            pc1pfs::Pdr,
            Pc1Pfs_SPEC,
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
        pc1pfs::Pcr,
        pc1pfs::Pcr,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pc1pfs::Pcr,
            pc1pfs::Pcr,
            Pc1Pfs_SPEC,
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
        pc1pfs::Ncodr,
        pc1pfs::Ncodr,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pc1pfs::Ncodr,
            pc1pfs::Ncodr,
            Pc1Pfs_SPEC,
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
        pc1pfs::Dscr,
        pc1pfs::Dscr,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pc1pfs::Dscr,
            pc1pfs::Dscr,
            Pc1Pfs_SPEC,
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
        pc1pfs::Eofr,
        pc1pfs::Eofr,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pc1pfs::Eofr,
            pc1pfs::Eofr,
            Pc1Pfs_SPEC,
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
        pc1pfs::Isel,
        pc1pfs::Isel,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pc1pfs::Isel,
            pc1pfs::Isel,
            Pc1Pfs_SPEC,
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
        pc1pfs::Asel,
        pc1pfs::Asel,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pc1pfs::Asel,
            pc1pfs::Asel,
            Pc1Pfs_SPEC,
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
        pc1pfs::Pmr,
        pc1pfs::Pmr,
        Pc1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pc1pfs::Pmr,
            pc1pfs::Pmr,
            Pc1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pc1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pc1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pc1Pfs {
    #[inline(always)]
    fn default() -> Pc1Pfs {
        <crate::RegValueT<Pc1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pc1pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc1PfsHa_SPEC;
impl crate::sealed::RegSpec for Pc1PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port C1%s Pin Function Select Register"]
pub type Pc1PfsHa = crate::RegValueT<Pc1PfsHa_SPEC>;

impl Pc1PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pc1pfs_ha::Podr,
        pc1pfs_ha::Podr,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pc1pfs_ha::Podr,
            pc1pfs_ha::Podr,
            Pc1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pc1pfs_ha::Pidr,
        pc1pfs_ha::Pidr,
        Pc1PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pc1pfs_ha::Pidr,
            pc1pfs_ha::Pidr,
            Pc1PfsHa_SPEC,
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
        pc1pfs_ha::Pdr,
        pc1pfs_ha::Pdr,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pc1pfs_ha::Pdr,
            pc1pfs_ha::Pdr,
            Pc1PfsHa_SPEC,
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
        pc1pfs_ha::Pcr,
        pc1pfs_ha::Pcr,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pc1pfs_ha::Pcr,
            pc1pfs_ha::Pcr,
            Pc1PfsHa_SPEC,
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
        pc1pfs_ha::Ncodr,
        pc1pfs_ha::Ncodr,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pc1pfs_ha::Ncodr,
            pc1pfs_ha::Ncodr,
            Pc1PfsHa_SPEC,
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
        pc1pfs_ha::Dscr,
        pc1pfs_ha::Dscr,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pc1pfs_ha::Dscr,
            pc1pfs_ha::Dscr,
            Pc1PfsHa_SPEC,
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
        pc1pfs_ha::Eofr,
        pc1pfs_ha::Eofr,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pc1pfs_ha::Eofr,
            pc1pfs_ha::Eofr,
            Pc1PfsHa_SPEC,
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
        pc1pfs_ha::Isel,
        pc1pfs_ha::Isel,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pc1pfs_ha::Isel,
            pc1pfs_ha::Isel,
            Pc1PfsHa_SPEC,
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
        pc1pfs_ha::Asel,
        pc1pfs_ha::Asel,
        Pc1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pc1pfs_ha::Asel,
            pc1pfs_ha::Asel,
            Pc1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pc1PfsHa {
    #[inline(always)]
    fn default() -> Pc1PfsHa {
        <crate::RegValueT<Pc1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pc1pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc1PfsBy_SPEC;
impl crate::sealed::RegSpec for Pc1PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port C1%s Pin Function Select Register"]
pub type Pc1PfsBy = crate::RegValueT<Pc1PfsBy_SPEC>;

impl Pc1PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pc1pfs_by::Podr,
        pc1pfs_by::Podr,
        Pc1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pc1pfs_by::Podr,
            pc1pfs_by::Podr,
            Pc1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pc1pfs_by::Pidr,
        pc1pfs_by::Pidr,
        Pc1PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pc1pfs_by::Pidr,
            pc1pfs_by::Pidr,
            Pc1PfsBy_SPEC,
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
        pc1pfs_by::Pdr,
        pc1pfs_by::Pdr,
        Pc1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pc1pfs_by::Pdr,
            pc1pfs_by::Pdr,
            Pc1PfsBy_SPEC,
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
        pc1pfs_by::Pcr,
        pc1pfs_by::Pcr,
        Pc1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pc1pfs_by::Pcr,
            pc1pfs_by::Pcr,
            Pc1PfsBy_SPEC,
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
        pc1pfs_by::Ncodr,
        pc1pfs_by::Ncodr,
        Pc1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pc1pfs_by::Ncodr,
            pc1pfs_by::Ncodr,
            Pc1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pc1PfsBy {
    #[inline(always)]
    fn default() -> Pc1PfsBy {
        <crate::RegValueT<Pc1PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pc1pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pd0Pfs_SPEC;
impl crate::sealed::RegSpec for Pd0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port D0%s Pin Function Select Register"]
pub type Pd0Pfs = crate::RegValueT<Pd0Pfs_SPEC>;

impl Pd0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pd0pfs::Podr,
        pd0pfs::Podr,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pd0pfs::Podr,
            pd0pfs::Podr,
            Pd0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pd0pfs::Pidr,
        pd0pfs::Pidr,
        Pd0Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pd0pfs::Pidr,
            pd0pfs::Pidr,
            Pd0Pfs_SPEC,
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
        pd0pfs::Pdr,
        pd0pfs::Pdr,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pd0pfs::Pdr,
            pd0pfs::Pdr,
            Pd0Pfs_SPEC,
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
        pd0pfs::Pcr,
        pd0pfs::Pcr,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pd0pfs::Pcr,
            pd0pfs::Pcr,
            Pd0Pfs_SPEC,
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
        pd0pfs::Ncodr,
        pd0pfs::Ncodr,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pd0pfs::Ncodr,
            pd0pfs::Ncodr,
            Pd0Pfs_SPEC,
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
        pd0pfs::Dscr,
        pd0pfs::Dscr,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pd0pfs::Dscr,
            pd0pfs::Dscr,
            Pd0Pfs_SPEC,
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
        pd0pfs::Eofr,
        pd0pfs::Eofr,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pd0pfs::Eofr,
            pd0pfs::Eofr,
            Pd0Pfs_SPEC,
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
        pd0pfs::Isel,
        pd0pfs::Isel,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pd0pfs::Isel,
            pd0pfs::Isel,
            Pd0Pfs_SPEC,
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
        pd0pfs::Asel,
        pd0pfs::Asel,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pd0pfs::Asel,
            pd0pfs::Asel,
            Pd0Pfs_SPEC,
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
        pd0pfs::Pmr,
        pd0pfs::Pmr,
        Pd0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pd0pfs::Pmr,
            pd0pfs::Pmr,
            Pd0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pd0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pd0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pd0Pfs {
    #[inline(always)]
    fn default() -> Pd0Pfs {
        <crate::RegValueT<Pd0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pd0pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pd0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pd0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port D0%s Pin Function Select Register"]
pub type Pd0PfsHa = crate::RegValueT<Pd0PfsHa_SPEC>;

impl Pd0PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pd0pfs_ha::Podr,
        pd0pfs_ha::Podr,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pd0pfs_ha::Podr,
            pd0pfs_ha::Podr,
            Pd0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pd0pfs_ha::Pidr,
        pd0pfs_ha::Pidr,
        Pd0PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pd0pfs_ha::Pidr,
            pd0pfs_ha::Pidr,
            Pd0PfsHa_SPEC,
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
        pd0pfs_ha::Pdr,
        pd0pfs_ha::Pdr,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pd0pfs_ha::Pdr,
            pd0pfs_ha::Pdr,
            Pd0PfsHa_SPEC,
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
        pd0pfs_ha::Pcr,
        pd0pfs_ha::Pcr,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pd0pfs_ha::Pcr,
            pd0pfs_ha::Pcr,
            Pd0PfsHa_SPEC,
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
        pd0pfs_ha::Ncodr,
        pd0pfs_ha::Ncodr,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pd0pfs_ha::Ncodr,
            pd0pfs_ha::Ncodr,
            Pd0PfsHa_SPEC,
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
        pd0pfs_ha::Dscr,
        pd0pfs_ha::Dscr,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pd0pfs_ha::Dscr,
            pd0pfs_ha::Dscr,
            Pd0PfsHa_SPEC,
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
        pd0pfs_ha::Eofr,
        pd0pfs_ha::Eofr,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pd0pfs_ha::Eofr,
            pd0pfs_ha::Eofr,
            Pd0PfsHa_SPEC,
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
        pd0pfs_ha::Isel,
        pd0pfs_ha::Isel,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pd0pfs_ha::Isel,
            pd0pfs_ha::Isel,
            Pd0PfsHa_SPEC,
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
        pd0pfs_ha::Asel,
        pd0pfs_ha::Asel,
        Pd0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pd0pfs_ha::Asel,
            pd0pfs_ha::Asel,
            Pd0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pd0PfsHa {
    #[inline(always)]
    fn default() -> Pd0PfsHa {
        <crate::RegValueT<Pd0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pd0pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pd0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pd0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port D0%s Pin Function Select Register"]
pub type Pd0PfsBy = crate::RegValueT<Pd0PfsBy_SPEC>;

impl Pd0PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pd0pfs_by::Podr,
        pd0pfs_by::Podr,
        Pd0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pd0pfs_by::Podr,
            pd0pfs_by::Podr,
            Pd0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pd0pfs_by::Pidr,
        pd0pfs_by::Pidr,
        Pd0PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pd0pfs_by::Pidr,
            pd0pfs_by::Pidr,
            Pd0PfsBy_SPEC,
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
        pd0pfs_by::Pdr,
        pd0pfs_by::Pdr,
        Pd0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pd0pfs_by::Pdr,
            pd0pfs_by::Pdr,
            Pd0PfsBy_SPEC,
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
        pd0pfs_by::Pcr,
        pd0pfs_by::Pcr,
        Pd0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pd0pfs_by::Pcr,
            pd0pfs_by::Pcr,
            Pd0PfsBy_SPEC,
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
        pd0pfs_by::Ncodr,
        pd0pfs_by::Ncodr,
        Pd0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pd0pfs_by::Ncodr,
            pd0pfs_by::Ncodr,
            Pd0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pd0PfsBy {
    #[inline(always)]
    fn default() -> Pd0PfsBy {
        <crate::RegValueT<Pd0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pd0pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pd1Pfs_SPEC;
impl crate::sealed::RegSpec for Pd1Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port D1%s Pin Function Select Register"]
pub type Pd1Pfs = crate::RegValueT<Pd1Pfs_SPEC>;

impl Pd1Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pd1pfs::Podr,
        pd1pfs::Podr,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pd1pfs::Podr,
            pd1pfs::Podr,
            Pd1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pd1pfs::Pidr,
        pd1pfs::Pidr,
        Pd1Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pd1pfs::Pidr,
            pd1pfs::Pidr,
            Pd1Pfs_SPEC,
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
        pd1pfs::Pdr,
        pd1pfs::Pdr,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pd1pfs::Pdr,
            pd1pfs::Pdr,
            Pd1Pfs_SPEC,
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
        pd1pfs::Pcr,
        pd1pfs::Pcr,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pd1pfs::Pcr,
            pd1pfs::Pcr,
            Pd1Pfs_SPEC,
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
        pd1pfs::Ncodr,
        pd1pfs::Ncodr,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pd1pfs::Ncodr,
            pd1pfs::Ncodr,
            Pd1Pfs_SPEC,
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
        pd1pfs::Dscr,
        pd1pfs::Dscr,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pd1pfs::Dscr,
            pd1pfs::Dscr,
            Pd1Pfs_SPEC,
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
        pd1pfs::Eofr,
        pd1pfs::Eofr,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pd1pfs::Eofr,
            pd1pfs::Eofr,
            Pd1Pfs_SPEC,
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
        pd1pfs::Isel,
        pd1pfs::Isel,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pd1pfs::Isel,
            pd1pfs::Isel,
            Pd1Pfs_SPEC,
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
        pd1pfs::Asel,
        pd1pfs::Asel,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pd1pfs::Asel,
            pd1pfs::Asel,
            Pd1Pfs_SPEC,
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
        pd1pfs::Pmr,
        pd1pfs::Pmr,
        Pd1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pd1pfs::Pmr,
            pd1pfs::Pmr,
            Pd1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pd1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pd1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pd1Pfs {
    #[inline(always)]
    fn default() -> Pd1Pfs {
        <crate::RegValueT<Pd1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pd1pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pd1PfsHa_SPEC;
impl crate::sealed::RegSpec for Pd1PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port D1%s Pin Function Select Register"]
pub type Pd1PfsHa = crate::RegValueT<Pd1PfsHa_SPEC>;

impl Pd1PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pd1pfs_ha::Podr,
        pd1pfs_ha::Podr,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pd1pfs_ha::Podr,
            pd1pfs_ha::Podr,
            Pd1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pd1pfs_ha::Pidr,
        pd1pfs_ha::Pidr,
        Pd1PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pd1pfs_ha::Pidr,
            pd1pfs_ha::Pidr,
            Pd1PfsHa_SPEC,
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
        pd1pfs_ha::Pdr,
        pd1pfs_ha::Pdr,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pd1pfs_ha::Pdr,
            pd1pfs_ha::Pdr,
            Pd1PfsHa_SPEC,
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
        pd1pfs_ha::Pcr,
        pd1pfs_ha::Pcr,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pd1pfs_ha::Pcr,
            pd1pfs_ha::Pcr,
            Pd1PfsHa_SPEC,
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
        pd1pfs_ha::Ncodr,
        pd1pfs_ha::Ncodr,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pd1pfs_ha::Ncodr,
            pd1pfs_ha::Ncodr,
            Pd1PfsHa_SPEC,
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
        pd1pfs_ha::Dscr,
        pd1pfs_ha::Dscr,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pd1pfs_ha::Dscr,
            pd1pfs_ha::Dscr,
            Pd1PfsHa_SPEC,
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
        pd1pfs_ha::Eofr,
        pd1pfs_ha::Eofr,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pd1pfs_ha::Eofr,
            pd1pfs_ha::Eofr,
            Pd1PfsHa_SPEC,
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
        pd1pfs_ha::Isel,
        pd1pfs_ha::Isel,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pd1pfs_ha::Isel,
            pd1pfs_ha::Isel,
            Pd1PfsHa_SPEC,
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
        pd1pfs_ha::Asel,
        pd1pfs_ha::Asel,
        Pd1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pd1pfs_ha::Asel,
            pd1pfs_ha::Asel,
            Pd1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pd1PfsHa {
    #[inline(always)]
    fn default() -> Pd1PfsHa {
        <crate::RegValueT<Pd1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pd1pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pd1PfsBy_SPEC;
impl crate::sealed::RegSpec for Pd1PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port D1%s Pin Function Select Register"]
pub type Pd1PfsBy = crate::RegValueT<Pd1PfsBy_SPEC>;

impl Pd1PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pd1pfs_by::Podr,
        pd1pfs_by::Podr,
        Pd1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pd1pfs_by::Podr,
            pd1pfs_by::Podr,
            Pd1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pd1pfs_by::Pidr,
        pd1pfs_by::Pidr,
        Pd1PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pd1pfs_by::Pidr,
            pd1pfs_by::Pidr,
            Pd1PfsBy_SPEC,
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
        pd1pfs_by::Pdr,
        pd1pfs_by::Pdr,
        Pd1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pd1pfs_by::Pdr,
            pd1pfs_by::Pdr,
            Pd1PfsBy_SPEC,
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
        pd1pfs_by::Pcr,
        pd1pfs_by::Pcr,
        Pd1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pd1pfs_by::Pcr,
            pd1pfs_by::Pcr,
            Pd1PfsBy_SPEC,
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
        pd1pfs_by::Ncodr,
        pd1pfs_by::Ncodr,
        Pd1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pd1pfs_by::Ncodr,
            pd1pfs_by::Ncodr,
            Pd1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pd1PfsBy {
    #[inline(always)]
    fn default() -> Pd1PfsBy {
        <crate::RegValueT<Pd1PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pd1pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe0Pfs_SPEC;
impl crate::sealed::RegSpec for Pe0Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port E0%s Pin Function Select Register"]
pub type Pe0Pfs = crate::RegValueT<Pe0Pfs_SPEC>;

impl Pe0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pe0pfs::Podr,
        pe0pfs::Podr,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pe0pfs::Podr,
            pe0pfs::Podr,
            Pe0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pe0pfs::Pidr,
        pe0pfs::Pidr,
        Pe0Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pe0pfs::Pidr,
            pe0pfs::Pidr,
            Pe0Pfs_SPEC,
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
        pe0pfs::Pdr,
        pe0pfs::Pdr,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pe0pfs::Pdr,
            pe0pfs::Pdr,
            Pe0Pfs_SPEC,
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
        pe0pfs::Pcr,
        pe0pfs::Pcr,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pe0pfs::Pcr,
            pe0pfs::Pcr,
            Pe0Pfs_SPEC,
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
        pe0pfs::Ncodr,
        pe0pfs::Ncodr,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pe0pfs::Ncodr,
            pe0pfs::Ncodr,
            Pe0Pfs_SPEC,
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
        pe0pfs::Dscr,
        pe0pfs::Dscr,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pe0pfs::Dscr,
            pe0pfs::Dscr,
            Pe0Pfs_SPEC,
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
        pe0pfs::Eofr,
        pe0pfs::Eofr,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pe0pfs::Eofr,
            pe0pfs::Eofr,
            Pe0Pfs_SPEC,
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
        pe0pfs::Isel,
        pe0pfs::Isel,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pe0pfs::Isel,
            pe0pfs::Isel,
            Pe0Pfs_SPEC,
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
        pe0pfs::Asel,
        pe0pfs::Asel,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pe0pfs::Asel,
            pe0pfs::Asel,
            Pe0Pfs_SPEC,
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
        pe0pfs::Pmr,
        pe0pfs::Pmr,
        Pe0Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pe0pfs::Pmr,
            pe0pfs::Pmr,
            Pe0Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pe0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pe0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pe0Pfs {
    #[inline(always)]
    fn default() -> Pe0Pfs {
        <crate::RegValueT<Pe0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pe0pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pe0PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port E0%s Pin Function Select Register"]
pub type Pe0PfsHa = crate::RegValueT<Pe0PfsHa_SPEC>;

impl Pe0PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pe0pfs_ha::Podr,
        pe0pfs_ha::Podr,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pe0pfs_ha::Podr,
            pe0pfs_ha::Podr,
            Pe0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pe0pfs_ha::Pidr,
        pe0pfs_ha::Pidr,
        Pe0PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pe0pfs_ha::Pidr,
            pe0pfs_ha::Pidr,
            Pe0PfsHa_SPEC,
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
        pe0pfs_ha::Pdr,
        pe0pfs_ha::Pdr,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pe0pfs_ha::Pdr,
            pe0pfs_ha::Pdr,
            Pe0PfsHa_SPEC,
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
        pe0pfs_ha::Pcr,
        pe0pfs_ha::Pcr,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pe0pfs_ha::Pcr,
            pe0pfs_ha::Pcr,
            Pe0PfsHa_SPEC,
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
        pe0pfs_ha::Ncodr,
        pe0pfs_ha::Ncodr,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pe0pfs_ha::Ncodr,
            pe0pfs_ha::Ncodr,
            Pe0PfsHa_SPEC,
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
        pe0pfs_ha::Dscr,
        pe0pfs_ha::Dscr,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pe0pfs_ha::Dscr,
            pe0pfs_ha::Dscr,
            Pe0PfsHa_SPEC,
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
        pe0pfs_ha::Eofr,
        pe0pfs_ha::Eofr,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pe0pfs_ha::Eofr,
            pe0pfs_ha::Eofr,
            Pe0PfsHa_SPEC,
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
        pe0pfs_ha::Isel,
        pe0pfs_ha::Isel,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pe0pfs_ha::Isel,
            pe0pfs_ha::Isel,
            Pe0PfsHa_SPEC,
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
        pe0pfs_ha::Asel,
        pe0pfs_ha::Asel,
        Pe0PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pe0pfs_ha::Asel,
            pe0pfs_ha::Asel,
            Pe0PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pe0PfsHa {
    #[inline(always)]
    fn default() -> Pe0PfsHa {
        <crate::RegValueT<Pe0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pe0pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pe0PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port E0%s Pin Function Select Register"]
pub type Pe0PfsBy = crate::RegValueT<Pe0PfsBy_SPEC>;

impl Pe0PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pe0pfs_by::Podr,
        pe0pfs_by::Podr,
        Pe0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pe0pfs_by::Podr,
            pe0pfs_by::Podr,
            Pe0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pe0pfs_by::Pidr,
        pe0pfs_by::Pidr,
        Pe0PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pe0pfs_by::Pidr,
            pe0pfs_by::Pidr,
            Pe0PfsBy_SPEC,
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
        pe0pfs_by::Pdr,
        pe0pfs_by::Pdr,
        Pe0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pe0pfs_by::Pdr,
            pe0pfs_by::Pdr,
            Pe0PfsBy_SPEC,
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
        pe0pfs_by::Pcr,
        pe0pfs_by::Pcr,
        Pe0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pe0pfs_by::Pcr,
            pe0pfs_by::Pcr,
            Pe0PfsBy_SPEC,
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
        pe0pfs_by::Ncodr,
        pe0pfs_by::Ncodr,
        Pe0PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pe0pfs_by::Ncodr,
            pe0pfs_by::Ncodr,
            Pe0PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pe0PfsBy {
    #[inline(always)]
    fn default() -> Pe0PfsBy {
        <crate::RegValueT<Pe0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pe0pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe1Pfs_SPEC;
impl crate::sealed::RegSpec for Pe1Pfs_SPEC {
    type DataType = u32;
}

#[doc = "Port E1%s Pin Function Select Register"]
pub type Pe1Pfs = crate::RegValueT<Pe1Pfs_SPEC>;

impl Pe1Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pe1pfs::Podr,
        pe1pfs::Podr,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pe1pfs::Podr,
            pe1pfs::Podr,
            Pe1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pe1pfs::Pidr,
        pe1pfs::Pidr,
        Pe1Pfs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pe1pfs::Pidr,
            pe1pfs::Pidr,
            Pe1Pfs_SPEC,
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
        pe1pfs::Pdr,
        pe1pfs::Pdr,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pe1pfs::Pdr,
            pe1pfs::Pdr,
            Pe1Pfs_SPEC,
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
        pe1pfs::Pcr,
        pe1pfs::Pcr,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pe1pfs::Pcr,
            pe1pfs::Pcr,
            Pe1Pfs_SPEC,
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
        pe1pfs::Ncodr,
        pe1pfs::Ncodr,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pe1pfs::Ncodr,
            pe1pfs::Ncodr,
            Pe1Pfs_SPEC,
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
        pe1pfs::Dscr,
        pe1pfs::Dscr,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pe1pfs::Dscr,
            pe1pfs::Dscr,
            Pe1Pfs_SPEC,
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
        pe1pfs::Eofr,
        pe1pfs::Eofr,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pe1pfs::Eofr,
            pe1pfs::Eofr,
            Pe1Pfs_SPEC,
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
        pe1pfs::Isel,
        pe1pfs::Isel,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pe1pfs::Isel,
            pe1pfs::Isel,
            Pe1Pfs_SPEC,
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
        pe1pfs::Asel,
        pe1pfs::Asel,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pe1pfs::Asel,
            pe1pfs::Asel,
            Pe1Pfs_SPEC,
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
        pe1pfs::Pmr,
        pe1pfs::Pmr,
        Pe1Pfs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pe1pfs::Pmr,
            pe1pfs::Pmr,
            Pe1Pfs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Pe1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Pe1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pe1Pfs {
    #[inline(always)]
    fn default() -> Pe1Pfs {
        <crate::RegValueT<Pe1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pe1pfs {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe1PfsHa_SPEC;
impl crate::sealed::RegSpec for Pe1PfsHa_SPEC {
    type DataType = u16;
}

#[doc = "Port E1%s Pin Function Select Register"]
pub type Pe1PfsHa = crate::RegValueT<Pe1PfsHa_SPEC>;

impl Pe1PfsHa {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pe1pfs_ha::Podr,
        pe1pfs_ha::Podr,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pe1pfs_ha::Podr,
            pe1pfs_ha::Podr,
            Pe1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pe1pfs_ha::Pidr,
        pe1pfs_ha::Pidr,
        Pe1PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pe1pfs_ha::Pidr,
            pe1pfs_ha::Pidr,
            Pe1PfsHa_SPEC,
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
        pe1pfs_ha::Pdr,
        pe1pfs_ha::Pdr,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pe1pfs_ha::Pdr,
            pe1pfs_ha::Pdr,
            Pe1PfsHa_SPEC,
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
        pe1pfs_ha::Pcr,
        pe1pfs_ha::Pcr,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pe1pfs_ha::Pcr,
            pe1pfs_ha::Pcr,
            Pe1PfsHa_SPEC,
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
        pe1pfs_ha::Ncodr,
        pe1pfs_ha::Ncodr,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pe1pfs_ha::Ncodr,
            pe1pfs_ha::Ncodr,
            Pe1PfsHa_SPEC,
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
        pe1pfs_ha::Dscr,
        pe1pfs_ha::Dscr,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            pe1pfs_ha::Dscr,
            pe1pfs_ha::Dscr,
            Pe1PfsHa_SPEC,
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
        pe1pfs_ha::Eofr,
        pe1pfs_ha::Eofr,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            pe1pfs_ha::Eofr,
            pe1pfs_ha::Eofr,
            Pe1PfsHa_SPEC,
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
        pe1pfs_ha::Isel,
        pe1pfs_ha::Isel,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pe1pfs_ha::Isel,
            pe1pfs_ha::Isel,
            Pe1PfsHa_SPEC,
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
        pe1pfs_ha::Asel,
        pe1pfs_ha::Asel,
        Pe1PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pe1pfs_ha::Asel,
            pe1pfs_ha::Asel,
            Pe1PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pe1PfsHa {
    #[inline(always)]
    fn default() -> Pe1PfsHa {
        <crate::RegValueT<Pe1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pe1pfs_ha {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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

        #[doc = "High-speed high-drive / High current drive"]
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
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe1PfsBy_SPEC;
impl crate::sealed::RegSpec for Pe1PfsBy_SPEC {
    type DataType = u8;
}

#[doc = "Port E1%s Pin Function Select Register"]
pub type Pe1PfsBy = crate::RegValueT<Pe1PfsBy_SPEC>;

impl Pe1PfsBy {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pe1pfs_by::Podr,
        pe1pfs_by::Podr,
        Pe1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pe1pfs_by::Podr,
            pe1pfs_by::Podr,
            Pe1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pe1pfs_by::Pidr,
        pe1pfs_by::Pidr,
        Pe1PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pe1pfs_by::Pidr,
            pe1pfs_by::Pidr,
            Pe1PfsBy_SPEC,
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
        pe1pfs_by::Pdr,
        pe1pfs_by::Pdr,
        Pe1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pe1pfs_by::Pdr,
            pe1pfs_by::Pdr,
            Pe1PfsBy_SPEC,
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
        pe1pfs_by::Pcr,
        pe1pfs_by::Pcr,
        Pe1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pe1pfs_by::Pcr,
            pe1pfs_by::Pcr,
            Pe1PfsBy_SPEC,
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
        pe1pfs_by::Ncodr,
        pe1pfs_by::Ncodr,
        Pe1PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pe1pfs_by::Ncodr,
            pe1pfs_by::Ncodr,
            Pe1PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pe1PfsBy {
    #[inline(always)]
    fn default() -> Pe1PfsBy {
        <crate::RegValueT<Pe1PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pe1pfs_by {

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
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "NMOS open-drain output"]
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
pub struct P0Sar_SPEC;
impl crate::sealed::RegSpec for P0Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port 0 Security Attribution register"]
pub type P0Sar = crate::RegValueT<P0Sar_SPEC>;

impl P0Sar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        p0sar::Pmnsa,
        p0sar::Pmnsa,
        P0Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            p0sar::Pmnsa,
            p0sar::Pmnsa,
            P0Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0Sar {
    #[inline(always)]
    fn default() -> P0Sar {
        <crate::RegValueT<P0Sar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod p0sar {

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
pub struct P2Sar_SPEC;
impl crate::sealed::RegSpec for P2Sar_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Security Attribution register"]
pub type P2Sar = crate::RegValueT<P2Sar_SPEC>;

impl P2Sar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        p2sar::Pmnsa,
        p2sar::Pmnsa,
        P2Sar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            p2sar::Pmnsa,
            p2sar::Pmnsa,
            P2Sar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2Sar {
    #[inline(always)]
    fn default() -> P2Sar {
        <crate::RegValueT<P2Sar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod p2sar {

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
pub struct Pasar_SPEC;
impl crate::sealed::RegSpec for Pasar_SPEC {
    type DataType = u16;
}

#[doc = "Port A Security Attribution register"]
pub type Pasar = crate::RegValueT<Pasar_SPEC>;

impl Pasar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pasar::Pmnsa,
        pasar::Pmnsa,
        Pasar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pasar::Pmnsa,
            pasar::Pmnsa,
            Pasar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pasar {
    #[inline(always)]
    fn default() -> Pasar {
        <crate::RegValueT<Pasar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod pasar {

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
pub struct Pbsar_SPEC;
impl crate::sealed::RegSpec for Pbsar_SPEC {
    type DataType = u16;
}

#[doc = "Port B Security Attribution register"]
pub type Pbsar = crate::RegValueT<Pbsar_SPEC>;

impl Pbsar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pbsar::Pmnsa,
        pbsar::Pmnsa,
        Pbsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pbsar::Pmnsa,
            pbsar::Pmnsa,
            Pbsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pbsar {
    #[inline(always)]
    fn default() -> Pbsar {
        <crate::RegValueT<Pbsar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod pbsar {

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
pub struct Pcsar_SPEC;
impl crate::sealed::RegSpec for Pcsar_SPEC {
    type DataType = u16;
}

#[doc = "Port C Security Attribution register"]
pub type Pcsar = crate::RegValueT<Pcsar_SPEC>;

impl Pcsar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pcsar::Pmnsa,
        pcsar::Pmnsa,
        Pcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pcsar::Pmnsa,
            pcsar::Pmnsa,
            Pcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcsar {
    #[inline(always)]
    fn default() -> Pcsar {
        <crate::RegValueT<Pcsar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod pcsar {

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
pub struct Pdsar_SPEC;
impl crate::sealed::RegSpec for Pdsar_SPEC {
    type DataType = u16;
}

#[doc = "Port D Security Attribution register"]
pub type Pdsar = crate::RegValueT<Pdsar_SPEC>;

impl Pdsar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pdsar::Pmnsa,
        pdsar::Pmnsa,
        Pdsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pdsar::Pmnsa,
            pdsar::Pmnsa,
            Pdsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pdsar {
    #[inline(always)]
    fn default() -> Pdsar {
        <crate::RegValueT<Pdsar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod pdsar {

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
pub struct Pesar_SPEC;
impl crate::sealed::RegSpec for Pesar_SPEC {
    type DataType = u16;
}

#[doc = "Port E Security Attribution register"]
pub type Pesar = crate::RegValueT<Pesar_SPEC>;

impl Pesar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        pesar::Pmnsa,
        pesar::Pmnsa,
        Pesar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            pesar::Pmnsa,
            pesar::Pmnsa,
            Pesar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pesar {
    #[inline(always)]
    fn default() -> Pesar {
        <crate::RegValueT<Pesar_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod pesar {

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
