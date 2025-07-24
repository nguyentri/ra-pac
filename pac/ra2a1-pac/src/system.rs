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
// Generated from SVD 1.1, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:11 +0000

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

    #[doc = "Memory Wait Cycle Control Register"]
    #[inline(always)]
    pub const fn memwait(
        &self,
    ) -> &'static crate::common::Reg<self::Memwait_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Memwait_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
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

    #[doc = "24-bit Sigma-Delta A/D Converter Clock Control Register"]
    #[inline(always)]
    pub const fn sdadcckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadcckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(209usize),
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

    #[doc = "Sub-clock Oscillator Control Register"]
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

    #[doc = "Sub-clock Oscillator Mode Control Register"]
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

    #[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
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

    #[doc = "Flash Operation Control Register"]
    #[inline(always)]
    pub const fn flstop(
        &self,
    ) -> &'static crate::common::Reg<self::Flstop_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Flstop_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(158usize),
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

    #[doc = "Voltage Monitor 1 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd1cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Cr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Cr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1050usize),
            )
        }
    }

    #[doc = "Voltage Monitor 2 Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvd2cr0(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd2Cr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd2Cr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1051usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvd1cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Cr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Cr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "Voltage Monitor 1 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd1sr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd1Sr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd1Sr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(225usize),
            )
        }
    }

    #[doc = "Voltage Monitor 2 Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvd2cr1(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd2Cr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd2Cr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(226usize),
            )
        }
    }

    #[doc = "Voltage Monitor 2 Circuit Status Register"]
    #[inline(always)]
    pub const fn lvd2sr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvd2Sr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvd2Sr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(227usize),
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Sckdivcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Sckdivcr_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<Sckdivcr_SPEC> as RegisterValue<_>>::new(1140851716)
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
pub struct Sckscr_SPEC;
impl crate::sealed::RegSpec for Sckscr_SPEC {
    type DataType = u8;
}

#[doc = "System Clock Source Control Register"]
pub type Sckscr = crate::RegValueT<Sckscr_SPEC>;

impl Sckscr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Sckscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Sckscr_SPEC,crate::common::RW>::from_register(self,0)
    }

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

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memwait_SPEC;
impl crate::sealed::RegSpec for Memwait_SPEC {
    type DataType = u8;
}

#[doc = "Memory Wait Cycle Control Register"]
pub type Memwait = crate::RegValueT<Memwait_SPEC>;

impl Memwait {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Memwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Memwait_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Memory Wait Cycle Select"]
    #[inline(always)]
    pub fn memwait(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        memwait::Memwait,
        memwait::Memwait,
        Memwait_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            memwait::Memwait,
            memwait::Memwait,
            Memwait_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Memwait {
    #[inline(always)]
    fn default() -> Memwait {
        <crate::RegValueT<Memwait_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memwait {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Memwait_SPEC;
    pub type Memwait = crate::EnumBitfieldStruct<u8, Memwait_SPEC>;
    impl Memwait {
        #[doc = "No wait"]
        pub const _0: Self = Self::new(0);

        #[doc = "Wait"]
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
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mosccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mosccr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Hococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Hococr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        #[doc = "HOCO is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO is stopped."]
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
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mococr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        #[doc = "MOCO is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "MOCO is stopped."]
        pub const _1: Self = Self::new(1);
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

    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Oscsf_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Oscsf_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Moscsf_SPEC;
    pub type Moscsf = crate::EnumBitfieldStruct<u8, Moscsf_SPEC>;
    impl Moscsf {
        #[doc = "MOSTP = 1 (stopping the main clock oscillator) or oscillation of the main clock has not yet become stable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillation of the main clock is stable so the clock is available for use as the system clock."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hocosf_SPEC;
    pub type Hocosf = crate::EnumBitfieldStruct<u8, Hocosf_SPEC>;
    impl Hocosf {
        #[doc = "The HOCO clock is stopped or oscillation of the HOCO clock has not yet become stable."]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillation of the HOCO clock is stable so the clock is available for use as the system clock."]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ckocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ckocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        #[doc = "Clock Out disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock Out enable"]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Ostdcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Ostdcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Oscillation stop detection function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillation stop detection function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdie_SPEC;
    pub type Ostdie = crate::EnumBitfieldStruct<u8, Ostdie_SPEC>;
    impl Ostdie {
        #[doc = "The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
        pub const _0: Self = Self::new(0);

        #[doc = "The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
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
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Ostdsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Ostdsr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        #[doc = "The main clock oscillation stop has not been detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The main clock oscillation stop has been detected."]
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
pub struct Sdadcckcr_SPEC;
impl crate::sealed::RegSpec for Sdadcckcr_SPEC {
    type DataType = u8;
}

#[doc = "24-bit Sigma-Delta A/D Converter Clock Control Register"]
pub type Sdadcckcr = crate::RegValueT<Sdadcckcr_SPEC>;

impl Sdadcckcr {
    #[doc = "24-bit Sigma-Delta A/D Converter Clock Select"]
    #[inline(always)]
    pub fn sdadccken(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sdadcckcr::Sdadccken,
        sdadcckcr::Sdadccken,
        Sdadcckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sdadcckcr::Sdadccken,
            sdadcckcr::Sdadccken,
            Sdadcckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Sdadcckcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Sdadcckcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "24-bit Sigma-Delta A/D Converter Clock Enable"]
    #[inline(always)]
    pub fn sdadccksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadcckcr::Sdadccksel,
        sdadcckcr::Sdadccksel,
        Sdadcckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadcckcr::Sdadccksel,
            sdadcckcr::Sdadccksel,
            Sdadcckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadcckcr {
    #[inline(always)]
    fn default() -> Sdadcckcr {
        <crate::RegValueT<Sdadcckcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadcckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadccken_SPEC;
    pub type Sdadccken = crate::EnumBitfieldStruct<u8, Sdadccken_SPEC>;
    impl Sdadccken {
        #[doc = "MOSC is chosen by a source clock of 24-bit Sigma-Delta A/D Converter Clock"]
        pub const _0: Self = Self::new(0);

        #[doc = "HOCO is chosen by a source clock of 24-bit Sigma-Delta A/D Converter Clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadccksel_SPEC;
    pub type Sdadccksel = crate::EnumBitfieldStruct<u8, Sdadccksel_SPEC>;
    impl Sdadccksel {
        #[doc = "24-bit Sigma-Delta A/D Converter Clock is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "24-bit Sigma-Delta A/D Converter Clock is enabled"]
        pub const _1: Self = Self::new(1);
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

    #[doc = "Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    pub fn modrv1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        momcr::Modrv1,
        momcr::Modrv1,
        Momcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            momcr::Modrv1,
            momcr::Modrv1,
            Momcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Momcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Momcr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Mosel_SPEC;
    pub type Mosel = crate::EnumBitfieldStruct<u8, Mosel_SPEC>;
    impl Mosel {
        #[doc = "Resonator"]
        pub const _0: Self = Self::new(0);

        #[doc = "External clock input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modrv1_SPEC;
    pub type Modrv1 = crate::EnumBitfieldStruct<u8, Modrv1_SPEC>;
    impl Modrv1 {
        #[doc = "10 MHz to 20 MHz"]
        pub const _0: Self = Self::new(0);

        #[doc = "1 MHz to 10 MHz"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccr_SPEC;
impl crate::sealed::RegSpec for Sosccr_SPEC {
    type DataType = u8;
}

#[doc = "Sub-clock Oscillator Control Register"]
pub type Sosccr = crate::RegValueT<Sosccr_SPEC>;

impl Sosccr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sosccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sosccr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        <crate::RegValueT<Sosccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostp_SPEC;
    pub type Sostp = crate::EnumBitfieldStruct<u8, Sostp_SPEC>;
    impl Sostp {
        #[doc = "Sub-clock oscillator is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "Sub-clock oscillator is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Somcr_SPEC;
impl crate::sealed::RegSpec for Somcr_SPEC {
    type DataType = u8;
}

#[doc = "Sub-clock Oscillator Mode Control Register"]
pub type Somcr = crate::RegValueT<Somcr_SPEC>;

impl Somcr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Somcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Somcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sub Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        somcr::Sodrv,
        somcr::Sodrv,
        Somcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            somcr::Sodrv,
            somcr::Sodrv,
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
    pub struct Sodrv_SPEC;
    pub type Sodrv = crate::EnumBitfieldStruct<u8, Sodrv_SPEC>;
    impl Sodrv {
        #[doc = "Normal Mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low power mode 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "Low power mode 2"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low power mode 3"]
        pub const _11: Self = Self::new(3);
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
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Lococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Lococr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        #[doc = "LOCO is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "LOCO is stopped."]
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
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Moscwtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Moscwtcr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        #[doc = "Wait time= 3 cycles (11.4us : calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Wait time= 35 cycles (133.5us : calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Wait time= 67 cycles (255.6us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Wait time= 131 cycles (499.7us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Wait time= 259 cycles (988.0us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Wait time= 547 cycles (2086.6us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0101: Self = Self::new(5);

        #[doc = "Wait time= 1059 cycles (4039.8us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0110: Self = Self::new(6);

        #[doc = "Wait time= 2147 cycles (8190.2us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _0111: Self = Self::new(7);

        #[doc = "Wait time= 4291 cycles (16368.9us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _1000: Self = Self::new(8);

        #[doc = "Wait time= 8163 cycles (31139.4us: calculated at LOCO=262.144KHz (3.81us TYP.))"]
        pub const _1001: Self = Self::new(9);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocowtcr_SPEC;
impl crate::sealed::RegSpec for Hocowtcr_SPEC {
    type DataType = u8;
}

#[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
pub type Hocowtcr = crate::RegValueT<Hocowtcr_SPEC>;

impl Hocowtcr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Hocowtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Hocowtcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "HOCO wait time setting"]
    #[inline(always)]
    pub fn hsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        hocowtcr::Hsts,
        hocowtcr::Hsts,
        Hocowtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            hocowtcr::Hsts,
            hocowtcr::Hsts,
            Hocowtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hocowtcr {
    #[inline(always)]
    fn default() -> Hocowtcr {
        <crate::RegValueT<Hocowtcr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod hocowtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsts_SPEC;
    pub type Hsts = crate::EnumBitfieldStruct<u8, Hsts_SPEC>;
    impl Hsts {
        #[doc = "If HOCO frequency is other than 64MHz, should set the value to 101b."]
        pub const _101: Self = Self::new(5);

        #[doc = "If HOCO frequency = 64MHz, should set the value to 110b."]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Sbycr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Sbycr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sbycr {
    #[inline(always)]
    fn default() -> Sbycr {
        <crate::RegValueT<Sbycr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssby_SPEC;
    pub type Ssby = crate::EnumBitfieldStruct<u8, Ssby_SPEC>;
    impl Ssby {
        #[doc = "Sleep Mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby Mode"]
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
    #[doc = "Data Transfer Controller Module Stop"]
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

    #[doc = "These bits are read as 1111111111111111111111. The write value should be 1111111111111111111111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, u32, Mstpcra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32,u32,Mstpcra_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4290772991)
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, Snzcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,Snzcr_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Snzedcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Snzedcr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn agtunfed(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzedcr::Agtunfed,
        snzedcr::Agtunfed,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzedcr::Agtunfed,
            snzedcr::Agtunfed,
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
    pub struct Agtunfed_SPEC;
    pub type Agtunfed = crate::EnumBitfieldStruct<u8, Agtunfed_SPEC>;
    impl Agtunfed {
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
    #[doc = "Snooze Request Enable 30Enable AGT1 compare match B snooze request"]
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

    #[doc = "Snooze Request Enable 29Enable AGT1 compare match A snooze request"]
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

    #[doc = "Snooze Request Enable 28Enable AGT1 underflow snooze request"]
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

    #[doc = "Snooze Request Enable 25Enable RTC period snooze request"]
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

    #[doc = "Snooze Request Enable 24Enable RTC alarm snooze request"]
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

    #[doc = "Snooze Request Enable 24Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen23,
        snzreqcr::Snzreqen23,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen23,
            snzreqcr::Snzreqen23,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Snooze Request Enable 17Enable KINT snooze request"]
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

    #[doc = "These bits are read as 000000000. The write value should be 000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0x1ff, 1, 0, u16, u16, Snzreqcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1ff,1,0,u16,u16,Snzreqcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Snooze Request Enable 7Enable IRQ7 pin snooze request"]
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

    #[doc = "Snooze Request Enable 6Enable IRQ6 pin snooze request"]
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

    #[doc = "Snooze Request Enable 5Enable IRQ5 pin snooze request"]
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

    #[doc = "Snooze Request Enable 4Enable IRQ4 pin snooze request"]
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

    #[doc = "Snooze Request Enable 3Enable IRQ3 pin snooze request"]
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

    #[doc = "Snooze Request Enable 2Enable IRQ2 pin snooze request"]
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

    #[doc = "Snooze Request Enable 1Enable IRQ1 pin snooze request"]
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

    #[doc = "Snooze Request Enable 0Enable IRQ0 pin snooze request"]
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
    pub struct Snzreqen23_SPEC;
    pub type Snzreqen23 = crate::EnumBitfieldStruct<u8, Snzreqen23_SPEC>;
    impl Snzreqen23 {
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
pub struct Flstop_SPEC;
impl crate::sealed::RegSpec for Flstop_SPEC {
    type DataType = u8;
}

#[doc = "Flash Operation Control Register"]
pub type Flstop = crate::RegValueT<Flstop_SPEC>;

impl Flstop {
    #[doc = "Flash Memory Operation Status Flag"]
    #[inline(always)]
    pub fn flstpf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        flstop::Flstpf,
        flstop::Flstpf,
        Flstop_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            flstop::Flstpf,
            flstop::Flstpf,
            Flstop_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Flstop_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Flstop_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    pub fn flstop(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        flstop::Flstop,
        flstop::Flstop,
        Flstop_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            flstop::Flstop,
            flstop::Flstop,
            Flstop_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Flstop {
    #[inline(always)]
    fn default() -> Flstop {
        <crate::RegValueT<Flstop_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flstop {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flstpf_SPEC;
    pub type Flstpf = crate::EnumBitfieldStruct<u8, Flstpf_SPEC>;
    impl Flstpf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);

        #[doc = "During transition (from the flash-stop-status to flashoperating- status or vice versa)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flstop_SPEC;
    pub type Flstop = crate::EnumBitfieldStruct<u8, Flstop_SPEC>;
    impl Flstop {
        #[doc = "Code flash memory operates. Data flash memory operation depends on DFLCTL.DFLEN bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Code flash/Data flash memory stops"]
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Opccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Opccr_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<Opccr_SPEC> as RegisterValue<_>>::new(2)
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Sopccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Sopccr_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Lvcmpcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Lvcmpcr_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
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

    #[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
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
        <crate::RegValueT<Lvdlvlr_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod lvdlvlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Lvl_SPEC;
    pub type Lvd2Lvl = crate::EnumBitfieldStruct<u8, Lvd2Lvl_SPEC>;
    impl Lvd2Lvl {
        #[doc = "4.29V(Vdet2_0)"]
        pub const _000: Self = Self::new(0);

        #[doc = "4.14V(Vdet2_1)"]
        pub const _001: Self = Self::new(1);

        #[doc = "4.02V(Vdet2_2)"]
        pub const _010: Self = Self::new(2);

        #[doc = "3.84V(Vdet2_3)"]
        pub const _011: Self = Self::new(3);

        #[doc = "Settingprohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Lvl_SPEC;
    pub type Lvd1Lvl = crate::EnumBitfieldStruct<u8, Lvd1Lvl_SPEC>;
    impl Lvd1Lvl {
        #[doc = "4.29V(Vdet1_0)"]
        pub const _00000: Self = Self::new(0);

        #[doc = "4.14V(Vdet1_1)"]
        pub const _00001: Self = Self::new(1);

        #[doc = "4.02V(Vdet1_2)"]
        pub const _00010: Self = Self::new(2);

        #[doc = "3.84V(Vdet1_3)"]
        pub const _00011: Self = Self::new(3);

        #[doc = "3.10V(Vdet1_4)"]
        pub const _00100: Self = Self::new(4);

        #[doc = "3.00V(Vdet1_5)"]
        pub const _00101: Self = Self::new(5);

        #[doc = "2.90V(Vdet1_6)"]
        pub const _00110: Self = Self::new(6);

        #[doc = "2.79V(Vdet1_7)"]
        pub const _00111: Self = Self::new(7);

        #[doc = "2.68V(Vdet1_8)"]
        pub const _01000: Self = Self::new(8);

        #[doc = "2.58V(Vdet1_9)"]
        pub const _01001: Self = Self::new(9);

        #[doc = "2.48V(Vdet1_A)"]
        pub const _01010: Self = Self::new(10);

        #[doc = "2.20V(Vdet1_B)"]
        pub const _01011: Self = Self::new(11);

        #[doc = "1.96V(Vdet1_C)"]
        pub const _01100: Self = Self::new(12);

        #[doc = "1.86V(Vdet1_D)"]
        pub const _01101: Self = Self::new(13);

        #[doc = "1.75V(Vdet1_E)"]
        pub const _01110: Self = Self::new(14);

        #[doc = "1.65V(Vdet1_F)"]
        pub const _01111: Self = Self::new(15);

        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Cr0_SPEC;
impl crate::sealed::RegSpec for Lvd1Cr0_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Control Register 0"]
pub type Lvd1Cr0 = crate::RegValueT<Lvd1Cr0_SPEC>;

impl Lvd1Cr0 {
    #[doc = "Voltage Monitor 1 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvd1cr0::Rn,
        lvd1cr0::Rn,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvd1cr0::Rn,
            lvd1cr0::Rn,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvd1cr0::Ri,
        lvd1cr0::Ri,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvd1cr0::Ri,
            lvd1cr0::Ri,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd1cr0::Cmpe,
        lvd1cr0::Cmpe,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd1cr0::Cmpe,
            lvd1cr0::Cmpe,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Lvd1Cr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Lvd1Cr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Voltage Monitor 1 Interrupt/ Reset Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd1cr0::Rie,
        lvd1cr0::Rie,
        Lvd1Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd1cr0::Rie,
            lvd1cr0::Rie,
            Lvd1Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Cr0 {
    #[inline(always)]
    fn default() -> Lvd1Cr0 {
        <crate::RegValueT<Lvd1Cr0_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod lvd1cr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rn_SPEC;
    pub type Rn = crate::EnumBitfieldStruct<u8, Rn_SPEC>;
    impl Rn {
        #[doc = "Negation follows a stabilization time (tLVD1) after VCC > Vdet1 is detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Negation follows a stabilization time (tLVD1) after assertion of the LVD1 reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri_SPEC;
    pub type Ri = crate::EnumBitfieldStruct<u8, Ri_SPEC>;
    impl Ri {
        #[doc = "Voltage monitor 1 interrupt during Vdet1 passage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage monitor 1 reset enabled when the voltage falls to and below Vdet1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Voltage monitor 1 circuit comparison result output disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage monitor 1 circuit comparison result output enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd2Cr0_SPEC;
impl crate::sealed::RegSpec for Lvd2Cr0_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 2 Circuit Control Register 0"]
pub type Lvd2Cr0 = crate::RegValueT<Lvd2Cr0_SPEC>;

impl Lvd2Cr0 {
    #[doc = "Voltage Monitor 2 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lvd2cr0::Rn,
        lvd2cr0::Rn,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lvd2cr0::Rn,
            lvd2cr0::Rn,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lvd2cr0::Ri,
        lvd2cr0::Ri,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lvd2cr0::Ri,
            lvd2cr0::Ri,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd2cr0::Cmpe,
        lvd2cr0::Cmpe,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd2cr0::Cmpe,
            lvd2cr0::Cmpe,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Lvd2Cr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Lvd2Cr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Voltage Monitor 2 Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd2cr0::Rie,
        lvd2cr0::Rie,
        Lvd2Cr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd2cr0::Rie,
            lvd2cr0::Rie,
            Lvd2Cr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd2Cr0 {
    #[inline(always)]
    fn default() -> Lvd2Cr0 {
        <crate::RegValueT<Lvd2Cr0_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod lvd2cr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rn_SPEC;
    pub type Rn = crate::EnumBitfieldStruct<u8, Rn_SPEC>;
    impl Rn {
        #[doc = "Negation follows a stabilization time (tLVD2) after VCC > Vdet2 is detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Negation follows a stabilization time (tLVD2) after assertion of the LVD2 reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri_SPEC;
    pub type Ri = crate::EnumBitfieldStruct<u8, Ri_SPEC>;
    impl Ri {
        #[doc = "Voltage monitor 2 interrupt during Vdet2 passage"]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage monitor 2 reset enabled when the voltage falls to and below Vdet2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Voltage monitor 2 circuit comparison result output disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage monitor 2 circuit comparison result output enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Cr1_SPEC;
impl crate::sealed::RegSpec for Lvd1Cr1_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Control Register 1"]
pub type Lvd1Cr1 = crate::RegValueT<Lvd1Cr1_SPEC>;

impl Lvd1Cr1 {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Lvd1Cr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Lvd1Cr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd1cr1::Irqsel,
        lvd1cr1::Irqsel,
        Lvd1Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd1cr1::Irqsel,
            lvd1cr1::Irqsel,
            Lvd1Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lvd1cr1::Idtsel,
        lvd1cr1::Idtsel,
        Lvd1Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lvd1cr1::Idtsel,
            lvd1cr1::Idtsel,
            Lvd1Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Cr1 {
    #[inline(always)]
    fn default() -> Lvd1Cr1 {
        <crate::RegValueT<Lvd1Cr1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lvd1cr1 {

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
        #[doc = "When VCC>=Vdet1 (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When VCC<Vdet1 (drop) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "When drop and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd1Sr_SPEC;
impl crate::sealed::RegSpec for Lvd1Sr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 1 Circuit Status Register"]
pub type Lvd1Sr = crate::RegValueT<Lvd1Sr_SPEC>;

impl Lvd1Sr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Lvd1Sr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Lvd1Sr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvd1sr::Mon,
        lvd1sr::Mon,
        Lvd1Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvd1sr::Mon,
            lvd1sr::Mon,
            Lvd1Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Voltage Change Detection Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd1sr::Det,
        lvd1sr::Det,
        Lvd1Sr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd1sr::Det,
            lvd1sr::Det,
            Lvd1Sr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd1Sr {
    #[inline(always)]
    fn default() -> Lvd1Sr {
        <crate::RegValueT<Lvd1Sr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod lvd1sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC < Vdet1"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC >= Vdet1 or MON is disabled"]
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
pub struct Lvd2Cr1_SPEC;
impl crate::sealed::RegSpec for Lvd2Cr1_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 2 Circuit Control Register 1"]
pub type Lvd2Cr1 = crate::RegValueT<Lvd2Cr1_SPEC>;

impl Lvd2Cr1 {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Lvd2Cr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Lvd2Cr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Monitor 2 Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lvd2cr1::Irqsel,
        lvd2cr1::Irqsel,
        Lvd2Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lvd2cr1::Irqsel,
            lvd2cr1::Irqsel,
            Lvd2Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lvd2cr1::Idtsel,
        lvd2cr1::Idtsel,
        Lvd2Cr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lvd2cr1::Idtsel,
            lvd2cr1::Idtsel,
            Lvd2Cr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd2Cr1 {
    #[inline(always)]
    fn default() -> Lvd2Cr1 {
        <crate::RegValueT<Lvd2Cr1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lvd2cr1 {

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
        #[doc = "When VCC>=Vdet2 (rise) is detected"]
        pub const _00: Self = Self::new(0);

        #[doc = "When VCC<Vdet2 (drop) is detected"]
        pub const _01: Self = Self::new(1);

        #[doc = "When drop and rise are detected"]
        pub const _10: Self = Self::new(2);

        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvd2Sr_SPEC;
impl crate::sealed::RegSpec for Lvd2Sr_SPEC {
    type DataType = u8;
}

#[doc = "Voltage Monitor 2 Circuit Status Register"]
pub type Lvd2Sr = crate::RegValueT<Lvd2Sr_SPEC>;

impl Lvd2Sr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Lvd2Sr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Lvd2Sr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Monitor 2 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lvd2sr::Mon,
        lvd2sr::Mon,
        Lvd2Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lvd2sr::Mon,
            lvd2sr::Mon,
            Lvd2Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Voltage Change Detection Flag"]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lvd2sr::Det,
        lvd2sr::Det,
        Lvd2Sr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lvd2sr::Det,
            lvd2sr::Det,
            Lvd2Sr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvd2Sr {
    #[inline(always)]
    fn default() -> Lvd2Sr {
        <crate::RegValueT<Lvd2Sr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod lvd2sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC < Vdet2"]
        pub const _0: Self = Self::new(0);

        #[doc = "VCC >= Vdet2 or MON is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Vdet2 passage detection"]
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
    #[doc = "Debugger Enable"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Syocdcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Syocdcr_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Prcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Prcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Rstsr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Rstsr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Monitor 2 Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
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

    #[doc = "Voltage Monitor 1 Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
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

    #[doc = "Voltage Monitor 0 Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
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

    #[doc = "Power-On Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
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
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Rstsr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Rstsr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cold/Warm Start Determination FlagNote: Only 1 can be written to set the flag."]
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
    #[doc = "SP Error Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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

    #[doc = "Bus Master MPU Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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

    #[doc = "Bus Slave MPU Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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

    #[doc = "RAM ECC Error Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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

    #[doc = "RAM Parity Error Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Rstsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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

    #[doc = "Watchdog Timer Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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

    #[doc = "Independent Watchdog Timer Reset Detect FlagNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
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
