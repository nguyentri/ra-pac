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
// Generated from SVD 1.20.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:02:40 +0000

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

    #[doc = "Binary Counter %s"]
    #[inline(always)]
    pub const fn bcnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bcnt_SPEC, crate::common::RW>,
        4,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2usize))
        }
    }
    #[inline(always)]
    pub const fn bcnt0(&self) -> &'static crate::common::Reg<self::Bcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt1(&self) -> &'static crate::common::Reg<self::Bcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt2(&self) -> &'static crate::common::Reg<self::Bcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt3(&self) -> &'static crate::common::Reg<self::Bcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }

    #[doc = "Second Counter (in Calendar Count Mode)"]
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

    #[doc = "Minute Counter (in Calendar Count Mode)"]
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

    #[doc = "Hour Counter (in Calendar Count Mode)"]
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

    #[doc = "Day-of-Week Counter (in Calendar Count Mode)"]
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

    #[doc = "Binary Counter %s Alarm Register"]
    #[inline(always)]
    pub const fn bcntar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bcntar_SPEC, crate::common::RW>,
        4,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10usize))
        }
    }
    #[inline(always)]
    pub const fn bcnt0ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcntar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcntar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt1ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcntar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcntar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt2ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcntar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcntar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt3ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcntar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcntar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16usize),
            )
        }
    }

    #[doc = "Second Alarm Register (in Calendar Count Mode)"]
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

    #[doc = "Minute Alarm Register (in Calendar Count Mode)"]
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

    #[doc = "Hour Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rhrar(&self) -> &'static crate::common::Reg<self::Rhrar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rhrar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Day-of-Week Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rwkar(&self) -> &'static crate::common::Reg<self::Rwkar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rwkar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Binary Counter %s Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcntaer(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bcntaer_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18usize))
        }
    }
    #[inline(always)]
    pub const fn bcnt0aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcntaer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcntaer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bcnt1aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcntaer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcntaer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ausize),
            )
        }
    }

    #[doc = "Date Alarm Register (in Calendar Count Mode)"]
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

    #[doc = "Month Alarm Register (in Calendar Count Mode)"]
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

    #[doc = "Year Alarm Register (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn ryrar(&self) -> &'static crate::common::Reg<self::Ryrar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ryrar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
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

    #[doc = "Year Alarm Enable Register (in Calendar Count Mode)"]
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

    #[doc = "RTC Control Register 1"]
    #[inline(always)]
    pub const fn rcr1(&self) -> &'static crate::common::Reg<self::Rcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "RTC Control Register 2 (in Calendar Count Mode)"]
    #[inline(always)]
    pub const fn rcr2(&self) -> &'static crate::common::Reg<self::Rcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "RTC Control Register 2 (in Binary Count Mode)"]
    #[inline(always)]
    pub const fn rcr2_bcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rcr2Bcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr2Bcnt_SPEC, crate::common::RW>::from_ptr(
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
    #[doc = "64-Hz Flag"]
    #[inline(always)]
    pub fn f64hz(self) -> crate::common::RegisterFieldBool<0, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "32-Hz Flag"]
    #[inline(always)]
    pub fn f32hz(self) -> crate::common::RegisterFieldBool<1, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "16-Hz Flag"]
    #[inline(always)]
    pub fn f16hz(self) -> crate::common::RegisterFieldBool<2, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "8-Hz Flag"]
    #[inline(always)]
    pub fn f8hz(self) -> crate::common::RegisterFieldBool<3, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "4-Hz Flag"]
    #[inline(always)]
    pub fn f4hz(self) -> crate::common::RegisterFieldBool<4, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "2-Hz Flag"]
    #[inline(always)]
    pub fn f2hz(self) -> crate::common::RegisterFieldBool<5, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "1-Hz Flag"]
    #[inline(always)]
    pub fn f1hz(self) -> crate::common::RegisterFieldBool<6, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn r64ovf(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
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
pub struct Bcnt_SPEC;
impl crate::sealed::RegSpec for Bcnt_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter %s"]
pub type Bcnt = crate::RegValueT<Bcnt_SPEC>;

impl Bcnt {
    #[doc = "Binary Counter"]
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt {
    #[inline(always)]
    fn default() -> Bcnt {
        <crate::RegValueT<Bcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rseccnt_SPEC;
impl crate::sealed::RegSpec for Rseccnt_SPEC {
    type DataType = u8;
}

#[doc = "Second Counter (in Calendar Count Mode)"]
pub type Rseccnt = crate::RegValueT<Rseccnt_SPEC>;

impl Rseccnt {
    #[doc = "1-Second Count"]
    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rseccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rseccnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-Second Count"]
    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rseccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rseccnt_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Rmincnt_SPEC;
impl crate::sealed::RegSpec for Rmincnt_SPEC {
    type DataType = u8;
}

#[doc = "Minute Counter (in Calendar Count Mode)"]
pub type Rmincnt = crate::RegValueT<Rmincnt_SPEC>;

impl Rmincnt {
    #[doc = "1-Minute Count"]
    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmincnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmincnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-Minute Count"]
    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rmincnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rmincnt_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Rhrcnt_SPEC;
impl crate::sealed::RegSpec for Rhrcnt_SPEC {
    type DataType = u8;
}

#[doc = "Hour Counter (in Calendar Count Mode)"]
pub type Rhrcnt = crate::RegValueT<Rhrcnt_SPEC>;

impl Rhrcnt {
    #[doc = "1-Hour Count"]
    #[inline(always)]
    pub fn hr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rhrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rhrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-Hour Count"]
    #[inline(always)]
    pub fn hr10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rhrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rhrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AM/PM select for time counter setting."]
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
        #[doc = "AM"]
        pub const _0: Self = Self::new(0);

        #[doc = "PM"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkcnt_SPEC;
impl crate::sealed::RegSpec for Rwkcnt_SPEC {
    type DataType = u8;
}

#[doc = "Day-of-Week Counter (in Calendar Count Mode)"]
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

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
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
    #[doc = "1-Day Count"]
    #[inline(always)]
    pub fn date1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rdaycnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rdaycnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-Day Count"]
    #[inline(always)]
    pub fn date10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rdaycnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rdaycnt_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "1-Month Count"]
    #[inline(always)]
    pub fn mon1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmoncnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmoncnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-Month Count"]
    #[inline(always)]
    pub fn mon10(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Rmoncnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmoncnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    #[doc = "1-Year Count"]
    #[inline(always)]
    pub fn yr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ryrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ryrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-Year Count"]
    #[inline(always)]
    pub fn yr10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ryrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ryrcnt_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Bcntar_SPEC;
impl crate::sealed::RegSpec for Bcntar_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter %s Alarm Register"]
pub type Bcntar = crate::RegValueT<Bcntar_SPEC>;

impl Bcntar {
    #[doc = "Alarm register associated with the 32-bit binary counter"]
    #[inline(always)]
    pub fn bcntar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcntar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcntar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcntar {
    #[inline(always)]
    fn default() -> Bcntar {
        <crate::RegValueT<Bcntar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsecar_SPEC;
impl crate::sealed::RegSpec for Rsecar_SPEC {
    type DataType = u8;
}

#[doc = "Second Alarm Register (in Calendar Count Mode)"]
pub type Rsecar = crate::RegValueT<Rsecar_SPEC>;

impl Rsecar {
    #[doc = "1 Second"]
    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rsecar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rsecar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10 Seconds"]
    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rsecar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rsecar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ENB"]
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
        #[doc = "Do not compare register value with RSECCNT counter value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare register value with RSECCNT counter value"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rminar_SPEC;
impl crate::sealed::RegSpec for Rminar_SPEC {
    type DataType = u8;
}

#[doc = "Minute Alarm Register (in Calendar Count Mode)"]
pub type Rminar = crate::RegValueT<Rminar_SPEC>;

impl Rminar {
    #[doc = "1 Minute"]
    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rminar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rminar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10 Minutes"]
    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rminar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rminar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ENB"]
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
        #[doc = "Do not compare register value with RMINCNT counter value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare register value with RMINCNT counter value"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrar_SPEC;
impl crate::sealed::RegSpec for Rhrar_SPEC {
    type DataType = u8;
}

#[doc = "Hour Alarm Register (in Calendar Count Mode)"]
pub type Rhrar = crate::RegValueT<Rhrar_SPEC>;

impl Rhrar {
    #[doc = "1 Hour"]
    #[inline(always)]
    pub fn hr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rhrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rhrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10 Hours"]
    #[inline(always)]
    pub fn hr10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rhrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rhrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AM/PM select for alarm setting."]
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

    #[doc = "ENB"]
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
}
impl ::core::default::Default for Rhrar {
    #[inline(always)]
    fn default() -> Rhrar {
        <crate::RegValueT<Rhrar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rhrar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "AM"]
        pub const _0: Self = Self::new(0);

        #[doc = "PM"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "Do not compare register value with RHRCNT counter value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare register value with RHRCNT counter value"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkar_SPEC;
impl crate::sealed::RegSpec for Rwkar_SPEC {
    type DataType = u8;
}

#[doc = "Day-of-Week Alarm Register (in Calendar Count Mode)"]
pub type Rwkar = crate::RegValueT<Rwkar_SPEC>;

impl Rwkar {
    #[doc = "Day-of-Week Setting"]
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

    #[doc = "ENB"]
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
}
impl ::core::default::Default for Rwkar {
    #[inline(always)]
    fn default() -> Rwkar {
        <crate::RegValueT<Rwkar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rwkar {

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

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "Do not compare register value with RWKCNT counter value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare register value with RWKCNT counter value"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcntaer_SPEC;
impl crate::sealed::RegSpec for Bcntaer_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter %s Alarm Enable Register"]
pub type Bcntaer = crate::RegValueT<Bcntaer_SPEC>;

impl Bcntaer {
    #[doc = "Setting the alarm enable associated with the 32-bit binary counter"]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcntaer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcntaer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcntaer {
    #[inline(always)]
    fn default() -> Bcntaer {
        <crate::RegValueT<Bcntaer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdayar_SPEC;
impl crate::sealed::RegSpec for Rdayar_SPEC {
    type DataType = u8;
}

#[doc = "Date Alarm Register (in Calendar Count Mode)"]
pub type Rdayar = crate::RegValueT<Rdayar_SPEC>;

impl Rdayar {
    #[doc = "1 Day"]
    #[inline(always)]
    pub fn date1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rdayar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rdayar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10 Days"]
    #[inline(always)]
    pub fn date10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rdayar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rdayar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ENB"]
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
        #[doc = "Do not compare register value with RDAYCNT counter value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare register value with RDAYCNT counter value"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmonar_SPEC;
impl crate::sealed::RegSpec for Rmonar_SPEC {
    type DataType = u8;
}

#[doc = "Month Alarm Register (in Calendar Count Mode)"]
pub type Rmonar = crate::RegValueT<Rmonar_SPEC>;

impl Rmonar {
    #[doc = "1 Month"]
    #[inline(always)]
    pub fn mon1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmonar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmonar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10 Months"]
    #[inline(always)]
    pub fn mon10(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Rmonar_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmonar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ENB"]
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
        #[doc = "Do not compare register value with RMONCNT counter value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare register value with RMONCNT counter value"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Setting the alarm enable associated with the 32-bit binary counter"]
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
pub struct Ryrar_SPEC;
impl crate::sealed::RegSpec for Ryrar_SPEC {
    type DataType = u16;
}

#[doc = "Year Alarm Register (in Calendar Count Mode)"]
pub type Ryrar = crate::RegValueT<Ryrar_SPEC>;

impl Ryrar {
    #[doc = "1 Year"]
    #[inline(always)]
    pub fn yr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ryrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ryrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10 Years"]
    #[inline(always)]
    pub fn yr10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ryrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ryrar_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Bcnt3Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt3Aer_SPEC {
    type DataType = u8;
}

#[doc = "Binary Counter 3 Alarm Enable Register"]
pub type Bcnt3Aer = crate::RegValueT<Bcnt3Aer_SPEC>;

impl Bcnt3Aer {
    #[doc = "Setting the alarm enable associated with the 32-bit binary counter"]
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
pub struct Ryraren_SPEC;
impl crate::sealed::RegSpec for Ryraren_SPEC {
    type DataType = u8;
}

#[doc = "Year Alarm Enable Register (in Calendar Count Mode)"]
pub type Ryraren = crate::RegValueT<Ryraren_SPEC>;

impl Ryraren {
    #[doc = "ENB"]
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
        #[doc = "Do not compare register value with the RYRCNT counter value"]
        pub const _0: Self = Self::new(0);

        #[doc = "Compare register value with the RYRCNT counter value"]
        pub const _1: Self = Self::new(1);
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
}
impl ::core::default::Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        <crate::RegValueT<Rcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aie_SPEC;
    pub type Aie = crate::EnumBitfieldStruct<u8, Aie_SPEC>;
    impl Aie {
        #[doc = "Disable alarm interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cie_SPEC;
    pub type Cie = crate::EnumBitfieldStruct<u8, Cie_SPEC>;
    impl Cie {
        #[doc = "Disable carry interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable carry interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pie_SPEC;
    pub type Pie = crate::EnumBitfieldStruct<u8, Pie_SPEC>;
    impl Pie {
        #[doc = "Disable periodic interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable periodic interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcos_SPEC;
    pub type Rtcos = crate::EnumBitfieldStruct<u8, Rtcos_SPEC>;
    impl Rtcos {
        #[doc = "Outputs 1 Hz on RTCOUT"]
        pub const _0: Self = Self::new(0);

        #[doc = "Outputs 64 Hz RTCOUT"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pes_SPEC;
    pub type Pes = crate::EnumBitfieldStruct<u8, Pes_SPEC>;
    impl Pes {
        #[doc = "Generate periodic interrupt every 1/256 second"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Generate periodic interrupt every 1/128 second"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "Generate periodic interrupt every 1/64 second"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Generate periodic interrupt every 1/32 second"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "Generate periodic interrupt every 1/16 second"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "Generate periodic interrupt every 1/8 second"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "Generate periodic interrupt every 1/4 second"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "Generate periodic interrupt every 1/2 second"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "Generate periodic interrupt every 1 second"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "Generate periodic interrupt every 2 seconds"]
        pub const _0_X_F: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2_SPEC;
impl crate::sealed::RegSpec for Rcr2_SPEC {
    type DataType = u8;
}

#[doc = "RTC Control Register 2 (in Calendar Count Mode)"]
pub type Rcr2 = crate::RegValueT<Rcr2_SPEC>;

impl Rcr2 {
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

    #[doc = "Automatic Adjustment Enable"]
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

    #[doc = "Automatic Adjustment Period Select"]
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
}
impl ::core::default::Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        <crate::RegValueT<Rcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Stop prescaler and time counter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operate prescaler and time counter normally"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reset_SPEC;
    pub type Reset = crate::EnumBitfieldStruct<u8, Reset_SPEC>;
    impl Reset {
        #[doc = "In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or an RTC software reset has completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "In writing: Initialize the prescaler and target registers for RTC software reset. In reading: RTC software reset in progress."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adj30_SPEC;
    pub type Adj30 = crate::EnumBitfieldStruct<u8, Adj30_SPEC>;
    impl Adj30 {
        #[doc = "In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or 30-second adjustment has completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "In writing: Execute 30-second adjustment. In reading: 30-second adjustment in progress."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcoe_SPEC;
    pub type Rtcoe = crate::EnumBitfieldStruct<u8, Rtcoe_SPEC>;
    impl Rtcoe {
        #[doc = "Disable RTCOUT output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable RTCOUT output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadje_SPEC;
    pub type Aadje = crate::EnumBitfieldStruct<u8, Aadje_SPEC>;
    impl Aadje {
        #[doc = "Disable automatic adjustment"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable automatic adjustment"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadjp_SPEC;
    pub type Aadjp = crate::EnumBitfieldStruct<u8, Aadjp_SPEC>;
    impl Aadjp {
        #[doc = "In normal operation mode, adjust RADJ.ADJ\\[5:0\\] setting from the count value of the prescaler every minute. In low-consumption clock mode, adjust RADJ.ADJ\\[5:0\\] setting from the count value of the 64-Hz counter every day."]
        pub const _0: Self = Self::new(0);

        #[doc = "In normal operation mode, adjust RADJ.ADJ\\[5:0\\] setting from the count value of the prescaler every 10 seconds. In low-consumption clock mode, adjust RADJ.ADJ\\[5:0\\] setting from the count value of the 64-Hz counter every hour."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hr24_SPEC;
    pub type Hr24 = crate::EnumBitfieldStruct<u8, Hr24_SPEC>;
    impl Hr24 {
        #[doc = "Operate RTC in 12-hour mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operate RTC in 24-hour mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntmd_SPEC;
    pub type Cntmd = crate::EnumBitfieldStruct<u8, Cntmd_SPEC>;
    impl Cntmd {
        #[doc = "Calendar count mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Binary count mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2Bcnt_SPEC;
impl crate::sealed::RegSpec for Rcr2Bcnt_SPEC {
    type DataType = u8;
}

#[doc = "RTC Control Register 2 (in Binary Count Mode)"]
pub type Rcr2Bcnt = crate::RegValueT<Rcr2Bcnt_SPEC>;

impl Rcr2Bcnt {
    #[doc = "Start"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr2_bcnt::Start,
        rcr2_bcnt::Start,
        Rcr2Bcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr2_bcnt::Start,
            rcr2_bcnt::Start,
            Rcr2Bcnt_SPEC,
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
        rcr2_bcnt::Reset,
        rcr2_bcnt::Reset,
        Rcr2Bcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rcr2_bcnt::Reset,
            rcr2_bcnt::Reset,
            Rcr2Bcnt_SPEC,
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
        rcr2_bcnt::Rtcoe,
        rcr2_bcnt::Rtcoe,
        Rcr2Bcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rcr2_bcnt::Rtcoe,
            rcr2_bcnt::Rtcoe,
            Rcr2Bcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Adjustment Enable"]
    #[inline(always)]
    pub fn aadje(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rcr2_bcnt::Aadje,
        rcr2_bcnt::Aadje,
        Rcr2Bcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rcr2_bcnt::Aadje,
            rcr2_bcnt::Aadje,
            Rcr2Bcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Adjustment Period Select"]
    #[inline(always)]
    pub fn aadjp(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rcr2_bcnt::Aadjp,
        rcr2_bcnt::Aadjp,
        Rcr2Bcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rcr2_bcnt::Aadjp,
            rcr2_bcnt::Aadjp,
            Rcr2Bcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Count Mode Select"]
    #[inline(always)]
    pub fn cntmd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rcr2_bcnt::Cntmd,
        rcr2_bcnt::Cntmd,
        Rcr2Bcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rcr2_bcnt::Cntmd,
            rcr2_bcnt::Cntmd,
            Rcr2Bcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr2Bcnt {
    #[inline(always)]
    fn default() -> Rcr2Bcnt {
        <crate::RegValueT<Rcr2Bcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr2_bcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Stop the 32-bit binary counter, 64-Hz counter, and prescaler"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operate the 32-bit binary counter, 64-Hz counter, and prescaler normally"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reset_SPEC;
    pub type Reset = crate::EnumBitfieldStruct<u8, Reset_SPEC>;
    impl Reset {
        #[doc = "In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or an RTC software reset has completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "In writing: Initialize the prescaler and target registers for RTC software reset. In reading: RTC software reset in progress."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcoe_SPEC;
    pub type Rtcoe = crate::EnumBitfieldStruct<u8, Rtcoe_SPEC>;
    impl Rtcoe {
        #[doc = "Disable RTCOUT output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable RTCOUT output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadje_SPEC;
    pub type Aadje = crate::EnumBitfieldStruct<u8, Aadje_SPEC>;
    impl Aadje {
        #[doc = "Disable automatic adjustment"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable automatic adjustment"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadjp_SPEC;
    pub type Aadjp = crate::EnumBitfieldStruct<u8, Aadjp_SPEC>;
    impl Aadjp {
        #[doc = "In normal operation mode, add or subtract the RADJ.ADJ\\[5:0\\] bits from the prescaler count value every 32 seconds. In low-consumption clock mode, add or subtract the RADJ.ADJ\\[5:0\\] bits from the 64-Hz counter count value every 8192 seconds."]
        pub const _0: Self = Self::new(0);

        #[doc = "In normal operation mode, add or subtract the RADJ.ADJ\\[5:0\\] bits from the prescaler count value every 8 seconds. In low-consumption clock mode, add or subtract the RADJ.ADJ\\[5:0\\] bits from the 64-Hz counter count value every 2048 seconds."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntmd_SPEC;
    pub type Cntmd = crate::EnumBitfieldStruct<u8, Cntmd_SPEC>;
    impl Cntmd {
        #[doc = "Calendar count mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Binary count mode"]
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
    #[doc = "Count Source Select in normal operation mode"]
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

    #[doc = "RTC Operation Mode Select"]
    #[inline(always)]
    pub fn ropsel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rcr4::Ropsel,
        rcr4::Ropsel,
        Rcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rcr4::Ropsel,
            rcr4::Ropsel,
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
        #[doc = "Sub-clock oscillator is selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "LOCO is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ropsel_SPEC;
    pub type Ropsel = crate::EnumBitfieldStruct<u8, Ropsel_SPEC>;
    impl Ropsel {
        #[doc = "Normal operation mode is selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Low-consumption clock mode is selected."]
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
    #[doc = "Write 0 before writing to the RFRL register after a cold start."]
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
    #[doc = "Frequency Comparison Value"]
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
    #[doc = "Adjustment Value"]
    #[inline(always)]
    pub fn adj(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Radj_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Radj_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        #[doc = "Do not perform adjustment."]
        pub const _00: Self = Self::new(0);

        #[doc = "In normal operation mode, adjustment is performed by the addition to the prescaler. In low-consumption clock mode, adjustment is performed by the addition to the 64-Hz counter."]
        pub const _01: Self = Self::new(1);

        #[doc = "In normal operation mode, adjustment is performed by the subtraction from the prescaler. In low-consumption clock mode, adjustment is performed by the subtraction from the 64-Hz counter."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(3);
    }
}
