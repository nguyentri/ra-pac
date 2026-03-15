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
#[doc = r"Graphics LCD Controller"]
unsafe impl ::core::marker::Send for super::Glcdc {}
unsafe impl ::core::marker::Sync for super::Glcdc {}
impl super::Glcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Color Palette 0 Plane for Graphics 1 Plane"]
    #[inline(always)]
    pub const fn gr1_clut0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_0_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_1_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_2_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_3_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_4_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_5_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_6_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_7_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_8_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_9_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_10_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_11_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_12_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_13_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_14_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_15_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_16_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_17_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_18_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_19_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_20_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_21_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_22_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_23_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_24_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_25_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_26_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_27_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_28_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_29_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_30_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_31_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_32_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_33_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_34_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_35_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_36_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_37_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_38_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_39_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_40_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_41_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_42_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_43_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_44_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_45_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_46_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_47_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_48_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_49_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_50_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_51_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_52_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_53_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_54_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_55_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_56_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_57_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_58_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_59_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_60_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_61_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_62_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_63_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_64_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_65_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_66_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_67_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_68_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_69_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_70_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_71_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_72_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_73_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_74_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_75_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_76_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_77_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_78_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_79_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_80_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_81_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_82_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_83_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_84_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_85_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_86_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x158usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_87_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x15cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_88_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_89_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_90_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_91_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_92_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_93_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_94_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_95_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_96_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_97_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_98_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_99_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_100_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x190usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_101_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x194usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_102_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x198usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_103_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x19cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_104_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_105_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_106_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_107_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_108_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_109_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_110_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_111_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_112_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_113_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_114_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_115_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_116_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_117_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_118_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_119_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_120_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_121_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_122_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_123_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_124_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_125_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_126_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_127_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_128_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_129_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_130_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_131_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_132_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_133_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_134_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_135_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_136_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_137_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_138_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_139_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_140_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_141_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_142_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_143_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_144_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_145_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_146_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_147_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_148_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_149_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_150_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_151_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x25cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_152_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_153_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_154_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_155_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_156_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_157_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_158_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_159_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_160_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_161_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_162_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_163_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_164_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_165_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_166_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_167_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_168_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_169_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_170_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_171_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_172_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_173_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_174_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_175_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_176_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_177_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_178_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_179_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_180_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_181_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_182_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_183_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_184_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_185_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_186_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_187_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_188_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_189_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_190_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_191_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_192_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_193_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_194_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_195_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_196_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_197_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_198_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_199_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_200_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_201_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_202_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_203_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_204_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_205_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_206_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_207_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_208_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_209_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_210_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_211_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_212_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_213_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_214_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_215_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_216_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_217_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_218_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_219_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_220_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_221_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_222_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_223_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_224_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x380usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_225_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x384usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_226_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x388usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_227_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_228_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x390usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_229_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x394usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_230_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x398usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_231_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x39cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_232_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_233_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_234_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_235_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_236_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_237_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_238_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_239_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_240_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_241_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_242_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_243_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_244_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_245_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_246_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_247_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_248_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_249_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_250_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_251_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_252_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_253_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_254_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0_255_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3fcusize),
            )
        }
    }

    #[doc = "Color Palette 1 Plane for Graphics 1 Plane"]
    #[inline(always)]
    pub const fn gr1_clut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_0_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_1_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_2_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_3_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_4_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_5_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x414usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_6_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_7_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x41cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_8_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_9_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_10_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_11_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_12_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_13_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x434usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_14_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x438usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_15_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x43cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_16_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_17_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_18_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_19_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_20_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_21_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x454usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_22_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x458usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_23_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x45cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_24_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_25_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_26_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_27_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x46cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_28_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x470usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_29_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x474usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_30_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x478usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_31_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x47cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_32_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x480usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_33_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x484usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_34_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x488usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_35_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_36_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x490usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_37_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x494usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_38_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x498usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_39_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x49cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_40_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_41_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_42_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_43_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_44_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_45_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_46_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_47_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_48_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_49_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_50_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_51_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_52_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_53_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_54_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_55_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_56_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_57_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_58_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_59_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_60_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_61_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_62_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_63_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_64_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x500usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_65_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x504usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_66_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_67_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_68_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_69_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x514usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_70_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x518usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_71_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_72_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x520usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_73_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x524usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_74_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x528usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_75_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_76_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x530usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_77_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x534usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_78_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x538usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_79_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_80_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x540usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_81_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x544usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_82_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x548usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_83_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_84_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x550usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_85_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x554usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_86_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x558usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_87_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_88_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x560usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_89_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x564usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_90_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x568usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_91_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_92_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x570usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_93_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x574usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_94_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x578usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_95_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_96_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x580usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_97_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x584usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_98_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x588usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_99_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_100_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x590usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_101_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x594usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_102_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x598usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_103_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_104_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_105_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_106_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_107_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_108_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_109_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_110_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_111_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_112_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_113_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_114_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_115_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_116_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_117_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_118_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_119_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_120_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_121_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_122_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_123_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_124_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_125_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_126_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_127_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_128_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_129_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_130_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_131_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_132_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_133_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_134_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x618usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_135_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_136_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_137_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_138_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_139_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_140_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_141_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_142_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x638usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_143_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_144_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x640usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_145_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x644usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_146_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x648usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_147_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_148_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x650usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_149_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x654usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_150_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x658usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_151_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_152_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x660usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_153_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x664usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_154_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x668usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_155_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_156_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x670usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_157_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x674usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_158_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x678usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_159_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_160_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x680usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_161_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x684usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_162_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x688usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_163_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_164_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x690usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_165_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x694usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_166_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x698usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_167_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_168_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_169_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_170_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_171_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_172_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_173_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_174_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_175_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_176_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_177_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_178_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_179_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_180_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_181_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_182_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_183_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_184_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_185_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_186_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_187_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_188_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_189_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_190_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_191_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_192_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x700usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_193_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x704usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_194_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x708usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_195_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_196_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x710usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_197_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x714usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_198_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x718usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_199_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x71cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_200_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x720usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_201_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x724usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_202_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x728usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_203_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x72cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_204_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x730usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_205_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x734usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_206_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x738usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_207_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x73cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_208_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x740usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_209_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x744usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_210_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x748usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_211_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_212_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x750usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_213_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x754usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_214_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x758usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_215_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x75cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_216_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x760usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_217_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x764usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_218_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x768usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_219_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x76cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_220_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x770usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_221_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x774usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_222_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x778usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_223_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x77cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_224_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x780usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_225_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x784usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_226_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x788usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_227_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_228_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x790usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_229_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x794usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_230_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x798usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_231_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x79cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_232_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_233_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_234_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_235_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_236_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_237_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_238_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_239_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_240_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_241_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_242_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_243_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_244_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_245_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_246_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_247_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_248_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_249_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_250_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_251_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_252_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_253_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_254_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1_255_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7fcusize),
            )
        }
    }

    #[doc = "Color Palette 0 Plane for Graphics 2 Plane"]
    #[inline(always)]
    pub const fn gr2_clut0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x800usize))
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_0_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_1_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_2_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_3_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_4_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x810usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_5_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_6_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x818usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_7_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x81cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_8_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x820usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_9_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_10_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x828usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_11_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x82cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_12_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x830usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_13_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_14_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x838usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_15_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x83cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_16_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_17_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_18_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x848usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_19_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_20_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x850usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_21_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_22_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x858usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_23_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x85cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_24_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x860usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_25_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_26_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x868usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_27_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x86cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_28_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x870usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_29_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x874usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_30_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x878usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_31_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x87cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_32_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x880usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_33_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x884usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_34_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x888usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_35_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_36_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x890usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_37_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x894usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_38_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x898usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_39_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x89cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_40_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_41_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_42_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_43_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_44_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_45_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_46_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_47_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_48_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_49_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_50_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_51_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_52_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_53_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_54_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_55_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_56_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_57_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_58_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_59_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_60_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_61_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_62_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_63_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_64_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x900usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_65_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x904usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_66_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x908usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_67_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_68_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x910usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_69_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x914usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_70_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x918usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_71_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x91cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_72_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x920usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_73_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x924usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_74_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x928usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_75_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x92cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_76_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x930usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_77_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x934usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_78_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x938usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_79_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x93cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_80_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x940usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_81_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x944usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_82_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x948usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_83_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_84_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x950usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_85_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x954usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_86_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x958usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_87_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x95cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_88_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x960usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_89_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x964usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_90_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x968usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_91_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x96cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_92_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x970usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_93_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x974usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_94_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x978usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_95_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x97cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_96_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x980usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_97_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x984usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_98_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x988usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_99_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_100_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x990usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_101_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x994usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_102_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x998usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_103_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x99cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_104_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_105_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_106_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_107_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_108_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_109_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_110_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_111_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_112_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_113_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_114_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_115_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_116_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_117_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_118_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_119_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_120_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_121_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_122_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_123_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_124_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_125_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_126_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_127_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_128_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_129_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_130_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_131_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_132_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_133_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_134_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_135_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_136_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_137_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_138_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_139_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_140_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_141_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_142_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_143_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_144_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_145_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_146_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_147_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_148_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_149_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_150_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_151_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_152_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_153_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_154_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_155_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_156_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_157_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_158_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_159_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_160_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_161_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_162_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_163_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_164_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_165_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_166_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_167_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_168_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_169_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_170_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_171_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_172_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_173_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_174_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_175_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xabcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_176_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xac0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_177_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xac4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_178_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xac8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_179_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_180_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xad0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_181_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xad4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_182_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xad8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_183_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xadcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_184_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xae0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_185_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xae4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_186_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xae8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_187_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_188_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_189_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_190_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_191_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xafcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_192_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_193_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_194_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_195_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_196_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_197_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_198_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_199_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_200_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_201_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_202_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_203_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_204_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_205_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_206_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_207_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_208_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_209_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_210_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_211_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_212_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_213_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_214_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_215_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_216_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_217_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_218_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_219_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_220_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_221_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_222_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_223_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_224_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_225_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_226_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_227_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_228_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_229_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_230_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_231_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_232_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xba0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_233_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xba4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_234_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xba8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_235_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_236_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_237_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_238_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_239_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_240_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_241_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_242_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_243_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_244_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_245_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_246_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_247_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_248_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_249_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_250_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_251_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_252_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_253_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_254_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0_255_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbfcusize),
            )
        }
    }

    #[doc = "Color Palette 1 Plane for Graphics 2 Plane"]
    #[inline(always)]
    pub const fn gr2_clut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc00usize))
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_0_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_1_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_2_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_3_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_4_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_5_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_6_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_7_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_8_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_9_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_10_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_11_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_12_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_13_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_14_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_15_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_16_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_17_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_18_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_19_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_20_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_21_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_22_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_23_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_24_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_25_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_26_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_27_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_28_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_29_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_30_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_31_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_32_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_33_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_34_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_35_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_36_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_37_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_38_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_39_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_40_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xca0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_41_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xca4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_42_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xca8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_43_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_44_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_45_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_46_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_47_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_48_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_49_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_50_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_51_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_52_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_53_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_54_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_55_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_56_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xce0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_57_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xce4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_58_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xce8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_59_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_60_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_61_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_62_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_63_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_64_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_65_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_66_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_67_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_68_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_69_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_70_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_71_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_72_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_73_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_74_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_75_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_76_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_77_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_78_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_79_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_80_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_81_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_82_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_83_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_84_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_85_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_86_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_87_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_88_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_89_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_90_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_91_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_92_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_93_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_94_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_95_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_96_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_97_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_98_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_99_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_100_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_101_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_102_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_103_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_104_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xda0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_105_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xda4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_106_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xda8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_107_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_108_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_109_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_110_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_111_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_112_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_113_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_114_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_115_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_116_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_117_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_118_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_119_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xddcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_120_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xde0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_121_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xde4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_122_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xde8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_123_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_124_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_125_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_126_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_127_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_128_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_129_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_130_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_131_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_132_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_133_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_134_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_135_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_136_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_137_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_138_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_139_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_140_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_141_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_142_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_143_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_144_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_145_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_146_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_147_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_148_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_149_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_150_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_151_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_152_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_153_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_154_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_155_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_156_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_157_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_158_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_159_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_160_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_161_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_162_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_163_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_164_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_165_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_166_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_167_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_168_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xea0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_169_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xea4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_170_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xea8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_171_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_172_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_173_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_174_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_175_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xebcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_176_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xec0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_177_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xec4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_178_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xec8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_179_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_180_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xed0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_181_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xed4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_182_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xed8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_183_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xedcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_184_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xee0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_185_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xee4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_186_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xee8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_187_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_188_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xef0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_189_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xef4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_190_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xef8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_191_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xefcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_192_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_193_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_194_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_195_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_196_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_197_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_198_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_199_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_200_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_201_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_202_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_203_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_204_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_205_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_206_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_207_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_208_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_209_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_210_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_211_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_212_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_213_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_214_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_215_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_216_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_217_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_218_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_219_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_220_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_221_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_222_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_223_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_224_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_225_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_226_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_227_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_228_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_229_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_230_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_231_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_232_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_233_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_234_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_235_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_236_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_237_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_238_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_239_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_240_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_241_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_242_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_243_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_244_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_245_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_246_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_247_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_248_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_249_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_250_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_251_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_252_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xff0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_253_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xff4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_254_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xff8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1_255_(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xffcusize),
            )
        }
    }

    #[doc = "Background Plane Setting Operation Control Register"]
    #[inline(always)]
    pub const fn bg_en(&self) -> &'static crate::common::Reg<self::BgEn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgEn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

    #[doc = "Background Plane Setting Free-Running Period Register"]
    #[inline(always)]
    pub const fn bg_peri(
        &self,
    ) -> &'static crate::common::Reg<self::BgPeri_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgPeri_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4100usize),
            )
        }
    }

    #[doc = "Background Plane Setting Synchronization Position Register"]
    #[inline(always)]
    pub const fn bg_sync(
        &self,
    ) -> &'static crate::common::Reg<self::BgSync_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgSync_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4104usize),
            )
        }
    }

    #[doc = "Background Plane Setting Full Image Vertical Size Register"]
    #[inline(always)]
    pub const fn bg_vsize(
        &self,
    ) -> &'static crate::common::Reg<self::BgVsize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgVsize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4108usize),
            )
        }
    }

    #[doc = "Background Plane Setting Full Image Horizontal Size Register"]
    #[inline(always)]
    pub const fn bg_hsize(
        &self,
    ) -> &'static crate::common::Reg<self::BgHsize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgHsize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4112usize),
            )
        }
    }

    #[doc = "Background Plane Setting Background Color Register"]
    #[inline(always)]
    pub const fn bg_bgc(&self) -> &'static crate::common::Reg<self::BgBgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgBgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

    #[doc = "Background Plane Setting Status Monitor Register"]
    #[inline(always)]
    pub const fn bg_mon(&self) -> &'static crate::common::Reg<self::BgMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::BgMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4120usize),
            )
        }
    }

    #[doc = "Graphics %s  Register Update Control Register"]
    #[inline(always)]
    pub const fn gr_ven(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrVen_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1100usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ven(
        &self,
    ) -> &'static crate::common::Reg<self::GrVen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrVen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ven(
        &self,
    ) -> &'static crate::common::Reg<self::GrVen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrVen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1200usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Read Control Register"]
    #[inline(always)]
    pub const fn gr_flmrd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlmrd_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1104usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flmrd(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlmrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlmrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flmrd(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlmrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlmrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1204usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 1"]
    #[inline(always)]
    pub const fn gr_flm1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm1_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1108usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm1(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm1(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1208usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 2"]
    #[inline(always)]
    pub const fn gr_flm2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm2_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x110cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm2(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm2(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120cusize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 3"]
    #[inline(always)]
    pub const fn gr_flm3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm3_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1110usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm3(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm3(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1210usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 5"]
    #[inline(always)]
    pub const fn gr_flm5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm5_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1118usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm5(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm5(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1218usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 6"]
    #[inline(always)]
    pub const fn gr_flm6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm6_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x111cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm6(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x111cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm6(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x121cusize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 1"]
    #[inline(always)]
    pub const fn gr_ab1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb1_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1120usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab1(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab1(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1220usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 2"]
    #[inline(always)]
    pub const fn gr_ab2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb2_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1124usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab2(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab2(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1224usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 3"]
    #[inline(always)]
    pub const fn gr_ab3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb3_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1128usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab3(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab3(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1228usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 4"]
    #[inline(always)]
    pub const fn gr_ab4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb4_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x112cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab4(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x112cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab4(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x122cusize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 5"]
    #[inline(always)]
    pub const fn gr_ab5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb5_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1130usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab5(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab5(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1230usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 6"]
    #[inline(always)]
    pub const fn gr_ab6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb6_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1134usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab6(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab6(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1234usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 7"]
    #[inline(always)]
    pub const fn gr_ab7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb7_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1138usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab7(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab7(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1238usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 8"]
    #[inline(always)]
    pub const fn gr_ab8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb8_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x113cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab8(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x113cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab8(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x123cusize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 9"]
    #[inline(always)]
    pub const fn gr_ab9(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb9_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1140usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab9(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab9(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1240usize),
            )
        }
    }

    #[doc = "Graphics %s Background Color Control Register"]
    #[inline(always)]
    pub const fn gr_base(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrBase_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x114cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_base(
        &self,
    ) -> &'static crate::common::Reg<self::GrBase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrBase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_base(
        &self,
    ) -> &'static crate::common::Reg<self::GrBase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrBase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124cusize),
            )
        }
    }

    #[doc = "Graphics %s CLUT Table Interrupt Control Register"]
    #[inline(always)]
    pub const fn gr_clutint(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrClutint_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1150usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_clutint(
        &self,
    ) -> &'static crate::common::Reg<self::GrClutint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrClutint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clutint(
        &self,
    ) -> &'static crate::common::Reg<self::GrClutint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrClutint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1250usize),
            )
        }
    }

    #[doc = "Graphics %s Status Monitor Register"]
    #[inline(always)]
    pub const fn gr_mon(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrMon_SPEC, crate::common::R>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1154usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_mon(&self) -> &'static crate::common::Reg<self::GrMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_mon(&self) -> &'static crate::common::Reg<self::GrMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1254usize),
            )
        }
    }

    #[doc = "Gamma %s Register Update Control Register"]
    #[inline(always)]
    pub const fn gam_latch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLatch_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1300usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GamLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GamLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GamLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1380usize),
            )
        }
    }

    #[doc = "Gamma Correction Block Function Switch Register"]
    #[inline(always)]
    pub const fn gam_sw(&self) -> &'static crate::common::Reg<self::GamSw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamSw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4868usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gam_lut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut1_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1308usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1388usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gam_lut2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut2_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x130cusize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138cusize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gam_lut3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut3_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1310usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1390usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gam_lut4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut4_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1314usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1394usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gam_lut5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut5_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1318usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1398usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gam_lut6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut6_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x131cusize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x131cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x135cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x139cusize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gam_lut7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut7_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1320usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13a0usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gam_lut8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut8_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1324usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1364usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GamLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13a4usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gam_area1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea1_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1328usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13a8usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gam_area2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea2_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x132cusize))
        }
    }
    #[inline(always)]
    pub const fn gamg_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x132cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x136cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13acusize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gam_area3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea3_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1330usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13b0usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gam_area4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea4_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1334usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13b4usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gam_area5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea5_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1338usize))
        }
    }
    #[inline(always)]
    pub const fn gamg_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamb_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gamr_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GamArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13b8usize),
            )
        }
    }

    #[doc = "Output Control Block Register Update Control Register"]
    #[inline(always)]
    pub const fn out_vlatch(
        &self,
    ) -> &'static crate::common::Reg<self::OutVlatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutVlatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5056usize),
            )
        }
    }

    #[doc = "Output Control Block Output Interface Register"]
    #[inline(always)]
    pub const fn out_set(
        &self,
    ) -> &'static crate::common::Reg<self::OutSet_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutSet_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5060usize),
            )
        }
    }

    #[doc = "Output Control Block Brightness Correction Register 1"]
    #[inline(always)]
    pub const fn out_bright1(
        &self,
    ) -> &'static crate::common::Reg<self::OutBright1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutBright1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5064usize),
            )
        }
    }

    #[doc = "Output Control Block Brightness Correction Register 2"]
    #[inline(always)]
    pub const fn out_bright2(
        &self,
    ) -> &'static crate::common::Reg<self::OutBright2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutBright2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5068usize),
            )
        }
    }

    #[doc = "Output Control Block Contrast Correction Register"]
    #[inline(always)]
    pub const fn out_contrast(
        &self,
    ) -> &'static crate::common::Reg<self::OutContrast_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutContrast_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5072usize),
            )
        }
    }

    #[doc = "Output Control Block Panel Dither Correction Register"]
    #[inline(always)]
    pub const fn out_pdtha(
        &self,
    ) -> &'static crate::common::Reg<self::OutPdtha_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutPdtha_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5076usize),
            )
        }
    }

    #[doc = "Output Control Block Output Phase Control Register"]
    #[inline(always)]
    pub const fn out_clkphase(
        &self,
    ) -> &'static crate::common::Reg<self::OutClkphase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutClkphase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5092usize),
            )
        }
    }

    #[doc = "TCON VLATCH Register"]
    #[inline(always)]
    pub const fn tcon_vlatch(
        &self,
    ) -> &'static crate::common::Reg<self::TconVlatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconVlatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5120usize),
            )
        }
    }

    #[doc = "TCON Reference Timing Setting Register"]
    #[inline(always)]
    pub const fn tcon_tim(
        &self,
    ) -> &'static crate::common::Reg<self::TconTim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconTim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5124usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register %s1"]
    #[inline(always)]
    pub const fn tcon_stv1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconStv1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1408usize))
        }
    }
    #[inline(always)]
    pub const fn tcon_stva1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStv1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStv1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tcon_stvb1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStv1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStv1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1410usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register %s2"]
    #[inline(always)]
    pub const fn tcon_stv2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconStv2_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140cusize))
        }
    }
    #[inline(always)]
    pub const fn tcon_stva2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStv2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStv2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x140cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tcon_stvb2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStv2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStv2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1414usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register STH%s1"]
    #[inline(always)]
    pub const fn tcon_sth1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconSth1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1418usize))
        }
    }
    #[inline(always)]
    pub const fn tcon_stha1(
        &self,
    ) -> &'static crate::common::Reg<self::TconSth1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSth1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tcon_sthb1(
        &self,
    ) -> &'static crate::common::Reg<self::TconSth1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSth1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1420usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register STH%s2"]
    #[inline(always)]
    pub const fn tcon_sth2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconSth2_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x141cusize))
        }
    }
    #[inline(always)]
    pub const fn tcon_stha2(
        &self,
    ) -> &'static crate::common::Reg<self::TconSth2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSth2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x141cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tcon_sthb2(
        &self,
    ) -> &'static crate::common::Reg<self::TconSth2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSth2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1424usize),
            )
        }
    }

    #[doc = "TCON Data Enable Polarity Setting Register"]
    #[inline(always)]
    pub const fn tcon_de(
        &self,
    ) -> &'static crate::common::Reg<self::TconDe_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconDe_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5160usize),
            )
        }
    }

    #[doc = "System Control Block State Detection Control Register"]
    #[inline(always)]
    pub const fn syscnt_dtcten(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntDtcten_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntDtcten_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5184usize),
            )
        }
    }

    #[doc = "System Control Block Interrupt Request Enable Control Register"]
    #[inline(always)]
    pub const fn syscnt_inten(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntInten_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntInten_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5188usize),
            )
        }
    }

    #[doc = "System Control Block Status Clear Register"]
    #[inline(always)]
    pub const fn syscnt_stclr(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntStclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntStclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5192usize),
            )
        }
    }

    #[doc = "System Control Block Status Monitor Register"]
    #[inline(always)]
    pub const fn syscnt_stmon(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntStmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SyscntStmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5196usize),
            )
        }
    }

    #[doc = "System Control Block Version and Panel Clock Control Register"]
    #[inline(always)]
    pub const fn syscnt_panel_clk(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntPanelClk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntPanelClk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5200usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr1Clut0_SPEC;
impl crate::sealed::RegSpec for Gr1Clut0_SPEC {
    type DataType = u32;
}

#[doc = "Color Palette 0 Plane for Graphics 1 Plane"]
pub type Gr1Clut0 = crate::RegValueT<Gr1Clut0_SPEC>;

impl Gr1Clut0 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr1Clut0 {
    #[inline(always)]
    fn default() -> Gr1Clut0 {
        <crate::RegValueT<Gr1Clut0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr1Clut1_SPEC;
impl crate::sealed::RegSpec for Gr1Clut1_SPEC {
    type DataType = u32;
}

#[doc = "Color Palette 1 Plane for Graphics 1 Plane"]
pub type Gr1Clut1 = crate::RegValueT<Gr1Clut1_SPEC>;

impl Gr1Clut1 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr1Clut1 {
    #[inline(always)]
    fn default() -> Gr1Clut1 {
        <crate::RegValueT<Gr1Clut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr2Clut0_SPEC;
impl crate::sealed::RegSpec for Gr2Clut0_SPEC {
    type DataType = u32;
}

#[doc = "Color Palette 0 Plane for Graphics 2 Plane"]
pub type Gr2Clut0 = crate::RegValueT<Gr2Clut0_SPEC>;

impl Gr2Clut0 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr2Clut0 {
    #[inline(always)]
    fn default() -> Gr2Clut0 {
        <crate::RegValueT<Gr2Clut0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr2Clut1_SPEC;
impl crate::sealed::RegSpec for Gr2Clut1_SPEC {
    type DataType = u32;
}

#[doc = "Color Palette 1 Plane for Graphics 2 Plane"]
pub type Gr2Clut1 = crate::RegValueT<Gr2Clut1_SPEC>;

impl Gr2Clut1 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr2Clut1 {
    #[inline(always)]
    fn default() -> Gr2Clut1 {
        <crate::RegValueT<Gr2Clut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgEn_SPEC;
impl crate::sealed::RegSpec for BgEn_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Operation Control Register"]
pub type BgEn = crate::RegValueT<BgEn_SPEC>;

impl BgEn {
    #[doc = "Background plane generation module operation enable"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bg_en::En,
        bg_en::En,
        BgEn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bg_en::En,
            bg_en::En,
            BgEn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control of LCDC internal register value reflection to internal operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bg_en::Ven,
        bg_en::Ven,
        BgEn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bg_en::Ven,
            bg_en::Ven,
            BgEn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Entire module SW reset control"]
    #[inline(always)]
    pub fn swrst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bg_en::Swrst,
        bg_en::Swrst,
        BgEn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bg_en::Swrst,
            bg_en::Swrst,
            BgEn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, BgEn_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,BgEn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgEn {
    #[inline(always)]
    fn default() -> BgEn {
        <crate::RegValueT<BgEn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bg_en {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Enables operation."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables operation."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Enables"]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables(Cleared to 0 by an internal source)"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "Releases the entire module from the SW reset state."]
        pub const _1: Self = Self::new(1);

        #[doc = "Places the entire module in the SW reset state."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgPeri_SPEC;
impl crate::sealed::RegSpec for BgPeri_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Free-Running Period Register"]
pub type BgPeri = crate::RegValueT<BgPeri_SPEC>;

impl BgPeri {
    #[doc = "Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn fh(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, BgPeri_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,BgPeri_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background plane vertical synchronization signal period on the basis of line."]
    #[inline(always)]
    pub fn fv(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, BgPeri_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,BgPeri_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, BgPeri_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,BgPeri_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgPeri {
    #[inline(always)]
    fn default() -> BgPeri {
        <crate::RegValueT<BgPeri_SPEC> as RegisterValue<_>>::new(1507351)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgSync_SPEC;
impl crate::sealed::RegSpec for BgSync_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Synchronization Position Register"]
pub type BgSync = crate::RegValueT<BgSync_SPEC>;

impl BgSync {
    #[doc = "Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        bg_sync::Hp,
        bg_sync::Hp,
        BgSync_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            bg_sync::Hp,
            bg_sync::Hp,
            BgSync_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Background plane vertical synchronization signal assertion position on the basis of line."]
    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        bg_sync::Vp,
        bg_sync::Vp,
        BgSync_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            bg_sync::Vp,
            bg_sync::Vp,
            BgSync_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, BgSync_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xfff,1,0,u16,u16,BgSync_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgSync {
    #[inline(always)]
    fn default() -> BgSync {
        <crate::RegValueT<BgSync_SPEC> as RegisterValue<_>>::new(65537)
    }
}
pub mod bg_sync {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hp_SPEC;
    pub type Hp = crate::EnumBitfieldStruct<u8, Hp_SPEC>;
    impl Hp {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vp_SPEC;
    pub type Vp = crate::EnumBitfieldStruct<u8, Vp_SPEC>;
    impl Vp {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgVsize_SPEC;
impl crate::sealed::RegSpec for BgVsize_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Full Image Vertical Size Register"]
pub type BgVsize = crate::RegValueT<BgVsize_SPEC>;

impl BgVsize {
    #[doc = "Background plane vertical valid pixel width on the basis of line"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background plane vertical valid pixel start position on the basis of line"]
    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, BgVsize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgVsize {
    #[inline(always)]
    fn default() -> BgVsize {
        <crate::RegValueT<BgVsize_SPEC> as RegisterValue<_>>::new(458768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgHsize_SPEC;
impl crate::sealed::RegSpec for BgHsize_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Full Image Horizontal Size Register"]
pub type BgHsize = crate::RegValueT<BgHsize_SPEC>;

impl BgHsize {
    #[doc = "Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field."]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, BgHsize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgHsize {
    #[inline(always)]
    fn default() -> BgHsize {
        <crate::RegValueT<BgHsize_SPEC> as RegisterValue<_>>::new(393232)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgBgc_SPEC;
impl crate::sealed::RegSpec for BgBgc_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Background Color Register"]
pub type BgBgc = crate::RegValueT<BgBgc_SPEC>;

impl BgBgc {
    #[doc = "B value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R value for background plane valid pixel area.Unsigned; 8-bit integer."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgBgc {
    #[inline(always)]
    fn default() -> BgBgc {
        <crate::RegValueT<BgBgc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgMon_SPEC;
impl crate::sealed::RegSpec for BgMon_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Status Monitor Register"]
pub type BgMon = crate::RegValueT<BgMon_SPEC>;

impl BgMon {
    #[doc = "Background plane generation module operation state monitor."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bg_mon::En,
        bg_mon::En,
        BgMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bg_mon::En,
            bg_mon::En,
            BgMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Entire module internal operation reflection control signal monitor.The signal  state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal."]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bg_mon::Ven,
        bg_mon::Ven,
        BgMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bg_mon::Ven,
            bg_mon::Ven,
            BgMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Entire module SW reset state monitor."]
    #[inline(always)]
    pub fn swrst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bg_mon::Swrst,
        bg_mon::Swrst,
        BgMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bg_mon::Swrst,
            bg_mon::Swrst,
            BgMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, BgMon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,BgMon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BgMon {
    #[inline(always)]
    fn default() -> BgMon {
        <crate::RegValueT<BgMon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bg_mon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Operation is in progress."]
        pub const _1: Self = Self::new(1);

        #[doc = "Operation is stopped."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is asserted."]
        pub const _1: Self = Self::new(1);

        #[doc = "The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is negated."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "The entire module is released from the SW reset state."]
        pub const _1: Self = Self::new(1);

        #[doc = "The entire module is in the SW reset state."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrVen_SPEC;
impl crate::sealed::RegSpec for GrVen_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s  Register Update Control Register"]
pub type GrVen = crate::RegValueT<GrVen_SPEC>;

impl GrVen {
    #[doc = "Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn pven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_ven::Pven,
        gr_ven::Pven,
        GrVen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_ven::Pven,
            gr_ven::Pven,
            GrVen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, GrVen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,GrVen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrVen {
    #[inline(always)]
    fn default() -> GrVen {
        <crate::RegValueT<GrVen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ven {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pven_SPEC;
    pub type Pven = crate::EnumBitfieldStruct<u8, Pven_SPEC>;
    impl Pven {
        #[doc = "Enables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlmrd_SPEC;
impl crate::sealed::RegSpec for GrFlmrd_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Read Control Register"]
pub type GrFlmrd = crate::RegValueT<GrFlmrd_SPEC>;

impl GrFlmrd {
    #[doc = "Graphics data (frame buffer data) read enable."]
    #[inline(always)]
    pub fn renb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_flmrd::Renb,
        gr_flmrd::Renb,
        GrFlmrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_flmrd::Renb,
            gr_flmrd::Renb,
            GrFlmrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, GrFlmrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,GrFlmrd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlmrd {
    #[inline(always)]
    fn default() -> GrFlmrd {
        <crate::RegValueT<GrFlmrd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_flmrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Renb_SPEC;
    pub type Renb = crate::EnumBitfieldStruct<u8, Renb_SPEC>;
    impl Renb {
        #[doc = "Enables reading."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables reading."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm1_SPEC;
impl crate::sealed::RegSpec for GrFlm1_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 1"]
pub type GrFlm1 = crate::RegValueT<GrFlm1_SPEC>;

impl GrFlm1 {
    #[doc = "Burst transfer control for graphics data (frame buffer data)access"]
    #[inline(always)]
    pub fn bstmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gr_flm1::Bstmd,
        gr_flm1::Bstmd,
        GrFlm1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gr_flm1::Bstmd,
            gr_flm1::Bstmd,
            GrFlm1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000000000000000000. The write value should be 000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, u32, GrFlm1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fffffff,1,0,u32,u32,GrFlm1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm1 {
    #[inline(always)]
    fn default() -> GrFlm1 {
        <crate::RegValueT<GrFlm1_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod gr_flm1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bstmd_SPEC;
    pub type Bstmd = crate::EnumBitfieldStruct<u8, Bstmd_SPEC>;
    impl Bstmd {
        #[doc = "16-beat increment burst transfer (64-byte boundary)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm2_SPEC;
impl crate::sealed::RegSpec for GrFlm2_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 2"]
pub type GrFlm2 = crate::RegValueT<GrFlm2_SPEC>;

impl GrFlm2 {
    #[doc = "Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\\[5:0\\] should be fixed to 0 during 64-byte burst transfer."]
    #[inline(always)]
    pub fn base(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, GrFlm2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,GrFlm2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm2 {
    #[inline(always)]
    fn default() -> GrFlm2 {
        <crate::RegValueT<GrFlm2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm3_SPEC;
impl crate::sealed::RegSpec for GrFlm3_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 3"]
pub type GrFlm3 = crate::RegValueT<GrFlm3_SPEC>;

impl GrFlm3 {
    #[doc = "Macro line offset address for accessing graphics data(frame buffer data)Signed; 16-bit integer"]
    #[inline(always)]
    pub fn lnoff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, GrFlm3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,GrFlm3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm3 {
    #[inline(always)]
    fn default() -> GrFlm3 {
        <crate::RegValueT<GrFlm3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm5_SPEC;
impl crate::sealed::RegSpec for GrFlm5_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 5"]
pub type GrFlm5 = crate::RegValueT<GrFlm5_SPEC>;

impl GrFlm5 {
    #[doc = "Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)"]
    #[inline(always)]
    pub fn datanum(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, GrFlm5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,GrFlm5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Number of lines per frame for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    pub fn lnnum(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrFlm5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrFlm5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm5 {
    #[inline(always)]
    fn default() -> GrFlm5 {
        <crate::RegValueT<GrFlm5_SPEC> as RegisterValue<_>>::new(983040)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm6_SPEC;
impl crate::sealed::RegSpec for GrFlm6_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 6"]
pub type GrFlm6 = crate::RegValueT<GrFlm6_SPEC>;

impl GrFlm6 {
    #[doc = "Data format for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    pub fn format(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        gr_flm6::Format,
        gr_flm6::Format,
        GrFlm6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            gr_flm6::Format,
            gr_flm6::Format,
            GrFlm6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, GrFlm6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, GrFlm6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for GrFlm6 {
    #[inline(always)]
    fn default() -> GrFlm6 {
        <crate::RegValueT<GrFlm6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_flm6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        #[doc = "CLUT11bit/pix)"]
        pub const _111: Self = Self::new(7);

        #[doc = "CLUT4 (4 bits/pix)"]
        pub const _110: Self = Self::new(6);

        #[doc = "CLUT8 (8 bits/pix)"]
        pub const _101: Self = Self::new(5);

        #[doc = "ARGB8888 (32 bits/pix)"]
        pub const _100: Self = Self::new(4);

        #[doc = "ARGB4444 (16 bits/pix)"]
        pub const _011: Self = Self::new(3);

        #[doc = "ARGB1555 (16 bits/pix, 1 bit of A is LUT data)"]
        pub const _010: Self = Self::new(2);

        #[doc = "RGB888 (32 bits/pix, 8 bits on the MSB side are invalid)"]
        pub const _001: Self = Self::new(1);

        #[doc = "RGB565 (16 bits/pix)"]
        pub const _000: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb1_SPEC;
impl crate::sealed::RegSpec for GrAb1_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 1"]
pub type GrAb1 = crate::RegValueT<GrAb1_SPEC>;

impl GrAb1 {
    #[doc = "Graphics display plane control."]
    #[inline(always)]
    pub fn dispsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gr_ab1::Dispsel,
        gr_ab1::Dispsel,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gr_ab1::Dispsel,
            gr_ab1::Dispsel,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics image area border display control."]
    #[inline(always)]
    pub fn grcdispon(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gr_ab1::Grcdispon,
        gr_ab1::Grcdispon,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gr_ab1::Grcdispon,
            gr_ab1::Grcdispon,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Image area border display control for rectangular area alpha blending."]
    #[inline(always)]
    pub fn arcdispon(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gr_ab1::Arcdispon,
        gr_ab1::Arcdispon,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gr_ab1::Arcdispon,
            gr_ab1::Arcdispon,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Rectangular area alpha blending control."]
    #[inline(always)]
    pub fn arcon(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gr_ab1::Arcon,
        gr_ab1::Arcon,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gr_ab1::Arcon,
            gr_ab1::Arcon,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000. The write value should be 0000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7ffff, 1, 0, u32, u32, GrAb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7ffff,1,0,u32,u32,GrAb1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb1 {
    #[inline(always)]
    fn default() -> GrAb1 {
        <crate::RegValueT<GrAb1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dispsel_SPEC;
    pub type Dispsel = crate::EnumBitfieldStruct<u8, Dispsel_SPEC>;
    impl Dispsel {
        #[doc = "Blended display of lower-layer graphics (input image from the previous stage) and current graphics (graphics data read from the AHB bus)"]
        pub const _11: Self = Self::new(3);

        #[doc = "Current graphics display"]
        pub const _10: Self = Self::new(2);

        #[doc = "Lower-layer graphics display"]
        pub const _01: Self = Self::new(1);

        #[doc = "Background color display (value set by the GRn_BASE register)."]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcdispon_SPEC;
    pub type Grcdispon = crate::EnumBitfieldStruct<u8, Grcdispon_SPEC>;
    impl Grcdispon {
        #[doc = "Display on"]
        pub const _1: Self = Self::new(1);

        #[doc = "Display off"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcdispon_SPEC;
    pub type Arcdispon = crate::EnumBitfieldStruct<u8, Arcdispon_SPEC>;
    impl Arcdispon {
        #[doc = "Display on"]
        pub const _1: Self = Self::new(1);

        #[doc = "Display off"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcon_SPEC;
    pub type Arcon = crate::EnumBitfieldStruct<u8, Arcon_SPEC>;
    impl Arcon {
        #[doc = "On"]
        pub const _1: Self = Self::new(1);

        #[doc = "Off"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb2_SPEC;
impl crate::sealed::RegSpec for GrAb2_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 2"]
pub type GrAb2 = crate::RegValueT<GrAb2_SPEC>;

impl GrAb2 {
    #[doc = "Vertical width of graphics image area."]
    #[inline(always)]
    pub fn grcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical start position of graphics image area."]
    #[inline(always)]
    pub fn grcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb2 {
    #[inline(always)]
    fn default() -> GrAb2 {
        <crate::RegValueT<GrAb2_SPEC> as RegisterValue<_>>::new(393232)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb3_SPEC;
impl crate::sealed::RegSpec for GrAb3_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 3"]
pub type GrAb3 = crate::RegValueT<GrAb3_SPEC>;

impl GrAb3 {
    #[doc = "Horizontal width of graphics image area."]
    #[inline(always)]
    pub fn grchw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal start position of graphics image area."]
    #[inline(always)]
    pub fn grchs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb3 {
    #[inline(always)]
    fn default() -> GrAb3 {
        <crate::RegValueT<GrAb3_SPEC> as RegisterValue<_>>::new(327696)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb4_SPEC;
impl crate::sealed::RegSpec for GrAb4_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 4"]
pub type GrAb4 = crate::RegValueT<GrAb4_SPEC>;

impl GrAb4 {
    #[doc = "Vertical width of rectangular area alpha blending image area."]
    #[inline(always)]
    pub fn arcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical start position of rectangular area alpha blending image area"]
    #[inline(always)]
    pub fn arcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb4 {
    #[inline(always)]
    fn default() -> GrAb4 {
        <crate::RegValueT<GrAb4_SPEC> as RegisterValue<_>>::new(393232)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb5_SPEC;
impl crate::sealed::RegSpec for GrAb5_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 5"]
pub type GrAb5 = crate::RegValueT<GrAb5_SPEC>;

impl GrAb5 {
    #[doc = "Horizontal width of rectangular area alpha blending image area."]
    #[inline(always)]
    pub fn archw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal start position of rectangular area alpha blending image area."]
    #[inline(always)]
    pub fn archs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb5 {
    #[inline(always)]
    fn default() -> GrAb5 {
        <crate::RegValueT<GrAb5_SPEC> as RegisterValue<_>>::new(327696)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb6_SPEC;
impl crate::sealed::RegSpec for GrAb6_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 6"]
pub type GrAb6 = crate::RegValueT<GrAb6_SPEC>;

impl GrAb6 {
    #[doc = "Frame rate for alpha blending in rectangular area."]
    #[inline(always)]
    pub fn arcrate(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha coefficient for alpha blending in rectangular area (-255 to 255).\\[8\\]: Sign (0: addition, 1: subtraction)\\[7:0\\]: Variation (absolute value)"]
    #[inline(always)]
    pub fn arccoef(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, GrAb6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, u8, GrAb6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x7f,1,0,u8,u8,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb6 {
    #[inline(always)]
    fn default() -> GrAb6 {
        <crate::RegValueT<GrAb6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb7_SPEC;
impl crate::sealed::RegSpec for GrAb7_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 7"]
pub type GrAb7 = crate::RegValueT<GrAb7_SPEC>;

impl GrAb7 {
    #[doc = "RGB-index chroma-key processing control."]
    #[inline(always)]
    pub fn ckon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_ab7::Ckon,
        gr_ab7::Ckon,
        GrAb7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_ab7::Ckon,
            gr_ab7::Ckon,
            GrAb7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Initial alpha value for alpha blending in rectangular area."]
    #[inline(always)]
    pub fn arcdef(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrAb7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb7 {
    #[inline(always)]
    fn default() -> GrAb7 {
        <crate::RegValueT<GrAb7_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab7 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckon_SPEC;
    pub type Ckon = crate::EnumBitfieldStruct<u8, Ckon_SPEC>;
    impl Ckon {
        #[doc = "Enables chroma-key processing"]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables chroma-key processing"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb8_SPEC;
impl crate::sealed::RegSpec for GrAb8_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 8"]
pub type GrAb8 = crate::RegValueT<GrAb8_SPEC>;

impl GrAb8 {
    #[doc = "R signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb8 {
    #[inline(always)]
    fn default() -> GrAb8 {
        <crate::RegValueT<GrAb8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb9_SPEC;
impl crate::sealed::RegSpec for GrAb9_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 9"]
pub type GrAb9 = crate::RegValueT<GrAb9_SPEC>;

impl GrAb9 {
    #[doc = "R value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A value after RGB-index chroma-key processing replacement."]
    #[inline(always)]
    pub fn cka(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb9 {
    #[inline(always)]
    fn default() -> GrAb9 {
        <crate::RegValueT<GrAb9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrBase_SPEC;
impl crate::sealed::RegSpec for GrBase_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Background Color Control Register"]
pub type GrBase = crate::RegValueT<GrBase_SPEC>;

impl GrBase {
    #[doc = "Background color R valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background color B valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background color G valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrBase {
    #[inline(always)]
    fn default() -> GrBase {
        <crate::RegValueT<GrBase_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrClutint_SPEC;
impl crate::sealed::RegSpec for GrClutint_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s CLUT Table Interrupt Control Register"]
pub type GrClutint = crate::RegValueT<GrClutint_SPEC>;

impl GrClutint {
    #[doc = "Number of detection lines"]
    #[inline(always)]
    pub fn line(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrClutint_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrClutint_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CLUT table control"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gr_clutint::Sel,
        gr_clutint::Sel,
        GrClutint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gr_clutint::Sel,
            gr_clutint::Sel,
            GrClutint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, GrClutint_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,GrClutint_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrClutint {
    #[inline(always)]
    fn default() -> GrClutint {
        <crate::RegValueT<GrClutint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_clutint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Uses CLUT1 plane for internal operations."]
        pub const _1: Self = Self::new(1);

        #[doc = "Uses CLUT0 plane for internal operations."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrMon_SPEC;
impl crate::sealed::RegSpec for GrMon_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Status Monitor Register"]
pub type GrMon = crate::RegValueT<GrMon_SPEC>;

impl GrMon {
    #[doc = "Status monitor for alpha blending in rectangular area"]
    #[inline(always)]
    pub fn arcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_mon::Arcst,
        gr_mon::Arcst,
        GrMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_mon::Arcst,
            gr_mon::Arcst,
            GrMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Status monitor for underflow"]
    #[inline(always)]
    pub fn undflst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gr_mon::Undflst,
        gr_mon::Undflst,
        GrMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gr_mon::Undflst,
            gr_mon::Undflst,
            GrMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, GrMon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,GrMon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for GrMon {
    #[inline(always)]
    fn default() -> GrMon {
        <crate::RegValueT<GrMon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_mon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcst_SPEC;
    pub type Arcst = crate::EnumBitfieldStruct<u8, Arcst_SPEC>;
    impl Arcst {
        #[doc = "Fade-in/fade-out is in progress."]
        pub const _1: Self = Self::new(1);

        #[doc = "Fade-in/fade-out is not in progress."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Undflst_SPEC;
    pub type Undflst = crate::EnumBitfieldStruct<u8, Undflst_SPEC>;
    impl Undflst {
        #[doc = "An underflow occurs in internal operations."]
        pub const _1: Self = Self::new(1);

        #[doc = "No underflow occurs in internal operations."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLatch_SPEC;
impl crate::sealed::RegSpec for GamLatch_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Register Update Control Register"]
pub type GamLatch = crate::RegValueT<GamLatch_SPEC>;

impl GamLatch {
    #[doc = "Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gam_latch::Ven,
        gam_latch::Ven,
        GamLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gam_latch::Ven,
            gam_latch::Ven,
            GamLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, GamLatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            GamLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GamLatch {
    #[inline(always)]
    fn default() -> GamLatch {
        <crate::RegValueT<GamLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamSw_SPEC;
impl crate::sealed::RegSpec for GamSw_SPEC {
    type DataType = u32;
}

#[doc = "Gamma Correction Block Function Switch Register"]
pub type GamSw = crate::RegValueT<GamSw_SPEC>;

impl GamSw {
    #[doc = "Gamma correction on/off control"]
    #[inline(always)]
    pub fn gamon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gam_sw::Gamon,
        gam_sw::Gamon,
        GamSw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gam_sw::Gamon,
            gam_sw::Gamon,
            GamSw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, GamSw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,GamSw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamSw {
    #[inline(always)]
    fn default() -> GamSw {
        <crate::RegValueT<GamSw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_sw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gamon_SPEC;
    pub type Gamon = crate::EnumBitfieldStruct<u8, Gamon_SPEC>;
    impl Gamon {
        #[doc = "Turns on gamma correction."]
        pub const _1: Self = Self::new(1);

        #[doc = "Turns off gamma correction."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut1_SPEC;
impl crate::sealed::RegSpec for GamLut1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 1"]
pub type GamLut1 = crate::RegValueT<GamLut1_SPEC>;

impl GamLut1 {
    #[doc = "Gain value of area 1Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 0.Unsigned 11-bit fixed point."]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut1 {
    #[inline(always)]
    fn default() -> GamLut1 {
        <crate::RegValueT<GamLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut2_SPEC;
impl crate::sealed::RegSpec for GamLut2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 2"]
pub type GamLut2 = crate::RegValueT<GamLut2_SPEC>;

impl GamLut2 {
    #[doc = "Gain value of area 3Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 2Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut2 {
    #[inline(always)]
    fn default() -> GamLut2 {
        <crate::RegValueT<GamLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut3_SPEC;
impl crate::sealed::RegSpec for GamLut3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 3"]
pub type GamLut3 = crate::RegValueT<GamLut3_SPEC>;

impl GamLut3 {
    #[doc = "Gain value of area 5Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 4Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut3 {
    #[inline(always)]
    fn default() -> GamLut3 {
        <crate::RegValueT<GamLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut4_SPEC;
impl crate::sealed::RegSpec for GamLut4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 4"]
pub type GamLut4 = crate::RegValueT<GamLut4_SPEC>;

impl GamLut4 {
    #[doc = "Gain value of area 7Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 6Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut4 {
    #[inline(always)]
    fn default() -> GamLut4 {
        <crate::RegValueT<GamLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut5_SPEC;
impl crate::sealed::RegSpec for GamLut5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 5"]
pub type GamLut5 = crate::RegValueT<GamLut5_SPEC>;

impl GamLut5 {
    #[doc = "Gain value of area 9Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 8Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut5 {
    #[inline(always)]
    fn default() -> GamLut5 {
        <crate::RegValueT<GamLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut6_SPEC;
impl crate::sealed::RegSpec for GamLut6_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 6"]
pub type GamLut6 = crate::RegValueT<GamLut6_SPEC>;

impl GamLut6 {
    #[doc = "Gain value of area 11Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 10Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut6 {
    #[inline(always)]
    fn default() -> GamLut6 {
        <crate::RegValueT<GamLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut7_SPEC;
impl crate::sealed::RegSpec for GamLut7_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 7"]
pub type GamLut7 = crate::RegValueT<GamLut7_SPEC>;

impl GamLut7 {
    #[doc = "Gain value of area 13Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 12Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain12(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut7 {
    #[inline(always)]
    fn default() -> GamLut7 {
        <crate::RegValueT<GamLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut8_SPEC;
impl crate::sealed::RegSpec for GamLut8_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Table Setting Register 8"]
pub type GamLut8 = crate::RegValueT<GamLut8_SPEC>;

impl GamLut8 {
    #[doc = "Gain value of area 15Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain value of area 14Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut8 {
    #[inline(always)]
    fn default() -> GamLut8 {
        <crate::RegValueT<GamLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea1_SPEC;
impl crate::sealed::RegSpec for GamArea1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Area Setting Register 1"]
pub type GamArea1 = crate::RegValueT<GamArea1_SPEC>;

impl GamArea1 {
    #[doc = "Start threshold of area 3Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 2Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 1Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea1 {
    #[inline(always)]
    fn default() -> GamArea1 {
        <crate::RegValueT<GamArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea2_SPEC;
impl crate::sealed::RegSpec for GamArea2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Area Setting Register 2"]
pub type GamArea2 = crate::RegValueT<GamArea2_SPEC>;

impl GamArea2 {
    #[doc = "Start threshold of area 6Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 5Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 4Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea2 {
    #[inline(always)]
    fn default() -> GamArea2 {
        <crate::RegValueT<GamArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea3_SPEC;
impl crate::sealed::RegSpec for GamArea3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Area Setting Register 3"]
pub type GamArea3 = crate::RegValueT<GamArea3_SPEC>;

impl GamArea3 {
    #[doc = "Start threshold of area 9Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 8Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 7Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea3 {
    #[inline(always)]
    fn default() -> GamArea3 {
        <crate::RegValueT<GamArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea4_SPEC;
impl crate::sealed::RegSpec for GamArea4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Area Setting Register 4"]
pub type GamArea4 = crate::RegValueT<GamArea4_SPEC>;

impl GamArea4 {
    #[doc = "Start threshold of area 12Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 11Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 10Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea4 {
    #[inline(always)]
    fn default() -> GamArea4 {
        <crate::RegValueT<GamArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea5_SPEC;
impl crate::sealed::RegSpec for GamArea5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma %s Correction Block Area Setting Register 5"]
pub type GamArea5 = crate::RegValueT<GamArea5_SPEC>;

impl GamArea5 {
    #[doc = "Start threshold of area 15Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 14Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start threshold of area 13Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea5 {
    #[inline(always)]
    fn default() -> GamArea5 {
        <crate::RegValueT<GamArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutVlatch_SPEC;
impl crate::sealed::RegSpec for OutVlatch_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Register Update Control Register"]
pub type OutVlatch = crate::RegValueT<OutVlatch_SPEC>;

impl OutVlatch {
    #[doc = "Control of output control module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        out_vlatch::Ven,
        out_vlatch::Ven,
        OutVlatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            out_vlatch::Ven,
            out_vlatch::Ven,
            OutVlatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7fffffff,
        1,
        0,
        u32,
        u32,
        OutVlatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            OutVlatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OutVlatch {
    #[inline(always)]
    fn default() -> OutVlatch {
        <crate::RegValueT<OutVlatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_vlatch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet_SPEC;
impl crate::sealed::RegSpec for OutSet_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Output Interface Register"]
pub type OutSet = crate::RegValueT<OutSet_SPEC>;

impl OutSet {
    #[doc = "Data delay in serial RGB format (based on OUTCLK)"]
    #[inline(always)]
    pub fn phase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        out_set::Phase,
        out_set::Phase,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            out_set::Phase,
            out_set::Phase,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Invalid data position control in serial RGB format"]
    #[inline(always)]
    pub fn dirsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        out_set::Dirsel,
        out_set::Dirsel,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            out_set::Dirsel,
            out_set::Dirsel,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock frequency division control"]
    #[inline(always)]
    pub fn frqsel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        out_set::Frqsel,
        out_set::Frqsel,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            out_set::Frqsel,
            out_set::Frqsel,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output format select"]
    #[inline(always)]
    pub fn format(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        out_set::Format,
        out_set::Format,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            out_set::Format,
            out_set::Format,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pixel order control"]
    #[inline(always)]
    pub fn swapon(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        out_set::Swapon,
        out_set::Swapon,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            out_set::Swapon,
            out_set::Swapon,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit endian change control"]
    #[inline(always)]
    pub fn endianon(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        out_set::Endianon,
        out_set::Endianon,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            out_set::Endianon,
            out_set::Endianon,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, OutSet_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,OutSet_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutSet {
    #[inline(always)]
    fn default() -> OutSet {
        <crate::RegValueT<OutSet_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_set {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phase_SPEC;
    pub type Phase = crate::EnumBitfieldStruct<u8, Phase_SPEC>;
    impl Phase {
        #[doc = "3 cycles"]
        pub const _11: Self = Self::new(3);

        #[doc = "2 cycles"]
        pub const _10: Self = Self::new(2);

        #[doc = "1 cycle"]
        pub const _01: Self = Self::new(1);

        #[doc = "0 cycle"]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirsel_SPEC;
    pub type Dirsel = crate::EnumBitfieldStruct<u8, Dirsel_SPEC>;
    impl Dirsel {
        #[doc = "Invalid data is output prior to valid (RGB) data."]
        pub const _1: Self = Self::new(1);

        #[doc = "Invalid data is output following valid (RGB) data."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frqsel_SPEC;
    pub type Frqsel = crate::EnumBitfieldStruct<u8, Frqsel_SPEC>;
    impl Frqsel {
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);

        #[doc = "Quarter frequency (serial RGB)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "No frequency division, parallel RGB"]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        #[doc = "Serial RGB; select RGB888 as dither output format."]
        pub const _11: Self = Self::new(3);

        #[doc = "RGB565; select RGB565 as dither output format."]
        pub const _10: Self = Self::new(2);

        #[doc = "RGB666; select RGB666 as dither output format."]
        pub const _01: Self = Self::new(1);

        #[doc = "RGB888; select RGB888 as dither output format."]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swapon_SPEC;
    pub type Swapon = crate::EnumBitfieldStruct<u8, Swapon_SPEC>;
    impl Swapon {
        #[doc = "In the order of BGR"]
        pub const _1: Self = Self::new(1);

        #[doc = "In the order of RGB"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Endianon_SPEC;
    pub type Endianon = crate::EnumBitfieldStruct<u8, Endianon_SPEC>;
    impl Endianon {
        #[doc = "Ascending order (big endian)"]
        pub const _1: Self = Self::new(1);

        #[doc = "Descending order (little endian)"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBright1_SPEC;
impl crate::sealed::RegSpec for OutBright1_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Brightness Correction Register 1"]
pub type OutBright1 = crate::RegValueT<OutBright1_SPEC>;

impl OutBright1 {
    #[doc = "Brightness (DC) adjustment of G signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtg(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000000000. The write value should be 0000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        u32,
        OutBright1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            u32,
            OutBright1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OutBright1 {
    #[inline(always)]
    fn default() -> OutBright1 {
        <crate::RegValueT<OutBright1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBright2_SPEC;
impl crate::sealed::RegSpec for OutBright2_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Brightness Correction Register 2"]
pub type OutBright2 = crate::RegValueT<OutBright2_SPEC>;

impl OutBright2 {
    #[doc = "Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtb(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, u8, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3f,1,0,u8,u8,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutBright2 {
    #[inline(always)]
    fn default() -> OutBright2 {
        <crate::RegValueT<OutBright2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutContrast_SPEC;
impl crate::sealed::RegSpec for OutContrast_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Contrast Correction Register"]
pub type OutContrast = crate::RegValueT<OutContrast_SPEC>;

impl OutContrast {
    #[doc = "Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    pub fn contr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    pub fn contb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point."]
    #[inline(always)]
    pub fn contg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutContrast {
    #[inline(always)]
    fn default() -> OutContrast {
        <crate::RegValueT<OutContrast_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPdtha_SPEC;
impl crate::sealed::RegSpec for OutPdtha_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Panel Dither Correction Register"]
pub type OutPdtha = crate::RegValueT<OutPdtha_SPEC>;

impl OutPdtha {
    #[doc = "Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pb(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pa(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Output format select"]
    #[inline(always)]
    pub fn form(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        out_pdtha::Form,
        out_pdtha::Form,
        OutPdtha_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            out_pdtha::Form,
            out_pdtha::Form,
            OutPdtha_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation mode"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        out_pdtha::Sel,
        out_pdtha::Sel,
        OutPdtha_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            out_pdtha::Sel,
            out_pdtha::Sel,
            OutPdtha_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, u16, OutPdtha_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3ff,1,0,u16,u16,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutPdtha {
    #[inline(always)]
    fn default() -> OutPdtha {
        <crate::RegValueT<OutPdtha_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_pdtha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Form_SPEC;
    pub type Form = crate::EnumBitfieldStruct<u8, Form_SPEC>;
    impl Form {
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);

        #[doc = "RGB565; select RGB565 as output interface format."]
        pub const _10: Self = Self::new(2);

        #[doc = "RGB666; select RGB666 as output interface format."]
        pub const _01: Self = Self::new(1);

        #[doc = "RGB888; select RGB888 or serial RGB as output interface format."]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);

        #[doc = "2x2 pattern dither"]
        pub const _10: Self = Self::new(2);

        #[doc = "Round-off"]
        pub const _01: Self = Self::new(1);

        #[doc = "Truncate"]
        pub const _00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClkphase_SPEC;
impl crate::sealed::RegSpec for OutClkphase_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Output Phase Control Register"]
pub type OutClkphase = crate::RegValueT<OutClkphase_SPEC>;

impl OutClkphase {
    #[doc = "LCD_TCON3 Output Phase Control"]
    #[inline(always)]
    pub fn tcon3edge(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        out_clkphase::Tcon3Edge,
        out_clkphase::Tcon3Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            out_clkphase::Tcon3Edge,
            out_clkphase::Tcon3Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_TCON2 Output Phase Control"]
    #[inline(always)]
    pub fn tcon2edge(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        out_clkphase::Tcon2Edge,
        out_clkphase::Tcon2Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            out_clkphase::Tcon2Edge,
            out_clkphase::Tcon2Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_TCON1 Output Phase Control"]
    #[inline(always)]
    pub fn tcon1edge(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        out_clkphase::Tcon1Edge,
        out_clkphase::Tcon1Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            out_clkphase::Tcon1Edge,
            out_clkphase::Tcon1Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_TCON0 Output Phase Control"]
    #[inline(always)]
    pub fn tcon0edge(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        out_clkphase::Tcon0Edge,
        out_clkphase::Tcon0Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            out_clkphase::Tcon0Edge,
            out_clkphase::Tcon0Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_DATA Output Phase Control"]
    #[inline(always)]
    pub fn lcdedge(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        out_clkphase::Lcdedge,
        out_clkphase::Lcdedge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            out_clkphase::Lcdedge,
            out_clkphase::Lcdedge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Correction control"]
    #[inline(always)]
    pub fn frontgam(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        out_clkphase::Frontgam,
        out_clkphase::Frontgam,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            out_clkphase::Frontgam,
            out_clkphase::Frontgam,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, OutClkphase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OutClkphase {
    #[inline(always)]
    fn default() -> OutClkphase {
        <crate::RegValueT<OutClkphase_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_clkphase {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon3Edge_SPEC;
    pub type Tcon3Edge = crate::EnumBitfieldStruct<u8, Tcon3Edge_SPEC>;
    impl Tcon3Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);

        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon2Edge_SPEC;
    pub type Tcon2Edge = crate::EnumBitfieldStruct<u8, Tcon2Edge_SPEC>;
    impl Tcon2Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);

        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon1Edge_SPEC;
    pub type Tcon1Edge = crate::EnumBitfieldStruct<u8, Tcon1Edge_SPEC>;
    impl Tcon1Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);

        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon0Edge_SPEC;
    pub type Tcon0Edge = crate::EnumBitfieldStruct<u8, Tcon0Edge_SPEC>;
    impl Tcon0Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);

        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdedge_SPEC;
    pub type Lcdedge = crate::EnumBitfieldStruct<u8, Lcdedge_SPEC>;
    impl Lcdedge {
        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);

        #[doc = "In synchronization with the falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frontgam_SPEC;
    pub type Frontgam = crate::EnumBitfieldStruct<u8, Frontgam_SPEC>;
    impl Frontgam {
        #[doc = "Gamma correction is followed by brightness/contrast correction."]
        pub const _1: Self = Self::new(1);

        #[doc = "Brightness/contrast correction is followed by gamma correction."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconVlatch_SPEC;
impl crate::sealed::RegSpec for TconVlatch_SPEC {
    type DataType = u32;
}

#[doc = "TCON VLATCH Register"]
pub type TconVlatch = crate::RegValueT<TconVlatch_SPEC>;

impl TconVlatch {
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TconVlatch_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TconVlatch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, TconVlatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,TconVlatch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconVlatch {
    #[inline(always)]
    fn default() -> TconVlatch {
        <crate::RegValueT<TconVlatch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconTim_SPEC;
impl crate::sealed::RegSpec for TconTim_SPEC {
    type DataType = u32;
}

#[doc = "TCON Reference Timing Setting Register"]
pub type TconTim = crate::RegValueT<TconTim_SPEC>;

impl TconTim {
    #[doc = "Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconTim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    pub fn half(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconTim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, TconTim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconTim {
    #[inline(always)]
    fn default() -> TconTim {
        <crate::RegValueT<TconTim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStv1_SPEC;
impl crate::sealed::RegSpec for TconStv1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register %s1"]
pub type TconStv1 = crate::RegValueT<TconStv1_SPEC>;

impl TconStv1 {
    #[doc = "STVx1 second change timingSets the signal assertion width."]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconStv1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconStv1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "STVx1 first change timing"]
    #[inline(always)]
    pub fn vs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconStv1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconStv1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, TconStv1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,TconStv1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStv1 {
    #[inline(always)]
    fn default() -> TconStv1 {
        <crate::RegValueT<TconStv1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStv2_SPEC;
impl crate::sealed::RegSpec for TconStv2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register %s2"]
pub type TconStv2 = crate::RegValueT<TconStv2_SPEC>;

impl TconStv2 {
    #[doc = "Output signal select control for VSOUT (controlled by TCON_STVA2 register)/VEOUT (controlled by the TCON_STVB2 register) pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stv2::Sel,
        tcon_stv2::Sel,
        TconStv2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stv2::Sel,
            tcon_stv2::Sel,
            TconStv2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STVx signal polarity inversion control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stv2::Inv,
        tcon_stv2::Inv,
        TconStv2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stv2::Inv,
            tcon_stv2::Inv,
            TconStv2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000000000000000. The write value should be 000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, TconStv2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,TconStv2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStv2 {
    #[inline(always)]
    fn default() -> TconStv2 {
        <crate::RegValueT<TconStv2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stv2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "DE"]
        pub const _111: Self = Self::new(7);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Inverted"]
        pub const _1: Self = Self::new(1);

        #[doc = "Not inverted"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSth1_SPEC;
impl crate::sealed::RegSpec for TconSth1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register STH%s1"]
pub type TconSth1 = crate::RegValueT<TconSth1_SPEC>;

impl TconSth1 {
    #[doc = "STHx1 second change timing.Sets the signal assertion width."]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconSth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconSth1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "STHx1 first change timing"]
    #[inline(always)]
    pub fn hs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconSth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconSth1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, TconSth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,TconSth1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSth1 {
    #[inline(always)]
    fn default() -> TconSth1 {
        <crate::RegValueT<TconSth1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSth2_SPEC;
impl crate::sealed::RegSpec for TconSth2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register STH%s2"]
pub type TconSth2 = crate::RegValueT<TconSth2_SPEC>;

impl TconSth2 {
    #[doc = "Output signal select control for LCD_TCON2 (controlled by TCON_STHA2 register)/LCD_TCON3 (controlled by the TCON_STHB2 register) pin."]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_sth2::Sel,
        tcon_sth2::Sel,
        TconSth2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_sth2::Sel,
            tcon_sth2::Sel,
            TconSth2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STVx signal polarity inversion control."]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_sth2::Inv,
        tcon_sth2::Inv,
        TconSth2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_sth2::Inv,
            tcon_sth2::Inv,
            TconSth2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STHx signal generation reference timing control."]
    #[inline(always)]
    pub fn hssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tcon_sth2::Hssel,
        tcon_sth2::Hssel,
        TconSth2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tcon_sth2::Hssel,
            tcon_sth2::Hssel,
            TconSth2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000000000000. The write value should be 00000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, TconSth2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,TconSth2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSth2 {
    #[inline(always)]
    fn default() -> TconSth2 {
        <crate::RegValueT<TconSth2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_sth2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "DE"]
        pub const _111: Self = Self::new(7);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Inverted"]
        pub const _1: Self = Self::new(1);

        #[doc = "Not inverted"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hssel_SPEC;
    pub type Hssel = crate::EnumBitfieldStruct<u8, Hssel_SPEC>;
    impl Hssel {
        #[doc = "Reference timing is the offset set with the TCON_TIM.OFFSET\\[10:0\\] (horizontal synchronization generation reference timing) field"]
        pub const _1: Self = Self::new(1);

        #[doc = "Reference timing is the input horizontal synchronization signal (HSIN)"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconDe_SPEC;
impl crate::sealed::RegSpec for TconDe_SPEC {
    type DataType = u32;
}

#[doc = "TCON Data Enable Polarity Setting Register"]
pub type TconDe = crate::RegValueT<TconDe_SPEC>;

impl TconDe {
    #[doc = "DE signal polarity inversion control."]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tcon_de::Inv,
        tcon_de::Inv,
        TconDe_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcon_de::Inv,
            tcon_de::Inv,
            TconDe_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, TconDe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,TconDe_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconDe {
    #[inline(always)]
    fn default() -> TconDe {
        <crate::RegValueT<TconDe_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_de {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Inverted"]
        pub const _1: Self = Self::new(1);

        #[doc = "Not inverted"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntDtcten_SPEC;
impl crate::sealed::RegSpec for SyscntDtcten_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block State Detection Control Register"]
pub type SyscntDtcten = crate::RegValueT<SyscntDtcten_SPEC>;

impl SyscntDtcten {
    #[doc = "Specified line detection control"]
    #[inline(always)]
    pub fn vposdtc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_dtcten::Vposdtc,
        syscnt_dtcten::Vposdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_dtcten::Vposdtc,
            syscnt_dtcten::Vposdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 1 underflow detection control"]
    #[inline(always)]
    pub fn l1undfdtc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_dtcten::L1Undfdtc,
        syscnt_dtcten::L1Undfdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_dtcten::L1Undfdtc,
            syscnt_dtcten::L1Undfdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 2 underflow detection control"]
    #[inline(always)]
    pub fn l2undfdtc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_dtcten::L2Undfdtc,
        syscnt_dtcten::L2Undfdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_dtcten::L2Undfdtc,
            syscnt_dtcten::L2Undfdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000000000000000000. The write value should be 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntDtcten {
    #[inline(always)]
    fn default() -> SyscntDtcten {
        <crate::RegValueT<SyscntDtcten_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_dtcten {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposdtc_SPEC;
    pub type Vposdtc = crate::EnumBitfieldStruct<u8, Vposdtc_SPEC>;
    impl Vposdtc {
        #[doc = "Enables detection."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables detection."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfdtc_SPEC;
    pub type L1Undfdtc = crate::EnumBitfieldStruct<u8, L1Undfdtc_SPEC>;
    impl L1Undfdtc {
        #[doc = "Enables detection."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables detection."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfdtc_SPEC;
    pub type L2Undfdtc = crate::EnumBitfieldStruct<u8, L2Undfdtc_SPEC>;
    impl L2Undfdtc {
        #[doc = "Enables detection."]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables detection."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntInten_SPEC;
impl crate::sealed::RegSpec for SyscntInten_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Interrupt Request Enable Control Register"]
pub type SyscntInten = crate::RegValueT<SyscntInten_SPEC>;

impl SyscntInten {
    #[doc = "Interrupt request signal GLCDC_VPOS enable control."]
    #[inline(always)]
    pub fn vposinten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_inten::Vposinten,
        syscnt_inten::Vposinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_inten::Vposinten,
            syscnt_inten::Vposinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt request signal GLCDC_L1UNDF enable control."]
    #[inline(always)]
    pub fn l1undfinten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_inten::L1Undfinten,
        syscnt_inten::L1Undfinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_inten::L1Undfinten,
            syscnt_inten::L1Undfinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt request signal GLCDC_L2UNDF enable control."]
    #[inline(always)]
    pub fn l2undfinten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_inten::L2Undfinten,
        syscnt_inten::L2Undfinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_inten::L2Undfinten,
            syscnt_inten::L2Undfinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000000000000000000. The write value should be 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntInten {
    #[inline(always)]
    fn default() -> SyscntInten {
        <crate::RegValueT<SyscntInten_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_inten {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposinten_SPEC;
    pub type Vposinten = crate::EnumBitfieldStruct<u8, Vposinten_SPEC>;
    impl Vposinten {
        #[doc = "Enables GLCDC_VPOS output"]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables GLCDC_VPOS output"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfinten_SPEC;
    pub type L1Undfinten = crate::EnumBitfieldStruct<u8, L1Undfinten_SPEC>;
    impl L1Undfinten {
        #[doc = "Enables GLCDC_L1UNDF output"]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables GLCDC_L1UNDF output"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfinten_SPEC;
    pub type L2Undfinten = crate::EnumBitfieldStruct<u8, L2Undfinten_SPEC>;
    impl L2Undfinten {
        #[doc = "Enables GLCDC_L2UNDF output"]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables GLCDC_L2UNDF output"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStclr_SPEC;
impl crate::sealed::RegSpec for SyscntStclr_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Status Clear Register"]
pub type SyscntStclr = crate::RegValueT<SyscntStclr_SPEC>;

impl SyscntStclr {
    #[doc = "Graphics 2 specified line detection flag clear field"]
    #[inline(always)]
    pub fn vposclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_stclr::Vposclr,
        syscnt_stclr::Vposclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_stclr::Vposclr,
            syscnt_stclr::Vposclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 1 underflow detection flag clear field"]
    #[inline(always)]
    pub fn l1undfclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_stclr::L1Undfclr,
        syscnt_stclr::L1Undfclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_stclr::L1Undfclr,
            syscnt_stclr::L1Undfclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 2 underflow detection flag clear field"]
    #[inline(always)]
    pub fn l2undfclr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_stclr::L2Undfclr,
        syscnt_stclr::L2Undfclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_stclr::L2Undfclr,
            syscnt_stclr::L2Undfclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000000000000000000. The write value should be 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntStclr {
    #[inline(always)]
    fn default() -> SyscntStclr {
        <crate::RegValueT<SyscntStclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_stclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposclr_SPEC;
    pub type Vposclr = crate::EnumBitfieldStruct<u8, Vposclr_SPEC>;
    impl Vposclr {
        #[doc = "Clears the specified line detection flag."]
        pub const _1: Self = Self::new(1);

        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfclr_SPEC;
    pub type L1Undfclr = crate::EnumBitfieldStruct<u8, L1Undfclr_SPEC>;
    impl L1Undfclr {
        #[doc = "Clears the graphics 1 underflow detection flag."]
        pub const _1: Self = Self::new(1);

        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfclr_SPEC;
    pub type L2Undfclr = crate::EnumBitfieldStruct<u8, L2Undfclr_SPEC>;
    impl L2Undfclr {
        #[doc = "Clears the graphics 2 underflow detection flag."]
        pub const _1: Self = Self::new(1);

        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStmon_SPEC;
impl crate::sealed::RegSpec for SyscntStmon_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Status Monitor Register"]
pub type SyscntStmon = crate::RegValueT<SyscntStmon_SPEC>;

impl SyscntStmon {
    #[doc = "Graphics 2 specified line detection flag"]
    #[inline(always)]
    pub fn vpos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_stmon::Vpos,
        syscnt_stmon::Vpos,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_stmon::Vpos,
            syscnt_stmon::Vpos,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 1 underflow detection flag"]
    #[inline(always)]
    pub fn l1undf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_stmon::L1Undf,
        syscnt_stmon::L1Undf,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_stmon::L1Undf,
            syscnt_stmon::L1Undf,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 2 underflow detection flag"]
    #[inline(always)]
    pub fn l2undf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_stmon::L2Undf,
        syscnt_stmon::L2Undf,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_stmon::L2Undf,
            syscnt_stmon::L2Undf,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntStmon {
    #[inline(always)]
    fn default() -> SyscntStmon {
        <crate::RegValueT<SyscntStmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_stmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vpos_SPEC;
    pub type Vpos = crate::EnumBitfieldStruct<u8, Vpos_SPEC>;
    impl Vpos {
        #[doc = "A specified line notification has been detected in graphics 2."]
        pub const _1: Self = Self::new(1);

        #[doc = "No specified line notification has been detected in graphics 2."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undf_SPEC;
    pub type L1Undf = crate::EnumBitfieldStruct<u8, L1Undf_SPEC>;
    impl L1Undf {
        #[doc = "An underflow has been detected in graphics 1."]
        pub const _1: Self = Self::new(1);

        #[doc = "No underflow has been detected in graphics 1."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undf_SPEC;
    pub type L2Undf = crate::EnumBitfieldStruct<u8, L2Undf_SPEC>;
    impl L2Undf {
        #[doc = "An underflow has been detected in graphics 2."]
        pub const _1: Self = Self::new(1);

        #[doc = "No underflow has been detected in graphics 2."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntPanelClk_SPEC;
impl crate::sealed::RegSpec for SyscntPanelClk_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Version and Panel Clock Control Register"]
pub type SyscntPanelClk = crate::RegValueT<SyscntPanelClk_SPEC>;

impl SyscntPanelClk {
    #[doc = "Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited."]
    #[inline(always)]
    pub fn dcdr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0."]
    #[inline(always)]
    pub fn clken(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syscnt_panel_clk::Clken,
        syscnt_panel_clk::Clken,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syscnt_panel_clk::Clken,
            syscnt_panel_clk::Clken,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Panel clock supply source select"]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        syscnt_panel_clk::Clksel,
        syscnt_panel_clk::Clksel,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            syscnt_panel_clk::Clksel,
            syscnt_panel_clk::Clksel,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\\[1\\]."]
    #[inline(always)]
    pub fn pixsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        syscnt_panel_clk::Pixsel,
        syscnt_panel_clk::Pixsel,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            syscnt_panel_clk::Pixsel,
            syscnt_panel_clk::Pixsel,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Version informationVersion information of the GLCD"]
    #[inline(always)]
    pub fn ver(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        SyscntPanelClk_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            SyscntPanelClk_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntPanelClk {
    #[inline(always)]
    fn default() -> SyscntPanelClk {
        <crate::RegValueT<SyscntPanelClk_SPEC> as RegisterValue<_>>::new(17825792)
    }
}
pub mod syscnt_panel_clk {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clken_SPEC;
    pub type Clken = crate::EnumBitfieldStruct<u8, Clken_SPEC>;
    impl Clken {
        #[doc = "Disable panel clock output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable panel clock output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "External clock select"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL output select"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pixsel_SPEC;
    pub type Pixsel = crate::EnumBitfieldStruct<u8, Pixsel_SPEC>;
    impl Pixsel {
        #[doc = "No frequency division, parallel RGB"]
        pub const _0: Self = Self::new(0);

        #[doc = "Quarter frequency,serial RGB"]
        pub const _1: Self = Self::new(1);
    }
}
