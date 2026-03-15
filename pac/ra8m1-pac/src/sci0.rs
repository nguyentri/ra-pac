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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Communication Interface 0"]
unsafe impl ::core::marker::Send for super::Sci0 {}
unsafe impl ::core::marker::Sync for super::Sci0 {}
impl super::Sci0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Received Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &'static crate::common::Reg<self::Rdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Transmission Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &'static crate::common::Reg<self::Tdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Transmission Data Register"]
    #[inline(always)]
    pub const fn tdr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::TdrHaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TdrHaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Transmission Data Register"]
    #[inline(always)]
    pub const fn tdr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::TdrByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TdrByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Transmission Data Register"]
    #[inline(always)]
    pub const fn tdr_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::TdrByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TdrByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0(&self) -> &'static crate::common::Reg<self::Ccr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr0HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr0ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr0ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr0HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr0ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Common Control Register 0"]
    #[inline(always)]
    pub const fn ccr0_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr0ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &'static crate::common::Reg<self::Ccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr1HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr1ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr1ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr1HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr1ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Common Control Register 1"]
    #[inline(always)]
    pub const fn ccr1_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr1ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &'static crate::common::Reg<self::Ccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr2HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr2ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr2ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr2HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr2ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Common Control Register 2"]
    #[inline(always)]
    pub const fn ccr2_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr2ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &'static crate::common::Reg<self::Ccr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr3HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr3ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr3ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(21usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr3HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr3ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Common Control Register 3"]
    #[inline(always)]
    pub const fn ccr3_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr3ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(23usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &'static crate::common::Reg<self::Ccr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr4HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr4ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr4ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(25usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr4HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr4ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "Common Control Register 4"]
    #[inline(always)]
    pub const fn ccr4_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Ccr4ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(27usize),
            )
        }
    }

    #[doc = "Communication Enable Status Register"]
    #[inline(always)]
    pub const fn cesr(&self) -> &'static crate::common::Reg<self::Cesr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cesr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "HBS valid mode Control Register"]
    #[inline(always)]
    pub const fn hcr(&self) -> &'static crate::common::Reg<self::Hcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "Simple-I2C Control Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &'static crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Simple-I2C Control Register"]
    #[inline(always)]
    pub const fn icr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::IcrHaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IcrHaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Simple-I2C Control Register"]
    #[inline(always)]
    pub const fn icr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::IcrByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IcrByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Simple-I2C Control Register"]
    #[inline(always)]
    pub const fn icr_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::IcrByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IcrByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33usize),
            )
        }
    }

    #[doc = "Simple-I2C Control Register"]
    #[inline(always)]
    pub const fn icr_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::IcrHaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IcrHaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "Simple-I2C Control Register"]
    #[inline(always)]
    pub const fn icr_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::IcrByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IcrByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &'static crate::common::Reg<self::Fcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::FcrHaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FcrHaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::FcrByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FcrByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::FcrByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FcrByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(37usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::FcrHaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FcrHaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::FcrByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FcrByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::FcrByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FcrByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(39usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &'static crate::common::Reg<self::Mcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::McrHaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::McrHaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::McrByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::McrByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::McrByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::McrByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(45usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::McrHaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::McrHaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::McrByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::McrByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "Manchester Control Register"]
    #[inline(always)]
    pub const fn mcr_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::McrByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::McrByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(47usize),
            )
        }
    }

    #[doc = "Driver Control Register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &'static crate::common::Reg<self::Dcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Driver Control Register"]
    #[inline(always)]
    pub const fn dcr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcrHaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcrHaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Driver Control Register"]
    #[inline(always)]
    pub const fn dcr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::DcrByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcrByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Driver Control Register"]
    #[inline(always)]
    pub const fn dcr_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::DcrByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcrByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
            )
        }
    }

    #[doc = "Driver Control Register"]
    #[inline(always)]
    pub const fn dcr_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcrHaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcrHaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "Driver Control Register"]
    #[inline(always)]
    pub const fn dcr_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::DcrByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcrByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 0"]
    #[inline(always)]
    pub const fn xcr0(&self) -> &'static crate::common::Reg<self::Xcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 0"]
    #[inline(always)]
    pub const fn xcr0_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr0HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 0"]
    #[inline(always)]
    pub const fn xcr0_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr0ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 0"]
    #[inline(always)]
    pub const fn xcr0_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr0ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(53usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 0"]
    #[inline(always)]
    pub const fn xcr0_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr0HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 0"]
    #[inline(always)]
    pub const fn xcr0_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr0ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 0"]
    #[inline(always)]
    pub const fn xcr0_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr0ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(55usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 1"]
    #[inline(always)]
    pub const fn xcr1(&self) -> &'static crate::common::Reg<self::Xcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 1"]
    #[inline(always)]
    pub const fn xcr1_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr1HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 1"]
    #[inline(always)]
    pub const fn xcr1_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr1ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 1"]
    #[inline(always)]
    pub const fn xcr1_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr1ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(57usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 1"]
    #[inline(always)]
    pub const fn xcr1_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr1HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 1"]
    #[inline(always)]
    pub const fn xcr1_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr1ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 1"]
    #[inline(always)]
    pub const fn xcr1_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr1ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(59usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 2"]
    #[inline(always)]
    pub const fn xcr2(&self) -> &'static crate::common::Reg<self::Xcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 2"]
    #[inline(always)]
    pub const fn xcr2_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr2HaL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2HaL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 2"]
    #[inline(always)]
    pub const fn xcr2_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr2ByLl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2ByLl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 2"]
    #[inline(always)]
    pub const fn xcr2_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr2ByLh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2ByLh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(61usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 2"]
    #[inline(always)]
    pub const fn xcr2_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr2HaH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2HaH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 2"]
    #[inline(always)]
    pub const fn xcr2_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr2ByHl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2ByHl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Control Register 2"]
    #[inline(always)]
    pub const fn xcr2_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::Xcr2ByHh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2ByHh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(63usize),
            )
        }
    }

    #[doc = "Common Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &'static crate::common::Reg<self::Csr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Csr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Simple-I2C Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &'static crate::common::Reg<self::Isr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Isr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "FIFO Receive Status Register"]
    #[inline(always)]
    pub const fn frsr(&self) -> &'static crate::common::Reg<self::Frsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "FIFO Transmit Status Register"]
    #[inline(always)]
    pub const fn ftsr(&self) -> &'static crate::common::Reg<self::Ftsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ftsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Manchester Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &'static crate::common::Reg<self::Msr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Msr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Status Register 0"]
    #[inline(always)]
    pub const fn xsr0(&self) -> &'static crate::common::Reg<self::Xsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Xsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Simple-LIN(SCIX) Status Register 1"]
    #[inline(always)]
    pub const fn xsr1(&self) -> &'static crate::common::Reg<self::Xsr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Xsr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Common Flag Clear Register"]
    #[inline(always)]
    pub const fn cfclr(&self) -> &'static crate::common::Reg<self::Cfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Common Flag Clear Register"]
    #[inline(always)]
    pub const fn cfclr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::CfclrHaL_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::CfclrHaL_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Common Flag Clear Register"]
    #[inline(always)]
    pub const fn cfclr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::CfclrByLl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::CfclrByLl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Common Flag Clear Register"]
    #[inline(always)]
    pub const fn cfclr_ha_h(
        &self,
    ) -> &'static crate::common::Reg<self::CfclrHaH_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::CfclrHaH_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[doc = "Common Flag Clear Register"]
    #[inline(always)]
    pub const fn cfclr_by_hl(
        &self,
    ) -> &'static crate::common::Reg<self::CfclrByHl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::CfclrByHl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[doc = "Common Flag Clear Register"]
    #[inline(always)]
    pub const fn cfclr_by_hh(
        &self,
    ) -> &'static crate::common::Reg<self::CfclrByHh_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::CfclrByHh_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(107usize),
            )
        }
    }

    #[doc = "Simple-I2C Flag Clear Register"]
    #[inline(always)]
    pub const fn icfclr(&self) -> &'static crate::common::Reg<self::Icfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Icfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Simple-I2C Flag Clear Register"]
    #[inline(always)]
    pub const fn icfclr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::IcfclrHaL_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::IcfclrHaL_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Simple-I2C Flag Clear Register"]
    #[inline(always)]
    pub const fn icfclr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::IcfclrByLl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::IcfclrByLl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "FIFO Flag Clear Register"]
    #[inline(always)]
    pub const fn ffclr(&self) -> &'static crate::common::Reg<self::Ffclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ffclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "FIFO Flag Clear Register"]
    #[inline(always)]
    pub const fn ffclr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::FfclrHaL_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::FfclrHaL_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "FIFO Flag Clear Register"]
    #[inline(always)]
    pub const fn ffclr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::FfclrByLl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::FfclrByLl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Manchester Flag Clear Register"]
    #[inline(always)]
    pub const fn mfclr(&self) -> &'static crate::common::Reg<self::Mfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Mfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Manchester Flag Clear Register"]
    #[inline(always)]
    pub const fn mfclr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::MfclrHaL_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::MfclrHaL_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Manchester Flag Clear Register"]
    #[inline(always)]
    pub const fn mfclr_by_ll(
        &self,
    ) -> &'static crate::common::Reg<self::MfclrByLl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::MfclrByLl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Simpe-LIN(SCIX) Flag Clear Register"]
    #[inline(always)]
    pub const fn xfclr(&self) -> &'static crate::common::Reg<self::Xfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Xfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Simpe-LIN(SCIX) Flag Clear Register"]
    #[inline(always)]
    pub const fn xfclr_ha_l(
        &self,
    ) -> &'static crate::common::Reg<self::XfclrHaL_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::XfclrHaL_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Simpe-LIN(SCIX) Flag Clear Register"]
    #[inline(always)]
    pub const fn xfclr_by_lh(
        &self,
    ) -> &'static crate::common::Reg<self::XfclrByLh_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::XfclrByLh_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(121usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr_SPEC;
impl crate::sealed::RegSpec for Rdr_SPEC {
    type DataType = u32;
}

#[doc = "Received Data Register"]
pub type Rdr = crate::RegValueT<Rdr_SPEC>;

impl Rdr {
    #[doc = "Received  data."]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multiprocessor bit flag"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rdr::Mpb, rdr::Mpb, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,rdr::Mpb,rdr::Mpb,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive data ready flag"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, rdr::Dr, rdr::Dr, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x1,1,0,rdr::Dr,rdr::Dr,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity error flag"]
    #[inline(always)]
    pub fn fper(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, rdr::Fper, rdr::Fper, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            rdr::Fper,
            rdr::Fper,
            Rdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framing error flag"]
    #[inline(always)]
    pub fn ffer(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, rdr::Ffer, rdr::Ffer, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rdr::Ffer,
            rdr::Ffer,
            Rdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overrun error flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, rdr::Orer, rdr::Orer, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            rdr::Orer,
            rdr::Orer,
            Rdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, rdr::Per, rdr::Per, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x1,1,0,rdr::Per,rdr::Per,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, rdr::Fer, rdr::Fer, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,rdr::Fer,rdr::Fer,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        <crate::RegValueT<Rdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data has remained in received-FIFO (RDR) after normally completed receiving."]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data has not been received for a period after normal completed receiving."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fper_SPEC;
    pub type Fper = crate::EnumBitfieldStruct<u8, Fper_SPEC>;
    impl Fper {
        #[doc = "No parity error occurred at the first data of received-FIFO (RDR)"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred at the first data of received-FIFO (RDR)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ffer_SPEC;
    pub type Ffer = crate::EnumBitfieldStruct<u8, Ffer_SPEC>;
    impl Ffer {
        #[doc = "No framing error occurred at the first data of received-FIFO (RDR)"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred at the first data of received-FIFO (RDR)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An overrun error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr_SPEC;
impl crate::sealed::RegSpec for Tdr_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Data Register"]
pub type Tdr = crate::RegValueT<Tdr_SPEC>;

impl Tdr {
    #[doc = "Transmission  data."]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Tdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Tdr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-processor transfer bit flag"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tdr::Mpbt, tdr::Mpbt, Tdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tdr::Mpbt,
            tdr::Mpbt,
            Tdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit sync data bit."]
    #[inline(always)]
    pub fn tsync(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tdr::Tsync,
        tdr::Tsync,
        Tdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tdr::Tsync,
            tdr::Tsync,
            Tdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 1111111111111111. The write value should be 1111111111111111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Tdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Tdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        <crate::RegValueT<Tdr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod tdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsync_SPEC;
    pub type Tsync = crate::EnumBitfieldStruct<u8, Tsync_SPEC>;
    impl Tsync {
        #[doc = "Start bit outputs data Sync"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit outputs command Sync"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TdrHaL_SPEC;
impl crate::sealed::RegSpec for TdrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Transmission Data Register"]
pub type TdrHaL = crate::RegValueT<TdrHaL_SPEC>;

impl TdrHaL {
    #[doc = "Transmission  data."]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, TdrHaL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,TdrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-processor transfer bit flag"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        tdr_ha_l::Mpbt,
        tdr_ha_l::Mpbt,
        TdrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tdr_ha_l::Mpbt,
            tdr_ha_l::Mpbt,
            TdrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit sync data bit."]
    #[inline(always)]
    pub fn tsync(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tdr_ha_l::Tsync,
        tdr_ha_l::Tsync,
        TdrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tdr_ha_l::Tsync,
            tdr_ha_l::Tsync,
            TdrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 111. The write value should be 111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, TdrHaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,TdrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TdrHaL {
    #[inline(always)]
    fn default() -> TdrHaL {
        <crate::RegValueT<TdrHaL_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod tdr_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsync_SPEC;
    pub type Tsync = crate::EnumBitfieldStruct<u8, Tsync_SPEC>;
    impl Tsync {
        #[doc = "Start bit outputs data Sync"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit outputs command Sync"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TdrByLl_SPEC;
impl crate::sealed::RegSpec for TdrByLl_SPEC {
    type DataType = u8;
}

#[doc = "Transmission Data Register"]
pub type TdrByLl = crate::RegValueT<TdrByLl_SPEC>;

impl TdrByLl {
    #[doc = "Transmission  data."]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, TdrByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,TdrByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TdrByLl {
    #[inline(always)]
    fn default() -> TdrByLl {
        <crate::RegValueT<TdrByLl_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TdrByLh_SPEC;
impl crate::sealed::RegSpec for TdrByLh_SPEC {
    type DataType = u8;
}

#[doc = "Transmission Data Register"]
pub type TdrByLh = crate::RegValueT<TdrByLh_SPEC>;

impl TdrByLh {
    #[doc = "Transmission  data."]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TdrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, TdrByLh_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Multi-processor transfer bit flag"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tdr_by_lh::Mpbt,
        tdr_by_lh::Mpbt,
        TdrByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tdr_by_lh::Mpbt,
            tdr_by_lh::Mpbt,
            TdrByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit sync data bit."]
    #[inline(always)]
    pub fn tsync(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tdr_by_lh::Tsync,
        tdr_by_lh::Tsync,
        TdrByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tdr_by_lh::Tsync,
            tdr_by_lh::Tsync,
            TdrByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 111. The write value should be 111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, TdrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,TdrByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TdrByLh {
    #[inline(always)]
    fn default() -> TdrByLh {
        <crate::RegValueT<TdrByLh_SPEC> as RegisterValue<_>>::new(255)
    }
}
pub mod tdr_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "ID transmission cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsync_SPEC;
    pub type Tsync = crate::EnumBitfieldStruct<u8, Tsync_SPEC>;
    impl Tsync {
        #[doc = "Start bit outputs data Sync"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit outputs command Sync"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0_SPEC;
impl crate::sealed::RegSpec for Ccr0_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 0"]
pub type Ccr0 = crate::RegValueT<Ccr0_SPEC>;

impl Ccr0 {
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ccr0::Re, ccr0::Re, Ccr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ccr0::Re,ccr0::Re,Ccr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ccr0::Te, ccr0::Te, Ccr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ccr0::Te,ccr0::Te,Ccr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccr0::Mpie,
        ccr0::Mpie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccr0::Mpie,
            ccr0::Mpie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Enable"]
    #[inline(always)]
    pub fn dcme(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ccr0::Dcme,
        ccr0::Dcme,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ccr0::Dcme,
            ccr0::Dcme,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ID frame select Bit"]
    #[inline(always)]
    pub fn idsel(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ccr0::Idsel,
        ccr0::Idsel,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ccr0::Idsel,
            ccr0::Idsel,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr0::Rie,
        ccr0::Rie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr0::Rie,
            ccr0::Rie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ccr0::Tie,
        ccr0::Tie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr0::Tie,
            ccr0::Tie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ccr0::Teie,
        ccr0::Teie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ccr0::Teie,
            ccr0::Teie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSn# Pin Function Enable"]
    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ccr0::Sse,
        ccr0::Sse,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ccr0::Sse,
            ccr0::Sse,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, u8, Ccr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x7f,1,0,u8,u8,Ccr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr0 {
    #[inline(always)]
    fn default() -> Ccr0 {
        <crate::RegValueT<Ccr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Serial reception is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial reception is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Serial transmission is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial transmission is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        #[doc = "Normal reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "When the data with the multi-processor bit set to \"0\" is received, the data is not read, and setting the status flags RDRF,ORER FER ,MER ,SYER , PFER and SBER to \"1\" is disabled. When the data with the multiprocessor bit set to \"1\" is received, the MPIE bit is automatically cleared to \"0\", and normal reception is resumed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        #[doc = "Address match function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address match function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        #[doc = "It\'s always compared data in spite of the value of the MPB bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "It\'s compared data when the MPB bit is “1” ( ID frame ) only."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "RXI and ERI interrupt requests are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RXI and ERI interrupt requests are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "A TXI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TXI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "A TEI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TEI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        #[doc = "SSn# pin function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "SSn# pin function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0HaL_SPEC;
impl crate::sealed::RegSpec for Ccr0HaL_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 0"]
pub type Ccr0HaL = crate::RegValueT<Ccr0HaL_SPEC>;

impl Ccr0HaL {
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr0_ha_l::Re,
        ccr0_ha_l::Re,
        Ccr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr0_ha_l::Re,
            ccr0_ha_l::Re,
            Ccr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr0_ha_l::Te,
        ccr0_ha_l::Te,
        Ccr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr0_ha_l::Te,
            ccr0_ha_l::Te,
            Ccr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccr0_ha_l::Mpie,
        ccr0_ha_l::Mpie,
        Ccr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccr0_ha_l::Mpie,
            ccr0_ha_l::Mpie,
            Ccr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Enable"]
    #[inline(always)]
    pub fn dcme(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ccr0_ha_l::Dcme,
        ccr0_ha_l::Dcme,
        Ccr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ccr0_ha_l::Dcme,
            ccr0_ha_l::Dcme,
            Ccr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ID frame select Bit"]
    #[inline(always)]
    pub fn idsel(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ccr0_ha_l::Idsel,
        ccr0_ha_l::Idsel,
        Ccr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ccr0_ha_l::Idsel,
            ccr0_ha_l::Idsel,
            Ccr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, Ccr0HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,Ccr0HaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr0HaL {
    #[inline(always)]
    fn default() -> Ccr0HaL {
        <crate::RegValueT<Ccr0HaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Serial reception is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial reception is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Serial transmission is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial transmission is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        #[doc = "Normal reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "When the data with the multi-processor bit set to \"0\" is received, the data is not read, and setting the status flags RDRF,ORER FER ,MER ,SYER , PFER and SBER to \"1\" is disabled. When the data with the multiprocessor bit set to \"1\" is received, the MPIE bit is automatically cleared to \"0\", and normal reception is resumed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        #[doc = "Address match function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address match function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        #[doc = "It\'s always compared data in spite of the value of the MPB bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "It\'s compared data when the MPB bit is “1” ( ID frame ) only."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0ByLl_SPEC;
impl crate::sealed::RegSpec for Ccr0ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 0"]
pub type Ccr0ByLl = crate::RegValueT<Ccr0ByLl_SPEC>;

impl Ccr0ByLl {
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr0_by_ll::Re,
        ccr0_by_ll::Re,
        Ccr0ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr0_by_ll::Re,
            ccr0_by_ll::Re,
            Ccr0ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr0_by_ll::Te,
        ccr0_by_ll::Te,
        Ccr0ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr0_by_ll::Te,
            ccr0_by_ll::Te,
            Ccr0ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Ccr0ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Ccr0ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr0ByLl {
    #[inline(always)]
    fn default() -> Ccr0ByLl {
        <crate::RegValueT<Ccr0ByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Serial reception is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial reception is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Serial transmission is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial transmission is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0ByLh_SPEC;
impl crate::sealed::RegSpec for Ccr0ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 0"]
pub type Ccr0ByLh = crate::RegValueT<Ccr0ByLh_SPEC>;

impl Ccr0ByLh {
    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr0_by_lh::Mpie,
        ccr0_by_lh::Mpie,
        Ccr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr0_by_lh::Mpie,
            ccr0_by_lh::Mpie,
            Ccr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Enable"]
    #[inline(always)]
    pub fn dcme(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr0_by_lh::Dcme,
        ccr0_by_lh::Dcme,
        Ccr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr0_by_lh::Dcme,
            ccr0_by_lh::Dcme,
            Ccr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ID frame select Bit"]
    #[inline(always)]
    pub fn idsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ccr0_by_lh::Idsel,
        ccr0_by_lh::Idsel,
        Ccr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ccr0_by_lh::Idsel,
            ccr0_by_lh::Idsel,
            Ccr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ccr0ByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ccr0ByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr0ByLh {
    #[inline(always)]
    fn default() -> Ccr0ByLh {
        <crate::RegValueT<Ccr0ByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        #[doc = "Normal reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "When the data with the multi-processor bit set to \"0\" is received, the data is not read, and setting the status flags RDRF,ORER FER ,MER ,SYER , PFER and SBER to \"1\" is disabled. When the data with the multiprocessor bit set to \"1\" is received, the MPIE bit is automatically cleared to \"0\", and normal reception is resumed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        #[doc = "Address match function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address match function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        #[doc = "It\'s always compared data in spite of the value of the MPB bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "It\'s compared data when the MPB bit is “1” ( ID frame ) only."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0HaH_SPEC;
impl crate::sealed::RegSpec for Ccr0HaH_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 0"]
pub type Ccr0HaH = crate::RegValueT<Ccr0HaH_SPEC>;

impl Ccr0HaH {
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr0_ha_h::Rie,
        ccr0_ha_h::Rie,
        Ccr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr0_ha_h::Rie,
            ccr0_ha_h::Rie,
            Ccr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr0_ha_h::Tie,
        ccr0_ha_h::Tie,
        Ccr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr0_ha_h::Tie,
            ccr0_ha_h::Tie,
            Ccr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr0_ha_h::Teie,
        ccr0_ha_h::Teie,
        Ccr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr0_ha_h::Teie,
            ccr0_ha_h::Teie,
            Ccr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSn# Pin Function Enable"]
    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccr0_ha_h::Sse,
        ccr0_ha_h::Sse,
        Ccr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccr0_ha_h::Sse,
            ccr0_ha_h::Sse,
            Ccr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Ccr0HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Ccr0HaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr0HaH {
    #[inline(always)]
    fn default() -> Ccr0HaH {
        <crate::RegValueT<Ccr0HaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "RXI and ERI interrupt requests are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RXI and ERI interrupt requests are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "A TXI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TXI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "A TEI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TEI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        #[doc = "SSn# pin function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "SSn# pin function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0ByHl_SPEC;
impl crate::sealed::RegSpec for Ccr0ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 0"]
pub type Ccr0ByHl = crate::RegValueT<Ccr0ByHl_SPEC>;

impl Ccr0ByHl {
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr0_by_hl::Rie,
        ccr0_by_hl::Rie,
        Ccr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr0_by_hl::Rie,
            ccr0_by_hl::Rie,
            Ccr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr0_by_hl::Tie,
        ccr0_by_hl::Tie,
        Ccr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr0_by_hl::Tie,
            ccr0_by_hl::Tie,
            Ccr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr0_by_hl::Teie,
        ccr0_by_hl::Teie,
        Ccr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr0_by_hl::Teie,
            ccr0_by_hl::Teie,
            Ccr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ccr0ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ccr0ByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr0ByHl {
    #[inline(always)]
    fn default() -> Ccr0ByHl {
        <crate::RegValueT<Ccr0ByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "RXI and ERI interrupt requests are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RXI and ERI interrupt requests are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "A TXI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TXI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "A TEI interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TEI interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0ByHh_SPEC;
impl crate::sealed::RegSpec for Ccr0ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 0"]
pub type Ccr0ByHh = crate::RegValueT<Ccr0ByHh_SPEC>;

impl Ccr0ByHh {
    #[doc = "SSn# Pin Function Enable"]
    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr0_by_hh::Sse,
        ccr0_by_hh::Sse,
        Ccr0ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr0_by_hh::Sse,
            ccr0_by_hh::Sse,
            Ccr0ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Ccr0ByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Ccr0ByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr0ByHh {
    #[inline(always)]
    fn default() -> Ccr0ByHh {
        <crate::RegValueT<Ccr0ByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0_by_hh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        #[doc = "SSn# pin function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "SSn# pin function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1_SPEC;
impl crate::sealed::RegSpec for Ccr1_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 1"]
pub type Ccr1 = crate::RegValueT<Ccr1_SPEC>;

impl Ccr1 {
    #[doc = "CTS Enable"]
    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1::Ctse,
        ccr1::Ctse,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1::Ctse,
            ccr1::Ctse,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTS external terminal enable bit."]
    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr1::Ctspen,
        ccr1::Ctspen,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr1::Ctspen,
            ccr1::Ctspen,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial port break data select bit"]
    #[inline(always)]
    pub fn spb2dt(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1::Spb2Dt,
        ccr1::Spb2Dt,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1::Spb2Dt,
            ccr1::Spb2Dt,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial port break I/O bit"]
    #[inline(always)]
    pub fn spb2io(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr1::Spb2Io,
        ccr1::Spb2Io,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr1::Spb2Io,
            ccr1::Spb2Io,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ccr1::Pe, ccr1::Pe, Ccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ccr1::Pe,ccr1::Pe,Ccr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ccr1::Pm, ccr1::Pm, Ccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ccr1::Pm,ccr1::Pm,Ccr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TxD invert bit"]
    #[inline(always)]
    pub fn tinv(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr1::Tinv,
        ccr1::Tinv,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr1::Tinv,
            ccr1::Tinv,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RxD invert bit"]
    #[inline(always)]
    pub fn rinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr1::Rinv,
        ccr1::Rinv,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr1::Rinv,
            ccr1::Rinv,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial communication Port LoopBack"]
    #[inline(always)]
    pub fn splp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr1::Splp,
        ccr1::Splp,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr1::Splp,
            ccr1::Splp,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TxD/RxD Pin Multiplexing Select"]
    #[inline(always)]
    pub fn sharps(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ccr1::Sharps,
        ccr1::Sharps,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr1::Sharps,
            ccr1::Sharps,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ccr1::Nfcs,
        ccr1::Nfcs,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ccr1::Nfcs,
            ccr1::Nfcs,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ccr1::Nfen,
        ccr1::Nfen,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ccr1::Nfen,
            ccr1::Nfen,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Ccr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Ccr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        <crate::RegValueT<Ccr1_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod ccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        #[doc = "CTS function is disabled (RTS output function is enabled)."]
        pub const _0: Self = Self::new(0);

        #[doc = "CTS function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        #[doc = "Alternate setting for using either CTS / RTS function as one terminal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Dedicated setting to simultaneously use CTS / RTS function with 2 pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Dt_SPEC;
    pub type Spb2Dt = crate::EnumBitfieldStruct<u8, Spb2Dt_SPEC>;
    impl Spb2Dt {
        #[doc = "Low level is output in TxD terminal, when TINV is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "High level is output in TxD terminal, when TINV is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Io_SPEC;
    pub type Spb2Io = crate::EnumBitfieldStruct<u8, Spb2Io_SPEC>;
    impl Spb2Io {
        #[doc = "The value of SPB2DT bit isn\'t output in TxD terminal."]
        pub const _0: Self = Self::new(0);

        #[doc = "The value of SPB2DT bit is output in TxD terminal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "Parity bit addition or checking is not performed"]
        pub const _0: Self = Self::new(0);

        #[doc = "The parity bit is added or checked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Selects even parity"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects odd parity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tinv_SPEC;
    pub type Tinv = crate::EnumBitfieldStruct<u8, Tinv_SPEC>;
    impl Tinv {
        #[doc = "Transmit data is output directly to TxD."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit data is inverted and output to TxD."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rinv_SPEC;
    pub type Rinv = crate::EnumBitfieldStruct<u8, Rinv_SPEC>;
    impl Rinv {
        #[doc = "Received data from RxD is input directly."]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data from RxD is inverted and input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp_SPEC;
    pub type Splp = crate::EnumBitfieldStruct<u8, Splp_SPEC>;
    impl Splp {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loopback mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sharps_SPEC;
    pub type Sharps = crate::EnumBitfieldStruct<u8, Sharps_SPEC>;
    impl Sharps {
        #[doc = "The TxD and RxD pins are independent."]
        pub const _0: Self = Self::new(0);

        #[doc = "The TxD and RxD signals are multiplexed on the same pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "The base clock divided by 1 is used with the noise filter."]
        pub const _000: Self = Self::new(0);

        #[doc = "The on-chip baud rate generator clock divided by 1 is used"]
        pub const _001: Self = Self::new(1);

        #[doc = "The on-chip baud rate generator clock divided by 2 is used"]
        pub const _010: Self = Self::new(2);

        #[doc = "The on-chip baud rate generator clock divided by 4 is used"]
        pub const _011: Self = Self::new(3);

        #[doc = "The on-chip baud rate generator clock divided by 8 is used"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1HaL_SPEC;
impl crate::sealed::RegSpec for Ccr1HaL_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 1"]
pub type Ccr1HaL = crate::RegValueT<Ccr1HaL_SPEC>;

impl Ccr1HaL {
    #[doc = "CTS Enable"]
    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1_ha_l::Ctse,
        ccr1_ha_l::Ctse,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1_ha_l::Ctse,
            ccr1_ha_l::Ctse,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTS external terminal enable bit."]
    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr1_ha_l::Ctspen,
        ccr1_ha_l::Ctspen,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr1_ha_l::Ctspen,
            ccr1_ha_l::Ctspen,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial port break data select bit"]
    #[inline(always)]
    pub fn spb2dt(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1_ha_l::Spb2Dt,
        ccr1_ha_l::Spb2Dt,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1_ha_l::Spb2Dt,
            ccr1_ha_l::Spb2Dt,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial port break I/O bit"]
    #[inline(always)]
    pub fn spb2io(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr1_ha_l::Spb2Io,
        ccr1_ha_l::Spb2Io,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr1_ha_l::Spb2Io,
            ccr1_ha_l::Spb2Io,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccr1_ha_l::Pe,
        ccr1_ha_l::Pe,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccr1_ha_l::Pe,
            ccr1_ha_l::Pe,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ccr1_ha_l::Pm,
        ccr1_ha_l::Pm,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ccr1_ha_l::Pm,
            ccr1_ha_l::Pm,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TxD invert bit"]
    #[inline(always)]
    pub fn tinv(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr1_ha_l::Tinv,
        ccr1_ha_l::Tinv,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr1_ha_l::Tinv,
            ccr1_ha_l::Tinv,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RxD invert bit"]
    #[inline(always)]
    pub fn rinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr1_ha_l::Rinv,
        ccr1_ha_l::Rinv,
        Ccr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr1_ha_l::Rinv,
            ccr1_ha_l::Rinv,
            Ccr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Ccr1HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Ccr1HaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr1HaL {
    #[inline(always)]
    fn default() -> Ccr1HaL {
        <crate::RegValueT<Ccr1HaL_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod ccr1_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        #[doc = "CTS function is disabled (RTS output function is enabled)."]
        pub const _0: Self = Self::new(0);

        #[doc = "CTS function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        #[doc = "Alternate setting for using either CTS / RTS function as one terminal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Dedicated setting to simultaneously use CTS / RTS function with 2 pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Dt_SPEC;
    pub type Spb2Dt = crate::EnumBitfieldStruct<u8, Spb2Dt_SPEC>;
    impl Spb2Dt {
        #[doc = "Low level is output in TxD terminal, when TINV is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "High level is output in TxD terminal, when TINV is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Io_SPEC;
    pub type Spb2Io = crate::EnumBitfieldStruct<u8, Spb2Io_SPEC>;
    impl Spb2Io {
        #[doc = "The value of SPB2DT bit isn\'t output in TxD terminal."]
        pub const _0: Self = Self::new(0);

        #[doc = "The value of SPB2DT bit is output in TxD terminal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "Parity bit addition or checking is not performed"]
        pub const _0: Self = Self::new(0);

        #[doc = "The parity bit is added or checked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Selects even parity"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects odd parity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tinv_SPEC;
    pub type Tinv = crate::EnumBitfieldStruct<u8, Tinv_SPEC>;
    impl Tinv {
        #[doc = "Transmit data is output directly to TxD."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit data is inverted and output to TxD."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rinv_SPEC;
    pub type Rinv = crate::EnumBitfieldStruct<u8, Rinv_SPEC>;
    impl Rinv {
        #[doc = "Received data from RxD is input directly."]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data from RxD is inverted and input."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1ByLl_SPEC;
impl crate::sealed::RegSpec for Ccr1ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 1"]
pub type Ccr1ByLl = crate::RegValueT<Ccr1ByLl_SPEC>;

impl Ccr1ByLl {
    #[doc = "CTS Enable"]
    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1_by_ll::Ctse,
        ccr1_by_ll::Ctse,
        Ccr1ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1_by_ll::Ctse,
            ccr1_by_ll::Ctse,
            Ccr1ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTS external terminal enable bit."]
    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr1_by_ll::Ctspen,
        ccr1_by_ll::Ctspen,
        Ccr1ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr1_by_ll::Ctspen,
            ccr1_by_ll::Ctspen,
            Ccr1ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial port break data select bit"]
    #[inline(always)]
    pub fn spb2dt(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1_by_ll::Spb2Dt,
        ccr1_by_ll::Spb2Dt,
        Ccr1ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1_by_ll::Spb2Dt,
            ccr1_by_ll::Spb2Dt,
            Ccr1ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial port break I/O bit"]
    #[inline(always)]
    pub fn spb2io(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr1_by_ll::Spb2Io,
        ccr1_by_ll::Spb2Io,
        Ccr1ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr1_by_ll::Spb2Io,
            ccr1_by_ll::Spb2Io,
            Ccr1ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ccr1ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ccr1ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr1ByLl {
    #[inline(always)]
    fn default() -> Ccr1ByLl {
        <crate::RegValueT<Ccr1ByLl_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod ccr1_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        #[doc = "CTS function is disabled (RTS output function is enabled)."]
        pub const _0: Self = Self::new(0);

        #[doc = "CTS function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        #[doc = "Alternate setting for using either CTS / RTS function as one terminal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Dedicated setting to simultaneously use CTS / RTS function with 2 pins"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Dt_SPEC;
    pub type Spb2Dt = crate::EnumBitfieldStruct<u8, Spb2Dt_SPEC>;
    impl Spb2Dt {
        #[doc = "Low level is output in TxD terminal, when TINV is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "High level is output in TxD terminal, when TINV is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Io_SPEC;
    pub type Spb2Io = crate::EnumBitfieldStruct<u8, Spb2Io_SPEC>;
    impl Spb2Io {
        #[doc = "The value of SPB2DT bit isn\'t output in TxD terminal."]
        pub const _0: Self = Self::new(0);

        #[doc = "The value of SPB2DT bit is output in TxD terminal."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1ByLh_SPEC;
impl crate::sealed::RegSpec for Ccr1ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 1"]
pub type Ccr1ByLh = crate::RegValueT<Ccr1ByLh_SPEC>;

impl Ccr1ByLh {
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1_by_lh::Pe,
        ccr1_by_lh::Pe,
        Ccr1ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1_by_lh::Pe,
            ccr1_by_lh::Pe,
            Ccr1ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr1_by_lh::Pm,
        ccr1_by_lh::Pm,
        Ccr1ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr1_by_lh::Pm,
            ccr1_by_lh::Pm,
            Ccr1ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TxD invert bit"]
    #[inline(always)]
    pub fn tinv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1_by_lh::Tinv,
        ccr1_by_lh::Tinv,
        Ccr1ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1_by_lh::Tinv,
            ccr1_by_lh::Tinv,
            Ccr1ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RxD invert bit"]
    #[inline(always)]
    pub fn rinv(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr1_by_lh::Rinv,
        ccr1_by_lh::Rinv,
        Ccr1ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr1_by_lh::Rinv,
            ccr1_by_lh::Rinv,
            Ccr1ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ccr1ByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ccr1ByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr1ByLh {
    #[inline(always)]
    fn default() -> Ccr1ByLh {
        <crate::RegValueT<Ccr1ByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr1_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "Parity bit addition or checking is not performed"]
        pub const _0: Self = Self::new(0);

        #[doc = "The parity bit is added or checked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Selects even parity"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects odd parity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tinv_SPEC;
    pub type Tinv = crate::EnumBitfieldStruct<u8, Tinv_SPEC>;
    impl Tinv {
        #[doc = "Transmit data is output directly to TxD."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit data is inverted and output to TxD."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rinv_SPEC;
    pub type Rinv = crate::EnumBitfieldStruct<u8, Rinv_SPEC>;
    impl Rinv {
        #[doc = "Received data from RxD is input directly."]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data from RxD is inverted and input."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1HaH_SPEC;
impl crate::sealed::RegSpec for Ccr1HaH_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 1"]
pub type Ccr1HaH = crate::RegValueT<Ccr1HaH_SPEC>;

impl Ccr1HaH {
    #[doc = "Serial communication Port LoopBack"]
    #[inline(always)]
    pub fn splp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1_ha_h::Splp,
        ccr1_ha_h::Splp,
        Ccr1HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1_ha_h::Splp,
            ccr1_ha_h::Splp,
            Ccr1HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TxD/RxD Pin Multiplexing Select"]
    #[inline(always)]
    pub fn sharps(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1_ha_h::Sharps,
        ccr1_ha_h::Sharps,
        Ccr1HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1_ha_h::Sharps,
            ccr1_ha_h::Sharps,
            Ccr1HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        ccr1_ha_h::Nfcs,
        ccr1_ha_h::Nfcs,
        Ccr1HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            ccr1_ha_h::Nfcs,
            ccr1_ha_h::Nfcs,
            Ccr1HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr1_ha_h::Nfen,
        ccr1_ha_h::Nfen,
        Ccr1HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr1_ha_h::Nfen,
            ccr1_ha_h::Nfen,
            Ccr1HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, Ccr1HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,Ccr1HaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr1HaH {
    #[inline(always)]
    fn default() -> Ccr1HaH {
        <crate::RegValueT<Ccr1HaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr1_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp_SPEC;
    pub type Splp = crate::EnumBitfieldStruct<u8, Splp_SPEC>;
    impl Splp {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loopback mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sharps_SPEC;
    pub type Sharps = crate::EnumBitfieldStruct<u8, Sharps_SPEC>;
    impl Sharps {
        #[doc = "The TxD and RxD pins are independent."]
        pub const _0: Self = Self::new(0);

        #[doc = "The TxD and RxD signals are multiplexed on the same pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "The base clock divided by 1 is used with the noise filter."]
        pub const _000: Self = Self::new(0);

        #[doc = "The on-chip baud rate generator clock divided by 1 is used"]
        pub const _001: Self = Self::new(1);

        #[doc = "The on-chip baud rate generator clock divided by 2 is used"]
        pub const _010: Self = Self::new(2);

        #[doc = "The on-chip baud rate generator clock divided by 4 is used"]
        pub const _011: Self = Self::new(3);

        #[doc = "The on-chip baud rate generator clock divided by 8 is used"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1ByHl_SPEC;
impl crate::sealed::RegSpec for Ccr1ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 1"]
pub type Ccr1ByHl = crate::RegValueT<Ccr1ByHl_SPEC>;

impl Ccr1ByHl {
    #[doc = "Serial communication Port LoopBack"]
    #[inline(always)]
    pub fn splp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1_by_hl::Splp,
        ccr1_by_hl::Splp,
        Ccr1ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1_by_hl::Splp,
            ccr1_by_hl::Splp,
            Ccr1ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TxD/RxD Pin Multiplexing Select"]
    #[inline(always)]
    pub fn sharps(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1_by_hl::Sharps,
        ccr1_by_hl::Sharps,
        Ccr1ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1_by_hl::Sharps,
            ccr1_by_hl::Sharps,
            Ccr1ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Ccr1ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Ccr1ByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr1ByHl {
    #[inline(always)]
    fn default() -> Ccr1ByHl {
        <crate::RegValueT<Ccr1ByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr1_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp_SPEC;
    pub type Splp = crate::EnumBitfieldStruct<u8, Splp_SPEC>;
    impl Splp {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loopback mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sharps_SPEC;
    pub type Sharps = crate::EnumBitfieldStruct<u8, Sharps_SPEC>;
    impl Sharps {
        #[doc = "The TxD and RxD pins are independent."]
        pub const _0: Self = Self::new(0);

        #[doc = "The TxD and RxD signals are multiplexed on the same pin."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1ByHh_SPEC;
impl crate::sealed::RegSpec for Ccr1ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 1"]
pub type Ccr1ByHh = crate::RegValueT<Ccr1ByHh_SPEC>;

impl Ccr1ByHh {
    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ccr1_by_hh::Nfcs,
        ccr1_by_hh::Nfcs,
        Ccr1ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ccr1_by_hh::Nfcs,
            ccr1_by_hh::Nfcs,
            Ccr1ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1_by_hh::Nfen,
        ccr1_by_hh::Nfen,
        Ccr1ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1_by_hh::Nfen,
            ccr1_by_hh::Nfen,
            Ccr1ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Ccr1ByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Ccr1ByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr1ByHh {
    #[inline(always)]
    fn default() -> Ccr1ByHh {
        <crate::RegValueT<Ccr1ByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr1_by_hh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "The base clock divided by 1 is used with the noise filter."]
        pub const _000: Self = Self::new(0);

        #[doc = "The on-chip baud rate generator clock divided by 1 is used"]
        pub const _001: Self = Self::new(1);

        #[doc = "The on-chip baud rate generator clock divided by 2 is used"]
        pub const _010: Self = Self::new(2);

        #[doc = "The on-chip baud rate generator clock divided by 4 is used"]
        pub const _011: Self = Self::new(3);

        #[doc = "The on-chip baud rate generator clock divided by 8 is used"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Noise cancellation function for the RXDn/TXDn input signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2_SPEC;
impl crate::sealed::RegSpec for Ccr2_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 2"]
pub type Ccr2 = crate::RegValueT<Ccr2_SPEC>;

impl Ccr2 {
    #[doc = "Basic clock pulse"]
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr2::Bgdm,
        ccr2::Bgdm,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr2::Bgdm,
            ccr2::Bgdm,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr2::Abcs,
        ccr2::Abcs,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr2::Abcs,
            ccr2::Abcs,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select1"]
    #[inline(always)]
    pub fn abcse(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ccr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub fn brr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bit Modulation Enable"]
    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr2::Brme,
        ccr2::Brme,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr2::Brme,
            ccr2::Brme,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ccr2::Cks,
        ccr2::Cks,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ccr2::Cks,
            ccr2::Cks,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MDDR corrects the bit rate adjusted by the BRR setting"]
    #[inline(always)]
    pub fn mddr(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        <crate::RegValueT<Ccr2_SPEC> as RegisterValue<_>>::new(4278255364)
    }
}
pub mod ccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        #[doc = "Baud rate generator outputs the clock with normal frequency."]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate generator outputs the clock with doubled frequency."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        #[doc = "Selects 16 base clock cycles for 1-bit period."]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects 8 base clock cycles for 1-bit period."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Bit rate modulation function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate modulation function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "TCLK clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "TCLK/4 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2HaL_SPEC;
impl crate::sealed::RegSpec for Ccr2HaL_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 2"]
pub type Ccr2HaL = crate::RegValueT<Ccr2HaL_SPEC>;

impl Ccr2HaL {
    #[doc = "Basic clock pulse"]
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Ccr2HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Ccr2HaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr2_ha_l::Bgdm,
        ccr2_ha_l::Bgdm,
        Ccr2HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr2_ha_l::Bgdm,
            ccr2_ha_l::Bgdm,
            Ccr2HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr2_ha_l::Abcs,
        ccr2_ha_l::Abcs,
        Ccr2HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr2_ha_l::Abcs,
            ccr2_ha_l::Abcs,
            Ccr2HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select1"]
    #[inline(always)]
    pub fn abcse(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ccr2HaL_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ccr2HaL_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ccr2HaL_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ccr2HaL_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub fn brr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ccr2HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ccr2HaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2HaL {
    #[inline(always)]
    fn default() -> Ccr2HaL {
        <crate::RegValueT<Ccr2HaL_SPEC> as RegisterValue<_>>::new(65284)
    }
}
pub mod ccr2_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        #[doc = "Baud rate generator outputs the clock with normal frequency."]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate generator outputs the clock with doubled frequency."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        #[doc = "Selects 16 base clock cycles for 1-bit period."]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects 8 base clock cycles for 1-bit period."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2ByLl_SPEC;
impl crate::sealed::RegSpec for Ccr2ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 2"]
pub type Ccr2ByLl = crate::RegValueT<Ccr2ByLl_SPEC>;

impl Ccr2ByLl {
    #[doc = "Basic clock pulse"]
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Ccr2ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Ccr2ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr2_by_ll::Bgdm,
        ccr2_by_ll::Bgdm,
        Ccr2ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr2_by_ll::Bgdm,
            ccr2_by_ll::Bgdm,
            Ccr2ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr2_by_ll::Abcs,
        ccr2_by_ll::Abcs,
        Ccr2ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr2_by_ll::Abcs,
            ccr2_by_ll::Abcs,
            Ccr2ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Base Clock Select1"]
    #[inline(always)]
    pub fn abcse(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ccr2ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ccr2ByLl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ccr2ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ccr2ByLl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ccr2ByLl {
    #[inline(always)]
    fn default() -> Ccr2ByLl {
        <crate::RegValueT<Ccr2ByLl_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod ccr2_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        #[doc = "Baud rate generator outputs the clock with normal frequency."]
        pub const _0: Self = Self::new(0);

        #[doc = "Baud rate generator outputs the clock with doubled frequency."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        #[doc = "Selects 16 base clock cycles for 1-bit period."]
        pub const _0: Self = Self::new(0);

        #[doc = "Selects 8 base clock cycles for 1-bit period."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2ByLh_SPEC;
impl crate::sealed::RegSpec for Ccr2ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 2"]
pub type Ccr2ByLh = crate::RegValueT<Ccr2ByLh_SPEC>;

impl Ccr2ByLh {
    #[doc = "BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub fn brr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ccr2ByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ccr2ByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2ByLh {
    #[inline(always)]
    fn default() -> Ccr2ByLh {
        <crate::RegValueT<Ccr2ByLh_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2HaH_SPEC;
impl crate::sealed::RegSpec for Ccr2HaH_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 2"]
pub type Ccr2HaH = crate::RegValueT<Ccr2HaH_SPEC>;

impl Ccr2HaH {
    #[doc = "Bit Modulation Enable"]
    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr2_ha_h::Brme,
        ccr2_ha_h::Brme,
        Ccr2HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr2_ha_h::Brme,
            ccr2_ha_h::Brme,
            Ccr2HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ccr2_ha_h::Cks,
        ccr2_ha_h::Cks,
        Ccr2HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ccr2_ha_h::Cks,
            ccr2_ha_h::Cks,
            Ccr2HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ccr2HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ccr2HaH_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MDDR corrects the bit rate adjusted by the BRR setting"]
    #[inline(always)]
    pub fn mddr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ccr2HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ccr2HaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2HaH {
    #[inline(always)]
    fn default() -> Ccr2HaH {
        <crate::RegValueT<Ccr2HaH_SPEC> as RegisterValue<_>>::new(65280)
    }
}
pub mod ccr2_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Bit rate modulation function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate modulation function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "TCLK clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "TCLK/4 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2ByHl_SPEC;
impl crate::sealed::RegSpec for Ccr2ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 2"]
pub type Ccr2ByHl = crate::RegValueT<Ccr2ByHl_SPEC>;

impl Ccr2ByHl {
    #[doc = "Bit Modulation Enable"]
    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr2_by_hl::Brme,
        ccr2_by_hl::Brme,
        Ccr2ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr2_by_hl::Brme,
            ccr2_by_hl::Brme,
            Ccr2ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ccr2_by_hl::Cks,
        ccr2_by_hl::Cks,
        Ccr2ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ccr2_by_hl::Cks,
            ccr2_by_hl::Cks,
            Ccr2ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ccr2ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ccr2ByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2ByHl {
    #[inline(always)]
    fn default() -> Ccr2ByHl {
        <crate::RegValueT<Ccr2ByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr2_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Bit rate modulation function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate modulation function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "TCLK clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "TCLK/4 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64 clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2ByHh_SPEC;
impl crate::sealed::RegSpec for Ccr2ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 2"]
pub type Ccr2ByHh = crate::RegValueT<Ccr2ByHh_SPEC>;

impl Ccr2ByHh {
    #[doc = "MDDR corrects the bit rate adjusted by the BRR setting"]
    #[inline(always)]
    pub fn mddr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ccr2ByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ccr2ByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2ByHh {
    #[inline(always)]
    fn default() -> Ccr2ByHh {
        <crate::RegValueT<Ccr2ByHh_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3_SPEC;
impl crate::sealed::RegSpec for Ccr3_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 3"]
pub type Ccr3 = crate::RegValueT<Ccr3_SPEC>;

impl Ccr3 {
    #[doc = "Clock Phase Setting"]
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr3::Cpha,
        ccr3::Cpha,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr3::Cpha,
            ccr3::Cpha,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Polarity Setting"]
    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr3::Cpol,
        ccr3::Cpol,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr3::Cpol,
            ccr3::Cpol,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Synchronizer ByPass Enable"]
    #[inline(always)]
    pub fn bpen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr3::Bpen,
        ccr3::Bpen,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr3::Bpen,
            ccr3::Bpen,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CHaRacter length select"]
    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        ccr3::Chr,
        ccr3::Chr,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            ccr3::Chr,
            ccr3::Chr,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LSB first select bit"]
    #[inline(always)]
    pub fn lsbf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr3::Lsbf,
        ccr3::Lsbf,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr3::Lsbf,
            ccr3::Lsbf,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmitted/Received Data Invert"]
    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr3::Sinv,
        ccr3::Sinv,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr3::Sinv,
            ccr3::Sinv,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Bit Length"]
    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ccr3::Stp,
        ccr3::Stp,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ccr3::Stp,
            ccr3::Stp,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ccr3::Rxdesel,
        ccr3::Rxdesel,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ccr3::Rxdesel,
            ccr3::Rxdesel,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "communication MODe select bit"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        ccr3::Mod,
        ccr3::Mod,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            ccr3::Mod,
            ccr3::Mod,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor mode"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ccr3::Mp, ccr3::Mp, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ccr3::Mp,
            ccr3::Mp,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Mode Select"]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ccr3::Fm, ccr3::Fm, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr3::Fm,
            ccr3::Fm,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Driver control Enable"]
    #[inline(always)]
    pub fn den(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ccr3::Den,
        ccr3::Den,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ccr3::Den,
            ccr3::Den,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        ccr3::Cke,
        ccr3::Cke,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            ccr3::Cke,
            ccr3::Cke,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Clock Source Select"]
    #[inline(always)]
    pub fn acs0(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ccr3::Acs0,
        ccr3::Acs0,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ccr3::Acs0,
            ccr3::Acs0,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GSM Mode"]
    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ccr3::Gm, ccr3::Gm, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ccr3::Gm,
            ccr3::Gm,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Block transfer mode"]
    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ccr3::Blk,
        ccr3::Blk,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ccr3::Blk,
            ccr3::Blk,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, Ccr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,Ccr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr3 {
    #[inline(always)]
    fn default() -> Ccr3 {
        <crate::RegValueT<Ccr3_SPEC> as RegisterValue<_>>::new(4611)
    }
}
pub mod ccr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        #[doc = "Select data sampling on leading edge, data change on trailing edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select data change on leading edge, data sampling on trailing edge."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        #[doc = "Set RSPCK low during idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set RSPCK high during idle."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpen_SPEC;
    pub type Bpen = crate::EnumBitfieldStruct<u8, Bpen_SPEC>;
    impl Bpen {
        #[doc = "Synchronizer-bypass desable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronizer-bypass enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        #[doc = "Transmit/receive in 9-bit length"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmit/receive in 9-bit length"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmit/receive in 8-bit length"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmit/receive in 7-bit length"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf_SPEC;
    pub type Lsbf = crate::EnumBitfieldStruct<u8, Lsbf_SPEC>;
    impl Lsbf {
        #[doc = "MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "LSB first."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        #[doc = "TDR contents are transmitted to TSR as they are."]
        pub const _0: Self = Self::new(0);

        #[doc = "TDR contents are inverted before being transmitted to TSR."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stp_SPEC;
    pub type Stp = crate::EnumBitfieldStruct<u8, Stp_SPEC>;
    impl Stp {
        #[doc = "1 stop bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 stop bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of two clock cycles output from the TMR"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mod_SPEC;
    pub type Mod = crate::EnumBitfieldStruct<u8, Mod_SPEC>;
    impl Mod {
        #[doc = "Asynchronous"]
        pub const _000: Self = Self::new(0);

        #[doc = "Smart card interface"]
        pub const _001: Self = Self::new(1);

        #[doc = "Clock snchronous"]
        pub const _010: Self = Self::new(2);

        #[doc = "Simple-SPI"]
        pub const _011: Self = Self::new(3);

        #[doc = "Simple-I2C"]
        pub const _100: Self = Self::new(4);

        #[doc = "Manchester"]
        pub const _101: Self = Self::new(5);

        #[doc = "Simple-LIN"]
        pub const _110: Self = Self::new(6);

        #[doc = "Prohibit"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
        #[doc = "Multi-processor communications function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-processor communications function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "Non-FIFO mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Den_SPEC;
    pub type Den = crate::EnumBitfieldStruct<u8, Den_SPEC>;
    impl Den {
        #[doc = "RS-485 driver control function is enable"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "On-chip baud rate generator"]
        pub const _00: Self = Self::new(0);

        #[doc = "On-chip baud rate generator"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output fixed high"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acs0_SPEC;
    pub type Acs0 = crate::EnumBitfieldStruct<u8, Acs0_SPEC>;
    impl Acs0 {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of two clock cycles output from the TMR"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        #[doc = "Operates in non-GSM mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "It operates in GSM mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        #[doc = "Operates in non-block transfer mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operates in block transfer mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3HaL_SPEC;
impl crate::sealed::RegSpec for Ccr3HaL_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 3"]
pub type Ccr3HaL = crate::RegValueT<Ccr3HaL_SPEC>;

impl Ccr3HaL {
    #[doc = "Clock Phase Setting"]
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr3_ha_l::Cpha,
        ccr3_ha_l::Cpha,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr3_ha_l::Cpha,
            ccr3_ha_l::Cpha,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Polarity Setting"]
    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr3_ha_l::Cpol,
        ccr3_ha_l::Cpol,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr3_ha_l::Cpol,
            ccr3_ha_l::Cpol,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Synchronizer ByPass Enable"]
    #[inline(always)]
    pub fn bpen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr3_ha_l::Bpen,
        ccr3_ha_l::Bpen,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr3_ha_l::Bpen,
            ccr3_ha_l::Bpen,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CHaRacter length select"]
    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        ccr3_ha_l::Chr,
        ccr3_ha_l::Chr,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            ccr3_ha_l::Chr,
            ccr3_ha_l::Chr,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, Ccr3HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,Ccr3HaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LSB first select bit"]
    #[inline(always)]
    pub fn lsbf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr3_ha_l::Lsbf,
        ccr3_ha_l::Lsbf,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr3_ha_l::Lsbf,
            ccr3_ha_l::Lsbf,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmitted/Received Data Invert"]
    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr3_ha_l::Sinv,
        ccr3_ha_l::Sinv,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr3_ha_l::Sinv,
            ccr3_ha_l::Sinv,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Bit Length"]
    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ccr3_ha_l::Stp,
        ccr3_ha_l::Stp,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ccr3_ha_l::Stp,
            ccr3_ha_l::Stp,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ccr3_ha_l::Rxdesel,
        ccr3_ha_l::Rxdesel,
        Ccr3HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ccr3_ha_l::Rxdesel,
            ccr3_ha_l::Rxdesel,
            Ccr3HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr3HaL {
    #[inline(always)]
    fn default() -> Ccr3HaL {
        <crate::RegValueT<Ccr3HaL_SPEC> as RegisterValue<_>>::new(4611)
    }
}
pub mod ccr3_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        #[doc = "Select data sampling on leading edge, data change on trailing edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select data change on leading edge, data sampling on trailing edge."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        #[doc = "Set RSPCK low during idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set RSPCK high during idle."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpen_SPEC;
    pub type Bpen = crate::EnumBitfieldStruct<u8, Bpen_SPEC>;
    impl Bpen {
        #[doc = "Synchronizer-bypass desable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronizer-bypass enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        #[doc = "Transmit/receive in 9-bit length"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmit/receive in 9-bit length"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmit/receive in 8-bit length"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmit/receive in 7-bit length"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf_SPEC;
    pub type Lsbf = crate::EnumBitfieldStruct<u8, Lsbf_SPEC>;
    impl Lsbf {
        #[doc = "MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "LSB first."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        #[doc = "TDR contents are transmitted to TSR as they are."]
        pub const _0: Self = Self::new(0);

        #[doc = "TDR contents are inverted before being transmitted to TSR."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stp_SPEC;
    pub type Stp = crate::EnumBitfieldStruct<u8, Stp_SPEC>;
    impl Stp {
        #[doc = "1 stop bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 stop bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of two clock cycles output from the TMR"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3ByLl_SPEC;
impl crate::sealed::RegSpec for Ccr3ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 3"]
pub type Ccr3ByLl = crate::RegValueT<Ccr3ByLl_SPEC>;

impl Ccr3ByLl {
    #[doc = "Clock Phase Setting"]
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr3_by_ll::Cpha,
        ccr3_by_ll::Cpha,
        Ccr3ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr3_by_ll::Cpha,
            ccr3_by_ll::Cpha,
            Ccr3ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Polarity Setting"]
    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr3_by_ll::Cpol,
        ccr3_by_ll::Cpol,
        Ccr3ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr3_by_ll::Cpol,
            ccr3_by_ll::Cpol,
            Ccr3ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, Ccr3ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,Ccr3ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Synchronizer ByPass Enable"]
    #[inline(always)]
    pub fn bpen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr3_by_ll::Bpen,
        ccr3_by_ll::Bpen,
        Ccr3ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr3_by_ll::Bpen,
            ccr3_by_ll::Bpen,
            Ccr3ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr3ByLl {
    #[inline(always)]
    fn default() -> Ccr3ByLl {
        <crate::RegValueT<Ccr3ByLl_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod ccr3_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        #[doc = "Select data sampling on leading edge, data change on trailing edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select data change on leading edge, data sampling on trailing edge."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        #[doc = "Set RSPCK low during idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set RSPCK high during idle."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpen_SPEC;
    pub type Bpen = crate::EnumBitfieldStruct<u8, Bpen_SPEC>;
    impl Bpen {
        #[doc = "Synchronizer-bypass desable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronizer-bypass enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3ByLh_SPEC;
impl crate::sealed::RegSpec for Ccr3ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 3"]
pub type Ccr3ByLh = crate::RegValueT<Ccr3ByLh_SPEC>;

impl Ccr3ByLh {
    #[doc = "CHaRacter length select"]
    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ccr3_by_lh::Chr,
        ccr3_by_lh::Chr,
        Ccr3ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ccr3_by_lh::Chr,
            ccr3_by_lh::Chr,
            Ccr3ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Ccr3ByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Ccr3ByLh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LSB first select bit"]
    #[inline(always)]
    pub fn lsbf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr3_by_lh::Lsbf,
        ccr3_by_lh::Lsbf,
        Ccr3ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr3_by_lh::Lsbf,
            ccr3_by_lh::Lsbf,
            Ccr3ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmitted/Received Data Invert"]
    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr3_by_lh::Sinv,
        ccr3_by_lh::Sinv,
        Ccr3ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr3_by_lh::Sinv,
            ccr3_by_lh::Sinv,
            Ccr3ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Bit Length"]
    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ccr3_by_lh::Stp,
        ccr3_by_lh::Stp,
        Ccr3ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ccr3_by_lh::Stp,
            ccr3_by_lh::Stp,
            Ccr3ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr3_by_lh::Rxdesel,
        ccr3_by_lh::Rxdesel,
        Ccr3ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr3_by_lh::Rxdesel,
            ccr3_by_lh::Rxdesel,
            Ccr3ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr3ByLh {
    #[inline(always)]
    fn default() -> Ccr3ByLh {
        <crate::RegValueT<Ccr3ByLh_SPEC> as RegisterValue<_>>::new(18)
    }
}
pub mod ccr3_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        #[doc = "Transmit/receive in 9-bit length"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmit/receive in 9-bit length"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmit/receive in 8-bit length"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmit/receive in 7-bit length"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf_SPEC;
    pub type Lsbf = crate::EnumBitfieldStruct<u8, Lsbf_SPEC>;
    impl Lsbf {
        #[doc = "MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "LSB first."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        #[doc = "TDR contents are transmitted to TSR as they are."]
        pub const _0: Self = Self::new(0);

        #[doc = "TDR contents are inverted before being transmitted to TSR."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stp_SPEC;
    pub type Stp = crate::EnumBitfieldStruct<u8, Stp_SPEC>;
    impl Stp {
        #[doc = "1 stop bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 stop bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of two clock cycles output from the TMR"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3HaH_SPEC;
impl crate::sealed::RegSpec for Ccr3HaH_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 3"]
pub type Ccr3HaH = crate::RegValueT<Ccr3HaH_SPEC>;

impl Ccr3HaH {
    #[doc = "communication MODe select bit"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ccr3_ha_h::Mod,
        ccr3_ha_h::Mod,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ccr3_ha_h::Mod,
            ccr3_ha_h::Mod,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor mode"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ccr3_ha_h::Mp,
        ccr3_ha_h::Mp,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ccr3_ha_h::Mp,
            ccr3_ha_h::Mp,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Mode Select"]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr3_ha_h::Fm,
        ccr3_ha_h::Fm,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr3_ha_h::Fm,
            ccr3_ha_h::Fm,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Driver control Enable"]
    #[inline(always)]
    pub fn den(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr3_ha_h::Den,
        ccr3_ha_h::Den,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr3_ha_h::Den,
            ccr3_ha_h::Den,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        ccr3_ha_h::Cke,
        ccr3_ha_h::Cke,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            ccr3_ha_h::Cke,
            ccr3_ha_h::Cke,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Clock Source Select"]
    #[inline(always)]
    pub fn acs0(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ccr3_ha_h::Acs0,
        ccr3_ha_h::Acs0,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ccr3_ha_h::Acs0,
            ccr3_ha_h::Acs0,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GSM Mode"]
    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr3_ha_h::Gm,
        ccr3_ha_h::Gm,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr3_ha_h::Gm,
            ccr3_ha_h::Gm,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Block transfer mode"]
    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr3_ha_h::Blk,
        ccr3_ha_h::Blk,
        Ccr3HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr3_ha_h::Blk,
            ccr3_ha_h::Blk,
            Ccr3HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Ccr3HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Ccr3HaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr3HaH {
    #[inline(always)]
    fn default() -> Ccr3HaH {
        <crate::RegValueT<Ccr3HaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr3_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mod_SPEC;
    pub type Mod = crate::EnumBitfieldStruct<u8, Mod_SPEC>;
    impl Mod {
        #[doc = "Asynchronous"]
        pub const _000: Self = Self::new(0);

        #[doc = "Smart card interface"]
        pub const _001: Self = Self::new(1);

        #[doc = "Clock snchronous"]
        pub const _010: Self = Self::new(2);

        #[doc = "Simple-SPI"]
        pub const _011: Self = Self::new(3);

        #[doc = "Simple-I2C"]
        pub const _100: Self = Self::new(4);

        #[doc = "Manchester"]
        pub const _101: Self = Self::new(5);

        #[doc = "Simple-LIN"]
        pub const _110: Self = Self::new(6);

        #[doc = "Prohibit"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
        #[doc = "Multi-processor communications function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-processor communications function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "Non-FIFO mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Den_SPEC;
    pub type Den = crate::EnumBitfieldStruct<u8, Den_SPEC>;
    impl Den {
        #[doc = "RS-485 driver control function is enable"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "On-chip baud rate generator"]
        pub const _00: Self = Self::new(0);

        #[doc = "On-chip baud rate generator"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output fixed high"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acs0_SPEC;
    pub type Acs0 = crate::EnumBitfieldStruct<u8, Acs0_SPEC>;
    impl Acs0 {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of two clock cycles output from the TMR"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        #[doc = "Operates in non-GSM mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "It operates in GSM mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        #[doc = "Operates in non-block transfer mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operates in block transfer mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3ByHl_SPEC;
impl crate::sealed::RegSpec for Ccr3ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 3"]
pub type Ccr3ByHl = crate::RegValueT<Ccr3ByHl_SPEC>;

impl Ccr3ByHl {
    #[doc = "communication MODe select bit"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ccr3_by_hl::Mod,
        ccr3_by_hl::Mod,
        Ccr3ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ccr3_by_hl::Mod,
            ccr3_by_hl::Mod,
            Ccr3ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multi-Processor mode"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ccr3_by_hl::Mp,
        ccr3_by_hl::Mp,
        Ccr3ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ccr3_by_hl::Mp,
            ccr3_by_hl::Mp,
            Ccr3ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Mode Select"]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr3_by_hl::Fm,
        ccr3_by_hl::Fm,
        Ccr3ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr3_by_hl::Fm,
            ccr3_by_hl::Fm,
            Ccr3ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Driver control Enable"]
    #[inline(always)]
    pub fn den(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr3_by_hl::Den,
        ccr3_by_hl::Den,
        Ccr3ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr3_by_hl::Den,
            ccr3_by_hl::Den,
            Ccr3ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ccr3ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ccr3ByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr3ByHl {
    #[inline(always)]
    fn default() -> Ccr3ByHl {
        <crate::RegValueT<Ccr3ByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr3_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mod_SPEC;
    pub type Mod = crate::EnumBitfieldStruct<u8, Mod_SPEC>;
    impl Mod {
        #[doc = "Asynchronous"]
        pub const _000: Self = Self::new(0);

        #[doc = "Smart card interface"]
        pub const _001: Self = Self::new(1);

        #[doc = "Clock snchronous"]
        pub const _010: Self = Self::new(2);

        #[doc = "Simple-SPI"]
        pub const _011: Self = Self::new(3);

        #[doc = "Simple-I2C"]
        pub const _100: Self = Self::new(4);

        #[doc = "Manchester"]
        pub const _101: Self = Self::new(5);

        #[doc = "Simple-LIN"]
        pub const _110: Self = Self::new(6);

        #[doc = "Prohibit"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
        #[doc = "Multi-processor communications function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multi-processor communications function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "Non-FIFO mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Den_SPEC;
    pub type Den = crate::EnumBitfieldStruct<u8, Den_SPEC>;
    impl Den {
        #[doc = "RS-485 driver control function is enable"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3ByHh_SPEC;
impl crate::sealed::RegSpec for Ccr3ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 3"]
pub type Ccr3ByHh = crate::RegValueT<Ccr3ByHh_SPEC>;

impl Ccr3ByHh {
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ccr3_by_hh::Cke,
        ccr3_by_hh::Cke,
        Ccr3ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ccr3_by_hh::Cke,
            ccr3_by_hh::Cke,
            Ccr3ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Asynchronous Mode Clock Source Select"]
    #[inline(always)]
    pub fn acs0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ccr3_by_hh::Acs0,
        ccr3_by_hh::Acs0,
        Ccr3ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ccr3_by_hh::Acs0,
            ccr3_by_hh::Acs0,
            Ccr3ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GSM Mode"]
    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr3_by_hh::Gm,
        ccr3_by_hh::Gm,
        Ccr3ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr3_by_hh::Gm,
            ccr3_by_hh::Gm,
            Ccr3ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Block transfer mode"]
    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr3_by_hh::Blk,
        ccr3_by_hh::Blk,
        Ccr3ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr3_by_hh::Blk,
            ccr3_by_hh::Blk,
            Ccr3ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Ccr3ByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Ccr3ByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr3ByHh {
    #[inline(always)]
    fn default() -> Ccr3ByHh {
        <crate::RegValueT<Ccr3ByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr3_by_hh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "On-chip baud rate generator"]
        pub const _00: Self = Self::new(0);

        #[doc = "On-chip baud rate generator"]
        pub const _01: Self = Self::new(1);

        #[doc = "Output fixed high"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acs0_SPEC;
    pub type Acs0 = crate::EnumBitfieldStruct<u8, Acs0_SPEC>;
    impl Acs0 {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Logical AND of two clock cycles output from the TMR"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        #[doc = "Operates in non-GSM mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "It operates in GSM mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        #[doc = "Operates in non-block transfer mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operates in block transfer mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4_SPEC;
impl crate::sealed::RegSpec for Ccr4_SPEC {
    type DataType = u32;
}

#[doc = "Common Control Register 4"]
pub type Ccr4 = crate::RegValueT<Ccr4_SPEC>;

impl Ccr4 {
    #[doc = "Compare Match Data"]
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Ccr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Ccr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Adjustment enable bit for  sampling timing"]
    #[inline(always)]
    pub fn asen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr4::Asen,
        ccr4::Asen,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr4::Asen,
            ccr4::Asen,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment enable bit for transmit waveform"]
    #[inline(always)]
    pub fn aten(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ccr4::Aten,
        ccr4::Aten,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ccr4::Aten,
            ccr4::Aten,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3f, 1, 0, u8, u8, Ccr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3f,1,0,u8,u8,Ccr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Adjustment Sampling Timing value"]
    #[inline(always)]
    pub fn ast(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ccr4::Ast,
        ccr4::Ast,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ccr4::Ast,
            ccr4::Ast,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Direction for sampling timing"]
    #[inline(always)]
    pub fn ajd(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ccr4::Ajd,
        ccr4::Ajd,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ccr4::Ajd,
            ccr4::Ajd,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustmen Transmission timing value"]
    #[inline(always)]
    pub fn att(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        ccr4::Att,
        ccr4::Att,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            ccr4::Att,
            ccr4::Att,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Duty control level select"]
    #[inline(always)]
    pub fn aet(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ccr4::Aet,
        ccr4::Aet,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ccr4::Aet,
            ccr4::Aet,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr4 {
    #[inline(always)]
    fn default() -> Ccr4 {
        <crate::RegValueT<Ccr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asen_SPEC;
    pub type Asen = crate::EnumBitfieldStruct<u8, Asen_SPEC>;
    impl Asen {
        #[doc = "Adjustment sampling timing function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjustment sampling timing  function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aten_SPEC;
    pub type Aten = crate::EnumBitfieldStruct<u8, Aten_SPEC>;
    impl Aten {
        #[doc = "Adjustment transmit waveform function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjustment transmit waveform  function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ast_SPEC;
    pub type Ast = crate::EnumBitfieldStruct<u8, Ast_SPEC>;
    impl Ast {
        #[doc = "No adjustment (The sampling at default timing)"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajd_SPEC;
    pub type Ajd = crate::EnumBitfieldStruct<u8, Ajd_SPEC>;
    impl Ajd {
        #[doc = "The sampling timing is adjusted after the default timing."]
        pub const _0: Self = Self::new(0);

        #[doc = "The sampling timing is adjusted before the default timing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Att_SPEC;
    pub type Att = crate::EnumBitfieldStruct<u8, Att_SPEC>;
    impl Att {
        #[doc = "No adjustment"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aet_SPEC;
    pub type Aet = crate::EnumBitfieldStruct<u8, Aet_SPEC>;
    impl Aet {
        #[doc = "Expand the low period by adjustment of the rising edge of TxD"]
        pub const _0: Self = Self::new(0);

        #[doc = "Expand the high period by adjustment of the falling edge of TxD"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4HaL_SPEC;
impl crate::sealed::RegSpec for Ccr4HaL_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 4"]
pub type Ccr4HaL = crate::RegValueT<Ccr4HaL_SPEC>;

impl Ccr4HaL {
    #[doc = "Compare Match Data"]
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Ccr4HaL_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Ccr4HaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Ccr4HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Ccr4HaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr4HaL {
    #[inline(always)]
    fn default() -> Ccr4HaL {
        <crate::RegValueT<Ccr4HaL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4ByLl_SPEC;
impl crate::sealed::RegSpec for Ccr4ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 4"]
pub type Ccr4ByLl = crate::RegValueT<Ccr4ByLl_SPEC>;

impl Ccr4ByLl {
    #[doc = "Compare Match Data"]
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ccr4ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ccr4ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr4ByLl {
    #[inline(always)]
    fn default() -> Ccr4ByLl {
        <crate::RegValueT<Ccr4ByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4ByLh_SPEC;
impl crate::sealed::RegSpec for Ccr4ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 4"]
pub type Ccr4ByLh = crate::RegValueT<Ccr4ByLh_SPEC>;

impl Ccr4ByLh {
    #[doc = "Compare Match Data"]
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ccr4ByLh_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ccr4ByLh_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ccr4ByLh_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ccr4ByLh_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ccr4ByLh {
    #[inline(always)]
    fn default() -> Ccr4ByLh {
        <crate::RegValueT<Ccr4ByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4HaH_SPEC;
impl crate::sealed::RegSpec for Ccr4HaH_SPEC {
    type DataType = u16;
}

#[doc = "Common Control Register 4"]
pub type Ccr4HaH = crate::RegValueT<Ccr4HaH_SPEC>;

impl Ccr4HaH {
    #[doc = "Adjustment enable bit for  sampling timing"]
    #[inline(always)]
    pub fn asen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr4_ha_h::Asen,
        ccr4_ha_h::Asen,
        Ccr4HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr4_ha_h::Asen,
            ccr4_ha_h::Asen,
            Ccr4HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment enable bit for transmit waveform"]
    #[inline(always)]
    pub fn aten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr4_ha_h::Aten,
        ccr4_ha_h::Aten,
        Ccr4HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr4_ha_h::Aten,
            ccr4_ha_h::Aten,
            Ccr4HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Ccr4HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Ccr4HaH_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Adjustment Sampling Timing value"]
    #[inline(always)]
    pub fn ast(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        ccr4_ha_h::Ast,
        ccr4_ha_h::Ast,
        Ccr4HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            ccr4_ha_h::Ast,
            ccr4_ha_h::Ast,
            Ccr4HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Direction for sampling timing"]
    #[inline(always)]
    pub fn ajd(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ccr4_ha_h::Ajd,
        ccr4_ha_h::Ajd,
        Ccr4HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ccr4_ha_h::Ajd,
            ccr4_ha_h::Ajd,
            Ccr4HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustmen Transmission timing value"]
    #[inline(always)]
    pub fn att(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        ccr4_ha_h::Att,
        ccr4_ha_h::Att,
        Ccr4HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            ccr4_ha_h::Att,
            ccr4_ha_h::Att,
            Ccr4HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Duty control level select"]
    #[inline(always)]
    pub fn aet(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ccr4_ha_h::Aet,
        ccr4_ha_h::Aet,
        Ccr4HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ccr4_ha_h::Aet,
            ccr4_ha_h::Aet,
            Ccr4HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr4HaH {
    #[inline(always)]
    fn default() -> Ccr4HaH {
        <crate::RegValueT<Ccr4HaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr4_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asen_SPEC;
    pub type Asen = crate::EnumBitfieldStruct<u8, Asen_SPEC>;
    impl Asen {
        #[doc = "Adjustment sampling timing function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjustment sampling timing  function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aten_SPEC;
    pub type Aten = crate::EnumBitfieldStruct<u8, Aten_SPEC>;
    impl Aten {
        #[doc = "Adjustment transmit waveform function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjustment transmit waveform  function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ast_SPEC;
    pub type Ast = crate::EnumBitfieldStruct<u8, Ast_SPEC>;
    impl Ast {
        #[doc = "No adjustment (The sampling at default timing)"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajd_SPEC;
    pub type Ajd = crate::EnumBitfieldStruct<u8, Ajd_SPEC>;
    impl Ajd {
        #[doc = "The sampling timing is adjusted after the default timing."]
        pub const _0: Self = Self::new(0);

        #[doc = "The sampling timing is adjusted before the default timing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Att_SPEC;
    pub type Att = crate::EnumBitfieldStruct<u8, Att_SPEC>;
    impl Att {
        #[doc = "No adjustment"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aet_SPEC;
    pub type Aet = crate::EnumBitfieldStruct<u8, Aet_SPEC>;
    impl Aet {
        #[doc = "Expand the low period by adjustment of the rising edge of TxD"]
        pub const _0: Self = Self::new(0);

        #[doc = "Expand the high period by adjustment of the falling edge of TxD"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4ByHl_SPEC;
impl crate::sealed::RegSpec for Ccr4ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 4"]
pub type Ccr4ByHl = crate::RegValueT<Ccr4ByHl_SPEC>;

impl Ccr4ByHl {
    #[doc = "Adjustment enable bit for  sampling timing"]
    #[inline(always)]
    pub fn asen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr4_by_hl::Asen,
        ccr4_by_hl::Asen,
        Ccr4ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr4_by_hl::Asen,
            ccr4_by_hl::Asen,
            Ccr4ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment enable bit for transmit waveform"]
    #[inline(always)]
    pub fn aten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr4_by_hl::Aten,
        ccr4_by_hl::Aten,
        Ccr4ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr4_by_hl::Aten,
            ccr4_by_hl::Aten,
            Ccr4ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Ccr4ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Ccr4ByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr4ByHl {
    #[inline(always)]
    fn default() -> Ccr4ByHl {
        <crate::RegValueT<Ccr4ByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr4_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asen_SPEC;
    pub type Asen = crate::EnumBitfieldStruct<u8, Asen_SPEC>;
    impl Asen {
        #[doc = "Adjustment sampling timing function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjustment sampling timing  function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aten_SPEC;
    pub type Aten = crate::EnumBitfieldStruct<u8, Aten_SPEC>;
    impl Aten {
        #[doc = "Adjustment transmit waveform function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Adjustment transmit waveform  function is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4ByHh_SPEC;
impl crate::sealed::RegSpec for Ccr4ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Common Control Register 4"]
pub type Ccr4ByHh = crate::RegValueT<Ccr4ByHh_SPEC>;

impl Ccr4ByHh {
    #[doc = "Adjustment Sampling Timing value"]
    #[inline(always)]
    pub fn ast(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ccr4_by_hh::Ast,
        ccr4_by_hh::Ast,
        Ccr4ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ccr4_by_hh::Ast,
            ccr4_by_hh::Ast,
            Ccr4ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Direction for sampling timing"]
    #[inline(always)]
    pub fn ajd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ccr4_by_hh::Ajd,
        ccr4_by_hh::Ajd,
        Ccr4ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ccr4_by_hh::Ajd,
            ccr4_by_hh::Ajd,
            Ccr4ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustmen Transmission timing value"]
    #[inline(always)]
    pub fn att(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        ccr4_by_hh::Att,
        ccr4_by_hh::Att,
        Ccr4ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            ccr4_by_hh::Att,
            ccr4_by_hh::Att,
            Ccr4ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Duty control level select"]
    #[inline(always)]
    pub fn aet(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr4_by_hh::Aet,
        ccr4_by_hh::Aet,
        Ccr4ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr4_by_hh::Aet,
            ccr4_by_hh::Aet,
            Ccr4ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr4ByHh {
    #[inline(always)]
    fn default() -> Ccr4ByHh {
        <crate::RegValueT<Ccr4ByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr4_by_hh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ast_SPEC;
    pub type Ast = crate::EnumBitfieldStruct<u8, Ast_SPEC>;
    impl Ast {
        #[doc = "No adjustment (The sampling at default timing)"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajd_SPEC;
    pub type Ajd = crate::EnumBitfieldStruct<u8, Ajd_SPEC>;
    impl Ajd {
        #[doc = "The sampling timing is adjusted after the default timing."]
        pub const _0: Self = Self::new(0);

        #[doc = "The sampling timing is adjusted before the default timing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Att_SPEC;
    pub type Att = crate::EnumBitfieldStruct<u8, Att_SPEC>;
    impl Att {
        #[doc = "No adjustment"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aet_SPEC;
    pub type Aet = crate::EnumBitfieldStruct<u8, Aet_SPEC>;
    impl Aet {
        #[doc = "Expand the low period by adjustment of the rising edge of TxD"]
        pub const _0: Self = Self::new(0);

        #[doc = "Expand the high period by adjustment of the falling edge of TxD"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cesr_SPEC;
impl crate::sealed::RegSpec for Cesr_SPEC {
    type DataType = u8;
}

#[doc = "Communication Enable Status Register"]
pub type Cesr = crate::RegValueT<Cesr_SPEC>;

impl Cesr {
    #[doc = "Internal status of RE signal"]
    #[inline(always)]
    pub fn rist(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cesr::Rist,
        cesr::Rist,
        Cesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cesr::Rist,
            cesr::Rist,
            Cesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Internal status of TE signal"]
    #[inline(always)]
    pub fn tist(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cesr::Tist,
        cesr::Tist,
        Cesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cesr::Tist,
            cesr::Tist,
            Cesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Cesr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Cesr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cesr {
    #[inline(always)]
    fn default() -> Cesr {
        <crate::RegValueT<Cesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rist_SPEC;
    pub type Rist = crate::EnumBitfieldStruct<u8, Rist_SPEC>;
    impl Rist {
        #[doc = "Internal status of RE is \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal status of RE is \"1\""]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tist_SPEC;
    pub type Tist = crate::EnumBitfieldStruct<u8, Tist_SPEC>;
    impl Tist {
        #[doc = "Internal status of TE is \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal status of TE is \"1\""]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcr_SPEC;
impl crate::sealed::RegSpec for Hcr_SPEC {
    type DataType = u8;
}

#[doc = "HBS valid mode Control Register"]
pub type Hcr = crate::RegValueT<Hcr_SPEC>;

impl Hcr {
    #[doc = "HDC valid mode Enable"]
    #[inline(always)]
    pub fn hden(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, hcr::Hden, hcr::Hden, Hcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hcr::Hden,
            hcr::Hden,
            Hcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HDC valid mode Output Control"]
    #[inline(always)]
    pub fn hdoc(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, hcr::Hdoc, hcr::Hdoc, Hcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            hcr::Hdoc,
            hcr::Hdoc,
            Hcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HDC valid mode STart bit"]
    #[inline(always)]
    pub fn hdst(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, hcr::Hdst, hcr::Hdst, Hcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            hcr::Hdst,
            hcr::Hdst,
            Hcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HDC valid mode start bit Initialize Control"]
    #[inline(always)]
    pub fn hdic(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, hcr::Hdic, hcr::Hdic, Hcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            hcr::Hdic,
            hcr::Hdic,
            Hcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Hcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Hcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hcr {
    #[inline(always)]
    fn default() -> Hcr {
        <crate::RegValueT<Hcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hden_SPEC;
    pub type Hden = crate::EnumBitfieldStruct<u8, Hden_SPEC>;
    impl Hden {
        #[doc = "Disabled Half Data valid mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled Half Data valid mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdoc_SPEC;
    pub type Hdoc = crate::EnumBitfieldStruct<u8, Hdoc_SPEC>;
    impl Hdoc {
        #[doc = "Select 1 terminal output mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select 2 terminal output mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdst_SPEC;
    pub type Hdst = crate::EnumBitfieldStruct<u8, Hdst_SPEC>;
    impl Hdst {
        #[doc = "Start bit is started from TXD terminal."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit is started from TXDB terminal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdic_SPEC;
    pub type Hdic = crate::EnumBitfieldStruct<u8, Hdic_SPEC>;
    impl Hdic {
        #[doc = "Output terminal is initialized for each start bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output terminal is not initialized for each start bit."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}

#[doc = "Simple-I2C Control Register"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "SSDA Delay Output Select"]
    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        icr::Iicdl,
        icr::Iicdl,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            icr::Iicdl,
            icr::Iicdl,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icr::Iicintm,
        icr::Iicintm,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icr::Iicintm,
            icr::Iicintm,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icr::Iiccsc,
        icr::Iiccsc,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icr::Iiccsc,
            icr::Iiccsc,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icr::Iicackt,
        icr::Iicackt,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icr::Iicackt,
            icr::Iicackt,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icr::Iicstareq,
        icr::Iicstareq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icr::Iicstareq,
            icr::Iicstareq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icr::Iicrstareq,
        icr::Iicrstareq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icr::Iicrstareq,
            icr::Iicrstareq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icr::Iicstpreq,
        icr::Iicstpreq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icr::Iicstpreq,
            icr::Iicstpreq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        icr::Iicsdas,
        icr::Iicsdas,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            icr::Iicsdas,
            icr::Iicsdas,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        icr::Iicscls,
        icr::Iicscls,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            icr::Iicscls,
            icr::Iicscls,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicdl_SPEC;
    pub type Iicdl = crate::EnumBitfieldStruct<u8, Iicdl_SPEC>;
    impl Iicdl {
        #[doc = "No output delay"]
        pub const _00000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        #[doc = "Use ACK/NACK interrupts."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use reception and transmission interrupts"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        #[doc = "No synchronization with the clock signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronization with the clock signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        #[doc = "ACK transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK transmission and reception of ACK/NACK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        #[doc = "A start condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A start condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        #[doc = "A restart condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A restart condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        #[doc = "A stop condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A stop condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        #[doc = "Serial data output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSDAn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSDAn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        #[doc = "Serial clock output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSCLn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSCLn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcrHaL_SPEC;
impl crate::sealed::RegSpec for IcrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Simple-I2C Control Register"]
pub type IcrHaL = crate::RegValueT<IcrHaL_SPEC>;

impl IcrHaL {
    #[doc = "SSDA Delay Output Select"]
    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        icr_ha_l::Iicdl,
        icr_ha_l::Iicdl,
        IcrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            icr_ha_l::Iicdl,
            icr_ha_l::Iicdl,
            IcrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icr_ha_l::Iicintm,
        icr_ha_l::Iicintm,
        IcrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icr_ha_l::Iicintm,
            icr_ha_l::Iicintm,
            IcrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icr_ha_l::Iiccsc,
        icr_ha_l::Iiccsc,
        IcrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icr_ha_l::Iiccsc,
            icr_ha_l::Iiccsc,
            IcrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icr_ha_l::Iicackt,
        icr_ha_l::Iicackt,
        IcrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icr_ha_l::Iicackt,
            icr_ha_l::Iicackt,
            IcrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, IcrHaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,IcrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for IcrHaL {
    #[inline(always)]
    fn default() -> IcrHaL {
        <crate::RegValueT<IcrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicdl_SPEC;
    pub type Iicdl = crate::EnumBitfieldStruct<u8, Iicdl_SPEC>;
    impl Iicdl {
        #[doc = "No output delay"]
        pub const _00000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        #[doc = "Use ACK/NACK interrupts."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use reception and transmission interrupts"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        #[doc = "No synchronization with the clock signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronization with the clock signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        #[doc = "ACK transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK transmission and reception of ACK/NACK"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcrByLl_SPEC;
impl crate::sealed::RegSpec for IcrByLl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-I2C Control Register"]
pub type IcrByLl = crate::RegValueT<IcrByLl_SPEC>;

impl IcrByLl {
    #[doc = "SSDA Delay Output Select"]
    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        icr_by_ll::Iicdl,
        icr_by_ll::Iicdl,
        IcrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            icr_by_ll::Iicdl,
            icr_by_ll::Iicdl,
            IcrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, IcrByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,IcrByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for IcrByLl {
    #[inline(always)]
    fn default() -> IcrByLl {
        <crate::RegValueT<IcrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicdl_SPEC;
    pub type Iicdl = crate::EnumBitfieldStruct<u8, Iicdl_SPEC>;
    impl Iicdl {
        #[doc = "No output delay"]
        pub const _00000: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcrByLh_SPEC;
impl crate::sealed::RegSpec for IcrByLh_SPEC {
    type DataType = u8;
}

#[doc = "Simple-I2C Control Register"]
pub type IcrByLh = crate::RegValueT<IcrByLh_SPEC>;

impl IcrByLh {
    #[doc = "I2C Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icr_by_lh::Iicintm,
        icr_by_lh::Iicintm,
        IcrByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icr_by_lh::Iicintm,
            icr_by_lh::Iicintm,
            IcrByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icr_by_lh::Iiccsc,
        icr_by_lh::Iiccsc,
        IcrByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icr_by_lh::Iiccsc,
            icr_by_lh::Iiccsc,
            IcrByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icr_by_lh::Iicackt,
        icr_by_lh::Iicackt,
        IcrByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icr_by_lh::Iicackt,
            icr_by_lh::Iicackt,
            IcrByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, IcrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,IcrByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for IcrByLh {
    #[inline(always)]
    fn default() -> IcrByLh {
        <crate::RegValueT<IcrByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        #[doc = "Use ACK/NACK interrupts."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use reception and transmission interrupts"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        #[doc = "No synchronization with the clock signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronization with the clock signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        #[doc = "ACK transmission"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK transmission and reception of ACK/NACK"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcrHaH_SPEC;
impl crate::sealed::RegSpec for IcrHaH_SPEC {
    type DataType = u16;
}

#[doc = "Simple-I2C Control Register"]
pub type IcrHaH = crate::RegValueT<IcrHaH_SPEC>;

impl IcrHaH {
    #[doc = "Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icr_ha_h::Iicstareq,
        icr_ha_h::Iicstareq,
        IcrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icr_ha_h::Iicstareq,
            icr_ha_h::Iicstareq,
            IcrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icr_ha_h::Iicrstareq,
        icr_ha_h::Iicrstareq,
        IcrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icr_ha_h::Iicrstareq,
            icr_ha_h::Iicrstareq,
            IcrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icr_ha_h::Iicstpreq,
        icr_ha_h::Iicstpreq,
        IcrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icr_ha_h::Iicstpreq,
            icr_ha_h::Iicstpreq,
            IcrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        icr_ha_h::Iicsdas,
        icr_ha_h::Iicsdas,
        IcrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            icr_ha_h::Iicsdas,
            icr_ha_h::Iicsdas,
            IcrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        icr_ha_h::Iicscls,
        icr_ha_h::Iicscls,
        IcrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            icr_ha_h::Iicscls,
            icr_ha_h::Iicscls,
            IcrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, IcrHaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,IcrHaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for IcrHaH {
    #[inline(always)]
    fn default() -> IcrHaH {
        <crate::RegValueT<IcrHaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        #[doc = "A start condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A start condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        #[doc = "A restart condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A restart condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        #[doc = "A stop condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A stop condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        #[doc = "Serial data output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSDAn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSDAn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        #[doc = "Serial clock output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSCLn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSCLn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcrByHl_SPEC;
impl crate::sealed::RegSpec for IcrByHl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-I2C Control Register"]
pub type IcrByHl = crate::RegValueT<IcrByHl_SPEC>;

impl IcrByHl {
    #[doc = "Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icr_by_hl::Iicstareq,
        icr_by_hl::Iicstareq,
        IcrByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icr_by_hl::Iicstareq,
            icr_by_hl::Iicstareq,
            IcrByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icr_by_hl::Iicrstareq,
        icr_by_hl::Iicrstareq,
        IcrByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icr_by_hl::Iicrstareq,
            icr_by_hl::Iicrstareq,
            IcrByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icr_by_hl::Iicstpreq,
        icr_by_hl::Iicstpreq,
        IcrByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icr_by_hl::Iicstpreq,
            icr_by_hl::Iicstpreq,
            IcrByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IcrByHl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IcrByHl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        icr_by_hl::Iicsdas,
        icr_by_hl::Iicsdas,
        IcrByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            icr_by_hl::Iicsdas,
            icr_by_hl::Iicsdas,
            IcrByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        icr_by_hl::Iicscls,
        icr_by_hl::Iicscls,
        IcrByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            icr_by_hl::Iicscls,
            icr_by_hl::Iicscls,
            IcrByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for IcrByHl {
    #[inline(always)]
    fn default() -> IcrByHl {
        <crate::RegValueT<IcrByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        #[doc = "A start condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A start condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        #[doc = "A restart condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A restart condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        #[doc = "A stop condition is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A stop condition is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        #[doc = "Serial data output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSDAn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSDAn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        #[doc = "Serial clock output"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate a start, restart, or stop condition."]
        pub const _01: Self = Self::new(1);

        #[doc = "Output the low level on the SSCLn pin."]
        pub const _10: Self = Self::new(2);

        #[doc = "Place the SSCLn pin in the high-impedance state."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr_SPEC;
impl crate::sealed::RegSpec for Fcr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Control Register"]
pub type Fcr = crate::RegValueT<Fcr_SPEC>;

impl Fcr {
    #[doc = "Receive data ready error select bit"]
    #[inline(always)]
    pub fn dres(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fcr::Dres, fcr::Dres, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcr::Dres,
            fcr::Dres,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO data trigger number"]
    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fcr::Tfrst,
        fcr::Tfrst,
        Fcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fcr::Tfrst,
            fcr::Tfrst,
            Fcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO data trigger number"]
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fcr::Rfrst,
        fcr::Rfrst,
        Fcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fcr::Rfrst,
            fcr::Rfrst,
            Fcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "RTS# Output ActiveTrigger Number Select"]
    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        <crate::RegValueT<Fcr_SPEC> as RegisterValue<_>>::new(522125312)
    }
}
pub mod fcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        #[doc = "reception data full interrupt (RXI)"]
        pub const _0: Self = Self::new(0);

        #[doc = "receive error interrupt (ERI)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        #[doc = "The number of data stored in TDR register are NOT made \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in TDR register are made \"0\""]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        #[doc = "The number of data stored in RDR register are NOT made \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in RDR register are made \"0\""]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcrHaL_SPEC;
impl crate::sealed::RegSpec for FcrHaL_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Control Register"]
pub type FcrHaL = crate::RegValueT<FcrHaL_SPEC>;

impl FcrHaL {
    #[doc = "Receive data ready error select bit"]
    #[inline(always)]
    pub fn dres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fcr_ha_l::Dres,
        fcr_ha_l::Dres,
        FcrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcr_ha_l::Dres,
            fcr_ha_l::Dres,
            FcrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO data trigger number"]
    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, FcrHaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,FcrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, FcrHaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,FcrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fcr_ha_l::Tfrst,
        fcr_ha_l::Tfrst,
        FcrHaL_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fcr_ha_l::Tfrst,
            fcr_ha_l::Tfrst,
            FcrHaL_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FcrHaL {
    #[inline(always)]
    fn default() -> FcrHaL {
        <crate::RegValueT<FcrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcr_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        #[doc = "reception data full interrupt (RXI)"]
        pub const _0: Self = Self::new(0);

        #[doc = "receive error interrupt (ERI)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        #[doc = "The number of data stored in TDR register are NOT made \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in TDR register are made \"0\""]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcrByLl_SPEC;
impl crate::sealed::RegSpec for FcrByLl_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Control Register"]
pub type FcrByLl = crate::RegValueT<FcrByLl_SPEC>;

impl FcrByLl {
    #[doc = "Receive data ready error select bit"]
    #[inline(always)]
    pub fn dres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fcr_by_ll::Dres,
        fcr_by_ll::Dres,
        FcrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcr_by_ll::Dres,
            fcr_by_ll::Dres,
            FcrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, FcrByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,FcrByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FcrByLl {
    #[inline(always)]
    fn default() -> FcrByLl {
        <crate::RegValueT<FcrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcr_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        #[doc = "reception data full interrupt (RXI)"]
        pub const _0: Self = Self::new(0);

        #[doc = "receive error interrupt (ERI)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcrByLh_SPEC;
impl crate::sealed::RegSpec for FcrByLh_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Control Register"]
pub type FcrByLh = crate::RegValueT<FcrByLh_SPEC>;

impl FcrByLh {
    #[doc = "Transmit FIFO data trigger number"]
    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, FcrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,FcrByLh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, FcrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,FcrByLh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fcr_by_lh::Tfrst,
        fcr_by_lh::Tfrst,
        FcrByLh_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fcr_by_lh::Tfrst,
            fcr_by_lh::Tfrst,
            FcrByLh_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FcrByLh {
    #[inline(always)]
    fn default() -> FcrByLh {
        <crate::RegValueT<FcrByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcr_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        #[doc = "The number of data stored in TDR register are NOT made \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in TDR register are made \"0\""]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcrHaH_SPEC;
impl crate::sealed::RegSpec for FcrHaH_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Control Register"]
pub type FcrHaH = crate::RegValueT<FcrHaH_SPEC>;

impl FcrHaH {
    #[doc = "Receive FIFO data trigger number"]
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, FcrHaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,FcrHaH_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fcr_ha_h::Rfrst,
        fcr_ha_h::Rfrst,
        FcrHaH_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fcr_ha_h::Rfrst,
            fcr_ha_h::Rfrst,
            FcrHaH_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "RTS# Output ActiveTrigger Number Select"]
    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, FcrHaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,FcrHaH_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, FcrHaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,FcrHaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FcrHaH {
    #[inline(always)]
    fn default() -> FcrHaH {
        <crate::RegValueT<FcrHaH_SPEC> as RegisterValue<_>>::new(7967)
    }
}
pub mod fcr_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        #[doc = "The number of data stored in RDR register are NOT made \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in RDR register are made \"0\""]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcrByHl_SPEC;
impl crate::sealed::RegSpec for FcrByHl_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Control Register"]
pub type FcrByHl = crate::RegValueT<FcrByHl_SPEC>;

impl FcrByHl {
    #[doc = "Receive FIFO data trigger number"]
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, FcrByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,FcrByHl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, FcrByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,FcrByHl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fcr_by_hl::Rfrst,
        fcr_by_hl::Rfrst,
        FcrByHl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fcr_by_hl::Rfrst,
            fcr_by_hl::Rfrst,
            FcrByHl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FcrByHl {
    #[inline(always)]
    fn default() -> FcrByHl {
        <crate::RegValueT<FcrByHl_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod fcr_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        #[doc = "The number of data stored in RDR register are NOT made \"0\""]
        pub const _0: Self = Self::new(0);

        #[doc = "The number of data stored in RDR register are made \"0\""]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FcrByHh_SPEC;
impl crate::sealed::RegSpec for FcrByHh_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Control Register"]
pub type FcrByHh = crate::RegValueT<FcrByHh_SPEC>;

impl FcrByHh {
    #[doc = "RTS# Output ActiveTrigger Number Select"]
    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, FcrByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,FcrByHh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, FcrByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,FcrByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FcrByHh {
    #[inline(always)]
    fn default() -> FcrByHh {
        <crate::RegValueT<FcrByHh_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr_SPEC;
impl crate::sealed::RegSpec for Mcr_SPEC {
    type DataType = u32;
}

#[doc = "Manchester Control Register"]
pub type Mcr = crate::RegValueT<Mcr_SPEC>;

impl Mcr {
    #[doc = "Receive Manchester polarity"]
    #[inline(always)]
    pub fn rmpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mcr::Rmpol,
        mcr::Rmpol,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mcr::Rmpol,
            mcr::Rmpol,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Manchester polarity"]
    #[inline(always)]
    pub fn tmpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mcr::Tmpol,
        mcr::Tmpol,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mcr::Tmpol,
            mcr::Tmpol,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Manchester edge retiming enable"]
    #[inline(always)]
    pub fn erten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mcr::Erten,
        mcr::Erten,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mcr::Erten,
            mcr::Erten,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync type of Manchester code start bit is set."]
    #[inline(always)]
    pub fn synval(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mcr::Synval,
        mcr::Synval,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mcr::Synval,
            mcr::Synval,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync select"]
    #[inline(always)]
    pub fn synsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mcr::Synsel,
        mcr::Synsel,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mcr::Synsel,
            mcr::Synsel,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start bit select"]
    #[inline(always)]
    pub fn sbsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mcr::Sbsel,
        mcr::Sbsel,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mcr::Sbsel,
            mcr::Sbsel,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit preface length."]
    #[inline(always)]
    pub fn tplen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        mcr::Tplen,
        mcr::Tplen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            mcr::Tplen,
            mcr::Tplen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit preface pattern."]
    #[inline(always)]
    pub fn tppat(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        mcr::Tppat,
        mcr::Tppat,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            mcr::Tppat,
            mcr::Tppat,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive preface length"]
    #[inline(always)]
    pub fn rplen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        mcr::Rplen,
        mcr::Rplen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            mcr::Rplen,
            mcr::Rplen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive preface pattern"]
    #[inline(always)]
    pub fn rppat(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        mcr::Rppat,
        mcr::Rppat,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            mcr::Rppat,
            mcr::Rppat,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preface Error enable"]
    #[inline(always)]
    pub fn pferen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mcr::Pferen,
        mcr::Pferen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mcr::Pferen,
            mcr::Pferen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Error enable"]
    #[inline(always)]
    pub fn syeren(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mcr::Syeren,
        mcr::Syeren,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mcr::Syeren,
            mcr::Syeren,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start bit error enable"]
    #[inline(always)]
    pub fn sberen(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mcr::Sberen,
        mcr::Sberen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mcr::Sberen,
            mcr::Sberen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        <crate::RegValueT<Mcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmpol_SPEC;
    pub type Rmpol = crate::EnumBitfieldStruct<u8, Rmpol_SPEC>;
    impl Rmpol {
        #[doc = "Logic code 0 is changed from 0 to 1 of Manchester code. Logic code 1 is changed from 1 to 0 of Manchester code."]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic code 0 is changed from 1 to 0 of Manchester code. Logic code 1 is changed from 0 to 1 of Manchester code."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmpol_SPEC;
    pub type Tmpol = crate::EnumBitfieldStruct<u8, Tmpol_SPEC>;
    impl Tmpol {
        #[doc = "Logic code 0 is changed from 0 to 1 of Manchester code.Logic code 1 is changed from 1 to 0 of Manchester code."]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic code 0 is changed from 1 to 0 of Manchester code.Logic code 1 is changed from 0 to 1 of Manchester code."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erten_SPEC;
    pub type Erten = crate::EnumBitfieldStruct<u8, Erten_SPEC>;
    impl Erten {
        #[doc = "Disables the receive retiming function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the receive retiming function."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synval_SPEC;
    pub type Synval = crate::EnumBitfieldStruct<u8, Synval_SPEC>;
    impl Synval {
        #[doc = "Start bit is added as &lt;0 → 1 transition&gt; with 1 bit.(SBSEL=0 Transmission )/ Receive only when the start bit is 1 bit <0 → 1 transition> pattern,Otherwise it is judged as an error.(SBSEL=0 Receive)/Start bit is added as <0 → 1 transition> with 3 bits(SBSEL=1 Transmission)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit is added as &lt;1 → 0 transition&gt; with 1 bit.(SBSEL=0 Transmission )/ Receive only when the start bit is 1 bit <1 → 0 transition> pattern.Otherwise it is judged as an error.(SEBSEL=0,Receive)/ Start bit is added as <1 → 0 transition> with 3 bits(SBSEL=1 Transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synsel_SPEC;
    pub type Synsel = crate::EnumBitfieldStruct<u8, Synsel_SPEC>;
    impl Synsel {
        #[doc = "Sync type is set with the SYNVAL bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync type is set by TSYNC bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbsel_SPEC;
    pub type Sbsel = crate::EnumBitfieldStruct<u8, Sbsel_SPEC>;
    impl Sbsel {
        #[doc = "Start bit is 1 bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit is Command Sync / Data Sync (3 bits)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tplen_SPEC;
    pub type Tplen = crate::EnumBitfieldStruct<u8, Tplen_SPEC>;
    impl Tplen {
        #[doc = "Transmit preface generation disabled."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tppat_SPEC;
    pub type Tppat = crate::EnumBitfieldStruct<u8, Tppat_SPEC>;
    impl Tppat {
        #[doc = "ALL ZERO"]
        pub const _0: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rplen_SPEC;
    pub type Rplen = crate::EnumBitfieldStruct<u8, Rplen_SPEC>;
    impl Rplen {
        #[doc = "Receive preface generation disabled."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rppat_SPEC;
    pub type Rppat = crate::EnumBitfieldStruct<u8, Rppat_SPEC>;
    impl Rppat {
        #[doc = "ALL ZERO"]
        pub const _00: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _01: Self = Self::new(1);

        #[doc = "ONE ZERO"]
        pub const _10: Self = Self::new(2);

        #[doc = "ALL ONE"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pferen_SPEC;
    pub type Pferen = crate::EnumBitfieldStruct<u8, Pferen_SPEC>;
    impl Pferen {
        #[doc = "Not handle preface error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle preface error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syeren_SPEC;
    pub type Syeren = crate::EnumBitfieldStruct<u8, Syeren_SPEC>;
    impl Syeren {
        #[doc = "Not handle recive sync error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle receive sync error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sberen_SPEC;
    pub type Sberen = crate::EnumBitfieldStruct<u8, Sberen_SPEC>;
    impl Sberen {
        #[doc = "Not handle start bit error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle start bit error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McrHaL_SPEC;
impl crate::sealed::RegSpec for McrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Manchester Control Register"]
pub type McrHaL = crate::RegValueT<McrHaL_SPEC>;

impl McrHaL {
    #[doc = "Receive Manchester polarity"]
    #[inline(always)]
    pub fn rmpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mcr_ha_l::Rmpol,
        mcr_ha_l::Rmpol,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mcr_ha_l::Rmpol,
            mcr_ha_l::Rmpol,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Manchester polarity"]
    #[inline(always)]
    pub fn tmpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mcr_ha_l::Tmpol,
        mcr_ha_l::Tmpol,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mcr_ha_l::Tmpol,
            mcr_ha_l::Tmpol,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Manchester edge retiming enable"]
    #[inline(always)]
    pub fn erten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mcr_ha_l::Erten,
        mcr_ha_l::Erten,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mcr_ha_l::Erten,
            mcr_ha_l::Erten,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync type of Manchester code start bit is set."]
    #[inline(always)]
    pub fn synval(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mcr_ha_l::Synval,
        mcr_ha_l::Synval,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mcr_ha_l::Synval,
            mcr_ha_l::Synval,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync select"]
    #[inline(always)]
    pub fn synsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mcr_ha_l::Synsel,
        mcr_ha_l::Synsel,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mcr_ha_l::Synsel,
            mcr_ha_l::Synsel,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start bit select"]
    #[inline(always)]
    pub fn sbsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mcr_ha_l::Sbsel,
        mcr_ha_l::Sbsel,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mcr_ha_l::Sbsel,
            mcr_ha_l::Sbsel,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit preface length."]
    #[inline(always)]
    pub fn tplen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        mcr_ha_l::Tplen,
        mcr_ha_l::Tplen,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            mcr_ha_l::Tplen,
            mcr_ha_l::Tplen,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit preface pattern."]
    #[inline(always)]
    pub fn tppat(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        mcr_ha_l::Tppat,
        mcr_ha_l::Tppat,
        McrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            mcr_ha_l::Tppat,
            mcr_ha_l::Tppat,
            McrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, McrHaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,McrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for McrHaL {
    #[inline(always)]
    fn default() -> McrHaL {
        <crate::RegValueT<McrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmpol_SPEC;
    pub type Rmpol = crate::EnumBitfieldStruct<u8, Rmpol_SPEC>;
    impl Rmpol {
        #[doc = "Logic code 0 is changed from 0 to 1 of Manchester code. Logic code 1 is changed from 1 to 0 of Manchester code."]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic code 0 is changed from 1 to 0 of Manchester code. Logic code 1 is changed from 0 to 1 of Manchester code."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmpol_SPEC;
    pub type Tmpol = crate::EnumBitfieldStruct<u8, Tmpol_SPEC>;
    impl Tmpol {
        #[doc = "Logic code 0 is changed from 0 to 1 of Manchester code.Logic code 1 is changed from 1 to 0 of Manchester code."]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic code 0 is changed from 1 to 0 of Manchester code.Logic code 1 is changed from 0 to 1 of Manchester code."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erten_SPEC;
    pub type Erten = crate::EnumBitfieldStruct<u8, Erten_SPEC>;
    impl Erten {
        #[doc = "Disables the receive retiming function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the receive retiming function."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synval_SPEC;
    pub type Synval = crate::EnumBitfieldStruct<u8, Synval_SPEC>;
    impl Synval {
        #[doc = "Start bit is added as &lt;0 → 1 transition&gt; with 1 bit.(SBSEL=0 Transmission )/ Receive only when the start bit is 1 bit <0 → 1 transition> pattern,Otherwise it is judged as an error.(SBSEL=0 Receive)/Start bit is added as <0 → 1 transition> with 3 bits(SBSEL=1 Transmission)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit is added as &lt;1 → 0 transition&gt; with 1 bit.(SBSEL=0 Transmission )/ Receive only when the start bit is 1 bit <1 → 0 transition> pattern.Otherwise it is judged as an error.(SEBSEL=0,Receive)/ Start bit is added as <1 → 0 transition> with 3 bits(SBSEL=1 Transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synsel_SPEC;
    pub type Synsel = crate::EnumBitfieldStruct<u8, Synsel_SPEC>;
    impl Synsel {
        #[doc = "Sync type is set with the SYNVAL bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync type is set by TSYNC bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbsel_SPEC;
    pub type Sbsel = crate::EnumBitfieldStruct<u8, Sbsel_SPEC>;
    impl Sbsel {
        #[doc = "Start bit is 1 bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit is Command Sync / Data Sync (3 bits)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tplen_SPEC;
    pub type Tplen = crate::EnumBitfieldStruct<u8, Tplen_SPEC>;
    impl Tplen {
        #[doc = "Transmit preface generation disabled."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tppat_SPEC;
    pub type Tppat = crate::EnumBitfieldStruct<u8, Tppat_SPEC>;
    impl Tppat {
        #[doc = "ALL ZERO"]
        pub const _0: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McrByLl_SPEC;
impl crate::sealed::RegSpec for McrByLl_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Control Register"]
pub type McrByLl = crate::RegValueT<McrByLl_SPEC>;

impl McrByLl {
    #[doc = "Receive Manchester polarity"]
    #[inline(always)]
    pub fn rmpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mcr_by_ll::Rmpol,
        mcr_by_ll::Rmpol,
        McrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mcr_by_ll::Rmpol,
            mcr_by_ll::Rmpol,
            McrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Manchester polarity"]
    #[inline(always)]
    pub fn tmpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mcr_by_ll::Tmpol,
        mcr_by_ll::Tmpol,
        McrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mcr_by_ll::Tmpol,
            mcr_by_ll::Tmpol,
            McrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Manchester edge retiming enable"]
    #[inline(always)]
    pub fn erten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mcr_by_ll::Erten,
        mcr_by_ll::Erten,
        McrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mcr_by_ll::Erten,
            mcr_by_ll::Erten,
            McrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync type of Manchester code start bit is set."]
    #[inline(always)]
    pub fn synval(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mcr_by_ll::Synval,
        mcr_by_ll::Synval,
        McrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mcr_by_ll::Synval,
            mcr_by_ll::Synval,
            McrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync select"]
    #[inline(always)]
    pub fn synsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mcr_by_ll::Synsel,
        mcr_by_ll::Synsel,
        McrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mcr_by_ll::Synsel,
            mcr_by_ll::Synsel,
            McrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start bit select"]
    #[inline(always)]
    pub fn sbsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mcr_by_ll::Sbsel,
        mcr_by_ll::Sbsel,
        McrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mcr_by_ll::Sbsel,
            mcr_by_ll::Sbsel,
            McrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, McrByLl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, McrByLl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for McrByLl {
    #[inline(always)]
    fn default() -> McrByLl {
        <crate::RegValueT<McrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmpol_SPEC;
    pub type Rmpol = crate::EnumBitfieldStruct<u8, Rmpol_SPEC>;
    impl Rmpol {
        #[doc = "Logic code 0 is changed from 0 to 1 of Manchester code. Logic code 1 is changed from 1 to 0 of Manchester code."]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic code 0 is changed from 1 to 0 of Manchester code. Logic code 1 is changed from 0 to 1 of Manchester code."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmpol_SPEC;
    pub type Tmpol = crate::EnumBitfieldStruct<u8, Tmpol_SPEC>;
    impl Tmpol {
        #[doc = "Logic code 0 is changed from 0 to 1 of Manchester code.Logic code 1 is changed from 1 to 0 of Manchester code."]
        pub const _0: Self = Self::new(0);

        #[doc = "Logic code 0 is changed from 1 to 0 of Manchester code.Logic code 1 is changed from 0 to 1 of Manchester code."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erten_SPEC;
    pub type Erten = crate::EnumBitfieldStruct<u8, Erten_SPEC>;
    impl Erten {
        #[doc = "Disables the receive retiming function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the receive retiming function."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synval_SPEC;
    pub type Synval = crate::EnumBitfieldStruct<u8, Synval_SPEC>;
    impl Synval {
        #[doc = "Start bit is added as &lt;0 → 1 transition&gt; with 1 bit.(SBSEL=0 Transmission )/ Receive only when the start bit is 1 bit <0 → 1 transition> pattern,Otherwise it is judged as an error.(SBSEL=0 Receive)/Start bit is added as <0 → 1 transition> with 3 bits(SBSEL=1 Transmission)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit is added as &lt;1 → 0 transition&gt; with 1 bit.(SBSEL=0 Transmission )/ Receive only when the start bit is 1 bit <1 → 0 transition> pattern.Otherwise it is judged as an error.(SEBSEL=0,Receive)/ Start bit is added as <1 → 0 transition> with 3 bits(SBSEL=1 Transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synsel_SPEC;
    pub type Synsel = crate::EnumBitfieldStruct<u8, Synsel_SPEC>;
    impl Synsel {
        #[doc = "Sync type is set with the SYNVAL bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync type is set by TSYNC bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbsel_SPEC;
    pub type Sbsel = crate::EnumBitfieldStruct<u8, Sbsel_SPEC>;
    impl Sbsel {
        #[doc = "Start bit is 1 bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit is Command Sync / Data Sync (3 bits)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McrByLh_SPEC;
impl crate::sealed::RegSpec for McrByLh_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Control Register"]
pub type McrByLh = crate::RegValueT<McrByLh_SPEC>;

impl McrByLh {
    #[doc = "Transmit preface length."]
    #[inline(always)]
    pub fn tplen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        mcr_by_lh::Tplen,
        mcr_by_lh::Tplen,
        McrByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            mcr_by_lh::Tplen,
            mcr_by_lh::Tplen,
            McrByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit preface pattern."]
    #[inline(always)]
    pub fn tppat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        mcr_by_lh::Tppat,
        mcr_by_lh::Tppat,
        McrByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            mcr_by_lh::Tppat,
            mcr_by_lh::Tppat,
            McrByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, McrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,McrByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for McrByLh {
    #[inline(always)]
    fn default() -> McrByLh {
        <crate::RegValueT<McrByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tplen_SPEC;
    pub type Tplen = crate::EnumBitfieldStruct<u8, Tplen_SPEC>;
    impl Tplen {
        #[doc = "Transmit preface generation disabled."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tppat_SPEC;
    pub type Tppat = crate::EnumBitfieldStruct<u8, Tppat_SPEC>;
    impl Tppat {
        #[doc = "ALL ZERO"]
        pub const _0: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McrHaH_SPEC;
impl crate::sealed::RegSpec for McrHaH_SPEC {
    type DataType = u16;
}

#[doc = "Manchester Control Register"]
pub type McrHaH = crate::RegValueT<McrHaH_SPEC>;

impl McrHaH {
    #[doc = "Receive preface length"]
    #[inline(always)]
    pub fn rplen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        mcr_ha_h::Rplen,
        mcr_ha_h::Rplen,
        McrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            mcr_ha_h::Rplen,
            mcr_ha_h::Rplen,
            McrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive preface pattern"]
    #[inline(always)]
    pub fn rppat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        mcr_ha_h::Rppat,
        mcr_ha_h::Rppat,
        McrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            mcr_ha_h::Rppat,
            mcr_ha_h::Rppat,
            McrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preface Error enable"]
    #[inline(always)]
    pub fn pferen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mcr_ha_h::Pferen,
        mcr_ha_h::Pferen,
        McrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mcr_ha_h::Pferen,
            mcr_ha_h::Pferen,
            McrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Error enable"]
    #[inline(always)]
    pub fn syeren(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mcr_ha_h::Syeren,
        mcr_ha_h::Syeren,
        McrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mcr_ha_h::Syeren,
            mcr_ha_h::Syeren,
            McrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start bit error enable"]
    #[inline(always)]
    pub fn sberen(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mcr_ha_h::Sberen,
        mcr_ha_h::Sberen,
        McrHaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mcr_ha_h::Sberen,
            mcr_ha_h::Sberen,
            McrHaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, McrHaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,McrHaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for McrHaH {
    #[inline(always)]
    fn default() -> McrHaH {
        <crate::RegValueT<McrHaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rplen_SPEC;
    pub type Rplen = crate::EnumBitfieldStruct<u8, Rplen_SPEC>;
    impl Rplen {
        #[doc = "Receive preface generation disabled."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rppat_SPEC;
    pub type Rppat = crate::EnumBitfieldStruct<u8, Rppat_SPEC>;
    impl Rppat {
        #[doc = "ALL ZERO"]
        pub const _00: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _01: Self = Self::new(1);

        #[doc = "ONE ZERO"]
        pub const _10: Self = Self::new(2);

        #[doc = "ALL ONE"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pferen_SPEC;
    pub type Pferen = crate::EnumBitfieldStruct<u8, Pferen_SPEC>;
    impl Pferen {
        #[doc = "Not handle preface error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle preface error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syeren_SPEC;
    pub type Syeren = crate::EnumBitfieldStruct<u8, Syeren_SPEC>;
    impl Syeren {
        #[doc = "Not handle recive sync error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle receive sync error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sberen_SPEC;
    pub type Sberen = crate::EnumBitfieldStruct<u8, Sberen_SPEC>;
    impl Sberen {
        #[doc = "Not handle start bit error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle start bit error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McrByHl_SPEC;
impl crate::sealed::RegSpec for McrByHl_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Control Register"]
pub type McrByHl = crate::RegValueT<McrByHl_SPEC>;

impl McrByHl {
    #[doc = "Receive preface length"]
    #[inline(always)]
    pub fn rplen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        mcr_by_hl::Rplen,
        mcr_by_hl::Rplen,
        McrByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            mcr_by_hl::Rplen,
            mcr_by_hl::Rplen,
            McrByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive preface pattern"]
    #[inline(always)]
    pub fn rppat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        mcr_by_hl::Rppat,
        mcr_by_hl::Rppat,
        McrByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            mcr_by_hl::Rppat,
            mcr_by_hl::Rppat,
            McrByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, McrByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,McrByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for McrByHl {
    #[inline(always)]
    fn default() -> McrByHl {
        <crate::RegValueT<McrByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rplen_SPEC;
    pub type Rplen = crate::EnumBitfieldStruct<u8, Rplen_SPEC>;
    impl Rplen {
        #[doc = "Receive preface generation disabled."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rppat_SPEC;
    pub type Rppat = crate::EnumBitfieldStruct<u8, Rppat_SPEC>;
    impl Rppat {
        #[doc = "ALL ZERO"]
        pub const _00: Self = Self::new(0);

        #[doc = "ZERO ONE"]
        pub const _01: Self = Self::new(1);

        #[doc = "ONE ZERO"]
        pub const _10: Self = Self::new(2);

        #[doc = "ALL ONE"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McrByHh_SPEC;
impl crate::sealed::RegSpec for McrByHh_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Control Register"]
pub type McrByHh = crate::RegValueT<McrByHh_SPEC>;

impl McrByHh {
    #[doc = "Preface Error enable"]
    #[inline(always)]
    pub fn pferen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mcr_by_hh::Pferen,
        mcr_by_hh::Pferen,
        McrByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mcr_by_hh::Pferen,
            mcr_by_hh::Pferen,
            McrByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Error enable"]
    #[inline(always)]
    pub fn syeren(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mcr_by_hh::Syeren,
        mcr_by_hh::Syeren,
        McrByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mcr_by_hh::Syeren,
            mcr_by_hh::Syeren,
            McrByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start bit error enable"]
    #[inline(always)]
    pub fn sberen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mcr_by_hh::Sberen,
        mcr_by_hh::Sberen,
        McrByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mcr_by_hh::Sberen,
            mcr_by_hh::Sberen,
            McrByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, McrByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,McrByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for McrByHh {
    #[inline(always)]
    fn default() -> McrByHh {
        <crate::RegValueT<McrByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr_by_hh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pferen_SPEC;
    pub type Pferen = crate::EnumBitfieldStruct<u8, Pferen_SPEC>;
    impl Pferen {
        #[doc = "Not handle preface error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle preface error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syeren_SPEC;
    pub type Syeren = crate::EnumBitfieldStruct<u8, Syeren_SPEC>;
    impl Syeren {
        #[doc = "Not handle recive sync error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle receive sync error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sberen_SPEC;
    pub type Sberen = crate::EnumBitfieldStruct<u8, Sberen_SPEC>;
    impl Sberen {
        #[doc = "Not handle start bit error as interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Handle start bit error as interrupt."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr_SPEC;
impl crate::sealed::RegSpec for Dcr_SPEC {
    type DataType = u32;
}

#[doc = "Driver Control Register"]
pub type Dcr = crate::RegValueT<Dcr_SPEC>;

impl Dcr {
    #[doc = "Driver Enable POLarity select"]
    #[inline(always)]
    pub fn depol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dcr::Depol,
        dcr::Depol,
        Dcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dcr::Depol,
            dcr::Depol,
            Dcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Driver Enable Assertion Time"]
    #[inline(always)]
    pub fn deast(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Deriver Enable NeGate Time"]
    #[inline(always)]
    pub fn dengt(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<21, 0x7ff, 1, 0, u16, u16, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x7ff,1,0,u16,u16,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        <crate::RegValueT<Dcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Depol_SPEC;
    pub type Depol = crate::EnumBitfieldStruct<u8, Depol_SPEC>;
    impl Depol {
        #[doc = "Driver enable signal is active high."]
        pub const _0: Self = Self::new(0);

        #[doc = "Driver enable signal is active low."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcrHaL_SPEC;
impl crate::sealed::RegSpec for DcrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Driver Control Register"]
pub type DcrHaL = crate::RegValueT<DcrHaL_SPEC>;

impl DcrHaL {
    #[doc = "Driver Enable POLarity select"]
    #[inline(always)]
    pub fn depol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dcr_ha_l::Depol,
        dcr_ha_l::Depol,
        DcrHaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dcr_ha_l::Depol,
            dcr_ha_l::Depol,
            DcrHaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Driver Enable Assertion Time"]
    #[inline(always)]
    pub fn deast(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, DcrHaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,DcrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, DcrHaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,DcrHaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcrHaL {
    #[inline(always)]
    fn default() -> DcrHaL {
        <crate::RegValueT<DcrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcr_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Depol_SPEC;
    pub type Depol = crate::EnumBitfieldStruct<u8, Depol_SPEC>;
    impl Depol {
        #[doc = "Driver enable signal is active high."]
        pub const _0: Self = Self::new(0);

        #[doc = "Driver enable signal is active low."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcrByLl_SPEC;
impl crate::sealed::RegSpec for DcrByLl_SPEC {
    type DataType = u8;
}

#[doc = "Driver Control Register"]
pub type DcrByLl = crate::RegValueT<DcrByLl_SPEC>;

impl DcrByLl {
    #[doc = "Driver Enable POLarity select"]
    #[inline(always)]
    pub fn depol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dcr_by_ll::Depol,
        dcr_by_ll::Depol,
        DcrByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dcr_by_ll::Depol,
            dcr_by_ll::Depol,
            DcrByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, DcrByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,DcrByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcrByLl {
    #[inline(always)]
    fn default() -> DcrByLl {
        <crate::RegValueT<DcrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcr_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Depol_SPEC;
    pub type Depol = crate::EnumBitfieldStruct<u8, Depol_SPEC>;
    impl Depol {
        #[doc = "Driver enable signal is active high."]
        pub const _0: Self = Self::new(0);

        #[doc = "Driver enable signal is active low."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcrByLh_SPEC;
impl crate::sealed::RegSpec for DcrByLh_SPEC {
    type DataType = u8;
}

#[doc = "Driver Control Register"]
pub type DcrByLh = crate::RegValueT<DcrByLh_SPEC>;

impl DcrByLh {
    #[doc = "Driver Enable Assertion Time"]
    #[inline(always)]
    pub fn deast(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcrByLh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DcrByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DcrByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcrByLh {
    #[inline(always)]
    fn default() -> DcrByLh {
        <crate::RegValueT<DcrByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcrHaH_SPEC;
impl crate::sealed::RegSpec for DcrHaH_SPEC {
    type DataType = u16;
}

#[doc = "Driver Control Register"]
pub type DcrHaH = crate::RegValueT<DcrHaH_SPEC>;

impl DcrHaH {
    #[doc = "Deriver Enable NeGate Time"]
    #[inline(always)]
    pub fn dengt(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcrHaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcrHaH_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, u16, DcrHaH_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ff,1,0,u16,u16,DcrHaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcrHaH {
    #[inline(always)]
    fn default() -> DcrHaH {
        <crate::RegValueT<DcrHaH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcrByHl_SPEC;
impl crate::sealed::RegSpec for DcrByHl_SPEC {
    type DataType = u8;
}

#[doc = "Driver Control Register"]
pub type DcrByHl = crate::RegValueT<DcrByHl_SPEC>;

impl DcrByHl {
    #[doc = "Deriver Enable NeGate Time"]
    #[inline(always)]
    pub fn dengt(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcrByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcrByHl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DcrByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DcrByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcrByHl {
    #[inline(always)]
    fn default() -> DcrByHl {
        <crate::RegValueT<DcrByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0_SPEC;
impl crate::sealed::RegSpec for Xcr0_SPEC {
    type DataType = u32;
}

#[doc = "Simple-LIN(SCIX) Control Register 0"]
pub type Xcr0 = crate::RegValueT<Xcr0_SPEC>;

impl Xcr0 {
    #[doc = "Timer count clock source select"]
    #[inline(always)]
    pub fn tcss(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        xcr0::Tcss,
        xcr0::Tcss,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            xcr0::Tcss,
            xcr0::Tcss,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break field presence/absence select"]
    #[inline(always)]
    pub fn bfe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        xcr0::Bfe,
        xcr0::Bfe,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            xcr0::Bfe,
            xcr0::Bfe,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control field 0 presence/absence select"]
    #[inline(always)]
    pub fn cf0re(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        xcr0::Cf0Re,
        xcr0::Cf0Re,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            xcr0::Cf0Re,
            xcr0::Cf0Re,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control field 1 compare data select"]
    #[inline(always)]
    pub fn cf1ds(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        xcr0::Cf1Ds,
        xcr0::Cf1Ds,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            xcr0::Cf1Ds,
            xcr0::Cf1Ds,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit Enable"]
    #[inline(always)]
    pub fn pibe(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        xcr0::Pibe,
        xcr0::Pibe,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            xcr0::Pibe,
            xcr0::Pibe,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit select"]
    #[inline(always)]
    pub fn pibs(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x7,
        1,
        0,
        xcr0::Pibs,
        xcr0::Pibs,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            xcr0::Pibs,
            xcr0::Pibs,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Filed Output end Interrupt Enable"]
    #[inline(always)]
    pub fn bfoie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        xcr0::Bfoie,
        xcr0::Bfoie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            xcr0::Bfoie,
            xcr0::Bfoie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Collision Detect Interrupt Enable"]
    #[inline(always)]
    pub fn bcdie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        xcr0::Bcdie,
        xcr0::Bcdie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            xcr0::Bcdie,
            xcr0::Bcdie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Filed Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bfdie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        xcr0::Bfdie,
        xcr0::Bfdie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            xcr0::Bfdie,
            xcr0::Bfdie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Counter Over Flow Interrupt Enable"]
    #[inline(always)]
    pub fn cofie(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        xcr0::Cofie,
        xcr0::Cofie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            xcr0::Cofie,
            xcr0::Cofie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Active Edge Detection Interrupt Enable"]
    #[inline(always)]
    pub fn aedie(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        xcr0::Aedie,
        xcr0::Aedie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            xcr0::Aedie,
            xcr0::Aedie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Collision detection Clock Select"]
    #[inline(always)]
    pub fn bccs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        xcr0::Bccs,
        xcr0::Bccs,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            xcr0::Bccs,
            xcr0::Bccs,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, u8, Xcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3f,1,0,u8,u8,Xcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr0 {
    #[inline(always)]
    fn default() -> Xcr0 {
        <crate::RegValueT<Xcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcss_SPEC;
    pub type Tcss = crate::EnumBitfieldStruct<u8, Tcss_SPEC>;
    impl Tcss {
        #[doc = "TCLK"]
        pub const _00: Self = Self::new(0);

        #[doc = "TCLK/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfe_SPEC;
    pub type Bfe = crate::EnumBitfieldStruct<u8, Bfe_SPEC>;
    impl Bfe {
        #[doc = "Break field exists in Start frame"]
        pub const _0: Self = Self::new(0);

        #[doc = "No break field in Start frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Re_SPEC;
    pub type Cf0Re = crate::EnumBitfieldStruct<u8, Cf0Re_SPEC>;
    impl Cf0Re {
        #[doc = "Control field 0 exists in Start frame"]
        pub const _0: Self = Self::new(0);

        #[doc = "No Control field 0 in Start frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ds_SPEC;
    pub type Cf1Ds = crate::EnumBitfieldStruct<u8, Cf1Ds_SPEC>;
    impl Cf1Ds {
        #[doc = "XCR1.PCF1D\\[7:0\\] is compare data"]
        pub const _00: Self = Self::new(0);

        #[doc = "XCR1.SCF1D\\[7:0\\] is compare data"]
        pub const _01: Self = Self::new(1);

        #[doc = "XCR1.PCF1D\\[7:0\\] and  XCR1.SCF1D\\[7:0\\]  are compare data"]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibit"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibe_SPEC;
    pub type Pibe = crate::EnumBitfieldStruct<u8, Pibe_SPEC>;
    impl Pibe {
        #[doc = "Priority interrupt bit is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority interrupt bit is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibs_SPEC;
    pub type Pibs = crate::EnumBitfieldStruct<u8, Pibs_SPEC>;
    impl Pibs {
        #[doc = "Control Field 1, bit 0."]
        pub const _000: Self = Self::new(0);

        #[doc = "Control Field 1, bit 1."]
        pub const _001: Self = Self::new(1);

        #[doc = "Control Field 1, bit 2."]
        pub const _010: Self = Self::new(2);

        #[doc = "Control Field 1, bit 3."]
        pub const _011: Self = Self::new(3);

        #[doc = "Control Field 1, bit 4."]
        pub const _100: Self = Self::new(4);

        #[doc = "Control Field 1, bit 5."]
        pub const _101: Self = Self::new(5);

        #[doc = "Control Field 1, bit 6."]
        pub const _110: Self = Self::new(6);

        #[doc = "Control Field 1, bit 7."]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfoie_SPEC;
    pub type Bfoie = crate::EnumBitfieldStruct<u8, Bfoie_SPEC>;
    impl Bfoie {
        #[doc = "Break field output end is not a factor of TXI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Break field output end is a factor of TXI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdie_SPEC;
    pub type Bcdie = crate::EnumBitfieldStruct<u8, Bcdie_SPEC>;
    impl Bcdie {
        #[doc = "Bus collision detection is not a factor of ERI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus collision detection is a factor of ERI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdie_SPEC;
    pub type Bfdie = crate::EnumBitfieldStruct<u8, Bfdie_SPEC>;
    impl Bfdie {
        #[doc = "Break field detection interrupt is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Break field detection interrupt is enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cofie_SPEC;
    pub type Cofie = crate::EnumBitfieldStruct<u8, Cofie_SPEC>;
    impl Cofie {
        #[doc = "Counter over flow is not a factor of ERI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter over flow is a factor of ERI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedie_SPEC;
    pub type Aedie = crate::EnumBitfieldStruct<u8, Aedie_SPEC>;
    impl Aedie {
        #[doc = "Active edge detection interrupt is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Active edge detection interrupt is enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bccs_SPEC;
    pub type Bccs = crate::EnumBitfieldStruct<u8, Bccs_SPEC>;
    impl Bccs {
        #[doc = "RSCI base clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "RSCI base clock/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "RSCI base clock/4"]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibit"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0HaL_SPEC;
impl crate::sealed::RegSpec for Xcr0HaL_SPEC {
    type DataType = u16;
}

#[doc = "Simple-LIN(SCIX) Control Register 0"]
pub type Xcr0HaL = crate::RegValueT<Xcr0HaL_SPEC>;

impl Xcr0HaL {
    #[doc = "Timer count clock source select"]
    #[inline(always)]
    pub fn tcss(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        xcr0_ha_l::Tcss,
        xcr0_ha_l::Tcss,
        Xcr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            xcr0_ha_l::Tcss,
            xcr0_ha_l::Tcss,
            Xcr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Xcr0HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Xcr0HaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Break field presence/absence select"]
    #[inline(always)]
    pub fn bfe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        xcr0_ha_l::Bfe,
        xcr0_ha_l::Bfe,
        Xcr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            xcr0_ha_l::Bfe,
            xcr0_ha_l::Bfe,
            Xcr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control field 0 presence/absence select"]
    #[inline(always)]
    pub fn cf0re(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        xcr0_ha_l::Cf0Re,
        xcr0_ha_l::Cf0Re,
        Xcr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            xcr0_ha_l::Cf0Re,
            xcr0_ha_l::Cf0Re,
            Xcr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control field 1 compare data select"]
    #[inline(always)]
    pub fn cf1ds(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        xcr0_ha_l::Cf1Ds,
        xcr0_ha_l::Cf1Ds,
        Xcr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            xcr0_ha_l::Cf1Ds,
            xcr0_ha_l::Cf1Ds,
            Xcr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit Enable"]
    #[inline(always)]
    pub fn pibe(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        xcr0_ha_l::Pibe,
        xcr0_ha_l::Pibe,
        Xcr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            xcr0_ha_l::Pibe,
            xcr0_ha_l::Pibe,
            Xcr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit select"]
    #[inline(always)]
    pub fn pibs(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x7,
        1,
        0,
        xcr0_ha_l::Pibs,
        xcr0_ha_l::Pibs,
        Xcr0HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            xcr0_ha_l::Pibs,
            xcr0_ha_l::Pibs,
            Xcr0HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr0HaL {
    #[inline(always)]
    fn default() -> Xcr0HaL {
        <crate::RegValueT<Xcr0HaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcss_SPEC;
    pub type Tcss = crate::EnumBitfieldStruct<u8, Tcss_SPEC>;
    impl Tcss {
        #[doc = "TCLK"]
        pub const _00: Self = Self::new(0);

        #[doc = "TCLK/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfe_SPEC;
    pub type Bfe = crate::EnumBitfieldStruct<u8, Bfe_SPEC>;
    impl Bfe {
        #[doc = "Break field exists in Start frame"]
        pub const _0: Self = Self::new(0);

        #[doc = "No break field in Start frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Re_SPEC;
    pub type Cf0Re = crate::EnumBitfieldStruct<u8, Cf0Re_SPEC>;
    impl Cf0Re {
        #[doc = "Control field 0 exists in Start frame"]
        pub const _0: Self = Self::new(0);

        #[doc = "No Control field 0 in Start frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ds_SPEC;
    pub type Cf1Ds = crate::EnumBitfieldStruct<u8, Cf1Ds_SPEC>;
    impl Cf1Ds {
        #[doc = "XCR1.PCF1D\\[7:0\\] is compare data"]
        pub const _00: Self = Self::new(0);

        #[doc = "XCR1.SCF1D\\[7:0\\] is compare data"]
        pub const _01: Self = Self::new(1);

        #[doc = "XCR1.PCF1D\\[7:0\\] and  XCR1.SCF1D\\[7:0\\]  are compare data"]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibit"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibe_SPEC;
    pub type Pibe = crate::EnumBitfieldStruct<u8, Pibe_SPEC>;
    impl Pibe {
        #[doc = "Priority interrupt bit is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority interrupt bit is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibs_SPEC;
    pub type Pibs = crate::EnumBitfieldStruct<u8, Pibs_SPEC>;
    impl Pibs {
        #[doc = "Control Field 1, bit 0."]
        pub const _000: Self = Self::new(0);

        #[doc = "Control Field 1, bit 1."]
        pub const _001: Self = Self::new(1);

        #[doc = "Control Field 1, bit 2."]
        pub const _010: Self = Self::new(2);

        #[doc = "Control Field 1, bit 3."]
        pub const _011: Self = Self::new(3);

        #[doc = "Control Field 1, bit 4."]
        pub const _100: Self = Self::new(4);

        #[doc = "Control Field 1, bit 5."]
        pub const _101: Self = Self::new(5);

        #[doc = "Control Field 1, bit 6."]
        pub const _110: Self = Self::new(6);

        #[doc = "Control Field 1, bit 7."]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0ByLl_SPEC;
impl crate::sealed::RegSpec for Xcr0ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 0"]
pub type Xcr0ByLl = crate::RegValueT<Xcr0ByLl_SPEC>;

impl Xcr0ByLl {
    #[doc = "Timer count clock source select"]
    #[inline(always)]
    pub fn tcss(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        xcr0_by_ll::Tcss,
        xcr0_by_ll::Tcss,
        Xcr0ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            xcr0_by_ll::Tcss,
            xcr0_by_ll::Tcss,
            Xcr0ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Xcr0ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Xcr0ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr0ByLl {
    #[inline(always)]
    fn default() -> Xcr0ByLl {
        <crate::RegValueT<Xcr0ByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcss_SPEC;
    pub type Tcss = crate::EnumBitfieldStruct<u8, Tcss_SPEC>;
    impl Tcss {
        #[doc = "TCLK"]
        pub const _00: Self = Self::new(0);

        #[doc = "TCLK/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "TCLK/16"]
        pub const _10: Self = Self::new(2);

        #[doc = "TCLK/64"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0ByLh_SPEC;
impl crate::sealed::RegSpec for Xcr0ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 0"]
pub type Xcr0ByLh = crate::RegValueT<Xcr0ByLh_SPEC>;

impl Xcr0ByLh {
    #[doc = "Break field presence/absence select"]
    #[inline(always)]
    pub fn bfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr0_by_lh::Bfe,
        xcr0_by_lh::Bfe,
        Xcr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr0_by_lh::Bfe,
            xcr0_by_lh::Bfe,
            Xcr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control field 0 presence/absence select"]
    #[inline(always)]
    pub fn cf0re(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        xcr0_by_lh::Cf0Re,
        xcr0_by_lh::Cf0Re,
        Xcr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            xcr0_by_lh::Cf0Re,
            xcr0_by_lh::Cf0Re,
            Xcr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control field 1 compare data select"]
    #[inline(always)]
    pub fn cf1ds(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        xcr0_by_lh::Cf1Ds,
        xcr0_by_lh::Cf1Ds,
        Xcr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            xcr0_by_lh::Cf1Ds,
            xcr0_by_lh::Cf1Ds,
            Xcr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit Enable"]
    #[inline(always)]
    pub fn pibe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr0_by_lh::Pibe,
        xcr0_by_lh::Pibe,
        Xcr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr0_by_lh::Pibe,
            xcr0_by_lh::Pibe,
            Xcr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Priority Interrupt Bit select"]
    #[inline(always)]
    pub fn pibs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7,
        1,
        0,
        xcr0_by_lh::Pibs,
        xcr0_by_lh::Pibs,
        Xcr0ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7,
            1,
            0,
            xcr0_by_lh::Pibs,
            xcr0_by_lh::Pibs,
            Xcr0ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr0ByLh {
    #[inline(always)]
    fn default() -> Xcr0ByLh {
        <crate::RegValueT<Xcr0ByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfe_SPEC;
    pub type Bfe = crate::EnumBitfieldStruct<u8, Bfe_SPEC>;
    impl Bfe {
        #[doc = "Break field exists in Start frame"]
        pub const _0: Self = Self::new(0);

        #[doc = "No break field in Start frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Re_SPEC;
    pub type Cf0Re = crate::EnumBitfieldStruct<u8, Cf0Re_SPEC>;
    impl Cf0Re {
        #[doc = "Control field 0 exists in Start frame"]
        pub const _0: Self = Self::new(0);

        #[doc = "No Control field 0 in Start frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ds_SPEC;
    pub type Cf1Ds = crate::EnumBitfieldStruct<u8, Cf1Ds_SPEC>;
    impl Cf1Ds {
        #[doc = "XCR1.PCF1D\\[7:0\\] is compare data"]
        pub const _00: Self = Self::new(0);

        #[doc = "XCR1.SCF1D\\[7:0\\] is compare data"]
        pub const _01: Self = Self::new(1);

        #[doc = "XCR1.PCF1D\\[7:0\\] and  XCR1.SCF1D\\[7:0\\]  are compare data"]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibit"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibe_SPEC;
    pub type Pibe = crate::EnumBitfieldStruct<u8, Pibe_SPEC>;
    impl Pibe {
        #[doc = "Priority interrupt bit is disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority interrupt bit is enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibs_SPEC;
    pub type Pibs = crate::EnumBitfieldStruct<u8, Pibs_SPEC>;
    impl Pibs {
        #[doc = "Control Field 1, bit 0."]
        pub const _000: Self = Self::new(0);

        #[doc = "Control Field 1, bit 1."]
        pub const _001: Self = Self::new(1);

        #[doc = "Control Field 1, bit 2."]
        pub const _010: Self = Self::new(2);

        #[doc = "Control Field 1, bit 3."]
        pub const _011: Self = Self::new(3);

        #[doc = "Control Field 1, bit 4."]
        pub const _100: Self = Self::new(4);

        #[doc = "Control Field 1, bit 5."]
        pub const _101: Self = Self::new(5);

        #[doc = "Control Field 1, bit 6."]
        pub const _110: Self = Self::new(6);

        #[doc = "Control Field 1, bit 7."]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0HaH_SPEC;
impl crate::sealed::RegSpec for Xcr0HaH_SPEC {
    type DataType = u16;
}

#[doc = "Simple-LIN(SCIX) Control Register 0"]
pub type Xcr0HaH = crate::RegValueT<Xcr0HaH_SPEC>;

impl Xcr0HaH {
    #[doc = "Break Filed Output end Interrupt Enable"]
    #[inline(always)]
    pub fn bfoie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr0_ha_h::Bfoie,
        xcr0_ha_h::Bfoie,
        Xcr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr0_ha_h::Bfoie,
            xcr0_ha_h::Bfoie,
            Xcr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Collision Detect Interrupt Enable"]
    #[inline(always)]
    pub fn bcdie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        xcr0_ha_h::Bcdie,
        xcr0_ha_h::Bcdie,
        Xcr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            xcr0_ha_h::Bcdie,
            xcr0_ha_h::Bcdie,
            Xcr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Filed Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bfdie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr0_ha_h::Bfdie,
        xcr0_ha_h::Bfdie,
        Xcr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr0_ha_h::Bfdie,
            xcr0_ha_h::Bfdie,
            Xcr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Counter Over Flow Interrupt Enable"]
    #[inline(always)]
    pub fn cofie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        xcr0_ha_h::Cofie,
        xcr0_ha_h::Cofie,
        Xcr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            xcr0_ha_h::Cofie,
            xcr0_ha_h::Cofie,
            Xcr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Active Edge Detection Interrupt Enable"]
    #[inline(always)]
    pub fn aedie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        xcr0_ha_h::Aedie,
        xcr0_ha_h::Aedie,
        Xcr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            xcr0_ha_h::Aedie,
            xcr0_ha_h::Aedie,
            Xcr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Collision detection Clock Select"]
    #[inline(always)]
    pub fn bccs(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        xcr0_ha_h::Bccs,
        xcr0_ha_h::Bccs,
        Xcr0HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            xcr0_ha_h::Bccs,
            xcr0_ha_h::Bccs,
            Xcr0HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Xcr0HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Xcr0HaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr0HaH {
    #[inline(always)]
    fn default() -> Xcr0HaH {
        <crate::RegValueT<Xcr0HaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfoie_SPEC;
    pub type Bfoie = crate::EnumBitfieldStruct<u8, Bfoie_SPEC>;
    impl Bfoie {
        #[doc = "Break field output end is not a factor of TXI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Break field output end is a factor of TXI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdie_SPEC;
    pub type Bcdie = crate::EnumBitfieldStruct<u8, Bcdie_SPEC>;
    impl Bcdie {
        #[doc = "Bus collision detection is not a factor of ERI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus collision detection is a factor of ERI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdie_SPEC;
    pub type Bfdie = crate::EnumBitfieldStruct<u8, Bfdie_SPEC>;
    impl Bfdie {
        #[doc = "Break field detection interrupt is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Break field detection interrupt is enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cofie_SPEC;
    pub type Cofie = crate::EnumBitfieldStruct<u8, Cofie_SPEC>;
    impl Cofie {
        #[doc = "Counter over flow is not a factor of ERI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter over flow is a factor of ERI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedie_SPEC;
    pub type Aedie = crate::EnumBitfieldStruct<u8, Aedie_SPEC>;
    impl Aedie {
        #[doc = "Active edge detection interrupt is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Active edge detection interrupt is enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bccs_SPEC;
    pub type Bccs = crate::EnumBitfieldStruct<u8, Bccs_SPEC>;
    impl Bccs {
        #[doc = "RSCI base clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "RSCI base clock/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "RSCI base clock/4"]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibit"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0ByHl_SPEC;
impl crate::sealed::RegSpec for Xcr0ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 0"]
pub type Xcr0ByHl = crate::RegValueT<Xcr0ByHl_SPEC>;

impl Xcr0ByHl {
    #[doc = "Break Filed Output end Interrupt Enable"]
    #[inline(always)]
    pub fn bfoie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr0_by_hl::Bfoie,
        xcr0_by_hl::Bfoie,
        Xcr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr0_by_hl::Bfoie,
            xcr0_by_hl::Bfoie,
            Xcr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Collision Detect Interrupt Enable"]
    #[inline(always)]
    pub fn bcdie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        xcr0_by_hl::Bcdie,
        xcr0_by_hl::Bcdie,
        Xcr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            xcr0_by_hl::Bcdie,
            xcr0_by_hl::Bcdie,
            Xcr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Filed Detection Interrupt Enable"]
    #[inline(always)]
    pub fn bfdie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr0_by_hl::Bfdie,
        xcr0_by_hl::Bfdie,
        Xcr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr0_by_hl::Bfdie,
            xcr0_by_hl::Bfdie,
            Xcr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Counter Over Flow Interrupt Enable"]
    #[inline(always)]
    pub fn cofie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        xcr0_by_hl::Cofie,
        xcr0_by_hl::Cofie,
        Xcr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            xcr0_by_hl::Cofie,
            xcr0_by_hl::Cofie,
            Xcr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Active Edge Detection Interrupt Enable"]
    #[inline(always)]
    pub fn aedie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        xcr0_by_hl::Aedie,
        xcr0_by_hl::Aedie,
        Xcr0ByHl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            xcr0_by_hl::Aedie,
            xcr0_by_hl::Aedie,
            Xcr0ByHl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Xcr0ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Xcr0ByHl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Xcr0ByHl {
    #[inline(always)]
    fn default() -> Xcr0ByHl {
        <crate::RegValueT<Xcr0ByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0_by_hl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfoie_SPEC;
    pub type Bfoie = crate::EnumBitfieldStruct<u8, Bfoie_SPEC>;
    impl Bfoie {
        #[doc = "Break field output end is not a factor of TXI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Break field output end is a factor of TXI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdie_SPEC;
    pub type Bcdie = crate::EnumBitfieldStruct<u8, Bcdie_SPEC>;
    impl Bcdie {
        #[doc = "Bus collision detection is not a factor of ERI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus collision detection is a factor of ERI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdie_SPEC;
    pub type Bfdie = crate::EnumBitfieldStruct<u8, Bfdie_SPEC>;
    impl Bfdie {
        #[doc = "Break field detection interrupt is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Break field detection interrupt is enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cofie_SPEC;
    pub type Cofie = crate::EnumBitfieldStruct<u8, Cofie_SPEC>;
    impl Cofie {
        #[doc = "Counter over flow is not a factor of ERI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter over flow is a factor of ERI."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedie_SPEC;
    pub type Aedie = crate::EnumBitfieldStruct<u8, Aedie_SPEC>;
    impl Aedie {
        #[doc = "Active edge detection interrupt is disable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Active edge detection interrupt is enable."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0ByHh_SPEC;
impl crate::sealed::RegSpec for Xcr0ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 0"]
pub type Xcr0ByHh = crate::RegValueT<Xcr0ByHh_SPEC>;

impl Xcr0ByHh {
    #[doc = "Bus Collision detection Clock Select"]
    #[inline(always)]
    pub fn bccs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        xcr0_by_hh::Bccs,
        xcr0_by_hh::Bccs,
        Xcr0ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            xcr0_by_hh::Bccs,
            xcr0_by_hh::Bccs,
            Xcr0ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Xcr0ByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Xcr0ByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr0ByHh {
    #[inline(always)]
    fn default() -> Xcr0ByHh {
        <crate::RegValueT<Xcr0ByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0_by_hh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bccs_SPEC;
    pub type Bccs = crate::EnumBitfieldStruct<u8, Bccs_SPEC>;
    impl Bccs {
        #[doc = "RSCI base clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "RSCI base clock/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "RSCI base clock/4"]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibit"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1_SPEC;
impl crate::sealed::RegSpec for Xcr1_SPEC {
    type DataType = u32;
}

#[doc = "Simple-LIN(SCIX) Control Register 1"]
pub type Xcr1 = crate::RegValueT<Xcr1_SPEC>;

impl Xcr1 {
    #[doc = "break field Timer Count Start trigger"]
    #[inline(always)]
    pub fn tcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr1::Tcst,
        xcr1::Tcst,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr1::Tcst,
            xcr1::Tcst,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start frame Detection Start trigger"]
    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr1::Sdst,
        xcr1::Sdst,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr1::Sdst,
            xcr1::Sdst,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit rate Measurement function Enable"]
    #[inline(always)]
    pub fn bmen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        xcr1::Bmen,
        xcr1::Bmen,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            xcr1::Bmen,
            xcr1::Bmen,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Xcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Xcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Priority compare Data for Control Field 1"]
    #[inline(always)]
    pub fn pcf1d(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Xcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Xcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Secondary compare Data for Control Field 1"]
    #[inline(always)]
    pub fn scf1d(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Xcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Xcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Field 1 Compare bit Enable"]
    #[inline(always)]
    pub fn cf1ce(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        xcr1::Cf1Ce,
        xcr1::Cf1Ce,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            xcr1::Cf1Ce,
            xcr1::Cf1Ce,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr1 {
    #[inline(always)]
    fn default() -> Xcr1 {
        <crate::RegValueT<Xcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcst_SPEC;
    pub type Tcst = crate::EnumBitfieldStruct<u8, Tcst_SPEC>;
    impl Tcst {
        #[doc = "Timer count abort for break field output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer count start for break field output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdst_SPEC;
    pub type Sdst = crate::EnumBitfieldStruct<u8, Sdst_SPEC>;
    impl Sdst {
        #[doc = "Start frame / Break field detetion disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start frame / Break field detetion enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bmen_SPEC;
    pub type Bmen = crate::EnumBitfieldStruct<u8, Bmen_SPEC>;
    impl Bmen {
        #[doc = "Bit rate Measurement function disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate Measurement function enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce_SPEC;
    pub type Cf1Ce = crate::EnumBitfieldStruct<u8, Cf1Ce_SPEC>;
    impl Cf1Ce {
        #[doc = "Bit N of control Field 1  is not for comparison"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit N of control Field 1  is for comparison"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1HaL_SPEC;
impl crate::sealed::RegSpec for Xcr1HaL_SPEC {
    type DataType = u16;
}

#[doc = "Simple-LIN(SCIX) Control Register 1"]
pub type Xcr1HaL = crate::RegValueT<Xcr1HaL_SPEC>;

impl Xcr1HaL {
    #[doc = "break field Timer Count Start trigger"]
    #[inline(always)]
    pub fn tcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr1_ha_l::Tcst,
        xcr1_ha_l::Tcst,
        Xcr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr1_ha_l::Tcst,
            xcr1_ha_l::Tcst,
            Xcr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start frame Detection Start trigger"]
    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr1_ha_l::Sdst,
        xcr1_ha_l::Sdst,
        Xcr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr1_ha_l::Sdst,
            xcr1_ha_l::Sdst,
            Xcr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit rate Measurement function Enable"]
    #[inline(always)]
    pub fn bmen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        xcr1_ha_l::Bmen,
        xcr1_ha_l::Bmen,
        Xcr1HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            xcr1_ha_l::Bmen,
            xcr1_ha_l::Bmen,
            Xcr1HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Xcr1HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Xcr1HaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Priority compare Data for Control Field 1"]
    #[inline(always)]
    pub fn pcf1d(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Xcr1HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Xcr1HaL_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr1HaL {
    #[inline(always)]
    fn default() -> Xcr1HaL {
        <crate::RegValueT<Xcr1HaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr1_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcst_SPEC;
    pub type Tcst = crate::EnumBitfieldStruct<u8, Tcst_SPEC>;
    impl Tcst {
        #[doc = "Timer count abort for break field output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer count start for break field output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdst_SPEC;
    pub type Sdst = crate::EnumBitfieldStruct<u8, Sdst_SPEC>;
    impl Sdst {
        #[doc = "Start frame / Break field detetion disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start frame / Break field detetion enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bmen_SPEC;
    pub type Bmen = crate::EnumBitfieldStruct<u8, Bmen_SPEC>;
    impl Bmen {
        #[doc = "Bit rate Measurement function disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate Measurement function enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1ByLl_SPEC;
impl crate::sealed::RegSpec for Xcr1ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 1"]
pub type Xcr1ByLl = crate::RegValueT<Xcr1ByLl_SPEC>;

impl Xcr1ByLl {
    #[doc = "break field Timer Count Start trigger"]
    #[inline(always)]
    pub fn tcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr1_by_ll::Tcst,
        xcr1_by_ll::Tcst,
        Xcr1ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr1_by_ll::Tcst,
            xcr1_by_ll::Tcst,
            Xcr1ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start frame Detection Start trigger"]
    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr1_by_ll::Sdst,
        xcr1_by_ll::Sdst,
        Xcr1ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr1_by_ll::Sdst,
            xcr1_by_ll::Sdst,
            Xcr1ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit rate Measurement function Enable"]
    #[inline(always)]
    pub fn bmen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        xcr1_by_ll::Bmen,
        xcr1_by_ll::Bmen,
        Xcr1ByLl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            xcr1_by_ll::Bmen,
            xcr1_by_ll::Bmen,
            Xcr1ByLl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Xcr1ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Xcr1ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr1ByLl {
    #[inline(always)]
    fn default() -> Xcr1ByLl {
        <crate::RegValueT<Xcr1ByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr1_by_ll {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcst_SPEC;
    pub type Tcst = crate::EnumBitfieldStruct<u8, Tcst_SPEC>;
    impl Tcst {
        #[doc = "Timer count abort for break field output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer count start for break field output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdst_SPEC;
    pub type Sdst = crate::EnumBitfieldStruct<u8, Sdst_SPEC>;
    impl Sdst {
        #[doc = "Start frame / Break field detetion disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start frame / Break field detetion enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bmen_SPEC;
    pub type Bmen = crate::EnumBitfieldStruct<u8, Bmen_SPEC>;
    impl Bmen {
        #[doc = "Bit rate Measurement function disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit rate Measurement function enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1ByLh_SPEC;
impl crate::sealed::RegSpec for Xcr1ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 1"]
pub type Xcr1ByLh = crate::RegValueT<Xcr1ByLh_SPEC>;

impl Xcr1ByLh {
    #[doc = "Priority compare Data for Control Field 1"]
    #[inline(always)]
    pub fn pcf1d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr1ByLh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr1ByLh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr1ByLh {
    #[inline(always)]
    fn default() -> Xcr1ByLh {
        <crate::RegValueT<Xcr1ByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1HaH_SPEC;
impl crate::sealed::RegSpec for Xcr1HaH_SPEC {
    type DataType = u16;
}

#[doc = "Simple-LIN(SCIX) Control Register 1"]
pub type Xcr1HaH = crate::RegValueT<Xcr1HaH_SPEC>;

impl Xcr1HaH {
    #[doc = "Secondary compare Data for Control Field 1"]
    #[inline(always)]
    pub fn scf1d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr1HaH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr1HaH_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Field 1 Compare bit Enable"]
    #[inline(always)]
    pub fn cf1ce(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        xcr1_ha_h::Cf1Ce,
        xcr1_ha_h::Cf1Ce,
        Xcr1HaH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            xcr1_ha_h::Cf1Ce,
            xcr1_ha_h::Cf1Ce,
            Xcr1HaH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr1HaH {
    #[inline(always)]
    fn default() -> Xcr1HaH {
        <crate::RegValueT<Xcr1HaH_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr1_ha_h {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce_SPEC;
    pub type Cf1Ce = crate::EnumBitfieldStruct<u8, Cf1Ce_SPEC>;
    impl Cf1Ce {
        #[doc = "Bit N of control Field 1  is not for comparison"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit N of control Field 1  is for comparison"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1ByHl_SPEC;
impl crate::sealed::RegSpec for Xcr1ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 1"]
pub type Xcr1ByHl = crate::RegValueT<Xcr1ByHl_SPEC>;

impl Xcr1ByHl {
    #[doc = "Secondary compare Data for Control Field 1"]
    #[inline(always)]
    pub fn scf1d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr1ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr1ByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr1ByHl {
    #[inline(always)]
    fn default() -> Xcr1ByHl {
        <crate::RegValueT<Xcr1ByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1ByHh_SPEC;
impl crate::sealed::RegSpec for Xcr1ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 1"]
pub type Xcr1ByHh = crate::RegValueT<Xcr1ByHh_SPEC>;

impl Xcr1ByHh {
    #[doc = "Control Field 1 Compare bit Enable"]
    #[inline(always)]
    pub fn cf1ce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        xcr1_by_hh::Cf1Ce,
        xcr1_by_hh::Cf1Ce,
        Xcr1ByHh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            xcr1_by_hh::Cf1Ce,
            xcr1_by_hh::Cf1Ce,
            Xcr1ByHh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr1ByHh {
    #[inline(always)]
    fn default() -> Xcr1ByHh {
        <crate::RegValueT<Xcr1ByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr1_by_hh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce_SPEC;
    pub type Cf1Ce = crate::EnumBitfieldStruct<u8, Cf1Ce_SPEC>;
    impl Cf1Ce {
        #[doc = "Bit N of control Field 1  is not for comparison"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit N of control Field 1  is for comparison"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2_SPEC;
impl crate::sealed::RegSpec for Xcr2_SPEC {
    type DataType = u32;
}

#[doc = "Simple-LIN(SCIX) Control Register 2"]
pub type Xcr2 = crate::RegValueT<Xcr2_SPEC>;

impl Xcr2 {
    #[doc = "Compare Data for Control Field 0"]
    #[inline(always)]
    pub fn cf0d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Field 0 Compare bit Enable"]
    #[inline(always)]
    pub fn cf0ce(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        xcr2::Cf0Ce,
        xcr2::Cf0Ce,
        Xcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            xcr2::Cf0Ce,
            xcr2::Cf0Ce,
            Xcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Break Field Low Width"]
    #[inline(always)]
    pub fn bflw(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Xcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Xcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr2 {
    #[inline(always)]
    fn default() -> Xcr2 {
        <crate::RegValueT<Xcr2_SPEC> as RegisterValue<_>>::new(4294836224)
    }
}
pub mod xcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce_SPEC;
    pub type Cf0Ce = crate::EnumBitfieldStruct<u8, Cf0Ce_SPEC>;
    impl Cf0Ce {
        #[doc = "Bit N of control Field 0  is not for comparison"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit N of control Field 0  is for comparison"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2HaL_SPEC;
impl crate::sealed::RegSpec for Xcr2HaL_SPEC {
    type DataType = u16;
}

#[doc = "Simple-LIN(SCIX) Control Register 2"]
pub type Xcr2HaL = crate::RegValueT<Xcr2HaL_SPEC>;

impl Xcr2HaL {
    #[doc = "Compare Data for Control Field 0"]
    #[inline(always)]
    pub fn cf0d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr2HaL_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr2HaL_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Field 0 Compare bit Enable"]
    #[inline(always)]
    pub fn cf0ce(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        xcr2_ha_l::Cf0Ce,
        xcr2_ha_l::Cf0Ce,
        Xcr2HaL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            xcr2_ha_l::Cf0Ce,
            xcr2_ha_l::Cf0Ce,
            Xcr2HaL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr2HaL {
    #[inline(always)]
    fn default() -> Xcr2HaL {
        <crate::RegValueT<Xcr2HaL_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr2_ha_l {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce_SPEC;
    pub type Cf0Ce = crate::EnumBitfieldStruct<u8, Cf0Ce_SPEC>;
    impl Cf0Ce {
        #[doc = "Bit N of control Field 0  is not for comparison"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit N of control Field 0  is for comparison"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2ByLl_SPEC;
impl crate::sealed::RegSpec for Xcr2ByLl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 2"]
pub type Xcr2ByLl = crate::RegValueT<Xcr2ByLl_SPEC>;

impl Xcr2ByLl {
    #[doc = "Compare Data for Control Field 0"]
    #[inline(always)]
    pub fn cf0d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr2ByLl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr2ByLl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr2ByLl {
    #[inline(always)]
    fn default() -> Xcr2ByLl {
        <crate::RegValueT<Xcr2ByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2ByLh_SPEC;
impl crate::sealed::RegSpec for Xcr2ByLh_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 2"]
pub type Xcr2ByLh = crate::RegValueT<Xcr2ByLh_SPEC>;

impl Xcr2ByLh {
    #[doc = "Control Field 0 Compare bit Enable"]
    #[inline(always)]
    pub fn cf0ce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        xcr2_by_lh::Cf0Ce,
        xcr2_by_lh::Cf0Ce,
        Xcr2ByLh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            xcr2_by_lh::Cf0Ce,
            xcr2_by_lh::Cf0Ce,
            Xcr2ByLh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr2ByLh {
    #[inline(always)]
    fn default() -> Xcr2ByLh {
        <crate::RegValueT<Xcr2ByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr2_by_lh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce_SPEC;
    pub type Cf0Ce = crate::EnumBitfieldStruct<u8, Cf0Ce_SPEC>;
    impl Cf0Ce {
        #[doc = "Bit N of control Field 0  is not for comparison"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bit N of control Field 0  is for comparison"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2HaH_SPEC;
impl crate::sealed::RegSpec for Xcr2HaH_SPEC {
    type DataType = u16;
}

#[doc = "Simple-LIN(SCIX) Control Register 2"]
pub type Xcr2HaH = crate::RegValueT<Xcr2HaH_SPEC>;

impl Xcr2HaH {
    #[doc = "Break Field Low Width"]
    #[inline(always)]
    pub fn bflw(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Xcr2HaH_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Xcr2HaH_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr2HaH {
    #[inline(always)]
    fn default() -> Xcr2HaH {
        <crate::RegValueT<Xcr2HaH_SPEC> as RegisterValue<_>>::new(65534)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2ByHl_SPEC;
impl crate::sealed::RegSpec for Xcr2ByHl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 2"]
pub type Xcr2ByHl = crate::RegValueT<Xcr2ByHl_SPEC>;

impl Xcr2ByHl {
    #[doc = "Break Field Low Width"]
    #[inline(always)]
    pub fn bflw(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr2ByHl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr2ByHl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr2ByHl {
    #[inline(always)]
    fn default() -> Xcr2ByHl {
        <crate::RegValueT<Xcr2ByHl_SPEC> as RegisterValue<_>>::new(254)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2ByHh_SPEC;
impl crate::sealed::RegSpec for Xcr2ByHh_SPEC {
    type DataType = u8;
}

#[doc = "Simple-LIN(SCIX) Control Register 2"]
pub type Xcr2ByHh = crate::RegValueT<Xcr2ByHh_SPEC>;

impl Xcr2ByHh {
    #[doc = "Break Field Low Width"]
    #[inline(always)]
    pub fn bflw(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr2ByHh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr2ByHh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr2ByHh {
    #[inline(always)]
    fn default() -> Xcr2ByHh {
        <crate::RegValueT<Xcr2ByHh_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr_SPEC;
impl crate::sealed::RegSpec for Csr_SPEC {
    type DataType = u32;
}

#[doc = "Common Status Register"]
pub type Csr = crate::RegValueT<Csr_SPEC>;

impl Csr {
    #[doc = "Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, csr::Ers, csr::Ers, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,csr::Ers,csr::Ers,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Serial input data monitor bit"]
    #[inline(always)]
    pub fn rxdmon(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        csr::RxDmon,
        csr::RxDmon,
        Csr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            csr::RxDmon,
            csr::RxDmon,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Flag"]
    #[inline(always)]
    pub fn dcmf(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, csr::Dcmf, csr::Dcmf, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            csr::Dcmf,
            csr::Dcmf,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub fn dper(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, csr::Dper, csr::Dper, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            csr::Dper,
            csr::Dper,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub fn dfer(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, csr::Dfer, csr::Dfer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            csr::Dfer,
            csr::Dfer,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, csr::Orer, csr::Orer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            csr::Orer,
            csr::Orer,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Csr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Csr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Mode Fault Flag"]
    #[inline(always)]
    pub fn mff(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, csr::Mff, csr::Mff, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,csr::Mff,csr::Mff,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, csr::Per, csr::Per, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x1,1,0,csr::Per,csr::Per,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, csr::Fer, csr::Fer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,csr::Fer,csr::Fer,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, csr::Tdre, csr::Tdre, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            csr::Tdre,
            csr::Tdre,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, csr::Tend, csr::Tend, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            csr::Tend,
            csr::Tend,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, csr::Rdrf, csr::Rdrf, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            csr::Rdrf,
            csr::Rdrf,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        <crate::RegValueT<Csr_SPEC> as RegisterValue<_>>::new(1610645504)
    }
}
pub mod csr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ers_SPEC;
    pub type Ers = crate::EnumBitfieldStruct<u8, Ers_SPEC>;
    impl Ers {
        #[doc = "Low error signal not responded"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low error signal responded"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RxDmon_SPEC;
    pub type RxDmon = crate::EnumBitfieldStruct<u8, RxDmon_SPEC>;
    impl RxDmon {
        #[doc = "RXD terminal is the Low level, when RINV is 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "RXD terminal is the High level, when RINV is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcmf_SPEC;
    pub type Dcmf = crate::EnumBitfieldStruct<u8, Dcmf_SPEC>;
    impl Dcmf {
        #[doc = "No matched"]
        pub const _0: Self = Self::new(0);

        #[doc = "Matched"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dper_SPEC;
    pub type Dper = crate::EnumBitfieldStruct<u8, Dper_SPEC>;
    impl Dper {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfer_SPEC;
    pub type Dfer = crate::EnumBitfieldStruct<u8, Dfer_SPEC>;
    impl Dfer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An overrun error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mff_SPEC;
    pub type Mff = crate::EnumBitfieldStruct<u8, Mff_SPEC>;
    impl Mff {
        #[doc = "No mode fault error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mode fault error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A parity error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A framing error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data is in TDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "No transmit data is in TDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Character transfer has been completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data is in RDR register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Received data is in RDR register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}

#[doc = "Simple-I2C Status Register"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "ACK Reception Data Flag"]
    #[inline(always)]
    pub fn iicackr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        isr::Iicackr,
        isr::Iicackr,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            isr::Iicackr,
            isr::Iicackr,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bus busy flag."]
    #[inline(always)]
    pub fn iicbbs(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        isr::Iicbbs,
        isr::Iicbbs,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            isr::Iicbbs,
            isr::Iicbbs,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag"]
    #[inline(always)]
    pub fn iicstif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        isr::Iicstif,
        isr::Iicstif,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            isr::Iicstif,
            isr::Iicstif,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SDA input monitor bit."]
    #[inline(always)]
    pub fn iicsdai(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        isr::Iicsdai,
        isr::Iicsdai,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            isr::Iicsdai,
            isr::Iicsdai,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SCL input monitor bit"]
    #[inline(always)]
    pub fn iicscli(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        isr::Iicscli,
        isr::Iicscli,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            isr::Iicscli,
            isr::Iicscli,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3ffffff, 1, 0, u32, u32, Isr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3ffffff,1,0,u32,u32,Isr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod isr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackr_SPEC;
    pub type Iicackr = crate::EnumBitfieldStruct<u8, Iicackr_SPEC>;
    impl Iicackr {
        #[doc = "ACK received"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicbbs_SPEC;
    pub type Iicbbs = crate::EnumBitfieldStruct<u8, Iicbbs_SPEC>;
    impl Iicbbs {
        #[doc = "Stop condition detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start condition detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstif_SPEC;
    pub type Iicstif = crate::EnumBitfieldStruct<u8, Iicstif_SPEC>;
    impl Iicstif {
        #[doc = "There are no requests for generating conditions or a condition is being generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A start, restart, or stop condition is completely generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdai_SPEC;
    pub type Iicsdai = crate::EnumBitfieldStruct<u8, Iicsdai_SPEC>;
    impl Iicsdai {
        #[doc = "SDA pin state is low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "SDA pin state is high level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscli_SPEC;
    pub type Iicscli = crate::EnumBitfieldStruct<u8, Iicscli_SPEC>;
    impl Iicscli {
        #[doc = "SCL pin state is low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCL pin state is high level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frsr_SPEC;
impl crate::sealed::RegSpec for Frsr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Receive Status Register"]
pub type Frsr = crate::RegValueT<Frsr_SPEC>;

impl Frsr {
    #[doc = "Receive Data Ready flag"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, frsr::Dr, frsr::Dr, Frsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,frsr::Dr,frsr::Dr,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Break detection signal flag"]
    #[inline(always)]
    pub fn brk(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, frsr::Brk, frsr::Brk, Frsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            frsr::Brk,
            frsr::Brk,
            Frsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO Data Count"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity Error Count"]
    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error Count"]
    #[inline(always)]
    pub fn fnum(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frsr {
    #[inline(always)]
    fn default() -> Frsr {
        <crate::RegValueT<Frsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "Receiving is in progress, or no received data has remained in RDR after normally completed receiving.(receive FIFO is empty)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Next receive data has not been received for a fixed period after normal completed receiving"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brk_SPEC;
    pub type Brk = crate::EnumBitfieldStruct<u8, Brk_SPEC>;
    impl Brk {
        #[doc = "No break signal is received."]
        pub const _0: Self = Self::new(0);

        #[doc = "A break signal is received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftsr_SPEC;
impl crate::sealed::RegSpec for Ftsr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Transmit Status Register"]
pub type Ftsr = crate::RegValueT<Ftsr_SPEC>;

impl Ftsr {
    #[doc = "Transmit FIFO Data Count"]
    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Ftsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Ftsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3ffffff, 1, 0, u32, u32, Ftsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3ffffff,1,0,u32,u32,Ftsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ftsr {
    #[inline(always)]
    fn default() -> Ftsr {
        <crate::RegValueT<Ftsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr_SPEC;
impl crate::sealed::RegSpec for Msr_SPEC {
    type DataType = u32;
}

#[doc = "Manchester Status Register"]
pub type Msr = crate::RegValueT<Msr_SPEC>;

impl Msr {
    #[doc = "Preface Error Register"]
    #[inline(always)]
    pub fn pfer(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, msr::Pfer, msr::Pfer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,msr::Pfer,msr::Pfer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Sync Error Registe"]
    #[inline(always)]
    pub fn syer(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, msr::Syer, msr::Syer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,msr::Syer,msr::Syer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Start bit Error Register"]
    #[inline(always)]
    pub fn sber(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, msr::Sber, msr::Sber, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,msr::Sber,msr::Sber,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Manchester error flag"]
    #[inline(always)]
    pub fn mer(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, msr::Mer, msr::Mer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,msr::Mer,msr::Mer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive sync data bit."]
    #[inline(always)]
    pub fn rsync(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        msr::Rsync,
        msr::Rsync,
        Msr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            msr::Rsync,
            msr::Rsync,
            Msr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<7, 0x1ffffff, 1, 0, u32, u32, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1ffffff,1,0,u32,u32,Msr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        <crate::RegValueT<Msr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfer_SPEC;
    pub type Pfer = crate::EnumBitfieldStruct<u8, Pfer_SPEC>;
    impl Pfer {
        #[doc = "No prefetch error detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Prefetch error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syer_SPEC;
    pub type Syer = crate::EnumBitfieldStruct<u8, Syer_SPEC>;
    impl Syer {
        #[doc = "Receive sync error not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive sync error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sber_SPEC;
    pub type Sber = crate::EnumBitfieldStruct<u8, Sber_SPEC>;
    impl Sber {
        #[doc = "No start bit error detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mer_SPEC;
    pub type Mer = crate::EnumBitfieldStruct<u8, Mer_SPEC>;
    impl Mer {
        #[doc = "No Manchester error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A manchester error has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsync_SPEC;
    pub type Rsync = crate::EnumBitfieldStruct<u8, Rsync_SPEC>;
    impl Rsync {
        #[doc = "Start bit receives data Sync"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start bit receives command Sync"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xsr0_SPEC;
impl crate::sealed::RegSpec for Xsr0_SPEC {
    type DataType = u32;
}

#[doc = "Simple-LIN(SCIX) Status Register 0"]
pub type Xsr0 = crate::RegValueT<Xsr0_SPEC>;

impl Xsr0 {
    #[doc = "Start Frame Status Flag"]
    #[inline(always)]
    pub fn sfsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xsr0::Sfsf,
        xsr0::Sfsf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xsr0::Sfsf,
            xsr0::Sfsf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RXD input Status Flag"]
    #[inline(always)]
    pub fn rxdsf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        xsr0::Rxdsf,
        xsr0::Rxdsf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            xsr0::Rxdsf,
            xsr0::Rxdsf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Xsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Break Field Output end Flag"]
    #[inline(always)]
    pub fn bfof(self) -> crate::common::RegisterFieldBool<8, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Bus Collision Detection Flag"]
    #[inline(always)]
    pub fn bcdf(self) -> crate::common::RegisterFieldBool<9, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Break Field Detection Flag"]
    #[inline(always)]
    pub fn bfdf(self) -> crate::common::RegisterFieldBool<10, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Field 0 Match Flag"]
    #[inline(always)]
    pub fn cf0mf(self) -> crate::common::RegisterFieldBool<11, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Field 1 Match Flag"]
    #[inline(always)]
    pub fn cf1mf(self) -> crate::common::RegisterFieldBool<12, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Priority Interrupt Bit Detection Flag"]
    #[inline(always)]
    pub fn pibdf(self) -> crate::common::RegisterFieldBool<13, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Counter Over flow Flag"]
    #[inline(always)]
    pub fn cof(self) -> crate::common::RegisterFieldBool<14, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Active Edge Detection Flag"]
    #[inline(always)]
    pub fn aedf(self) -> crate::common::RegisterFieldBool<15, 1, 0, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Xsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Field 0 Read Data"]
    #[inline(always)]
    pub fn cf0rd(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Xsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Control Field 1 Read Data"]
    #[inline(always)]
    pub fn cf1rd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Xsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xsr0 {
    #[inline(always)]
    fn default() -> Xsr0 {
        <crate::RegValueT<Xsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfsf_SPEC;
    pub type Sfsf = crate::EnumBitfieldStruct<u8, Sfsf_SPEC>;
    impl Sfsf {
        #[doc = "Start frame / Break field detetion disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start frame / Break field detetion enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdsf_SPEC;
    pub type Rxdsf = crate::EnumBitfieldStruct<u8, Rxdsf_SPEC>;
    impl Rxdsf {
        #[doc = "Permit RXD input to core"]
        pub const _0: Self = Self::new(0);

        #[doc = "Prohibit RXD input to core"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xsr1_SPEC;
impl crate::sealed::RegSpec for Xsr1_SPEC {
    type DataType = u32;
}

#[doc = "Simple-LIN(SCIX) Status Register 1"]
pub type Xsr1 = crate::RegValueT<Xsr1_SPEC>;

impl Xsr1 {
    #[doc = "Timer CouNT capture value"]
    #[inline(always)]
    pub fn tcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Xsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Xsr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Xsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Xsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xsr1 {
    #[inline(always)]
    fn default() -> Xsr1 {
        <crate::RegValueT<Xsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfclr_SPEC;
impl crate::sealed::RegSpec for Cfclr_SPEC {
    type DataType = u32;
}

#[doc = "Common Flag Clear Register"]
pub type Cfclr = crate::RegValueT<Cfclr_SPEC>;

impl Cfclr {
    #[doc = "ERS Clear bit"]
    #[inline(always)]
    pub fn ersc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DCMF Clear bit"]
    #[inline(always)]
    pub fn dcmfc(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DPER Clear bit"]
    #[inline(always)]
    pub fn dperc(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DFER Clear bit"]
    #[inline(always)]
    pub fn dferc(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ORER Clear bit"]
    #[inline(always)]
    pub fn orerc(self) -> crate::common::RegisterFieldBool<24, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MFF Clear bit"]
    #[inline(always)]
    pub fn mffc(self) -> crate::common::RegisterFieldBool<26, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PER Clear bit"]
    #[inline(always)]
    pub fn perc(self) -> crate::common::RegisterFieldBool<27, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "FER Clear bit"]
    #[inline(always)]
    pub fn ferc(self) -> crate::common::RegisterFieldBool<28, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "TDRE Clear bit"]
    #[inline(always)]
    pub fn tdrec(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "RDRF Clear bit"]
    #[inline(always)]
    pub fn rdrfc(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cfclr {
    #[inline(always)]
    fn default() -> Cfclr {
        <crate::RegValueT<Cfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfclrHaL_SPEC;
impl crate::sealed::RegSpec for CfclrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Common Flag Clear Register"]
pub type CfclrHaL = crate::RegValueT<CfclrHaL_SPEC>;

impl CfclrHaL {
    #[doc = "ERS Clear bit"]
    #[inline(always)]
    pub fn ersc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, CfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, u16, CfclrHaL_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x7ff,1,0,u16,u16,CfclrHaL_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CfclrHaL {
    #[inline(always)]
    fn default() -> CfclrHaL {
        <crate::RegValueT<CfclrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfclrByLl_SPEC;
impl crate::sealed::RegSpec for CfclrByLl_SPEC {
    type DataType = u8;
}

#[doc = "Common Flag Clear Register"]
pub type CfclrByLl = crate::RegValueT<CfclrByLl_SPEC>;

impl CfclrByLl {
    #[doc = "ERS Clear bit"]
    #[inline(always)]
    pub fn ersc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, CfclrByLl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, CfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,CfclrByLl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CfclrByLl {
    #[inline(always)]
    fn default() -> CfclrByLl {
        <crate::RegValueT<CfclrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfclrHaH_SPEC;
impl crate::sealed::RegSpec for CfclrHaH_SPEC {
    type DataType = u16;
}

#[doc = "Common Flag Clear Register"]
pub type CfclrHaH = crate::RegValueT<CfclrHaH_SPEC>;

impl CfclrHaH {
    #[doc = "DCMF Clear bit"]
    #[inline(always)]
    pub fn dcmfc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DPER Clear bit"]
    #[inline(always)]
    pub fn dperc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DFER Clear bit"]
    #[inline(always)]
    pub fn dferc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ORER Clear bit"]
    #[inline(always)]
    pub fn orerc(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MFF Clear bit"]
    #[inline(always)]
    pub fn mffc(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PER Clear bit"]
    #[inline(always)]
    pub fn perc(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "FER Clear bit"]
    #[inline(always)]
    pub fn ferc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "TDRE Clear bit"]
    #[inline(always)]
    pub fn tdrec(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "RDRF Clear bit"]
    #[inline(always)]
    pub fn rdrfc(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CfclrHaH_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, CfclrHaH_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for CfclrHaH {
    #[inline(always)]
    fn default() -> CfclrHaH {
        <crate::RegValueT<CfclrHaH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfclrByHl_SPEC;
impl crate::sealed::RegSpec for CfclrByHl_SPEC {
    type DataType = u8;
}

#[doc = "Common Flag Clear Register"]
pub type CfclrByHl = crate::RegValueT<CfclrByHl_SPEC>;

impl CfclrByHl {
    #[doc = "DCMF Clear bit"]
    #[inline(always)]
    pub fn dcmfc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CfclrByHl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, CfclrByHl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DPER Clear bit"]
    #[inline(always)]
    pub fn dperc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CfclrByHl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, CfclrByHl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DFER Clear bit"]
    #[inline(always)]
    pub fn dferc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CfclrByHl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, CfclrByHl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, CfclrByHl_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,CfclrByHl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CfclrByHl {
    #[inline(always)]
    fn default() -> CfclrByHl {
        <crate::RegValueT<CfclrByHl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfclrByHh_SPEC;
impl crate::sealed::RegSpec for CfclrByHh_SPEC {
    type DataType = u8;
}

#[doc = "Common Flag Clear Register"]
pub type CfclrByHh = crate::RegValueT<CfclrByHh_SPEC>;

impl CfclrByHh {
    #[doc = "ORER Clear bit"]
    #[inline(always)]
    pub fn orerc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CfclrByHh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, CfclrByHh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MFF Clear bit"]
    #[inline(always)]
    pub fn mffc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CfclrByHh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, CfclrByHh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PER Clear bit"]
    #[inline(always)]
    pub fn perc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CfclrByHh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, CfclrByHh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "FER Clear bit"]
    #[inline(always)]
    pub fn ferc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CfclrByHh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, CfclrByHh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "TDRE Clear bit"]
    #[inline(always)]
    pub fn tdrec(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CfclrByHh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, CfclrByHh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CfclrByHh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, CfclrByHh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "RDRF Clear bit"]
    #[inline(always)]
    pub fn rdrfc(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CfclrByHh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, CfclrByHh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for CfclrByHh {
    #[inline(always)]
    fn default() -> CfclrByHh {
        <crate::RegValueT<CfclrByHh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icfclr_SPEC;
impl crate::sealed::RegSpec for Icfclr_SPEC {
    type DataType = u32;
}

#[doc = "Simple-I2C Flag Clear Register"]
pub type Icfclr = crate::RegValueT<Icfclr_SPEC>;

impl Icfclr {
    #[doc = "IICBBS Clear bit"]
    #[inline(always)]
    pub fn iicbbsc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Icfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "IICSTIF Clear bit"]
    #[inline(always)]
    pub fn iicstifc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 0000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Icfclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0xfffffff,1,0,u32,u32,Icfclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Icfclr {
    #[inline(always)]
    fn default() -> Icfclr {
        <crate::RegValueT<Icfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcfclrHaL_SPEC;
impl crate::sealed::RegSpec for IcfclrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Simple-I2C Flag Clear Register"]
pub type IcfclrHaL = crate::RegValueT<IcfclrHaL_SPEC>;

impl IcfclrHaL {
    #[doc = "IICBBSC Clear bit"]
    #[inline(always)]
    pub fn iicbbsc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IcfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, IcfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "IICSTIF Clear bit"]
    #[inline(always)]
    pub fn iicstifc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IcfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, IcfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, IcfclrHaL_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,IcfclrHaL_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for IcfclrHaL {
    #[inline(always)]
    fn default() -> IcfclrHaL {
        <crate::RegValueT<IcfclrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IcfclrByLl_SPEC;
impl crate::sealed::RegSpec for IcfclrByLl_SPEC {
    type DataType = u8;
}

#[doc = "Simple-I2C Flag Clear Register"]
pub type IcfclrByLl = crate::RegValueT<IcfclrByLl_SPEC>;

impl IcfclrByLl {
    #[doc = "IICBBSC Clear bit"]
    #[inline(always)]
    pub fn iicbbsc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IcfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,IcfclrByLl_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "IICSTIF Clear bit"]
    #[inline(always)]
    pub fn iicstifc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IcfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,IcfclrByLl_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, IcfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,IcfclrByLl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for IcfclrByLl {
    #[inline(always)]
    fn default() -> IcfclrByLl {
        <crate::RegValueT<IcfclrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffclr_SPEC;
impl crate::sealed::RegSpec for Ffclr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Flag Clear Register"]
pub type Ffclr = crate::RegValueT<Ffclr_SPEC>;

impl Ffclr {
    #[doc = "DR Clear bit"]
    #[inline(always)]
    pub fn drc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ffclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ffclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BRK Clear bit"]
    #[inline(always)]
    pub fn brkc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ffclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ffclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, u32, Ffclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x3fffffff,1,0,u32,u32,Ffclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ffclr {
    #[inline(always)]
    fn default() -> Ffclr {
        <crate::RegValueT<Ffclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FfclrHaL_SPEC;
impl crate::sealed::RegSpec for FfclrHaL_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Flag Clear Register"]
pub type FfclrHaL = crate::RegValueT<FfclrHaL_SPEC>;

impl FfclrHaL {
    #[doc = "DR Clear bit"]
    #[inline(always)]
    pub fn drc(self) -> crate::common::RegisterFieldBool<0, 1, 0, FfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, FfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BRK Clear bit"]
    #[inline(always)]
    pub fn brkc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, FfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 00000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, u16, FfclrHaL_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16,u16,FfclrHaL_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FfclrHaL {
    #[inline(always)]
    fn default() -> FfclrHaL {
        <crate::RegValueT<FfclrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FfclrByLl_SPEC;
impl crate::sealed::RegSpec for FfclrByLl_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Flag Clear Register"]
pub type FfclrByLl = crate::RegValueT<FfclrByLl_SPEC>;

impl FfclrByLl {
    #[doc = "DR Clear bit"]
    #[inline(always)]
    pub fn drc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, FfclrByLl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BRK Clear bit"]
    #[inline(always)]
    pub fn brkc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, FfclrByLl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, FfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,FfclrByLl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FfclrByLl {
    #[inline(always)]
    fn default() -> FfclrByLl {
        <crate::RegValueT<FfclrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfclr_SPEC;
impl crate::sealed::RegSpec for Mfclr_SPEC {
    type DataType = u32;
}

#[doc = "Manchester Flag Clear Register"]
pub type Mfclr = crate::RegValueT<Mfclr_SPEC>;

impl Mfclr {
    #[doc = "PFER Clear bit"]
    #[inline(always)]
    pub fn pferc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SYER Clear bit"]
    #[inline(always)]
    pub fn syerc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SBER Clear bit"]
    #[inline(always)]
    pub fn sberc(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MER Clear bit"]
    #[inline(always)]
    pub fn merc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Mfclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,Mfclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mfclr {
    #[inline(always)]
    fn default() -> Mfclr {
        <crate::RegValueT<Mfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MfclrHaL_SPEC;
impl crate::sealed::RegSpec for MfclrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Manchester Flag Clear Register"]
pub type MfclrHaL = crate::RegValueT<MfclrHaL_SPEC>;

impl MfclrHaL {
    #[doc = "PFER Clear bit"]
    #[inline(always)]
    pub fn pferc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, MfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SYER Clear bit"]
    #[inline(always)]
    pub fn syerc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, MfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, MfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SBER Clear bit"]
    #[inline(always)]
    pub fn sberc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, MfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, MfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MER Clear bit"]
    #[inline(always)]
    pub fn merc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, MfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, MfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, u16, MfclrHaL_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x7ff,1,0,u16,u16,MfclrHaL_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for MfclrHaL {
    #[inline(always)]
    fn default() -> MfclrHaL {
        <crate::RegValueT<MfclrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MfclrByLl_SPEC;
impl crate::sealed::RegSpec for MfclrByLl_SPEC {
    type DataType = u8;
}

#[doc = "Manchester Flag Clear Register"]
pub type MfclrByLl = crate::RegValueT<MfclrByLl_SPEC>;

impl MfclrByLl {
    #[doc = "PFER Clear bit"]
    #[inline(always)]
    pub fn pferc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, MfclrByLl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SYER Clear bit"]
    #[inline(always)]
    pub fn syerc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, MfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, MfclrByLl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "SBER Clear bit"]
    #[inline(always)]
    pub fn sberc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, MfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, MfclrByLl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MER Clear bit"]
    #[inline(always)]
    pub fn merc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, MfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, MfclrByLl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, MfclrByLl_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,MfclrByLl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for MfclrByLl {
    #[inline(always)]
    fn default() -> MfclrByLl {
        <crate::RegValueT<MfclrByLl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xfclr_SPEC;
impl crate::sealed::RegSpec for Xfclr_SPEC {
    type DataType = u32;
}

#[doc = "Simpe-LIN(SCIX) Flag Clear Register"]
pub type Xfclr = crate::RegValueT<Xfclr_SPEC>;

impl Xfclr {
    #[doc = "BFO Clear bit"]
    #[inline(always)]
    pub fn bfoc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BCD Clear bit"]
    #[inline(always)]
    pub fn bcdc(self) -> crate::common::RegisterFieldBool<9, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BFD Clear bit"]
    #[inline(always)]
    pub fn bfdc(self) -> crate::common::RegisterFieldBool<10, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF0M Clear bit"]
    #[inline(always)]
    pub fn cf0mc(self) -> crate::common::RegisterFieldBool<11, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF1M Clear bit"]
    #[inline(always)]
    pub fn cf1mc(self) -> crate::common::RegisterFieldBool<12, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PIBD Clear bit"]
    #[inline(always)]
    pub fn pibdc(self) -> crate::common::RegisterFieldBool<13, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "COF Clear bit"]
    #[inline(always)]
    pub fn cofc(self) -> crate::common::RegisterFieldBool<14, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "AED Clear bit"]
    #[inline(always)]
    pub fn aedc(self) -> crate::common::RegisterFieldBool<15, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Xfclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Xfclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Xfclr {
    #[inline(always)]
    fn default() -> Xfclr {
        <crate::RegValueT<Xfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XfclrHaL_SPEC;
impl crate::sealed::RegSpec for XfclrHaL_SPEC {
    type DataType = u16;
}

#[doc = "Simpe-LIN(SCIX) Flag Clear Register"]
pub type XfclrHaL = crate::RegValueT<XfclrHaL_SPEC>;

impl XfclrHaL {
    #[doc = "The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,XfclrHaL_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "BFO Clear bit"]
    #[inline(always)]
    pub fn bfoc(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BCD Clear bit"]
    #[inline(always)]
    pub fn bcdc(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BFD Clear bit"]
    #[inline(always)]
    pub fn bfdc(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF0M Clear bit"]
    #[inline(always)]
    pub fn cf0mc(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF1M Clear bit"]
    #[inline(always)]
    pub fn cf1mc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PIBD Clear bit"]
    #[inline(always)]
    pub fn pibdc(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "COF Clear bit"]
    #[inline(always)]
    pub fn cofc(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "AED Clear bit"]
    #[inline(always)]
    pub fn aedc(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, XfclrHaL_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, XfclrHaL_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for XfclrHaL {
    #[inline(always)]
    fn default() -> XfclrHaL {
        <crate::RegValueT<XfclrHaL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XfclrByLh_SPEC;
impl crate::sealed::RegSpec for XfclrByLh_SPEC {
    type DataType = u8;
}

#[doc = "Simpe-LIN(SCIX) Flag Clear Register"]
pub type XfclrByLh = crate::RegValueT<XfclrByLh_SPEC>;

impl XfclrByLh {
    #[doc = "BFO Clear bit"]
    #[inline(always)]
    pub fn bfoc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BCD Clear bit"]
    #[inline(always)]
    pub fn bcdc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BFD Clear bit"]
    #[inline(always)]
    pub fn bfdc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF0M Clear bit"]
    #[inline(always)]
    pub fn cf0mc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CF1M Clear bit"]
    #[inline(always)]
    pub fn cf1mc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PIBD Clear bit"]
    #[inline(always)]
    pub fn pibdc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "COF Clear bit"]
    #[inline(always)]
    pub fn cofc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "AED Clear bit"]
    #[inline(always)]
    pub fn aedc(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, XfclrByLh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, XfclrByLh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for XfclrByLh {
    #[inline(always)]
    fn default() -> XfclrByLh {
        <crate::RegValueT<XfclrByLh_SPEC> as RegisterValue<_>>::new(0)
    }
}
