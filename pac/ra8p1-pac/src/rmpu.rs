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
#[doc = r"Renesas Memory Protection Unit"]
unsafe impl ::core::marker::Send for super::Rmpu {}
unsafe impl ::core::marker::Sync for super::Rmpu {}
impl super::Rmpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "MMPU Operation After Detection Register"]
    #[inline(always)]
    pub const fn mmpuoad(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "MMPU Operation After Detection Protect Register"]
    #[inline(always)]
    pub const fn mmpuoadpt(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuoadpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuoadpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for DMAC"]
    #[inline(always)]
    pub const fn mmpuendmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuendmac_SPEC, crate::common::RW>,
        2,
        0x200,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuendmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuendmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuendmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuendmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuendmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuendmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for DMAC %s"]
    #[inline(always)]
    pub const fn mmpuenptdmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuenptdmac_SPEC, crate::common::RW>,
        2,
        0x200,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x104usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuenptdmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptdmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptdmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuenptdmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptdmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptdmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for DMAC Secure %s"]
    #[inline(always)]
    pub const fn mmpurptdmac_sec(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MmpurptdmacSec_SPEC, crate::common::RW>,
        2,
        0x200,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10cusize))
        }
    }
    #[inline(always)]
    pub const fn mmpurptdmac_sec0(
        &self,
    ) -> &'static crate::common::Reg<self::MmpurptdmacSec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MmpurptdmacSec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpurptdmac_sec1(
        &self,
    ) -> &'static crate::common::Reg<self::MmpurptdmacSec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MmpurptdmacSec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for DMAC"]
    #[inline(always)]
    pub const fn mmpuacdmac0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac00(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac01(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac02(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac03(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac04(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac05(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac06(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac07(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for DMAC (n = 00 to 07)"]
    #[inline(always)]
    pub const fn mmpusdmac0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac00(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac01(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac02(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac03(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac04(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac05(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac06(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac07(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for DMAC (n = 00 to 07)"]
    #[inline(always)]
    pub const fn mmpuedmac0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac00(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac01(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac02(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac03(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac04(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac05(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac06(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac07(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for DMAC"]
    #[inline(always)]
    pub const fn mmpuacdmac1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac16(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdmac17(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x470usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for DMAC (n = 00 to 07)"]
    #[inline(always)]
    pub const fn mmpusdmac1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x404usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x414usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x434usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x454usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac16(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdmac17(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x474usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for DMAC (n = 00 to 07)"]
    #[inline(always)]
    pub const fn mmpuedmac1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x408usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x438usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x458usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac16(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedmac17(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x478usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpuenedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpuenptedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpurptedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpuacedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW>,
        5,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacedmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacedmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacedmac2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacedmac3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacedmac4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x640usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for EDMAC (n = 0 to 4)"]
    #[inline(always)]
    pub const fn mmpusedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW>,
        5,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusedmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusedmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusedmac2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusedmac3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusedmac4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x644usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpueedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW>,
        5,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }
    #[inline(always)]
    pub const fn mmpueedmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueedmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x618usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueedmac2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueedmac3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x638usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueedmac4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x648usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for GLCDC"]
    #[inline(always)]
    pub const fn mmpuenglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1792usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for GLCDC"]
    #[inline(always)]
    pub const fn mmpuenptglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1796usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for GLCDC"]
    #[inline(always)]
    pub const fn mmpurptglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1800usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for GLCDC (n = 0, 1)"]
    #[inline(always)]
    pub const fn mmpuacglcdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacglcdc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x800usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacglcdc0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacglcdc1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x810usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for GLCDC (n = 0, 1)"]
    #[inline(always)]
    pub const fn mmpusglcdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusglcdc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x804usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusglcdc0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusglcdc1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x814usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for GLCDC (n = 0, 1)"]
    #[inline(always)]
    pub const fn mmpueglcdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueglcdc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x808usize))
        }
    }
    #[inline(always)]
    pub const fn mmpueglcdc0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueglcdc1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x818usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for DRW"]
    #[inline(always)]
    pub const fn mmpuendrw(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuendrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuendrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2304usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for DRW"]
    #[inline(always)]
    pub const fn mmpuenpdrw(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenpdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenpdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2308usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for DRW"]
    #[inline(always)]
    pub const fn mmpurptdrw(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2312usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for DRW (n = 0 to 2)"]
    #[inline(always)]
    pub const fn mmpuacdrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacdrw_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa00usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacdrw0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdrw1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacdrw2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa20usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for DRW (n = 0 to 2)"]
    #[inline(always)]
    pub const fn mmpusdrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusdrw_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa04usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusdrw0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdrw1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusdrw2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa24usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for DRW (n = 0 to 2)"]
    #[inline(always)]
    pub const fn mmpuedrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuedrw_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa08usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuedrw0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedrw1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuedrw2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuedrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuedrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa28usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for MIPI-DSI"]
    #[inline(always)]
    pub const fn mmpuenmipid(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenmipid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenmipid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2816usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for MIPI-DSI"]
    #[inline(always)]
    pub const fn mmpuenptmipid(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptmipid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptmipid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2820usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for MIPI-DSI"]
    #[inline(always)]
    pub const fn mmpurptmipid(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptmipid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptmipid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2824usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for MIPI-DSI"]
    #[inline(always)]
    pub const fn mmpuacmipid(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacmipid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacmipid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for MIPI-DSI"]
    #[inline(always)]
    pub const fn mmpusmipid(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusmipid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusmipid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3076usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for MIPI-DSI"]
    #[inline(always)]
    pub const fn mmpuemipid(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuemipid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuemipid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3080usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for CEU"]
    #[inline(always)]
    pub const fn mmpuenceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3328usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for CEU"]
    #[inline(always)]
    pub const fn mmpuenptceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3332usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for CEU"]
    #[inline(always)]
    pub const fn mmpurptceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3336usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for CEU (n = 0, 1)"]
    #[inline(always)]
    pub const fn mmpuacceu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacceu_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe00usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacceu0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacceu1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe10usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for CEU (n = 0, 1)"]
    #[inline(always)]
    pub const fn mmpusceu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusceu_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe04usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusceu0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusceu1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe14usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for CEU (n = 0, 1)"]
    #[inline(always)]
    pub const fn mmpueceu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueceu_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe08usize))
        }
    }
    #[inline(always)]
    pub const fn mmpueceu0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueceu1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe18usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for MIPI-CSI by VIN"]
    #[inline(always)]
    pub const fn mmpuenmipic(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3840usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for MIPI-CSI by VIN"]
    #[inline(always)]
    pub const fn mmpuenptmipic(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3844usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for MIPI-CSI by VIN"]
    #[inline(always)]
    pub const fn mmpurptmipic(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3848usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for MIPI-CSI via VIN (n = 0 to 2)"]
    #[inline(always)]
    pub const fn mmpuacmipic(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacmipic_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacmipic0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1000usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacmipic1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1010usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacmipic2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1020usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for MIPI-CSI via VIN (n = 0 to 2)"]
    #[inline(always)]
    pub const fn mmpusmipic(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusmipic_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusmipic0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusmipic1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1014usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusmipic2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusmipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusmipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1024usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for MIPI-CSI via VIN (n = 0 to 2)"]
    #[inline(always)]
    pub const fn mmpuemipic(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuemipic_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1008usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuemipic0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuemipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuemipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1008usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuemipic1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuemipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuemipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1018usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuemipic2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuemipic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuemipic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1028usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for NPU"]
    #[inline(always)]
    pub const fn mmpuennpu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuennpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuennpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for NPU"]
    #[inline(always)]
    pub const fn mmpuenptnpu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4356usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for NPU"]
    #[inline(always)]
    pub const fn mmpurptnpu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4360usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for NPU (n = 0 to 4)"]
    #[inline(always)]
    pub const fn mmpuacnpu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacnpu_SPEC, crate::common::RW>,
        5,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1200usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacnpu0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacnpu1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacnpu2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacnpu3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacnpu4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1240usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for NPU (n = 0 to 4)"]
    #[inline(always)]
    pub const fn mmpusnpu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusnpu_SPEC, crate::common::RW>,
        5,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1204usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusnpu0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusnpu1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusnpu2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusnpu3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusnpu4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusnpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusnpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1244usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for NPU (n = 0 to 4)"]
    #[inline(always)]
    pub const fn mmpuenpu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuenpu_SPEC, crate::common::RW>,
        5,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1208usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuenpu0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuenpu1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuenpu2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuenpu3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuenpu4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1248usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuoad_SPEC;
impl crate::sealed::RegSpec for Mmpuoad_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Operation After Detection Register"]
pub type Mmpuoad = crate::RegValueT<Mmpuoad_SPEC>;

impl Mmpuoad {
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuoad::Oad,
        mmpuoad::Oad,
        Mmpuoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuoad::Oad,
            mmpuoad::Oad,
            Mmpuoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuoad_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuoad_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuoad {
    #[inline(always)]
    fn default() -> Mmpuoad {
        <crate::RegValueT<Mmpuoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "IRQ"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuoadpt_SPEC;
impl crate::sealed::RegSpec for Mmpuoadpt_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Operation After Detection Protect Register"]
pub type Mmpuoadpt = crate::RegValueT<Mmpuoadpt_SPEC>;

impl Mmpuoadpt {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuoadpt::Protect,
        mmpuoadpt::Protect,
        Mmpuoadpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuoadpt::Protect,
            mmpuoadpt::Protect,
            Mmpuoadpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuoadpt_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuoadpt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuoadpt {
    #[inline(always)]
    fn default() -> Mmpuoadpt {
        <crate::RegValueT<Mmpuoadpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuoadpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUOAD register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUOAD register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuendmac_SPEC;
impl crate::sealed::RegSpec for Mmpuendmac_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for DMAC"]
pub type Mmpuendmac = crate::RegValueT<Mmpuendmac_SPEC>;

impl Mmpuendmac {
    #[doc = "Bus master MPU of DMAC Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuendmac::Enable,
        mmpuendmac::Enable,
        Mmpuendmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuendmac::Enable,
            mmpuendmac::Enable,
            Mmpuendmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuendmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuendmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuendmac {
    #[inline(always)]
    fn default() -> Mmpuendmac {
        <crate::RegValueT<Mmpuendmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuendmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of DMAC is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of DMAC is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptdmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenptdmac_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for DMAC %s"]
pub type Mmpuenptdmac = crate::RegValueT<Mmpuenptdmac_SPEC>;

impl Mmpuenptdmac {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptdmac::Protect,
        mmpuenptdmac::Protect,
        Mmpuenptdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptdmac::Protect,
            mmpuenptdmac::Protect,
            Mmpuenptdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptdmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptdmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptdmac {
    #[inline(always)]
    fn default() -> Mmpuenptdmac {
        <crate::RegValueT<Mmpuenptdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptdmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENDMACm register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENDMACm register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmpurptdmacSec_SPEC;
impl crate::sealed::RegSpec for MmpurptdmacSec_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for DMAC Secure %s"]
pub type MmpurptdmacSec = crate::RegValueT<MmpurptdmacSec_SPEC>;

impl MmpurptdmacSec {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptdmac_sec::Protect,
        mmpurptdmac_sec::Protect,
        MmpurptdmacSec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptdmac_sec::Protect,
            mmpurptdmac_sec::Protect,
            MmpurptdmacSec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, MmpurptdmacSec_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,MmpurptdmacSec_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for MmpurptdmacSec {
    #[inline(always)]
    fn default() -> MmpurptdmacSec {
        <crate::RegValueT<MmpurptdmacSec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptdmac_sec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for DMAC Secure write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for DMAC Secure write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacdmac0_SPEC;
impl crate::sealed::RegSpec for Mmpuacdmac0_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for DMAC"]
pub type Mmpuacdmac0 = crate::RegValueT<Mmpuacdmac0_SPEC>;

impl Mmpuacdmac0 {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacdmac0::Enable,
        mmpuacdmac0::Enable,
        Mmpuacdmac0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacdmac0::Enable,
            mmpuacdmac0::Enable,
            Mmpuacdmac0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacdmac0::Rp,
        mmpuacdmac0::Rp,
        Mmpuacdmac0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacdmac0::Rp,
            mmpuacdmac0::Rp,
            Mmpuacdmac0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacdmac0::Wp,
        mmpuacdmac0::Wp,
        Mmpuacdmac0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacdmac0::Wp,
            mmpuacdmac0::Wp,
            Mmpuacdmac0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege protection"]
    #[inline(always)]
    pub fn pp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mmpuacdmac0::Pp,
        mmpuacdmac0::Pp,
        Mmpuacdmac0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mmpuacdmac0::Pp,
            mmpuacdmac0::Pp,
            Mmpuacdmac0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacdmac0 {
    #[inline(always)]
    fn default() -> Mmpuacdmac0 {
        <crate::RegValueT<Mmpuacdmac0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacdmac0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "DMAC region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pp_SPEC;
    pub type Pp = crate::EnumBitfieldStruct<u8, Pp_SPEC>;
    impl Pp {
        #[doc = "Unprivileged access permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged access protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusdmac0_SPEC;
impl crate::sealed::RegSpec for Mmpusdmac0_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for DMAC (n = 00 to 07)"]
pub type Mmpusdmac0 = crate::RegValueT<Mmpusdmac0_SPEC>;

impl NoBitfieldReg<Mmpusdmac0_SPEC> for Mmpusdmac0 {}
impl ::core::default::Default for Mmpusdmac0 {
    #[inline(always)]
    fn default() -> Mmpusdmac0 {
        <crate::RegValueT<Mmpusdmac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuedmac0_SPEC;
impl crate::sealed::RegSpec for Mmpuedmac0_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for DMAC (n = 00 to 07)"]
pub type Mmpuedmac0 = crate::RegValueT<Mmpuedmac0_SPEC>;

impl NoBitfieldReg<Mmpuedmac0_SPEC> for Mmpuedmac0 {}
impl ::core::default::Default for Mmpuedmac0 {
    #[inline(always)]
    fn default() -> Mmpuedmac0 {
        <crate::RegValueT<Mmpuedmac0_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacdmac1_SPEC;
impl crate::sealed::RegSpec for Mmpuacdmac1_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for DMAC"]
pub type Mmpuacdmac1 = crate::RegValueT<Mmpuacdmac1_SPEC>;

impl Mmpuacdmac1 {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacdmac1::Enable,
        mmpuacdmac1::Enable,
        Mmpuacdmac1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacdmac1::Enable,
            mmpuacdmac1::Enable,
            Mmpuacdmac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacdmac1::Rp,
        mmpuacdmac1::Rp,
        Mmpuacdmac1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacdmac1::Rp,
            mmpuacdmac1::Rp,
            Mmpuacdmac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacdmac1::Wp,
        mmpuacdmac1::Wp,
        Mmpuacdmac1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacdmac1::Wp,
            mmpuacdmac1::Wp,
            Mmpuacdmac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Privilege protection"]
    #[inline(always)]
    pub fn pp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mmpuacdmac1::Pp,
        mmpuacdmac1::Pp,
        Mmpuacdmac1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mmpuacdmac1::Pp,
            mmpuacdmac1::Pp,
            Mmpuacdmac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacdmac1 {
    #[inline(always)]
    fn default() -> Mmpuacdmac1 {
        <crate::RegValueT<Mmpuacdmac1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacdmac1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "DMAC region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pp_SPEC;
    pub type Pp = crate::EnumBitfieldStruct<u8, Pp_SPEC>;
    impl Pp {
        #[doc = "Unprivileged access permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged access protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusdmac1_SPEC;
impl crate::sealed::RegSpec for Mmpusdmac1_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for DMAC (n = 00 to 07)"]
pub type Mmpusdmac1 = crate::RegValueT<Mmpusdmac1_SPEC>;

impl NoBitfieldReg<Mmpusdmac1_SPEC> for Mmpusdmac1 {}
impl ::core::default::Default for Mmpusdmac1 {
    #[inline(always)]
    fn default() -> Mmpusdmac1 {
        <crate::RegValueT<Mmpusdmac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuedmac1_SPEC;
impl crate::sealed::RegSpec for Mmpuedmac1_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for DMAC (n = 00 to 07)"]
pub type Mmpuedmac1 = crate::RegValueT<Mmpuedmac1_SPEC>;

impl NoBitfieldReg<Mmpuedmac1_SPEC> for Mmpuedmac1 {}
impl ::core::default::Default for Mmpuedmac1 {
    #[inline(always)]
    fn default() -> Mmpuedmac1 {
        <crate::RegValueT<Mmpuedmac1_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenedmac_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for EDMAC"]
pub type Mmpuenedmac = crate::RegValueT<Mmpuenedmac_SPEC>;

impl Mmpuenedmac {
    #[doc = "Bus master MPU of EDMAC Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenedmac::Enable,
        mmpuenedmac::Enable,
        Mmpuenedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenedmac::Enable,
            mmpuenedmac::Enable,
            Mmpuenedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenedmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenedmac {
    #[inline(always)]
    fn default() -> Mmpuenedmac {
        <crate::RegValueT<Mmpuenedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of EDMAC is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of EDMAC is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenptedmac_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for EDMAC"]
pub type Mmpuenptedmac = crate::RegValueT<Mmpuenptedmac_SPEC>;

impl Mmpuenptedmac {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptedmac::Protect,
        mmpuenptedmac::Protect,
        Mmpuenptedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptedmac::Protect,
            mmpuenptedmac::Protect,
            Mmpuenptedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptedmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptedmac {
    #[inline(always)]
    fn default() -> Mmpuenptedmac {
        <crate::RegValueT<Mmpuenptedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENEDMAC register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENEDMAC register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptedmac_SPEC;
impl crate::sealed::RegSpec for Mmpurptedmac_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for EDMAC"]
pub type Mmpurptedmac = crate::RegValueT<Mmpurptedmac_SPEC>;

impl Mmpurptedmac {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptedmac::Protect,
        mmpurptedmac::Protect,
        Mmpurptedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptedmac::Protect,
            mmpurptedmac::Protect,
            Mmpurptedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptedmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptedmac {
    #[inline(always)]
    fn default() -> Mmpurptedmac {
        <crate::RegValueT<Mmpurptedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for EDMAC write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for EDMAC write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuacedmac_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for EDMAC"]
pub type Mmpuacedmac = crate::RegValueT<Mmpuacedmac_SPEC>;

impl Mmpuacedmac {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacedmac::Enable,
        mmpuacedmac::Enable,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacedmac::Enable,
            mmpuacedmac::Enable,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacedmac::Rp,
        mmpuacedmac::Rp,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacedmac::Rp,
            mmpuacedmac::Rp,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacedmac::Wp,
        mmpuacedmac::Wp,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacedmac::Wp,
            mmpuacedmac::Wp,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacedmac {
    #[inline(always)]
    fn default() -> Mmpuacedmac {
        <crate::RegValueT<Mmpuacedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "EDMAC region n unit is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "EDMAC region n unit is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusedmac_SPEC;
impl crate::sealed::RegSpec for Mmpusedmac_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for EDMAC (n = 0 to 4)"]
pub type Mmpusedmac = crate::RegValueT<Mmpusedmac_SPEC>;

impl NoBitfieldReg<Mmpusedmac_SPEC> for Mmpusedmac {}
impl ::core::default::Default for Mmpusedmac {
    #[inline(always)]
    fn default() -> Mmpusedmac {
        <crate::RegValueT<Mmpusedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueedmac_SPEC;
impl crate::sealed::RegSpec for Mmpueedmac_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for EDMAC"]
pub type Mmpueedmac = crate::RegValueT<Mmpueedmac_SPEC>;

impl NoBitfieldReg<Mmpueedmac_SPEC> for Mmpueedmac {}
impl ::core::default::Default for Mmpueedmac {
    #[inline(always)]
    fn default() -> Mmpueedmac {
        <crate::RegValueT<Mmpueedmac_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpuenglcdc_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for GLCDC"]
pub type Mmpuenglcdc = crate::RegValueT<Mmpuenglcdc_SPEC>;

impl Mmpuenglcdc {
    #[doc = "Bus master MPU of GLCDC Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenglcdc::Enable,
        mmpuenglcdc::Enable,
        Mmpuenglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenglcdc::Enable,
            mmpuenglcdc::Enable,
            Mmpuenglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenglcdc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenglcdc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenglcdc {
    #[inline(always)]
    fn default() -> Mmpuenglcdc {
        <crate::RegValueT<Mmpuenglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of GLCDC is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of GLCDC is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpuenptglcdc_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for GLCDC"]
pub type Mmpuenptglcdc = crate::RegValueT<Mmpuenptglcdc_SPEC>;

impl Mmpuenptglcdc {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptglcdc::Protect,
        mmpuenptglcdc::Protect,
        Mmpuenptglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptglcdc::Protect,
            mmpuenptglcdc::Protect,
            Mmpuenptglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptglcdc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptglcdc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptglcdc {
    #[inline(always)]
    fn default() -> Mmpuenptglcdc {
        <crate::RegValueT<Mmpuenptglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENGLCDC register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENGLCDC register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpurptglcdc_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for GLCDC"]
pub type Mmpurptglcdc = crate::RegValueT<Mmpurptglcdc_SPEC>;

impl Mmpurptglcdc {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptglcdc::Protect,
        mmpurptglcdc::Protect,
        Mmpurptglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptglcdc::Protect,
            mmpurptglcdc::Protect,
            Mmpurptglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptglcdc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptglcdc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptglcdc {
    #[inline(always)]
    fn default() -> Mmpurptglcdc {
        <crate::RegValueT<Mmpurptglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for GLCDC write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for GLCDC write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpuacglcdc_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for GLCDC (n = 0, 1)"]
pub type Mmpuacglcdc = crate::RegValueT<Mmpuacglcdc_SPEC>;

impl Mmpuacglcdc {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacglcdc::Enable,
        mmpuacglcdc::Enable,
        Mmpuacglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacglcdc::Enable,
            mmpuacglcdc::Enable,
            Mmpuacglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacglcdc::Rp,
        mmpuacglcdc::Rp,
        Mmpuacglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacglcdc::Rp,
            mmpuacglcdc::Rp,
            Mmpuacglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacglcdc::Wp,
        mmpuacglcdc::Wp,
        Mmpuacglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacglcdc::Wp,
            mmpuacglcdc::Wp,
            Mmpuacglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacglcdc {
    #[inline(always)]
    fn default() -> Mmpuacglcdc {
        <crate::RegValueT<Mmpuacglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "GLCDC region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "GLCDC region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpusglcdc_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for GLCDC (n = 0, 1)"]
pub type Mmpusglcdc = crate::RegValueT<Mmpusglcdc_SPEC>;

impl NoBitfieldReg<Mmpusglcdc_SPEC> for Mmpusglcdc {}
impl ::core::default::Default for Mmpusglcdc {
    #[inline(always)]
    fn default() -> Mmpusglcdc {
        <crate::RegValueT<Mmpusglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpueglcdc_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for GLCDC (n = 0, 1)"]
pub type Mmpueglcdc = crate::RegValueT<Mmpueglcdc_SPEC>;

impl NoBitfieldReg<Mmpueglcdc_SPEC> for Mmpueglcdc {}
impl ::core::default::Default for Mmpueglcdc {
    #[inline(always)]
    fn default() -> Mmpueglcdc {
        <crate::RegValueT<Mmpueglcdc_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuendrw_SPEC;
impl crate::sealed::RegSpec for Mmpuendrw_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for DRW"]
pub type Mmpuendrw = crate::RegValueT<Mmpuendrw_SPEC>;

impl Mmpuendrw {
    #[doc = "Bus master MPU of DRW Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuendrw::Enable,
        mmpuendrw::Enable,
        Mmpuendrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuendrw::Enable,
            mmpuendrw::Enable,
            Mmpuendrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuendrw_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuendrw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuendrw {
    #[inline(always)]
    fn default() -> Mmpuendrw {
        <crate::RegValueT<Mmpuendrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuendrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of DRW is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of DRW is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenpdrw_SPEC;
impl crate::sealed::RegSpec for Mmpuenpdrw_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for DRW"]
pub type Mmpuenpdrw = crate::RegValueT<Mmpuenpdrw_SPEC>;

impl Mmpuenpdrw {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenpdrw::Protect,
        mmpuenpdrw::Protect,
        Mmpuenpdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenpdrw::Protect,
            mmpuenpdrw::Protect,
            Mmpuenpdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenpdrw_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenpdrw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenpdrw {
    #[inline(always)]
    fn default() -> Mmpuenpdrw {
        <crate::RegValueT<Mmpuenpdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenpdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENDRW register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENDRW register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptdrw_SPEC;
impl crate::sealed::RegSpec for Mmpurptdrw_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for DRW"]
pub type Mmpurptdrw = crate::RegValueT<Mmpurptdrw_SPEC>;

impl Mmpurptdrw {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptdrw::Protect,
        mmpurptdrw::Protect,
        Mmpurptdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptdrw::Protect,
            mmpurptdrw::Protect,
            Mmpurptdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptdrw_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptdrw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptdrw {
    #[inline(always)]
    fn default() -> Mmpurptdrw {
        <crate::RegValueT<Mmpurptdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for DRW write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for DRW write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacdrw_SPEC;
impl crate::sealed::RegSpec for Mmpuacdrw_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for DRW (n = 0 to 2)"]
pub type Mmpuacdrw = crate::RegValueT<Mmpuacdrw_SPEC>;

impl Mmpuacdrw {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacdrw::Enable,
        mmpuacdrw::Enable,
        Mmpuacdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacdrw::Enable,
            mmpuacdrw::Enable,
            Mmpuacdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacdrw::Rp,
        mmpuacdrw::Rp,
        Mmpuacdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacdrw::Rp,
            mmpuacdrw::Rp,
            Mmpuacdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacdrw::Wp,
        mmpuacdrw::Wp,
        Mmpuacdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacdrw::Wp,
            mmpuacdrw::Wp,
            Mmpuacdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacdrw {
    #[inline(always)]
    fn default() -> Mmpuacdrw {
        <crate::RegValueT<Mmpuacdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "DRW region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "DRW region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusdrw_SPEC;
impl crate::sealed::RegSpec for Mmpusdrw_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for DRW (n = 0 to 2)"]
pub type Mmpusdrw = crate::RegValueT<Mmpusdrw_SPEC>;

impl NoBitfieldReg<Mmpusdrw_SPEC> for Mmpusdrw {}
impl ::core::default::Default for Mmpusdrw {
    #[inline(always)]
    fn default() -> Mmpusdrw {
        <crate::RegValueT<Mmpusdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuedrw_SPEC;
impl crate::sealed::RegSpec for Mmpuedrw_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for DRW (n = 0 to 2)"]
pub type Mmpuedrw = crate::RegValueT<Mmpuedrw_SPEC>;

impl NoBitfieldReg<Mmpuedrw_SPEC> for Mmpuedrw {}
impl ::core::default::Default for Mmpuedrw {
    #[inline(always)]
    fn default() -> Mmpuedrw {
        <crate::RegValueT<Mmpuedrw_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenmipid_SPEC;
impl crate::sealed::RegSpec for Mmpuenmipid_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for MIPI-DSI"]
pub type Mmpuenmipid = crate::RegValueT<Mmpuenmipid_SPEC>;

impl Mmpuenmipid {
    #[doc = "Bus master MPU of MIPI-DSI Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenmipid::Enable,
        mmpuenmipid::Enable,
        Mmpuenmipid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenmipid::Enable,
            mmpuenmipid::Enable,
            Mmpuenmipid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenmipid_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenmipid_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenmipid {
    #[inline(always)]
    fn default() -> Mmpuenmipid {
        <crate::RegValueT<Mmpuenmipid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenmipid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of MIPI-DSI is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of MIPI-DSI is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptmipid_SPEC;
impl crate::sealed::RegSpec for Mmpuenptmipid_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for MIPI-DSI"]
pub type Mmpuenptmipid = crate::RegValueT<Mmpuenptmipid_SPEC>;

impl Mmpuenptmipid {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptmipid::Protect,
        mmpuenptmipid::Protect,
        Mmpuenptmipid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptmipid::Protect,
            mmpuenptmipid::Protect,
            Mmpuenptmipid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptmipid_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptmipid_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptmipid {
    #[inline(always)]
    fn default() -> Mmpuenptmipid {
        <crate::RegValueT<Mmpuenptmipid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptmipid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENMIPID register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENMIPID register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptmipid_SPEC;
impl crate::sealed::RegSpec for Mmpurptmipid_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for MIPI-DSI"]
pub type Mmpurptmipid = crate::RegValueT<Mmpurptmipid_SPEC>;

impl Mmpurptmipid {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptmipid::Protect,
        mmpurptmipid::Protect,
        Mmpurptmipid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptmipid::Protect,
            mmpurptmipid::Protect,
            Mmpurptmipid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptmipid_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptmipid_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptmipid {
    #[inline(always)]
    fn default() -> Mmpurptmipid {
        <crate::RegValueT<Mmpurptmipid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptmipid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for MIPI-DSI write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for MIPI-DSI write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacmipid_SPEC;
impl crate::sealed::RegSpec for Mmpuacmipid_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for MIPI-DSI"]
pub type Mmpuacmipid = crate::RegValueT<Mmpuacmipid_SPEC>;

impl Mmpuacmipid {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacmipid::Enable,
        mmpuacmipid::Enable,
        Mmpuacmipid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacmipid::Enable,
            mmpuacmipid::Enable,
            Mmpuacmipid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacmipid::Rp,
        mmpuacmipid::Rp,
        Mmpuacmipid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacmipid::Rp,
            mmpuacmipid::Rp,
            Mmpuacmipid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacmipid::Wp,
        mmpuacmipid::Wp,
        Mmpuacmipid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacmipid::Wp,
            mmpuacmipid::Wp,
            Mmpuacmipid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacmipid {
    #[inline(always)]
    fn default() -> Mmpuacmipid {
        <crate::RegValueT<Mmpuacmipid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacmipid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "MIPI DSI region unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "MIPI DSI region unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusmipid_SPEC;
impl crate::sealed::RegSpec for Mmpusmipid_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for MIPI-DSI"]
pub type Mmpusmipid = crate::RegValueT<Mmpusmipid_SPEC>;

impl NoBitfieldReg<Mmpusmipid_SPEC> for Mmpusmipid {}
impl ::core::default::Default for Mmpusmipid {
    #[inline(always)]
    fn default() -> Mmpusmipid {
        <crate::RegValueT<Mmpusmipid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuemipid_SPEC;
impl crate::sealed::RegSpec for Mmpuemipid_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for MIPI-DSI"]
pub type Mmpuemipid = crate::RegValueT<Mmpuemipid_SPEC>;

impl NoBitfieldReg<Mmpuemipid_SPEC> for Mmpuemipid {}
impl ::core::default::Default for Mmpuemipid {
    #[inline(always)]
    fn default() -> Mmpuemipid {
        <crate::RegValueT<Mmpuemipid_SPEC> as RegisterValue<_>>::new(4095)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenceu_SPEC;
impl crate::sealed::RegSpec for Mmpuenceu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for CEU"]
pub type Mmpuenceu = crate::RegValueT<Mmpuenceu_SPEC>;

impl Mmpuenceu {
    #[doc = "Bus master MPU of CEU Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenceu::Enable,
        mmpuenceu::Enable,
        Mmpuenceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenceu::Enable,
            mmpuenceu::Enable,
            Mmpuenceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenceu_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenceu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenceu {
    #[inline(always)]
    fn default() -> Mmpuenceu {
        <crate::RegValueT<Mmpuenceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of CEU is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of CEU is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptceu_SPEC;
impl crate::sealed::RegSpec for Mmpuenptceu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for CEU"]
pub type Mmpuenptceu = crate::RegValueT<Mmpuenptceu_SPEC>;

impl Mmpuenptceu {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptceu::Protect,
        mmpuenptceu::Protect,
        Mmpuenptceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptceu::Protect,
            mmpuenptceu::Protect,
            Mmpuenptceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptceu_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptceu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptceu {
    #[inline(always)]
    fn default() -> Mmpuenptceu {
        <crate::RegValueT<Mmpuenptceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENCEU register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENCEU register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptceu_SPEC;
impl crate::sealed::RegSpec for Mmpurptceu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for CEU"]
pub type Mmpurptceu = crate::RegValueT<Mmpurptceu_SPEC>;

impl Mmpurptceu {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptceu::Protect,
        mmpurptceu::Protect,
        Mmpurptceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptceu::Protect,
            mmpurptceu::Protect,
            Mmpurptceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptceu_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptceu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptceu {
    #[inline(always)]
    fn default() -> Mmpurptceu {
        <crate::RegValueT<Mmpurptceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for CEU write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for CEU write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacceu_SPEC;
impl crate::sealed::RegSpec for Mmpuacceu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for CEU (n = 0, 1)"]
pub type Mmpuacceu = crate::RegValueT<Mmpuacceu_SPEC>;

impl Mmpuacceu {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacceu::Enable,
        mmpuacceu::Enable,
        Mmpuacceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacceu::Enable,
            mmpuacceu::Enable,
            Mmpuacceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacceu::Rp,
        mmpuacceu::Rp,
        Mmpuacceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacceu::Rp,
            mmpuacceu::Rp,
            Mmpuacceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacceu::Wp,
        mmpuacceu::Wp,
        Mmpuacceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacceu::Wp,
            mmpuacceu::Wp,
            Mmpuacceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacceu {
    #[inline(always)]
    fn default() -> Mmpuacceu {
        <crate::RegValueT<Mmpuacceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "CEU region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CEU region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusceu_SPEC;
impl crate::sealed::RegSpec for Mmpusceu_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for CEU (n = 0, 1)"]
pub type Mmpusceu = crate::RegValueT<Mmpusceu_SPEC>;

impl NoBitfieldReg<Mmpusceu_SPEC> for Mmpusceu {}
impl ::core::default::Default for Mmpusceu {
    #[inline(always)]
    fn default() -> Mmpusceu {
        <crate::RegValueT<Mmpusceu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueceu_SPEC;
impl crate::sealed::RegSpec for Mmpueceu_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for CEU (n = 0, 1)"]
pub type Mmpueceu = crate::RegValueT<Mmpueceu_SPEC>;

impl NoBitfieldReg<Mmpueceu_SPEC> for Mmpueceu {}
impl ::core::default::Default for Mmpueceu {
    #[inline(always)]
    fn default() -> Mmpueceu {
        <crate::RegValueT<Mmpueceu_SPEC> as RegisterValue<_>>::new(4095)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenmipic_SPEC;
impl crate::sealed::RegSpec for Mmpuenmipic_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for MIPI-CSI by VIN"]
pub type Mmpuenmipic = crate::RegValueT<Mmpuenmipic_SPEC>;

impl Mmpuenmipic {
    #[doc = "Bus master MPU of MIPI-CSI Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenmipic::Enable,
        mmpuenmipic::Enable,
        Mmpuenmipic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenmipic::Enable,
            mmpuenmipic::Enable,
            Mmpuenmipic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenmipic_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenmipic_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenmipic {
    #[inline(always)]
    fn default() -> Mmpuenmipic {
        <crate::RegValueT<Mmpuenmipic_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenmipic {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of MIPI-CSI is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of MIPI-CSI is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptmipic_SPEC;
impl crate::sealed::RegSpec for Mmpuenptmipic_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for MIPI-CSI by VIN"]
pub type Mmpuenptmipic = crate::RegValueT<Mmpuenptmipic_SPEC>;

impl Mmpuenptmipic {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptmipic::Protect,
        mmpuenptmipic::Protect,
        Mmpuenptmipic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptmipic::Protect,
            mmpuenptmipic::Protect,
            Mmpuenptmipic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptmipic_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptmipic_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptmipic {
    #[inline(always)]
    fn default() -> Mmpuenptmipic {
        <crate::RegValueT<Mmpuenptmipic_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptmipic {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENMIPIC register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENMIPIC register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptmipic_SPEC;
impl crate::sealed::RegSpec for Mmpurptmipic_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for MIPI-CSI by VIN"]
pub type Mmpurptmipic = crate::RegValueT<Mmpurptmipic_SPEC>;

impl Mmpurptmipic {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptmipic::Protect,
        mmpurptmipic::Protect,
        Mmpurptmipic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptmipic::Protect,
            mmpurptmipic::Protect,
            Mmpurptmipic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptmipic_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptmipic_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptmipic {
    #[inline(always)]
    fn default() -> Mmpurptmipic {
        <crate::RegValueT<Mmpurptmipic_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptmipic {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for MIPI-CSI write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for MIPI-CSI write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacmipic_SPEC;
impl crate::sealed::RegSpec for Mmpuacmipic_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for MIPI-CSI via VIN (n = 0 to 2)"]
pub type Mmpuacmipic = crate::RegValueT<Mmpuacmipic_SPEC>;

impl Mmpuacmipic {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacmipic::Enable,
        mmpuacmipic::Enable,
        Mmpuacmipic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacmipic::Enable,
            mmpuacmipic::Enable,
            Mmpuacmipic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacmipic::Rp,
        mmpuacmipic::Rp,
        Mmpuacmipic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacmipic::Rp,
            mmpuacmipic::Rp,
            Mmpuacmipic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacmipic::Wp,
        mmpuacmipic::Wp,
        Mmpuacmipic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacmipic::Wp,
            mmpuacmipic::Wp,
            Mmpuacmipic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacmipic {
    #[inline(always)]
    fn default() -> Mmpuacmipic {
        <crate::RegValueT<Mmpuacmipic_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacmipic {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "MIPI region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "MIPI region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusmipic_SPEC;
impl crate::sealed::RegSpec for Mmpusmipic_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for MIPI-CSI via VIN (n = 0 to 2)"]
pub type Mmpusmipic = crate::RegValueT<Mmpusmipic_SPEC>;

impl NoBitfieldReg<Mmpusmipic_SPEC> for Mmpusmipic {}
impl ::core::default::Default for Mmpusmipic {
    #[inline(always)]
    fn default() -> Mmpusmipic {
        <crate::RegValueT<Mmpusmipic_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuemipic_SPEC;
impl crate::sealed::RegSpec for Mmpuemipic_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for MIPI-CSI via VIN (n = 0 to 2)"]
pub type Mmpuemipic = crate::RegValueT<Mmpuemipic_SPEC>;

impl NoBitfieldReg<Mmpuemipic_SPEC> for Mmpuemipic {}
impl ::core::default::Default for Mmpuemipic {
    #[inline(always)]
    fn default() -> Mmpuemipic {
        <crate::RegValueT<Mmpuemipic_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuennpu_SPEC;
impl crate::sealed::RegSpec for Mmpuennpu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Register for NPU"]
pub type Mmpuennpu = crate::RegValueT<Mmpuennpu_SPEC>;

impl Mmpuennpu {
    #[doc = "Bus master MPU of NPU Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuennpu::Enable,
        mmpuennpu::Enable,
        Mmpuennpu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuennpu::Enable,
            mmpuennpu::Enable,
            Mmpuennpu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuennpu_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuennpu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuennpu {
    #[inline(always)]
    fn default() -> Mmpuennpu {
        <crate::RegValueT<Mmpuennpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuennpu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of NPU is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU of NPU is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptnpu_SPEC;
impl crate::sealed::RegSpec for Mmpuenptnpu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Enable Protect Register for NPU"]
pub type Mmpuenptnpu = crate::RegValueT<Mmpuenptnpu_SPEC>;

impl Mmpuenptnpu {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptnpu::Protect,
        mmpuenptnpu::Protect,
        Mmpuenptnpu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptnpu::Protect,
            mmpuenptnpu::Protect,
            Mmpuenptnpu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptnpu_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptnpu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptnpu {
    #[inline(always)]
    fn default() -> Mmpuenptnpu {
        <crate::RegValueT<Mmpuenptnpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptnpu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENNPU register write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MMPUENNPU register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptnpu_SPEC;
impl crate::sealed::RegSpec for Mmpurptnpu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Regions Protect Register for NPU"]
pub type Mmpurptnpu = crate::RegValueT<Mmpurptnpu_SPEC>;

impl Mmpurptnpu {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptnpu::Protect,
        mmpurptnpu::Protect,
        Mmpurptnpu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptnpu::Protect,
            mmpurptnpu::Protect,
            Mmpurptnpu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptnpu_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptnpu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptnpu {
    #[inline(always)]
    fn default() -> Mmpurptnpu {
        <crate::RegValueT<Mmpurptnpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptnpu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for NPU write is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus master MPU register for NPU write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacnpu_SPEC;
impl crate::sealed::RegSpec for Mmpuacnpu_SPEC {
    type DataType = u16;
}

#[doc = "MMPU Access Control Register for NPU (n = 0 to 4)"]
pub type Mmpuacnpu = crate::RegValueT<Mmpuacnpu_SPEC>;

impl Mmpuacnpu {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacnpu::Enable,
        mmpuacnpu::Enable,
        Mmpuacnpu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacnpu::Enable,
            mmpuacnpu::Enable,
            Mmpuacnpu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacnpu::Rp,
        mmpuacnpu::Rp,
        Mmpuacnpu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacnpu::Rp,
            mmpuacnpu::Rp,
            Mmpuacnpu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacnpu::Wp,
        mmpuacnpu::Wp,
        Mmpuacnpu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacnpu::Wp,
            mmpuacnpu::Wp,
            Mmpuacnpu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacnpu {
    #[inline(always)]
    fn default() -> Mmpuacnpu {
        <crate::RegValueT<Mmpuacnpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacnpu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "NPU region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "NPU region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusnpu_SPEC;
impl crate::sealed::RegSpec for Mmpusnpu_SPEC {
    type DataType = u32;
}

#[doc = "MMPU Start Address Register for NPU (n = 0 to 4)"]
pub type Mmpusnpu = crate::RegValueT<Mmpusnpu_SPEC>;

impl NoBitfieldReg<Mmpusnpu_SPEC> for Mmpusnpu {}
impl ::core::default::Default for Mmpusnpu {
    #[inline(always)]
    fn default() -> Mmpusnpu {
        <crate::RegValueT<Mmpusnpu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenpu_SPEC;
impl crate::sealed::RegSpec for Mmpuenpu_SPEC {
    type DataType = u32;
}

#[doc = "MMPU End Address Register for NPU (n = 0 to 4)"]
pub type Mmpuenpu = crate::RegValueT<Mmpuenpu_SPEC>;

impl NoBitfieldReg<Mmpuenpu_SPEC> for Mmpuenpu {}
impl ::core::default::Default for Mmpuenpu {
    #[inline(always)]
    fn default() -> Mmpuenpu {
        <crate::RegValueT<Mmpuenpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
