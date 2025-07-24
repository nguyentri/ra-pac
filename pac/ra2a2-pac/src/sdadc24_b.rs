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
// Generated from SVD 1.20.02, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:19 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"24-Bit Sigma-Delta A/D Converter B"]
unsafe impl ::core::marker::Send for super::Sdadc24B {}
unsafe impl ::core::marker::Sync for super::Sdadc24B {}
impl super::Sdadc24B {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Sigma-Delta A/D Clock Control Register"]
    #[inline(always)]
    pub const fn sdadccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Mode Register"]
    #[inline(always)]
    pub const fn sdadmr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Reset Register"]
    #[inline(always)]
    pub const fn sdadrr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Gain Control Register"]
    #[inline(always)]
    pub const fn sdadgcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadgcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadgcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D HPF Control Register"]
    #[inline(always)]
    pub const fn sdadhpfcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadhpfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadhpfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Interrupt Control Register"]
    #[inline(always)]
    pub const fn sdadicr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Interrupt Clear Register"]
    #[inline(always)]
    pub const fn sdadiclr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadiclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadiclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Interrupt Status Register"]
    #[inline(always)]
    pub const fn sdadisr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Phase Control Register %s"]
    #[inline(always)]
    pub const fn sdadphcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn sdadphcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadphcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadphcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadphcr3(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadphcr4(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadphcr5(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadphcr6(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadphcr7(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadphcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Conversion Result Register %s Type 1"]
    #[inline(always)]
    pub const fn sdadcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcr_SPEC, crate::common::R>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn sdadcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr3(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr4(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr5(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr6(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Conversion Result Register %s Type 2"]
    #[inline(always)]
    pub const fn sdadcrt2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcrt2_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa0usize))
        }
    }
    #[inline(always)]
    pub const fn sdadcr0t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrt2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrt2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr1t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrt2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrt2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr2t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrt2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrt2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcr3t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrt2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrt2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Conversion Result Register (LPF) %s Type 1"]
    #[inline(always)]
    pub const fn sdadcrlpf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf0(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf1(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf3(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf4(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf5(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf6(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }

    #[doc = "Sigma-Delta A/D Conversion Result Register (LPF) %s Type 2"]
    #[inline(always)]
    pub const fn sdadcrlpft2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcrlpft2_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf0t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpft2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpft2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf1t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpft2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpft2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf2t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpft2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpft2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdadcrlpf3t2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadcrlpft2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdadcrlpft2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadccr_SPEC;
impl crate::sealed::RegSpec for Sdadccr_SPEC {
    type DataType = u8;
}

#[doc = "Sigma-Delta A/D Clock Control Register"]
pub type Sdadccr = crate::RegValueT<Sdadccr_SPEC>;

impl Sdadccr {
    #[doc = "Operating clock of the digital block"]
    #[inline(always)]
    pub fn ck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sdadccr::Ck,
        sdadccr::Ck,
        Sdadccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sdadccr::Ck,
            sdadccr::Ck,
            Sdadccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadccr {
    #[inline(always)]
    fn default() -> Sdadccr {
        <crate::RegValueT<Sdadccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ck_SPEC;
    pub type Ck = crate::EnumBitfieldStruct<u8, Ck_SPEC>;
    impl Ck {
        #[doc = "Disabled"]
        pub const _00: Self = Self::new(0);

        #[doc = "Set when SDADCCLK frequency is 12.0 MHz or 12.8 MHz"]
        pub const _10: Self = Self::new(2);

        #[doc = "Set when SDADCCLK frequency is 16.0 MHz"]
        pub const _11: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadmr_SPEC;
impl crate::sealed::RegSpec for Sdadmr_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-Delta A/D Mode Register"]
pub type Sdadmr = crate::RegValueT<Sdadmr_SPEC>;

impl Sdadmr {
    #[doc = "Sigma-Delta A/D converter operation enable for channel n"]
    #[inline(always)]
    pub fn ce0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadmr::Ce0,
        sdadmr::Ce0,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadmr::Ce0,
            sdadmr::Ce0,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter operation enable for channel n"]
    #[inline(always)]
    pub fn ce1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadmr::Ce1,
        sdadmr::Ce1,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadmr::Ce1,
            sdadmr::Ce1,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter operation enable for channel n"]
    #[inline(always)]
    pub fn ce2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sdadmr::Ce2,
        sdadmr::Ce2,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sdadmr::Ce2,
            sdadmr::Ce2,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter operation enable for channel n"]
    #[inline(always)]
    pub fn ce3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sdadmr::Ce3,
        sdadmr::Ce3,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sdadmr::Ce3,
            sdadmr::Ce3,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter operation enable for channel n"]
    #[inline(always)]
    pub fn ce4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadmr::Ce4,
        sdadmr::Ce4,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadmr::Ce4,
            sdadmr::Ce4,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter operation enable for channel n"]
    #[inline(always)]
    pub fn ce5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadmr::Ce5,
        sdadmr::Ce5,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadmr::Ce5,
            sdadmr::Ce5,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter operation enable for channel n"]
    #[inline(always)]
    pub fn ce6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sdadmr::Ce6,
        sdadmr::Ce6,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sdadmr::Ce6,
            sdadmr::Ce6,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter power-on control for channel n"]
    #[inline(always)]
    pub fn pon0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        sdadmr::Pon0,
        sdadmr::Pon0,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            sdadmr::Pon0,
            sdadmr::Pon0,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter power-on control for channel n"]
    #[inline(always)]
    pub fn pon1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        sdadmr::Pon1,
        sdadmr::Pon1,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            sdadmr::Pon1,
            sdadmr::Pon1,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter power-on control for channel n"]
    #[inline(always)]
    pub fn pon2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        sdadmr::Pon2,
        sdadmr::Pon2,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            sdadmr::Pon2,
            sdadmr::Pon2,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter power-on control for channel n"]
    #[inline(always)]
    pub fn pon3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        sdadmr::Pon3,
        sdadmr::Pon3,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            sdadmr::Pon3,
            sdadmr::Pon3,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter power-on control for channel n"]
    #[inline(always)]
    pub fn pon4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        sdadmr::Pon4,
        sdadmr::Pon4,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            sdadmr::Pon4,
            sdadmr::Pon4,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter power-on control for channel n"]
    #[inline(always)]
    pub fn pon5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        sdadmr::Pon5,
        sdadmr::Pon5,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            sdadmr::Pon5,
            sdadmr::Pon5,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sigma-Delta A/D converter power-on control for channel n"]
    #[inline(always)]
    pub fn pon6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        sdadmr::Pon6,
        sdadmr::Pon6,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            sdadmr::Pon6,
            sdadmr::Pon6,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sampling mode select"]
    #[inline(always)]
    pub fn fr(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        sdadmr::Fr,
        sdadmr::Fr,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            sdadmr::Fr,
            sdadmr::Fr,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Resolution of A/D conversion result reading"]
    #[inline(always)]
    pub fn typ(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sdadmr::Typ,
        sdadmr::Typ,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sdadmr::Typ,
            sdadmr::Typ,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadmr {
    #[inline(always)]
    fn default() -> Sdadmr {
        <crate::RegValueT<Sdadmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce0_SPEC;
    pub type Ce0 = crate::EnumBitfieldStruct<u8, Ce0_SPEC>;
    impl Ce0 {
        #[doc = "Electric charge reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce1_SPEC;
    pub type Ce1 = crate::EnumBitfieldStruct<u8, Ce1_SPEC>;
    impl Ce1 {
        #[doc = "Electric charge reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce2_SPEC;
    pub type Ce2 = crate::EnumBitfieldStruct<u8, Ce2_SPEC>;
    impl Ce2 {
        #[doc = "Electric charge reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce3_SPEC;
    pub type Ce3 = crate::EnumBitfieldStruct<u8, Ce3_SPEC>;
    impl Ce3 {
        #[doc = "Electric charge reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce4_SPEC;
    pub type Ce4 = crate::EnumBitfieldStruct<u8, Ce4_SPEC>;
    impl Ce4 {
        #[doc = "Electric charge reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce5_SPEC;
    pub type Ce5 = crate::EnumBitfieldStruct<u8, Ce5_SPEC>;
    impl Ce5 {
        #[doc = "Electric charge reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce6_SPEC;
    pub type Ce6 = crate::EnumBitfieldStruct<u8, Ce6_SPEC>;
    impl Ce6 {
        #[doc = "Electric charge reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Normal operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon0_SPEC;
    pub type Pon0 = crate::EnumBitfieldStruct<u8, Pon0_SPEC>;
    impl Pon0 {
        #[doc = "Power off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon1_SPEC;
    pub type Pon1 = crate::EnumBitfieldStruct<u8, Pon1_SPEC>;
    impl Pon1 {
        #[doc = "Power off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon2_SPEC;
    pub type Pon2 = crate::EnumBitfieldStruct<u8, Pon2_SPEC>;
    impl Pon2 {
        #[doc = "Power off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon3_SPEC;
    pub type Pon3 = crate::EnumBitfieldStruct<u8, Pon3_SPEC>;
    impl Pon3 {
        #[doc = "Power off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon4_SPEC;
    pub type Pon4 = crate::EnumBitfieldStruct<u8, Pon4_SPEC>;
    impl Pon4 {
        #[doc = "Power off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon5_SPEC;
    pub type Pon5 = crate::EnumBitfieldStruct<u8, Pon5_SPEC>;
    impl Pon5 {
        #[doc = "Power off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon6_SPEC;
    pub type Pon6 = crate::EnumBitfieldStruct<u8, Pon6_SPEC>;
    impl Pon6 {
        #[doc = "Power off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Power on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fr_SPEC;
    pub type Fr = crate::EnumBitfieldStruct<u8, Fr_SPEC>;
    impl Fr {
        #[doc = "4 kHz sampling mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "8 kHz sampling mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "8 kHz/4 kHz hybrid sampling mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Typ_SPEC;
    pub type Typ = crate::EnumBitfieldStruct<u8, Typ_SPEC>;
    impl Typ {
        #[doc = "24-bit resolution"]
        pub const _0: Self = Self::new(0);

        #[doc = "16-bit resolution"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadrr_SPEC;
impl crate::sealed::RegSpec for Sdadrr_SPEC {
    type DataType = u8;
}

#[doc = "Sigma-Delta A/D Reset Register"]
pub type Sdadrr = crate::RegValueT<Sdadrr_SPEC>;

impl Sdadrr {
    #[doc = "Sigma-Delta A/D converter reset"]
    #[inline(always)]
    pub fn res(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadrr::Res,
        sdadrr::Res,
        Sdadrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadrr::Res,
            sdadrr::Res,
            Sdadrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadrr {
    #[inline(always)]
    fn default() -> Sdadrr {
        <crate::RegValueT<Sdadrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Res_SPEC;
    pub type Res = crate::EnumBitfieldStruct<u8, Res_SPEC>;
    impl Res {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadgcr_SPEC;
impl crate::sealed::RegSpec for Sdadgcr_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-Delta A/D Gain Control Register"]
pub type Sdadgcr = crate::RegValueT<Sdadgcr_SPEC>;

impl Sdadgcr {
    #[doc = "Preamplifier gain for channel 0"]
    #[inline(always)]
    pub fn gain0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sdadgcr::Gain0,
        sdadgcr::Gain0,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sdadgcr::Gain0,
            sdadgcr::Gain0,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamplifier gain for channel 1"]
    #[inline(always)]
    pub fn gain1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        sdadgcr::Gain1,
        sdadgcr::Gain1,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            sdadgcr::Gain1,
            sdadgcr::Gain1,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamplifier gain for channel 2"]
    #[inline(always)]
    pub fn gain2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        sdadgcr::Gain2,
        sdadgcr::Gain2,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            sdadgcr::Gain2,
            sdadgcr::Gain2,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamplifier gain for channel 3"]
    #[inline(always)]
    pub fn gain3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        sdadgcr::Gain3,
        sdadgcr::Gain3,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            sdadgcr::Gain3,
            sdadgcr::Gain3,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamplifier gain for channel 4"]
    #[inline(always)]
    pub fn gain4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        sdadgcr::Gain4,
        sdadgcr::Gain4,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            sdadgcr::Gain4,
            sdadgcr::Gain4,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamplifier gain for channel 5"]
    #[inline(always)]
    pub fn gain5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
        sdadgcr::Gain5,
        sdadgcr::Gain5,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x7,
            1,
            0,
            sdadgcr::Gain5,
            sdadgcr::Gain5,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Preamplifier gain for channel 6"]
    #[inline(always)]
    pub fn gain6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        sdadgcr::Gain6,
        sdadgcr::Gain6,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            sdadgcr::Gain6,
            sdadgcr::Gain6,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadgcr {
    #[inline(always)]
    fn default() -> Sdadgcr {
        <crate::RegValueT<Sdadgcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadgcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain0_SPEC;
    pub type Gain0 = crate::EnumBitfieldStruct<u8, Gain0_SPEC>;
    impl Gain0 {
        #[doc = "1x"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2x"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4x"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "8x"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16x"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "32x"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain1_SPEC;
    pub type Gain1 = crate::EnumBitfieldStruct<u8, Gain1_SPEC>;
    impl Gain1 {
        #[doc = "1x"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2x"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4x"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "8x"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16x"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "32x"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain2_SPEC;
    pub type Gain2 = crate::EnumBitfieldStruct<u8, Gain2_SPEC>;
    impl Gain2 {
        #[doc = "1x"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2x"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4x"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "8x"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16x"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "32x"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain3_SPEC;
    pub type Gain3 = crate::EnumBitfieldStruct<u8, Gain3_SPEC>;
    impl Gain3 {
        #[doc = "1x"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2x"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4x"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "8x"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16x"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "32x"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain4_SPEC;
    pub type Gain4 = crate::EnumBitfieldStruct<u8, Gain4_SPEC>;
    impl Gain4 {
        #[doc = "1x"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2x"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4x"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "8x"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16x"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "32x"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain5_SPEC;
    pub type Gain5 = crate::EnumBitfieldStruct<u8, Gain5_SPEC>;
    impl Gain5 {
        #[doc = "1x"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2x"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4x"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "8x"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16x"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "32x"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain6_SPEC;
    pub type Gain6 = crate::EnumBitfieldStruct<u8, Gain6_SPEC>;
    impl Gain6 {
        #[doc = "1x"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "2x"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "4x"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "8x"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "16x"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "32x"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadhpfcr_SPEC;
impl crate::sealed::RegSpec for Sdadhpfcr_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-Delta A/D HPF Control Register"]
pub type Sdadhpfcr = crate::RegValueT<Sdadhpfcr_SPEC>;

impl Sdadhpfcr {
    #[doc = "HPF bypass for channel n Type 1 output"]
    #[inline(always)]
    pub fn dis0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadhpfcr::Dis0,
        sdadhpfcr::Dis0,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadhpfcr::Dis0,
            sdadhpfcr::Dis0,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel n Type 1 output"]
    #[inline(always)]
    pub fn dis1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadhpfcr::Dis1,
        sdadhpfcr::Dis1,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadhpfcr::Dis1,
            sdadhpfcr::Dis1,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel n Type 1 output"]
    #[inline(always)]
    pub fn dis2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sdadhpfcr::Dis2,
        sdadhpfcr::Dis2,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sdadhpfcr::Dis2,
            sdadhpfcr::Dis2,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel n Type 1 output"]
    #[inline(always)]
    pub fn dis3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sdadhpfcr::Dis3,
        sdadhpfcr::Dis3,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sdadhpfcr::Dis3,
            sdadhpfcr::Dis3,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel n Type 1 output"]
    #[inline(always)]
    pub fn dis4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadhpfcr::Dis4,
        sdadhpfcr::Dis4,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadhpfcr::Dis4,
            sdadhpfcr::Dis4,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel n Type 1 output"]
    #[inline(always)]
    pub fn dis5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadhpfcr::Dis5,
        sdadhpfcr::Dis5,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadhpfcr::Dis5,
            sdadhpfcr::Dis5,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel n Type 1 output"]
    #[inline(always)]
    pub fn dis6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sdadhpfcr::Dis6,
        sdadhpfcr::Dis6,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sdadhpfcr::Dis6,
            sdadhpfcr::Dis6,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel m Type 2 output"]
    #[inline(always)]
    pub fn dis3t2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        sdadhpfcr::Dis3T2,
        sdadhpfcr::Dis3T2,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            sdadhpfcr::Dis3T2,
            sdadhpfcr::Dis3T2,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "HPF bypass for channel m Type 2 output"]
    #[inline(always)]
    pub fn dis0t2(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        sdadhpfcr::Dis0T2,
        sdadhpfcr::Dis0T2,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            sdadhpfcr::Dis0T2,
            sdadhpfcr::Dis0T2,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-off frequency of HPF (see )"]
    #[inline(always)]
    pub fn cof(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Sdadhpfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Sdadhpfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadhpfcr {
    #[inline(always)]
    fn default() -> Sdadhpfcr {
        <crate::RegValueT<Sdadhpfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadhpfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis0_SPEC;
    pub type Dis0 = crate::EnumBitfieldStruct<u8, Dis0_SPEC>;
    impl Dis0 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis1_SPEC;
    pub type Dis1 = crate::EnumBitfieldStruct<u8, Dis1_SPEC>;
    impl Dis1 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis2_SPEC;
    pub type Dis2 = crate::EnumBitfieldStruct<u8, Dis2_SPEC>;
    impl Dis2 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis3_SPEC;
    pub type Dis3 = crate::EnumBitfieldStruct<u8, Dis3_SPEC>;
    impl Dis3 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis4_SPEC;
    pub type Dis4 = crate::EnumBitfieldStruct<u8, Dis4_SPEC>;
    impl Dis4 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis5_SPEC;
    pub type Dis5 = crate::EnumBitfieldStruct<u8, Dis5_SPEC>;
    impl Dis5 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis6_SPEC;
    pub type Dis6 = crate::EnumBitfieldStruct<u8, Dis6_SPEC>;
    impl Dis6 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis3T2_SPEC;
    pub type Dis3T2 = crate::EnumBitfieldStruct<u8, Dis3T2_SPEC>;
    impl Dis3T2 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis0T2_SPEC;
    pub type Dis0T2 = crate::EnumBitfieldStruct<u8, Dis0T2_SPEC>;
    impl Dis0T2 {
        #[doc = "HPF enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "HPF disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadicr_SPEC;
impl crate::sealed::RegSpec for Sdadicr_SPEC {
    type DataType = u8;
}

#[doc = "Sigma-Delta A/D Interrupt Control Register"]
pub type Sdadicr = crate::RegValueT<Sdadicr_SPEC>;

impl Sdadicr {
    #[doc = "Detection channel for zero-cross detection 0"]
    #[inline(always)]
    pub fn zcctl0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadicr::Zcctl0,
        sdadicr::Zcctl0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadicr::Zcctl0,
            sdadicr::Zcctl0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt output mode for zero-cross detection 0"]
    #[inline(always)]
    pub fn zcmd0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadicr::Zcmd0,
        sdadicr::Zcmd0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadicr::Zcmd0,
            sdadicr::Zcmd0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Falling edge detection enable for zero-cross detection 0"]
    #[inline(always)]
    pub fn zcegn0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sdadicr::Zcegn0,
        sdadicr::Zcegn0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sdadicr::Zcegn0,
            sdadicr::Zcegn0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Rising edge detection enable for zero-cross detection 0"]
    #[inline(always)]
    pub fn zcegp0(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sdadicr::Zcegp0,
        sdadicr::Zcegp0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sdadicr::Zcegp0,
            sdadicr::Zcegp0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Detection channel for zero-cross detection 1"]
    #[inline(always)]
    pub fn zcctl1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadicr::Zcctl1,
        sdadicr::Zcctl1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadicr::Zcctl1,
            sdadicr::Zcctl1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt output mode for zero-cross detection 1"]
    #[inline(always)]
    pub fn zcmd1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadicr::Zcmd1,
        sdadicr::Zcmd1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadicr::Zcmd1,
            sdadicr::Zcmd1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Falling edge detection enable for zero-cross detection 1"]
    #[inline(always)]
    pub fn zcegn1(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sdadicr::Zcegn1,
        sdadicr::Zcegn1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sdadicr::Zcegn1,
            sdadicr::Zcegn1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Rising edge detection enable for Zero-cross detection 1"]
    #[inline(always)]
    pub fn zcegp1(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sdadicr::Zcegp1,
        sdadicr::Zcegp1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sdadicr::Zcegp1,
            sdadicr::Zcegp1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadicr {
    #[inline(always)]
    fn default() -> Sdadicr {
        <crate::RegValueT<Sdadicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcctl0_SPEC;
    pub type Zcctl0 = crate::EnumBitfieldStruct<u8, Zcctl0_SPEC>;
    impl Zcctl0 {
        #[doc = "Channel 2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Channel 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcmd0_SPEC;
    pub type Zcmd0 = crate::EnumBitfieldStruct<u8, Zcmd0_SPEC>;
    impl Zcmd0 {
        #[doc = "Pulse output mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Level output mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegn0_SPEC;
    pub type Zcegn0 = crate::EnumBitfieldStruct<u8, Zcegn0_SPEC>;
    impl Zcegn0 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegp0_SPEC;
    pub type Zcegp0 = crate::EnumBitfieldStruct<u8, Zcegp0_SPEC>;
    impl Zcegp0 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcctl1_SPEC;
    pub type Zcctl1 = crate::EnumBitfieldStruct<u8, Zcctl1_SPEC>;
    impl Zcctl1 {
        #[doc = "Channel 3"]
        pub const _0: Self = Self::new(0);

        #[doc = "Channel 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcmd1_SPEC;
    pub type Zcmd1 = crate::EnumBitfieldStruct<u8, Zcmd1_SPEC>;
    impl Zcmd1 {
        #[doc = "Pulse output mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Level output mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegn1_SPEC;
    pub type Zcegn1 = crate::EnumBitfieldStruct<u8, Zcegn1_SPEC>;
    impl Zcegn1 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegp1_SPEC;
    pub type Zcegp1 = crate::EnumBitfieldStruct<u8, Zcegp1_SPEC>;
    impl Zcegp1 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadiclr_SPEC;
impl crate::sealed::RegSpec for Sdadiclr_SPEC {
    type DataType = u8;
}

#[doc = "Sigma-Delta A/D Interrupt Clear Register"]
pub type Sdadiclr = crate::RegValueT<Sdadiclr_SPEC>;

impl Sdadiclr {
    #[doc = "Zero-cross detection interrupt 0 clear"]
    #[inline(always)]
    pub fn icl0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadiclr::Icl0,
        sdadiclr::Icl0,
        Sdadiclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadiclr::Icl0,
            sdadiclr::Icl0,
            Sdadiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Zero-cross detection interrupt 1 clear"]
    #[inline(always)]
    pub fn icl1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadiclr::Icl1,
        sdadiclr::Icl1,
        Sdadiclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadiclr::Icl1,
            sdadiclr::Icl1,
            Sdadiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadiclr {
    #[inline(always)]
    fn default() -> Sdadiclr {
        <crate::RegValueT<Sdadiclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadiclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icl0_SPEC;
    pub type Icl0 = crate::EnumBitfieldStruct<u8, Icl0_SPEC>;
    impl Icl0 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears zero-cross detection interrupt 0 (SDADC_ADZC0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icl1_SPEC;
    pub type Icl1 = crate::EnumBitfieldStruct<u8, Icl1_SPEC>;
    impl Icl1 {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears zero-cross detection interrupt 1 (SDADC_ADZC1)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadisr_SPEC;
impl crate::sealed::RegSpec for Sdadisr_SPEC {
    type DataType = u8;
}

#[doc = "Sigma-Delta A/D Interrupt Status Register"]
pub type Sdadisr = crate::RegValueT<Sdadisr_SPEC>;

impl Sdadisr {
    #[doc = "Zero-cross detection interrupt status for zero-cross detection 0"]
    #[inline(always)]
    pub fn zci0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadisr::Zci0,
        sdadisr::Zci0,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadisr::Zci0,
            sdadisr::Zci0,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DF output status for zero-cross detection 0"]
    #[inline(always)]
    pub fn zc0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadisr::Zc0,
        sdadisr::Zc0,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadisr::Zc0,
            sdadisr::Zc0,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Zero-cross detection interrupt status for zero-cross detection 1"]
    #[inline(always)]
    pub fn zci1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadisr::Zci1,
        sdadisr::Zci1,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadisr::Zci1,
            sdadisr::Zci1,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DF output status for Zero-cross detection 1"]
    #[inline(always)]
    pub fn zc1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadisr::Zc1,
        sdadisr::Zc1,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadisr::Zc1,
            sdadisr::Zc1,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadisr {
    #[inline(always)]
    fn default() -> Sdadisr {
        <crate::RegValueT<Sdadisr_SPEC> as RegisterValue<_>>::new(34)
    }
}
pub mod sdadisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zci0_SPEC;
    pub type Zci0 = crate::EnumBitfieldStruct<u8, Zci0_SPEC>;
    impl Zci0 {
        #[doc = "Zero-cross detection interrupt 0 (SDADC_ADZC0) signal is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Zero-cross detection interrupt 0 (SDADC_ADZC0) signal is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zc0_SPEC;
    pub type Zc0 = crate::EnumBitfieldStruct<u8, Zc0_SPEC>;
    impl Zc0 {
        #[doc = "DF output is negative"]
        pub const _0: Self = Self::new(0);

        #[doc = "DF output is positive"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zci1_SPEC;
    pub type Zci1 = crate::EnumBitfieldStruct<u8, Zci1_SPEC>;
    impl Zci1 {
        #[doc = "Zero-cross detection interrupt 1 (SDADC_ADZC1) signal is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Zero-cross detection interrupt 1 (SDADC_ADZC1) signal is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zc1_SPEC;
    pub type Zc1 = crate::EnumBitfieldStruct<u8, Zc1_SPEC>;
    impl Zc1 {
        #[doc = "DF output is negative"]
        pub const _0: Self = Self::new(0);

        #[doc = "DF output is positive"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadphcr_SPEC;
impl crate::sealed::RegSpec for Sdadphcr_SPEC {
    type DataType = u16;
}

#[doc = "Sigma-Delta A/D Phase Control Register %s"]
pub type Sdadphcr = crate::RegValueT<Sdadphcr_SPEC>;

impl Sdadphcr {
    #[doc = "Phase adjustment for channel n (n = 0 to 6)"]
    #[inline(always)]
    pub fn ph(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Sdadphcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Sdadphcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadphcr {
    #[inline(always)]
    fn default() -> Sdadphcr {
        <crate::RegValueT<Sdadphcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcr_SPEC;
impl crate::sealed::RegSpec for Sdadcr_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-Delta A/D Conversion Result Register %s Type 1"]
pub type Sdadcr = crate::RegValueT<Sdadcr_SPEC>;

impl Sdadcr {
    #[doc = "A/D conversion result for channel n Type 1"]
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sdadcr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Sdadcr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadcr {
    #[inline(always)]
    fn default() -> Sdadcr {
        <crate::RegValueT<Sdadcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcrt2_SPEC;
impl crate::sealed::RegSpec for Sdadcrt2_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-Delta A/D Conversion Result Register %s Type 2"]
pub type Sdadcrt2 = crate::RegValueT<Sdadcrt2_SPEC>;

impl Sdadcrt2 {
    #[doc = "A/D conversion result for channel m Type 2"]
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sdadcrt2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Sdadcrt2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadcrt2 {
    #[inline(always)]
    fn default() -> Sdadcrt2 {
        <crate::RegValueT<Sdadcrt2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcrlpf_SPEC;
impl crate::sealed::RegSpec for Sdadcrlpf_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-Delta A/D Conversion Result Register (LPF) %s Type 1"]
pub type Sdadcrlpf = crate::RegValueT<Sdadcrlpf_SPEC>;

impl Sdadcrlpf {
    #[doc = "A/D conversion result for channel n LPF output Type 1"]
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sdadcrlpf_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Sdadcrlpf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadcrlpf {
    #[inline(always)]
    fn default() -> Sdadcrlpf {
        <crate::RegValueT<Sdadcrlpf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcrlpft2_SPEC;
impl crate::sealed::RegSpec for Sdadcrlpft2_SPEC {
    type DataType = u32;
}

#[doc = "Sigma-Delta A/D Conversion Result Register (LPF) %s Type 2"]
pub type Sdadcrlpft2 = crate::RegValueT<Sdadcrlpft2_SPEC>;

impl Sdadcrlpft2 {
    #[doc = "A/D conversion result for channel m LPF output Type 2"]
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Sdadcrlpft2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Sdadcrlpft2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadcrlpft2 {
    #[inline(always)]
    fn default() -> Sdadcrlpft2 {
        <crate::RegValueT<Sdadcrlpft2_SPEC> as RegisterValue<_>>::new(0)
    }
}
