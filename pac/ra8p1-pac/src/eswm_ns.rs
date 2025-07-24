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
#[doc = r"Layer 3 Ethernet Switch Module"]
unsafe impl ::core::marker::Send for super::EswmNs {}
unsafe impl ::core::marker::Sync for super::EswmNs {}
impl super::EswmNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Error and Monitoring Interrupt Mapping Configuration Register 0"]
    #[inline(always)]
    pub const fn tpemimc0(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Error and Monitoring Interrupt Mapping Configuration Register 1"]
    #[inline(always)]
    pub const fn tpemimc1(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Error and Monitoring Interrupt Mapping Configuration Register 2"]
    #[inline(always)]
    pub const fn tpemimc2(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Error and Monitoring Interrupt Mapping Configuration Register 3"]
    #[inline(always)]
    pub const fn tpemimc3(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Error and Monitoring Interrupt Mapping Configuration Register 4"]
    #[inline(always)]
    pub const fn tpemimc4(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Error and Monitoring Interrupt Mapping Configuration Register 6%s"]
    #[inline(always)]
    pub const fn tpemimc6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tpemimc6_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn tpemimc60(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc61(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc62(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc63(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc64(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }

    #[doc = "Error and Monitoring Interrupt Mapping Configuration Register 7%s"]
    #[inline(always)]
    pub const fn tpemimc7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tpemimc7_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn tpemimc70(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc71(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc72(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc73(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tpemimc74(
        &self,
    ) -> &'static crate::common::Reg<self::Tpemimc7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpemimc7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }

    #[doc = "Summarized Interrupt Mirroring Register"]
    #[inline(always)]
    pub const fn tsim(&self) -> &'static crate::common::Reg<self::Tsim_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tsim_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1792usize),
            )
        }
    }

    #[doc = "MFWD Interrupt Mirroring Register"]
    #[inline(always)]
    pub const fn tfim(&self) -> &'static crate::common::Reg<self::Tfim_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tfim_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1796usize),
            )
        }
    }

    #[doc = "COMA Interrupt Mirroring Register"]
    #[inline(always)]
    pub const fn tcim(&self) -> &'static crate::common::Reg<self::Tcim_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tcim_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1800usize),
            )
        }
    }

    #[doc = "GWCA0 Interrupt Mirroring Register"]
    #[inline(always)]
    pub const fn tgim0(&self) -> &'static crate::common::Reg<self::Tgim0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tgim0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1808usize),
            )
        }
    }

    #[doc = "ETHA0 Interrupt Mirroring Register"]
    #[inline(always)]
    pub const fn teim0(&self) -> &'static crate::common::Reg<self::Teim0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Teim0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1824usize),
            )
        }
    }

    #[doc = "ETHA1 Interrupt Mirroring Register"]
    #[inline(always)]
    pub const fn teim1(&self) -> &'static crate::common::Reg<self::Teim1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Teim1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1828usize),
            )
        }
    }

    #[doc = "Media-independent Interface Reset Register"]
    #[inline(always)]
    pub const fn miirr(&self) -> &'static crate::common::Reg<self::Miirr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Miirr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(103424usize),
            )
        }
    }

    #[doc = "Media-independent Interface Control Register 0"]
    #[inline(always)]
    pub const fn miicr0(
        &self,
    ) -> &'static crate::common::Reg<self::Miicr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Miicr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(103428usize),
            )
        }
    }

    #[doc = "Media-independent Interface Control Register 1"]
    #[inline(always)]
    pub const fn miicr1(
        &self,
    ) -> &'static crate::common::Reg<self::Miicr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Miicr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(103432usize),
            )
        }
    }

    #[doc = "Media Clock Capture Event Select Register"]
    #[inline(always)]
    pub const fn mccesr(
        &self,
    ) -> &'static crate::common::Reg<self::Mccesr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mccesr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(103440usize),
            )
        }
    }

    #[doc = "TAS Status Monitor Signal Select Register"]
    #[inline(always)]
    pub const fn tasstsr(
        &self,
    ) -> &'static crate::common::Reg<self::Tasstsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tasstsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(103456usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpemimc0_SPEC;
impl crate::sealed::RegSpec for Tpemimc0_SPEC {
    type DataType = u32;
}

#[doc = "Error and Monitoring Interrupt Mapping Configuration Register 0"]
pub type Tpemimc0 = crate::RegValueT<Tpemimc0_SPEC>;

impl Tpemimc0 {
    #[doc = "Summarized Error Interrupt Mapping"]
    #[inline(always)]
    pub fn seim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tpemimc0::Seim,
        tpemimc0::Seim,
        Tpemimc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tpemimc0::Seim,
            tpemimc0::Seim,
            Tpemimc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Switch Error Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn seigm(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tpemimc0::Seigm,
        tpemimc0::Seigm,
        Tpemimc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tpemimc0::Seigm,
            tpemimc0::Seigm,
            Tpemimc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Summarized Error Interrupt Core Mapping"]
    #[inline(always)]
    pub fn seicm(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Tpemimc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Tpemimc0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Summarized Status Interrupt 0 Core Mapping"]
    #[inline(always)]
    pub fn ssicm0(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Tpemimc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Tpemimc0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Summarized Status Interrupt 1 Core Mapping"]
    #[inline(always)]
    pub fn ssicm1(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Tpemimc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Tpemimc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpemimc0 {
    #[inline(always)]
    fn default() -> Tpemimc0 {
        <crate::RegValueT<Tpemimc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tpemimc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seim_SPEC;
    pub type Seim = crate::EnumBitfieldStruct<u8, Seim_SPEC>;
    impl Seim {
        #[doc = "Summarized Error Interrupt is mapped to race_race_error_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "Summarized Error Interrupt is mapped to a specific core depending on TPEMIMC0.SEIGM register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seigm_SPEC;
    pub type Seigm = crate::EnumBitfieldStruct<u8, Seigm_SPEC>;
    impl Seigm {
        #[doc = "When TPEMIMC0.SEIM is set to 1, Switch Error Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC0.SEICM\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpemimc1_SPEC;
impl crate::sealed::RegSpec for Tpemimc1_SPEC {
    type DataType = u32;
}

#[doc = "Error and Monitoring Interrupt Mapping Configuration Register 1"]
pub type Tpemimc1 = crate::RegValueT<Tpemimc1_SPEC>;

impl Tpemimc1 {
    #[doc = "MFWD Error Interrupt Mapping"]
    #[inline(always)]
    pub fn feim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tpemimc1::Feim,
        tpemimc1::Feim,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tpemimc1::Feim,
            tpemimc1::Feim,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MFWD Error Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn feigm(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tpemimc1::Feigm,
        tpemimc1::Feigm,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tpemimc1::Feigm,
            tpemimc1::Feigm,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MFWD Error Interrupt Core Mapping"]
    #[inline(always)]
    pub fn feicm(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Tpemimc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Tpemimc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MFWD Status Interrupt Mapping"]
    #[inline(always)]
    pub fn fsim(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tpemimc1::Fsim,
        tpemimc1::Fsim,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tpemimc1::Fsim,
            tpemimc1::Fsim,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MFWD Status Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn fsigm(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        tpemimc1::Fsigm,
        tpemimc1::Fsigm,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tpemimc1::Fsigm,
            tpemimc1::Fsigm,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MFWD Status Interrupt Core Mapping"]
    #[inline(always)]
    pub fn fsicm(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Tpemimc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Tpemimc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "COMA Error Interrupt Mapping"]
    #[inline(always)]
    pub fn ceim(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        tpemimc1::Ceim,
        tpemimc1::Ceim,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            tpemimc1::Ceim,
            tpemimc1::Ceim,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "COMA Error Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn ceigm(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        tpemimc1::Ceigm,
        tpemimc1::Ceigm,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            tpemimc1::Ceigm,
            tpemimc1::Ceigm,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "COMA Error Interrupt Core Mapping"]
    #[inline(always)]
    pub fn ceicm(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Tpemimc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Tpemimc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "COMA Status Interrupt Mapping"]
    #[inline(always)]
    pub fn csim(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        tpemimc1::Csim,
        tpemimc1::Csim,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            tpemimc1::Csim,
            tpemimc1::Csim,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "COMA Status Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn csigm(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        tpemimc1::Csigm,
        tpemimc1::Csigm,
        Tpemimc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            tpemimc1::Csigm,
            tpemimc1::Csigm,
            Tpemimc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "COMA Status Interrupt Core Mapping"]
    #[inline(always)]
    pub fn csicm(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Tpemimc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Tpemimc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpemimc1 {
    #[inline(always)]
    fn default() -> Tpemimc1 {
        <crate::RegValueT<Tpemimc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tpemimc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Feim_SPEC;
    pub type Feim = crate::EnumBitfieldStruct<u8, Feim_SPEC>;
    impl Feim {
        #[doc = "MFWD Error Interrupt is mapped to race_mfwd_error_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "MFWD Error Interrupt is mapped to a specific core depending on TPEMIMC1.FEIGM register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Feigm_SPEC;
    pub type Feigm = crate::EnumBitfieldStruct<u8, Feigm_SPEC>;
    impl Feigm {
        #[doc = "When TPEMIMC1.FEIM is set to 1, MFWD Error Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC1.FEICM\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsim_SPEC;
    pub type Fsim = crate::EnumBitfieldStruct<u8, Fsim_SPEC>;
    impl Fsim {
        #[doc = "MFWD Status Interrupt is mapped to race_mfwd_status_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "MFWD Status Interrupt is mapped to a specific core depending on TPEMIMC1.FSIGM register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsigm_SPEC;
    pub type Fsigm = crate::EnumBitfieldStruct<u8, Fsigm_SPEC>;
    impl Fsigm {
        #[doc = "When TPEMIMC1.FSIM is set to 1, MFWD Status Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC1.FSICM\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceim_SPEC;
    pub type Ceim = crate::EnumBitfieldStruct<u8, Ceim_SPEC>;
    impl Ceim {
        #[doc = "COMA Error Interrupt is mapped to race_coma_error_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "COMA Error Interrupt is mapped to a specific core depending on TPEMIMC1.CEIGM register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceigm_SPEC;
    pub type Ceigm = crate::EnumBitfieldStruct<u8, Ceigm_SPEC>;
    impl Ceigm {
        #[doc = "When TPEMIMC1.CEIM is set to 1, COMA Error Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC1.CEICM\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csim_SPEC;
    pub type Csim = crate::EnumBitfieldStruct<u8, Csim_SPEC>;
    impl Csim {
        #[doc = "COMA Status Interrupt is mapped to race_coma_status_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "COMA Status Interrupt is mapped to a specific core depending on TPEMIMC1.CSIGM register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csigm_SPEC;
    pub type Csigm = crate::EnumBitfieldStruct<u8, Csigm_SPEC>;
    impl Csigm {
        #[doc = "When TPEMIMC1.CSIM is set to 1, COMA Status Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC1.CSICM\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpemimc2_SPEC;
impl crate::sealed::RegSpec for Tpemimc2_SPEC {
    type DataType = u32;
}

#[doc = "Error and Monitoring Interrupt Mapping Configuration Register 2"]
pub type Tpemimc2 = crate::RegValueT<Tpemimc2_SPEC>;

impl Tpemimc2 {
    #[doc = "GWCA0 Error Interrupt Mapping"]
    #[inline(always)]
    pub fn geim0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tpemimc2::Geim0,
        tpemimc2::Geim0,
        Tpemimc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tpemimc2::Geim0,
            tpemimc2::Geim0,
            Tpemimc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA0 Error Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn geigm0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tpemimc2::Geigm0,
        tpemimc2::Geigm0,
        Tpemimc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tpemimc2::Geigm0,
            tpemimc2::Geigm0,
            Tpemimc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA0 Error Interrupt Core Mapping"]
    #[inline(always)]
    pub fn geicm0(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Tpemimc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Tpemimc2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "GWCA0 Status Interrupt Mapping"]
    #[inline(always)]
    pub fn gsim0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tpemimc2::Gsim0,
        tpemimc2::Gsim0,
        Tpemimc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tpemimc2::Gsim0,
            tpemimc2::Gsim0,
            Tpemimc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA0 Status Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn gsigm0(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        tpemimc2::Gsigm0,
        tpemimc2::Gsigm0,
        Tpemimc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tpemimc2::Gsigm0,
            tpemimc2::Gsigm0,
            Tpemimc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA0 Status Interrupt Core Mapping"]
    #[inline(always)]
    pub fn gsicm0(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Tpemimc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Tpemimc2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpemimc2 {
    #[inline(always)]
    fn default() -> Tpemimc2 {
        <crate::RegValueT<Tpemimc2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tpemimc2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Geim0_SPEC;
    pub type Geim0 = crate::EnumBitfieldStruct<u8, Geim0_SPEC>;
    impl Geim0 {
        #[doc = "GWCA0 Error Interrupt is mapped to race_ gwca0_error_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "GWCA0 Error Interrupt is mapped to a specific core depending on TPEMIMC2.GEIGM0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Geigm0_SPEC;
    pub type Geigm0 = crate::EnumBitfieldStruct<u8, Geigm0_SPEC>;
    impl Geigm0 {
        #[doc = "When TPEMIMC2.GEIM0 is set to 1, GWCA0 Error Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC2.GEICM0\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gsim0_SPEC;
    pub type Gsim0 = crate::EnumBitfieldStruct<u8, Gsim0_SPEC>;
    impl Gsim0 {
        #[doc = "GWCA0 Status Interrupt is mapped to race_ gwca0_status_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "GWCA0 Status Interrupt is mapped to a specific core depending on TPEMIMC2.GSIGM0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gsigm0_SPEC;
    pub type Gsigm0 = crate::EnumBitfieldStruct<u8, Gsigm0_SPEC>;
    impl Gsigm0 {
        #[doc = "When TPEMIMC2.GSIM0 is set to 1, GWCA0 Status Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC2.GSICM0\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpemimc3_SPEC;
impl crate::sealed::RegSpec for Tpemimc3_SPEC {
    type DataType = u32;
}

#[doc = "Error and Monitoring Interrupt Mapping Configuration Register 3"]
pub type Tpemimc3 = crate::RegValueT<Tpemimc3_SPEC>;

impl Tpemimc3 {
    #[doc = "ETHA0 Error Interrupt Mapping"]
    #[inline(always)]
    pub fn eeim0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tpemimc3::Eeim0,
        tpemimc3::Eeim0,
        Tpemimc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tpemimc3::Eeim0,
            tpemimc3::Eeim0,
            Tpemimc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA0 Error Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn eeigm0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tpemimc3::Eeigm0,
        tpemimc3::Eeigm0,
        Tpemimc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tpemimc3::Eeigm0,
            tpemimc3::Eeigm0,
            Tpemimc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA0 Error Interrupt Core Mapping"]
    #[inline(always)]
    pub fn eeicm0(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Tpemimc3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Tpemimc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ETHA0 Status Interrupt Mapping"]
    #[inline(always)]
    pub fn esim0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tpemimc3::Esim0,
        tpemimc3::Esim0,
        Tpemimc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tpemimc3::Esim0,
            tpemimc3::Esim0,
            Tpemimc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA0 Status Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn esigm0(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        tpemimc3::Esigm0,
        tpemimc3::Esigm0,
        Tpemimc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tpemimc3::Esigm0,
            tpemimc3::Esigm0,
            Tpemimc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA0 Status Interrupt Core Mapping"]
    #[inline(always)]
    pub fn esicm0(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Tpemimc3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Tpemimc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RMAC0 Status Interrupt Mapping"]
    #[inline(always)]
    pub fn rsim0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        tpemimc3::Rsim0,
        tpemimc3::Rsim0,
        Tpemimc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            tpemimc3::Rsim0,
            tpemimc3::Rsim0,
            Tpemimc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RMAC0 Status Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn rsigm0(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        tpemimc3::Rsigm0,
        tpemimc3::Rsigm0,
        Tpemimc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            tpemimc3::Rsigm0,
            tpemimc3::Rsigm0,
            Tpemimc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RMAC0 Status Interrupt Core Mapping"]
    #[inline(always)]
    pub fn rsicm0(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Tpemimc3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Tpemimc3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpemimc3 {
    #[inline(always)]
    fn default() -> Tpemimc3 {
        <crate::RegValueT<Tpemimc3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tpemimc3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eeim0_SPEC;
    pub type Eeim0 = crate::EnumBitfieldStruct<u8, Eeim0_SPEC>;
    impl Eeim0 {
        #[doc = "ETHA0 Error Interrupt is mapped to race_ etha0_error_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "ETHA0 Error Interrupt is mapped to a specific core depending on TPEMIMC3.EEIGM0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eeigm0_SPEC;
    pub type Eeigm0 = crate::EnumBitfieldStruct<u8, Eeigm0_SPEC>;
    impl Eeigm0 {
        #[doc = "When TPEMIMC3.EEIM0 is set to 1, ETHA0 Error Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC3.EEICM0\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esim0_SPEC;
    pub type Esim0 = crate::EnumBitfieldStruct<u8, Esim0_SPEC>;
    impl Esim0 {
        #[doc = "ETHA0 Status Interrupt is mapped to race_etha0_status_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "ETHA0 Status Interrupt is mapped to a specific core depending on TPEMIMC3.ESIGM0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esigm0_SPEC;
    pub type Esigm0 = crate::EnumBitfieldStruct<u8, Esigm0_SPEC>;
    impl Esigm0 {
        #[doc = "When TPEMIMC3.ESIM0 is set to 1, ETHA0 Status Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC3.ESICM0\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsim0_SPEC;
    pub type Rsim0 = crate::EnumBitfieldStruct<u8, Rsim0_SPEC>;
    impl Rsim0 {
        #[doc = "RMAC0 Status Interrupt is mapped to race_rmac0_status_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "RMAC0 Status Interrupt is mapped to a specific core depending on TPEMIMC3.RSIGM0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsigm0_SPEC;
    pub type Rsigm0 = crate::EnumBitfieldStruct<u8, Rsigm0_SPEC>;
    impl Rsigm0 {
        #[doc = "When TPEMIMC3.RSIM0 is set to 1, RMAC0 Status Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC3.RSICM0\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpemimc4_SPEC;
impl crate::sealed::RegSpec for Tpemimc4_SPEC {
    type DataType = u32;
}

#[doc = "Error and Monitoring Interrupt Mapping Configuration Register 4"]
pub type Tpemimc4 = crate::RegValueT<Tpemimc4_SPEC>;

impl Tpemimc4 {
    #[doc = "ETHA1 Error Interrupt Mapping"]
    #[inline(always)]
    pub fn eeim1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tpemimc4::Eeim1,
        tpemimc4::Eeim1,
        Tpemimc4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tpemimc4::Eeim1,
            tpemimc4::Eeim1,
            Tpemimc4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA1 Error Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn eeigm1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tpemimc4::Eeigm1,
        tpemimc4::Eeigm1,
        Tpemimc4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tpemimc4::Eeigm1,
            tpemimc4::Eeigm1,
            Tpemimc4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA1 Error Interrupt Core Mapping"]
    #[inline(always)]
    pub fn eeicm1(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Tpemimc4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Tpemimc4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ETHA1 Status Interrupt Mapping"]
    #[inline(always)]
    pub fn esim1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tpemimc4::Esim1,
        tpemimc4::Esim1,
        Tpemimc4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tpemimc4::Esim1,
            tpemimc4::Esim1,
            Tpemimc4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA1 Status Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn esigm1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        tpemimc4::Esigm1,
        tpemimc4::Esigm1,
        Tpemimc4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tpemimc4::Esigm1,
            tpemimc4::Esigm1,
            Tpemimc4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHA1 Status Interrupt Core Mapping"]
    #[inline(always)]
    pub fn esicm1(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Tpemimc4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Tpemimc4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RMAC1 Status Interrupt Mapping"]
    #[inline(always)]
    pub fn rsim1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        tpemimc4::Rsim1,
        tpemimc4::Rsim1,
        Tpemimc4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            tpemimc4::Rsim1,
            tpemimc4::Rsim1,
            Tpemimc4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RMAC1 Status Interrupt GWCA Mapping"]
    #[inline(always)]
    pub fn rsigm1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        tpemimc4::Rsigm1,
        tpemimc4::Rsigm1,
        Tpemimc4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            tpemimc4::Rsigm1,
            tpemimc4::Rsigm1,
            Tpemimc4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RMAC1 Status Interrupt Core Mapping"]
    #[inline(always)]
    pub fn rsicm1(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Tpemimc4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Tpemimc4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpemimc4 {
    #[inline(always)]
    fn default() -> Tpemimc4 {
        <crate::RegValueT<Tpemimc4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tpemimc4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eeim1_SPEC;
    pub type Eeim1 = crate::EnumBitfieldStruct<u8, Eeim1_SPEC>;
    impl Eeim1 {
        #[doc = "ETHA1 Error Interrupt is mapped to race_ etha1_error_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "ETHA1 Error Interrupt is mapped to a specific core depending on TPEMIMC4.EEIGM1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eeigm1_SPEC;
    pub type Eeigm1 = crate::EnumBitfieldStruct<u8, Eeigm1_SPEC>;
    impl Eeigm1 {
        #[doc = "When TPEMIMC4.EEIM1 is set to 1, ETHA1 Error Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC4.EEICM1\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esim1_SPEC;
    pub type Esim1 = crate::EnumBitfieldStruct<u8, Esim1_SPEC>;
    impl Esim1 {
        #[doc = "ETHA1 Status Interrupt is mapped to race_ etha1_status_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "ETHA1 Status Interrupt is mapped to a specific core depending on TPEMIMC4.ESIGM1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esigm1_SPEC;
    pub type Esigm1 = crate::EnumBitfieldStruct<u8, Esigm1_SPEC>;
    impl Esigm1 {
        #[doc = "When TPEMIMC4.ESIM1 is set to 1, ETHA1 Status Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC4.ESICM1\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsim1_SPEC;
    pub type Rsim1 = crate::EnumBitfieldStruct<u8, Rsim1_SPEC>;
    impl Rsim1 {
        #[doc = "RMAC1 Status Interrupt is mapped to race_ rmac1_status_int"]
        pub const _0: Self = Self::new(0);

        #[doc = "RMAC1 Status Interrupt is mapped to a specific core depending on TPEMIMC4.RSIGM1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsigm1_SPEC;
    pub type Rsigm1 = crate::EnumBitfieldStruct<u8, Rsigm1_SPEC>;
    impl Rsigm1 {
        #[doc = "When TPEMIMC4.RSIM1 is set to 1, RMAC1 Status Interrupt is mapped to race_gwca0_core_int\\[TPEMIMC4.RSICM1\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpemimc6_SPEC;
impl crate::sealed::RegSpec for Tpemimc6_SPEC {
    type DataType = u32;
}

#[doc = "Error and Monitoring Interrupt Mapping Configuration Register 6%s"]
pub type Tpemimc6 = crate::RegValueT<Tpemimc6_SPEC>;

impl Tpemimc6 {
    #[doc = "GWCA0 Timestamp Interrupt Mapping"]
    #[inline(always)]
    pub fn gtsim0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tpemimc6::Gtsim0,
        tpemimc6::Gtsim0,
        Tpemimc6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tpemimc6::Gtsim0,
            tpemimc6::Gtsim0,
            Tpemimc6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA0 Timestamp Interrupt Core Mapping"]
    #[inline(always)]
    pub fn gtsicm0(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Tpemimc6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Tpemimc6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpemimc6 {
    #[inline(always)]
    fn default() -> Tpemimc6 {
        <crate::RegValueT<Tpemimc6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tpemimc6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gtsim0_SPEC;
    pub type Gtsim0 = crate::EnumBitfieldStruct<u8, Gtsim0_SPEC>;
    impl Gtsim0 {
        #[doc = "GWCA0 Timestamp Interrupt t is mapped to race_gwca0_timer_int\\[t\\]"]
        pub const _0: Self = Self::new(0);

        #[doc = "GWCA0 Timestamp Interrupt t is mapped to race_gwca0_core_int depending on TPEMIMC6t.GTSICM0 setting"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpemimc7_SPEC;
impl crate::sealed::RegSpec for Tpemimc7_SPEC {
    type DataType = u32;
}

#[doc = "Error and Monitoring Interrupt Mapping Configuration Register 7%s"]
pub type Tpemimc7 = crate::RegValueT<Tpemimc7_SPEC>;

impl Tpemimc7 {
    #[doc = "GWCA0 Data Interrupt Core Mapping"]
    #[inline(always)]
    pub fn gdicm0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Tpemimc7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Tpemimc7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpemimc7 {
    #[inline(always)]
    fn default() -> Tpemimc7 {
        <crate::RegValueT<Tpemimc7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsim_SPEC;
impl crate::sealed::RegSpec for Tsim_SPEC {
    type DataType = u32;
}

#[doc = "Summarized Interrupt Mirroring Register"]
pub type Tsim = crate::RegValueT<Tsim_SPEC>;

impl Tsim {
    #[doc = "MFWD Interrupt Mirroring"]
    #[inline(always)]
    pub fn fim(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tsim::Fim, tsim::Fim, Tsim_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tsim::Fim,
            tsim::Fim,
            Tsim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "COMA Interrupt Mirroring"]
    #[inline(always)]
    pub fn cim(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tsim::Cim, tsim::Cim, Tsim_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tsim::Cim,
            tsim::Cim,
            Tsim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWCA0 Interrupt Monitoring"]
    #[inline(always)]
    pub fn gim0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tsim::Gim0,
        tsim::Gim0,
        Tsim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tsim::Gim0,
            tsim::Gim0,
            Tsim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ETHA0 Interrupt Monitoring"]
    #[inline(always)]
    pub fn eim0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tsim::Eim0,
        tsim::Eim0,
        Tsim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tsim::Eim0,
            tsim::Eim0,
            Tsim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ETHA1 Interrupt Monitoring"]
    #[inline(always)]
    pub fn eim1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        tsim::Eim1,
        tsim::Eim1,
        Tsim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tsim::Eim1,
            tsim::Eim1,
            Tsim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tsim {
    #[inline(always)]
    fn default() -> Tsim {
        <crate::RegValueT<Tsim_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tsim {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fim_SPEC;
    pub type Fim = crate::EnumBitfieldStruct<u8, Fim_SPEC>;
    impl Fim {
        #[doc = "No interrupt is set in MFWD"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in MFWD"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cim_SPEC;
    pub type Cim = crate::EnumBitfieldStruct<u8, Cim_SPEC>;
    impl Cim {
        #[doc = "No interrupt is set in COMA agent"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in COMA agent"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gim0_SPEC;
    pub type Gim0 = crate::EnumBitfieldStruct<u8, Gim0_SPEC>;
    impl Gim0 {
        #[doc = "No interrupt is set in GWCA0"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eim0_SPEC;
    pub type Eim0 = crate::EnumBitfieldStruct<u8, Eim0_SPEC>;
    impl Eim0 {
        #[doc = "No interrupt is set in ETHA0"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eim1_SPEC;
    pub type Eim1 = crate::EnumBitfieldStruct<u8, Eim1_SPEC>;
    impl Eim1 {
        #[doc = "No interrupt is set in ETHA1"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfim_SPEC;
impl crate::sealed::RegSpec for Tfim_SPEC {
    type DataType = u32;
}

#[doc = "MFWD Interrupt Mirroring Register"]
pub type Tfim = crate::RegValueT<Tfim_SPEC>;

impl Tfim {
    #[doc = "FWEIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tfim::Fweisim0,
        tfim::Fweisim0,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tfim::Fweisim0,
            tfim::Fweisim0,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS1 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tfim::Fweisim1,
        tfim::Fweisim1,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tfim::Fweisim1,
            tfim::Fweisim1,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS2 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tfim::Fweisim2,
        tfim::Fweisim2,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tfim::Fweisim2,
            tfim::Fweisim2,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS3 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        tfim::Fweisim3,
        tfim::Fweisim3,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tfim::Fweisim3,
            tfim::Fweisim3,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS4 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tfim::Fweisim4,
        tfim::Fweisim4,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tfim::Fweisim4,
            tfim::Fweisim4,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS5 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        tfim::Fweisim5,
        tfim::Fweisim5,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tfim::Fweisim5,
            tfim::Fweisim5,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS6 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        tfim::Fweisim6,
        tfim::Fweisim6,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            tfim::Fweisim6,
            tfim::Fweisim6,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS7 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        tfim::Fweisim7,
        tfim::Fweisim7,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            tfim::Fweisim7,
            tfim::Fweisim7,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWEIS8 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fweisim8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tfim::Fweisim8,
        tfim::Fweisim8,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tfim::Fweisim8,
            tfim::Fweisim8,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FWMIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn fwmisim0(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        tfim::Fwmisim0,
        tfim::Fwmisim0,
        Tfim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tfim::Fwmisim0,
            tfim::Fwmisim0,
            Tfim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tfim {
    #[inline(always)]
    fn default() -> Tfim {
        <crate::RegValueT<Tfim_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tfim {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim0_SPEC;
    pub type Fweisim0 = crate::EnumBitfieldStruct<u8, Fweisim0_SPEC>;
    impl Fweisim0 {
        #[doc = "No interrupt is set in forwarding engine FWEIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim1_SPEC;
    pub type Fweisim1 = crate::EnumBitfieldStruct<u8, Fweisim1_SPEC>;
    impl Fweisim1 {
        #[doc = "No interrupt is set in forwarding engine FWEIS1 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim2_SPEC;
    pub type Fweisim2 = crate::EnumBitfieldStruct<u8, Fweisim2_SPEC>;
    impl Fweisim2 {
        #[doc = "No interrupt is set in forwarding engine FWEIS2 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS2 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim3_SPEC;
    pub type Fweisim3 = crate::EnumBitfieldStruct<u8, Fweisim3_SPEC>;
    impl Fweisim3 {
        #[doc = "No interrupt is set in forwarding engine FWEIS3 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS3 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim4_SPEC;
    pub type Fweisim4 = crate::EnumBitfieldStruct<u8, Fweisim4_SPEC>;
    impl Fweisim4 {
        #[doc = "No interrupt is set in forwarding engine FWEIS4 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS4 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim5_SPEC;
    pub type Fweisim5 = crate::EnumBitfieldStruct<u8, Fweisim5_SPEC>;
    impl Fweisim5 {
        #[doc = "No interrupt is set in forwarding engine FWEIS5 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS5 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim6_SPEC;
    pub type Fweisim6 = crate::EnumBitfieldStruct<u8, Fweisim6_SPEC>;
    impl Fweisim6 {
        #[doc = "No interrupt is set in forwarding engine FWEIS6 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS6 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim7_SPEC;
    pub type Fweisim7 = crate::EnumBitfieldStruct<u8, Fweisim7_SPEC>;
    impl Fweisim7 {
        #[doc = "No interrupt is set in forwarding engine FWEIS7 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS7 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fweisim8_SPEC;
    pub type Fweisim8 = crate::EnumBitfieldStruct<u8, Fweisim8_SPEC>;
    impl Fweisim8 {
        #[doc = "No interrupt is set in forwarding engine FWEIS8 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWEIS8 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fwmisim0_SPEC;
    pub type Fwmisim0 = crate::EnumBitfieldStruct<u8, Fwmisim0_SPEC>;
    impl Fwmisim0 {
        #[doc = "No interrupt is set in forwarding engine FWMIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in forwarding engine FWMIS0 register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcim_SPEC;
impl crate::sealed::RegSpec for Tcim_SPEC {
    type DataType = u32;
}

#[doc = "COMA Interrupt Mirroring Register"]
pub type Tcim = crate::RegValueT<Tcim_SPEC>;

impl Tcim {
    #[doc = "CAEIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn caeisim0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tcim::Caeisim0,
        tcim::Caeisim0,
        Tcim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tcim::Caeisim0,
            tcim::Caeisim0,
            Tcim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CAEIS1 Interrupt Mirroring"]
    #[inline(always)]
    pub fn caeisim1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tcim::Caeisim1,
        tcim::Caeisim1,
        Tcim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tcim::Caeisim1,
            tcim::Caeisim1,
            Tcim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CAMIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn camisim0(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        tcim::Camisim0,
        tcim::Camisim0,
        Tcim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tcim::Camisim0,
            tcim::Camisim0,
            Tcim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CAMIS1 Interrupt Mirroring"]
    #[inline(always)]
    pub fn camisim1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcim::Camisim1,
        tcim::Camisim1,
        Tcim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcim::Camisim1,
            tcim::Camisim1,
            Tcim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tcim {
    #[inline(always)]
    fn default() -> Tcim {
        <crate::RegValueT<Tcim_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcim {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Caeisim0_SPEC;
    pub type Caeisim0 = crate::EnumBitfieldStruct<u8, Caeisim0_SPEC>;
    impl Caeisim0 {
        #[doc = "No interrupt is set in COMA agent CAEIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in COMA agent CAEIS0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Caeisim1_SPEC;
    pub type Caeisim1 = crate::EnumBitfieldStruct<u8, Caeisim1_SPEC>;
    impl Caeisim1 {
        #[doc = "No interrupt is set in COMA agent CAEIS1 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in COMA agent CAEIS1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Camisim0_SPEC;
    pub type Camisim0 = crate::EnumBitfieldStruct<u8, Camisim0_SPEC>;
    impl Camisim0 {
        #[doc = "No interrupt is set in COMA agent CAMIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in COMA agent CAMIS0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Camisim1_SPEC;
    pub type Camisim1 = crate::EnumBitfieldStruct<u8, Camisim1_SPEC>;
    impl Camisim1 {
        #[doc = "No interrupt is set in COMA agent CAMIS1 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in COMA agent CAMIS1 register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tgim0_SPEC;
impl crate::sealed::RegSpec for Tgim0_SPEC {
    type DataType = u32;
}

#[doc = "GWCA0 Interrupt Mirroring Register"]
pub type Tgim0 = crate::RegValueT<Tgim0_SPEC>;

impl Tgim0 {
    #[doc = "GWDIS Interrupt Mirroring"]
    #[inline(always)]
    pub fn gwdisim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tgim0::Gwdisim,
        tgim0::Gwdisim,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tgim0::Gwdisim,
            tgim0::Gwdisim,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWTSDIS Interrupt Mirroring"]
    #[inline(always)]
    pub fn gwtsdisim(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tgim0::Gwtsdisim,
        tgim0::Gwtsdisim,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tgim0::Gwtsdisim,
            tgim0::Gwtsdisim,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWEIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn gweisim0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tgim0::Gweisim0,
        tgim0::Gweisim0,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tgim0::Gweisim0,
            tgim0::Gweisim0,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWEIS1 Interrupt Mirroring"]
    #[inline(always)]
    pub fn gweisim1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        tgim0::Gweisim1,
        tgim0::Gweisim1,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tgim0::Gweisim1,
            tgim0::Gweisim1,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWEIS2 Interrupt Mirroring"]
    #[inline(always)]
    pub fn gweisim2(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tgim0::Gweisim2,
        tgim0::Gweisim2,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tgim0::Gweisim2,
            tgim0::Gweisim2,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWEIS3 Interrupt Mirroring"]
    #[inline(always)]
    pub fn gweisim3(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        tgim0::Gweisim3,
        tgim0::Gweisim3,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tgim0::Gweisim3,
            tgim0::Gweisim3,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWEIS4 Interrupt Mirroring"]
    #[inline(always)]
    pub fn gweisim4(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        tgim0::Gweisim4,
        tgim0::Gweisim4,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            tgim0::Gweisim4,
            tgim0::Gweisim4,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "GWEIS5 Interrupt Mirroring"]
    #[inline(always)]
    pub fn gweisim5(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        tgim0::Gweisim5,
        tgim0::Gweisim5,
        Tgim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            tgim0::Gweisim5,
            tgim0::Gweisim5,
            Tgim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tgim0 {
    #[inline(always)]
    fn default() -> Tgim0 {
        <crate::RegValueT<Tgim0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tgim0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gwdisim_SPEC;
    pub type Gwdisim = crate::EnumBitfieldStruct<u8, Gwdisim_SPEC>;
    impl Gwdisim {
        #[doc = "No interrupt is set in GWCA0 GWDIS register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWDIS register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gwtsdisim_SPEC;
    pub type Gwtsdisim = crate::EnumBitfieldStruct<u8, Gwtsdisim_SPEC>;
    impl Gwtsdisim {
        #[doc = "No interrupt is set in GWCA0 GWTSDIS register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWTSDIS register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gweisim0_SPEC;
    pub type Gweisim0 = crate::EnumBitfieldStruct<u8, Gweisim0_SPEC>;
    impl Gweisim0 {
        #[doc = "No interrupt is set in GWCA0 GWEIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWEIS0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gweisim1_SPEC;
    pub type Gweisim1 = crate::EnumBitfieldStruct<u8, Gweisim1_SPEC>;
    impl Gweisim1 {
        #[doc = "No interrupt is set in GWCA0 GWEIS1 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWEIS1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gweisim2_SPEC;
    pub type Gweisim2 = crate::EnumBitfieldStruct<u8, Gweisim2_SPEC>;
    impl Gweisim2 {
        #[doc = "No interrupt is set in GWCA0 GWEIS2 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWEIS2 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gweisim3_SPEC;
    pub type Gweisim3 = crate::EnumBitfieldStruct<u8, Gweisim3_SPEC>;
    impl Gweisim3 {
        #[doc = "No interrupt is set in GWCA0 GWEIS3 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWEIS3 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gweisim4_SPEC;
    pub type Gweisim4 = crate::EnumBitfieldStruct<u8, Gweisim4_SPEC>;
    impl Gweisim4 {
        #[doc = "No interrupt is set in GWCA0 GWEIS4 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWEIS4 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gweisim5_SPEC;
    pub type Gweisim5 = crate::EnumBitfieldStruct<u8, Gweisim5_SPEC>;
    impl Gweisim5 {
        #[doc = "No interrupt is set in GWCA0 GWEIS5 register."]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in GWCA0 GWEIS5 register."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Teim0_SPEC;
impl crate::sealed::RegSpec for Teim0_SPEC {
    type DataType = u32;
}

#[doc = "ETHA0 Interrupt Mirroring Register"]
pub type Teim0 = crate::RegValueT<Teim0_SPEC>;

impl Teim0 {
    #[doc = "EAEIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn eaeisim0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        teim0::Eaeisim0,
        teim0::Eaeisim0,
        Teim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            teim0::Eaeisim0,
            teim0::Eaeisim0,
            Teim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EAEIS1 Interrupt Mirroring"]
    #[inline(always)]
    pub fn eaeisim1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        teim0::Eaeisim1,
        teim0::Eaeisim1,
        Teim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            teim0::Eaeisim1,
            teim0::Eaeisim1,
            Teim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EAEIS2 Interrupt Mirroring"]
    #[inline(always)]
    pub fn eaeisim2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        teim0::Eaeisim2,
        teim0::Eaeisim2,
        Teim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            teim0::Eaeisim2,
            teim0::Eaeisim2,
            Teim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MEIS Interrupt Mirroring"]
    #[inline(always)]
    pub fn meisim(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        teim0::Meisim,
        teim0::Meisim,
        Teim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            teim0::Meisim,
            teim0::Meisim,
            Teim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MMIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn mmisim(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        teim0::Mmisim,
        teim0::Mmisim,
        Teim0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            teim0::Mmisim,
            teim0::Mmisim,
            Teim0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Teim0 {
    #[inline(always)]
    fn default() -> Teim0 {
        <crate::RegValueT<Teim0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod teim0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eaeisim0_SPEC;
    pub type Eaeisim0 = crate::EnumBitfieldStruct<u8, Eaeisim0_SPEC>;
    impl Eaeisim0 {
        #[doc = "No interrupt is set in ETHA0 EAEIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA0 EAEIS0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eaeisim1_SPEC;
    pub type Eaeisim1 = crate::EnumBitfieldStruct<u8, Eaeisim1_SPEC>;
    impl Eaeisim1 {
        #[doc = "No interrupt is set in ETHA0 EAEIS1 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA0 EAEIS1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eaeisim2_SPEC;
    pub type Eaeisim2 = crate::EnumBitfieldStruct<u8, Eaeisim2_SPEC>;
    impl Eaeisim2 {
        #[doc = "No interrupt is set in ETHA0 EAEIS2 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA0 EAEIS2 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Meisim_SPEC;
    pub type Meisim = crate::EnumBitfieldStruct<u8, Meisim_SPEC>;
    impl Meisim {
        #[doc = "No interrupt is set in ETHA0 RMAC MEIS register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA0 RMAC MEIS register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmisim_SPEC;
    pub type Mmisim = crate::EnumBitfieldStruct<u8, Mmisim_SPEC>;
    impl Mmisim {
        #[doc = "No interrupt is set in ETHA0 RMAC MMIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA0 RMAC MMIS0 register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Teim1_SPEC;
impl crate::sealed::RegSpec for Teim1_SPEC {
    type DataType = u32;
}

#[doc = "ETHA1 Interrupt Mirroring Register"]
pub type Teim1 = crate::RegValueT<Teim1_SPEC>;

impl Teim1 {
    #[doc = "EAEIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn eaeisim0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        teim1::Eaeisim0,
        teim1::Eaeisim0,
        Teim1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            teim1::Eaeisim0,
            teim1::Eaeisim0,
            Teim1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EAEIS1 Interrupt Mirroring"]
    #[inline(always)]
    pub fn eaeisim1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        teim1::Eaeisim1,
        teim1::Eaeisim1,
        Teim1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            teim1::Eaeisim1,
            teim1::Eaeisim1,
            Teim1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EAEIS2 Interrupt Mirroring"]
    #[inline(always)]
    pub fn eaeisim2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        teim1::Eaeisim2,
        teim1::Eaeisim2,
        Teim1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            teim1::Eaeisim2,
            teim1::Eaeisim2,
            Teim1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MEIS Interrupt Mirroring"]
    #[inline(always)]
    pub fn meisim(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        teim1::Meisim,
        teim1::Meisim,
        Teim1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            teim1::Meisim,
            teim1::Meisim,
            Teim1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MMIS0 Interrupt Mirroring"]
    #[inline(always)]
    pub fn mmisim(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        teim1::Mmisim,
        teim1::Mmisim,
        Teim1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            teim1::Mmisim,
            teim1::Mmisim,
            Teim1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Teim1 {
    #[inline(always)]
    fn default() -> Teim1 {
        <crate::RegValueT<Teim1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod teim1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eaeisim0_SPEC;
    pub type Eaeisim0 = crate::EnumBitfieldStruct<u8, Eaeisim0_SPEC>;
    impl Eaeisim0 {
        #[doc = "No interrupt is set in ETHA1 EAEIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA1 EAEIS0 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eaeisim1_SPEC;
    pub type Eaeisim1 = crate::EnumBitfieldStruct<u8, Eaeisim1_SPEC>;
    impl Eaeisim1 {
        #[doc = "No interrupt is set in ETHA1 EAEIS1 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA1 EAEIS1 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eaeisim2_SPEC;
    pub type Eaeisim2 = crate::EnumBitfieldStruct<u8, Eaeisim2_SPEC>;
    impl Eaeisim2 {
        #[doc = "No interrupt is set in ETHA1 EAEIS2 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA1 EAEIS2 register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Meisim_SPEC;
    pub type Meisim = crate::EnumBitfieldStruct<u8, Meisim_SPEC>;
    impl Meisim {
        #[doc = "No interrupt is set in ETHA1 RMAC MEIS register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA1 RMAC MEIS register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmisim_SPEC;
    pub type Mmisim = crate::EnumBitfieldStruct<u8, Mmisim_SPEC>;
    impl Mmisim {
        #[doc = "No interrupt is set in ETHA1 RMAC MMIS0 register"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt is set in ETHA1 RMAC MMIS0 register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miirr_SPEC;
impl crate::sealed::RegSpec for Miirr_SPEC {
    type DataType = u32;
}

#[doc = "Media-independent Interface Reset Register"]
pub type Miirr = crate::RegValueT<Miirr_SPEC>;

impl Miirr {
    #[doc = "RGMII0 Reset"]
    #[inline(always)]
    pub fn rgrst0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        miirr::Rgrst0,
        miirr::Rgrst0,
        Miirr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            miirr::Rgrst0,
            miirr::Rgrst0,
            Miirr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RGMII1 Reset"]
    #[inline(always)]
    pub fn rgrst1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        miirr::Rgrst1,
        miirr::Rgrst1,
        Miirr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            miirr::Rgrst1,
            miirr::Rgrst1,
            Miirr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RMII0 Reset"]
    #[inline(always)]
    pub fn rmrst0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        miirr::Rmrst0,
        miirr::Rmrst0,
        Miirr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            miirr::Rmrst0,
            miirr::Rmrst0,
            Miirr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RMII1 Reset"]
    #[inline(always)]
    pub fn rmrst1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        miirr::Rmrst1,
        miirr::Rmrst1,
        Miirr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            miirr::Rmrst1,
            miirr::Rmrst1,
            Miirr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Miirr {
    #[inline(always)]
    fn default() -> Miirr {
        <crate::RegValueT<Miirr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod miirr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rgrst0_SPEC;
    pub type Rgrst0 = crate::EnumBitfieldStruct<u8, Rgrst0_SPEC>;
    impl Rgrst0 {
        #[doc = "Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rgrst1_SPEC;
    pub type Rgrst1 = crate::EnumBitfieldStruct<u8, Rgrst1_SPEC>;
    impl Rgrst1 {
        #[doc = "Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmrst0_SPEC;
    pub type Rmrst0 = crate::EnumBitfieldStruct<u8, Rmrst0_SPEC>;
    impl Rmrst0 {
        #[doc = "Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmrst1_SPEC;
    pub type Rmrst1 = crate::EnumBitfieldStruct<u8, Rmrst1_SPEC>;
    impl Rmrst1 {
        #[doc = "Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miicr0_SPEC;
impl crate::sealed::RegSpec for Miicr0_SPEC {
    type DataType = u32;
}

#[doc = "Media-independent Interface Control Register 0"]
pub type Miicr0 = crate::RegValueT<Miicr0_SPEC>;

impl Miicr0 {
    #[doc = "Media-independent Interface Select"]
    #[inline(always)]
    pub fn miisel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        miicr0::Miisel,
        miicr0::Miisel,
        Miicr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            miicr0::Miisel,
            miicr0::Miisel,
            Miicr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Divider Stop"]
    #[inline(always)]
    pub fn divstp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        miicr0::Divstp,
        miicr0::Divstp,
        Miicr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            miicr0::Divstp,
            miicr0::Divstp,
            Miicr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RGMII TXC Internal Delay Enable"]
    #[inline(always)]
    pub fn txcide(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        miicr0::Txcide,
        miicr0::Txcide,
        Miicr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            miicr0::Txcide,
            miicr0::Txcide,
            Miicr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Miicr0 {
    #[inline(always)]
    fn default() -> Miicr0 {
        <crate::RegValueT<Miicr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod miicr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Miisel_SPEC;
    pub type Miisel = crate::EnumBitfieldStruct<u8, Miisel_SPEC>;
    impl Miisel {
        #[doc = "MII/GMII"]
        pub const _00: Self = Self::new(0);

        #[doc = "RGMII"]
        pub const _01: Self = Self::new(1);

        #[doc = "RMII"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Divstp_SPEC;
    pub type Divstp = crate::EnumBitfieldStruct<u8, Divstp_SPEC>;
    impl Divstp {
        #[doc = "Enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txcide_SPEC;
    pub type Txcide = crate::EnumBitfieldStruct<u8, Txcide_SPEC>;
    impl Txcide {
        #[doc = "TXC internal delay disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "TXC internal delay enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miicr1_SPEC;
impl crate::sealed::RegSpec for Miicr1_SPEC {
    type DataType = u32;
}

#[doc = "Media-independent Interface Control Register 1"]
pub type Miicr1 = crate::RegValueT<Miicr1_SPEC>;

impl Miicr1 {
    #[doc = "Media-independent Interface Select"]
    #[inline(always)]
    pub fn miisel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        miicr1::Miisel,
        miicr1::Miisel,
        Miicr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            miicr1::Miisel,
            miicr1::Miisel,
            Miicr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Divider Stop"]
    #[inline(always)]
    pub fn divstp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        miicr1::Divstp,
        miicr1::Divstp,
        Miicr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            miicr1::Divstp,
            miicr1::Divstp,
            Miicr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RGMII TXC Internal Delay Enable"]
    #[inline(always)]
    pub fn txcide(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        miicr1::Txcide,
        miicr1::Txcide,
        Miicr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            miicr1::Txcide,
            miicr1::Txcide,
            Miicr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Miicr1 {
    #[inline(always)]
    fn default() -> Miicr1 {
        <crate::RegValueT<Miicr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod miicr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Miisel_SPEC;
    pub type Miisel = crate::EnumBitfieldStruct<u8, Miisel_SPEC>;
    impl Miisel {
        #[doc = "MII/GMII"]
        pub const _00: Self = Self::new(0);

        #[doc = "RGMII"]
        pub const _01: Self = Self::new(1);

        #[doc = "RMII"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Divstp_SPEC;
    pub type Divstp = crate::EnumBitfieldStruct<u8, Divstp_SPEC>;
    impl Divstp {
        #[doc = "Enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txcide_SPEC;
    pub type Txcide = crate::EnumBitfieldStruct<u8, Txcide_SPEC>;
    impl Txcide {
        #[doc = "TXC Internal delay disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "TXC Internal delay enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mccesr_SPEC;
impl crate::sealed::RegSpec for Mccesr_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock Capture Event Select Register"]
pub type Mccesr = crate::RegValueT<Mccesr_SPEC>;

impl Mccesr {
    #[doc = "Media Clock 0 Capture Event Select"]
    #[inline(always)]
    pub fn mcces0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mccesr::Mcces0,
        mccesr::Mcces0,
        Mccesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mccesr::Mcces0,
            mccesr::Mcces0,
            Mccesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock 1 Capture Event Select"]
    #[inline(always)]
    pub fn mcces1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mccesr::Mcces1,
        mccesr::Mcces1,
        Mccesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mccesr::Mcces1,
            mccesr::Mcces1,
            Mccesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mccesr {
    #[inline(always)]
    fn default() -> Mccesr {
        <crate::RegValueT<Mccesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mccesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcces0_SPEC;
    pub type Mcces0 = crate::EnumBitfieldStruct<u8, Mcces0_SPEC>;
    impl Mcces0 {
        #[doc = "I/O Port"]
        pub const _0: Self = Self::new(0);

        #[doc = "ELC"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcces1_SPEC;
    pub type Mcces1 = crate::EnumBitfieldStruct<u8, Mcces1_SPEC>;
    impl Mcces1 {
        #[doc = "I/O Port"]
        pub const _0: Self = Self::new(0);

        #[doc = "ELC"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tasstsr_SPEC;
impl crate::sealed::RegSpec for Tasstsr_SPEC {
    type DataType = u32;
}

#[doc = "TAS Status Monitor Signal Select Register"]
pub type Tasstsr = crate::RegValueT<Tasstsr_SPEC>;

impl Tasstsr {
    #[doc = "ET_TAS_STA0 Monitor Signal Select"]
    #[inline(always)]
    pub fn mss0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Tasstsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Tasstsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ET_TAS_STA1 Monitor Signal Select"]
    #[inline(always)]
    pub fn mss1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Tasstsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Tasstsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ET_TAS_STA2 Monitor Signal Select"]
    #[inline(always)]
    pub fn mss2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Tasstsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Tasstsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ET_TAS_STA3 Monitor Signal Select"]
    #[inline(always)]
    pub fn mss3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Tasstsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Tasstsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tasstsr {
    #[inline(always)]
    fn default() -> Tasstsr {
        <crate::RegValueT<Tasstsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
