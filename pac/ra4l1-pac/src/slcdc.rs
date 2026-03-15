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
// Generated from SVD 0.90.02, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:05:50 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Segment LCD Controller"]
unsafe impl ::core::marker::Send for super::Slcdc {}
unsafe impl ::core::marker::Sync for super::Slcdc {}
impl super::Slcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "LCD Mode Register 0"]
    #[inline(always)]
    pub const fn lcdm0(&self) -> &'static crate::common::Reg<self::Lcdm0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdm0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "LCD Mode Register 1"]
    #[inline(always)]
    pub const fn lcdm1(&self) -> &'static crate::common::Reg<self::Lcdm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "LCD Clock Control Register 0"]
    #[inline(always)]
    pub const fn lcdc0(&self) -> &'static crate::common::Reg<self::Lcdc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "LCD Boost Level Control Register"]
    #[inline(always)]
    pub const fn vlcd(&self) -> &'static crate::common::Reg<self::Vlcd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vlcd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "LCD Display Data Register %s"]
    #[inline(always)]
    pub const fn seg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Seg_SPEC, crate::common::RW>,
        52,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm0_SPEC;
impl crate::sealed::RegSpec for Lcdm0_SPEC {
    type DataType = u8;
}

#[doc = "LCD Mode Register 0"]
pub type Lcdm0 = crate::RegValueT<Lcdm0_SPEC>;

impl Lcdm0 {
    #[doc = "LCD Display Bias Method Select"]
    #[inline(always)]
    pub fn lbas(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lcdm0::Lbas,
        lcdm0::Lbas,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lcdm0::Lbas,
            lcdm0::Lbas,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Time Slice of LCD Display Select"]
    #[inline(always)]
    pub fn ldty(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        lcdm0::Ldty,
        lcdm0::Ldty,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            lcdm0::Ldty,
            lcdm0::Ldty,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD Display Waveform Select"]
    #[inline(always)]
    pub fn lwave(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lcdm0::Lwave,
        lcdm0::Lwave,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lcdm0::Lwave,
            lcdm0::Lwave,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD Drive Voltage Generator Select"]
    #[inline(always)]
    pub fn mdset(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        lcdm0::Mdset,
        lcdm0::Mdset,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            lcdm0::Mdset,
            lcdm0::Mdset,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdm0 {
    #[inline(always)]
    fn default() -> Lcdm0 {
        <crate::RegValueT<Lcdm0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdm0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lbas_SPEC;
    pub type Lbas = crate::EnumBitfieldStruct<u8, Lbas_SPEC>;
    impl Lbas {
        #[doc = "1/2 bias method"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/3 bias method"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/4 bias method"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldty_SPEC;
    pub type Ldty = crate::EnumBitfieldStruct<u8, Ldty_SPEC>;
    impl Ldty {
        #[doc = "Static"]
        pub const _000: Self = Self::new(0);

        #[doc = "2-time slice"]
        pub const _001: Self = Self::new(1);

        #[doc = "3-time slice"]
        pub const _010: Self = Self::new(2);

        #[doc = "4-time slice"]
        pub const _011: Self = Self::new(3);

        #[doc = "6-time slice"]
        pub const _100: Self = Self::new(4);

        #[doc = "8-time slice"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lwave_SPEC;
    pub type Lwave = crate::EnumBitfieldStruct<u8, Lwave_SPEC>;
    impl Lwave {
        #[doc = "Waveform A"]
        pub const _0: Self = Self::new(0);

        #[doc = "Waveform B"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mdset_SPEC;
    pub type Mdset = crate::EnumBitfieldStruct<u8, Mdset_SPEC>;
    impl Mdset {
        #[doc = "External resistance division method"]
        pub const _00: Self = Self::new(0);

        #[doc = "Internal voltage boosting method"]
        pub const _01: Self = Self::new(1);

        #[doc = "Capacitor split method"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm1_SPEC;
impl crate::sealed::RegSpec for Lcdm1_SPEC {
    type DataType = u8;
}

#[doc = "LCD Mode Register 1"]
pub type Lcdm1 = crate::RegValueT<Lcdm1_SPEC>;

impl Lcdm1 {
    #[doc = "Voltage Boosting Pin Initial Value Switching Control"]
    #[inline(always)]
    pub fn lcdvlm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lcdm1::Lcdvlm,
        lcdm1::Lcdvlm,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lcdm1::Lcdvlm,
            lcdm1::Lcdvlm,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Display Area Data Selection"]
    #[inline(always)]
    pub fn lcdsel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        lcdm1::Lcdsel,
        lcdm1::Lcdsel,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            lcdm1::Lcdsel,
            lcdm1::Lcdsel,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alternately Display A-pattern and B-pattern Area Data (blinking display) Control"]
    #[inline(always)]
    pub fn blon(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        lcdm1::Blon,
        lcdm1::Blon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            lcdm1::Blon,
            lcdm1::Blon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Boost Circuit or Capacitor Split Circuit Operation Enable/Disable"]
    #[inline(always)]
    pub fn vlcon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lcdm1::Vlcon,
        lcdm1::Vlcon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lcdm1::Vlcon,
            lcdm1::Vlcon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn scoc(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lcdm1::Scoc,
        lcdm1::Scoc,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lcdm1::Scoc,
            lcdm1::Scoc,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn lcdon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lcdm1::Lcdon,
        lcdm1::Lcdon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lcdm1::Lcdon,
            lcdm1::Lcdon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdm1 {
    #[inline(always)]
    fn default() -> Lcdm1 {
        <crate::RegValueT<Lcdm1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdm1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdvlm_SPEC;
    pub type Lcdvlm = crate::EnumBitfieldStruct<u8, Lcdvlm_SPEC>;
    impl Lcdvlm {
        #[doc = "Set when VCC ≥ 2.7 V"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set when VCC ≤ 3.6 V"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdsel_SPEC;
    pub type Lcdsel = crate::EnumBitfieldStruct<u8, Lcdsel_SPEC>;
    impl Lcdsel {
        #[doc = "Display an A-pattern area data (lower 4 bits of LCD display data register)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Display a B-pattern area data (higher 4 bits of LCD display data register)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blon_SPEC;
    pub type Blon = crate::EnumBitfieldStruct<u8, Blon_SPEC>;
    impl Blon {
        #[doc = "Alternately display A-pattern and B-pattern area data (blinking display) disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Alternately display A-pattern and B-pattern area data (blinking display corresponding to the periodic interrupt (RTC_PRD) timing of the Realtime Clock (RTC))."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlcon_SPEC;
    pub type Vlcon = crate::EnumBitfieldStruct<u8, Vlcon_SPEC>;
    impl Vlcon {
        #[doc = "Disable voltage boost circuit or capacitor split circuit operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable voltage boost circuit or capacitor split circuit operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scoc_SPEC;
    pub type Scoc = crate::EnumBitfieldStruct<u8, Scoc_SPEC>;
    impl Scoc {
        #[doc = "Output ground level to segment/common pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Display data to segment/common pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdon_SPEC;
    pub type Lcdon = crate::EnumBitfieldStruct<u8, Lcdon_SPEC>;
    impl Lcdon {
        #[doc = "Display off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Display on"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdc0_SPEC;
impl crate::sealed::RegSpec for Lcdc0_SPEC {
    type DataType = u8;
}

#[doc = "LCD Clock Control Register 0"]
pub type Lcdc0 = crate::RegValueT<Lcdc0_SPEC>;

impl Lcdc0 {
    #[doc = "LCD Clock (LCDCL) Setting"]
    #[inline(always)]
    pub fn lcdc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        lcdc0::Lcdc0,
        lcdc0::Lcdc0,
        Lcdc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            lcdc0::Lcdc0,
            lcdc0::Lcdc0,
            Lcdc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdc0 {
    #[inline(always)]
    fn default() -> Lcdc0 {
        <crate::RegValueT<Lcdc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdc0_SPEC;
    pub type Lcdc0 = crate::EnumBitfieldStruct<u8, Lcdc0_SPEC>;
    impl Lcdc0 {
        #[doc = "(SOSC)/22 or (LOCO)/22"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "(SOSC)/23 or (LOCO)/23"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "(SOSC)/24 or (LOCO)/24"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "(SOSC)/25 or (LOCO)/25"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "(SOSC)/26 or (LOCO)/26"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "(SOSC)/27 or (LOCO)/27"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "(SOSC)/28 or (LOCO)/28"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "(SOSC)/29 or (LOCO)/29"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "(SOSC)/210 or (LOCO)/210"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "(MOSC)/28 or (HOCO)/28"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "(MOSC)/29 or (HOCO)/29"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "(MOSC)/210 or (HOCO)/210"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "(MOSC)/211 or (HOCO)/211"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "(MOSC)/212 or (HOCO)/212"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "(MOSC)/213 or (HOCO)/213"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "(MOSC)/214 or (HOCO)/214"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "(MOSC)/215 or (HOCO)/215"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "(MOSC)/216 or (HOCO)/216"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "(MOSC)/217 or (HOCO)/217"]
        pub const _0_X_1_A: Self = Self::new(26);

        #[doc = "(MOSC)/218 or (HOCO)/218"]
        pub const _0_X_1_B: Self = Self::new(27);

        #[doc = "(MOSC)/219 or (HOCO)/219"]
        pub const _0_X_2_B: Self = Self::new(43);

        #[doc = "(MOSC)/220 or (HOCO)/220"]
        pub const _0_X_3_B: Self = Self::new(59);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vlcd_SPEC;
impl crate::sealed::RegSpec for Vlcd_SPEC {
    type DataType = u8;
}

#[doc = "LCD Boost Level Control Register"]
pub type Vlcd = crate::RegValueT<Vlcd_SPEC>;

impl Vlcd {
    #[doc = "Reference Voltage Select (Contrast Adjustment)"]
    #[inline(always)]
    pub fn vlcd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        vlcd::Vlcd,
        vlcd::Vlcd,
        Vlcd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            vlcd::Vlcd,
            vlcd::Vlcd,
            Vlcd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reference Voltage Selection"]
    #[inline(always)]
    pub fn mdset2(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        vlcd::Mdset2,
        vlcd::Mdset2,
        Vlcd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            vlcd::Mdset2,
            vlcd::Mdset2,
            Vlcd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vlcd {
    #[inline(always)]
    fn default() -> Vlcd {
        <crate::RegValueT<Vlcd_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod vlcd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlcd_SPEC;
    pub type Vlcd = crate::EnumBitfieldStruct<u8, Vlcd_SPEC>;
    impl Vlcd {
        #[doc = "1.01 V (VL1 reference voltage) 2.02 V (VL2 reference voltage)"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "1.04 V (VL1 reference voltage) 2.09 V (VL2 reference voltage)"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "1.07 V (VL1 reference voltage) 2.16 V (VL2 reference voltage)"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "1.11 V (VL1 reference voltage) 2.22 V (VL2 reference voltage)"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "1.14 V (VL1 reference voltage) 2.29 V (VL2 reference voltage)"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "1.17 V (VL1 reference voltage) 2.36 V (VL2 reference voltage)"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "1.21 V (VL1 reference voltage) 2.42 V (VL2 reference voltage)"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "1.24 V (VL1 reference voltage) 2.49 V (VL2 reference voltage)"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "1.27 V (VL1 reference voltage) 2.56 V (VL2 reference voltage)"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "1.31 V (VL1 reference voltage) 2.62 V (VL2 reference voltage)"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "1.34 V (VL1 reference voltage) 2.69 V (VL2 reference voltage)"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "1.37 V (VL1 reference voltage) 2.76 V (VL2 reference voltage)"]
        pub const _0_X_F: Self = Self::new(15);

        #[doc = "1.40 V (VL1 reference voltage) 2.82 V (VL2 reference voltage)"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "1.44 V (VL1 reference voltage) 2.89 V (VL2 reference voltage)"]
        pub const _0_X_11: Self = Self::new(17);

        #[doc = "1.47 V (VL1 reference voltage) 2.96 V (VL2 reference voltage)"]
        pub const _0_X_12: Self = Self::new(18);

        #[doc = "1.50 V (VL1 reference voltage) 3.02 V (VL2 reference voltage)"]
        pub const _0_X_13: Self = Self::new(19);

        #[doc = "1.54 V (VL1 reference voltage) 3.09 V (VL2 reference voltage)"]
        pub const _0_X_14: Self = Self::new(20);

        #[doc = "1.57 V (VL1 reference voltage) 3.15 V (VL2 reference voltage)"]
        pub const _0_X_15: Self = Self::new(21);

        #[doc = "1.60 V (VL1 reference voltage) 3.22 V (VL2 reference voltage)"]
        pub const _0_X_16: Self = Self::new(22);

        #[doc = "1.64 V (VL1 reference voltage) 3.29 V (VL2 reference voltage)"]
        pub const _0_X_17: Self = Self::new(23);

        #[doc = "1.67 V (VL1 reference voltage) 3.35 V (VL2 reference voltage)"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "1.70 V (VL1 reference voltage) 3.42 V (VL2 reference voltage)"]
        pub const _0_X_19: Self = Self::new(25);

        #[doc = "1.74 V (VL1 reference voltage) 3.49 V (VL2 reference voltage)"]
        pub const _0_X_1_A: Self = Self::new(26);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mdset2_SPEC;
    pub type Mdset2 = crate::EnumBitfieldStruct<u8, Mdset2_SPEC>;
    impl Mdset2 {
        #[doc = "External resistance division method or VL1 reference mode of internal voltage boosting method or VCC reference mode of capacitor split method"]
        pub const _0: Self = Self::new(0);

        #[doc = "VL2 reference mode of internal voltage boosting method or VL4 reference mode of capacitor split method"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seg_SPEC;
impl crate::sealed::RegSpec for Seg_SPEC {
    type DataType = u8;
}

#[doc = "LCD Display Data Register %s"]
pub type Seg = crate::RegValueT<Seg_SPEC>;

impl NoBitfieldReg<Seg_SPEC> for Seg {}
impl ::core::default::Default for Seg {
    #[inline(always)]
    fn default() -> Seg {
        <crate::RegValueT<Seg_SPEC> as RegisterValue<_>>::new(0)
    }
}
