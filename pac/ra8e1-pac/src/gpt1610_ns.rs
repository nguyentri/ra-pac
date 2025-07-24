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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"General PWM 16-bit Timer 10"]
unsafe impl ::core::marker::Send for super::Gpt1610Ns {}
unsafe impl ::core::marker::Sync for super::Gpt1610Ns {}
impl super::Gpt1610Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "General PWM Timer Write-Protection Register"]
    #[inline(always)]
    pub const fn gtwp(&self) -> &'static crate::common::Reg<self::Gtwp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtwp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "General PWM Timer Count Direction and Duty Setting Register"]
    #[inline(always)]
    pub const fn gtuddtyc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtuddtyc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtuddtyc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "General PWM Timer I/O Control Register"]
    #[inline(always)]
    pub const fn gtior(&self) -> &'static crate::common::Reg<self::Gtior_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtior_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "General PWM Timer Status Register"]
    #[inline(always)]
    pub const fn gtst(&self) -> &'static crate::common::Reg<self::Gtst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "General PWM Timer Buffer Enable Register"]
    #[inline(always)]
    pub const fn gtber(&self) -> &'static crate::common::Reg<self::Gtber_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtber_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "General PWM Timer Counter"]
    #[inline(always)]
    pub const fn gtcnt(&self) -> &'static crate::common::Reg<self::Gtcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register A"]
    #[inline(always)]
    pub const fn gtccra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register B"]
    #[inline(always)]
    pub const fn gtccrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register C"]
    #[inline(always)]
    pub const fn gtccrc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register E"]
    #[inline(always)]
    pub const fn gtccre(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register D"]
    #[inline(always)]
    pub const fn gtccrd(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register F"]
    #[inline(always)]
    pub const fn gtccrf(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "General PWM Timer Cycle Setting Register"]
    #[inline(always)]
    pub const fn gtpr(&self) -> &'static crate::common::Reg<self::Gtpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "General PWM Timer Cycle Setting Buffer Register"]
    #[inline(always)]
    pub const fn gtpbr(&self) -> &'static crate::common::Reg<self::Gtpbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "General PWM Timer Dead Time Control Register"]
    #[inline(always)]
    pub const fn gtdtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "General PWM Timer Dead Time Value Register U"]
    #[inline(always)]
    pub const fn gtdvu(&self) -> &'static crate::common::Reg<self::Gtdvu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdvu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "General PWM Timer Period Count Register"]
    #[inline(always)]
    pub const fn gtpc(&self) -> &'static crate::common::Reg<self::Gtpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[doc = "General PWM Timer Operation Enable Bit Simultaneous Control Channel Select Register"]
    #[inline(always)]
    pub const fn gtsecsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtsecsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtsecsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "General PWM Timer Operation Enable Bit Simultaneous Control Register"]
    #[inline(always)]
    pub const fn gtsecr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtsecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtsecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtwp_SPEC;
impl crate::sealed::RegSpec for Gtwp_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Write-Protection Register"]
pub type Gtwp = crate::RegValueT<Gtwp_SPEC>;

impl Gtwp {
    #[doc = "Register Write Disable"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtwp::Wp, gtwp::Wp, Gtwp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtwp::Wp,gtwp::Wp,Gtwp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "GTSTR.CSTRT Bit Write Disable"]
    #[inline(always)]
    pub fn strwp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtwp::Strwp,
        gtwp::Strwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtwp::Strwp,
            gtwp::Strwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTSTP.CSTOP Bit Write Disable"]
    #[inline(always)]
    pub fn stpwp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtwp::Stpwp,
        gtwp::Stpwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtwp::Stpwp,
            gtwp::Stpwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTCLR.CCLR Bit Write Disable"]
    #[inline(always)]
    pub fn clrwp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtwp::Clrwp,
        gtwp::Clrwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtwp::Clrwp,
            gtwp::Clrwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Common Register Write Disabled"]
    #[inline(always)]
    pub fn cmnwp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtwp::Cmnwp,
        gtwp::Cmnwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtwp::Cmnwp,
            gtwp::Cmnwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTWP Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gtwp_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gtwp_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtwp {
    #[inline(always)]
    fn default() -> Gtwp {
        <crate::RegValueT<Gtwp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtwp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write to the register enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to the register disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strwp_SPEC;
    pub type Strwp = crate::EnumBitfieldStruct<u8, Strwp_SPEC>;
    impl Strwp {
        #[doc = "Write to the bit is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to the bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stpwp_SPEC;
    pub type Stpwp = crate::EnumBitfieldStruct<u8, Stpwp_SPEC>;
    impl Stpwp {
        #[doc = "Write to the bit is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to the bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrwp_SPEC;
    pub type Clrwp = crate::EnumBitfieldStruct<u8, Clrwp_SPEC>;
    impl Clrwp {
        #[doc = "Write to the bit is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to the bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmnwp_SPEC;
    pub type Cmnwp = crate::EnumBitfieldStruct<u8, Cmnwp_SPEC>;
    impl Cmnwp {
        #[doc = "Write to the register is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to the register is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuddtyc_SPEC;
impl crate::sealed::RegSpec for Gtuddtyc_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Count Direction and Duty Setting Register"]
pub type Gtuddtyc = crate::RegValueT<Gtuddtyc_SPEC>;

impl Gtuddtyc {
    #[doc = "Count Direction Setting"]
    #[inline(always)]
    pub fn ud(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtuddtyc::Ud,
        gtuddtyc::Ud,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtuddtyc::Ud,
            gtuddtyc::Ud,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Forcible Count Direction Setting"]
    #[inline(always)]
    pub fn udf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtuddtyc::Udf,
        gtuddtyc::Udf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtuddtyc::Udf,
            gtuddtyc::Udf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Output Duty Setting"]
    #[inline(always)]
    pub fn oadty(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtuddtyc::Oadty,
        gtuddtyc::Oadty,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtuddtyc::Oadty,
            gtuddtyc::Oadty,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Forcible GTIOCnA Output Duty Setting"]
    #[inline(always)]
    pub fn oadtyf(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtuddtyc::Oadtyf,
        gtuddtyc::Oadtyf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtuddtyc::Oadtyf,
            gtuddtyc::Oadtyf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Output Value Selecting after Releasing 0%/100% Duty Setting"]
    #[inline(always)]
    pub fn oadtyr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtuddtyc::Oadtyr,
        gtuddtyc::Oadtyr,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtuddtyc::Oadtyr,
            gtuddtyc::Oadtyr,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Output Duty Setting"]
    #[inline(always)]
    pub fn obdty(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        gtuddtyc::Obdty,
        gtuddtyc::Obdty,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            gtuddtyc::Obdty,
            gtuddtyc::Obdty,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Forcible GTIOCnB Output Duty Setting"]
    #[inline(always)]
    pub fn obdtyf(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtuddtyc::Obdtyf,
        gtuddtyc::Obdtyf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtuddtyc::Obdtyf,
            gtuddtyc::Obdtyf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Output Value Selecting after Releasing 0%/100% Duty Setting"]
    #[inline(always)]
    pub fn obdtyr(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtuddtyc::Obdtyr,
        gtuddtyc::Obdtyr,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtuddtyc::Obdtyr,
            gtuddtyc::Obdtyr,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtuddtyc {
    #[inline(always)]
    fn default() -> Gtuddtyc {
        <crate::RegValueT<Gtuddtyc_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod gtuddtyc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ud_SPEC;
    pub type Ud = crate::EnumBitfieldStruct<u8, Ud_SPEC>;
    impl Ud {
        #[doc = "GTCNT counts down"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counts up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udf_SPEC;
    pub type Udf = crate::EnumBitfieldStruct<u8, Udf_SPEC>;
    impl Udf {
        #[doc = "Not forcibly set"]
        pub const _0: Self = Self::new(0);

        #[doc = "Forcibly set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadty_SPEC;
    pub type Oadty = crate::EnumBitfieldStruct<u8, Oadty_SPEC>;
    impl Oadty {
        #[doc = "GTIOCnA pin duty depends on the compare match"]
        pub const _00: Self = Self::new(0);

        #[doc = "GTIOCnA pin duty depends on the compare match"]
        pub const _01: Self = Self::new(1);

        #[doc = "GTIOCnA pin duty 0%"]
        pub const _10: Self = Self::new(2);

        #[doc = "GTIOCnA pin duty 100%"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadtyf_SPEC;
    pub type Oadtyf = crate::EnumBitfieldStruct<u8, Oadtyf_SPEC>;
    impl Oadtyf {
        #[doc = "Not forcibly set"]
        pub const _0: Self = Self::new(0);

        #[doc = "Forcibly set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadtyr_SPEC;
    pub type Oadtyr = crate::EnumBitfieldStruct<u8, Oadtyr_SPEC>;
    impl Oadtyr {
        #[doc = "The function selected by the GTIOA\\[3:2\\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting."]
        pub const _0: Self = Self::new(0);

        #[doc = "The function selected by the GTIOA\\[3:2\\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdty_SPEC;
    pub type Obdty = crate::EnumBitfieldStruct<u8, Obdty_SPEC>;
    impl Obdty {
        #[doc = "GTIOCnB pin duty depends on the compare match"]
        pub const _00: Self = Self::new(0);

        #[doc = "GTIOCnB pin duty depends on the compare match"]
        pub const _01: Self = Self::new(1);

        #[doc = "GTIOCnB pin duty 0%"]
        pub const _10: Self = Self::new(2);

        #[doc = "GTIOCnB pin duty 100%"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdtyf_SPEC;
    pub type Obdtyf = crate::EnumBitfieldStruct<u8, Obdtyf_SPEC>;
    impl Obdtyf {
        #[doc = "Not forcibly set"]
        pub const _0: Self = Self::new(0);

        #[doc = "Forcibly set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdtyr_SPEC;
    pub type Obdtyr = crate::EnumBitfieldStruct<u8, Obdtyr_SPEC>;
    impl Obdtyr {
        #[doc = "The function selected by the GTIOB\\[3:2\\] bits is applied to the output value when the duty cycle is set after release from the 0 or 100% duty-cycle setting."]
        pub const _0: Self = Self::new(0);

        #[doc = "The function selected by the GTIOB\\[3:2\\] bits is applied to the compare match output value which is masked after release from the 0 or 100% duty-cycle setting."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtior_SPEC;
impl crate::sealed::RegSpec for Gtior_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer I/O Control Register"]
pub type Gtior = crate::RegValueT<Gtior_SPEC>;

impl Gtior {
    #[doc = "GTIOCnA Pin Function Select"]
    #[inline(always)]
    pub fn gtioa(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Gtior_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "GTIOCnA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn oadflt(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtior::Oadflt,
        gtior::Oadflt,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtior::Oadflt,
            gtior::Oadflt,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn oahld(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtior::Oahld,
        gtior::Oahld,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtior::Oahld,
            gtior::Oahld,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Output Enable"]
    #[inline(always)]
    pub fn oae(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtior::Oae,
        gtior::Oae,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtior::Oae,
            gtior::Oae,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnA Pin Disable Value Setting"]
    #[inline(always)]
    pub fn oadf(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x3,
        1,
        0,
        gtior::Oadf,
        gtior::Oadf,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x3,
            1,
            0,
            gtior::Oadf,
            gtior::Oadf,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter A Enable"]
    #[inline(always)]
    pub fn nfaen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtior::Nfaen,
        gtior::Nfaen,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtior::Nfaen,
            gtior::Nfaen,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        gtior::Nfcsa,
        gtior::Nfcsa,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            gtior::Nfcsa,
            gtior::Nfcsa,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Function Select"]
    #[inline(always)]
    pub fn gtiob(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Gtior_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "GTIOCnB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn obdflt(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtior::Obdflt,
        gtior::Obdflt,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtior::Obdflt,
            gtior::Obdflt,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn obhld(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtior::Obhld,
        gtior::Obhld,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtior::Obhld,
            gtior::Obhld,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Output Enable"]
    #[inline(always)]
    pub fn obe(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtior::Obe,
        gtior::Obe,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtior::Obe,
            gtior::Obe,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTIOCnB Pin Disable Value Setting"]
    #[inline(always)]
    pub fn obdf(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x3,
        1,
        0,
        gtior::Obdf,
        gtior::Obdf,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x3,
            1,
            0,
            gtior::Obdf,
            gtior::Obdf,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter B Enable"]
    #[inline(always)]
    pub fn nfben(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtior::Nfben,
        gtior::Nfben,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtior::Nfben,
            gtior::Nfben,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsb(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        gtior::Nfcsb,
        gtior::Nfcsb,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            gtior::Nfcsb,
            gtior::Nfcsb,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtior {
    #[inline(always)]
    fn default() -> Gtior {
        <crate::RegValueT<Gtior_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtior {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadflt_SPEC;
    pub type Oadflt = crate::EnumBitfieldStruct<u8, Oadflt_SPEC>;
    impl Oadflt {
        #[doc = "The GTIOCnA pin outputs low when counting stops"]
        pub const _0: Self = Self::new(0);

        #[doc = "The GTIOCnA pin outputs high when counting stops"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oahld_SPEC;
    pub type Oahld = crate::EnumBitfieldStruct<u8, Oahld_SPEC>;
    impl Oahld {
        #[doc = "The GTIOCnA pin output level at the start or stop of counting depends on the register setting"]
        pub const _0: Self = Self::new(0);

        #[doc = "The GTIOCnA pin output level is retained at the start or stop of counting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oae_SPEC;
    pub type Oae = crate::EnumBitfieldStruct<u8, Oae_SPEC>;
    impl Oae {
        #[doc = "Output is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadf_SPEC;
    pub type Oadf = crate::EnumBitfieldStruct<u8, Oadf_SPEC>;
    impl Oadf {
        #[doc = "None of the below options are specified"]
        pub const _00: Self = Self::new(0);

        #[doc = "GTIOCnA pin is set to Hi-Z in response to controlling the output negation"]
        pub const _01: Self = Self::new(1);

        #[doc = "GTIOCnA pin is set to 0 in response to controlling the output negation"]
        pub const _10: Self = Self::new(2);

        #[doc = "GTIOCnA pin is set to 1 in response to controlling the output negation"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfaen_SPEC;
    pub type Nfaen = crate::EnumBitfieldStruct<u8, Nfaen_SPEC>;
    impl Nfaen {
        #[doc = "The noise filter for the GTIOCnA pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "The noise filter for the GTIOCnA pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcsa_SPEC;
    pub type Nfcsa = crate::EnumBitfieldStruct<u8, Nfcsa_SPEC>;
    impl Nfcsa {
        #[doc = "PCLKD/1"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLKD/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLKD/16"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLKD/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdflt_SPEC;
    pub type Obdflt = crate::EnumBitfieldStruct<u8, Obdflt_SPEC>;
    impl Obdflt {
        #[doc = "The GTIOCnB pin outputs low when counting stops"]
        pub const _0: Self = Self::new(0);

        #[doc = "The GTIOCnB pin outputs high when counting stops"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obhld_SPEC;
    pub type Obhld = crate::EnumBitfieldStruct<u8, Obhld_SPEC>;
    impl Obhld {
        #[doc = "The GTIOCnB pin output level at the start/stop of counting depends on the register setting"]
        pub const _0: Self = Self::new(0);

        #[doc = "The GTIOCnB pin output level is retained at the start/stop of counting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obe_SPEC;
    pub type Obe = crate::EnumBitfieldStruct<u8, Obe_SPEC>;
    impl Obe {
        #[doc = "Output is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdf_SPEC;
    pub type Obdf = crate::EnumBitfieldStruct<u8, Obdf_SPEC>;
    impl Obdf {
        #[doc = "None of the below options are specified"]
        pub const _00: Self = Self::new(0);

        #[doc = "GTIOCnB pin is set to Hi-Z in response to controlling the output negation"]
        pub const _01: Self = Self::new(1);

        #[doc = "GTIOCnB pin is set to 0 in response to controlling the output negation"]
        pub const _10: Self = Self::new(2);

        #[doc = "GTIOCnB pin is set to 1 in response to controlling the output negation"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfben_SPEC;
    pub type Nfben = crate::EnumBitfieldStruct<u8, Nfben_SPEC>;
    impl Nfben {
        #[doc = "The noise filter for the GTIOCnB pin is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "The noise filter for the GTIOCnB pin is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcsb_SPEC;
    pub type Nfcsb = crate::EnumBitfieldStruct<u8, Nfcsb_SPEC>;
    impl Nfcsb {
        #[doc = "PCLKD/1"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLKD/4"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLKD/16"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLKD/64"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtst_SPEC;
impl crate::sealed::RegSpec for Gtst_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Status Register"]
pub type Gtst = crate::RegValueT<Gtst_SPEC>;

impl Gtst {
    #[doc = "Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub fn tcfa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtst::Tcfa,
        gtst::Tcfa,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtst::Tcfa,
            gtst::Tcfa,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub fn tcfb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtst::Tcfb,
        gtst::Tcfb,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtst::Tcfb,
            gtst::Tcfb,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input Compare Match Flag C"]
    #[inline(always)]
    pub fn tcfc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtst::Tcfc,
        gtst::Tcfc,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtst::Tcfc,
            gtst::Tcfc,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input Compare Match Flag D"]
    #[inline(always)]
    pub fn tcfd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtst::Tcfd,
        gtst::Tcfd,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtst::Tcfd,
            gtst::Tcfd,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input Compare Match Flag E"]
    #[inline(always)]
    pub fn tcfe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtst::Tcfe,
        gtst::Tcfe,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtst::Tcfe,
            gtst::Tcfe,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input Compare Match Flag F"]
    #[inline(always)]
    pub fn tcff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtst::Tcff,
        gtst::Tcff,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtst::Tcff,
            gtst::Tcff,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overflow Flag"]
    #[inline(always)]
    pub fn tcfpo(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtst::Tcfpo,
        gtst::Tcfpo,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtst::Tcfpo,
            gtst::Tcfpo,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub fn tcfpu(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtst::Tcfpu,
        gtst::Tcfpu,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtst::Tcfpu,
            gtst::Tcfpu,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Count Direction Flag"]
    #[inline(always)]
    pub fn tucf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtst::Tucf,
        gtst::Tucf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtst::Tucf,
            gtst::Tucf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtrauf(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtst::Adtrauf,
        gtst::Adtrauf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtst::Adtrauf,
            gtst::Adtrauf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtradf(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtst::Adtradf,
        gtst::Adtradf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtst::Adtradf,
            gtst::Adtradf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtrbuf(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtst::Adtrbuf,
        gtst::Adtrbuf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtst::Adtrbuf,
            gtst::Adtrbuf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtrbdf(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtst::Adtrbdf,
        gtst::Adtrbdf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtst::Adtrbdf,
            gtst::Adtrbdf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output Disable Flag"]
    #[inline(always)]
    pub fn odf(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtst::Odf,
        gtst::Odf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtst::Odf,
            gtst::Odf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Same Time Output Level High Flag"]
    #[inline(always)]
    pub fn oabhf(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtst::Oabhf,
        gtst::Oabhf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtst::Oabhf,
            gtst::Oabhf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Same Time Output Level Low Flag"]
    #[inline(always)]
    pub fn oablf(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtst::Oablf,
        gtst::Oablf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtst::Oablf,
            gtst::Oablf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Period Count Function Finish Flag"]
    #[inline(always)]
    pub fn pcf(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtst::Pcf,
        gtst::Pcf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtst::Pcf,
            gtst::Pcf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtst {
    #[inline(always)]
    fn default() -> Gtst {
        <crate::RegValueT<Gtst_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod gtst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfa_SPEC;
    pub type Tcfa = crate::EnumBitfieldStruct<u8, Tcfa_SPEC>;
    impl Tcfa {
        #[doc = "No input capture/compare match of GTCCRA is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "An input capture/compare match of GTCCRA is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfb_SPEC;
    pub type Tcfb = crate::EnumBitfieldStruct<u8, Tcfb_SPEC>;
    impl Tcfb {
        #[doc = "No input capture/compare match of GTCCRB is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "An input capture/compare match of GTCCRB is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfc_SPEC;
    pub type Tcfc = crate::EnumBitfieldStruct<u8, Tcfc_SPEC>;
    impl Tcfc {
        #[doc = "No compare match of GTCCRC is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "A compare match of GTCCRC is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfd_SPEC;
    pub type Tcfd = crate::EnumBitfieldStruct<u8, Tcfd_SPEC>;
    impl Tcfd {
        #[doc = "No compare match of GTCCRD is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "A compare match of GTCCRD is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfe_SPEC;
    pub type Tcfe = crate::EnumBitfieldStruct<u8, Tcfe_SPEC>;
    impl Tcfe {
        #[doc = "No compare match of GTCCRE is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "A compare match of GTCCRE is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcff_SPEC;
    pub type Tcff = crate::EnumBitfieldStruct<u8, Tcff_SPEC>;
    impl Tcff {
        #[doc = "No compare match of GTCCRF is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "A compare match of GTCCRF is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfpo_SPEC;
    pub type Tcfpo = crate::EnumBitfieldStruct<u8, Tcfpo_SPEC>;
    impl Tcfpo {
        #[doc = "No overflow (crest) occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An overflow (crest) occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfpu_SPEC;
    pub type Tcfpu = crate::EnumBitfieldStruct<u8, Tcfpu_SPEC>;
    impl Tcfpu {
        #[doc = "No underflow (trough) occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An underflow (trough) occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tucf_SPEC;
    pub type Tucf = crate::EnumBitfieldStruct<u8, Tucf_SPEC>;
    impl Tucf {
        #[doc = "GTCNT counter counts downward"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTCNT counter counts upward"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrauf_SPEC;
    pub type Adtrauf = crate::EnumBitfieldStruct<u8, Adtrauf_SPEC>;
    impl Adtrauf {
        #[doc = "No GTADTRA register compare match has occurred in up-counting."]
        pub const _0: Self = Self::new(0);

        #[doc = "A GTADTRA register compare match has occurred in up-counting."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtradf_SPEC;
    pub type Adtradf = crate::EnumBitfieldStruct<u8, Adtradf_SPEC>;
    impl Adtradf {
        #[doc = "No GTADTRA register compare match has occurred in down-counting."]
        pub const _0: Self = Self::new(0);

        #[doc = "A GTADTRA register compare match has occurred in down-counting."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrbuf_SPEC;
    pub type Adtrbuf = crate::EnumBitfieldStruct<u8, Adtrbuf_SPEC>;
    impl Adtrbuf {
        #[doc = "No GTADTRB register compare match has occurred in up-counting."]
        pub const _0: Self = Self::new(0);

        #[doc = "A GTADTRB register compare match has occurred in up-counting."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrbdf_SPEC;
    pub type Adtrbdf = crate::EnumBitfieldStruct<u8, Adtrbdf_SPEC>;
    impl Adtrbdf {
        #[doc = "No GTADTRB register compare match has occurred in down-counting."]
        pub const _0: Self = Self::new(0);

        #[doc = "A GTADTRB register compare match has occurred in down-counting."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Odf_SPEC;
    pub type Odf = crate::EnumBitfieldStruct<u8, Odf_SPEC>;
    impl Odf {
        #[doc = "No output disable request is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "An output disable request is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oabhf_SPEC;
    pub type Oabhf = crate::EnumBitfieldStruct<u8, Oabhf_SPEC>;
    impl Oabhf {
        #[doc = "No simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oablf_SPEC;
    pub type Oablf = crate::EnumBitfieldStruct<u8, Oablf_SPEC>;
    impl Oablf {
        #[doc = "No simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcf_SPEC;
    pub type Pcf = crate::EnumBitfieldStruct<u8, Pcf_SPEC>;
    impl Pcf {
        #[doc = "No period count function finish has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A period count function finish has occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtber_SPEC;
impl crate::sealed::RegSpec for Gtber_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Buffer Enable Register"]
pub type Gtber = crate::RegValueT<Gtber_SPEC>;

impl Gtber {
    #[doc = "GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtber::Bd0,
        gtber::Bd0,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtber::Bd0,
            gtber::Bd0,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTPR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtber::Bd1,
        gtber::Bd1,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtber::Bd1,
            gtber::Bd1,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRA/GTADTRB Registers Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtber::Bd2,
        gtber::Bd2,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtber::Bd2,
            gtber::Bd2,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTCCRA Buffer Operation"]
    #[inline(always)]
    pub fn ccra(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtber::Ccra,
        gtber::Ccra,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtber::Ccra,
            gtber::Ccra,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTCCRB Buffer Operation"]
    #[inline(always)]
    pub fn ccrb(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        gtber::Ccrb,
        gtber::Ccrb,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            gtber::Ccrb,
            gtber::Ccrb,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTPR Buffer Operation"]
    #[inline(always)]
    pub fn pr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        gtber::Pr,
        gtber::Pr,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            gtber::Pr,
            gtber::Pr,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTCCRA and GTCCRB Forcible Buffer Operation"]
    #[inline(always)]
    pub fn ccrswt(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Gtber_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gtber_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "GTADTRA Register Buffer Transfer Timing Select"]
    #[inline(always)]
    pub fn adtta(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        gtber::Adtta,
        gtber::Adtta,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            gtber::Adtta,
            gtber::Adtta,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRA Register Double Buffer Operation"]
    #[inline(always)]
    pub fn adtda(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtber::Adtda,
        gtber::Adtda,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtber::Adtda,
            gtber::Adtda,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRB Register Buffer Transfer Timing Select"]
    #[inline(always)]
    pub fn adttb(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        gtber::Adttb,
        gtber::Adttb,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            gtber::Adttb,
            gtber::Adttb,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTRB Register Double Buffer Operation"]
    #[inline(always)]
    pub fn adtdb(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtber::Adtdb,
        gtber::Adtdb,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtber::Adtdb,
            gtber::Adtdb,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtber {
    #[inline(always)]
    fn default() -> Gtber {
        <crate::RegValueT<Gtber_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtber {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd0_SPEC;
    pub type Bd0 = crate::EnumBitfieldStruct<u8, Bd0_SPEC>;
    impl Bd0 {
        #[doc = "Buffer operation is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer operation is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd1_SPEC;
    pub type Bd1 = crate::EnumBitfieldStruct<u8, Bd1_SPEC>;
    impl Bd1 {
        #[doc = "Buffer operation is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer operation is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd2_SPEC;
    pub type Bd2 = crate::EnumBitfieldStruct<u8, Bd2_SPEC>;
    impl Bd2 {
        #[doc = "Buffer operation is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer operation is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccra_SPEC;
    pub type Ccra = crate::EnumBitfieldStruct<u8, Ccra_SPEC>;
    impl Ccra {
        #[doc = "No buffer operation"]
        pub const _00: Self = Self::new(0);

        #[doc = "Single buffer operation (GTCCRA <---->GTCCRC)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Double buffer operation (GTCCRA <----> GTCCRC <----> GTCCRD)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccrb_SPEC;
    pub type Ccrb = crate::EnumBitfieldStruct<u8, Ccrb_SPEC>;
    impl Ccrb {
        #[doc = "No buffer operation"]
        pub const _00: Self = Self::new(0);

        #[doc = "Single buffer operation (GTCCRB <----> GTCCRE)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Double buffer operation (GTCCRB <----> GTCCRE <----> GTCCRF)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr_SPEC;
    pub type Pr = crate::EnumBitfieldStruct<u8, Pr_SPEC>;
    impl Pr {
        #[doc = "No buffer operation"]
        pub const _00: Self = Self::new(0);

        #[doc = "Single buffer operation (GTPBR --> GTPR)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtta_SPEC;
    pub type Adtta = crate::EnumBitfieldStruct<u8, Adtta_SPEC>;
    impl Adtta {
        #[doc = "In triangle wave mode, no transfer. In saw-wave mode, no transfer."]
        pub const _00: Self = Self::new(0);

        #[doc = "In triangle wave mode, transfer at crest. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
        pub const _01: Self = Self::new(1);

        #[doc = "In triangle wave mode, transfer at trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
        pub const _10: Self = Self::new(2);

        #[doc = "In triangle wave, transfer at both crest and trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtda_SPEC;
    pub type Adtda = crate::EnumBitfieldStruct<u8, Adtda_SPEC>;
    impl Adtda {
        #[doc = "Single buffer operation (GTADTBRA --> GTADTRA)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Double buffer operation (GTADTDBRA --> GTADTBRA --> GTADTRA)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adttb_SPEC;
    pub type Adttb = crate::EnumBitfieldStruct<u8, Adttb_SPEC>;
    impl Adttb {
        #[doc = "In triangle wave mode, no transfer. In saw-wave mode, no transfer."]
        pub const _00: Self = Self::new(0);

        #[doc = "In triangle wave mode, transfer at crest. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
        pub const _01: Self = Self::new(1);

        #[doc = "In triangle wave mode, transfer at trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
        pub const _10: Self = Self::new(2);

        #[doc = "In triangle wave mode, transfer at both crest and trough. In saw-wave mode, transfer at underflow (in down-counting), overflow (in up-counting), or counter clearing."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtdb_SPEC;
    pub type Adtdb = crate::EnumBitfieldStruct<u8, Adtdb_SPEC>;
    impl Adtdb {
        #[doc = "Single buffer operation (GTADTBRB --> GTADTRB)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Double buffer operation (GTADTDBRB --> GTADTBRB --> GTADTRB)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcnt_SPEC;
impl crate::sealed::RegSpec for Gtcnt_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Counter"]
pub type Gtcnt = crate::RegValueT<Gtcnt_SPEC>;

impl NoBitfieldReg<Gtcnt_SPEC> for Gtcnt {}
impl ::core::default::Default for Gtcnt {
    #[inline(always)]
    fn default() -> Gtcnt {
        <crate::RegValueT<Gtcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccra_SPEC;
impl crate::sealed::RegSpec for Gtccra_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Compare Capture Register A"]
pub type Gtccra = crate::RegValueT<Gtccra_SPEC>;

impl NoBitfieldReg<Gtccra_SPEC> for Gtccra {}
impl ::core::default::Default for Gtccra {
    #[inline(always)]
    fn default() -> Gtccra {
        <crate::RegValueT<Gtccra_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrb_SPEC;
impl crate::sealed::RegSpec for Gtccrb_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Compare Capture Register B"]
pub type Gtccrb = crate::RegValueT<Gtccrb_SPEC>;

impl NoBitfieldReg<Gtccrb_SPEC> for Gtccrb {}
impl ::core::default::Default for Gtccrb {
    #[inline(always)]
    fn default() -> Gtccrb {
        <crate::RegValueT<Gtccrb_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrc_SPEC;
impl crate::sealed::RegSpec for Gtccrc_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Compare Capture Register C"]
pub type Gtccrc = crate::RegValueT<Gtccrc_SPEC>;

impl NoBitfieldReg<Gtccrc_SPEC> for Gtccrc {}
impl ::core::default::Default for Gtccrc {
    #[inline(always)]
    fn default() -> Gtccrc {
        <crate::RegValueT<Gtccrc_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccre_SPEC;
impl crate::sealed::RegSpec for Gtccre_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Compare Capture Register E"]
pub type Gtccre = crate::RegValueT<Gtccre_SPEC>;

impl NoBitfieldReg<Gtccre_SPEC> for Gtccre {}
impl ::core::default::Default for Gtccre {
    #[inline(always)]
    fn default() -> Gtccre {
        <crate::RegValueT<Gtccre_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrd_SPEC;
impl crate::sealed::RegSpec for Gtccrd_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Compare Capture Register D"]
pub type Gtccrd = crate::RegValueT<Gtccrd_SPEC>;

impl NoBitfieldReg<Gtccrd_SPEC> for Gtccrd {}
impl ::core::default::Default for Gtccrd {
    #[inline(always)]
    fn default() -> Gtccrd {
        <crate::RegValueT<Gtccrd_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrf_SPEC;
impl crate::sealed::RegSpec for Gtccrf_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Compare Capture Register F"]
pub type Gtccrf = crate::RegValueT<Gtccrf_SPEC>;

impl NoBitfieldReg<Gtccrf_SPEC> for Gtccrf {}
impl ::core::default::Default for Gtccrf {
    #[inline(always)]
    fn default() -> Gtccrf {
        <crate::RegValueT<Gtccrf_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpr_SPEC;
impl crate::sealed::RegSpec for Gtpr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Cycle Setting Register"]
pub type Gtpr = crate::RegValueT<Gtpr_SPEC>;

impl NoBitfieldReg<Gtpr_SPEC> for Gtpr {}
impl ::core::default::Default for Gtpr {
    #[inline(always)]
    fn default() -> Gtpr {
        <crate::RegValueT<Gtpr_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpbr_SPEC;
impl crate::sealed::RegSpec for Gtpbr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Cycle Setting Buffer Register"]
pub type Gtpbr = crate::RegValueT<Gtpbr_SPEC>;

impl NoBitfieldReg<Gtpbr_SPEC> for Gtpbr {}
impl ::core::default::Default for Gtpbr {
    #[inline(always)]
    fn default() -> Gtpbr {
        <crate::RegValueT<Gtpbr_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdtcr_SPEC;
impl crate::sealed::RegSpec for Gtdtcr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Dead Time Control Register"]
pub type Gtdtcr = crate::RegValueT<Gtdtcr_SPEC>;

impl Gtdtcr {
    #[doc = "Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtdtcr::Tde,
        gtdtcr::Tde,
        Gtdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtdtcr::Tde,
            gtdtcr::Tde,
            Gtdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdtcr {
    #[inline(always)]
    fn default() -> Gtdtcr {
        <crate::RegValueT<Gtdtcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        #[doc = "GTCCRB is set without using GTDVU"]
        pub const _0: Self = Self::new(0);

        #[doc = "GTDVU is used to set the compare match value for negative-phase waveform with dead time automatically in GTCCRB"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdvu_SPEC;
impl crate::sealed::RegSpec for Gtdvu_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Dead Time Value Register U"]
pub type Gtdvu = crate::RegValueT<Gtdvu_SPEC>;

impl NoBitfieldReg<Gtdvu_SPEC> for Gtdvu {}
impl ::core::default::Default for Gtdvu {
    #[inline(always)]
    fn default() -> Gtdvu {
        <crate::RegValueT<Gtdvu_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpc_SPEC;
impl crate::sealed::RegSpec for Gtpc_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Period Count Register"]
pub type Gtpc = crate::RegValueT<Gtpc_SPEC>;

impl Gtpc {
    #[doc = "Period Count Function Enable"]
    #[inline(always)]
    pub fn pcen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtpc::Pcen,
        gtpc::Pcen,
        Gtpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtpc::Pcen,
            gtpc::Pcen,
            Gtpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Automatic Stop Function Enable"]
    #[inline(always)]
    pub fn astp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtpc::Astp,
        gtpc::Astp,
        Gtpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtpc::Astp,
            gtpc::Astp,
            Gtpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Period Counter"]
    #[inline(always)]
    pub fn pcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Gtpc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Gtpc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtpc {
    #[inline(always)]
    fn default() -> Gtpc {
        <crate::RegValueT<Gtpc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtpc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcen_SPEC;
    pub type Pcen = crate::EnumBitfieldStruct<u8, Pcen_SPEC>;
    impl Pcen {
        #[doc = "Period count function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Period count function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Astp_SPEC;
    pub type Astp = crate::EnumBitfieldStruct<u8, Astp_SPEC>;
    impl Astp {
        #[doc = "Automatic stop function is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatic stop function is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtsecsr_SPEC;
impl crate::sealed::RegSpec for Gtsecsr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Operation Enable Bit Simultaneous Control Channel Select Register"]
pub type Gtsecsr = crate::RegValueT<Gtsecsr_SPEC>;

impl Gtsecsr {
    #[doc = "Channel 0 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtsecsr::Secsel0,
        gtsecsr::Secsel0,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtsecsr::Secsel0,
            gtsecsr::Secsel0,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 1 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtsecsr::Secsel1,
        gtsecsr::Secsel1,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtsecsr::Secsel1,
            gtsecsr::Secsel1,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 2 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtsecsr::Secsel2,
        gtsecsr::Secsel2,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtsecsr::Secsel2,
            gtsecsr::Secsel2,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 3 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtsecsr::Secsel3,
        gtsecsr::Secsel3,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtsecsr::Secsel3,
            gtsecsr::Secsel3,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 4 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtsecsr::Secsel4,
        gtsecsr::Secsel4,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtsecsr::Secsel4,
            gtsecsr::Secsel4,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 5 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtsecsr::Secsel5,
        gtsecsr::Secsel5,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtsecsr::Secsel5,
            gtsecsr::Secsel5,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 10 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtsecsr::Secsel10,
        gtsecsr::Secsel10,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtsecsr::Secsel10,
            gtsecsr::Secsel10,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 11 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtsecsr::Secsel11,
        gtsecsr::Secsel11,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtsecsr::Secsel11,
            gtsecsr::Secsel11,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 12 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtsecsr::Secsel12,
        gtsecsr::Secsel12,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtsecsr::Secsel12,
            gtsecsr::Secsel12,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channel 13 Operation Enable Bit Simultaneous Control Channel Select"]
    #[inline(always)]
    pub fn secsel13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtsecsr::Secsel13,
        gtsecsr::Secsel13,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtsecsr::Secsel13,
            gtsecsr::Secsel13,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtsecsr {
    #[inline(always)]
    fn default() -> Gtsecsr {
        <crate::RegValueT<Gtsecsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtsecsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel0_SPEC;
    pub type Secsel0 = crate::EnumBitfieldStruct<u8, Secsel0_SPEC>;
    impl Secsel0 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel1_SPEC;
    pub type Secsel1 = crate::EnumBitfieldStruct<u8, Secsel1_SPEC>;
    impl Secsel1 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel2_SPEC;
    pub type Secsel2 = crate::EnumBitfieldStruct<u8, Secsel2_SPEC>;
    impl Secsel2 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel3_SPEC;
    pub type Secsel3 = crate::EnumBitfieldStruct<u8, Secsel3_SPEC>;
    impl Secsel3 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel4_SPEC;
    pub type Secsel4 = crate::EnumBitfieldStruct<u8, Secsel4_SPEC>;
    impl Secsel4 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel5_SPEC;
    pub type Secsel5 = crate::EnumBitfieldStruct<u8, Secsel5_SPEC>;
    impl Secsel5 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel10_SPEC;
    pub type Secsel10 = crate::EnumBitfieldStruct<u8, Secsel10_SPEC>;
    impl Secsel10 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel11_SPEC;
    pub type Secsel11 = crate::EnumBitfieldStruct<u8, Secsel11_SPEC>;
    impl Secsel11 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel12_SPEC;
    pub type Secsel12 = crate::EnumBitfieldStruct<u8, Secsel12_SPEC>;
    impl Secsel12 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel13_SPEC;
    pub type Secsel13 = crate::EnumBitfieldStruct<u8, Secsel13_SPEC>;
    impl Secsel13 {
        #[doc = "Disable simultaneous control"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable simultaneous control"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtsecr_SPEC;
impl crate::sealed::RegSpec for Gtsecr_SPEC {
    type DataType = u32;
}

#[doc = "General PWM Timer Operation Enable Bit Simultaneous Control Register"]
pub type Gtsecr = crate::RegValueT<Gtsecr_SPEC>;

impl Gtsecr {
    #[doc = "GTCCR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtsecr::Sbdce,
        gtsecr::Sbdce,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtsecr::Sbdce,
            gtsecr::Sbdce,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTPR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdpe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtsecr::Sbdpe,
        gtsecr::Sbdpe,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtsecr::Sbdpe,
            gtsecr::Sbdpe,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTR Register Buffer Operation Simultaneous Enable"]
    #[inline(always)]
    pub fn sbdae(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtsecr::Sbdae,
        gtsecr::Sbdae,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtsecr::Sbdae,
            gtsecr::Sbdae,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTCCR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdcd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtsecr::Sbdcd,
        gtsecr::Sbdcd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtsecr::Sbdcd,
            gtsecr::Sbdcd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTPR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdpd(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtsecr::Sbdpd,
        gtsecr::Sbdpd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtsecr::Sbdpd,
            gtsecr::Sbdpd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GTADTR Register Buffer Operation Simultaneous Disable"]
    #[inline(always)]
    pub fn sbdad(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtsecr::Sbdad,
        gtsecr::Sbdad,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtsecr::Sbdad,
            gtsecr::Sbdad,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Period Count Function Simultaneous Enable"]
    #[inline(always)]
    pub fn spce(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtsecr::Spce,
        gtsecr::Spce,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtsecr::Spce,
            gtsecr::Spce,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Period Count Function Simultaneous Disable"]
    #[inline(always)]
    pub fn spcd(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtsecr::Spcd,
        gtsecr::Spcd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtsecr::Spcd,
            gtsecr::Spcd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtsecr {
    #[inline(always)]
    fn default() -> Gtsecr {
        <crate::RegValueT<Gtsecr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtsecr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdce_SPEC;
    pub type Sbdce = crate::EnumBitfieldStruct<u8, Sbdce_SPEC>;
    impl Sbdce {
        #[doc = "Disable simultaneous enabling GTCCR buffer operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GTCCR register buffer operations simultaneously"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdpe_SPEC;
    pub type Sbdpe = crate::EnumBitfieldStruct<u8, Sbdpe_SPEC>;
    impl Sbdpe {
        #[doc = "Disable simultaneous enabling GTPR buffer operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GTPR register buffer operations simultaneously"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdae_SPEC;
    pub type Sbdae = crate::EnumBitfieldStruct<u8, Sbdae_SPEC>;
    impl Sbdae {
        #[doc = "Disable simultaneous enabling GTADTR buffer operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GTADTR register buffer operations simultaneously"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdcd_SPEC;
    pub type Sbdcd = crate::EnumBitfieldStruct<u8, Sbdcd_SPEC>;
    impl Sbdcd {
        #[doc = "Disable simultaneous disabling GTCCR buffer operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable GTCCR register buffer operations simultaneously"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdpd_SPEC;
    pub type Sbdpd = crate::EnumBitfieldStruct<u8, Sbdpd_SPEC>;
    impl Sbdpd {
        #[doc = "Disable simultaneous disabling GTPR buffer operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable GTPR register buffer operations simultaneously"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdad_SPEC;
    pub type Sbdad = crate::EnumBitfieldStruct<u8, Sbdad_SPEC>;
    impl Sbdad {
        #[doc = "Disable simultaneous disabling GTADTR buffer operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable GTADTR register buffer operations simultaneously"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spce_SPEC;
    pub type Spce = crate::EnumBitfieldStruct<u8, Spce_SPEC>;
    impl Spce {
        #[doc = "Disable simultaneous enabling period count function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable period count function simultaneously"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcd_SPEC;
    pub type Spcd = crate::EnumBitfieldStruct<u8, Spcd_SPEC>;
    impl Spcd {
        #[doc = "Disable simultaneous disabling period count function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable period count function simultaneously"]
        pub const _1: Self = Self::new(1);
    }
}
