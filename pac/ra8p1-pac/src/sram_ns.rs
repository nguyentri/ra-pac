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
// Generated from SVD 1.00.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:39 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SRAM Control"]
unsafe impl ::core::marker::Send for super::SramNs {}
unsafe impl ::core::marker::Sync for super::SramNs {}
impl super::SramNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "SRAM Protection Control Register for Non-Secure"]
    #[inline(always)]
    pub const fn sramprcr_ns(
        &self,
    ) -> &'static crate::common::Reg<self::SramprcrNs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SramprcrNs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "SRAM Wait State Control Register"]
    #[inline(always)]
    pub const fn sramwtsc(
        &self,
    ) -> &'static crate::common::Reg<self::Sramwtsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramwtsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "SRAM Control Register %s For ECC RAM"]
    #[inline(always)]
    pub const fn sramcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sramcr_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10usize))
        }
    }
    #[inline(always)]
    pub const fn sramcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Sramcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Sramcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Sramcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramcr3(
        &self,
    ) -> &'static crate::common::Reg<self::Sramcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }

    #[doc = "SRAM ECC Region Control Register 0"]
    #[inline(always)]
    pub const fn srameccrgn0(
        &self,
    ) -> &'static crate::common::Reg<self::Srameccrgn0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srameccrgn0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "SRAM ECC Region Control Register 1"]
    #[inline(always)]
    pub const fn srameccrgn1(
        &self,
    ) -> &'static crate::common::Reg<self::Srameccrgn1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srameccrgn1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "SRAM ECC Region Control Register 2"]
    #[inline(always)]
    pub const fn srameccrgn2(
        &self,
    ) -> &'static crate::common::Reg<self::Srameccrgn2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srameccrgn2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "SRAM ECC Region Control Register 3"]
    #[inline(always)]
    pub const fn srameccrgn3(
        &self,
    ) -> &'static crate::common::Reg<self::Srameccrgn3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srameccrgn3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "SRAM Error Status Register For ECC RAM"]
    #[inline(always)]
    pub const fn sramesr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramesr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramesr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "SRAM Error Status Clear Register For ECC RAM"]
    #[inline(always)]
    pub const fn sramesclr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramesclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Sramesclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "SRAM Error Address Register n0 For ECC RAM"]
    #[inline(always)]
    pub const fn sramear0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sramear0_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x50usize))
        }
    }
    #[inline(always)]
    pub const fn sramear00(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramear10(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramear20(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramear30(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }

    #[doc = "SRAM Error Address Register n1 For ECC RAM"]
    #[inline(always)]
    pub const fn sramear1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sramear1_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x54usize))
        }
    }
    #[inline(always)]
    pub const fn sramear01(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramear11(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramear21(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sramear31(
        &self,
    ) -> &'static crate::common::Reg<self::Sramear1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramear1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramprcrNs_SPEC;
impl crate::sealed::RegSpec for SramprcrNs_SPEC {
    type DataType = u16;
}

#[doc = "SRAM Protection Control Register for Non-Secure"]
pub type SramprcrNs = crate::RegValueT<SramprcrNs_SPEC>;

impl SramprcrNs {
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub fn pr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr_ns::Pr,
        sramprcr_ns::Pr,
        SramprcrNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr_ns::Pr,
            sramprcr_ns::Pr,
            SramprcrNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, SramprcrNs_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,SramprcrNs_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for SramprcrNs {
    #[inline(always)]
    fn default() -> SramprcrNs {
        <crate::RegValueT<SramprcrNs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr_ns {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr_SPEC;
    pub type Pr = crate::EnumBitfieldStruct<u8, Pr_SPEC>;
    impl Pr {
        #[doc = "Writing to registers are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to registers are enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramwtsc_SPEC;
impl crate::sealed::RegSpec for Sramwtsc_SPEC {
    type DataType = u8;
}

#[doc = "SRAM Wait State Control Register"]
pub type Sramwtsc = crate::RegValueT<Sramwtsc_SPEC>;

impl Sramwtsc {
    #[doc = "SRAM wait enable"]
    #[inline(always)]
    pub fn wten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramwtsc::Wten,
        sramwtsc::Wten,
        Sramwtsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramwtsc::Wten,
            sramwtsc::Wten,
            Sramwtsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramwtsc {
    #[inline(always)]
    fn default() -> Sramwtsc {
        <crate::RegValueT<Sramwtsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramwtsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wten_SPEC;
    pub type Wten = crate::EnumBitfieldStruct<u8, Wten_SPEC>;
    impl Wten {
        #[doc = "Do not add wait state in the access cycle to SRAMs."]
        pub const _0: Self = Self::new(0);

        #[doc = "Add wait state in the access cycle to SRAMs."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr_SPEC;
impl crate::sealed::RegSpec for Sramcr_SPEC {
    type DataType = u8;
}

#[doc = "SRAM Control Register %s For ECC RAM"]
pub type Sramcr = crate::RegValueT<Sramcr_SPEC>;

impl Sramcr {
    #[doc = "Operation after error detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramcr::Oad,
        sramcr::Oad,
        Sramcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramcr::Oad,
            sramcr::Oad,
            Sramcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        sramcr::Eccmod,
        sramcr::Eccmod,
        Sramcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            sramcr::Eccmod,
            sramcr::Eccmod,
            Sramcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC 1-bit Error Update Enable"]
    #[inline(always)]
    pub fn e1stsen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sramcr::E1Stsen,
        sramcr::E1Stsen,
        Sramcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sramcr::E1Stsen,
            sramcr::E1Stsen,
            Sramcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Test Enable / ECC Bypass Select"]
    #[inline(always)]
    pub fn tstbyp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sramcr::Tstbyp,
        sramcr::Tstbyp,
        Sramcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sramcr::Tstbyp,
            sramcr::Tstbyp,
            Sramcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramcr {
    #[inline(always)]
    fn default() -> Sramcr {
        <crate::RegValueT<Sramcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccmod_SPEC;
    pub type Eccmod = crate::EnumBitfieldStruct<u8, Eccmod_SPEC>;
    impl Eccmod {
        #[doc = "Disable ECC function"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Enable ECC function without error checking"]
        pub const _10: Self = Self::new(2);

        #[doc = "Enable ECC function with error checking"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct E1Stsen_SPEC;
    pub type E1Stsen = crate::EnumBitfieldStruct<u8, E1Stsen_SPEC>;
    impl E1Stsen {
        #[doc = "Disable updating of 1-bit ECC error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable updating of 1-bit ECC error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstbyp_SPEC;
    pub type Tstbyp = crate::EnumBitfieldStruct<u8, Tstbyp_SPEC>;
    impl Tstbyp {
        #[doc = "Disable ECC bypass"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ECC bypass"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srameccrgn0_SPEC;
impl crate::sealed::RegSpec for Srameccrgn0_SPEC {
    type DataType = u8;
}

#[doc = "SRAM ECC Region Control Register 0"]
pub type Srameccrgn0 = crate::RegValueT<Srameccrgn0_SPEC>;

impl Srameccrgn0 {
    #[doc = "ECC target region select"]
    #[inline(always)]
    pub fn eccrgn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        srameccrgn0::Eccrgn,
        srameccrgn0::Eccrgn,
        Srameccrgn0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            srameccrgn0::Eccrgn,
            srameccrgn0::Eccrgn,
            Srameccrgn0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srameccrgn0 {
    #[inline(always)]
    fn default() -> Srameccrgn0 {
        <crate::RegValueT<Srameccrgn0_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod srameccrgn0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccrgn_SPEC;
    pub type Eccrgn = crate::EnumBitfieldStruct<u8, Eccrgn_SPEC>;
    impl Eccrgn {
        #[doc = "No ECC target region"]
        pub const _000: Self = Self::new(0);

        #[doc = "0x2200_0000–0x2201_FFFF 0x3200_0000–0x3201_FFFF (128 KB)"]
        pub const _001: Self = Self::new(1);

        #[doc = "0x2200_0000–0x2203_FFFF 0x3200_0000–0x3203_FFFF (256 KB)"]
        pub const _010: Self = Self::new(2);

        #[doc = "0x2200_0000–0x2205_FFFF 0x3200_0000–0x3205_FFFF (384 KB)"]
        pub const _011: Self = Self::new(3);

        #[doc = "0x2200_0000–0x2207_FFFF 0x3200_0000–0x3207_FFFF (512 KB)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srameccrgn1_SPEC;
impl crate::sealed::RegSpec for Srameccrgn1_SPEC {
    type DataType = u8;
}

#[doc = "SRAM ECC Region Control Register 1"]
pub type Srameccrgn1 = crate::RegValueT<Srameccrgn1_SPEC>;

impl Srameccrgn1 {
    #[doc = "ECC target region select"]
    #[inline(always)]
    pub fn eccrgn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        srameccrgn1::Eccrgn,
        srameccrgn1::Eccrgn,
        Srameccrgn1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            srameccrgn1::Eccrgn,
            srameccrgn1::Eccrgn,
            Srameccrgn1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srameccrgn1 {
    #[inline(always)]
    fn default() -> Srameccrgn1 {
        <crate::RegValueT<Srameccrgn1_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod srameccrgn1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccrgn_SPEC;
    pub type Eccrgn = crate::EnumBitfieldStruct<u8, Eccrgn_SPEC>;
    impl Eccrgn {
        #[doc = "No ECC target region"]
        pub const _000: Self = Self::new(0);

        #[doc = "0x2208_0000–0x2209_FFFF 0x3208_0000–0x3209_FFFF (128 KB)"]
        pub const _001: Self = Self::new(1);

        #[doc = "0x2208_0000–0x220B_FFFF 0x3208_0000–0x320B_FFFF (256 KB)"]
        pub const _010: Self = Self::new(2);

        #[doc = "0x2208_0000–0x220D_FFFF 0x3208_0000–0x320D_FFFF (384 KB)"]
        pub const _011: Self = Self::new(3);

        #[doc = "0x2208_0000–0x220F_FFFF 0x3208_0000–0x320F_FFFF (512 KB)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srameccrgn2_SPEC;
impl crate::sealed::RegSpec for Srameccrgn2_SPEC {
    type DataType = u8;
}

#[doc = "SRAM ECC Region Control Register 2"]
pub type Srameccrgn2 = crate::RegValueT<Srameccrgn2_SPEC>;

impl Srameccrgn2 {
    #[doc = "ECC target region select"]
    #[inline(always)]
    pub fn eccrgn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        srameccrgn2::Eccrgn,
        srameccrgn2::Eccrgn,
        Srameccrgn2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            srameccrgn2::Eccrgn,
            srameccrgn2::Eccrgn,
            Srameccrgn2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srameccrgn2 {
    #[inline(always)]
    fn default() -> Srameccrgn2 {
        <crate::RegValueT<Srameccrgn2_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod srameccrgn2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccrgn_SPEC;
    pub type Eccrgn = crate::EnumBitfieldStruct<u8, Eccrgn_SPEC>;
    impl Eccrgn {
        #[doc = "No ECC target region"]
        pub const _000: Self = Self::new(0);

        #[doc = "0x2210_0000–0x2211_FFFF 0x3210_0000–0x3211_FFFF (128 KB)"]
        pub const _001: Self = Self::new(1);

        #[doc = "0x2210_0000–0x2213_FFFF 0x3210_0000–0x3213_FFFF (256 KB)"]
        pub const _010: Self = Self::new(2);

        #[doc = "0x2210_0000–0x2215_FFFF 0x3210_0000–0x3215_FFFF (384 KB)"]
        pub const _011: Self = Self::new(3);

        #[doc = "0x2210_0000–0x2217_FFFF 0x3210_0000–0x3217_FFFF (512 KB)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srameccrgn3_SPEC;
impl crate::sealed::RegSpec for Srameccrgn3_SPEC {
    type DataType = u8;
}

#[doc = "SRAM ECC Region Control Register 3"]
pub type Srameccrgn3 = crate::RegValueT<Srameccrgn3_SPEC>;

impl Srameccrgn3 {
    #[doc = "ECC target region select"]
    #[inline(always)]
    pub fn eccrgn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        srameccrgn3::Eccrgn,
        srameccrgn3::Eccrgn,
        Srameccrgn3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            srameccrgn3::Eccrgn,
            srameccrgn3::Eccrgn,
            Srameccrgn3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srameccrgn3 {
    #[inline(always)]
    fn default() -> Srameccrgn3 {
        <crate::RegValueT<Srameccrgn3_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod srameccrgn3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccrgn_SPEC;
    pub type Eccrgn = crate::EnumBitfieldStruct<u8, Eccrgn_SPEC>;
    impl Eccrgn {
        #[doc = "No ECC target region"]
        pub const _000: Self = Self::new(0);

        #[doc = "0x2218_0000–0x2219_FFFF 0x3218_0000–0x3219_FFFF (128 KB)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramesr_SPEC;
impl crate::sealed::RegSpec for Sramesr_SPEC {
    type DataType = u16;
}

#[doc = "SRAM Error Status Register For ECC RAM"]
pub type Sramesr = crate::RegValueT<Sramesr_SPEC>;

impl Sramesr {
    #[doc = "SRAM0 1-bit ECC Error Status"]
    #[inline(always)]
    pub fn err00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramesr::Err00,
        sramesr::Err00,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramesr::Err00,
            sramesr::Err00,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM0 2-bit ECC Error Status"]
    #[inline(always)]
    pub fn err01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sramesr::Err01,
        sramesr::Err01,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sramesr::Err01,
            sramesr::Err01,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM1 1-bit ECC Error Status"]
    #[inline(always)]
    pub fn err10(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sramesr::Err10,
        sramesr::Err10,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sramesr::Err10,
            sramesr::Err10,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM1 2-bit ECC Error Status"]
    #[inline(always)]
    pub fn err11(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sramesr::Err11,
        sramesr::Err11,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sramesr::Err11,
            sramesr::Err11,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM2 1-bit ECC Error Status"]
    #[inline(always)]
    pub fn err20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sramesr::Err20,
        sramesr::Err20,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sramesr::Err20,
            sramesr::Err20,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM2 2-bit ECC Error Status"]
    #[inline(always)]
    pub fn err21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sramesr::Err21,
        sramesr::Err21,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sramesr::Err21,
            sramesr::Err21,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM3 1-bit ECC Error Status"]
    #[inline(always)]
    pub fn err30(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sramesr::Err30,
        sramesr::Err30,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sramesr::Err30,
            sramesr::Err30,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM3 2-bit ECC Error Status"]
    #[inline(always)]
    pub fn err31(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sramesr::Err31,
        sramesr::Err31,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sramesr::Err31,
            sramesr::Err31,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramesr {
    #[inline(always)]
    fn default() -> Sramesr {
        <crate::RegValueT<Sramesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err00_SPEC;
    pub type Err00 = crate::EnumBitfieldStruct<u8, Err00_SPEC>;
    impl Err00 {
        #[doc = "1-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err01_SPEC;
    pub type Err01 = crate::EnumBitfieldStruct<u8, Err01_SPEC>;
    impl Err01 {
        #[doc = "2-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err10_SPEC;
    pub type Err10 = crate::EnumBitfieldStruct<u8, Err10_SPEC>;
    impl Err10 {
        #[doc = "1-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err11_SPEC;
    pub type Err11 = crate::EnumBitfieldStruct<u8, Err11_SPEC>;
    impl Err11 {
        #[doc = "2-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err20_SPEC;
    pub type Err20 = crate::EnumBitfieldStruct<u8, Err20_SPEC>;
    impl Err20 {
        #[doc = "1-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err21_SPEC;
    pub type Err21 = crate::EnumBitfieldStruct<u8, Err21_SPEC>;
    impl Err21 {
        #[doc = "2-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err30_SPEC;
    pub type Err30 = crate::EnumBitfieldStruct<u8, Err30_SPEC>;
    impl Err30 {
        #[doc = "1-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err31_SPEC;
    pub type Err31 = crate::EnumBitfieldStruct<u8, Err31_SPEC>;
    impl Err31 {
        #[doc = "2-bit ECC error has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "2-bit ECC error has occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramesclr_SPEC;
impl crate::sealed::RegSpec for Sramesclr_SPEC {
    type DataType = u16;
}

#[doc = "SRAM Error Status Clear Register For ECC RAM"]
pub type Sramesclr = crate::RegValueT<Sramesclr_SPEC>;

impl Sramesclr {
    #[doc = "SRAM0 1-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramesclr::Clr00,
        sramesclr::Clr00,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramesclr::Clr00,
            sramesclr::Clr00,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SRAM0 2-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sramesclr::Clr01,
        sramesclr::Clr01,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sramesclr::Clr01,
            sramesclr::Clr01,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SRAM1 1-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr10(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sramesclr::Clr10,
        sramesclr::Clr10,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sramesclr::Clr10,
            sramesclr::Clr10,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SRAM1 2-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr11(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sramesclr::Clr11,
        sramesclr::Clr11,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sramesclr::Clr11,
            sramesclr::Clr11,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SRAM2 1-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sramesclr::Clr20,
        sramesclr::Clr20,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sramesclr::Clr20,
            sramesclr::Clr20,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SRAM2 2-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sramesclr::Clr21,
        sramesclr::Clr21,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sramesclr::Clr21,
            sramesclr::Clr21,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SRAM3 1-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr30(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sramesclr::Clr30,
        sramesclr::Clr30,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sramesclr::Clr30,
            sramesclr::Clr30,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SRAM3 2-bit ECC Error Status Clear"]
    #[inline(always)]
    pub fn clr31(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sramesclr::Clr31,
        sramesclr::Clr31,
        Sramesclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sramesclr::Clr31,
            sramesclr::Clr31,
            Sramesclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramesclr {
    #[inline(always)]
    fn default() -> Sramesclr {
        <crate::RegValueT<Sramesclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramesclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr00_SPEC;
    pub type Clr00 = crate::EnumBitfieldStruct<u8, Clr00_SPEC>;
    impl Clr00 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 1-bit ECC error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr01_SPEC;
    pub type Clr01 = crate::EnumBitfieldStruct<u8, Clr01_SPEC>;
    impl Clr01 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 2-bit ECC error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr10_SPEC;
    pub type Clr10 = crate::EnumBitfieldStruct<u8, Clr10_SPEC>;
    impl Clr10 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 1-bit ECC error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr11_SPEC;
    pub type Clr11 = crate::EnumBitfieldStruct<u8, Clr11_SPEC>;
    impl Clr11 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 2-bit ECC error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr20_SPEC;
    pub type Clr20 = crate::EnumBitfieldStruct<u8, Clr20_SPEC>;
    impl Clr20 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 1-bit ECC error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr21_SPEC;
    pub type Clr21 = crate::EnumBitfieldStruct<u8, Clr21_SPEC>;
    impl Clr21 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 2-bit ECC error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr30_SPEC;
    pub type Clr30 = crate::EnumBitfieldStruct<u8, Clr30_SPEC>;
    impl Clr30 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 1-bit ECC error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr31_SPEC;
    pub type Clr31 = crate::EnumBitfieldStruct<u8, Clr31_SPEC>;
    impl Clr31 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear 2-bit ECC error."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramear0_SPEC;
impl crate::sealed::RegSpec for Sramear0_SPEC {
    type DataType = u32;
}

#[doc = "SRAM Error Address Register n0 For ECC RAM"]
pub type Sramear0 = crate::RegValueT<Sramear0_SPEC>;

impl NoBitfieldReg<Sramear0_SPEC> for Sramear0 {}
impl ::core::default::Default for Sramear0 {
    #[inline(always)]
    fn default() -> Sramear0 {
        <crate::RegValueT<Sramear0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramear1_SPEC;
impl crate::sealed::RegSpec for Sramear1_SPEC {
    type DataType = u32;
}

#[doc = "SRAM Error Address Register n1 For ECC RAM"]
pub type Sramear1 = crate::RegValueT<Sramear1_SPEC>;

impl NoBitfieldReg<Sramear1_SPEC> for Sramear1 {}
impl ::core::default::Default for Sramear1 {
    #[inline(always)]
    fn default() -> Sramear1 {
        <crate::RegValueT<Sramear1_SPEC> as RegisterValue<_>>::new(0)
    }
}
