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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:54:26 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Clock Frequency Accuracy Measurement Circuit"]
unsafe impl ::core::marker::Send for super::Cac {}
unsafe impl ::core::marker::Sync for super::Cac {}
impl super::Cac {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CAC Control Register 0"]
    #[inline(always)]
    pub const fn cacr0(&self) -> &'static crate::common::Reg<self::Cacr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cacr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "CAC Control Register 1"]
    #[inline(always)]
    pub const fn cacr1(&self) -> &'static crate::common::Reg<self::Cacr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cacr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "CAC Control Register 2"]
    #[inline(always)]
    pub const fn cacr2(&self) -> &'static crate::common::Reg<self::Cacr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cacr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "CAC Interrupt Control Register"]
    #[inline(always)]
    pub const fn caicr(&self) -> &'static crate::common::Reg<self::Caicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "CAC Status Register"]
    #[inline(always)]
    pub const fn castr(&self) -> &'static crate::common::Reg<self::Castr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Castr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "CAC Upper-Limit Value Setting Register"]
    #[inline(always)]
    pub const fn caulvr(
        &self,
    ) -> &'static crate::common::Reg<self::Caulvr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caulvr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "CAC Lower-Limit Value Setting Register"]
    #[inline(always)]
    pub const fn callvr(
        &self,
    ) -> &'static crate::common::Reg<self::Callvr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Callvr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "CAC Counter Buffer Register"]
    #[inline(always)]
    pub const fn cacntbr(
        &self,
    ) -> &'static crate::common::Reg<self::Cacntbr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cacntbr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacr0_SPEC;
impl crate::sealed::RegSpec for Cacr0_SPEC {
    type DataType = u8;
}

#[doc = "CAC Control Register 0"]
pub type Cacr0 = crate::RegValueT<Cacr0_SPEC>;

impl Cacr0 {
    #[doc = "Clock Frequency Measurement Enable"]
    #[inline(always)]
    pub fn cfme(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cacr0::Cfme,
        cacr0::Cfme,
        Cacr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cacr0::Cfme,
            cacr0::Cfme,
            Cacr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cacr0 {
    #[inline(always)]
    fn default() -> Cacr0 {
        <crate::RegValueT<Cacr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cacr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfme_SPEC;
    pub type Cfme = crate::EnumBitfieldStruct<u8, Cfme_SPEC>;
    impl Cfme {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacr1_SPEC;
impl crate::sealed::RegSpec for Cacr1_SPEC {
    type DataType = u8;
}

#[doc = "CAC Control Register 1"]
pub type Cacr1 = crate::RegValueT<Cacr1_SPEC>;

impl Cacr1 {
    #[doc = "CACREF Pin Input Enable"]
    #[inline(always)]
    pub fn cacrefe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cacr1::Cacrefe,
        cacr1::Cacrefe,
        Cacr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cacr1::Cacrefe,
            cacr1::Cacrefe,
            Cacr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Measurement Target Clock Select"]
    #[inline(always)]
    pub fn fmcs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        cacr1::Fmcs,
        cacr1::Fmcs,
        Cacr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            cacr1::Fmcs,
            cacr1::Fmcs,
            Cacr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timer Count Clock Source Select"]
    #[inline(always)]
    pub fn tcss(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        cacr1::Tcss,
        cacr1::Tcss,
        Cacr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            cacr1::Tcss,
            cacr1::Tcss,
            Cacr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Valid Edge Select"]
    #[inline(always)]
    pub fn edges(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        cacr1::Edges,
        cacr1::Edges,
        Cacr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            cacr1::Edges,
            cacr1::Edges,
            Cacr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cacr1 {
    #[inline(always)]
    fn default() -> Cacr1 {
        <crate::RegValueT<Cacr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cacr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cacrefe_SPEC;
    pub type Cacrefe = crate::EnumBitfieldStruct<u8, Cacrefe_SPEC>;
    impl Cacrefe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmcs_SPEC;
    pub type Fmcs = crate::EnumBitfieldStruct<u8, Fmcs_SPEC>;
    impl Fmcs {
        #[doc = "Main clock oscillator (CACMCLK)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Sub-clock oscillator (CACSCLK)"]
        pub const _001: Self = Self::new(1);

        #[doc = "HOCO clock (CACHCLK)"]
        pub const _010: Self = Self::new(2);

        #[doc = "MOCO clock (CACMOCLK)"]
        pub const _011: Self = Self::new(3);

        #[doc = "LOCO clock (CACLCLK)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Peripheral module clock B (PCLKB)"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcss_SPEC;
    pub type Tcss = crate::EnumBitfieldStruct<u8, Tcss_SPEC>;
    impl Tcss {
        #[doc = "No division"]
        pub const _00: Self = Self::new(0);

        #[doc = "x 1/4 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "x 1/8 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "x 1/32 clock"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edges_SPEC;
    pub type Edges = crate::EnumBitfieldStruct<u8, Edges_SPEC>;
    impl Edges {
        #[doc = "Rising edge"]
        pub const _00: Self = Self::new(0);

        #[doc = "Falling edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Both rising and falling edges"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacr2_SPEC;
impl crate::sealed::RegSpec for Cacr2_SPEC {
    type DataType = u8;
}

#[doc = "CAC Control Register 2"]
pub type Cacr2 = crate::RegValueT<Cacr2_SPEC>;

impl Cacr2 {
    #[doc = "Reference Signal Select"]
    #[inline(always)]
    pub fn rps(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cacr2::Rps,
        cacr2::Rps,
        Cacr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cacr2::Rps,
            cacr2::Rps,
            Cacr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Measurement Reference Clock Select"]
    #[inline(always)]
    pub fn rscs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        cacr2::Rscs,
        cacr2::Rscs,
        Cacr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            cacr2::Rscs,
            cacr2::Rscs,
            Cacr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn rcds(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        cacr2::Rcds,
        cacr2::Rcds,
        Cacr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            cacr2::Rcds,
            cacr2::Rcds,
            Cacr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Digital Filter Select"]
    #[inline(always)]
    pub fn dfs(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        cacr2::Dfs,
        cacr2::Dfs,
        Cacr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            cacr2::Dfs,
            cacr2::Dfs,
            Cacr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cacr2 {
    #[inline(always)]
    fn default() -> Cacr2 {
        <crate::RegValueT<Cacr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cacr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rps_SPEC;
    pub type Rps = crate::EnumBitfieldStruct<u8, Rps_SPEC>;
    impl Rps {
        #[doc = "CACREF pin input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal clock (internally generated signal)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rscs_SPEC;
    pub type Rscs = crate::EnumBitfieldStruct<u8, Rscs_SPEC>;
    impl Rscs {
        #[doc = "Main clock oscillator (CACMCLK)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Sub-clock oscillator (CACSCLK)"]
        pub const _001: Self = Self::new(1);

        #[doc = "HOCO clock (CACHCLK)"]
        pub const _010: Self = Self::new(2);

        #[doc = "MOCO clock (CACMOCLK)"]
        pub const _011: Self = Self::new(3);

        #[doc = "LOCO clock (CACLCLK)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Peripheral module clock B (PCLKB)"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcds_SPEC;
    pub type Rcds = crate::EnumBitfieldStruct<u8, Rcds_SPEC>;
    impl Rcds {
        #[doc = "x 1/32 clock"]
        pub const _00: Self = Self::new(0);

        #[doc = "x 1/128 clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "x 1/1024 clock"]
        pub const _10: Self = Self::new(2);

        #[doc = "x 1/8192 clock"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfs_SPEC;
    pub type Dfs = crate::EnumBitfieldStruct<u8, Dfs_SPEC>;
    impl Dfs {
        #[doc = "Disable digital filtering"]
        pub const _00: Self = Self::new(0);

        #[doc = "Use sampling clock for the digital filter as the frequency measuring clock"]
        pub const _01: Self = Self::new(1);

        #[doc = "Use sampling clock for the digital filter as the frequency measuring clock divided by 4"]
        pub const _10: Self = Self::new(2);

        #[doc = "Use sampling clock for the digital filter as the frequency measuring clock divided by 16."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caicr_SPEC;
impl crate::sealed::RegSpec for Caicr_SPEC {
    type DataType = u8;
}

#[doc = "CAC Interrupt Control Register"]
pub type Caicr = crate::RegValueT<Caicr_SPEC>;

impl Caicr {
    #[doc = "Frequency Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn ferrie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        caicr::Ferrie,
        caicr::Ferrie,
        Caicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            caicr::Ferrie,
            caicr::Ferrie,
            Caicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Measurement End Interrupt Request Enable"]
    #[inline(always)]
    pub fn mendie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        caicr::Mendie,
        caicr::Mendie,
        Caicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            caicr::Mendie,
            caicr::Mendie,
            Caicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn ovfie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        caicr::Ovfie,
        caicr::Ovfie,
        Caicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            caicr::Ovfie,
            caicr::Ovfie,
            Caicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FERRF Clear"]
    #[inline(always)]
    pub fn ferrfcl(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        caicr::Ferrfcl,
        caicr::Ferrfcl,
        Caicr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            caicr::Ferrfcl,
            caicr::Ferrfcl,
            Caicr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "MENDF Clear"]
    #[inline(always)]
    pub fn mendfcl(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        caicr::Mendfcl,
        caicr::Mendfcl,
        Caicr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            caicr::Mendfcl,
            caicr::Mendfcl,
            Caicr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "OVFF Clear"]
    #[inline(always)]
    pub fn ovffcl(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        caicr::Ovffcl,
        caicr::Ovffcl,
        Caicr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            caicr::Ovffcl,
            caicr::Ovffcl,
            Caicr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Caicr {
    #[inline(always)]
    fn default() -> Caicr {
        <crate::RegValueT<Caicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod caicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferrie_SPEC;
    pub type Ferrie = crate::EnumBitfieldStruct<u8, Ferrie_SPEC>;
    impl Ferrie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mendie_SPEC;
    pub type Mendie = crate::EnumBitfieldStruct<u8, Mendie_SPEC>;
    impl Mendie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfie_SPEC;
    pub type Ovfie = crate::EnumBitfieldStruct<u8, Ovfie_SPEC>;
    impl Ovfie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferrfcl_SPEC;
    pub type Ferrfcl = crate::EnumBitfieldStruct<u8, Ferrfcl_SPEC>;
    impl Ferrfcl {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "The CASTR.FERRF flag is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mendfcl_SPEC;
    pub type Mendfcl = crate::EnumBitfieldStruct<u8, Mendfcl_SPEC>;
    impl Mendfcl {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "The CASTR.MENDF flag is cleared"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovffcl_SPEC;
    pub type Ovffcl = crate::EnumBitfieldStruct<u8, Ovffcl_SPEC>;
    impl Ovffcl {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "The CASTR.OVFF flag is cleared."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Castr_SPEC;
impl crate::sealed::RegSpec for Castr_SPEC {
    type DataType = u8;
}

#[doc = "CAC Status Register"]
pub type Castr = crate::RegValueT<Castr_SPEC>;

impl Castr {
    #[doc = "Frequency Error Flag"]
    #[inline(always)]
    pub fn ferrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        castr::Ferrf,
        castr::Ferrf,
        Castr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            castr::Ferrf,
            castr::Ferrf,
            Castr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Measurement End Flag"]
    #[inline(always)]
    pub fn mendf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        castr::Mendf,
        castr::Mendf,
        Castr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            castr::Mendf,
            castr::Mendf,
            Castr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Overflow Flag"]
    #[inline(always)]
    pub fn ovff(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        castr::Ovff,
        castr::Ovff,
        Castr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            castr::Ovff,
            castr::Ovff,
            Castr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Castr {
    #[inline(always)]
    fn default() -> Castr {
        <crate::RegValueT<Castr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod castr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferrf_SPEC;
    pub type Ferrf = crate::EnumBitfieldStruct<u8, Ferrf_SPEC>;
    impl Ferrf {
        #[doc = "Clock frequency is within the allowable range"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock frequency has deviated beyond the allowable range (frequency error)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mendf_SPEC;
    pub type Mendf = crate::EnumBitfieldStruct<u8, Mendf_SPEC>;
    impl Mendf {
        #[doc = "Measurement is in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Measurement ended"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovff_SPEC;
    pub type Ovff = crate::EnumBitfieldStruct<u8, Ovff_SPEC>;
    impl Ovff {
        #[doc = "Counter has not overflowed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Counter overflowed"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caulvr_SPEC;
impl crate::sealed::RegSpec for Caulvr_SPEC {
    type DataType = u16;
}

#[doc = "CAC Upper-Limit Value Setting Register"]
pub type Caulvr = crate::RegValueT<Caulvr_SPEC>;

impl NoBitfieldReg<Caulvr_SPEC> for Caulvr {}
impl ::core::default::Default for Caulvr {
    #[inline(always)]
    fn default() -> Caulvr {
        <crate::RegValueT<Caulvr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Callvr_SPEC;
impl crate::sealed::RegSpec for Callvr_SPEC {
    type DataType = u16;
}

#[doc = "CAC Lower-Limit Value Setting Register"]
pub type Callvr = crate::RegValueT<Callvr_SPEC>;

impl NoBitfieldReg<Callvr_SPEC> for Callvr {}
impl ::core::default::Default for Callvr {
    #[inline(always)]
    fn default() -> Callvr {
        <crate::RegValueT<Callvr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacntbr_SPEC;
impl crate::sealed::RegSpec for Cacntbr_SPEC {
    type DataType = u16;
}

#[doc = "CAC Counter Buffer Register"]
pub type Cacntbr = crate::RegValueT<Cacntbr_SPEC>;

impl NoBitfieldReg<Cacntbr_SPEC> for Cacntbr {}
impl ::core::default::Default for Cacntbr {
    #[inline(always)]
    fn default() -> Cacntbr {
        <crate::RegValueT<Cacntbr_SPEC> as RegisterValue<_>>::new(0)
    }
}
