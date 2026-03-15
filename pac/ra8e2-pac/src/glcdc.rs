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
// Generated from SVD 1.00.01, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:40:06 +0000

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

    #[doc = "Color Palette"]
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
    pub const fn gr1_clut00(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut01(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut02(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut03(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut04(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut05(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut06(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut07(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut08(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut09(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut010(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut011(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut012(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut013(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut014(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut015(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut016(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut017(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut018(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut019(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut020(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut021(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut022(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut023(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut024(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut025(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut026(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut027(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut028(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut029(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut030(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut031(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut032(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut033(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut034(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut035(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut036(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut037(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut038(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut039(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut040(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut041(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut042(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut043(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut044(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut045(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut046(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut047(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut048(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut049(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut050(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut051(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut052(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut053(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut054(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut055(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut056(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut057(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut058(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut059(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut060(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut061(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut062(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut063(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut064(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut065(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut066(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut067(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut068(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut069(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut070(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut071(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut072(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut073(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut074(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut075(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut076(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut077(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut078(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut079(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut080(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut081(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut082(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut083(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut084(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut085(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut086(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x158usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut087(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x15cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut088(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut089(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut090(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut091(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut092(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut093(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut094(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut095(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut096(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut097(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut098(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut099(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0100(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x190usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0101(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x194usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0102(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x198usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0103(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x19cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0104(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0105(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0106(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0107(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0108(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0109(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0110(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0111(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0112(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0113(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0114(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0115(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0116(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0117(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0118(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0119(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0120(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0121(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0122(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0123(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0124(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0125(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0126(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0127(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0128(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0129(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0130(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0131(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0132(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0133(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0134(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0135(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0136(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0137(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0138(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0139(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0140(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0141(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0142(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0143(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0144(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0145(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0146(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0147(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0148(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0149(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0150(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0151(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x25cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0152(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0153(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0154(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0155(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x26cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0156(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0157(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0158(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0159(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x27cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0160(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0161(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0162(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0163(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0164(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0165(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0166(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0167(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0168(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0169(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0170(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0171(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0172(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0173(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0174(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0175(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0176(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0177(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0178(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0179(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0180(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0181(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0182(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0183(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0184(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0185(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0186(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0187(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0188(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0189(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0190(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0191(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0192(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0193(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0194(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0195(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0196(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0197(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0198(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0199(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0200(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0201(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0202(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0203(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0204(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0205(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0206(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0207(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0208(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0209(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0210(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0211(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0212(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0213(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0214(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0215(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0216(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0217(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0218(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0219(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0220(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0221(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0222(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0223(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0224(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x380usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0225(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x384usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0226(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x388usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0227(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0228(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x390usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0229(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x394usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0230(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x398usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0231(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x39cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0232(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0233(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0234(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0235(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0236(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0237(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0238(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0239(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0240(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0241(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0242(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0243(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0244(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0245(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0246(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0247(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0248(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0249(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0250(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0251(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0252(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0253(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0254(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut0255(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3fcusize),
            )
        }
    }

    #[doc = "Color Palette"]
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
    pub const fn gr1_clut10(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut11(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut12(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut13(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut14(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut15(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x414usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut16(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut17(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x41cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut18(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut19(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut110(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut111(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut112(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut113(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x434usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut114(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x438usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut115(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x43cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut116(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut117(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut118(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut119(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut120(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut121(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x454usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut122(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x458usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut123(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x45cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut124(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut125(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut126(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut127(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x46cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut128(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x470usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut129(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x474usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut130(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x478usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut131(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x47cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut132(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x480usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut133(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x484usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut134(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x488usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut135(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut136(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x490usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut137(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x494usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut138(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x498usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut139(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x49cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut140(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut141(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut142(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut143(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut144(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut145(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut146(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut147(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut148(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut149(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut150(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut151(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut152(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut153(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut154(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut155(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut156(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut157(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut158(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut159(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut160(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut161(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut162(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut163(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut164(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x500usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut165(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x504usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut166(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut167(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut168(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut169(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x514usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut170(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x518usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut171(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut172(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x520usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut173(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x524usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut174(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x528usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut175(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut176(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x530usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut177(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x534usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut178(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x538usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut179(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut180(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x540usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut181(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x544usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut182(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x548usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut183(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut184(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x550usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut185(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x554usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut186(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x558usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut187(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut188(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x560usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut189(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x564usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut190(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x568usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut191(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut192(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x570usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut193(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x574usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut194(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x578usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut195(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut196(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x580usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut197(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x584usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut198(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x588usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut199(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1100(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x590usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1101(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x594usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1102(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x598usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1103(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1104(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1105(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1106(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1107(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1108(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1109(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1110(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1111(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1112(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1113(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1114(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1115(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1116(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1117(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1118(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1119(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1120(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1121(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1122(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1123(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1124(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1125(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1126(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1127(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1128(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1129(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1130(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1131(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1132(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1133(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1134(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x618usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1135(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1136(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1137(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1138(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1139(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1140(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1141(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1142(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x638usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1143(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1144(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x640usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1145(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x644usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1146(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x648usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1147(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1148(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x650usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1149(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x654usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1150(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x658usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1151(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1152(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x660usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1153(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x664usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1154(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x668usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1155(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1156(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x670usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1157(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x674usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1158(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x678usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1159(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1160(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x680usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1161(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x684usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1162(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x688usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1163(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1164(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x690usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1165(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x694usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1166(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x698usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1167(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1168(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1169(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1170(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1171(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1172(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1173(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1174(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1175(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1176(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1177(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1178(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1179(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1180(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1181(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1182(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1183(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1184(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1185(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1186(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1187(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1188(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1189(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1190(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1191(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1192(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x700usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1193(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x704usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1194(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x708usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1195(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1196(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x710usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1197(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x714usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1198(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x718usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1199(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x71cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1200(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x720usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1201(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x724usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1202(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x728usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1203(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x72cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1204(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x730usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1205(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x734usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1206(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x738usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1207(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x73cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1208(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x740usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1209(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x744usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1210(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x748usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1211(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1212(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x750usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1213(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x754usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1214(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x758usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1215(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x75cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1216(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x760usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1217(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x764usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1218(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x768usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1219(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x76cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1220(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x770usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1221(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x774usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1222(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x778usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1223(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x77cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1224(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x780usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1225(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x784usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1226(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x788usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1227(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1228(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x790usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1229(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x794usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1230(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x798usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1231(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x79cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1232(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1233(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1234(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1235(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1236(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1237(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1238(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1239(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1240(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1241(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1242(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1243(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1244(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1245(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1246(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1247(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1248(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1249(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1250(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1251(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1252(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1253(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1254(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr1_clut1255(
        &self,
    ) -> &'static crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr1Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7fcusize),
            )
        }
    }

    #[doc = "Color Palette"]
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
    pub const fn gr2_clut00(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut01(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut02(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut03(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut04(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x810usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut05(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut06(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x818usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut07(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x81cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut08(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x820usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut09(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut010(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x828usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut011(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x82cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut012(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x830usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut013(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut014(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x838usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut015(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x83cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut016(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut017(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut018(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x848usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut019(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut020(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x850usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut021(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut022(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x858usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut023(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x85cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut024(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x860usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut025(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut026(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x868usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut027(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x86cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut028(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x870usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut029(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x874usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut030(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x878usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut031(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x87cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut032(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x880usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut033(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x884usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut034(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x888usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut035(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut036(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x890usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut037(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x894usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut038(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x898usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut039(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x89cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut040(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut041(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut042(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut043(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut044(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut045(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut046(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut047(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut048(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut049(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut050(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut051(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut052(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut053(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut054(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut055(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut056(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut057(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut058(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut059(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut060(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut061(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut062(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut063(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut064(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x900usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut065(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x904usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut066(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x908usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut067(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut068(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x910usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut069(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x914usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut070(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x918usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut071(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x91cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut072(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x920usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut073(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x924usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut074(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x928usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut075(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x92cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut076(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x930usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut077(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x934usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut078(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x938usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut079(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x93cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut080(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x940usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut081(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x944usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut082(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x948usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut083(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut084(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x950usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut085(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x954usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut086(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x958usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut087(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x95cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut088(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x960usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut089(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x964usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut090(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x968usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut091(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x96cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut092(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x970usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut093(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x974usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut094(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x978usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut095(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x97cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut096(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x980usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut097(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x984usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut098(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x988usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut099(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0100(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x990usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0101(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x994usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0102(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x998usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0103(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x99cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0104(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0105(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0106(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0107(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0108(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0109(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0110(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0111(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0112(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0113(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0114(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0115(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0116(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0117(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0118(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0119(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0120(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0121(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0122(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0123(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0124(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0125(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0126(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0127(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0128(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0129(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0130(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0131(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0132(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0133(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0134(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0135(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0136(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0137(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0138(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0139(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0140(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0141(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0142(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0143(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0144(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0145(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0146(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0147(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0148(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0149(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0150(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0151(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0152(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0153(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0154(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0155(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0156(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0157(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0158(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0159(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0160(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0161(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0162(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0163(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0164(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0165(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0166(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0167(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0168(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0169(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0170(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0171(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0172(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0173(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0174(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0175(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xabcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0176(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xac0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0177(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xac4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0178(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xac8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0179(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0180(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xad0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0181(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xad4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0182(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xad8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0183(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xadcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0184(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xae0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0185(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xae4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0186(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xae8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0187(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0188(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0189(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0190(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0191(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xafcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0192(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0193(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0194(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0195(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0196(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0197(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0198(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0199(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0200(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0201(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0202(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0203(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0204(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0205(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0206(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0207(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0208(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0209(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0210(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0211(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0212(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0213(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0214(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0215(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0216(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0217(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0218(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0219(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0220(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0221(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0222(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0223(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0224(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0225(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0226(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0227(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0228(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0229(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0230(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0231(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0232(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xba0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0233(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xba4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0234(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xba8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0235(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0236(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0237(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0238(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0239(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0240(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0241(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0242(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0243(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0244(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0245(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0246(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0247(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0248(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0249(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0250(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0251(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0252(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0253(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0254(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut0255(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbfcusize),
            )
        }
    }

    #[doc = "Color Palette"]
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
    pub const fn gr2_clut10(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut11(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut12(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut13(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut14(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut15(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut16(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut17(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut18(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut19(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut110(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut111(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut112(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut113(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut114(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut115(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut116(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut117(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut118(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut119(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut120(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut121(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut122(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut123(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut124(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut125(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut126(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut127(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut128(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut129(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut130(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut131(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut132(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut133(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut134(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut135(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut136(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut137(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut138(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut139(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut140(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xca0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut141(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xca4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut142(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xca8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut143(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut144(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut145(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut146(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut147(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut148(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut149(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut150(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut151(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut152(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut153(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut154(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut155(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut156(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xce0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut157(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xce4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut158(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xce8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut159(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut160(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut161(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut162(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut163(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut164(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut165(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut166(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut167(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut168(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut169(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut170(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut171(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut172(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut173(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut174(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut175(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut176(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut177(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut178(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut179(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut180(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut181(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut182(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut183(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut184(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut185(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut186(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut187(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut188(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut189(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut190(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut191(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut192(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut193(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut194(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut195(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut196(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut197(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut198(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut199(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1100(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1101(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1102(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1103(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1104(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xda0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1105(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xda4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1106(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xda8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1107(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1108(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1109(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1110(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1111(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1112(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1113(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1114(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1115(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1116(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1117(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1118(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1119(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xddcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1120(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xde0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1121(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xde4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1122(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xde8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1123(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1124(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1125(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1126(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1127(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1128(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1129(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1130(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1131(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1132(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1133(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1134(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1135(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1136(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1137(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1138(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1139(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1140(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1141(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1142(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1143(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1144(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1145(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1146(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1147(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1148(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1149(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1150(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1151(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1152(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1153(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1154(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1155(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1156(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1157(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1158(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1159(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1160(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1161(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1162(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1163(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1164(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1165(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1166(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1167(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1168(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xea0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1169(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xea4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1170(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xea8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1171(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1172(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1173(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1174(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1175(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xebcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1176(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xec0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1177(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xec4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1178(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xec8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1179(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1180(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xed0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1181(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xed4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1182(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xed8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1183(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xedcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1184(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xee0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1185(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xee4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1186(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xee8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1187(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1188(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xef0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1189(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xef4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1190(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xef8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1191(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xefcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1192(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1193(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1194(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1195(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1196(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1197(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1198(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1199(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1200(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1201(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1202(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1203(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1204(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1205(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1206(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1207(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1208(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1209(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1210(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1211(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1212(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1213(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1214(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1215(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1216(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1217(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1218(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1219(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1220(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1221(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1222(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1223(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1224(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1225(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1226(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1227(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1228(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1229(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1230(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1231(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1232(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1233(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1234(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1235(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1236(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1237(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1238(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1239(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1240(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1241(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1242(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1243(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1244(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1245(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1246(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1247(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1248(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1249(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1250(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1251(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1252(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xff0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1253(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xff4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1254(
        &self,
    ) -> &'static crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gr2Clut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xff8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clut1255(
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

    #[doc = "Graphics %s Register Update Control Register"]
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

    #[doc = "Gamma G Register Update Control Register"]
    #[inline(always)]
    pub const fn gamg_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4864usize),
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

    #[doc = "Gamma G Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gamg_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4872usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gamg_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4876usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gamg_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4880usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gamg_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4884usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gamg_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4888usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gamg_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4892usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gamg_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4896usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gamg_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4900usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gamg_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4904usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gamg_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4908usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gamg_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4912usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gamg_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4916usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gamg_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4920usize),
            )
        }
    }

    #[doc = "Gamma B Register Update Control Register"]
    #[inline(always)]
    pub const fn gamb_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GambLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4928usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gamb_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4936usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gamb_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4940usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gamb_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4944usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gamb_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4948usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gamb_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4952usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gamb_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4956usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gamb_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4960usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gamb_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4964usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gamb_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4968usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gamb_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4972usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gamb_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4976usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gamb_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4980usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gamb_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4984usize),
            )
        }
    }

    #[doc = "Gamma R Register Update Control Register"]
    #[inline(always)]
    pub const fn gamr_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4992usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gamr_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5000usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gamr_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5004usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gamr_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5008usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gamr_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5012usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gamr_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5016usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gamr_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5020usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gamr_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5024usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gamr_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5028usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gamr_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5032usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gamr_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5036usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gamr_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5040usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gamr_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5044usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gamr_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5048usize),
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

    #[doc = "TCON Vertical Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_stva1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStva1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStva1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5128usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_stva2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStva2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStva2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5132usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_stvb1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStvb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStvb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5136usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_stvb2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStvb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStvb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5140usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_stha1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStha1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStha1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5144usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_stha2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStha2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStha2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5148usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_sthb1(
        &self,
    ) -> &'static crate::common::Reg<self::TconSthb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSthb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5152usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_sthb2(
        &self,
    ) -> &'static crate::common::Reg<self::TconSthb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSthb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5156usize),
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

#[doc = "Color Palette"]
pub type Gr1Clut0 = crate::RegValueT<Gr1Clut0_SPEC>;

impl Gr1Clut0 {
    #[doc = "B value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha blending value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
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

#[doc = "Color Palette"]
pub type Gr1Clut1 = crate::RegValueT<Gr1Clut1_SPEC>;

impl Gr1Clut1 {
    #[doc = "B value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha blending value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
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

#[doc = "Color Palette"]
pub type Gr2Clut0 = crate::RegValueT<Gr2Clut0_SPEC>;

impl Gr2Clut0 {
    #[doc = "B value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha blending value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
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

#[doc = "Color Palette"]
pub type Gr2Clut1 = crate::RegValueT<Gr2Clut1_SPEC>;

impl Gr2Clut1 {
    #[doc = "B value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "R value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha blending value of color palette n plane for graphics m plane. Unsigned 8-bit integer."]
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
    #[doc = "Background Plane Operation Enable"]
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

    #[doc = "Control of GLCDC Internal Register Value Reflection to Internal Operations"]
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

    #[doc = "Software Reset Control"]
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
        #[doc = "Disable background plane operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable background plane operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable GLCDC register values from being reflected in internal operations at start of screen generation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC register values to be reflected in internal operations at start of screen generation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "Place entire module in software reset state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Release entire module from software reset state"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Background Plane Horizontal Synchronization Signal Period"]
    #[inline(always)]
    pub fn fh(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        bg_peri::Fh,
        bg_peri::Fh,
        BgPeri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            bg_peri::Fh,
            bg_peri::Fh,
            BgPeri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Background Plane Vertical Synchronization Signal Period"]
    #[inline(always)]
    pub fn fv(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        bg_peri::Fv,
        bg_peri::Fv,
        BgPeri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            bg_peri::Fv,
            bg_peri::Fv,
            BgPeri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BgPeri {
    #[inline(always)]
    fn default() -> BgPeri {
        <crate::RegValueT<BgPeri_SPEC> as RegisterValue<_>>::new(1507351)
    }
}
pub mod bg_peri {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fh_SPEC;
    pub type Fh = crate::EnumBitfieldStruct<u16, Fh_SPEC>;
    impl Fh {
        #[doc = "24 cycles (pixels)"]
        pub const _0_X_017: Self = Self::new(23);

        #[doc = "1024 cycles (pixels)"]
        pub const _0_X_3_FF: Self = Self::new(1023);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fv_SPEC;
    pub type Fv = crate::EnumBitfieldStruct<u16, Fv_SPEC>;
    impl Fv {
        #[doc = "20 lines"]
        pub const _0_X_013: Self = Self::new(19);

        #[doc = "1024 lines"]
        pub const _0_X_3_FF: Self = Self::new(1023);
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
    #[doc = "Background Plane Horizontal Synchronization Signal Assertion Position"]
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

    #[doc = "Background Plane Vertical Synchronization Assertion Position"]
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
    #[doc = "Background Plane Vertical Valid Pixel Width"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Vertical Valid Pixel Start Position"]
    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Background Plane Horizontal Valid Pixel Width"]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Horizontal Valid Pixel Start Position"]
    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Background Plane Valid Pixel Area B Value"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Valid Pixel Area G Value"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Valid Pixel Area R Value"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Background Plane Operation Monitor"]
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

    #[doc = "Entire Module Internal Operation Reflection Control Signal Monitor"]
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

    #[doc = "Entire Module SW Reset State Monitor"]
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
        #[doc = "Operation is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation is in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Signal for controlling reflection of the register values to internal operations on assertion of vertical synchronization signal is negated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Signal for controlling reflection of the register values to internal operations on assertion of vertical synchronization signal is asserted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "Entire module is in software reset state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Entire module is released from software reset state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrVen_SPEC;
impl crate::sealed::RegSpec for GrVen_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Register Update Control Register"]
pub type GrVen = crate::RegValueT<GrVen_SPEC>;

impl GrVen {
    #[doc = "This bit is cleared to 0 by an internal source."]
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
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Graphics Data Read Enable"]
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
        #[doc = "Disable reading"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reading"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Burst Transfer Control for Graphics Data Access"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gr_flm1::Bstmd,
            gr_flm1::Bstmd,
            GrFlm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    #[doc = "Base Address for Accessing Graphics Data"]
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
    #[doc = "Macro Line Offset Address for Accessing Graphics Data"]
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
    #[doc = "Number of Data Transfer Times Per Line for Accessing Graphics Data"]
    #[inline(always)]
    pub fn datanum(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, GrFlm5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,GrFlm5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Number of Lines Per Frame for Accessing Graphics Data"]
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
    #[doc = "Data Format for Accessing Graphics Data"]
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
        #[doc = "RGB565 (16 bits/pixel)"]
        pub const _000: Self = Self::new(0);

        #[doc = "RGB888 (32 bits/pixel, 8 bits on the MSB side are invalid)"]
        pub const _001: Self = Self::new(1);

        #[doc = "ARGB1555 (16 bits/pixel, 1 bit of A is LUT data)"]
        pub const _010: Self = Self::new(2);

        #[doc = "ARGB4444 (16 bits/pixel)"]
        pub const _011: Self = Self::new(3);

        #[doc = "ARGB8888 (32 bits/pixel)"]
        pub const _100: Self = Self::new(4);

        #[doc = "CLUT8 (8 bits/pixel)"]
        pub const _101: Self = Self::new(5);

        #[doc = "CLUT4 (4 bits/pixel)"]
        pub const _110: Self = Self::new(6);

        #[doc = "CLUT1 (1 bit/pixel)"]
        pub const _111: Self = Self::new(7);
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
    #[doc = "Graphics Display Plane Control"]
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

    #[doc = "Graphics Image Area Border Display Control"]
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

    #[doc = "Image Area Border Display Control for Rectangular Area Alpha Blending"]
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

    #[doc = "Rectangular Area Alpha Blending Control"]
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
        #[doc = "Background color display (value set in the GRn_BASE register)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Lower-layer graphics display"]
        pub const _01: Self = Self::new(1);

        #[doc = "Current graphics display"]
        pub const _10: Self = Self::new(2);

        #[doc = "Blended display of lower-layer graphics (input image from the previous stage) and current graphics (data read from the GLCDC0 and GLCDC1 bus)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcdispon_SPEC;
    pub type Grcdispon = crate::EnumBitfieldStruct<u8, Grcdispon_SPEC>;
    impl Grcdispon {
        #[doc = "Turn display off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn display on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcdispon_SPEC;
    pub type Arcdispon = crate::EnumBitfieldStruct<u8, Arcdispon_SPEC>;
    impl Arcdispon {
        #[doc = "Turn display off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn display on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcon_SPEC;
    pub type Arcon = crate::EnumBitfieldStruct<u8, Arcon_SPEC>;
    impl Arcon {
        #[doc = "Turn blending off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn blending on"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Vertical Width of Graphics Image Area"]
    #[inline(always)]
    pub fn grcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Start Position of Graphics Image Area"]
    #[inline(always)]
    pub fn grcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Horizontal Width of Graphics Image Area"]
    #[inline(always)]
    pub fn grchw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Start Position of Graphics Image Area"]
    #[inline(always)]
    pub fn grchs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Vertical Width of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn arcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Start Position of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn arcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Horizontal Width of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn archw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Start Position of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn archs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Frame Rate for Alpha Blending in Rectangular Area"]
    #[inline(always)]
    pub fn arcrate(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha Coefficient for Alpha Blending in Rectangular Area"]
    #[inline(always)]
    pub fn arccoef(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, GrAb6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "RGB-Index Chroma-Key Processing Control"]
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

    #[doc = "Initial Alpha Value for Alpha Blending in Rectangular Area"]
    #[inline(always)]
    pub fn arcdef(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb7_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disable chroma-key processing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable chroma-key processing"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "R Signal for RGB-Index Chroma-Key Processing"]
    #[inline(always)]
    pub fn ckkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B Signal for RGB-Index Chroma-Key Processing"]
    #[inline(always)]
    pub fn ckkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Signal for RGB-Index Chroma-Key Processing"]
    #[inline(always)]
    pub fn ckkg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "R Value after RGB-Index Chroma-Key Processing Replacement"]
    #[inline(always)]
    pub fn ckr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B Value after RGB-Index Chroma-Key Processing Replacement"]
    #[inline(always)]
    pub fn ckb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Value after RGB-Index Chroma-Key Processing Replacement"]
    #[inline(always)]
    pub fn ckg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A Value after RGB-Index Chroma-Key Processing Replacement"]
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
    #[doc = "Background Color R Value"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Color B Value"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Color G Value"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Number of Detection Lines"]
    #[inline(always)]
    pub fn line(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrClutint_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrClutint_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CLUT Table Control"]
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
        #[doc = "Select CLUT table 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select CLUT table 1"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Status Monitor for Alpha Blending in Rectangular Area"]
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

    #[doc = "Underflow Status Monitor"]
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
        #[doc = "Fade-in/fade-out not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fade-in/fade-out in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Undflst_SPEC;
    pub type Undflst = crate::EnumBitfieldStruct<u8, Undflst_SPEC>;
    impl Undflst {
        #[doc = "No underflow occurred in internal operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow occurred in internal operations"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLatch_SPEC;
impl crate::sealed::RegSpec for GamgLatch_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Register Update Control Register"]
pub type GamgLatch = crate::RegValueT<GamgLatch_SPEC>;

impl GamgLatch {
    #[doc = "Control of Gamma Correction × Module Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gamg_latch::Ven,
        gamg_latch::Ven,
        GamgLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gamg_latch::Ven,
            gamg_latch::Ven,
            GamgLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GamgLatch {
    #[inline(always)]
    fn default() -> GamgLatch {
        <crate::RegValueT<GamgLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gamg_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Gamma Correction On/Off Control"]
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
        #[doc = "Turn off gamma correction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn on gamma correction"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut1_SPEC;
impl crate::sealed::RegSpec for GamgLut1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 1"]
pub type GamgLut1 = crate::RegValueT<GamgLut1_SPEC>;

impl GamgLut1 {
    #[doc = "Gain Value of Area 1"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 0"]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut1 {
    #[inline(always)]
    fn default() -> GamgLut1 {
        <crate::RegValueT<GamgLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut2_SPEC;
impl crate::sealed::RegSpec for GamgLut2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 2"]
pub type GamgLut2 = crate::RegValueT<GamgLut2_SPEC>;

impl GamgLut2 {
    #[doc = "Gain Value of Area 3"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 2"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut2 {
    #[inline(always)]
    fn default() -> GamgLut2 {
        <crate::RegValueT<GamgLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut3_SPEC;
impl crate::sealed::RegSpec for GamgLut3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 3"]
pub type GamgLut3 = crate::RegValueT<GamgLut3_SPEC>;

impl GamgLut3 {
    #[doc = "Gain Value of Area 5"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 4"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut3 {
    #[inline(always)]
    fn default() -> GamgLut3 {
        <crate::RegValueT<GamgLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut4_SPEC;
impl crate::sealed::RegSpec for GamgLut4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 4"]
pub type GamgLut4 = crate::RegValueT<GamgLut4_SPEC>;

impl GamgLut4 {
    #[doc = "Gain Value of Area 7"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 6"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut4 {
    #[inline(always)]
    fn default() -> GamgLut4 {
        <crate::RegValueT<GamgLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut5_SPEC;
impl crate::sealed::RegSpec for GamgLut5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 5"]
pub type GamgLut5 = crate::RegValueT<GamgLut5_SPEC>;

impl GamgLut5 {
    #[doc = "Gain Value of Area 9"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 8"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut5 {
    #[inline(always)]
    fn default() -> GamgLut5 {
        <crate::RegValueT<GamgLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut6_SPEC;
impl crate::sealed::RegSpec for GamgLut6_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 6"]
pub type GamgLut6 = crate::RegValueT<GamgLut6_SPEC>;

impl GamgLut6 {
    #[doc = "Gain Value of Area 11"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 10"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut6 {
    #[inline(always)]
    fn default() -> GamgLut6 {
        <crate::RegValueT<GamgLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut7_SPEC;
impl crate::sealed::RegSpec for GamgLut7_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 7"]
pub type GamgLut7 = crate::RegValueT<GamgLut7_SPEC>;

impl GamgLut7 {
    #[doc = "Gain Value of Area 13"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 12"]
    #[inline(always)]
    pub fn gain12(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut7 {
    #[inline(always)]
    fn default() -> GamgLut7 {
        <crate::RegValueT<GamgLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut8_SPEC;
impl crate::sealed::RegSpec for GamgLut8_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 8"]
pub type GamgLut8 = crate::RegValueT<GamgLut8_SPEC>;

impl GamgLut8 {
    #[doc = "Gain Value of Area 15"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 14"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut8 {
    #[inline(always)]
    fn default() -> GamgLut8 {
        <crate::RegValueT<GamgLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea1_SPEC;
impl crate::sealed::RegSpec for GamgArea1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 1"]
pub type GamgArea1 = crate::RegValueT<GamgArea1_SPEC>;

impl GamgArea1 {
    #[doc = "Start Threshold of Area 3"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 2"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 1"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea1 {
    #[inline(always)]
    fn default() -> GamgArea1 {
        <crate::RegValueT<GamgArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea2_SPEC;
impl crate::sealed::RegSpec for GamgArea2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 2"]
pub type GamgArea2 = crate::RegValueT<GamgArea2_SPEC>;

impl GamgArea2 {
    #[doc = "Start Threshold of Area 6"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 5"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 4"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea2 {
    #[inline(always)]
    fn default() -> GamgArea2 {
        <crate::RegValueT<GamgArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea3_SPEC;
impl crate::sealed::RegSpec for GamgArea3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 3"]
pub type GamgArea3 = crate::RegValueT<GamgArea3_SPEC>;

impl GamgArea3 {
    #[doc = "Start Threshold of Area 9"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 8"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 7"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea3 {
    #[inline(always)]
    fn default() -> GamgArea3 {
        <crate::RegValueT<GamgArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea4_SPEC;
impl crate::sealed::RegSpec for GamgArea4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 4"]
pub type GamgArea4 = crate::RegValueT<GamgArea4_SPEC>;

impl GamgArea4 {
    #[doc = "Start Threshold of Area 12"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 11"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 10"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea4 {
    #[inline(always)]
    fn default() -> GamgArea4 {
        <crate::RegValueT<GamgArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea5_SPEC;
impl crate::sealed::RegSpec for GamgArea5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 5"]
pub type GamgArea5 = crate::RegValueT<GamgArea5_SPEC>;

impl GamgArea5 {
    #[doc = "Start Threshold of Area 15"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 14"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 13"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea5 {
    #[inline(always)]
    fn default() -> GamgArea5 {
        <crate::RegValueT<GamgArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLatch_SPEC;
impl crate::sealed::RegSpec for GambLatch_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Register Update Control Register"]
pub type GambLatch = crate::RegValueT<GambLatch_SPEC>;

impl GambLatch {
    #[doc = "Control of Gamma Correction × Module Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gamb_latch::Ven,
        gamb_latch::Ven,
        GambLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gamb_latch::Ven,
            gamb_latch::Ven,
            GambLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GambLatch {
    #[inline(always)]
    fn default() -> GambLatch {
        <crate::RegValueT<GambLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gamb_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut1_SPEC;
impl crate::sealed::RegSpec for GambLut1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 1"]
pub type GambLut1 = crate::RegValueT<GambLut1_SPEC>;

impl GambLut1 {
    #[doc = "Gain Value of Area 1"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 0"]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut1 {
    #[inline(always)]
    fn default() -> GambLut1 {
        <crate::RegValueT<GambLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut2_SPEC;
impl crate::sealed::RegSpec for GambLut2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 2"]
pub type GambLut2 = crate::RegValueT<GambLut2_SPEC>;

impl GambLut2 {
    #[doc = "Gain Value of Area 3"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 2"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut2 {
    #[inline(always)]
    fn default() -> GambLut2 {
        <crate::RegValueT<GambLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut3_SPEC;
impl crate::sealed::RegSpec for GambLut3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 3"]
pub type GambLut3 = crate::RegValueT<GambLut3_SPEC>;

impl GambLut3 {
    #[doc = "Gain Value of Area 5"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 4"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut3 {
    #[inline(always)]
    fn default() -> GambLut3 {
        <crate::RegValueT<GambLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut4_SPEC;
impl crate::sealed::RegSpec for GambLut4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 4"]
pub type GambLut4 = crate::RegValueT<GambLut4_SPEC>;

impl GambLut4 {
    #[doc = "Gain Value of Area 7"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 6"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut4 {
    #[inline(always)]
    fn default() -> GambLut4 {
        <crate::RegValueT<GambLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut5_SPEC;
impl crate::sealed::RegSpec for GambLut5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 5"]
pub type GambLut5 = crate::RegValueT<GambLut5_SPEC>;

impl GambLut5 {
    #[doc = "Gain Value of Area 9"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 8"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut5 {
    #[inline(always)]
    fn default() -> GambLut5 {
        <crate::RegValueT<GambLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut6_SPEC;
impl crate::sealed::RegSpec for GambLut6_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 6"]
pub type GambLut6 = crate::RegValueT<GambLut6_SPEC>;

impl GambLut6 {
    #[doc = "Gain Value of Area 11"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 10"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut6 {
    #[inline(always)]
    fn default() -> GambLut6 {
        <crate::RegValueT<GambLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut7_SPEC;
impl crate::sealed::RegSpec for GambLut7_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 7"]
pub type GambLut7 = crate::RegValueT<GambLut7_SPEC>;

impl GambLut7 {
    #[doc = "Gain Value of Area 13"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 12"]
    #[inline(always)]
    pub fn gain12(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut7 {
    #[inline(always)]
    fn default() -> GambLut7 {
        <crate::RegValueT<GambLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut8_SPEC;
impl crate::sealed::RegSpec for GambLut8_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 8"]
pub type GambLut8 = crate::RegValueT<GambLut8_SPEC>;

impl GambLut8 {
    #[doc = "Gain Value of Area 15"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 14"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut8 {
    #[inline(always)]
    fn default() -> GambLut8 {
        <crate::RegValueT<GambLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea1_SPEC;
impl crate::sealed::RegSpec for GambArea1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 1"]
pub type GambArea1 = crate::RegValueT<GambArea1_SPEC>;

impl GambArea1 {
    #[doc = "Start Threshold of Area 3"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 2"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 1"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea1 {
    #[inline(always)]
    fn default() -> GambArea1 {
        <crate::RegValueT<GambArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea2_SPEC;
impl crate::sealed::RegSpec for GambArea2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 2"]
pub type GambArea2 = crate::RegValueT<GambArea2_SPEC>;

impl GambArea2 {
    #[doc = "Start Threshold of Area 6"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 5"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 4"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea2 {
    #[inline(always)]
    fn default() -> GambArea2 {
        <crate::RegValueT<GambArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea3_SPEC;
impl crate::sealed::RegSpec for GambArea3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 3"]
pub type GambArea3 = crate::RegValueT<GambArea3_SPEC>;

impl GambArea3 {
    #[doc = "Start Threshold of Area 9"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 8"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 7"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea3 {
    #[inline(always)]
    fn default() -> GambArea3 {
        <crate::RegValueT<GambArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea4_SPEC;
impl crate::sealed::RegSpec for GambArea4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 4"]
pub type GambArea4 = crate::RegValueT<GambArea4_SPEC>;

impl GambArea4 {
    #[doc = "Start Threshold of Area 12"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 11"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 10"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea4 {
    #[inline(always)]
    fn default() -> GambArea4 {
        <crate::RegValueT<GambArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea5_SPEC;
impl crate::sealed::RegSpec for GambArea5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 5"]
pub type GambArea5 = crate::RegValueT<GambArea5_SPEC>;

impl GambArea5 {
    #[doc = "Start Threshold of Area 15"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 14"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 13"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea5 {
    #[inline(always)]
    fn default() -> GambArea5 {
        <crate::RegValueT<GambArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLatch_SPEC;
impl crate::sealed::RegSpec for GamrLatch_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Register Update Control Register"]
pub type GamrLatch = crate::RegValueT<GamrLatch_SPEC>;

impl GamrLatch {
    #[doc = "Control of Gamma Correction × Module Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gamr_latch::Ven,
        gamr_latch::Ven,
        GamrLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gamr_latch::Ven,
            gamr_latch::Ven,
            GamrLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GamrLatch {
    #[inline(always)]
    fn default() -> GamrLatch {
        <crate::RegValueT<GamrLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gamr_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut1_SPEC;
impl crate::sealed::RegSpec for GamrLut1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 1"]
pub type GamrLut1 = crate::RegValueT<GamrLut1_SPEC>;

impl GamrLut1 {
    #[doc = "Gain Value of Area 1"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 0"]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut1 {
    #[inline(always)]
    fn default() -> GamrLut1 {
        <crate::RegValueT<GamrLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut2_SPEC;
impl crate::sealed::RegSpec for GamrLut2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 2"]
pub type GamrLut2 = crate::RegValueT<GamrLut2_SPEC>;

impl GamrLut2 {
    #[doc = "Gain Value of Area 3"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 2"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut2 {
    #[inline(always)]
    fn default() -> GamrLut2 {
        <crate::RegValueT<GamrLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut3_SPEC;
impl crate::sealed::RegSpec for GamrLut3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 3"]
pub type GamrLut3 = crate::RegValueT<GamrLut3_SPEC>;

impl GamrLut3 {
    #[doc = "Gain Value of Area 5"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 4"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut3 {
    #[inline(always)]
    fn default() -> GamrLut3 {
        <crate::RegValueT<GamrLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut4_SPEC;
impl crate::sealed::RegSpec for GamrLut4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 4"]
pub type GamrLut4 = crate::RegValueT<GamrLut4_SPEC>;

impl GamrLut4 {
    #[doc = "Gain Value of Area 7"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 6"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut4 {
    #[inline(always)]
    fn default() -> GamrLut4 {
        <crate::RegValueT<GamrLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut5_SPEC;
impl crate::sealed::RegSpec for GamrLut5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 5"]
pub type GamrLut5 = crate::RegValueT<GamrLut5_SPEC>;

impl GamrLut5 {
    #[doc = "Gain Value of Area 9"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 8"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut5 {
    #[inline(always)]
    fn default() -> GamrLut5 {
        <crate::RegValueT<GamrLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut6_SPEC;
impl crate::sealed::RegSpec for GamrLut6_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 6"]
pub type GamrLut6 = crate::RegValueT<GamrLut6_SPEC>;

impl GamrLut6 {
    #[doc = "Gain Value of Area 11"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 10"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut6 {
    #[inline(always)]
    fn default() -> GamrLut6 {
        <crate::RegValueT<GamrLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut7_SPEC;
impl crate::sealed::RegSpec for GamrLut7_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 7"]
pub type GamrLut7 = crate::RegValueT<GamrLut7_SPEC>;

impl GamrLut7 {
    #[doc = "Gain Value of Area 13"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 12"]
    #[inline(always)]
    pub fn gain12(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut7 {
    #[inline(always)]
    fn default() -> GamrLut7 {
        <crate::RegValueT<GamrLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut8_SPEC;
impl crate::sealed::RegSpec for GamrLut8_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 8"]
pub type GamrLut8 = crate::RegValueT<GamrLut8_SPEC>;

impl GamrLut8 {
    #[doc = "Gain Value of Area 15"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 14"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut8 {
    #[inline(always)]
    fn default() -> GamrLut8 {
        <crate::RegValueT<GamrLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea1_SPEC;
impl crate::sealed::RegSpec for GamrArea1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 1"]
pub type GamrArea1 = crate::RegValueT<GamrArea1_SPEC>;

impl GamrArea1 {
    #[doc = "Start Threshold of Area 3"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 2"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 1"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea1 {
    #[inline(always)]
    fn default() -> GamrArea1 {
        <crate::RegValueT<GamrArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea2_SPEC;
impl crate::sealed::RegSpec for GamrArea2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 2"]
pub type GamrArea2 = crate::RegValueT<GamrArea2_SPEC>;

impl GamrArea2 {
    #[doc = "Start Threshold of Area 6"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 5"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 4"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea2 {
    #[inline(always)]
    fn default() -> GamrArea2 {
        <crate::RegValueT<GamrArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea3_SPEC;
impl crate::sealed::RegSpec for GamrArea3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 3"]
pub type GamrArea3 = crate::RegValueT<GamrArea3_SPEC>;

impl GamrArea3 {
    #[doc = "Start Threshold of Area 9"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 8"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 7"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea3 {
    #[inline(always)]
    fn default() -> GamrArea3 {
        <crate::RegValueT<GamrArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea4_SPEC;
impl crate::sealed::RegSpec for GamrArea4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 4"]
pub type GamrArea4 = crate::RegValueT<GamrArea4_SPEC>;

impl GamrArea4 {
    #[doc = "Start Threshold of Area 12"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 11"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 10"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea4 {
    #[inline(always)]
    fn default() -> GamrArea4 {
        <crate::RegValueT<GamrArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea5_SPEC;
impl crate::sealed::RegSpec for GamrArea5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 5"]
pub type GamrArea5 = crate::RegValueT<GamrArea5_SPEC>;

impl GamrArea5 {
    #[doc = "Start Threshold of Area 15"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 14"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 13"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea5 {
    #[inline(always)]
    fn default() -> GamrArea5 {
        <crate::RegValueT<GamrArea5_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Control of Output Control Module Register Value Reflection to Internal Operations"]
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
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Data Output Delay Control in Serial RGB Format"]
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

    #[doc = "Scan Direction Select of Serial RGB Format"]
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

    #[doc = "Clock Frequency Division Control"]
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

    #[doc = "Output Format Select"]
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

    #[doc = "Pixel Order Control"]
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

    #[doc = "Bit Endian Control"]
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
        #[doc = "0 cycle"]
        pub const _00: Self = Self::new(0);

        #[doc = "1 cycle"]
        pub const _01: Self = Self::new(1);

        #[doc = "2 cycles"]
        pub const _10: Self = Self::new(2);

        #[doc = "3 cycles"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirsel_SPEC;
    pub type Dirsel = crate::EnumBitfieldStruct<u8, Dirsel_SPEC>;
    impl Dirsel {
        #[doc = "Forward scan"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reverse scan"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frqsel_SPEC;
    pub type Frqsel = crate::EnumBitfieldStruct<u8, Frqsel_SPEC>;
    impl Frqsel {
        #[doc = "No frequency division, parallel RGB"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Quarter frequency (serial RGB)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        #[doc = "RGB888 — select RGB888 as dither output format"]
        pub const _00: Self = Self::new(0);

        #[doc = "RGB666 — select RGB666 as dither output format"]
        pub const _01: Self = Self::new(1);

        #[doc = "RGB565 — select RGB565 as dither output format"]
        pub const _10: Self = Self::new(2);

        #[doc = "Serial RGB — select RGB888 as dither output format. Select dither output format in OUT_PDTHA.FORM\\[1:0\\]"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swapon_SPEC;
    pub type Swapon = crate::EnumBitfieldStruct<u8, Swapon_SPEC>;
    impl Swapon {
        #[doc = "RGB order"]
        pub const _0: Self = Self::new(0);

        #[doc = "BGR order"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Endianon_SPEC;
    pub type Endianon = crate::EnumBitfieldStruct<u8, Endianon_SPEC>;
    impl Endianon {
        #[doc = "Descending order (little endian)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Ascending order (big endian)"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Brightness Adjustment of G Signal"]
    #[inline(always)]
    pub fn brtg(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright1_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Brightness Adjustment of R Signal"]
    #[inline(always)]
    pub fn brtr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Brightness Adjustment of B Signal"]
    #[inline(always)]
    pub fn brtb(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Contrast Adjustment of R Signal"]
    #[inline(always)]
    pub fn contr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Contrast Adjustment of B Signal"]
    #[inline(always)]
    pub fn contb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Contrast Adjustment of G Signal"]
    #[inline(always)]
    pub fn contg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Pattern Value (D) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern Value (C) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern Value (B) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pb(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern Value (A) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pa(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Output Format Select"]
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

    #[doc = "Operation Mode"]
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
        #[doc = "RGB888; select RGB888 or serial RGB as output interface format"]
        pub const _00: Self = Self::new(0);

        #[doc = "RGB666; select RGB666 as output interface format"]
        pub const _01: Self = Self::new(1);

        #[doc = "RGB565; select RGB565 as output interface format"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited Select output interface format in OUT_SET.FORMAT\\[1:0\\]."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Truncate"]
        pub const _00: Self = Self::new(0);

        #[doc = "Round-off"]
        pub const _01: Self = Self::new(1);

        #[doc = "2×2 pattern dither"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "Correction Control"]
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
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon2Edge_SPEC;
    pub type Tcon2Edge = crate::EnumBitfieldStruct<u8, Tcon2Edge_SPEC>;
    impl Tcon2Edge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon1Edge_SPEC;
    pub type Tcon1Edge = crate::EnumBitfieldStruct<u8, Tcon1Edge_SPEC>;
    impl Tcon1Edge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon0Edge_SPEC;
    pub type Tcon0Edge = crate::EnumBitfieldStruct<u8, Tcon0Edge_SPEC>;
    impl Tcon0Edge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdedge_SPEC;
    pub type Lcdedge = crate::EnumBitfieldStruct<u8, Lcdedge_SPEC>;
    impl Lcdedge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frontgam_SPEC;
    pub type Frontgam = crate::EnumBitfieldStruct<u8, Frontgam_SPEC>;
    impl Frontgam {
        #[doc = "Process brightness/contrast correction followed by gamma correction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Process gamma correction followed by brightness/contrast correction"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Horizontal Synchronization Signal Generation Reference Timing"]
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconTim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Synchronization Signal Generation Change Timing"]
    #[inline(always)]
    pub fn half(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconTim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconTim_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct TconStva1_SPEC;
impl crate::sealed::RegSpec for TconStva1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x1"]
pub type TconStva1 = crate::RegValueT<TconStva1_SPEC>;

impl TconStva1 {
    #[doc = "Vertical Synchronization Signal STVx1 Second Change Timing"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconStva1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconStva1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Synchronization Signal STVx1 First Change Timing"]
    #[inline(always)]
    pub fn vs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconStva1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconStva1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStva1 {
    #[inline(always)]
    fn default() -> TconStva1 {
        <crate::RegValueT<TconStva1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStva2_SPEC;
impl crate::sealed::RegSpec for TconStva2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x2"]
pub type TconStva2 = crate::RegValueT<TconStva2_SPEC>;

impl TconStva2 {
    #[doc = "Output Signal Select Control for LCD_TCON0/LCD_TCON1 Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stva2::Sel,
        tcon_stva2::Sel,
        TconStva2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stva2::Sel,
            tcon_stva2::Sel,
            TconStva2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Vertical Synchronization Signal STVx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stva2::Inv,
        tcon_stva2::Inv,
        TconStva2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stva2::Inv,
            tcon_stva2::Inv,
            TconStva2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconStva2 {
    #[inline(always)]
    fn default() -> TconStva2 {
        <crate::RegValueT<TconStva2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stva2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStvb1_SPEC;
impl crate::sealed::RegSpec for TconStvb1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x1"]
pub type TconStvb1 = crate::RegValueT<TconStvb1_SPEC>;

impl TconStvb1 {
    #[doc = "Vertical Synchronization Signal STVx1 Second Change Timing"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconStvb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconStvb1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Synchronization Signal STVx1 First Change Timing"]
    #[inline(always)]
    pub fn vs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconStvb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconStvb1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStvb1 {
    #[inline(always)]
    fn default() -> TconStvb1 {
        <crate::RegValueT<TconStvb1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStvb2_SPEC;
impl crate::sealed::RegSpec for TconStvb2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x2"]
pub type TconStvb2 = crate::RegValueT<TconStvb2_SPEC>;

impl TconStvb2 {
    #[doc = "Output Signal Select Control for LCD_TCON0/LCD_TCON1 Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stvb2::Sel,
        tcon_stvb2::Sel,
        TconStvb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stvb2::Sel,
            tcon_stvb2::Sel,
            TconStvb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Vertical Synchronization Signal STVx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stvb2::Inv,
        tcon_stvb2::Inv,
        TconStvb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stvb2::Inv,
            tcon_stvb2::Inv,
            TconStvb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconStvb2 {
    #[inline(always)]
    fn default() -> TconStvb2 {
        <crate::RegValueT<TconStvb2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stvb2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStha1_SPEC;
impl crate::sealed::RegSpec for TconStha1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x1"]
pub type TconStha1 = crate::RegValueT<TconStha1_SPEC>;

impl TconStha1 {
    #[doc = "Horizontal Synchronization Signal STHx1 Second Change Timing"]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconStha1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconStha1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Synchronization Signal STHx1 First Change Timing"]
    #[inline(always)]
    pub fn hs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconStha1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconStha1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStha1 {
    #[inline(always)]
    fn default() -> TconStha1 {
        <crate::RegValueT<TconStha1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStha2_SPEC;
impl crate::sealed::RegSpec for TconStha2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x2"]
pub type TconStha2 = crate::RegValueT<TconStha2_SPEC>;

impl TconStha2 {
    #[doc = "Output Signal Select Control for LCD_TCON2/LCD_TCON3 Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stha2::Sel,
        tcon_stha2::Sel,
        TconStha2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stha2::Sel,
            tcon_stha2::Sel,
            TconStha2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stha2::Inv,
        tcon_stha2::Inv,
        TconStha2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stha2::Inv,
            tcon_stha2::Inv,
            TconStha2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Reference Timing Control"]
    #[inline(always)]
    pub fn hssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tcon_stha2::Hssel,
        tcon_stha2::Hssel,
        TconStha2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tcon_stha2::Hssel,
            tcon_stha2::Hssel,
            TconStha2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconStha2 {
    #[inline(always)]
    fn default() -> TconStha2 {
        <crate::RegValueT<TconStha2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stha2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hssel_SPEC;
    pub type Hssel = crate::EnumBitfieldStruct<u8, Hssel_SPEC>;
    impl Hssel {
        #[doc = "Select input horizontal synchronization signal (HSIN) as reference for signal generation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select offset specified in TCON_TIM.OFFSET\\[10:0\\] (horizontal synchronization generation reference timing) as reference for signal generation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSthb1_SPEC;
impl crate::sealed::RegSpec for TconSthb1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x1"]
pub type TconSthb1 = crate::RegValueT<TconSthb1_SPEC>;

impl TconSthb1 {
    #[doc = "Horizontal Synchronization Signal STHx1 Second Change Timing"]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconSthb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconSthb1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Synchronization Signal STHx1 First Change Timing"]
    #[inline(always)]
    pub fn hs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconSthb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconSthb1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSthb1 {
    #[inline(always)]
    fn default() -> TconSthb1 {
        <crate::RegValueT<TconSthb1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSthb2_SPEC;
impl crate::sealed::RegSpec for TconSthb2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x2"]
pub type TconSthb2 = crate::RegValueT<TconSthb2_SPEC>;

impl TconSthb2 {
    #[doc = "Output Signal Select Control for LCD_TCON2/LCD_TCON3 Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_sthb2::Sel,
        tcon_sthb2::Sel,
        TconSthb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_sthb2::Sel,
            tcon_sthb2::Sel,
            TconSthb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_sthb2::Inv,
        tcon_sthb2::Inv,
        TconSthb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_sthb2::Inv,
            tcon_sthb2::Inv,
            TconSthb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Reference Timing Control"]
    #[inline(always)]
    pub fn hssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tcon_sthb2::Hssel,
        tcon_sthb2::Hssel,
        TconSthb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tcon_sthb2::Hssel,
            tcon_sthb2::Hssel,
            TconSthb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconSthb2 {
    #[inline(always)]
    fn default() -> TconSthb2 {
        <crate::RegValueT<TconSthb2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_sthb2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hssel_SPEC;
    pub type Hssel = crate::EnumBitfieldStruct<u8, Hssel_SPEC>;
    impl Hssel {
        #[doc = "Select input horizontal synchronization signal (HSIN) as reference for signal generation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select offset specified in TCON_TIM.OFFSET\\[10:0\\] (horizontal synchronization generation reference timing) as reference for signal generation"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Data Enable Signal DE Polarity Inversion Control"]
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
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Specified Line Detection Control"]
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

    #[doc = "Graphics 1 Underflow Detection Control"]
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

    #[doc = "Graphics 2 Underflow Detection Control"]
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
        #[doc = "Disable detection of specified line"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable detection of specified line"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfdtc_SPEC;
    pub type L1Undfdtc = crate::EnumBitfieldStruct<u8, L1Undfdtc_SPEC>;
    impl L1Undfdtc {
        #[doc = "Disable detection of graphics 1 underflow"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable detection of graphics 1 underflow"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfdtc_SPEC;
    pub type L2Undfdtc = crate::EnumBitfieldStruct<u8, L2Undfdtc_SPEC>;
    impl L2Undfdtc {
        #[doc = "Disable detection of graphics 2 underflow"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable detection of graphics 2 underflow"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Interrupt Request Signal GLCDC_VPOS Enable Control"]
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

    #[doc = "Interrupt Request Signal GLCDC_L1UNDF Enable Control"]
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

    #[doc = "Interrupt Request Signal GLCDC_L2UNDF Enable Control"]
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
        #[doc = "Disable GLCDC_VPOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC_VPOS output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfinten_SPEC;
    pub type L1Undfinten = crate::EnumBitfieldStruct<u8, L1Undfinten_SPEC>;
    impl L1Undfinten {
        #[doc = "Disable GLCDC_L1UNDF output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC_L1UNDF output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfinten_SPEC;
    pub type L2Undfinten = crate::EnumBitfieldStruct<u8, L2Undfinten_SPEC>;
    impl L2Undfinten {
        #[doc = "Disable GLCDC_L2UNDF output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC_L2UNDF output"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Graphics 2 Specified Line Detection Flag Clear"]
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

    #[doc = "Graphics 1 Underflow Detection Flag Clear"]
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

    #[doc = "Graphics 2 Underflow Detection Flag Clear"]
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
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the graphics 2 specified line detection flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfclr_SPEC;
    pub type L1Undfclr = crate::EnumBitfieldStruct<u8, L1Undfclr_SPEC>;
    impl L1Undfclr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the graphics 1 underflow detection flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfclr_SPEC;
    pub type L2Undfclr = crate::EnumBitfieldStruct<u8, L2Undfclr_SPEC>;
    impl L2Undfclr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the graphics 2 underflow detection flag"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Graphics 2 Specified Line Detection Flag"]
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

    #[doc = "Graphics 1 Underflow Detection Flag"]
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

    #[doc = "Graphics 2 Underflow Detection Flag"]
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
        #[doc = "Specified line notification not detected in graphics 2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Specified line notification detected in graphics 2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undf_SPEC;
    pub type L1Undf = crate::EnumBitfieldStruct<u8, L1Undf_SPEC>;
    impl L1Undf {
        #[doc = "No underflow detected in graphics 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow detected in graphics 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undf_SPEC;
    pub type L2Undf = crate::EnumBitfieldStruct<u8, L2Undf_SPEC>;
    impl L2Undf {
        #[doc = "No underflow detected in graphics 2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow detected in graphics 2"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Clock Division Ratio Setting Control"]
    #[inline(always)]
    pub fn dcdr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Panel Clock Output Enable Control"]
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

    #[doc = "Panel Clock Supply Source Control"]
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

    #[doc = "Pixel Clock Select Control"]
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

    #[doc = "Version Information"]
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

        #[doc = "Enable panel clock output Before changing the PIXSEL, CLKSEL, or DCDR bit, this bit must be set to 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "Select external clock (LCD_EXTCLK)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select LCDCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pixsel_SPEC;
    pub type Pixsel = crate::EnumBitfieldStruct<u8, Pixsel_SPEC>;
    impl Pixsel {
        #[doc = "Select no frequency division, parallel RGB"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select quarter frequency, serial RGB This setting must have the same value as OUT_SET.FRQSEL\\[1\\]."]
        pub const _1: Self = Self::new(1);
    }
}
