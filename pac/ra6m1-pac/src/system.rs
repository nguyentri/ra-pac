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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:01 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"System Control"]
unsafe impl ::core::marker::Send for super::System {}
unsafe impl ::core::marker::Sync for super::System {}
impl super::System {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Flash P/E Protect Register"]
    #[inline(always)]
    pub const fn fwepror(
        &self,
    ) -> &'static crate::common::Reg<self::Fwepror_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwepror_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1046usize),
            )
        }
    }

    #[doc = "VBATT Input Control Register"]
    #[inline(always)]
    pub const fn vbtictlr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtictlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtictlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1211usize),
            )
        }
    }

    #[doc = "VBATT Backup Register \\[%s\\]"]
    #[inline(always)]
    pub const fn vbtbkr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW>,
        512,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x500usize))
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_0_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x500usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_1_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x501usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_2_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x502usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_3_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x503usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_4_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x504usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_5_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x505usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_6_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x506usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_7_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x507usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_8_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_9_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x509usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_10_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_11_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_12_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_13_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_14_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_15_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_16_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_17_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x511usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_18_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x512usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_19_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x513usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_20_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x514usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_21_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x515usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_22_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x516usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_23_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x517usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_24_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x518usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_25_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x519usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_26_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_27_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_28_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_29_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_30_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_31_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_32_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x520usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_33_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x521usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_34_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x522usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_35_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x523usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_36_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x524usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_37_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x525usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_38_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x526usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_39_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x527usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_40_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x528usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_41_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x529usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_42_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_43_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_44_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_45_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_46_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_47_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_48_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x530usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_49_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x531usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_50_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x532usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_51_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x533usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_52_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x534usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_53_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x535usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_54_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x536usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_55_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x537usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_56_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x538usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_57_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x539usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_58_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_59_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_60_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_61_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_62_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_63_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_64_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x540usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_65_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x541usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_66_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x542usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_67_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x543usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_68_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x544usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_69_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x545usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_70_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x546usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_71_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x547usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_72_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x548usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_73_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x549usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_74_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_75_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_76_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_77_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_78_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_79_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_80_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x550usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_81_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x551usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_82_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x552usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_83_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x553usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_84_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x554usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_85_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x555usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_86_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x556usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_87_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x557usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_88_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x558usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_89_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x559usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_90_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_91_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_92_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_93_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_94_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_95_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x55fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_96_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x560usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_97_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x561usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_98_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x562usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_99_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x563usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_100_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x564usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_101_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x565usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_102_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x566usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_103_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x567usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_104_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x568usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_105_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x569usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_106_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_107_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_108_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_109_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_110_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_111_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_112_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x570usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_113_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x571usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_114_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x572usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_115_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x573usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_116_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x574usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_117_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x575usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_118_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x576usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_119_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x577usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_120_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x578usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_121_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x579usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_122_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_123_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_124_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_125_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_126_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_127_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_128_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x580usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_129_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x581usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_130_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x582usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_131_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x583usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_132_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x584usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_133_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x585usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_134_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x586usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_135_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x587usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_136_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x588usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_137_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x589usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_138_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_139_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_140_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_141_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_142_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_143_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_144_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x590usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_145_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x591usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_146_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x592usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_147_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x593usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_148_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x594usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_149_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x595usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_150_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x596usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_151_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x597usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_152_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x598usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_153_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x599usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_154_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_155_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_156_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_157_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_158_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_159_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_160_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_161_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_162_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_163_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_164_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_165_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_166_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_167_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_168_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_169_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_170_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5aausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_171_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5abusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_172_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_173_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5adusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_174_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5aeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_175_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5afusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_176_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_177_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_178_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_179_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_180_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_181_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_182_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_183_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_184_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_185_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_186_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5bausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_187_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5bbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_188_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_189_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5bdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_190_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5beusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_191_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5bfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_192_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_193_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_194_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_195_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_196_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_197_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_198_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_199_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_200_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_201_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_202_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5causize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_203_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_204_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_205_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_206_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ceusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_207_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_208_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_209_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_210_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_211_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_212_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_213_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_214_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_215_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_216_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_217_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_218_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5dausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_219_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5dbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_220_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_221_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ddusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_222_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5deusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_223_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5dfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_224_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_225_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_226_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_227_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_228_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_229_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_230_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_231_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_232_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_233_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_234_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5eausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_235_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ebusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_236_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_237_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5edusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_238_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5eeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_239_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5efusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_240_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_241_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_242_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_243_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_244_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_245_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_246_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_247_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_248_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_249_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_250_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5fausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_251_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5fbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_252_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_253_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5fdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_254_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5feusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_255_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ffusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_256_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_257_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x601usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_258_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x602usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_259_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x603usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_260_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_261_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x605usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_262_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x606usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_263_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x607usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_264_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_265_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x609usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_266_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_267_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_268_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_269_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_270_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_271_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_272_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_273_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x611usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_274_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x612usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_275_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x613usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_276_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_277_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x615usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_278_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x616usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_279_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x617usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_280_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x618usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_281_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x619usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_282_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_283_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_284_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_285_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_286_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_287_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_288_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_289_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x621usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_290_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x622usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_291_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x623usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_292_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_293_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x625usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_294_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x626usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_295_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x627usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_296_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_297_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x629usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_298_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_299_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_300_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_301_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_302_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_303_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_304_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_305_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x631usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_306_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x632usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_307_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x633usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_308_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_309_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x635usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_310_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x636usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_311_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x637usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_312_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x638usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_313_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x639usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_314_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_315_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_316_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_317_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_318_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_319_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x63fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_320_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x640usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_321_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x641usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_322_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x642usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_323_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x643usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_324_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x644usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_325_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x645usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_326_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x646usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_327_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x647usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_328_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x648usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_329_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x649usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_330_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_331_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_332_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_333_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_334_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_335_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_336_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x650usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_337_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x651usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_338_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x652usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_339_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x653usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_340_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x654usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_341_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x655usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_342_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x656usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_343_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x657usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_344_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x658usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_345_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x659usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_346_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_347_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_348_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_349_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_350_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_351_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x65fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_352_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x660usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_353_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x661usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_354_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x662usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_355_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x663usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_356_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x664usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_357_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x665usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_358_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x666usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_359_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x667usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_360_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x668usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_361_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x669usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_362_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_363_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_364_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_365_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_366_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_367_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x66fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_368_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x670usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_369_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x671usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_370_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x672usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_371_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x673usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_372_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x674usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_373_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x675usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_374_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x676usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_375_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x677usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_376_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x678usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_377_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x679usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_378_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_379_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_380_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_381_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_382_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_383_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x67fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_384_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x680usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_385_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x681usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_386_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x682usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_387_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x683usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_388_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x684usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_389_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x685usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_390_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x686usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_391_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x687usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_392_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x688usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_393_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x689usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_394_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_395_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_396_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_397_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_398_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_399_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_400_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x690usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_401_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x691usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_402_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x692usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_403_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x693usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_404_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x694usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_405_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x695usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_406_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x696usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_407_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x697usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_408_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x698usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_409_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x699usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_410_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_411_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69busize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_412_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_413_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_414_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_415_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x69fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_416_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_417_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_418_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_419_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_420_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_421_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_422_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_423_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_424_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_425_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_426_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6aausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_427_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6abusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_428_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_429_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6adusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_430_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6aeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_431_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6afusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_432_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_433_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_434_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_435_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_436_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_437_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_438_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_439_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_440_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_441_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_442_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_443_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_444_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_445_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_446_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6beusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_447_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_448_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_449_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_450_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_451_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_452_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_453_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_454_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_455_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_456_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_457_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_458_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6causize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_459_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_460_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_461_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_462_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ceusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_463_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_464_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_465_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_466_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_467_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_468_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_469_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_470_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_471_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_472_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_473_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_474_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_475_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_476_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_477_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ddusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_478_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6deusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_479_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dfusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_480_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_481_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_482_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_483_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_484_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_485_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_486_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_487_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_488_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_489_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6e9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_490_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6eausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_491_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ebusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_492_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_493_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6edusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_494_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6eeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_495_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6efusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_496_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_497_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_498_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_499_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_500_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_501_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_502_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_503_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f7usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_504_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_505_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6f9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_506_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6fausize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_507_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6fbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_508_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_509_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6fdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_510_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6feusize),
            )
        }
    }
    #[inline(always)]
    pub const fn vbtbkr_511_(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtbkr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ffusize),
            )
        }
    }

    #[doc = "System Clock Division Control Register"]
    #[inline(always)]
    pub const fn sckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "System Clock Division Control Register 2"]
    #[inline(always)]
    pub const fn sckdivcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Sckdivcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckdivcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "System Clock Source Control Register"]
    #[inline(always)]
    pub const fn sckscr(
        &self,
    ) -> &'static crate::common::Reg<self::Sckscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "PLL Clock Control Register"]
    #[inline(always)]
    pub const fn pllccr(
        &self,
    ) -> &'static crate::common::Reg<self::Pllccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pllccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "PLL Control Register"]
    #[inline(always)]
    pub const fn pllcr(&self) -> &'static crate::common::Reg<self::Pllcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pllcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "External Bus Clock Control Register"]
    #[inline(always)]
    pub const fn bckcr(&self) -> &'static crate::common::Reg<self::Bckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn mosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Mosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "High-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn hococr(
        &self,
    ) -> &'static crate::common::Reg<self::Hococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "Middle-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn mococr(
        &self,
    ) -> &'static crate::common::Reg<self::Mococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "FLL Control Register 1"]
    #[inline(always)]
    pub const fn fllcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fllcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fllcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(57usize),
            )
        }
    }

    #[doc = "FLL Control Register 2"]
    #[inline(always)]
    pub const fn fllcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fllcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fllcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[doc = "Oscillation Stabilization Flag Register"]
    #[inline(always)]
    pub const fn oscsf(&self) -> &'static crate::common::Reg<self::Oscsf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Oscsf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Clock Out Control Register"]
    #[inline(always)]
    pub const fn ckocr(&self) -> &'static crate::common::Reg<self::Ckocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ckocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Trace Clock Control Register"]
    #[inline(always)]
    pub const fn trckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Trckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(63usize),
            )
        }
    }

    #[doc = "Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn ostdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ostdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ostdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Oscillation Stop Detection Status Register"]
    #[inline(always)]
    pub const fn ostdsr(
        &self,
    ) -> &'static crate::common::Reg<self::Ostdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ostdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[doc = "External Bus Clock Output Control Register"]
    #[inline(always)]
    pub const fn ebckocr(
        &self,
    ) -> &'static crate::common::Reg<self::Ebckocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ebckocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[doc = "SDRAM Clock Output Control Register"]
    #[inline(always)]
    pub const fn sdckocr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdckocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdckocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(83usize),
            )
        }
    }

    #[doc = "MOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn mocoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Mocoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mocoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(97usize),
            )
        }
    }

    #[doc = "HOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn hocoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
    #[inline(always)]
    pub const fn momcr(&self) -> &'static crate::common::Reg<self::Momcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Momcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1043usize),
            )
        }
    }

    #[doc = "Sub-clock oscillator control register"]
    #[inline(always)]
    pub const fn sosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1152usize),
            )
        }
    }

    #[doc = "Sub Clock Oscillator Mode Control Register"]
    #[inline(always)]
    pub const fn somcr(&self) -> &'static crate::common::Reg<self::Somcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Somcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1153usize),
            )
        }
    }

    #[doc = "Low-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn lococr(
        &self,
    ) -> &'static crate::common::Reg<self::Lococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1168usize),
            )
        }
    }

    #[doc = "LOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn locoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Locoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Locoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1170usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn moscwtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Moscwtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Moscwtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(162usize),
            )
        }
    }

    #[doc = "High-speed on-chip oscillator wait control register"]
    #[inline(always)]
    pub const fn hocowtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocowtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocowtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(165usize),
            )
        }
    }

    #[doc = "Standby Control Register"]
    #[inline(always)]
    pub const fn sbycr(&self) -> &'static crate::common::Reg<self::Sbycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sbycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Module Stop Control Register A"]
    #[inline(always)]
    pub const fn mstpcra(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Snooze Control Register"]
    #[inline(always)]
    pub const fn snzcr(&self) -> &'static crate::common::Reg<self::Snzcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(146usize),
            )
        }
    }

    #[doc = "Snooze End Control Register"]
    #[inline(always)]
    pub const fn snzedcr(
        &self,
    ) -> &'static crate::common::Reg<self::Snzedcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzedcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Snooze Request Control Register"]
    #[inline(always)]
    pub const fn snzreqcr(
        &self,
    ) -> &'static crate::common::Reg<self::Snzreqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzreqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Operating Power Control Register"]
    #[inline(always)]
    pub const fn opccr(&self) -> &'static crate::common::Reg<self::Opccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Opccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Sub Operating Power Control Register"]
    #[inline(always)]
    pub const fn sopccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sopccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sopccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(170usize),
            )
        }
    }

    #[doc = "Deep Standby Control Register"]
    #[inline(always)]
    pub const fn dpsbycr(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsbycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsbycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn dpsier0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1026usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn dpsier1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1027usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn dpsier2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Enable Register 3"]
    #[inline(always)]
    pub const fn dpsier3(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsier3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsier3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1029usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 0"]
    #[inline(always)]
    pub const fn dpsifr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1030usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 1"]
    #[inline(always)]
    pub const fn dpsifr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1031usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 2"]
    #[inline(always)]
    pub const fn dpsifr2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1032usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Flag Register 3"]
    #[inline(always)]
    pub const fn dpsifr3(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsifr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsifr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1033usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Edge Register 0"]
    #[inline(always)]
    pub const fn dpsiegr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1034usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Edge Register 1"]
    #[inline(always)]
    pub const fn dpsiegr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1035usize),
            )
        }
    }

    #[doc = "Deep Standby Interrupt Edge Register 2"]
    #[inline(always)]
    pub const fn dpsiegr2(
        &self,
    ) -> &'static crate::common::Reg<self::Dpsiegr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpsiegr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1036usize),
            )
        }
    }

    #[doc = "System Control OCD Control Register"]
    #[inline(always)]
    pub const fn syocdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Syocdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syocdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1038usize),
            )
        }
    }

    #[doc = "Standby Condition Register"]
    #[inline(always)]
    pub const fn stconr(
        &self,
    ) -> &'static crate::common::Reg<self::Stconr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stconr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1039usize),
            )
        }
    }

    #[doc = "Voltage Monitor %s Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvdcr1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lvdcr1_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }
    #[inline(always)]
    pub const fn lvd1cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn lvd2cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe2usize),
            )
        }
    }

    #[doc = "Voltage Monitor %s Circuit Status Register"]
    #[inline(always)]
    pub const fn lvdsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lvdsr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe1usize))
        }
    }
    #[inline(always)]
    pub const fn lvd1sr(&self) -> &'static crate::common::Reg<self::Lvdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn lvd2sr(&self) -> &'static crate::common::Reg<self::Lvdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe3usize),
            )
        }
    }

    #[doc = "Voltage Monitor Circuit Control Register"]
    #[inline(always)]
    pub const fn lvcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1047usize),
            )
        }
    }

    #[doc = "Voltage Detection Level Select Register"]
    #[inline(always)]
    pub const fn lvdlvlr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdlvlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdlvlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[doc = "Voltage Monitor %s Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvdcr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lvdcr0_SPEC, crate::common::RW>,
        2,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x41ausize))
        }
    }
    #[inline(always)]
    pub const fn lvd1cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x41ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn lvd2cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x41busize),
            )
        }
    }

    #[doc = "Protect Register"]
    #[inline(always)]
    pub const fn prcr(&self) -> &'static crate::common::Reg<self::Prcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1022usize),
            )
        }
    }

    #[doc = "Reset Status Register 0"]
    #[inline(always)]
    pub const fn rstsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Reset Status Register 2"]
    #[inline(always)]
    pub const fn rstsr2(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1041usize),
            )
        }
    }

    #[doc = "Reset Status Register 1"]
    #[inline(always)]
    pub const fn rstsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwepror_SPEC;
impl crate::sealed::RegSpec for Fwepror_SPEC {
    type DataType = u8;
}

#[doc = "Flash P/E Protect Register"]
pub type Fwepror = crate::RegValueT<Fwepror_SPEC>;

impl Fwepror {
    #[doc = "Flash Programming and Erasure"]
    #[inline(always)]
    pub fn flwe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fwepror::Flwe,
        fwepror::Flwe,
        Fwepror_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fwepror::Flwe,
            fwepror::Flwe,
            Fwepror_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwepror {
    #[inline(always)]
    fn default() -> Fwepror {
        <crate::RegValueT<Fwepror_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod fwepror {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flwe_SPEC;
    pub type Flwe = crate::EnumBitfieldStruct<u8, Flwe_SPEC>;
    impl Flwe {
        #[doc = "Prohibits programming and erasure of the code flash, data flash or blank checking."]
        pub const _00: Self = Self::new(0);

        #[doc = "Permits programming and erasure of the code flash, data flash or blank checking."]
        pub const _01: Self = Self::new(1);

        #[doc = "Prohibits programming and erasure of the code flash, data flash or blank checking."]
        pub const _10: Self = Self::new(2);

        #[doc = "Prohibits programming and erasure of the code flash, data flash or blank checking."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtictlr_SPEC;
impl crate::sealed::RegSpec for Vbtictlr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Input Control Register"]
pub type Vbtictlr = crate::RegValueT<Vbtictlr_SPEC>;

impl Vbtictlr {
    #[doc = "RTCIC2 Input Enable"]
    #[inline(always)]
    pub fn vch2inen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtictlr::Vch2Inen,
        vbtictlr::Vch2Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtictlr::Vch2Inen,
            vbtictlr::Vch2Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTCIC1 Input Enable"]
    #[inline(always)]
    pub fn vch1inen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtictlr::Vch1Inen,
        vbtictlr::Vch1Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtictlr::Vch1Inen,
            vbtictlr::Vch1Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTCIC0 Input Enable"]
    #[inline(always)]
    pub fn vch0inen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtictlr::Vch0Inen,
        vbtictlr::Vch0Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtictlr::Vch0Inen,
            vbtictlr::Vch0Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtictlr {
    #[inline(always)]
    fn default() -> Vbtictlr {
        <crate::RegValueT<Vbtictlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtictlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Inen_SPEC;
    pub type Vch2Inen = crate::EnumBitfieldStruct<u8, Vch2Inen_SPEC>;
    impl Vch2Inen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Inen_SPEC;
    pub type Vch1Inen = crate::EnumBitfieldStruct<u8, Vch1Inen_SPEC>;
    impl Vch1Inen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Inen_SPEC;
    pub type Vch0Inen = crate::EnumBitfieldStruct<u8, Vch0Inen_SPEC>;
    impl Vch0Inen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbkr_SPEC;
impl crate::sealed::RegSpec for Vbtbkr_SPEC {
    type DataType = u8;
}

#[doc = "VBATT Backup Register \\[%s\\]"]
pub type Vbtbkr = crate::RegValueT<Vbtbkr_SPEC>;

impl Vbtbkr {
    #[doc = "VBTBKR is a 512-byte readable/writable register to store data powered by VBATT.The value of this register is retained even when VCC is not powered but VBATT is powered.VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    pub fn vbtbkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Vbtbkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Vbtbkr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtbkr {
    #[inline(always)]
    fn default() -> Vbtbkr {
        <crate::RegValueT<Vbtbkr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckdivcr_SPEC;
impl crate::sealed::RegSpec for Sckdivcr_SPEC {
    type DataType = u32;
}

#[doc = "System Clock Division Control Register"]
pub type Sckdivcr = crate::RegValueT<Sckdivcr_SPEC>;

impl Sckdivcr {
    #[doc = "Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    pub fn fck(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        sckdivcr::Fck,
        sckdivcr::Fck,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            sckdivcr::Fck,
            sckdivcr::Fck,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        sckdivcr::Ick,
        sckdivcr::Ick,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            sckdivcr::Ick,
            sckdivcr::Ick,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Bus Clock (BCLK) Select"]
    #[inline(always)]
    pub fn bck(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        sckdivcr::Bck,
        sckdivcr::Bck,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            sckdivcr::Bck,
            sckdivcr::Bck,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        sckdivcr::Pcka,
        sckdivcr::Pcka,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            sckdivcr::Pcka,
            sckdivcr::Pcka,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        sckdivcr::Pckb,
        sckdivcr::Pckb,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            sckdivcr::Pckb,
            sckdivcr::Pckb,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    pub fn pckc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        sckdivcr::Pckc,
        sckdivcr::Pckc,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            sckdivcr::Pckc,
            sckdivcr::Pckc,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub fn pckd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sckdivcr::Pckd,
        sckdivcr::Pckd,
        Sckdivcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sckdivcr::Pckd,
            sckdivcr::Pckd,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sckdivcr {
    #[inline(always)]
    fn default() -> Sckdivcr {
        <crate::RegValueT<Sckdivcr_SPEC> as RegisterValue<_>>::new(570565154)
    }
}
pub mod sckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fck_SPEC;
    pub type Fck = crate::EnumBitfieldStruct<u8, Fck_SPEC>;
    impl Fck {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ick_SPEC;
    pub type Ick = crate::EnumBitfieldStruct<u8, Ick_SPEC>;
    impl Ick {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bck_SPEC;
    pub type Bck = crate::EnumBitfieldStruct<u8, Bck_SPEC>;
    impl Bck {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcka_SPEC;
    pub type Pcka = crate::EnumBitfieldStruct<u8, Pcka_SPEC>;
    impl Pcka {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckb_SPEC;
    pub type Pckb = crate::EnumBitfieldStruct<u8, Pckb_SPEC>;
    impl Pckb {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckc_SPEC;
    pub type Pckc = crate::EnumBitfieldStruct<u8, Pckc_SPEC>;
    impl Pckc {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckd_SPEC;
    pub type Pckd = crate::EnumBitfieldStruct<u8, Pckd_SPEC>;
    impl Pckd {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckdivcr2_SPEC;
impl crate::sealed::RegSpec for Sckdivcr2_SPEC {
    type DataType = u8;
}

#[doc = "System Clock Division Control Register 2"]
pub type Sckdivcr2 = crate::RegValueT<Sckdivcr2_SPEC>;

impl Sckdivcr2 {
    #[doc = "USB Clock (UCLK) Select"]
    #[inline(always)]
    pub fn uck(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        sckdivcr2::Uck,
        sckdivcr2::Uck,
        Sckdivcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            sckdivcr2::Uck,
            sckdivcr2::Uck,
            Sckdivcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sckdivcr2 {
    #[inline(always)]
    fn default() -> Sckdivcr2 {
        <crate::RegValueT<Sckdivcr2_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod sckdivcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uck_SPEC;
    pub type Uck = crate::EnumBitfieldStruct<u8, Uck_SPEC>;
    impl Uck {
        #[doc = "/3"]
        pub const _010: Self = Self::new(2);

        #[doc = "/4"]
        pub const _011: Self = Self::new(3);

        #[doc = "/5"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckscr_SPEC;
impl crate::sealed::RegSpec for Sckscr_SPEC {
    type DataType = u8;
}

#[doc = "System Clock Source Control Register"]
pub type Sckscr = crate::RegValueT<Sckscr_SPEC>;

impl Sckscr {
    #[doc = "Clock Source Select"]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sckscr::Cksel,
        sckscr::Cksel,
        Sckscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sckscr::Cksel,
            sckscr::Cksel,
            Sckscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sckscr {
    #[inline(always)]
    fn default() -> Sckscr {
        <crate::RegValueT<Sckscr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sckscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _000: Self = Self::new(0);

        #[doc = "MOCO"]
        pub const _001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);

        #[doc = "Main clock oscillator"]
        pub const _011: Self = Self::new(3);

        #[doc = "Sub-clock oscillator"]
        pub const _100: Self = Self::new(4);

        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllccr_SPEC;
impl crate::sealed::RegSpec for Pllccr_SPEC {
    type DataType = u16;
}

#[doc = "PLL Clock Control Register"]
pub type Pllccr = crate::RegValueT<Pllccr_SPEC>;

impl Pllccr {
    #[doc = "PLL Frequency Multiplication Factor Select   \\[PLL Frequency Multiplication Factor\\] = (PLLUMUL+1) / 2   Range: 0x23 - 0x3B   for example       010011: x10.0       010100: x10.5       010101: x11.0          :       011100: x14.5       011101: x15.0       011110: x15.5          :       111010: x29.5       111011: x30.0"]
    #[inline(always)]
    pub fn pllmul(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        pllccr::Pllmul,
        pllccr::Pllmul,
        Pllccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            pllccr::Pllmul,
            pllccr::Pllmul,
            Pllccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL Clock Source Select"]
    #[inline(always)]
    pub fn plsrcsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pllccr::Plsrcsel,
        pllccr::Plsrcsel,
        Pllccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pllccr::Plsrcsel,
            pllccr::Plsrcsel,
            Pllccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL Input Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn plidiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        pllccr::Plidiv,
        pllccr::Plidiv,
        Pllccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            pllccr::Plidiv,
            pllccr::Plidiv,
            Pllccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pllccr {
    #[inline(always)]
    fn default() -> Pllccr {
        <crate::RegValueT<Pllccr_SPEC> as RegisterValue<_>>::new(4864)
    }
}
pub mod pllccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllmul_SPEC;
    pub type Pllmul = crate::EnumBitfieldStruct<u8, Pllmul_SPEC>;
    impl Pllmul {
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plsrcsel_SPEC;
    pub type Plsrcsel = crate::EnumBitfieldStruct<u8, Plsrcsel_SPEC>;
    impl Plsrcsel {
        #[doc = "Main clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plidiv_SPEC;
    pub type Plidiv = crate::EnumBitfieldStruct<u8, Plidiv_SPEC>;
    impl Plidiv {
        #[doc = "/1"]
        pub const _00: Self = Self::new(0);

        #[doc = "/2"]
        pub const _01: Self = Self::new(1);

        #[doc = "/3"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllcr_SPEC;
impl crate::sealed::RegSpec for Pllcr_SPEC {
    type DataType = u8;
}

#[doc = "PLL Control Register"]
pub type Pllcr = crate::RegValueT<Pllcr_SPEC>;

impl Pllcr {
    #[doc = "PLL Stop Control"]
    #[inline(always)]
    pub fn pllstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pllcr::Pllstp,
        pllcr::Pllstp,
        Pllcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pllcr::Pllstp,
            pllcr::Pllstp,
            Pllcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pllcr {
    #[inline(always)]
    fn default() -> Pllcr {
        <crate::RegValueT<Pllcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod pllcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllstp_SPEC;
    pub type Pllstp = crate::EnumBitfieldStruct<u8, Pllstp_SPEC>;
    impl Pllstp {
        #[doc = "Operate the PLL"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the PLL."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bckcr_SPEC;
impl crate::sealed::RegSpec for Bckcr_SPEC {
    type DataType = u8;
}

#[doc = "External Bus Clock Control Register"]
pub type Bckcr = crate::RegValueT<Bckcr_SPEC>;

impl Bckcr {
    #[doc = "BCLK Pin Output Select"]
    #[inline(always)]
    pub fn bclkdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bckcr::Bclkdiv,
        bckcr::Bclkdiv,
        Bckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bckcr::Bclkdiv,
            bckcr::Bclkdiv,
            Bckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bckcr {
    #[inline(always)]
    fn default() -> Bckcr {
        <crate::RegValueT<Bckcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bclkdiv_SPEC;
    pub type Bclkdiv = crate::EnumBitfieldStruct<u8, Bclkdiv_SPEC>;
    impl Bclkdiv {
        #[doc = "BCLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "BCLK/2"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mosccr_SPEC;
impl crate::sealed::RegSpec for Mosccr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Control Register"]
pub type Mosccr = crate::RegValueT<Mosccr_SPEC>;

impl Mosccr {
    #[doc = "Main Clock Oscillator Stop"]
    #[inline(always)]
    pub fn mostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mosccr::Mostp,
        mosccr::Mostp,
        Mosccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mosccr::Mostp,
            mosccr::Mostp,
            Mosccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mosccr {
    #[inline(always)]
    fn default() -> Mosccr {
        <crate::RegValueT<Mosccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod mosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mostp_SPEC;
    pub type Mostp = crate::EnumBitfieldStruct<u8, Mostp_SPEC>;
    impl Mostp {
        #[doc = "Main clock oscillator is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "Main clock oscillator is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hococr_SPEC;
impl crate::sealed::RegSpec for Hococr_SPEC {
    type DataType = u8;
}

#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub type Hococr = crate::RegValueT<Hococr_SPEC>;

impl Hococr {
    #[doc = "HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hococr::Hcstp,
        hococr::Hcstp,
        Hococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hococr::Hcstp,
            hococr::Hcstp,
            Hococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hococr {
    #[inline(always)]
    fn default() -> Hococr {
        <crate::RegValueT<Hococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hcstp_SPEC;
    pub type Hcstp = crate::EnumBitfieldStruct<u8, Hcstp_SPEC>;
    impl Hcstp {
        #[doc = "Operate the HOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the HOCO clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mococr_SPEC;
impl crate::sealed::RegSpec for Mococr_SPEC {
    type DataType = u8;
}

#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub type Mococr = crate::RegValueT<Mococr_SPEC>;

impl Mococr {
    #[doc = "MOCO Stop"]
    #[inline(always)]
    pub fn mcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mococr::Mcstp,
        mococr::Mcstp,
        Mococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mococr::Mcstp,
            mococr::Mcstp,
            Mococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mococr {
    #[inline(always)]
    fn default() -> Mococr {
        <crate::RegValueT<Mococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcstp_SPEC;
    pub type Mcstp = crate::EnumBitfieldStruct<u8, Mcstp_SPEC>;
    impl Mcstp {
        #[doc = "Operate the MOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the MOCO clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fllcr1_SPEC;
impl crate::sealed::RegSpec for Fllcr1_SPEC {
    type DataType = u8;
}

#[doc = "FLL Control Register 1"]
pub type Fllcr1 = crate::RegValueT<Fllcr1_SPEC>;

impl Fllcr1 {
    #[doc = "FLL Enable"]
    #[inline(always)]
    pub fn fllen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fllcr1::Fllen,
        fllcr1::Fllen,
        Fllcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fllcr1::Fllen,
            fllcr1::Fllen,
            Fllcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fllcr1 {
    #[inline(always)]
    fn default() -> Fllcr1 {
        <crate::RegValueT<Fllcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fllcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fllen_SPEC;
    pub type Fllen = crate::EnumBitfieldStruct<u8, Fllen_SPEC>;
    impl Fllen {
        #[doc = "FLL function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "FLL function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fllcr2_SPEC;
impl crate::sealed::RegSpec for Fllcr2_SPEC {
    type DataType = u16;
}

#[doc = "FLL Control Register 2"]
pub type Fllcr2 = crate::RegValueT<Fllcr2_SPEC>;

impl Fllcr2 {
    #[doc = "FLL Multiplication ControlMultiplication ratio  of the FLL reference clock select"]
    #[inline(always)]
    pub fn fllcntl(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fllcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fllcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fllcr2 {
    #[inline(always)]
    fn default() -> Fllcr2 {
        <crate::RegValueT<Fllcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscsf_SPEC;
impl crate::sealed::RegSpec for Oscsf_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stabilization Flag Register"]
pub type Oscsf = crate::RegValueT<Oscsf_SPEC>;

impl Oscsf {
    #[doc = "PLL Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        oscsf::Pllsf,
        oscsf::Pllsf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            oscsf::Pllsf,
            oscsf::Pllsf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Main Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn moscsf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        oscsf::Moscsf,
        oscsf::Moscsf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            oscsf::Moscsf,
            oscsf::Moscsf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "HOCO Clock Oscillation Stabilization FlagNOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
    #[inline(always)]
    pub fn hocosf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        oscsf::Hocosf,
        oscsf::Hocosf,
        Oscsf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            oscsf::Hocosf,
            oscsf::Hocosf,
            Oscsf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Oscsf {
    #[inline(always)]
    fn default() -> Oscsf {
        <crate::RegValueT<Oscsf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oscsf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "PLL clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL clock is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moscsf_SPEC;
    pub type Moscsf = crate::EnumBitfieldStruct<u8, Moscsf_SPEC>;
    impl Moscsf {
        #[doc = "Main clock oscillator is stopped (MOSTP = 1) or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Main clock oscillator is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hocosf_SPEC;
    pub type Hocosf = crate::EnumBitfieldStruct<u8, Hocosf_SPEC>;
    impl Hocosf {
        #[doc = "HOCO clock is stopped or is not yet stable"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO clock is stable, so is available for use as the system clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckocr_SPEC;
impl crate::sealed::RegSpec for Ckocr_SPEC {
    type DataType = u8;
}

#[doc = "Clock Out Control Register"]
pub type Ckocr = crate::RegValueT<Ckocr_SPEC>;

impl Ckocr {
    #[doc = "Clock out enable"]
    #[inline(always)]
    pub fn ckoen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ckocr::Ckoen,
        ckocr::Ckoen,
        Ckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ckocr::Ckoen,
            ckocr::Ckoen,
            Ckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock out input frequency Division Select"]
    #[inline(always)]
    pub fn ckodiv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        ckocr::Ckodiv,
        ckocr::Ckodiv,
        Ckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            ckocr::Ckodiv,
            ckocr::Ckodiv,
            Ckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock out source select"]
    #[inline(always)]
    pub fn ckosel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ckocr::Ckosel,
        ckocr::Ckosel,
        Ckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ckocr::Ckosel,
            ckocr::Ckosel,
            Ckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ckocr {
    #[inline(always)]
    fn default() -> Ckocr {
        <crate::RegValueT<Ckocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ckocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckoen_SPEC;
    pub type Ckoen = crate::EnumBitfieldStruct<u8, Ckoen_SPEC>;
    impl Ckoen {
        #[doc = "Disable clock out"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable clock out"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckodiv_SPEC;
    pub type Ckodiv = crate::EnumBitfieldStruct<u8, Ckodiv_SPEC>;
    impl Ckodiv {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _010: Self = Self::new(2);

        #[doc = "/8"]
        pub const _011: Self = Self::new(3);

        #[doc = "/16"]
        pub const _100: Self = Self::new(4);

        #[doc = "/32"]
        pub const _101: Self = Self::new(5);

        #[doc = "/64"]
        pub const _110: Self = Self::new(6);

        #[doc = "/128"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckosel_SPEC;
    pub type Ckosel = crate::EnumBitfieldStruct<u8, Ckosel_SPEC>;
    impl Ckosel {
        #[doc = "HOCO"]
        pub const _000: Self = Self::new(0);

        #[doc = "MOCO"]
        pub const _001: Self = Self::new(1);

        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);

        #[doc = "MOSC"]
        pub const _011: Self = Self::new(3);

        #[doc = "SOSC"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trckcr_SPEC;
impl crate::sealed::RegSpec for Trckcr_SPEC {
    type DataType = u8;
}

#[doc = "Trace Clock Control Register"]
pub type Trckcr = crate::RegValueT<Trckcr_SPEC>;

impl Trckcr {
    #[doc = "Trace Clock operating Enable"]
    #[inline(always)]
    pub fn trcken(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        trckcr::Trcken,
        trckcr::Trcken,
        Trckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            trckcr::Trcken,
            trckcr::Trcken,
            Trckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Trace Clock operating frequency select"]
    #[inline(always)]
    pub fn trck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        trckcr::Trck,
        trckcr::Trck,
        Trckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            trckcr::Trck,
            trckcr::Trck,
            Trckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trckcr {
    #[inline(always)]
    fn default() -> Trckcr {
        <crate::RegValueT<Trckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod trckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trcken_SPEC;
    pub type Trcken = crate::EnumBitfieldStruct<u8, Trcken_SPEC>;
    impl Trcken {
        #[doc = "Disable operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trck_SPEC;
    pub type Trck = crate::EnumBitfieldStruct<u8, Trck_SPEC>;
    impl Trck {
        #[doc = "/1"]
        pub const _0000: Self = Self::new(0);

        #[doc = "/2"]
        pub const _0001: Self = Self::new(1);

        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdcr_SPEC;
impl crate::sealed::RegSpec for Ostdcr_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stop Detection Control Register"]
pub type Ostdcr = crate::RegValueT<Ostdcr_SPEC>;

impl Ostdcr {
    #[doc = "Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub fn ostde(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ostdcr::Ostde,
        ostdcr::Ostde,
        Ostdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ostdcr::Ostde,
            ostdcr::Ostde,
            Ostdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ostdie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ostdcr::Ostdie,
        ostdcr::Ostdie,
        Ostdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ostdcr::Ostdie,
            ostdcr::Ostdie,
            Ostdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ostdcr {
    #[inline(always)]
    fn default() -> Ostdcr {
        <crate::RegValueT<Ostdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostde_SPEC;
    pub type Ostde = crate::EnumBitfieldStruct<u8, Ostde_SPEC>;
    impl Ostde {
        #[doc = "Disable oscillation stop detection function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable oscillation stop detection function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdie_SPEC;
    pub type Ostdie = crate::EnumBitfieldStruct<u8, Ostdie_SPEC>;
    impl Ostdie {
        #[doc = "Disable oscillation stop detection interrupt (do not notify the POEG)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable oscillation stop detection interrupt (notify the POEG)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdsr_SPEC;
impl crate::sealed::RegSpec for Ostdsr_SPEC {
    type DataType = u8;
}

#[doc = "Oscillation Stop Detection Status Register"]
pub type Ostdsr = crate::RegValueT<Ostdsr_SPEC>;

impl Ostdsr {
    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostdf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ostdsr::Ostdf,
        ostdsr::Ostdf,
        Ostdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ostdsr::Ostdf,
            ostdsr::Ostdf,
            Ostdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ostdsr {
    #[inline(always)]
    fn default() -> Ostdsr {
        <crate::RegValueT<Ostdsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdf_SPEC;
    pub type Ostdf = crate::EnumBitfieldStruct<u8, Ostdf_SPEC>;
    impl Ostdf {
        #[doc = "Main clock oscillation stop not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Main clock oscillation stop detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ebckocr_SPEC;
impl crate::sealed::RegSpec for Ebckocr_SPEC {
    type DataType = u8;
}

#[doc = "External Bus Clock Output Control Register"]
pub type Ebckocr = crate::RegValueT<Ebckocr_SPEC>;

impl Ebckocr {
    #[doc = "BCLK Pin Output Control"]
    #[inline(always)]
    pub fn ebckoen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ebckocr::Ebckoen,
        ebckocr::Ebckoen,
        Ebckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ebckocr::Ebckoen,
            ebckocr::Ebckoen,
            Ebckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ebckocr {
    #[inline(always)]
    fn default() -> Ebckocr {
        <crate::RegValueT<Ebckocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ebckocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ebckoen_SPEC;
    pub type Ebckoen = crate::EnumBitfieldStruct<u8, Ebckoen_SPEC>;
    impl Ebckoen {
        #[doc = "Disable EBCLK pin output (fixed high)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable EBCLK pin output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdckocr_SPEC;
impl crate::sealed::RegSpec for Sdckocr_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Clock Output Control Register"]
pub type Sdckocr = crate::RegValueT<Sdckocr_SPEC>;

impl Sdckocr {
    #[doc = "SDCLK Pin Output Control"]
    #[inline(always)]
    pub fn sdckoen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdckocr::Sdckoen,
        sdckocr::Sdckoen,
        Sdckocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdckocr::Sdckoen,
            sdckocr::Sdckoen,
            Sdckocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdckocr {
    #[inline(always)]
    fn default() -> Sdckocr {
        <crate::RegValueT<Sdckocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdckocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdckoen_SPEC;
    pub type Sdckoen = crate::EnumBitfieldStruct<u8, Sdckoen_SPEC>;
    impl Sdckoen {
        #[doc = "Disable SDCLK pin output (fixed high)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable SDCLK pin output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mocoutcr_SPEC;
impl crate::sealed::RegSpec for Mocoutcr_SPEC {
    type DataType = u8;
}

#[doc = "MOCO User Trimming Control Register"]
pub type Mocoutcr = crate::RegValueT<Mocoutcr_SPEC>;

impl Mocoutcr {
    #[doc = "MOCO User Trimming  1000_0000 : -128   1000_0001 : -127   1000_0010 : -126   . . .  1111_1111 : -1  0000_0000 : Center Code  0000_0001 : +1  . . .  0111_1101 : +125  0111_1110 : +126  0111_1111 : +127These bits are added to original MOCO trimming bits"]
    #[inline(always)]
    pub fn mocoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mocoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mocoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mocoutcr {
    #[inline(always)]
    fn default() -> Mocoutcr {
        <crate::RegValueT<Mocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocoutcr_SPEC;
impl crate::sealed::RegSpec for Hocoutcr_SPEC {
    type DataType = u8;
}

#[doc = "HOCO User Trimming Control Register"]
pub type Hocoutcr = crate::RegValueT<Hocoutcr_SPEC>;

impl Hocoutcr {
    #[doc = "HOCO User Trimming  1000_0000 : -128   1000_0001 : -127   1000_0010 : -126   . . .  1111_1111 : -1  0000_0000 : Center Code  0000_0001 : +1  . . .  0111_1101 : +125  0111_1110 : +126  0111_1111 : +127These bits are added to original HOCO trimming bits"]
    #[inline(always)]
    pub fn hocoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Hocoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Hocoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hocoutcr {
    #[inline(always)]
    fn default() -> Hocoutcr {
        <crate::RegValueT<Hocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Momcr_SPEC;
impl crate::sealed::RegSpec for Momcr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub type Momcr = crate::RegValueT<Momcr_SPEC>;

impl Momcr {
    #[doc = "Main Clock Oscillator Drive Capability Auto Switching Enable"]
    #[inline(always)]
    pub fn autodrven(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        momcr::Autodrven,
        momcr::Autodrven,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            momcr::Autodrven,
            momcr::Autodrven,
            Momcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        momcr::Mosel,
        momcr::Mosel,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            momcr::Mosel,
            momcr::Mosel,
            Momcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Main Clock Oscillator Drive Capability 0 Switching"]
    #[inline(always)]
    pub fn modrv0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        momcr::Modrv0,
        momcr::Modrv0,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            momcr::Modrv0,
            momcr::Modrv0,
            Momcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Momcr {
    #[inline(always)]
    fn default() -> Momcr {
        <crate::RegValueT<Momcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod momcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Autodrven_SPEC;
    pub type Autodrven = crate::EnumBitfieldStruct<u8, Autodrven_SPEC>;
    impl Autodrven {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mosel_SPEC;
    pub type Mosel = crate::EnumBitfieldStruct<u8, Mosel_SPEC>;
    impl Mosel {
        #[doc = "Resonator"]
        pub const _0: Self = Self::new(0);

        #[doc = "External clock input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modrv0_SPEC;
    pub type Modrv0 = crate::EnumBitfieldStruct<u8, Modrv0_SPEC>;
    impl Modrv0 {
        #[doc = "20MHz to 24MHz"]
        pub const _00: Self = Self::new(0);

        #[doc = "16MHz to 20MHz"]
        pub const _01: Self = Self::new(1);

        #[doc = "8MHz to 16MHz"]
        pub const _10: Self = Self::new(2);

        #[doc = "8MHz"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccr_SPEC;
impl crate::sealed::RegSpec for Sosccr_SPEC {
    type DataType = u8;
}

#[doc = "Sub-clock oscillator control register"]
pub type Sosccr = crate::RegValueT<Sosccr_SPEC>;

impl Sosccr {
    #[doc = "Sub-Clock Oscillator Stop"]
    #[inline(always)]
    pub fn sostp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sosccr::Sostp,
        sosccr::Sostp,
        Sosccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sosccr::Sostp,
            sosccr::Sostp,
            Sosccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sosccr {
    #[inline(always)]
    fn default() -> Sosccr {
        <crate::RegValueT<Sosccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostp_SPEC;
    pub type Sostp = crate::EnumBitfieldStruct<u8, Sostp_SPEC>;
    impl Sostp {
        #[doc = "Operate the sub-clock oscillator"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the sub-clock oscillator"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Somcr_SPEC;
impl crate::sealed::RegSpec for Somcr_SPEC {
    type DataType = u8;
}

#[doc = "Sub Clock Oscillator Mode Control Register"]
pub type Somcr = crate::RegValueT<Somcr_SPEC>;

impl Somcr {
    #[doc = "Sub Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        somcr::Sodrv1,
        somcr::Sodrv1,
        Somcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            somcr::Sodrv1,
            somcr::Sodrv1,
            Somcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Somcr {
    #[inline(always)]
    fn default() -> Somcr {
        <crate::RegValueT<Somcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod somcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sodrv1_SPEC;
    pub type Sodrv1 = crate::EnumBitfieldStruct<u8, Sodrv1_SPEC>;
    impl Sodrv1 {
        #[doc = "Standard"]
        pub const _0: Self = Self::new(0);

        #[doc = "Middle"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lococr_SPEC;
impl crate::sealed::RegSpec for Lococr_SPEC {
    type DataType = u8;
}

#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub type Lococr = crate::RegValueT<Lococr_SPEC>;

impl Lococr {
    #[doc = "LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lococr::Lcstp,
        lococr::Lcstp,
        Lococr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lococr::Lcstp,
            lococr::Lcstp,
            Lococr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lococr {
    #[inline(always)]
    fn default() -> Lococr {
        <crate::RegValueT<Lococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcstp_SPEC;
    pub type Lcstp = crate::EnumBitfieldStruct<u8, Lcstp_SPEC>;
    impl Lcstp {
        #[doc = "Operate the LOCO clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop the LOCO clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locoutcr_SPEC;
impl crate::sealed::RegSpec for Locoutcr_SPEC {
    type DataType = u8;
}

#[doc = "LOCO User Trimming Control Register"]
pub type Locoutcr = crate::RegValueT<Locoutcr_SPEC>;

impl Locoutcr {
    #[doc = "LOCO User Trimming  1000_0000 : -128   1000_0001 : -127   1000_0010 : -126   . . .  1111_1111 : -1  0000_0000 : Center Code  0000_0001 : +1  . . .  0111_1101 : +125  0111_1110 : +126  0111_1111 : +127These bits are added to original LOCO trimming bits"]
    #[inline(always)]
    pub fn locoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Locoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Locoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Locoutcr {
    #[inline(always)]
    fn default() -> Locoutcr {
        <crate::RegValueT<Locoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moscwtcr_SPEC;
impl crate::sealed::RegSpec for Moscwtcr_SPEC {
    type DataType = u8;
}

#[doc = "Main Clock Oscillator Wait Control Register"]
pub type Moscwtcr = crate::RegValueT<Moscwtcr_SPEC>;

impl Moscwtcr {
    #[doc = "Main clock oscillator wait time setting"]
    #[inline(always)]
    pub fn msts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        moscwtcr::Msts,
        moscwtcr::Msts,
        Moscwtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            moscwtcr::Msts,
            moscwtcr::Msts,
            Moscwtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Moscwtcr {
    #[inline(always)]
    fn default() -> Moscwtcr {
        <crate::RegValueT<Moscwtcr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod moscwtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msts_SPEC;
    pub type Msts = crate::EnumBitfieldStruct<u8, Msts_SPEC>;
    impl Msts {
        #[doc = "Wait time = 35 cycles (133.5 μs)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Wait time = 67 cycles (255.6 μs)"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Wait time = 131 cycles (499.7 μs)"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Wait time = 259 cycles (988.0 μs)"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Wait time = 547 cycles (2086.6 μs) (value after reset)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "Wait time = 1059 cycles (4039.8 μs)"]
        pub const _0110: Self = Self::new(6);

        #[doc = "Wait time = 2147 cycles (8190.2 μs)"]
        pub const _0111: Self = Self::new(7);

        #[doc = "Wait time = 4291 cycles (16368.9 μs)"]
        pub const _1000: Self = Self::new(8);

        #[doc = "Wait time = 8163 cycles (31139.4 μs)."]
        pub const _1001: Self = Self::new(9);

        #[doc = "settings prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocowtcr_SPEC;
impl crate::sealed::RegSpec for Hocowtcr_SPEC {
    type DataType = u8;
}

#[doc = "High-speed on-chip oscillator wait control register"]
pub type Hocowtcr = crate::RegValueT<Hocowtcr_SPEC>;

impl Hocowtcr {
    #[doc = "HOCO wait time settingWaiting time (sec) = setting of the HSTS\\[2:0\\] bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)"]
    #[inline(always)]
    pub fn hsts(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Hocowtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Hocowtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hocowtcr {
    #[inline(always)]
    fn default() -> Hocowtcr {
        <crate::RegValueT<Hocowtcr_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbycr_SPEC;
impl crate::sealed::RegSpec for Sbycr_SPEC {
    type DataType = u16;
}

#[doc = "Standby Control Register"]
pub type Sbycr = crate::RegValueT<Sbycr_SPEC>;

impl Sbycr {
    #[doc = "Software Standby"]
    #[inline(always)]
    pub fn ssby(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sbycr::Ssby,
        sbycr::Ssby,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sbycr::Ssby,
            sbycr::Ssby,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output Port Enable"]
    #[inline(always)]
    pub fn ope(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sbycr::Ope,
        sbycr::Ope,
        Sbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sbycr::Ope,
            sbycr::Ope,
            Sbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sbycr {
    #[inline(always)]
    fn default() -> Sbycr {
        <crate::RegValueT<Sbycr_SPEC> as RegisterValue<_>>::new(16384)
    }
}
pub mod sbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssby_SPEC;
    pub type Ssby = crate::EnumBitfieldStruct<u8, Ssby_SPEC>;
    impl Ssby {
        #[doc = "Sleep mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby mode (DPSBYCR.DPSBY=0) / Deep Software Standby mode (DPSBYCR.DPSBY=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ope_SPEC;
    pub type Ope = crate::EnumBitfieldStruct<u8, Ope_SPEC>;
    impl Ope {
        #[doc = "In software standby mode or deep software standby mode, the address bus and bus control signals are set to the high-impedance state."]
        pub const _0: Self = Self::new(0);

        #[doc = "In software standby mode or deep software standby mode, the address bus and bus control signals retain the output state.."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcra_SPEC;
impl crate::sealed::RegSpec for Mstpcra_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Control Register A"]
pub type Mstpcra = crate::RegValueT<Mstpcra_SPEC>;

impl Mstpcra {
    #[doc = "DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub fn mstpa22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mstpcra::Mstpa22,
        mstpcra::Mstpa22,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mstpcra::Mstpa22,
            mstpcra::Mstpa22,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Standny RAM Module Stop"]
    #[inline(always)]
    pub fn mstpa7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mstpcra::Mstpa7,
        mstpcra::Mstpa7,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mstpcra::Mstpa7,
            mstpcra::Mstpa7,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECCRAM Module Stop"]
    #[inline(always)]
    pub fn mstpa6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mstpcra::Mstpa6,
        mstpcra::Mstpa6,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mstpcra::Mstpa6,
            mstpcra::Mstpa6,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-Speed RAM Module Stop"]
    #[inline(always)]
    pub fn mstpa5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mstpcra::Mstpa5,
        mstpcra::Mstpa5,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mstpcra::Mstpa5,
            mstpcra::Mstpa5,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM1 Module Stop"]
    #[inline(always)]
    pub fn mstpa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mstpcra::Mstpa1,
        mstpcra::Mstpa1,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mstpcra::Mstpa1,
            mstpcra::Mstpa1,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM0 Module Stop"]
    #[inline(always)]
    pub fn mstpa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mstpcra::Mstpa0,
        mstpcra::Mstpa0,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mstpcra::Mstpa0,
            mstpcra::Mstpa0,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4290772764)
    }
}
pub mod mstpcra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa22_SPEC;
    pub type Mstpa22 = crate::EnumBitfieldStruct<u8, Mstpa22_SPEC>;
    impl Mstpa22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa7_SPEC;
    pub type Mstpa7 = crate::EnumBitfieldStruct<u8, Mstpa7_SPEC>;
    impl Mstpa7 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa6_SPEC;
    pub type Mstpa6 = crate::EnumBitfieldStruct<u8, Mstpa6_SPEC>;
    impl Mstpa6 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa5_SPEC;
    pub type Mstpa5 = crate::EnumBitfieldStruct<u8, Mstpa5_SPEC>;
    impl Mstpa5 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa1_SPEC;
    pub type Mstpa1 = crate::EnumBitfieldStruct<u8, Mstpa1_SPEC>;
    impl Mstpa1 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa0_SPEC;
    pub type Mstpa0 = crate::EnumBitfieldStruct<u8, Mstpa0_SPEC>;
    impl Mstpa0 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzcr_SPEC;
impl crate::sealed::RegSpec for Snzcr_SPEC {
    type DataType = u8;
}

#[doc = "Snooze Control Register"]
pub type Snzcr = crate::RegValueT<Snzcr_SPEC>;

impl Snzcr {
    #[doc = "Snooze Mode Enable"]
    #[inline(always)]
    pub fn snze(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzcr::Snze,
        snzcr::Snze,
        Snzcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzcr::Snze,
            snzcr::Snze,
            Snzcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DTC Enable in Snooze Mode"]
    #[inline(always)]
    pub fn snzdtcen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzcr::Snzdtcen,
        snzcr::Snzdtcen,
        Snzcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzcr::Snzdtcen,
            snzcr::Snzdtcen,
            Snzcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub fn rxdreqen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzcr::Rxdreqen,
        snzcr::Rxdreqen,
        Snzcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzcr::Rxdreqen,
            snzcr::Rxdreqen,
            Snzcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzcr {
    #[inline(always)]
    fn default() -> Snzcr {
        <crate::RegValueT<Snzcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snze_SPEC;
    pub type Snze = crate::EnumBitfieldStruct<u8, Snze_SPEC>;
    impl Snze {
        #[doc = "Disable Snooze Mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable Snooze Mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzdtcen_SPEC;
    pub type Snzdtcen = crate::EnumBitfieldStruct<u8, Snzdtcen_SPEC>;
    impl Snzdtcen {
        #[doc = "Disable DTC operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable DTC operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdreqen_SPEC;
    pub type Rxdreqen = crate::EnumBitfieldStruct<u8, Rxdreqen_SPEC>;
    impl Rxdreqen {
        #[doc = "Ignore RXD0 falling edge in Standby mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzedcr_SPEC;
impl crate::sealed::RegSpec for Snzedcr_SPEC {
    type DataType = u8;
}

#[doc = "Snooze End Control Register"]
pub type Snzedcr = crate::RegValueT<Snzedcr_SPEC>;

impl Snzedcr {
    #[doc = "SCI0 address unmatch Snooze End EnableNote: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub fn sci0umted(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzedcr::Sci0Umted,
        snzedcr::Sci0Umted,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzedcr::Sci0Umted,
            snzedcr::Sci0Umted,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AD compare mismatch 1 Snooze End Enable"]
    #[inline(always)]
    pub fn ad1umted(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        snzedcr::Ad1Umted,
        snzedcr::Ad1Umted,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            snzedcr::Ad1Umted,
            snzedcr::Ad1Umted,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AD compare match 1 Snooze End Enable"]
    #[inline(always)]
    pub fn ad1mated(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        snzedcr::Ad1Mated,
        snzedcr::Ad1Mated,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            snzedcr::Ad1Mated,
            snzedcr::Ad1Mated,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AD compare mismatch 0 Snooze End Enable"]
    #[inline(always)]
    pub fn ad0umted(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        snzedcr::Ad0Umted,
        snzedcr::Ad0Umted,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            snzedcr::Ad0Umted,
            snzedcr::Ad0Umted,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AD compare match 0 Snooze End Enable"]
    #[inline(always)]
    pub fn ad0mated(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        snzedcr::Ad0Mated,
        snzedcr::Ad0Mated,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            snzedcr::Ad0Mated,
            snzedcr::Ad0Mated,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Not Last DTC transmission completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snzedcr::Dtcnzred,
        snzedcr::Dtcnzred,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snzedcr::Dtcnzred,
            snzedcr::Dtcnzred,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Last DTC transmission completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzedcr::Dtczred,
        snzedcr::Dtczred,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzedcr::Dtczred,
            snzedcr::Dtczred,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agt1unfed(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzedcr::Agt1Unfed,
        snzedcr::Agt1Unfed,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzedcr::Agt1Unfed,
            snzedcr::Agt1Unfed,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzedcr {
    #[inline(always)]
    fn default() -> Snzedcr {
        <crate::RegValueT<Snzedcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzedcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sci0Umted_SPEC;
    pub type Sci0Umted = crate::EnumBitfieldStruct<u8, Sci0Umted_SPEC>;
    impl Sci0Umted {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad1Umted_SPEC;
    pub type Ad1Umted = crate::EnumBitfieldStruct<u8, Ad1Umted_SPEC>;
    impl Ad1Umted {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad1Mated_SPEC;
    pub type Ad1Mated = crate::EnumBitfieldStruct<u8, Ad1Mated_SPEC>;
    impl Ad1Mated {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad0Umted_SPEC;
    pub type Ad0Umted = crate::EnumBitfieldStruct<u8, Ad0Umted_SPEC>;
    impl Ad0Umted {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad0Mated_SPEC;
    pub type Ad0Mated = crate::EnumBitfieldStruct<u8, Ad0Mated_SPEC>;
    impl Ad0Mated {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcnzred_SPEC;
    pub type Dtcnzred = crate::EnumBitfieldStruct<u8, Dtcnzred_SPEC>;
    impl Dtcnzred {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtczred_SPEC;
    pub type Dtczred = crate::EnumBitfieldStruct<u8, Dtczred_SPEC>;
    impl Dtczred {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Unfed_SPEC;
    pub type Agt1Unfed = crate::EnumBitfieldStruct<u8, Agt1Unfed_SPEC>;
    impl Agt1Unfed {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzreqcr_SPEC;
impl crate::sealed::RegSpec for Snzreqcr_SPEC {
    type DataType = u32;
}

#[doc = "Snooze Request Control Register"]
pub type Snzreqcr = crate::RegValueT<Snzreqcr_SPEC>;

impl Snzreqcr {
    #[doc = "Enable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen30,
        snzreqcr::Snzreqen30,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen30,
            snzreqcr::Snzreqen30,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen29,
        snzreqcr::Snzreqen29,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen29,
            snzreqcr::Snzreqen29,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable AGT1 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen28,
        snzreqcr::Snzreqen28,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen28,
            snzreqcr::Snzreqen28,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable RTC period snooze request"]
    #[inline(always)]
    pub fn snzreqen25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen25,
        snzreqcr::Snzreqen25,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen25,
            snzreqcr::Snzreqen25,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen24,
        snzreqcr::Snzreqen24,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen24,
            snzreqcr::Snzreqen24,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable ACMPHS0 snooze request"]
    #[inline(always)]
    pub fn snzreqen22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen22,
        snzreqcr::Snzreqen22,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen22,
            snzreqcr::Snzreqen22,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable KR snooze request"]
    #[inline(always)]
    pub fn snzreqen17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen17,
        snzreqcr::Snzreqen17,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen17,
            snzreqcr::Snzreqen17,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ13 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen13,
        snzreqcr::Snzreqen13,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen13,
            snzreqcr::Snzreqen13,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ12 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen12,
        snzreqcr::Snzreqen12,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen12,
            snzreqcr::Snzreqen12,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ11 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen11,
        snzreqcr::Snzreqen11,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen11,
            snzreqcr::Snzreqen11,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ10 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen10,
        snzreqcr::Snzreqen10,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen10,
            snzreqcr::Snzreqen10,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ9 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen9,
        snzreqcr::Snzreqen9,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen9,
            snzreqcr::Snzreqen9,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ8 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen8,
        snzreqcr::Snzreqen8,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen8,
            snzreqcr::Snzreqen8,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen7,
        snzreqcr::Snzreqen7,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen7,
            snzreqcr::Snzreqen7,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen6,
        snzreqcr::Snzreqen6,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen6,
            snzreqcr::Snzreqen6,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen5,
        snzreqcr::Snzreqen5,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen5,
            snzreqcr::Snzreqen5,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen4,
        snzreqcr::Snzreqen4,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen4,
            snzreqcr::Snzreqen4,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen3,
        snzreqcr::Snzreqen3,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen3,
            snzreqcr::Snzreqen3,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen2,
        snzreqcr::Snzreqen2,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen2,
            snzreqcr::Snzreqen2,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen1,
        snzreqcr::Snzreqen1,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen1,
            snzreqcr::Snzreqen1,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen0,
        snzreqcr::Snzreqen0,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen0,
            snzreqcr::Snzreqen0,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzreqcr {
    #[inline(always)]
    fn default() -> Snzreqcr {
        <crate::RegValueT<Snzreqcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzreqcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen30_SPEC;
    pub type Snzreqen30 = crate::EnumBitfieldStruct<u8, Snzreqen30_SPEC>;
    impl Snzreqen30 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen29_SPEC;
    pub type Snzreqen29 = crate::EnumBitfieldStruct<u8, Snzreqen29_SPEC>;
    impl Snzreqen29 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen28_SPEC;
    pub type Snzreqen28 = crate::EnumBitfieldStruct<u8, Snzreqen28_SPEC>;
    impl Snzreqen28 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen25_SPEC;
    pub type Snzreqen25 = crate::EnumBitfieldStruct<u8, Snzreqen25_SPEC>;
    impl Snzreqen25 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen24_SPEC;
    pub type Snzreqen24 = crate::EnumBitfieldStruct<u8, Snzreqen24_SPEC>;
    impl Snzreqen24 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen22_SPEC;
    pub type Snzreqen22 = crate::EnumBitfieldStruct<u8, Snzreqen22_SPEC>;
    impl Snzreqen22 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen17_SPEC;
    pub type Snzreqen17 = crate::EnumBitfieldStruct<u8, Snzreqen17_SPEC>;
    impl Snzreqen17 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen13_SPEC;
    pub type Snzreqen13 = crate::EnumBitfieldStruct<u8, Snzreqen13_SPEC>;
    impl Snzreqen13 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen12_SPEC;
    pub type Snzreqen12 = crate::EnumBitfieldStruct<u8, Snzreqen12_SPEC>;
    impl Snzreqen12 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen11_SPEC;
    pub type Snzreqen11 = crate::EnumBitfieldStruct<u8, Snzreqen11_SPEC>;
    impl Snzreqen11 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen10_SPEC;
    pub type Snzreqen10 = crate::EnumBitfieldStruct<u8, Snzreqen10_SPEC>;
    impl Snzreqen10 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen9_SPEC;
    pub type Snzreqen9 = crate::EnumBitfieldStruct<u8, Snzreqen9_SPEC>;
    impl Snzreqen9 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen8_SPEC;
    pub type Snzreqen8 = crate::EnumBitfieldStruct<u8, Snzreqen8_SPEC>;
    impl Snzreqen8 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen7_SPEC;
    pub type Snzreqen7 = crate::EnumBitfieldStruct<u8, Snzreqen7_SPEC>;
    impl Snzreqen7 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen6_SPEC;
    pub type Snzreqen6 = crate::EnumBitfieldStruct<u8, Snzreqen6_SPEC>;
    impl Snzreqen6 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen5_SPEC;
    pub type Snzreqen5 = crate::EnumBitfieldStruct<u8, Snzreqen5_SPEC>;
    impl Snzreqen5 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen4_SPEC;
    pub type Snzreqen4 = crate::EnumBitfieldStruct<u8, Snzreqen4_SPEC>;
    impl Snzreqen4 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen3_SPEC;
    pub type Snzreqen3 = crate::EnumBitfieldStruct<u8, Snzreqen3_SPEC>;
    impl Snzreqen3 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen2_SPEC;
    pub type Snzreqen2 = crate::EnumBitfieldStruct<u8, Snzreqen2_SPEC>;
    impl Snzreqen2 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen1_SPEC;
    pub type Snzreqen1 = crate::EnumBitfieldStruct<u8, Snzreqen1_SPEC>;
    impl Snzreqen1 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen0_SPEC;
    pub type Snzreqen0 = crate::EnumBitfieldStruct<u8, Snzreqen0_SPEC>;
    impl Snzreqen0 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opccr_SPEC;
impl crate::sealed::RegSpec for Opccr_SPEC {
    type DataType = u8;
}

#[doc = "Operating Power Control Register"]
pub type Opccr = crate::RegValueT<Opccr_SPEC>;

impl Opccr {
    #[doc = "Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn opcmtsf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        opccr::Opcmtsf,
        opccr::Opcmtsf,
        Opccr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            opccr::Opcmtsf,
            opccr::Opcmtsf,
            Opccr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn opcm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        opccr::Opcm,
        opccr::Opcm,
        Opccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            opccr::Opcm,
            opccr::Opcm,
            Opccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Opccr {
    #[inline(always)]
    fn default() -> Opccr {
        <crate::RegValueT<Opccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod opccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opcmtsf_SPEC;
    pub type Opcmtsf = crate::EnumBitfieldStruct<u8, Opcmtsf_SPEC>;
    impl Opcmtsf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);

        #[doc = "During transition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opcm_SPEC;
    pub type Opcm = crate::EnumBitfieldStruct<u8, Opcm_SPEC>;
    impl Opcm {
        #[doc = "High-speed mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low-speed mode"]
        pub const _11: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sopccr_SPEC;
impl crate::sealed::RegSpec for Sopccr_SPEC {
    type DataType = u8;
}

#[doc = "Sub Operating Power Control Register"]
pub type Sopccr = crate::RegValueT<Sopccr_SPEC>;

impl Sopccr {
    #[doc = "Sub Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn sopcmtsf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sopccr::Sopcmtsf,
        sopccr::Sopcmtsf,
        Sopccr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sopccr::Sopcmtsf,
            sopccr::Sopcmtsf,
            Sopccr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn sopcm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sopccr::Sopcm,
        sopccr::Sopcm,
        Sopccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sopccr::Sopcm,
            sopccr::Sopcm,
            Sopccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sopccr {
    #[inline(always)]
    fn default() -> Sopccr {
        <crate::RegValueT<Sopccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sopccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sopcmtsf_SPEC;
    pub type Sopcmtsf = crate::EnumBitfieldStruct<u8, Sopcmtsf_SPEC>;
    impl Sopcmtsf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);

        #[doc = "During transition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sopcm_SPEC;
    pub type Sopcm = crate::EnumBitfieldStruct<u8, Sopcm_SPEC>;
    impl Sopcm {
        #[doc = "Other than Subosc-speed mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Subosc-speed mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsbycr_SPEC;
impl crate::sealed::RegSpec for Dpsbycr_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Control Register"]
pub type Dpsbycr = crate::RegValueT<Dpsbycr_SPEC>;

impl Dpsbycr {
    #[doc = "Deep Software Standby"]
    #[inline(always)]
    pub fn dpsby(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsbycr::Dpsby,
        dpsbycr::Dpsby,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsbycr::Dpsby,
            dpsbycr::Dpsby,
            Dpsbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I/O Port Retention"]
    #[inline(always)]
    pub fn iokeep(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsbycr::Iokeep,
        dpsbycr::Iokeep,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsbycr::Iokeep,
            dpsbycr::Iokeep,
            Dpsbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Power-Supply Control"]
    #[inline(always)]
    pub fn deepcut(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dpsbycr::Deepcut,
        dpsbycr::Deepcut,
        Dpsbycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dpsbycr::Deepcut,
            dpsbycr::Deepcut,
            Dpsbycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsbycr {
    #[inline(always)]
    fn default() -> Dpsbycr {
        <crate::RegValueT<Dpsbycr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod dpsbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpsby_SPEC;
    pub type Dpsby = crate::EnumBitfieldStruct<u8, Dpsby_SPEC>;
    impl Dpsby {
        #[doc = "Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iokeep_SPEC;
    pub type Iokeep = crate::EnumBitfieldStruct<u8, Iokeep_SPEC>;
    impl Iokeep {
        #[doc = "When the Deep Software Standby mode is canceled, the I/O ports are in the reset state."]
        pub const _0: Self = Self::new(0);

        #[doc = "When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deepcut_SPEC;
    pub type Deepcut = crate::EnumBitfieldStruct<u8, Deepcut_SPEC>;
    impl Deepcut {
        #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode."]
        pub const _00: Self = Self::new(0);

        #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is not supplied in deep software standby mode."]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited."]
        pub const _10: Self = Self::new(2);

        #[doc = "Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier0_SPEC;
impl crate::sealed::RegSpec for Dpsier0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Enable Register 0"]
pub type Dpsier0 = crate::RegValueT<Dpsier0_SPEC>;

impl Dpsier0 {
    #[doc = "IRQ7-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq7e(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsier0::Dirq7E,
        dpsier0::Dirq7E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsier0::Dirq7E,
            dpsier0::Dirq7E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ6-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq6e(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsier0::Dirq6E,
        dpsier0::Dirq6E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsier0::Dirq6E,
            dpsier0::Dirq6E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ5-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq5e(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsier0::Dirq5E,
        dpsier0::Dirq5E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsier0::Dirq5E,
            dpsier0::Dirq5E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq4e(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsier0::Dirq4E,
        dpsier0::Dirq4E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsier0::Dirq4E,
            dpsier0::Dirq4E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ1-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq1e(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier0::Dirq1E,
        dpsier0::Dirq1E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier0::Dirq1E,
            dpsier0::Dirq1E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ0-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq0e(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier0::Dirq0E,
        dpsier0::Dirq0E,
        Dpsier0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier0::Dirq0E,
            dpsier0::Dirq0E,
            Dpsier0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier0 {
    #[inline(always)]
    fn default() -> Dpsier0 {
        <crate::RegValueT<Dpsier0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq7E_SPEC;
    pub type Dirq7E = crate::EnumBitfieldStruct<u8, Dirq7E_SPEC>;
    impl Dirq7E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq6E_SPEC;
    pub type Dirq6E = crate::EnumBitfieldStruct<u8, Dirq6E_SPEC>;
    impl Dirq6E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq5E_SPEC;
    pub type Dirq5E = crate::EnumBitfieldStruct<u8, Dirq5E_SPEC>;
    impl Dirq5E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq4E_SPEC;
    pub type Dirq4E = crate::EnumBitfieldStruct<u8, Dirq4E_SPEC>;
    impl Dirq4E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq1E_SPEC;
    pub type Dirq1E = crate::EnumBitfieldStruct<u8, Dirq1E_SPEC>;
    impl Dirq1E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq0E_SPEC;
    pub type Dirq0E = crate::EnumBitfieldStruct<u8, Dirq0E_SPEC>;
    impl Dirq0E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier1_SPEC;
impl crate::sealed::RegSpec for Dpsier1_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Enable Register 1"]
pub type Dpsier1 = crate::RegValueT<Dpsier1_SPEC>;

impl Dpsier1 {
    #[doc = "IRQ12-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq12e(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsier1::Dirq12E,
        dpsier1::Dirq12E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsier1::Dirq12E,
            dpsier1::Dirq12E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ11-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq11e(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsier1::Dirq11E,
        dpsier1::Dirq11E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsier1::Dirq11E,
            dpsier1::Dirq11E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ10-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq10e(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier1::Dirq10E,
        dpsier1::Dirq10E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier1::Dirq10E,
            dpsier1::Dirq10E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ9-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq9e(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier1::Dirq9E,
        dpsier1::Dirq9E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier1::Dirq9E,
            dpsier1::Dirq9E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ8-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq8e(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier1::Dirq8E,
        dpsier1::Dirq8E,
        Dpsier1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier1::Dirq8E,
            dpsier1::Dirq8E,
            Dpsier1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier1 {
    #[inline(always)]
    fn default() -> Dpsier1 {
        <crate::RegValueT<Dpsier1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq12E_SPEC;
    pub type Dirq12E = crate::EnumBitfieldStruct<u8, Dirq12E_SPEC>;
    impl Dirq12E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq11E_SPEC;
    pub type Dirq11E = crate::EnumBitfieldStruct<u8, Dirq11E_SPEC>;
    impl Dirq11E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq10E_SPEC;
    pub type Dirq10E = crate::EnumBitfieldStruct<u8, Dirq10E_SPEC>;
    impl Dirq10E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9E_SPEC;
    pub type Dirq9E = crate::EnumBitfieldStruct<u8, Dirq9E_SPEC>;
    impl Dirq9E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq8E_SPEC;
    pub type Dirq8E = crate::EnumBitfieldStruct<u8, Dirq8E_SPEC>;
    impl Dirq8E {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier2_SPEC;
impl crate::sealed::RegSpec for Dpsier2_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Enable Register 2"]
pub type Dpsier2 = crate::RegValueT<Dpsier2_SPEC>;

impl Dpsier2 {
    #[doc = "NMI Pin Enable"]
    #[inline(always)]
    pub fn dnmie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsier2::Dnmie,
        dpsier2::Dnmie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsier2::Dnmie,
            dpsier2::Dnmie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm interrupt Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn drtcaie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsier2::Drtcaie,
        dpsier2::Drtcaie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsier2::Drtcaie,
            dpsier2::Drtcaie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Interval interrupt Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dtrtciie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier2::Dtrtciie,
        dpsier2::Dtrtciie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier2::Dtrtciie,
            dpsier2::Dtrtciie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dlvd2ie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsier2::Dlvd2Ie,
        dpsier2::Dlvd2Ie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsier2::Dlvd2Ie,
            dpsier2::Dlvd2Ie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD1 Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dlvd1ie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier2::Dlvd1Ie,
        dpsier2::Dlvd1Ie,
        Dpsier2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier2::Dlvd1Ie,
            dpsier2::Dlvd1Ie,
            Dpsier2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier2 {
    #[inline(always)]
    fn default() -> Dpsier2 {
        <crate::RegValueT<Dpsier2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnmie_SPEC;
    pub type Dnmie = crate::EnumBitfieldStruct<u8, Dnmie_SPEC>;
    impl Dnmie {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drtcaie_SPEC;
    pub type Drtcaie = crate::EnumBitfieldStruct<u8, Drtcaie_SPEC>;
    impl Drtcaie {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtrtciie_SPEC;
    pub type Dtrtciie = crate::EnumBitfieldStruct<u8, Dtrtciie_SPEC>;
    impl Dtrtciie {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd2Ie_SPEC;
    pub type Dlvd2Ie = crate::EnumBitfieldStruct<u8, Dlvd2Ie_SPEC>;
    impl Dlvd2Ie {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd1Ie_SPEC;
    pub type Dlvd1Ie = crate::EnumBitfieldStruct<u8, Dlvd1Ie_SPEC>;
    impl Dlvd1Ie {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsier3_SPEC;
impl crate::sealed::RegSpec for Dpsier3_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Enable Register 3"]
pub type Dpsier3 = crate::RegValueT<Dpsier3_SPEC>;

impl Dpsier3 {
    #[doc = "AGT1 Underflow Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dagt1ie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsier3::Dagt1Ie,
        dpsier3::Dagt1Ie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsier3::Dagt1Ie,
            dpsier3::Dagt1Ie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBFS Suspend/Resume Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dusbfsie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsier3::Dusbfsie,
        dpsier3::Dusbfsie,
        Dpsier3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsier3::Dusbfsie,
            dpsier3::Dusbfsie,
            Dpsier3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsier3 {
    #[inline(always)]
    fn default() -> Dpsier3 {
        <crate::RegValueT<Dpsier3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsier3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dagt1Ie_SPEC;
    pub type Dagt1Ie = crate::EnumBitfieldStruct<u8, Dagt1Ie_SPEC>;
    impl Dagt1Ie {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dusbfsie_SPEC;
    pub type Dusbfsie = crate::EnumBitfieldStruct<u8, Dusbfsie_SPEC>;
    impl Dusbfsie {
        #[doc = "Canceling deep software standby mode is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Canceling deep software standby mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr0_SPEC;
impl crate::sealed::RegSpec for Dpsifr0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Flag Register 0"]
pub type Dpsifr0 = crate::RegValueT<Dpsifr0_SPEC>;

impl Dpsifr0 {
    #[doc = "IRQ7-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq7f(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsifr0::Dirq7F,
        dpsifr0::Dirq7F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsifr0::Dirq7F,
            dpsifr0::Dirq7F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ6-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq6f(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsifr0::Dirq6F,
        dpsifr0::Dirq6F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsifr0::Dirq6F,
            dpsifr0::Dirq6F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ5-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq5f(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsifr0::Dirq5F,
        dpsifr0::Dirq5F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsifr0::Dirq5F,
            dpsifr0::Dirq5F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq4f(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsifr0::Dirq4F,
        dpsifr0::Dirq4F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsifr0::Dirq4F,
            dpsifr0::Dirq4F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ1-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq1f(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr0::Dirq1F,
        dpsifr0::Dirq1F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr0::Dirq1F,
            dpsifr0::Dirq1F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ0-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq0f(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr0::Dirq0F,
        dpsifr0::Dirq0F,
        Dpsifr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr0::Dirq0F,
            dpsifr0::Dirq0F,
            Dpsifr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr0 {
    #[inline(always)]
    fn default() -> Dpsifr0 {
        <crate::RegValueT<Dpsifr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq7F_SPEC;
    pub type Dirq7F = crate::EnumBitfieldStruct<u8, Dirq7F_SPEC>;
    impl Dirq7F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq6F_SPEC;
    pub type Dirq6F = crate::EnumBitfieldStruct<u8, Dirq6F_SPEC>;
    impl Dirq6F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq5F_SPEC;
    pub type Dirq5F = crate::EnumBitfieldStruct<u8, Dirq5F_SPEC>;
    impl Dirq5F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq4F_SPEC;
    pub type Dirq4F = crate::EnumBitfieldStruct<u8, Dirq4F_SPEC>;
    impl Dirq4F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq1F_SPEC;
    pub type Dirq1F = crate::EnumBitfieldStruct<u8, Dirq1F_SPEC>;
    impl Dirq1F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq0F_SPEC;
    pub type Dirq0F = crate::EnumBitfieldStruct<u8, Dirq0F_SPEC>;
    impl Dirq0F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr1_SPEC;
impl crate::sealed::RegSpec for Dpsifr1_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Flag Register 1"]
pub type Dpsifr1 = crate::RegValueT<Dpsifr1_SPEC>;

impl Dpsifr1 {
    #[doc = "IRQ12-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq12f(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsifr1::Dirq12F,
        dpsifr1::Dirq12F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsifr1::Dirq12F,
            dpsifr1::Dirq12F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ11-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq11f(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsifr1::Dirq11F,
        dpsifr1::Dirq11F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsifr1::Dirq11F,
            dpsifr1::Dirq11F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ10-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq10f(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr1::Dirq10F,
        dpsifr1::Dirq10F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr1::Dirq10F,
            dpsifr1::Dirq10F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ9-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq9f(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr1::Dirq9F,
        dpsifr1::Dirq9F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr1::Dirq9F,
            dpsifr1::Dirq9F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ8-DS Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dirq8f(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr1::Dirq8F,
        dpsifr1::Dirq8F,
        Dpsifr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr1::Dirq8F,
            dpsifr1::Dirq8F,
            Dpsifr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr1 {
    #[inline(always)]
    fn default() -> Dpsifr1 {
        <crate::RegValueT<Dpsifr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq12F_SPEC;
    pub type Dirq12F = crate::EnumBitfieldStruct<u8, Dirq12F_SPEC>;
    impl Dirq12F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq11F_SPEC;
    pub type Dirq11F = crate::EnumBitfieldStruct<u8, Dirq11F_SPEC>;
    impl Dirq11F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq10F_SPEC;
    pub type Dirq10F = crate::EnumBitfieldStruct<u8, Dirq10F_SPEC>;
    impl Dirq10F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9F_SPEC;
    pub type Dirq9F = crate::EnumBitfieldStruct<u8, Dirq9F_SPEC>;
    impl Dirq9F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq8F_SPEC;
    pub type Dirq8F = crate::EnumBitfieldStruct<u8, Dirq8F_SPEC>;
    impl Dirq8F {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr2_SPEC;
impl crate::sealed::RegSpec for Dpsifr2_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Flag Register 2"]
pub type Dpsifr2 = crate::RegValueT<Dpsifr2_SPEC>;

impl Dpsifr2 {
    #[doc = "NMI Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dnmif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsifr2::Dnmif,
        dpsifr2::Dnmif,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsifr2::Dnmif,
            dpsifr2::Dnmif,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm interrupt Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn drtcaif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsifr2::Drtcaif,
        dpsifr2::Drtcaif,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsifr2::Drtcaif,
            dpsifr2::Drtcaif,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Interval interrupt Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dtrtciif(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr2::Dtrtciif,
        dpsifr2::Dtrtciif,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr2::Dtrtciif,
            dpsifr2::Dtrtciif,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dlvd2if(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsifr2::Dlvd2If,
        dpsifr2::Dlvd2If,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsifr2::Dlvd2If,
            dpsifr2::Dlvd2If,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD1 Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dlvd1if(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr2::Dlvd1If,
        dpsifr2::Dlvd1If,
        Dpsifr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr2::Dlvd1If,
            dpsifr2::Dlvd1If,
            Dpsifr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr2 {
    #[inline(always)]
    fn default() -> Dpsifr2 {
        <crate::RegValueT<Dpsifr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnmif_SPEC;
    pub type Dnmif = crate::EnumBitfieldStruct<u8, Dnmif_SPEC>;
    impl Dnmif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drtcaif_SPEC;
    pub type Drtcaif = crate::EnumBitfieldStruct<u8, Drtcaif_SPEC>;
    impl Drtcaif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtrtciif_SPEC;
    pub type Dtrtciif = crate::EnumBitfieldStruct<u8, Dtrtciif_SPEC>;
    impl Dtrtciif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd2If_SPEC;
    pub type Dlvd2If = crate::EnumBitfieldStruct<u8, Dlvd2If_SPEC>;
    impl Dlvd2If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd1If_SPEC;
    pub type Dlvd1If = crate::EnumBitfieldStruct<u8, Dlvd1If_SPEC>;
    impl Dlvd1If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsifr3_SPEC;
impl crate::sealed::RegSpec for Dpsifr3_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Flag Register 3"]
pub type Dpsifr3 = crate::RegValueT<Dpsifr3_SPEC>;

impl Dpsifr3 {
    #[doc = "AGT1 Underflow Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dagt1if(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsifr3::Dagt1If,
        dpsifr3::Dagt1If,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsifr3::Dagt1If,
            dpsifr3::Dagt1If,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBFS Suspend/Resume Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dusbfsif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsifr3::Dusbfsif,
        dpsifr3::Dusbfsif,
        Dpsifr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsifr3::Dusbfsif,
            dpsifr3::Dusbfsif,
            Dpsifr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsifr3 {
    #[inline(always)]
    fn default() -> Dpsifr3 {
        <crate::RegValueT<Dpsifr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsifr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dagt1If_SPEC;
    pub type Dagt1If = crate::EnumBitfieldStruct<u8, Dagt1If_SPEC>;
    impl Dagt1If {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dusbfsif_SPEC;
    pub type Dusbfsif = crate::EnumBitfieldStruct<u8, Dusbfsif_SPEC>;
    impl Dusbfsif {
        #[doc = "The cancel request is not generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "The cancel request is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr0_SPEC;
impl crate::sealed::RegSpec for Dpsiegr0_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Edge Register 0"]
pub type Dpsiegr0 = crate::RegValueT<Dpsiegr0_SPEC>;

impl Dpsiegr0 {
    #[doc = "IRQ7-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq7eg(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpsiegr0::Dirq7Eg,
        dpsiegr0::Dirq7Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpsiegr0::Dirq7Eg,
            dpsiegr0::Dirq7Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ6-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq6eg(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dpsiegr0::Dirq6Eg,
        dpsiegr0::Dirq6Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dpsiegr0::Dirq6Eg,
            dpsiegr0::Dirq6Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ5-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq5eg(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpsiegr0::Dirq5Eg,
        dpsiegr0::Dirq5Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpsiegr0::Dirq5Eg,
            dpsiegr0::Dirq5Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq4eg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsiegr0::Dirq4Eg,
        dpsiegr0::Dirq4Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsiegr0::Dirq4Eg,
            dpsiegr0::Dirq4Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ1-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq1eg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsiegr0::Dirq1Eg,
        dpsiegr0::Dirq1Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsiegr0::Dirq1Eg,
            dpsiegr0::Dirq1Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ0-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq0eg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsiegr0::Dirq0Eg,
        dpsiegr0::Dirq0Eg,
        Dpsiegr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsiegr0::Dirq0Eg,
            dpsiegr0::Dirq0Eg,
            Dpsiegr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsiegr0 {
    #[inline(always)]
    fn default() -> Dpsiegr0 {
        <crate::RegValueT<Dpsiegr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsiegr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq7Eg_SPEC;
    pub type Dirq7Eg = crate::EnumBitfieldStruct<u8, Dirq7Eg_SPEC>;
    impl Dirq7Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq6Eg_SPEC;
    pub type Dirq6Eg = crate::EnumBitfieldStruct<u8, Dirq6Eg_SPEC>;
    impl Dirq6Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq5Eg_SPEC;
    pub type Dirq5Eg = crate::EnumBitfieldStruct<u8, Dirq5Eg_SPEC>;
    impl Dirq5Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq4Eg_SPEC;
    pub type Dirq4Eg = crate::EnumBitfieldStruct<u8, Dirq4Eg_SPEC>;
    impl Dirq4Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq1Eg_SPEC;
    pub type Dirq1Eg = crate::EnumBitfieldStruct<u8, Dirq1Eg_SPEC>;
    impl Dirq1Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq0Eg_SPEC;
    pub type Dirq0Eg = crate::EnumBitfieldStruct<u8, Dirq0Eg_SPEC>;
    impl Dirq0Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr1_SPEC;
impl crate::sealed::RegSpec for Dpsiegr1_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Edge Register 1"]
pub type Dpsiegr1 = crate::RegValueT<Dpsiegr1_SPEC>;

impl Dpsiegr1 {
    #[doc = "IRQ12-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq12eg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsiegr1::Dirq12Eg,
        dpsiegr1::Dirq12Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsiegr1::Dirq12Eg,
            dpsiegr1::Dirq12Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ11-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq11eg(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dpsiegr1::Dirq11Eg,
        dpsiegr1::Dirq11Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dpsiegr1::Dirq11Eg,
            dpsiegr1::Dirq11Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ10-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq10eg(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dpsiegr1::Dirq10Eg,
        dpsiegr1::Dirq10Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dpsiegr1::Dirq10Eg,
            dpsiegr1::Dirq10Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ9-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq9eg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsiegr1::Dirq9Eg,
        dpsiegr1::Dirq9Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsiegr1::Dirq9Eg,
            dpsiegr1::Dirq9Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ8-DS Pin Edge Select"]
    #[inline(always)]
    pub fn dirq8eg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsiegr1::Dirq8Eg,
        dpsiegr1::Dirq8Eg,
        Dpsiegr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsiegr1::Dirq8Eg,
            dpsiegr1::Dirq8Eg,
            Dpsiegr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsiegr1 {
    #[inline(always)]
    fn default() -> Dpsiegr1 {
        <crate::RegValueT<Dpsiegr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsiegr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq12Eg_SPEC;
    pub type Dirq12Eg = crate::EnumBitfieldStruct<u8, Dirq12Eg_SPEC>;
    impl Dirq12Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq11Eg_SPEC;
    pub type Dirq11Eg = crate::EnumBitfieldStruct<u8, Dirq11Eg_SPEC>;
    impl Dirq11Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq10Eg_SPEC;
    pub type Dirq10Eg = crate::EnumBitfieldStruct<u8, Dirq10Eg_SPEC>;
    impl Dirq10Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq9Eg_SPEC;
    pub type Dirq9Eg = crate::EnumBitfieldStruct<u8, Dirq9Eg_SPEC>;
    impl Dirq9Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirq8Eg_SPEC;
    pub type Dirq8Eg = crate::EnumBitfieldStruct<u8, Dirq8Eg_SPEC>;
    impl Dirq8Eg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpsiegr2_SPEC;
impl crate::sealed::RegSpec for Dpsiegr2_SPEC {
    type DataType = u8;
}

#[doc = "Deep Standby Interrupt Edge Register 2"]
pub type Dpsiegr2 = crate::RegValueT<Dpsiegr2_SPEC>;

impl Dpsiegr2 {
    #[doc = "NMI Pin Edge Select"]
    #[inline(always)]
    pub fn dnmieg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpsiegr2::Dnmieg,
        dpsiegr2::Dnmieg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpsiegr2::Dnmieg,
            dpsiegr2::Dnmieg,
            Dpsiegr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 Edge Select"]
    #[inline(always)]
    pub fn dlvd2ieg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpsiegr2::Dlvd2Ieg,
        dpsiegr2::Dlvd2Ieg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpsiegr2::Dlvd2Ieg,
            dpsiegr2::Dlvd2Ieg,
            Dpsiegr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD1 Edge Select"]
    #[inline(always)]
    pub fn dlvd1ieg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpsiegr2::Dlvd1Ieg,
        dpsiegr2::Dlvd1Ieg,
        Dpsiegr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpsiegr2::Dlvd1Ieg,
            dpsiegr2::Dlvd1Ieg,
            Dpsiegr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpsiegr2 {
    #[inline(always)]
    fn default() -> Dpsiegr2 {
        <crate::RegValueT<Dpsiegr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpsiegr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnmieg_SPEC;
    pub type Dnmieg = crate::EnumBitfieldStruct<u8, Dnmieg_SPEC>;
    impl Dnmieg {
        #[doc = "A cancel request is generated at a falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated at a rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd2Ieg_SPEC;
    pub type Dlvd2Ieg = crate::EnumBitfieldStruct<u8, Dlvd2Ieg_SPEC>;
    impl Dlvd2Ieg {
        #[doc = "A cancel request is generated when VCC<Vdet2 (fall) is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated when VCC>=Vdet2 (rise) is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlvd1Ieg_SPEC;
    pub type Dlvd1Ieg = crate::EnumBitfieldStruct<u8, Dlvd1Ieg_SPEC>;
    impl Dlvd1Ieg {
        #[doc = "A cancel request is generated when VCC<Vdet1 (fall) is detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A cancel request is generated when VCC>=Vdet1 (rise) is detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syocdcr_SPEC;
impl crate::sealed::RegSpec for Syocdcr_SPEC {
    type DataType = u8;
}

#[doc = "System Control OCD Control Register"]
pub type Syocdcr = crate::RegValueT<Syocdcr_SPEC>;

impl Syocdcr {
    #[doc = "Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        syocdcr::Dbgen,
        syocdcr::Dbgen,
        Syocdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            syocdcr::Dbgen,
            syocdcr::Dbgen,
            Syocdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Deep Standby OCD flag"]
    #[inline(always)]
    pub fn docdf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syocdcr::Docdf,
        syocdcr::Docdf,
        Syocdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syocdcr::Docdf,
            syocdcr::Docdf,
            Syocdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syocdcr {
    #[inline(always)]
    fn default() -> Syocdcr {
        <crate::RegValueT<Syocdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syocdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgen_SPEC;
    pub type Dbgen = crate::EnumBitfieldStruct<u8, Dbgen_SPEC>;
    impl Dbgen {
        #[doc = "On-chip debugger is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "On-chip debugger is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Docdf_SPEC;
    pub type Docdf = crate::EnumBitfieldStruct<u8, Docdf_SPEC>;
    impl Docdf {
        #[doc = "On-chip debugger is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "On-chip debugger is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stconr_SPEC;
impl crate::sealed::RegSpec for Stconr_SPEC {
    type DataType = u8;
}

#[doc = "Standby Condition Register"]
pub type Stconr = crate::RegValueT<Stconr_SPEC>;

impl Stconr {
    #[doc = "SSTBY condition bit"]
    #[inline(always)]
    pub fn stcon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        stconr::Stcon,
        stconr::Stcon,
        Stconr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            stconr::Stcon,
            stconr::Stcon,
            Stconr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stconr {
    #[inline(always)]
    fn default() -> Stconr {
        <crate::RegValueT<Stconr_SPEC> as RegisterValue<_>>::new(195)
    }
}
pub mod stconr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcon_SPEC;
    pub type Stcon = crate::EnumBitfieldStruct<u8, Stcon_SPEC>;
    impl Stcon {
        #[doc = "set this value in case of transferring to Software Standby Mode in using HOCO."]
        pub const _00: Self = Self::new(0);

        #[doc = "set this value in case of transferring to Software Standby Mode in using expect for HOCO."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdcr1_SPEC;
impl crate::sealed::RegSpec for Lvdcr1_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor %s Circuit Control Register 1"]
pub type Lvdcr1 = crate::RegValueT<Lvdcr1_SPEC>;

impl Lvdcr1 {
    #[doc = "Voltage Monitor %s Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvdcr1::Irqsel,
        lvdcr1::Irqsel,
        Lvdcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvdcr1::Irqsel,
            lvdcr1::Irqsel,
            Lvdcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor %s Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lvdcr1::Idtsel,
        lvdcr1::Idtsel,
        Lvdcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lvdcr1::Idtsel,
            lvdcr1::Idtsel,
            Lvdcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvdcr1 {
    #[inline(always)]
    fn default() -> Lvdcr1 {
        <crate::RegValueT<Lvdcr1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lvdcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqsel_SPEC;
    pub type Irqsel = crate::EnumBitfieldStruct<u8, Irqsel_SPEC>;
    impl Irqsel {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Maskable interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "Generate when VCC>=Vdet (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "Generate when VCC<Vdet (drop) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "Generate when drop and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdsr_SPEC;
impl crate::sealed::RegSpec for Lvdsr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor %s Circuit Status Register"]
pub type Lvdsr = crate::RegValueT<Lvdsr_SPEC>;

impl Lvdsr {
    #[doc = "Voltage Monitor %s Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvdsr::Mon,
        lvdsr::Mon,
        Lvdsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvdsr::Mon,
            lvdsr::Mon,
            Lvdsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor %s Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvdsr::Det,
        lvdsr::Det,
        Lvdsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvdsr::Det,
            lvdsr::Det,
            Lvdsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvdsr {
    #[inline(always)]
    fn default() -> Lvdsr {
        <crate::RegValueT<Lvdsr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod lvdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC < Vdet"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC >= Vdet or MON bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Vdet1 passage detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvcmpcr_SPEC;
impl crate::sealed::RegSpec for Lvcmpcr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor Circuit Control Register"]
pub type Lvcmpcr = crate::RegValueT<Lvcmpcr_SPEC>;

impl Lvcmpcr {
    #[doc = "Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvcmpcr::Lvd2E,
        lvcmpcr::Lvd2E,
        Lvcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvcmpcr::Lvd2E,
            lvcmpcr::Lvd2E,
            Lvcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lvcmpcr::Lvd1E,
        lvcmpcr::Lvd1E,
        Lvcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lvcmpcr::Lvd1E,
            lvcmpcr::Lvd1E,
            Lvcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvcmpcr {
    #[inline(always)]
    fn default() -> Lvcmpcr {
        <crate::RegValueT<Lvcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lvcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2E_SPEC;
    pub type Lvd2E = crate::EnumBitfieldStruct<u8, Lvd2E_SPEC>;
    impl Lvd2E {
        #[doc = "Voltage detection 2 circuit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage detection 2 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1E_SPEC;
    pub type Lvd1E = crate::EnumBitfieldStruct<u8, Lvd1E_SPEC>;
    impl Lvd1E {
        #[doc = "Voltage detection 1 circuit disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage detection 1 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdlvlr_SPEC;
impl crate::sealed::RegSpec for Lvdlvlr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Detection Level Select Register"]
pub type Lvdlvlr = crate::RegValueT<Lvdlvlr_SPEC>;

impl Lvdlvlr {
    #[doc = "Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7,
        1,
        0,
        lvdlvlr::Lvd2Lvl,
        lvdlvlr::Lvd2Lvl,
        Lvdlvlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7,
            1,
            0,
            lvdlvlr::Lvd2Lvl,
            lvdlvlr::Lvd2Lvl,
            Lvdlvlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        lvdlvlr::Lvd1Lvl,
        lvdlvlr::Lvd1Lvl,
        Lvdlvlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            lvdlvlr::Lvd1Lvl,
            lvdlvlr::Lvd1Lvl,
            Lvdlvlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvdlvlr {
    #[inline(always)]
    fn default() -> Lvdlvlr {
        <crate::RegValueT<Lvdlvlr_SPEC> as RegisterValue<_>>::new(243)
    }
}
pub mod lvdlvlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Lvl_SPEC;
    pub type Lvd2Lvl = crate::EnumBitfieldStruct<u8, Lvd2Lvl_SPEC>;
    impl Lvd2Lvl {
        #[doc = "2.99V (Vdet2_1)"]
        pub const _101: Self = Self::new(5);

        #[doc = "2.92V (Vdet2_2)"]
        pub const _110: Self = Self::new(6);

        #[doc = "2.85V (Vdet2_3)"]
        pub const _111: Self = Self::new(7);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Lvl_SPEC;
    pub type Lvd1Lvl = crate::EnumBitfieldStruct<u8, Lvd1Lvl_SPEC>;
    impl Lvd1Lvl {
        #[doc = "2.99V  (Vdet1_1)"]
        pub const _10001: Self = Self::new(17);

        #[doc = "2.92V (Vdet1_2)"]
        pub const _10010: Self = Self::new(18);

        #[doc = "2.85V  (Vdet1_3)"]
        pub const _10011: Self = Self::new(19);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdcr0_SPEC;
impl crate::sealed::RegSpec for Lvdcr0_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor %s Circuit Control Register 0"]
pub type Lvdcr0 = crate::RegValueT<Lvdcr0_SPEC>;

impl Lvdcr0 {
    #[doc = "Voltage Monitor %s Reset Negate Select"]
    #[inline(always)]
    pub fn rn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvdcr0::Rn,
        lvdcr0::Rn,
        Lvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvdcr0::Rn,
            lvdcr0::Rn,
            Lvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor %s Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvdcr0::Ri,
        lvdcr0::Ri,
        Lvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvdcr0::Ri,
            lvdcr0::Ri,
            Lvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sampling Clock Select"]
    #[inline(always)]
    pub fn fsamp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        lvdcr0::Fsamp,
        lvdcr0::Fsamp,
        Lvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            lvdcr0::Fsamp,
            lvdcr0::Fsamp,
            Lvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor %s Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvdcr0::Cmpe,
        lvdcr0::Cmpe,
        Lvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvdcr0::Cmpe,
            lvdcr0::Cmpe,
            Lvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor %s Digital Filter Disable Mode Select"]
    #[inline(always)]
    pub fn dfdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvdcr0::Dfdis,
        lvdcr0::Dfdis,
        Lvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvdcr0::Dfdis,
            lvdcr0::Dfdis,
            Lvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor %s Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvdcr0::Rie,
        lvdcr0::Rie,
        Lvdcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvdcr0::Rie,
            lvdcr0::Rie,
            Lvdcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvdcr0 {
    #[inline(always)]
    fn default() -> Lvdcr0 {
        <crate::RegValueT<Lvdcr0_SPEC> as RegisterValue<_>>::new(130)
    }
}
pub mod lvdcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rn_SPEC;
    pub type Rn = crate::EnumBitfieldStruct<u8, Rn_SPEC>;
    impl Rn {
        #[doc = "Negation follows a stabilization time (tLVD) after VCC > Vdet is detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri_SPEC;
    pub type Ri = crate::EnumBitfieldStruct<u8, Ri_SPEC>;
    impl Ri {
        #[doc = "Voltage Monitor  interrupt during Vdet1 passage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor  reset enabled when the voltage falls to and below Vdet1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsamp_SPEC;
    pub type Fsamp = crate::EnumBitfieldStruct<u8, Fsamp_SPEC>;
    impl Fsamp {
        #[doc = "1/2 LOCO frequency"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/4 LOCO frequency"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/8 LOCO frequency"]
        pub const _10: Self = Self::new(2);

        #[doc = "1/16 LOCO frequency"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Disable voltage monitor 1 circuit comparison result output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage monitor 1 circuit comparison result output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfdis_SPEC;
    pub type Dfdis = crate::EnumBitfieldStruct<u8, Dfdis_SPEC>;
    impl Dfdis {
        #[doc = "Enable digital filter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable digital filter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prcr_SPEC;
impl crate::sealed::RegSpec for Prcr_SPEC {
    type DataType = u16;
}

#[doc = "Protect Register"]
pub type Prcr = crate::RegValueT<Prcr_SPEC>;

impl Prcr {
    #[doc = "PRKEY Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        prcr::Prkey,
        prcr::Prkey,
        Prcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            prcr::Prkey,
            prcr::Prkey,
            Prcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the LVD."]
    #[inline(always)]
    pub fn prc3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        prcr::Prc3,
        prcr::Prc3,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prcr::Prc3,
            prcr::Prc3,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the operating modes,  the low power consumption modes and the battery backup function."]
    #[inline(always)]
    pub fn prc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        prcr::Prc1,
        prcr::Prc1,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            prcr::Prc1,
            prcr::Prc1,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables writing to the registers related to the clock generation circuit."]
    #[inline(always)]
    pub fn prc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        prcr::Prc0,
        prcr::Prc0,
        Prcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prcr::Prc0,
            prcr::Prc0,
            Prcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Prcr {
    #[inline(always)]
    fn default() -> Prcr {
        <crate::RegValueT<Prcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prkey_SPEC;
    pub type Prkey = crate::EnumBitfieldStruct<u8, Prkey_SPEC>;
    impl Prkey {
        #[doc = "Enables writing to the PRCR register."]
        pub const _0_X_5_A: Self = Self::new(90);

        #[doc = "Disables writing to the PRCR register."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc3_SPEC;
    pub type Prc3 = crate::EnumBitfieldStruct<u8, Prc3_SPEC>;
    impl Prc3 {
        #[doc = "Writes protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Writes not protected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc1_SPEC;
    pub type Prc1 = crate::EnumBitfieldStruct<u8, Prc1_SPEC>;
    impl Prc1 {
        #[doc = "Writes protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Writes not protected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc0_SPEC;
    pub type Prc0 = crate::EnumBitfieldStruct<u8, Prc0_SPEC>;
    impl Prc0 {
        #[doc = "Writes protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Writes not protected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr0_SPEC;
impl crate::sealed::RegSpec for Rstsr0_SPEC {
    type DataType = u8;
}

#[doc = "Reset Status Register 0"]
pub type Rstsr0 = crate::RegValueT<Rstsr0_SPEC>;

impl Rstsr0 {
    #[doc = "Deep Software Standby Reset FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn dpsrstf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rstsr0::Dpsrstf,
        rstsr0::Dpsrstf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rstsr0::Dpsrstf,
            rstsr0::Dpsrstf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lvd2rf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rstsr0::Lvd2Rf,
        rstsr0::Lvd2Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rstsr0::Lvd2Rf,
            rstsr0::Lvd2Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lvd1rf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstsr0::Lvd1Rf,
        rstsr0::Lvd1Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstsr0::Lvd1Rf,
            rstsr0::Lvd1Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 0 Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn lvd0rf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsr0::Lvd0Rf,
        rstsr0::Lvd0Rf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsr0::Lvd0Rf,
            rstsr0::Lvd0Rf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Power-On Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn porf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsr0::Porf,
        rstsr0::Porf,
        Rstsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsr0::Porf,
            rstsr0::Porf,
            Rstsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsr0 {
    #[inline(always)]
    fn default() -> Rstsr0 {
        <crate::RegValueT<Rstsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpsrstf_SPEC;
    pub type Dpsrstf = crate::EnumBitfieldStruct<u8, Dpsrstf_SPEC>;
    impl Dpsrstf {
        #[doc = "Deep software standby mode cancelation not requested by an interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep software standby mode cancelation requested by an interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Rf_SPEC;
    pub type Lvd2Rf = crate::EnumBitfieldStruct<u8, Lvd2Rf_SPEC>;
    impl Lvd2Rf {
        #[doc = "Voltage Monitor 2 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 2 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Rf_SPEC;
    pub type Lvd1Rf = crate::EnumBitfieldStruct<u8, Lvd1Rf_SPEC>;
    impl Lvd1Rf {
        #[doc = "Voltage Monitor 1 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 1 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd0Rf_SPEC;
    pub type Lvd0Rf = crate::EnumBitfieldStruct<u8, Lvd0Rf_SPEC>;
    impl Lvd0Rf {
        #[doc = "Voltage Monitor 0 reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage Monitor 0 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porf_SPEC;
    pub type Porf = crate::EnumBitfieldStruct<u8, Porf_SPEC>;
    impl Porf {
        #[doc = "Power-on reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Power-on reset detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr2_SPEC;
impl crate::sealed::RegSpec for Rstsr2_SPEC {
    type DataType = u8;
}

#[doc = "Reset Status Register 2"]
pub type Rstsr2 = crate::RegValueT<Rstsr2_SPEC>;

impl Rstsr2 {
    #[doc = "Cold/Warm Start Determination Flag"]
    #[inline(always)]
    pub fn cwsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsr2::Cwsf,
        rstsr2::Cwsf,
        Rstsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsr2::Cwsf,
            rstsr2::Cwsf,
            Rstsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsr2 {
    #[inline(always)]
    fn default() -> Rstsr2 {
        <crate::RegValueT<Rstsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cwsf_SPEC;
    pub type Cwsf = crate::EnumBitfieldStruct<u8, Cwsf_SPEC>;
    impl Cwsf {
        #[doc = "Cold start"]
        pub const _0: Self = Self::new(0);

        #[doc = "Warm start"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr1_SPEC;
impl crate::sealed::RegSpec for Rstsr1_SPEC {
    type DataType = u16;
}

#[doc = "Reset Status Register 1"]
pub type Rstsr1 = crate::RegValueT<Rstsr1_SPEC>;

impl Rstsr1 {
    #[doc = "SP Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn sperf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        rstsr1::Sperf,
        rstsr1::Sperf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rstsr1::Sperf,
            rstsr1::Sperf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Master MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn busmrf(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        rstsr1::Busmrf,
        rstsr1::Busmrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            rstsr1::Busmrf,
            rstsr1::Busmrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Slave MPU Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn bussrf(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rstsr1::Bussrf,
        rstsr1::Bussrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rstsr1::Bussrf,
            rstsr1::Bussrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM ECC Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn reerf(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        rstsr1::Reerf,
        rstsr1::Reerf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            rstsr1::Reerf,
            rstsr1::Reerf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Parity Error Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn rperf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        rstsr1::Rperf,
        rstsr1::Rperf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            rstsr1::Rperf,
            rstsr1::Rperf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn swrf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstsr1::Swrf,
        rstsr1::Swrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstsr1::Swrf,
            rstsr1::Swrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn wdtrf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstsr1::Wdtrf,
        rstsr1::Wdtrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstsr1::Wdtrf,
            rstsr1::Wdtrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Independent Watchdog Timer Reset Detect FlagNOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn iwdtrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstsr1::Iwdtrf,
        rstsr1::Iwdtrf,
        Rstsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstsr1::Iwdtrf,
            rstsr1::Iwdtrf,
            Rstsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstsr1 {
    #[inline(always)]
    fn default() -> Rstsr1 {
        <crate::RegValueT<Rstsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sperf_SPEC;
    pub type Sperf = crate::EnumBitfieldStruct<u8, Sperf_SPEC>;
    impl Sperf {
        #[doc = "SP error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "SP error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmrf_SPEC;
    pub type Busmrf = crate::EnumBitfieldStruct<u8, Busmrf_SPEC>;
    impl Busmrf {
        #[doc = "Bus Master MPU reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus Master MPU reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussrf_SPEC;
    pub type Bussrf = crate::EnumBitfieldStruct<u8, Bussrf_SPEC>;
    impl Bussrf {
        #[doc = "Bus Slave MPU reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus Slave MPU reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reerf_SPEC;
    pub type Reerf = crate::EnumBitfieldStruct<u8, Reerf_SPEC>;
    impl Reerf {
        #[doc = "RAM ECC error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "RAM ECC error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rperf_SPEC;
    pub type Rperf = crate::EnumBitfieldStruct<u8, Rperf_SPEC>;
    impl Rperf {
        #[doc = "RAM parity error reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "RAM parity error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrf_SPEC;
    pub type Swrf = crate::EnumBitfieldStruct<u8, Swrf_SPEC>;
    impl Swrf {
        #[doc = "Software reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtrf_SPEC;
    pub type Wdtrf = crate::EnumBitfieldStruct<u8, Wdtrf_SPEC>;
    impl Wdtrf {
        #[doc = "Watchdog timer reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Watchdog timer reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtrf_SPEC;
    pub type Iwdtrf = crate::EnumBitfieldStruct<u8, Iwdtrf_SPEC>;
    impl Iwdtrf {
        #[doc = "Independent watchdog timer reset not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Independent watchdog timer reset detected."]
        pub const _1: Self = Self::new(1);
    }
}
