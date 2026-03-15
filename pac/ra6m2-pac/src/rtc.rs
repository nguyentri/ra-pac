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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:11:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Realtime Clock"]
unsafe impl ::core::marker::Send for super::Rtc {}
unsafe impl ::core::marker::Sync for super::Rtc {}
impl super::Rtc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "64-Hz Counter"]
    #[inline(always)]
    pub const fn r64cnt(&self) -> &'static crate::common::Reg<self::R64Cnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::R64Cnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Second Counter"]
    #[inline(always)]
    pub const fn rseccnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rseccnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rseccnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Binary Counter 0"]
    #[inline(always)]
    pub const fn bcnt0(&self) -> &'static crate::common::Reg<self::Bcnt0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Minute Counter"]
    #[inline(always)]
    pub const fn rmincnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rmincnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmincnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Binary Counter 1"]
    #[inline(always)]
    pub const fn bcnt1(&self) -> &'static crate::common::Reg<self::Bcnt1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Hour Counter"]
    #[inline(always)]
    pub const fn rhrcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rhrcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rhrcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Binary Counter 2"]
    #[inline(always)]
    pub const fn bcnt2(&self) -> &'static crate::common::Reg<self::Bcnt2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Day-of-Week Counter"]
    #[inline(always)]
    pub const fn rwkcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rwkcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rwkcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Binary Counter 3"]
    #[inline(always)]
    pub const fn bcnt3(&self) -> &'static crate::common::Reg<self::Bcnt3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Day Counter"]
    #[inline(always)]
    pub const fn rdaycnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rdaycnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdaycnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Month Counter"]
    #[inline(always)]
    pub const fn rmoncnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rmoncnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmoncnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Year Counter"]
    #[inline(always)]
    pub const fn ryrcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Ryrcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ryrcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Second Alarm Register"]
    #[inline(always)]
    pub const fn rsecar(
        &self,
    ) -> &'static crate::common::Reg<self::Rsecar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rsecar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Binary Counter 0 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt0ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt0Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt0Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Minute Alarm Register"]
    #[inline(always)]
    pub const fn rminar(
        &self,
    ) -> &'static crate::common::Reg<self::Rminar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rminar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Binary Counter 1 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt1ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt1Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt1Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Hour Alarm Register"]
    #[inline(always)]
    pub const fn rhrar(&self) -> &'static crate::common::Reg<self::Rhrar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rhrar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Binary Counter 2 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt2ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt2Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt2Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Day-of-Week Alarm Register"]
    #[inline(always)]
    pub const fn rwkar(&self) -> &'static crate::common::Reg<self::Rwkar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rwkar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Binary Counter 3 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt3ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt3Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt3Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Date Alarm Register"]
    #[inline(always)]
    pub const fn rdayar(
        &self,
    ) -> &'static crate::common::Reg<self::Rdayar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdayar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Binary Counter 0 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt0aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt0Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt0Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Month Alarm Register"]
    #[inline(always)]
    pub const fn rmonar(
        &self,
    ) -> &'static crate::common::Reg<self::Rmonar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmonar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "Binary Counter 1 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt1aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt1Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt1Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "Year Alarm Register"]
    #[inline(always)]
    pub const fn ryrar(&self) -> &'static crate::common::Reg<self::Ryrar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ryrar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Binary Counter 2 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt2aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt2Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt2Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Year Alarm Enable Register"]
    #[inline(always)]
    pub const fn ryraren(
        &self,
    ) -> &'static crate::common::Reg<self::Ryraren_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ryraren_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "Binary Counter 3 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt3aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt3Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt3Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "RTC Control Register 1"]
    #[inline(always)]
    pub const fn rcr1(&self) -> &'static crate::common::Reg<self::Rcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "RTC Control Register 2"]
    #[inline(always)]
    pub const fn rcr2(&self) -> &'static crate::common::Reg<self::Rcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "RTC Control Register 4"]
    #[inline(always)]
    pub const fn rcr4(&self) -> &'static crate::common::Reg<self::Rcr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Frequency Register H"]
    #[inline(always)]
    pub const fn rfrh(&self) -> &'static crate::common::Reg<self::Rfrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "Frequency Register L"]
    #[inline(always)]
    pub const fn rfrl(&self) -> &'static crate::common::Reg<self::Rfrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Time Error Adjustment Register"]
    #[inline(always)]
    pub const fn radj(&self) -> &'static crate::common::Reg<self::Radj_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Radj_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "Time Capture Control Register %s"]
    #[inline(always)]
    pub const fn rtccr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rtccr_SPEC, crate::common::RW>,
        3,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn rtccr0(&self) -> &'static crate::common::Reg<self::Rtccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rtccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rtccr1(&self) -> &'static crate::common::Reg<self::Rtccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rtccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rtccr2(&self) -> &'static crate::common::Reg<self::Rtccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rtccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }

    #[doc = "Second Capture Register %s"]
    #[inline(always)]
    pub const fn rseccp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rseccp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x52usize))
        }
    }
    #[inline(always)]
    pub const fn rseccp0(
        &self,
    ) -> &'static crate::common::Reg<self::Rseccp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rseccp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x52usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rseccp1(
        &self,
    ) -> &'static crate::common::Reg<self::Rseccp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rseccp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x62usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rseccp2(
        &self,
    ) -> &'static crate::common::Reg<self::Rseccp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rseccp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x72usize),
            )
        }
    }

    #[doc = "BCNT0 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt0cp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bcnt0Cp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x52usize))
        }
    }
    #[inline(always)]
    pub const fn bcnt0cp0(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt0Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt0Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x52usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt0cp1(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt0Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt0Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x62usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt0cp2(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt0Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt0Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x72usize),
            )
        }
    }

    #[doc = "Minute Capture Register %s"]
    #[inline(always)]
    pub const fn rmincp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rmincp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x54usize))
        }
    }
    #[inline(always)]
    pub const fn rmincp0(
        &self,
    ) -> &'static crate::common::Reg<self::Rmincp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rmincp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rmincp1(
        &self,
    ) -> &'static crate::common::Reg<self::Rmincp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rmincp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rmincp2(
        &self,
    ) -> &'static crate::common::Reg<self::Rmincp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rmincp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }

    #[doc = "BCNT1 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt1cp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bcnt1Cp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x54usize))
        }
    }
    #[inline(always)]
    pub const fn bcnt1cp0(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt1Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt1Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt1cp1(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt1Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt1Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt1cp2(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt1Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt1Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }

    #[doc = "Hour Capture Register %s"]
    #[inline(always)]
    pub const fn rhrcp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rhrcp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x56usize))
        }
    }
    #[inline(always)]
    pub const fn rhrcp0(&self) -> &'static crate::common::Reg<self::Rhrcp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rhrcp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x56usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rhrcp1(&self) -> &'static crate::common::Reg<self::Rhrcp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rhrcp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x66usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rhrcp2(&self) -> &'static crate::common::Reg<self::Rhrcp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rhrcp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x76usize),
            )
        }
    }

    #[doc = "BCNT2 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt2cp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bcnt2Cp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x56usize))
        }
    }
    #[inline(always)]
    pub const fn bcnt2cp0(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt2Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt2Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x56usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt2cp1(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt2Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt2Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x66usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt2cp2(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt2Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt2Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x76usize),
            )
        }
    }

    #[doc = "Date Capture Register %s"]
    #[inline(always)]
    pub const fn rdaycp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rdaycp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5ausize))
        }
    }
    #[inline(always)]
    pub const fn rdaycp0(
        &self,
    ) -> &'static crate::common::Reg<self::Rdaycp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdaycp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn rdaycp1(
        &self,
    ) -> &'static crate::common::Reg<self::Rdaycp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdaycp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn rdaycp2(
        &self,
    ) -> &'static crate::common::Reg<self::Rdaycp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdaycp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ausize),
            )
        }
    }

    #[doc = "BCNT3 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt3cp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bcnt3Cp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5ausize))
        }
    }
    #[inline(always)]
    pub const fn bcnt3cp0(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt3Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt3Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt3cp1(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt3Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt3Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt3cp2(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt3Cp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcnt3Cp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ausize),
            )
        }
    }

    #[doc = "Month Capture Register %s"]
    #[inline(always)]
    pub const fn rmoncp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rmoncp_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5cusize))
        }
    }
    #[inline(always)]
    pub const fn rmoncp0(
        &self,
    ) -> &'static crate::common::Reg<self::Rmoncp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rmoncp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn rmoncp1(
        &self,
    ) -> &'static crate::common::Reg<self::Rmoncp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rmoncp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn rmoncp2(
        &self,
    ) -> &'static crate::common::Reg<self::Rmoncp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rmoncp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R64Cnt_SPEC;
impl crate::sealed::RegSpec for R64Cnt_SPEC {
    type DataType = u8;
}

#[doc = "64-Hz Counter"]
pub type R64Cnt = crate::RegValueT<R64Cnt_SPEC>;

impl R64Cnt {
    #[doc = "1Hz"]
    #[inline(always)]
    pub fn f1hz(self) -> crate::common::RegisterFieldBool<6, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "2Hz"]
    #[inline(always)]
    pub fn f2hz(self) -> crate::common::RegisterFieldBool<5, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "4Hz"]
    #[inline(always)]
    pub fn f4hz(self) -> crate::common::RegisterFieldBool<4, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "8Hz"]
    #[inline(always)]
    pub fn f8hz(self) -> crate::common::RegisterFieldBool<3, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "16Hz"]
    #[inline(always)]
    pub fn f16hz(self) -> crate::common::RegisterFieldBool<2, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "32Hz"]
    #[inline(always)]
    pub fn f32hz(self) -> crate::common::RegisterFieldBool<1, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "64Hz"]
    #[inline(always)]
    pub fn f64hz(self) -> crate::common::RegisterFieldBool<0, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for R64Cnt {
    #[inline(always)]
    fn default() -> R64Cnt {
        <crate::RegValueT<R64Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rseccnt_SPEC;
impl crate::sealed::RegSpec for Rseccnt_SPEC {
    type DataType = u8;
}

#[doc = "Second Counter"]
pub type Rseccnt = crate::RegValueT<Rseccnt_SPEC>;

impl Rseccnt {
    #[doc = "10-Second Count Counts from 0 to 5 for 60-second counting."]
    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rseccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rseccnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Second Count Counts from 0 to 9 every second. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rseccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rseccnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rseccnt {
    #[inline(always)]
    fn default() -> Rseccnt {
        <crate::RegValueT<Rseccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0_SPEC;
impl crate::sealed::RegSpec for Bcnt0_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 0"]
pub type Bcnt0 = crate::RegValueT<Bcnt0_SPEC>;

impl Bcnt0 {
    #[doc = "The BCNT0 counter is a readable/writable 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt0 {
    #[inline(always)]
    fn default() -> Bcnt0 {
        <crate::RegValueT<Bcnt0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmincnt_SPEC;
impl crate::sealed::RegSpec for Rmincnt_SPEC {
    type DataType = u8;
}

#[doc = "Minute Counter"]
pub type Rmincnt = crate::RegValueT<Rmincnt_SPEC>;

impl Rmincnt {
    #[doc = "10-Minute Count Counts from 0 to 5 for 60-minute counting."]
    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rmincnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rmincnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmincnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmincnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmincnt {
    #[inline(always)]
    fn default() -> Rmincnt {
        <crate::RegValueT<Rmincnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1_SPEC;
impl crate::sealed::RegSpec for Bcnt1_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 1"]
pub type Bcnt1 = crate::RegValueT<Bcnt1_SPEC>;

impl Bcnt1 {
    #[doc = "The BCNT1 counter is a readable/writable 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt1 {
    #[inline(always)]
    fn default() -> Bcnt1 {
        <crate::RegValueT<Bcnt1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrcnt_SPEC;
impl crate::sealed::RegSpec for Rhrcnt_SPEC {
    type DataType = u8;
}

#[doc = "Hour Counter"]
pub type Rhrcnt = crate::RegValueT<Rhrcnt_SPEC>;

impl Rhrcnt {
    #[doc = "Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rhrcnt::Pm,
        rhrcnt::Pm,
        Rhrcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rhrcnt::Pm,
            rhrcnt::Pm,
            Rhrcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
    #[inline(always)]
    pub fn hr10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rhrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rhrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn hr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rhrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rhrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rhrcnt {
    #[inline(always)]
    fn default() -> Rhrcnt {
        <crate::RegValueT<Rhrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rhrcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "a.m."]
        pub const _0: Self = Self::new(0);

        #[doc = "p.m."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2_SPEC;
impl crate::sealed::RegSpec for Bcnt2_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 2"]
pub type Bcnt2 = crate::RegValueT<Bcnt2_SPEC>;

impl Bcnt2 {
    #[doc = "The BCNT2 counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt2 {
    #[inline(always)]
    fn default() -> Bcnt2 {
        <crate::RegValueT<Bcnt2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkcnt_SPEC;
impl crate::sealed::RegSpec for Rwkcnt_SPEC {
    type DataType = u8;
}

#[doc = "Day-of-Week Counter"]
pub type Rwkcnt = crate::RegValueT<Rwkcnt_SPEC>;

impl Rwkcnt {
    #[doc = "Day-of-Week Counting"]
    #[inline(always)]
    pub fn dayw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        rwkcnt::Dayw,
        rwkcnt::Dayw,
        Rwkcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            rwkcnt::Dayw,
            rwkcnt::Dayw,
            Rwkcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rwkcnt {
    #[inline(always)]
    fn default() -> Rwkcnt {
        <crate::RegValueT<Rwkcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rwkcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dayw_SPEC;
    pub type Dayw = crate::EnumBitfieldStruct<u8, Dayw_SPEC>;
    impl Dayw {
        #[doc = "Sunday"]
        pub const _000: Self = Self::new(0);

        #[doc = "Monday"]
        pub const _001: Self = Self::new(1);

        #[doc = "Tuesday"]
        pub const _010: Self = Self::new(2);

        #[doc = "Wednesday"]
        pub const _011: Self = Self::new(3);

        #[doc = "Thursday"]
        pub const _100: Self = Self::new(4);

        #[doc = "Friday"]
        pub const _101: Self = Self::new(5);

        #[doc = "Saturday"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting Prohibited"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3_SPEC;
impl crate::sealed::RegSpec for Bcnt3_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 3"]
pub type Bcnt3 = crate::RegValueT<Bcnt3_SPEC>;

impl Bcnt3 {
    #[doc = "The BCNT3 counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt3 {
    #[inline(always)]
    fn default() -> Bcnt3 {
        <crate::RegValueT<Bcnt3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdaycnt_SPEC;
impl crate::sealed::RegSpec for Rdaycnt_SPEC {
    type DataType = u8;
}

#[doc = "Day Counter"]
pub type Rdaycnt = crate::RegValueT<Rdaycnt_SPEC>;

impl Rdaycnt {
    #[doc = "10-Day Count Counts from 0 to 3 once per carry from the ones place."]
    #[inline(always)]
    pub fn date10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rdaycnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rdaycnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Day Count Counts from 0 to 9 once per day. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn date1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rdaycnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rdaycnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdaycnt {
    #[inline(always)]
    fn default() -> Rdaycnt {
        <crate::RegValueT<Rdaycnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmoncnt_SPEC;
impl crate::sealed::RegSpec for Rmoncnt_SPEC {
    type DataType = u8;
}

#[doc = "Month Counter"]
pub type Rmoncnt = crate::RegValueT<Rmoncnt_SPEC>;

impl Rmoncnt {
    #[doc = "10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    pub fn mon10(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Rmoncnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmoncnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn mon1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmoncnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmoncnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmoncnt {
    #[inline(always)]
    fn default() -> Rmoncnt {
        <crate::RegValueT<Rmoncnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryrcnt_SPEC;
impl crate::sealed::RegSpec for Ryrcnt_SPEC {
    type DataType = u16;
}

#[doc = "Year Counter"]
pub type Ryrcnt = crate::RegValueT<Ryrcnt_SPEC>;

impl Ryrcnt {
    #[doc = "10-Year Count Counts from 0 to 9 once per carry from ones place. When a carry is generated in the tens place, 1 is added to the hundreds place."]
    #[inline(always)]
    pub fn yr10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ryrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ryrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Year Count Counts from 0 to 9 once per year. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn yr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ryrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ryrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ryrcnt {
    #[inline(always)]
    fn default() -> Ryrcnt {
        <crate::RegValueT<Ryrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsecar_SPEC;
impl crate::sealed::RegSpec for Rsecar_SPEC {
    type DataType = u8;
}

#[doc = "Second Alarm Register"]
pub type Rsecar = crate::RegValueT<Rsecar_SPEC>;

impl Rsecar {
    #[doc = "Compare enable"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rsecar::Enb,
        rsecar::Enb,
        Rsecar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rsecar::Enb,
            rsecar::Enb,
            Rsecar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "10-Seconds Value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rsecar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rsecar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Second Value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rsecar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rsecar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rsecar {
    #[inline(always)]
    fn default() -> Rsecar {
        <crate::RegValueT<Rsecar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rsecar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "The register value is not compared with the RSECCNT counter value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The register value is compared with the RSECCNT counter value."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt0Ar_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 0 Alarm Register"]
pub type Bcnt0Ar = crate::RegValueT<Bcnt0Ar_SPEC>;

impl Bcnt0Ar {
    #[doc = "he BCNT0AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn bcnt0ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt0Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt0Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt0Ar {
    #[inline(always)]
    fn default() -> Bcnt0Ar {
        <crate::RegValueT<Bcnt0Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rminar_SPEC;
impl crate::sealed::RegSpec for Rminar_SPEC {
    type DataType = u8;
}

#[doc = "Minute Alarm Register"]
pub type Rminar = crate::RegValueT<Rminar_SPEC>;

impl Rminar {
    #[doc = "Compare enable"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rminar::Enb,
        rminar::Enb,
        Rminar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rminar::Enb,
            rminar::Enb,
            Rminar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "10-Minute Count Value for the tens place of minutes"]
    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rminar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rminar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Minute Count Value for the ones place of minutes"]
    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rminar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rminar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rminar {
    #[inline(always)]
    fn default() -> Rminar {
        <crate::RegValueT<Rminar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rminar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "The register value is not compared with the RMINCNT counter value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The register value is compared with the RMINCNT counter value."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt1Ar_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 1 Alarm Register"]
pub type Bcnt1Ar = crate::RegValueT<Bcnt1Ar_SPEC>;

impl Bcnt1Ar {
    #[doc = "he BCNT1AR counter is a readable/writable alarm register corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn bcnt1ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt1Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt1Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt1Ar {
    #[inline(always)]
    fn default() -> Bcnt1Ar {
        <crate::RegValueT<Bcnt1Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrar_SPEC;
impl crate::sealed::RegSpec for Rhrar_SPEC {
    type DataType = u8;
}

#[doc = "Hour Alarm Register"]
pub type Rhrar = crate::RegValueT<Rhrar_SPEC>;

impl Rhrar {
    #[doc = "Compare enable"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rhrar::Enb,
        rhrar::Enb,
        Rhrar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rhrar::Enb,
            rhrar::Enb,
            Rhrar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rhrar::Pm,
        rhrar::Pm,
        Rhrar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rhrar::Pm,
            rhrar::Pm,
            Rhrar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "10-Hour Count Value for the tens place of hours"]
    #[inline(always)]
    pub fn hr10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rhrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rhrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1-Hour Count Value for the ones place of hours"]
    #[inline(always)]
    pub fn hr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rhrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rhrar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rhrar {
    #[inline(always)]
    fn default() -> Rhrar {
        <crate::RegValueT<Rhrar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rhrar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "The register value is not compared with the RHRCNT counter value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The register value is compared with the RHRCNT counter value."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "a.m."]
        pub const _0: Self = Self::new(0);

        #[doc = "p.m."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt2Ar_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 2 Alarm Register"]
pub type Bcnt2Ar = crate::RegValueT<Bcnt2Ar_SPEC>;

impl Bcnt2Ar {
    #[doc = "The BCNT2AR counter is a readable/writable 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn bcnt2ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt2Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt2Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt2Ar {
    #[inline(always)]
    fn default() -> Bcnt2Ar {
        <crate::RegValueT<Bcnt2Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkar_SPEC;
impl crate::sealed::RegSpec for Rwkar_SPEC {
    type DataType = u8;
}

#[doc = "Day-of-Week Alarm Register"]
pub type Rwkar = crate::RegValueT<Rwkar_SPEC>;

impl Rwkar {
    #[doc = "Compare enable"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rwkar::Enb,
        rwkar::Enb,
        Rwkar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rwkar::Enb,
            rwkar::Enb,
            Rwkar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Day-of-Week Counting"]
    #[inline(always)]
    pub fn dayw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        rwkar::Dayw,
        rwkar::Dayw,
        Rwkar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            rwkar::Dayw,
            rwkar::Dayw,
            Rwkar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rwkar {
    #[inline(always)]
    fn default() -> Rwkar {
        <crate::RegValueT<Rwkar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rwkar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "The register value is not compared with the RWKCNT counter value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The register value is compared with the RWKCNT counter value."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dayw_SPEC;
    pub type Dayw = crate::EnumBitfieldStruct<u8, Dayw_SPEC>;
    impl Dayw {
        #[doc = "Sunday"]
        pub const _000: Self = Self::new(0);

        #[doc = "Monday"]
        pub const _001: Self = Self::new(1);

        #[doc = "Tuesday"]
        pub const _010: Self = Self::new(2);

        #[doc = "Wednesday"]
        pub const _011: Self = Self::new(3);

        #[doc = "Thursday"]
        pub const _100: Self = Self::new(4);

        #[doc = "Friday"]
        pub const _101: Self = Self::new(5);

        #[doc = "Saturday"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting Prohibited"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt3Ar_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 3 Alarm Register"]
pub type Bcnt3Ar = crate::RegValueT<Bcnt3Ar_SPEC>;

impl Bcnt3Ar {
    #[doc = "The BCNT3AR counter is a readable/writable 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn bcnt3ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt3Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt3Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt3Ar {
    #[inline(always)]
    fn default() -> Bcnt3Ar {
        <crate::RegValueT<Bcnt3Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdayar_SPEC;
impl crate::sealed::RegSpec for Rdayar_SPEC {
    type DataType = u8;
}

#[doc = "Date Alarm Register"]
pub type Rdayar = crate::RegValueT<Rdayar_SPEC>;

impl Rdayar {
    #[doc = "Compare enable"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rdayar::Enb,
        rdayar::Enb,
        Rdayar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rdayar::Enb,
            rdayar::Enb,
            Rdayar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "10 Days Value for the tens place of days"]
    #[inline(always)]
    pub fn date10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rdayar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rdayar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1 Day Value for the ones place of days"]
    #[inline(always)]
    pub fn date1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rdayar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rdayar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdayar {
    #[inline(always)]
    fn default() -> Rdayar {
        <crate::RegValueT<Rdayar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdayar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "The register value is not compared with the RDAYCNT counter value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The register value is compared with the RDAYCNT counter value."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt0Aer_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 0 Alarm Enable Register"]
pub type Bcnt0Aer = crate::RegValueT<Bcnt0Aer_SPEC>;

impl Bcnt0Aer {
    #[doc = "The BCNT0AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b7 to b0."]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt0Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt0Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt0Aer {
    #[inline(always)]
    fn default() -> Bcnt0Aer {
        <crate::RegValueT<Bcnt0Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmonar_SPEC;
impl crate::sealed::RegSpec for Rmonar_SPEC {
    type DataType = u8;
}

#[doc = "Month Alarm Register"]
pub type Rmonar = crate::RegValueT<Rmonar_SPEC>;

impl Rmonar {
    #[doc = "Compare enable"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rmonar::Enb,
        rmonar::Enb,
        Rmonar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rmonar::Enb,
            rmonar::Enb,
            Rmonar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "10 Months Value for the tens place of months"]
    #[inline(always)]
    pub fn mon10(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Rmonar_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmonar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "1 Month Value for the ones place of months"]
    #[inline(always)]
    pub fn mon1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmonar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmonar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmonar {
    #[inline(always)]
    fn default() -> Rmonar {
        <crate::RegValueT<Rmonar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmonar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "The register value is not compared with the RMONCNT counter value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The register value is compared with the RMONCNT counter value."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt1Aer_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 1 Alarm Enable Register"]
pub type Bcnt1Aer = crate::RegValueT<Bcnt1Aer_SPEC>;

impl Bcnt1Aer {
    #[doc = "The BCNT1AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b15 to b8."]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt1Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt1Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt1Aer {
    #[inline(always)]
    fn default() -> Bcnt1Aer {
        <crate::RegValueT<Bcnt1Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryrar_SPEC;
impl crate::sealed::RegSpec for Ryrar_SPEC {
    type DataType = u16;
}

#[doc = "Year Alarm Register"]
pub type Ryrar = crate::RegValueT<Ryrar_SPEC>;

impl Ryrar {
    #[doc = "10 Years Value for the tens place of years"]
    #[inline(always)]
    pub fn yr10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ryrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ryrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1 Year Value for the ones place of years"]
    #[inline(always)]
    pub fn yr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ryrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ryrar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ryrar {
    #[inline(always)]
    fn default() -> Ryrar {
        <crate::RegValueT<Ryrar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt2Aer_SPEC {
    type DataType = u16;
}

#[doc = "Binary Counter 2 Alarm Enable Register"]
pub type Bcnt2Aer = crate::RegValueT<Bcnt2Aer_SPEC>;

impl Bcnt2Aer {
    #[doc = "The BCNT2AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b23 to b16."]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt2Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt2Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt2Aer {
    #[inline(always)]
    fn default() -> Bcnt2Aer {
        <crate::RegValueT<Bcnt2Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryraren_SPEC;
impl crate::sealed::RegSpec for Ryraren_SPEC {
    type DataType = u8;
}

#[doc = "Year Alarm Enable Register"]
pub type Ryraren = crate::RegValueT<Ryraren_SPEC>;

impl Ryraren {
    #[doc = "Compare enable"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ryraren::Enb,
        ryraren::Enb,
        Ryraren_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ryraren::Enb,
            ryraren::Enb,
            Ryraren_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ryraren {
    #[inline(always)]
    fn default() -> Ryraren {
        <crate::RegValueT<Ryraren_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ryraren {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "The register value is not compared with the RYRCNT counter value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The register value is compared with the RYRCNT counter value."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt3Aer_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 3 Alarm Enable Register"]
pub type Bcnt3Aer = crate::RegValueT<Bcnt3Aer_SPEC>;

impl Bcnt3Aer {
    #[doc = "The BCNT3AER register is a readable/writable register for setting the alarm enable corresponding to 32-bit binary counter b31 to b24."]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt3Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt3Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt3Aer {
    #[inline(always)]
    fn default() -> Bcnt3Aer {
        <crate::RegValueT<Bcnt3Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr1_SPEC;
impl crate::sealed::RegSpec for Rcr1_SPEC {
    type DataType = u8;
}

#[doc = "RTC Control Register 1"]
pub type Rcr1 = crate::RegValueT<Rcr1_SPEC>;

impl Rcr1 {
    #[doc = "Periodic Interrupt Select"]
    #[inline(always)]
    pub fn pes(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        rcr1::Pes,
        rcr1::Pes,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            rcr1::Pes,
            rcr1::Pes,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTCOUT Output Select"]
    #[inline(always)]
    pub fn rtcos(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rcr1::Rtcos,
        rcr1::Rtcos,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rcr1::Rtcos,
            rcr1::Rtcos,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Periodic Interrupt Enable"]
    #[inline(always)]
    pub fn pie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rcr1::Pie,
        rcr1::Pie,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rcr1::Pie,
            rcr1::Pie,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Carry Interrupt Enable"]
    #[inline(always)]
    pub fn cie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rcr1::Cie,
        rcr1::Cie,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rcr1::Cie,
            rcr1::Cie,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn aie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr1::Aie,
        rcr1::Aie,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr1::Aie,
            rcr1::Aie,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        <crate::RegValueT<Rcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pes_SPEC;
    pub type Pes = crate::EnumBitfieldStruct<u8, Pes_SPEC>;
    impl Pes {
        #[doc = "A periodic interrupt is generated every 1/256 second((RCR4.RCKSEL = 0)./A periodic interrupt is generated every 1/128 second((RCR4.RCKSEL = 1)."]
        pub const _0110: Self = Self::new(6);

        #[doc = "A periodic interrupt is generated every 1/128 second."]
        pub const _0111: Self = Self::new(7);

        #[doc = "A periodic interrupt is generated every 1/64 second."]
        pub const _1000: Self = Self::new(8);

        #[doc = "A periodic interrupt is generated every 1/32 second."]
        pub const _1001: Self = Self::new(9);

        #[doc = "A periodic interrupt is generated every 1/16 second."]
        pub const _1010: Self = Self::new(10);

        #[doc = "A periodic interrupt is generated every 1/8 second."]
        pub const _1011: Self = Self::new(11);

        #[doc = "A periodic interrupt is generated every 1/4 second."]
        pub const _1100: Self = Self::new(12);

        #[doc = "A periodic interrupt is generated every 1/2 second."]
        pub const _1101: Self = Self::new(13);

        #[doc = "A periodic interrupt is generated every 1 second."]
        pub const _1110: Self = Self::new(14);

        #[doc = "A periodic interrupt is generated every 2 seconds."]
        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcos_SPEC;
    pub type Rtcos = crate::EnumBitfieldStruct<u8, Rtcos_SPEC>;
    impl Rtcos {
        #[doc = "RTCOUT outputs 1 Hz."]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCOUT outputs 64 Hz."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pie_SPEC;
    pub type Pie = crate::EnumBitfieldStruct<u8, Pie_SPEC>;
    impl Pie {
        #[doc = "A periodic interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A periodic interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cie_SPEC;
    pub type Cie = crate::EnumBitfieldStruct<u8, Cie_SPEC>;
    impl Cie {
        #[doc = "A carry interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "A carry interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aie_SPEC;
    pub type Aie = crate::EnumBitfieldStruct<u8, Aie_SPEC>;
    impl Aie {
        #[doc = "An alarm interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "An alarm interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2_SPEC;
impl crate::sealed::RegSpec for Rcr2_SPEC {
    type DataType = u8;
}

#[doc = "RTC Control Register 2"]
pub type Rcr2 = crate::RegValueT<Rcr2_SPEC>;

impl Rcr2 {
    #[doc = "Count Mode Select"]
    #[inline(always)]
    pub fn cntmd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rcr2::Cntmd,
        rcr2::Cntmd,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rcr2::Cntmd,
            rcr2::Cntmd,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Hours Mode"]
    #[inline(always)]
    pub fn hr24(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rcr2::Hr24,
        rcr2::Hr24,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rcr2::Hr24,
            rcr2::Hr24,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadjp(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rcr2::Aadjp,
        rcr2::Aadjp,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rcr2::Aadjp,
            rcr2::Aadjp,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadje(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rcr2::Aadje,
        rcr2::Aadje,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rcr2::Aadje,
            rcr2::Aadje,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTCOUT Output Enable"]
    #[inline(always)]
    pub fn rtcoe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rcr2::Rtcoe,
        rcr2::Rtcoe,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rcr2::Rtcoe,
            rcr2::Rtcoe,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "30-Second Adjustment"]
    #[inline(always)]
    pub fn adj30(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rcr2::Adj30,
        rcr2::Adj30,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rcr2::Adj30,
            rcr2::Adj30,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Software Reset"]
    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rcr2::Reset,
        rcr2::Reset,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rcr2::Reset,
            rcr2::Reset,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr2::Start,
        rcr2::Start,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr2::Start,
            rcr2::Start,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        <crate::RegValueT<Rcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntmd_SPEC;
    pub type Cntmd = crate::EnumBitfieldStruct<u8, Cntmd_SPEC>;
    impl Cntmd {
        #[doc = "The calendar count mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "The binary count mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hr24_SPEC;
    pub type Hr24 = crate::EnumBitfieldStruct<u8, Hr24_SPEC>;
    impl Hr24 {
        #[doc = "The RTC operates in 12-hour mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "The RTC operates in 24-hour mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadjp_SPEC;
    pub type Aadjp = crate::EnumBitfieldStruct<u8, Aadjp_SPEC>;
    impl Aadjp {
        #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every minute."]
        pub const _0: Self = Self::new(0);

        #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every 10 seconds."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadje_SPEC;
    pub type Aadje = crate::EnumBitfieldStruct<u8, Aadje_SPEC>;
    impl Aadje {
        #[doc = "Automatic adjustment is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatic adjustment is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcoe_SPEC;
    pub type Rtcoe = crate::EnumBitfieldStruct<u8, Rtcoe_SPEC>;
    impl Rtcoe {
        #[doc = "RTCOUT output disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "RTCOUT output enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adj30_SPEC;
    pub type Adj30 = crate::EnumBitfieldStruct<u8, Adj30_SPEC>;
    impl Adj30 {
        #[doc = "Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
        pub const _0: Self = Self::new(0);

        #[doc = "30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reset_SPEC;
    pub type Reset = crate::EnumBitfieldStruct<u8, Reset_SPEC>;
    impl Reset {
        #[doc = "Writing is invalid.(write) /  In normal time operation, or an RTC software reset has completed.(read)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Prescaler and time counter are stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Prescaler and time counter operate normally."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr4_SPEC;
impl crate::sealed::RegSpec for Rcr4_SPEC {
    type DataType = u8;
}

#[doc = "RTC Control Register 4"]
pub type Rcr4 = crate::RegValueT<Rcr4_SPEC>;

impl Rcr4 {
    #[doc = "Count Source Select"]
    #[inline(always)]
    pub fn rcksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr4::Rcksel,
        rcr4::Rcksel,
        Rcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr4::Rcksel,
            rcr4::Rcksel,
            Rcr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        <crate::RegValueT<Rcr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcksel_SPEC;
    pub type Rcksel = crate::EnumBitfieldStruct<u8, Rcksel_SPEC>;
    impl Rcksel {
        #[doc = "Sub-clock oscillator is selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "LOCO clock oscillator is selected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfrh_SPEC;
impl crate::sealed::RegSpec for Rfrh_SPEC {
    type DataType = u16;
}

#[doc = "Frequency Register H"]
pub type Rfrh = crate::RegValueT<Rfrh_SPEC>;

impl Rfrh {
    #[doc = "Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc16(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rfrh_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rfrh_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rfrh {
    #[inline(always)]
    fn default() -> Rfrh {
        <crate::RegValueT<Rfrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfrl_SPEC;
impl crate::sealed::RegSpec for Rfrl_SPEC {
    type DataType = u16;
}

#[doc = "Frequency Register L"]
pub type Rfrl = crate::RegValueT<Rfrl_SPEC>;

impl Rfrl {
    #[doc = "Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle."]
    #[inline(always)]
    pub fn rfc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Rfrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Rfrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfrl {
    #[inline(always)]
    fn default() -> Rfrl {
        <crate::RegValueT<Rfrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Radj_SPEC;
impl crate::sealed::RegSpec for Radj_SPEC {
    type DataType = u8;
}

#[doc = "Time Error Adjustment Register"]
pub type Radj = crate::RegValueT<Radj_SPEC>;

impl Radj {
    #[doc = "Plus-Minus"]
    #[inline(always)]
    pub fn pmadj(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        radj::Pmadj,
        radj::Pmadj,
        Radj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            radj::Pmadj,
            radj::Pmadj,
            Radj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Adjustment Value These bits specify the adjustment value from the prescaler."]
    #[inline(always)]
    pub fn adj(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Radj_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Radj_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Radj {
    #[inline(always)]
    fn default() -> Radj {
        <crate::RegValueT<Radj_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod radj {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmadj_SPEC;
    pub type Pmadj = crate::EnumBitfieldStruct<u8, Pmadj_SPEC>;
    impl Pmadj {
        #[doc = "Adjustment is not performed."]
        pub const _00: Self = Self::new(0);

        #[doc = "Adjustment is performed by the addition to the prescaler."]
        pub const _01: Self = Self::new(1);

        #[doc = "Adjustment is performed by the subtraction from the prescaler."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtccr_SPEC;
impl crate::sealed::RegSpec for Rtccr_SPEC {
    type DataType = u8;
}

#[doc = "Time Capture Control Register %s"]
pub type Rtccr = crate::RegValueT<Rtccr_SPEC>;

impl Rtccr {
    #[doc = "Time Capture Noise Filter Control"]
    #[inline(always)]
    pub fn tcnf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        rtccr::Tcnf,
        rtccr::Tcnf,
        Rtccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            rtccr::Tcnf,
            rtccr::Tcnf,
            Rtccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Time Capture Status"]
    #[inline(always)]
    pub fn tcst(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rtccr::Tcst,
        rtccr::Tcst,
        Rtccr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rtccr::Tcst,
            rtccr::Tcst,
            Rtccr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Time Capture Control"]
    #[inline(always)]
    pub fn tcct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        rtccr::Tcct,
        rtccr::Tcct,
        Rtccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            rtccr::Tcct,
            rtccr::Tcct,
            Rtccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rtccr {
    #[inline(always)]
    fn default() -> Rtccr {
        <crate::RegValueT<Rtccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rtccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcnf_SPEC;
    pub type Tcnf = crate::EnumBitfieldStruct<u8, Tcnf_SPEC>;
    impl Tcnf {
        #[doc = "The noise filter is off."]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "The noise filter is on (count source)."]
        pub const _10: Self = Self::new(2);

        #[doc = "The noise filter is on (count source by divided by 32)."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcst_SPEC;
    pub type Tcst = crate::EnumBitfieldStruct<u8, Tcst_SPEC>;
    impl Tcst {
        #[doc = "No event is detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "An event is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcct_SPEC;
    pub type Tcct = crate::EnumBitfieldStruct<u8, Tcct_SPEC>;
    impl Tcct {
        #[doc = "No event is detected."]
        pub const _00: Self = Self::new(0);

        #[doc = "Rising edge is detected."]
        pub const _01: Self = Self::new(1);

        #[doc = "Falling edge is detected."]
        pub const _10: Self = Self::new(2);

        #[doc = "Both edges are detected."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rseccp_SPEC;
impl crate::sealed::RegSpec for Rseccp_SPEC {
    type DataType = u8;
}

#[doc = "Second Capture Register %s"]
pub type Rseccp = crate::RegValueT<Rseccp_SPEC>;

impl Rseccp {
    #[doc = "10-Second Capture Capture value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rseccp_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rseccp_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1-Second Capture Capture value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rseccp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rseccp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rseccp {
    #[inline(always)]
    fn default() -> Rseccp {
        <crate::RegValueT<Rseccp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0Cp_SPEC;
impl crate::sealed::RegSpec for Bcnt0Cp_SPEC {
    type DataType = u8;
}

#[doc = "BCNT0 Capture Register %s"]
pub type Bcnt0Cp = crate::RegValueT<Bcnt0Cp_SPEC>;

impl Bcnt0Cp {
    #[doc = "BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt0cp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt0Cp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt0Cp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt0Cp {
    #[inline(always)]
    fn default() -> Bcnt0Cp {
        <crate::RegValueT<Bcnt0Cp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmincp_SPEC;
impl crate::sealed::RegSpec for Rmincp_SPEC {
    type DataType = u8;
}

#[doc = "Minute Capture Register %s"]
pub type Rmincp = crate::RegValueT<Rmincp_SPEC>;

impl Rmincp {
    #[doc = "10-Minute Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rmincp_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rmincp_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1-Minute Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmincp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmincp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmincp {
    #[inline(always)]
    fn default() -> Rmincp {
        <crate::RegValueT<Rmincp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1Cp_SPEC;
impl crate::sealed::RegSpec for Bcnt1Cp_SPEC {
    type DataType = u8;
}

#[doc = "BCNT1 Capture Register %s"]
pub type Bcnt1Cp = crate::RegValueT<Bcnt1Cp_SPEC>;

impl Bcnt1Cp {
    #[doc = "BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt1cp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt1Cp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt1Cp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt1Cp {
    #[inline(always)]
    fn default() -> Bcnt1Cp {
        <crate::RegValueT<Bcnt1Cp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrcp_SPEC;
impl crate::sealed::RegSpec for Rhrcp_SPEC {
    type DataType = u8;
}

#[doc = "Hour Capture Register %s"]
pub type Rhrcp = crate::RegValueT<Rhrcp_SPEC>;

impl Rhrcp {
    #[doc = "A.m./p.m. select for time counter setting."]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rhrcp::Pm,
        rhrcp::Pm,
        Rhrcp_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rhrcp::Pm,
            rhrcp::Pm,
            Rhrcp_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "10-Minute Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn hr10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rhrcp_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rhrcp_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1-Minute Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn hr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rhrcp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rhrcp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rhrcp {
    #[inline(always)]
    fn default() -> Rhrcp {
        <crate::RegValueT<Rhrcp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rhrcp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "a.m."]
        pub const _0: Self = Self::new(0);

        #[doc = "p.m."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2Cp_SPEC;
impl crate::sealed::RegSpec for Bcnt2Cp_SPEC {
    type DataType = u8;
}

#[doc = "BCNT2 Capture Register %s"]
pub type Bcnt2Cp = crate::RegValueT<Bcnt2Cp_SPEC>;

impl Bcnt2Cp {
    #[doc = "BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt2cp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt2Cp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt2Cp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt2Cp {
    #[inline(always)]
    fn default() -> Bcnt2Cp {
        <crate::RegValueT<Bcnt2Cp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdaycp_SPEC;
impl crate::sealed::RegSpec for Rdaycp_SPEC {
    type DataType = u8;
}

#[doc = "Date Capture Register %s"]
pub type Rdaycp = crate::RegValueT<Rdaycp_SPEC>;

impl Rdaycp {
    #[doc = "10-Day Capture Capture value for the tens place of minutes"]
    #[inline(always)]
    pub fn date10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rdaycp_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rdaycp_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1-Day Capture Capture value for the ones place of minutes"]
    #[inline(always)]
    pub fn date1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rdaycp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rdaycp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdaycp {
    #[inline(always)]
    fn default() -> Rdaycp {
        <crate::RegValueT<Rdaycp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3Cp_SPEC;
impl crate::sealed::RegSpec for Bcnt3Cp_SPEC {
    type DataType = u8;
}

#[doc = "BCNT3 Capture Register %s"]
pub type Bcnt3Cp = crate::RegValueT<Bcnt3Cp_SPEC>;

impl Bcnt3Cp {
    #[doc = "BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected."]
    #[inline(always)]
    pub fn bcnt3cp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt3Cp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt3Cp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt3Cp {
    #[inline(always)]
    fn default() -> Bcnt3Cp {
        <crate::RegValueT<Bcnt3Cp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmoncp_SPEC;
impl crate::sealed::RegSpec for Rmoncp_SPEC {
    type DataType = u8;
}

#[doc = "Month Capture Register %s"]
pub type Rmoncp = crate::RegValueT<Rmoncp_SPEC>;

impl Rmoncp {
    #[doc = "10-Month Capture Capture value for the tens place of months"]
    #[inline(always)]
    pub fn mon10(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rmoncp_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmoncp_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "1-Month Capture Capture value for the ones place of months"]
    #[inline(always)]
    pub fn mon1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmoncp_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmoncp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmoncp {
    #[inline(always)]
    fn default() -> Rmoncp {
        <crate::RegValueT<Rmoncp_SPEC> as RegisterValue<_>>::new(0)
    }
}
