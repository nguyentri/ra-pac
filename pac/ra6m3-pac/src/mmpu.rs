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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:12:21 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Bus Master MPU"]
unsafe impl ::core::marker::Send for super::Mmpu {}
unsafe impl ::core::marker::Sync for super::Mmpu {}
impl super::Mmpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Bus Master MPU Control Register"]
    #[inline(always)]
    pub const fn mmpuctl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuctl_SPEC, crate::common::RW>,
        3,
        0x400,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuctla(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuctlb(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuctlc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x800usize),
            )
        }
    }

    #[doc = "Group A Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuaca(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuaca0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca8(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca9(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca16(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca17(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca18(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca19(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca20(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca21(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca22(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca23(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca24(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x380usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca25(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x390usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca26(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca27(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca28(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca29(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca30(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca31(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f0usize),
            )
        }
    }

    #[doc = "Group B Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuacb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacb0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacb1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacb2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacb3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacb4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x640usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacb5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x650usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacb6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x660usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacb7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x670usize),
            )
        }
    }

    #[doc = "Group C Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuacc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa00usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuacc0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacc1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacc2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacc3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacc4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacc5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacc6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuacc7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa70usize),
            )
        }
    }

    #[doc = "Group A Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusa(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusa0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa8(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa9(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa16(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa17(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa18(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa19(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa20(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa21(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa22(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa23(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa24(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x384usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa25(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x394usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa26(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa27(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa28(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa29(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa30(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa31(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f4usize),
            )
        }
    }

    #[doc = "Group B Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusb0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusb1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusb2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusb3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusb4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x644usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusb5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x654usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusb6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x664usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusb7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x674usize),
            )
        }
    }

    #[doc = "Group C Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa04usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusc0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusc1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusc2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusc3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusc4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusc5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusc6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusc7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa74usize),
            )
        }
    }

    #[doc = "Group A Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuea(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuea0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea8(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea9(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea16(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea17(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea18(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea19(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea20(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea21(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea22(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea23(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea24(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x388usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea25(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x398usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea26(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea27(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea28(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea29(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea30(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea31(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f8usize),
            )
        }
    }

    #[doc = "Group B Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpueb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }
    #[inline(always)]
    pub const fn mmpueb0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueb1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x618usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueb2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueb3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x638usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueb4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x648usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueb5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x658usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueb6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x668usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpueb7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpueb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x678usize),
            )
        }
    }

    #[doc = "Group C Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuec(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa08usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuec0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuec1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuec2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuec3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuec4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuec5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuec6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuec7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa78usize),
            )
        }
    }

    #[doc = "Group A Protection of Register"]
    #[inline(always)]
    pub const fn mmpupta(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpupta_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpupta_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }

    #[doc = "Group B Protection of Register"]
    #[inline(always)]
    pub const fn mmpuptb(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuptb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuptb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1282usize),
            )
        }
    }

    #[doc = "Group C protection of register"]
    #[inline(always)]
    pub const fn mmpuptc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuptc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuptc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2306usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuctl_SPEC;
impl crate::sealed::RegSpec for Mmpuctl_SPEC {
    type DataType = u16;
}

#[doc = "Bus Master MPU Control Register"]
pub type Mmpuctl = crate::RegValueT<Mmpuctl_SPEC>;

impl Mmpuctl {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        mmpuctl::Key,
        mmpuctl::Key,
        Mmpuctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            mmpuctl::Key,
            mmpuctl::Key,
            Mmpuctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Operation after detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuctl::Oad,
        mmpuctl::Oad,
        Mmpuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuctl::Oad,
            mmpuctl::Oad,
            Mmpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master Group enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuctl::Enable,
        mmpuctl::Enable,
        Mmpuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuctl::Enable,
            mmpuctl::Enable,
            Mmpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuctl {
    #[inline(always)]
    fn default() -> Mmpuctl {
        <crate::RegValueT<Mmpuctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the OAD and ENABLE bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Master Group is disabled. Permission of all regions."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master Group is enabled. Protection of all regions."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuaca_SPEC;
impl crate::sealed::RegSpec for Mmpuaca_SPEC {
    type DataType = u16;
}

#[doc = "Group A Region %s Access Control Register"]
pub type Mmpuaca = crate::RegValueT<Mmpuaca_SPEC>;

impl Mmpuaca {
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuaca::Wp,
        mmpuaca::Wp,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuaca::Wp,
            mmpuaca::Wp,
            Mmpuaca_SPEC,
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
        mmpuaca::Rp,
        mmpuaca::Rp,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuaca::Rp,
            mmpuaca::Rp,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Region enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuaca::Enable,
        mmpuaca::Enable,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuaca::Enable,
            mmpuaca::Enable,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuaca {
    #[inline(always)]
    fn default() -> Mmpuaca {
        <crate::RegValueT<Mmpuaca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuaca {

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
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group m Region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Group m Region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacb_SPEC;
impl crate::sealed::RegSpec for Mmpuacb_SPEC {
    type DataType = u16;
}

#[doc = "Group B Region %s Access Control Register"]
pub type Mmpuacb = crate::RegValueT<Mmpuacb_SPEC>;

impl Mmpuacb {
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacb::Wp,
        mmpuacb::Wp,
        Mmpuacb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacb::Wp,
            mmpuacb::Wp,
            Mmpuacb_SPEC,
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
        mmpuacb::Rp,
        mmpuacb::Rp,
        Mmpuacb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacb::Rp,
            mmpuacb::Rp,
            Mmpuacb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Region enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacb::Enable,
        mmpuacb::Enable,
        Mmpuacb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacb::Enable,
            mmpuacb::Enable,
            Mmpuacb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacb {
    #[inline(always)]
    fn default() -> Mmpuacb {
        <crate::RegValueT<Mmpuacb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacb {

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
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group m Region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Group m Region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacc_SPEC;
impl crate::sealed::RegSpec for Mmpuacc_SPEC {
    type DataType = u16;
}

#[doc = "Group C Region %s Access Control Register"]
pub type Mmpuacc = crate::RegValueT<Mmpuacc_SPEC>;

impl Mmpuacc {
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacc::Wp,
        mmpuacc::Wp,
        Mmpuacc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacc::Wp,
            mmpuacc::Wp,
            Mmpuacc_SPEC,
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
        mmpuacc::Rp,
        mmpuacc::Rp,
        Mmpuacc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacc::Rp,
            mmpuacc::Rp,
            Mmpuacc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Region enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacc::Enable,
        mmpuacc::Enable,
        Mmpuacc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacc::Enable,
            mmpuacc::Enable,
            Mmpuacc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacc {
    #[inline(always)]
    fn default() -> Mmpuacc {
        <crate::RegValueT<Mmpuacc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacc {

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
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group m Region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Group m Region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusa_SPEC;
impl crate::sealed::RegSpec for Mmpusa_SPEC {
    type DataType = u32;
}

#[doc = "Group A Region %s Start Address Register"]
pub type Mmpusa = crate::RegValueT<Mmpusa_SPEC>;

impl Mmpusa {
    #[doc = "Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpusa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpusa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusa {
    #[inline(always)]
    fn default() -> Mmpusa {
        <crate::RegValueT<Mmpusa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusb_SPEC;
impl crate::sealed::RegSpec for Mmpusb_SPEC {
    type DataType = u32;
}

#[doc = "Group B Region %s Start Address Register"]
pub type Mmpusb = crate::RegValueT<Mmpusb_SPEC>;

impl Mmpusb {
    #[doc = "Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpusb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpusb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusb {
    #[inline(always)]
    fn default() -> Mmpusb {
        <crate::RegValueT<Mmpusb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusc_SPEC;
impl crate::sealed::RegSpec for Mmpusc_SPEC {
    type DataType = u32;
}

#[doc = "Group C Region %s Start Address Register"]
pub type Mmpusc = crate::RegValueT<Mmpusc_SPEC>;

impl Mmpusc {
    #[doc = "Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpusc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpusc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusc {
    #[inline(always)]
    fn default() -> Mmpusc {
        <crate::RegValueT<Mmpusc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuea_SPEC;
impl crate::sealed::RegSpec for Mmpuea_SPEC {
    type DataType = u32;
}

#[doc = "Group A Region %s End Address Register"]
pub type Mmpuea = crate::RegValueT<Mmpuea_SPEC>;

impl Mmpuea {
    #[doc = "Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuea(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpuea_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpuea_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuea {
    #[inline(always)]
    fn default() -> Mmpuea {
        <crate::RegValueT<Mmpuea_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueb_SPEC;
impl crate::sealed::RegSpec for Mmpueb_SPEC {
    type DataType = u32;
}

#[doc = "Group B Region %s End Address Register"]
pub type Mmpueb = crate::RegValueT<Mmpueb_SPEC>;

impl Mmpueb {
    #[doc = "Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpueb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpueb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpueb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpueb {
    #[inline(always)]
    fn default() -> Mmpueb {
        <crate::RegValueT<Mmpueb_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuec_SPEC;
impl crate::sealed::RegSpec for Mmpuec_SPEC {
    type DataType = u32;
}

#[doc = "Group C Region %s End Address Register"]
pub type Mmpuec = crate::RegValueT<Mmpuec_SPEC>;

impl Mmpuec {
    #[doc = "Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuec(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpuec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpuec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuec {
    #[inline(always)]
    fn default() -> Mmpuec {
        <crate::RegValueT<Mmpuec_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpupta_SPEC;
impl crate::sealed::RegSpec for Mmpupta_SPEC {
    type DataType = u16;
}

#[doc = "Group A Protection of Register"]
pub type Mmpupta = crate::RegValueT<Mmpupta_SPEC>;

impl Mmpupta {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        mmpupta::Key,
        mmpupta::Key,
        Mmpupta_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            mmpupta::Key,
            mmpupta::Key,
            Mmpupta_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Protection of register(MMPUSAn, MMPUEAn and MMPUACAn)"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpupta::Protect,
        mmpupta::Protect,
        Mmpupta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpupta::Protect,
            mmpupta::Protect,
            Mmpupta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpupta {
    #[inline(always)]
    fn default() -> Mmpupta {
        <crate::RegValueT<Mmpupta_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpupta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Master MPU Group A register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "All Bus Master MPU Group A register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuptb_SPEC;
impl crate::sealed::RegSpec for Mmpuptb_SPEC {
    type DataType = u16;
}

#[doc = "Group B Protection of Register"]
pub type Mmpuptb = crate::RegValueT<Mmpuptb_SPEC>;

impl Mmpuptb {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        mmpuptb::Key,
        mmpuptb::Key,
        Mmpuptb_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            mmpuptb::Key,
            mmpuptb::Key,
            Mmpuptb_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Protection of register(MMPUSBn, MMPUEBn and MMPUACBn)"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuptb::Protect,
        mmpuptb::Protect,
        Mmpuptb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuptb::Protect,
            mmpuptb::Protect,
            Mmpuptb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuptb {
    #[inline(always)]
    fn default() -> Mmpuptb {
        <crate::RegValueT<Mmpuptb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuptb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Master MPU Group B register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "All Bus Master MPU Group B register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuptc_SPEC;
impl crate::sealed::RegSpec for Mmpuptc_SPEC {
    type DataType = u16;
}

#[doc = "Group C protection of register"]
pub type Mmpuptc = crate::RegValueT<Mmpuptc_SPEC>;

impl Mmpuptc {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        mmpuptc::Key,
        mmpuptc::Key,
        Mmpuptc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            mmpuptc::Key,
            mmpuptc::Key,
            Mmpuptc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Protection of register (MMPUSCn, MMPUECn and MMPUACCn)"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuptc::Protect,
        mmpuptc::Protect,
        Mmpuptc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuptc::Protect,
            mmpuptc::Protect,
            Mmpuptc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuptc {
    #[inline(always)]
    fn default() -> Mmpuptc {
        <crate::RegValueT<Mmpuptc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuptc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Master MPU Group C register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "All Bus Master MPU Group C register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
