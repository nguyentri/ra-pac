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
// Generated from SVD 1.10.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:13 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Realtime Clock"]
unsafe impl ::core::marker::Send for super::RtcC {}
unsafe impl ::core::marker::Sync for super::RtcC {}
impl super::RtcC {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Second Count Register"]
    #[inline(always)]
    pub const fn sec(&self) -> &'static crate::common::Reg<self::Sec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Minute Count Register"]
    #[inline(always)]
    pub const fn min(&self) -> &'static crate::common::Reg<self::Min_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Min_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "Hour Count Register"]
    #[inline(always)]
    pub const fn hour(&self) -> &'static crate::common::Reg<self::Hour_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hour_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Day-of-Week Count Register"]
    #[inline(always)]
    pub const fn week(&self) -> &'static crate::common::Reg<self::Week_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Week_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "Day Count Register"]
    #[inline(always)]
    pub const fn day(&self) -> &'static crate::common::Reg<self::Day_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Day_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Month Count Register"]
    #[inline(always)]
    pub const fn month(&self) -> &'static crate::common::Reg<self::Month_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Month_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "Year Count Register"]
    #[inline(always)]
    pub const fn year(&self) -> &'static crate::common::Reg<self::Year_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Year_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Time Error Correction Register"]
    #[inline(always)]
    pub const fn subcud(
        &self,
    ) -> &'static crate::common::Reg<self::Subcud_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Subcud_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "Alarm Minute Register"]
    #[inline(always)]
    pub const fn alarmwm(
        &self,
    ) -> &'static crate::common::Reg<self::Alarmwm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Alarmwm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Alarm Hour Register"]
    #[inline(always)]
    pub const fn alarmwh(
        &self,
    ) -> &'static crate::common::Reg<self::Alarmwh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Alarmwh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "Alarm Day-of-Week Register"]
    #[inline(always)]
    pub const fn alarmww(
        &self,
    ) -> &'static crate::common::Reg<self::Alarmww_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Alarmww_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Realtime Clock Control Register 0"]
    #[inline(always)]
    pub const fn rtcc0(&self) -> &'static crate::common::Reg<self::Rtcc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rtcc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "Realtime Clock Control Register 1"]
    #[inline(always)]
    pub const fn rtcc1(&self) -> &'static crate::common::Reg<self::Rtcc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rtcc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sec_SPEC;
impl crate::sealed::RegSpec for Sec_SPEC {
    type DataType = u8;
}

#[doc = "Second Count Register"]
pub type Sec = crate::RegValueT<Sec_SPEC>;

impl Sec {
    #[doc = "1-second count"]
    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Sec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Sec_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-second count"]
    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Sec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Sec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sec {
    #[inline(always)]
    fn default() -> Sec {
        <crate::RegValueT<Sec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Min_SPEC;
impl crate::sealed::RegSpec for Min_SPEC {
    type DataType = u8;
}

#[doc = "Minute Count Register"]
pub type Min = crate::RegValueT<Min_SPEC>;

impl Min {
    #[doc = "1-minute count"]
    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Min_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Min_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10- minute count"]
    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Min_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Min_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Min {
    #[inline(always)]
    fn default() -> Min {
        <crate::RegValueT<Min_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hour_SPEC;
impl crate::sealed::RegSpec for Hour_SPEC {
    type DataType = u8;
}

#[doc = "Hour Count Register"]
pub type Hour = crate::RegValueT<Hour_SPEC>;

impl Hour {
    #[doc = "1-hour count"]
    #[inline(always)]
    pub fn hour1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Hour_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Hour_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-hour count"]
    #[inline(always)]
    pub fn hour10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Hour_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Hour_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hour {
    #[inline(always)]
    fn default() -> Hour {
        <crate::RegValueT<Hour_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Week_SPEC;
impl crate::sealed::RegSpec for Week_SPEC {
    type DataType = u8;
}

#[doc = "Day-of-Week Count Register"]
pub type Week = crate::RegValueT<Week_SPEC>;

impl Week {
    #[doc = "Day-of-Week Counting"]
    #[inline(always)]
    pub fn week(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        week::Week,
        week::Week,
        Week_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            week::Week,
            week::Week,
            Week_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Week {
    #[inline(always)]
    fn default() -> Week {
        <crate::RegValueT<Week_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod week {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Week_SPEC;
    pub type Week = crate::EnumBitfieldStruct<u8, Week_SPEC>;
    impl Week {
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
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Day_SPEC;
impl crate::sealed::RegSpec for Day_SPEC {
    type DataType = u8;
}

#[doc = "Day Count Register"]
pub type Day = crate::RegValueT<Day_SPEC>;

impl Day {
    #[doc = "1-day count"]
    #[inline(always)]
    pub fn day1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Day_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Day_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-day count"]
    #[inline(always)]
    pub fn day10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Day_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Day_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Day {
    #[inline(always)]
    fn default() -> Day {
        <crate::RegValueT<Day_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Month_SPEC;
impl crate::sealed::RegSpec for Month_SPEC {
    type DataType = u8;
}

#[doc = "Month Count Register"]
pub type Month = crate::RegValueT<Month_SPEC>;

impl Month {
    #[doc = "1-month count"]
    #[inline(always)]
    pub fn month1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Month_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Month_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-month count"]
    #[inline(always)]
    pub fn month10(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Month_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Month_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Month {
    #[inline(always)]
    fn default() -> Month {
        <crate::RegValueT<Month_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Year_SPEC;
impl crate::sealed::RegSpec for Year_SPEC {
    type DataType = u8;
}

#[doc = "Year Count Register"]
pub type Year = crate::RegValueT<Year_SPEC>;

impl Year {
    #[doc = "1-year count"]
    #[inline(always)]
    pub fn year1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Year_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Year_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-year count"]
    #[inline(always)]
    pub fn year10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Year_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Year_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Year {
    #[inline(always)]
    fn default() -> Year {
        <crate::RegValueT<Year_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Subcud_SPEC;
impl crate::sealed::RegSpec for Subcud_SPEC {
    type DataType = u8;
}

#[doc = "Time Error Correction Register"]
pub type Subcud = crate::RegValueT<Subcud_SPEC>;

impl Subcud {
    #[doc = "Adjustment Value"]
    #[inline(always)]
    pub fn f(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Subcud_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Subcud_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Setting of time error correction value"]
    #[inline(always)]
    pub fn f6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        subcud::F6,
        subcud::F6,
        Subcud_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            subcud::F6,
            subcud::F6,
            Subcud_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of time error correction timing"]
    #[inline(always)]
    pub fn dev(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        subcud::Dev,
        subcud::Dev,
        Subcud_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            subcud::Dev,
            subcud::Dev,
            Subcud_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Subcud {
    #[inline(always)]
    fn default() -> Subcud {
        <crate::RegValueT<Subcud_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod subcud {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct F6_SPEC;
    pub type F6 = crate::EnumBitfieldStruct<u8, F6_SPEC>;
    impl F6 {
        #[doc = "Increases by {F\\[5:0\\] – 1} × 2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Decreases by {/F\\[5:0\\] + 1} × 2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dev_SPEC;
    pub type Dev = crate::EnumBitfieldStruct<u8, Dev_SPEC>;
    impl Dev {
        #[doc = "Corrects time error when the second digits are at 00, 20, or 40 (every 20 seconds)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Corrects time error only when the second digits are at 00 (every 60 seconds)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alarmwm_SPEC;
impl crate::sealed::RegSpec for Alarmwm_SPEC {
    type DataType = u8;
}

#[doc = "Alarm Minute Register"]
pub type Alarmwm = crate::RegValueT<Alarmwm_SPEC>;

impl Alarmwm {
    #[doc = "1-digit minute setting"]
    #[inline(always)]
    pub fn wm1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Alarmwm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Alarmwm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-digit minute setting"]
    #[inline(always)]
    pub fn wm10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Alarmwm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Alarmwm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Alarmwm {
    #[inline(always)]
    fn default() -> Alarmwm {
        <crate::RegValueT<Alarmwm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alarmwh_SPEC;
impl crate::sealed::RegSpec for Alarmwh_SPEC {
    type DataType = u8;
}

#[doc = "Alarm Hour Register"]
pub type Alarmwh = crate::RegValueT<Alarmwh_SPEC>;

impl Alarmwh {
    #[doc = "1-digit hour setting"]
    #[inline(always)]
    pub fn wh1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Alarmwh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Alarmwh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-digit hour setting"]
    #[inline(always)]
    pub fn wh10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Alarmwh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Alarmwh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Alarmwh {
    #[inline(always)]
    fn default() -> Alarmwh {
        <crate::RegValueT<Alarmwh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alarmww_SPEC;
impl crate::sealed::RegSpec for Alarmww_SPEC {
    type DataType = u8;
}

#[doc = "Alarm Day-of-Week Register"]
pub type Alarmww = crate::RegValueT<Alarmww_SPEC>;

impl Alarmww {
    #[doc = "Alarm enabled setting \"Sunday\""]
    #[inline(always)]
    pub fn ww0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        alarmww::Ww0,
        alarmww::Ww0,
        Alarmww_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            alarmww::Ww0,
            alarmww::Ww0,
            Alarmww_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm enabled setting \"Monday\""]
    #[inline(always)]
    pub fn ww1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        alarmww::Ww1,
        alarmww::Ww1,
        Alarmww_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            alarmww::Ww1,
            alarmww::Ww1,
            Alarmww_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm enabled setting \"Tuesday\""]
    #[inline(always)]
    pub fn ww2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        alarmww::Ww2,
        alarmww::Ww2,
        Alarmww_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            alarmww::Ww2,
            alarmww::Ww2,
            Alarmww_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm enabled setting \"Wednesday\""]
    #[inline(always)]
    pub fn ww3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        alarmww::Ww3,
        alarmww::Ww3,
        Alarmww_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            alarmww::Ww3,
            alarmww::Ww3,
            Alarmww_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm enabled setting \"Thursday\""]
    #[inline(always)]
    pub fn ww4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        alarmww::Ww4,
        alarmww::Ww4,
        Alarmww_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            alarmww::Ww4,
            alarmww::Ww4,
            Alarmww_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm enabled setting \"Friday\""]
    #[inline(always)]
    pub fn ww5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        alarmww::Ww5,
        alarmww::Ww5,
        Alarmww_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            alarmww::Ww5,
            alarmww::Ww5,
            Alarmww_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm enabled setting \"Saturday\""]
    #[inline(always)]
    pub fn ww6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        alarmww::Ww6,
        alarmww::Ww6,
        Alarmww_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            alarmww::Ww6,
            alarmww::Ww6,
            Alarmww_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Alarmww {
    #[inline(always)]
    fn default() -> Alarmww {
        <crate::RegValueT<Alarmww_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod alarmww {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ww0_SPEC;
    pub type Ww0 = crate::EnumBitfieldStruct<u8, Ww0_SPEC>;
    impl Ww0 {
        #[doc = "Disable alarm settings for that day of the week"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm settings for that day of the week"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ww1_SPEC;
    pub type Ww1 = crate::EnumBitfieldStruct<u8, Ww1_SPEC>;
    impl Ww1 {
        #[doc = "Disable alarm settings for that day of the week"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm settings for that day of the week"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ww2_SPEC;
    pub type Ww2 = crate::EnumBitfieldStruct<u8, Ww2_SPEC>;
    impl Ww2 {
        #[doc = "Disable alarm settings for that day of the week"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm settings for that day of the week"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ww3_SPEC;
    pub type Ww3 = crate::EnumBitfieldStruct<u8, Ww3_SPEC>;
    impl Ww3 {
        #[doc = "Disable alarm settings for that day of the week"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm settings for that day of the week"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ww4_SPEC;
    pub type Ww4 = crate::EnumBitfieldStruct<u8, Ww4_SPEC>;
    impl Ww4 {
        #[doc = "Disable alarm settings for that day of the week"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm settings for that day of the week"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ww5_SPEC;
    pub type Ww5 = crate::EnumBitfieldStruct<u8, Ww5_SPEC>;
    impl Ww5 {
        #[doc = "Disable alarm settings for that day of the week"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm settings for that day of the week"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ww6_SPEC;
    pub type Ww6 = crate::EnumBitfieldStruct<u8, Ww6_SPEC>;
    impl Ww6 {
        #[doc = "Disable alarm settings for that day of the week"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alarm settings for that day of the week"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtcc0_SPEC;
impl crate::sealed::RegSpec for Rtcc0_SPEC {
    type DataType = u8;
}

#[doc = "Realtime Clock Control Register 0"]
pub type Rtcc0 = crate::RegValueT<Rtcc0_SPEC>;

impl Rtcc0 {
    #[doc = "Fixed-cycle interrupt (RTC_ALM_OR_PRD) selection"]
    #[inline(always)]
    pub fn ct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        rtcc0::Ct,
        rtcc0::Ct,
        Rtcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            rtcc0::Ct,
            rtcc0::Ct,
            Rtcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of 12- or 24-hour system"]
    #[inline(always)]
    pub fn ampm(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rtcc0::Ampm,
        rtcc0::Ampm,
        Rtcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rtcc0::Ampm,
            rtcc0::Ampm,
            Rtcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of the operating clock for the realtime clock (RTCCLK)"]
    #[inline(always)]
    pub fn rtc128en(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rtcc0::Rtc128En,
        rtcc0::Rtc128En,
        Rtcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rtcc0::Rtc128En,
            rtcc0::Rtc128En,
            Rtcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTCOUT pin output control"]
    #[inline(always)]
    pub fn rcloe1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rtcc0::Rcloe1,
        rtcc0::Rcloe1,
        Rtcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rtcc0::Rcloe1,
            rtcc0::Rcloe1,
            Rtcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Realtime clock operation control"]
    #[inline(always)]
    pub fn rtce(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rtcc0::Rtce,
        rtcc0::Rtce,
        Rtcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rtcc0::Rtce,
            rtcc0::Rtce,
            Rtcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rtcc0 {
    #[inline(always)]
    fn default() -> Rtcc0 {
        <crate::RegValueT<Rtcc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rtcc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ct_SPEC;
    pub type Ct = crate::EnumBitfieldStruct<u8, Ct_SPEC>;
    impl Ct {
        #[doc = "Does not use fixed-cycle interrupt"]
        pub const _000: Self = Self::new(0);

        #[doc = "Once per 0.5 s (synchronized with second count up)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Once per 1 s (same time as second count up)"]
        pub const _010: Self = Self::new(2);

        #[doc = "Once per 1 m (second 00 of every minute)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Once per 1 hour (minute 00 and second 00 of every hour)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Once per 1 day (hour 00, minute 00, and second 00 of every day)"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampm_SPEC;
    pub type Ampm = crate::EnumBitfieldStruct<u8, Ampm_SPEC>;
    impl Ampm {
        #[doc = "12-hour system (a.m. and p.m. are displayed.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "24-hour system"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtc128En_SPEC;
    pub type Rtc128En = crate::EnumBitfieldStruct<u8, Rtc128En_SPEC>;
    impl Rtc128En {
        #[doc = "SOSC (32.768 kHz)"]
        pub const _0: Self = Self::new(0);

        #[doc = "SOSC/256 (128 Hz)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcloe1_SPEC;
    pub type Rcloe1 = crate::EnumBitfieldStruct<u8, Rcloe1_SPEC>;
    impl Rcloe1 {
        #[doc = "Disables output of the RTCOUT pin (1 Hz)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables output of the RTCOUT pin (1 Hz)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtce_SPEC;
    pub type Rtce = crate::EnumBitfieldStruct<u8, Rtce_SPEC>;
    impl Rtce {
        #[doc = "Stops counter operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts counter operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtcc1_SPEC;
impl crate::sealed::RegSpec for Rtcc1_SPEC {
    type DataType = u8;
}

#[doc = "Realtime Clock Control Register 1"]
pub type Rtcc1 = crate::RegValueT<Rtcc1_SPEC>;

impl Rtcc1 {
    #[doc = "Wait control of realtime clock"]
    #[inline(always)]
    pub fn rwait(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rtcc1::Rwait,
        rtcc1::Rwait,
        Rtcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rtcc1::Rwait,
            rtcc1::Rwait,
            Rtcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wait status flag of realtime clock"]
    #[inline(always)]
    pub fn rwst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rtcc1::Rwst,
        rtcc1::Rwst,
        Rtcc1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rtcc1::Rwst,
            rtcc1::Rwst,
            Rtcc1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Fixed-cycle interrupt status flag"]
    #[inline(always)]
    pub fn rifg(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rtcc1::Rifg,
        rtcc1::Rifg,
        Rtcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rtcc1::Rifg,
            rtcc1::Rifg,
            Rtcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm detection status flag"]
    #[inline(always)]
    pub fn wafg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rtcc1::Wafg,
        rtcc1::Wafg,
        Rtcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rtcc1::Wafg,
            rtcc1::Wafg,
            Rtcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control of alarm interrupt (RTC_ALM_OR_PRD)"]
    #[inline(always)]
    pub fn walie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rtcc1::Walie,
        rtcc1::Walie,
        Rtcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rtcc1::Walie,
            rtcc1::Walie,
            Rtcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm operation control"]
    #[inline(always)]
    pub fn wale(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rtcc1::Wale,
        rtcc1::Wale,
        Rtcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rtcc1::Wale,
            rtcc1::Wale,
            Rtcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rtcc1 {
    #[inline(always)]
    fn default() -> Rtcc1 {
        <crate::RegValueT<Rtcc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rtcc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwait_SPEC;
    pub type Rwait = crate::EnumBitfieldStruct<u8, Rwait_SPEC>;
    impl Rwait {
        #[doc = "Counting proceeds"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stops the SEC to YEAR counters. Counter values are readable and writable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwst_SPEC;
    pub type Rwst = crate::EnumBitfieldStruct<u8, Rwst_SPEC>;
    impl Rwst {
        #[doc = "Counting is in progress."]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter values are readable and writable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rifg_SPEC;
    pub type Rifg = crate::EnumBitfieldStruct<u8, Rifg_SPEC>;
    impl Rifg {
        #[doc = "Fixed-cycle interrupt is not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Fixed-cycle interrupt is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wafg_SPEC;
    pub type Wafg = crate::EnumBitfieldStruct<u8, Wafg_SPEC>;
    impl Wafg {
        #[doc = "Alarm mismatch"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection of matching of alarm"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Walie_SPEC;
    pub type Walie = crate::EnumBitfieldStruct<u8, Walie_SPEC>;
    impl Walie {
        #[doc = "Does not generate interrupt on matching of alarm."]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates interrupt on matching of alarm."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wale_SPEC;
    pub type Wale = crate::EnumBitfieldStruct<u8, Wale_SPEC>;
    impl Wale {
        #[doc = "Match operation is invalid."]
        pub const _0: Self = Self::new(0);

        #[doc = "Match operation is valid."]
        pub const _1: Self = Self::new(1);
    }
}
