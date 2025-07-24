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
// Generated from SVD 1.40.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:37 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ICU for CPU"]
unsafe impl ::core::marker::Send for super::Icu {}
unsafe impl ::core::marker::Sync for super::Icu {}
impl super::Icu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "IRQ Control Register %s"]
    #[inline(always)]
    pub const fn irqcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Irqcr_SPEC, crate::common::RW>,
        8,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn irqcr0(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr1(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr2(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr3(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr4(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr5(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr6(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr7(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7usize),
            )
        }
    }

    #[doc = "NMI Pin Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmicr(&self) -> &'static crate::common::Reg<self::Nmicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(&self) -> &'static crate::common::Reg<self::Nmier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn nmiclr(
        &self,
    ) -> &'static crate::common::Reg<self::Nmiclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmiclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(&self) -> &'static crate::common::Reg<self::Nmisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nmisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "Wake Up Interrupt Enable Register"]
    #[inline(always)]
    pub const fn wupen(&self) -> &'static crate::common::Reg<self::Wupen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wupen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "ICU event Enable Register"]
    #[inline(always)]
    pub const fn ielen(&self) -> &'static crate::common::Reg<self::Ielen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(448usize),
            )
        }
    }

    #[doc = "SYS Event Link Setting Register"]
    #[inline(always)]
    pub const fn selsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Selsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Selsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "ICU Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn ielsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ielsr_SPEC, crate::common::RW>,
        32,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
    #[inline(always)]
    pub const fn ielsr0(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr1(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr2(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr3(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr4(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr5(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr6(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr7(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr8(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr9(&self) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr10(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr11(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr12(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr13(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr14(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr15(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr16(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr17(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr18(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr19(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr20(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr21(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr22(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr23(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr24(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr25(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr26(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr27(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr28(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr29(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr30(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr31(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqcr_SPEC;
impl crate::sealed::RegSpec for Irqcr_SPEC {
    type DataType = u8;
}

#[doc = "IRQ Control Register %s"]
pub type Irqcr = crate::RegValueT<Irqcr_SPEC>;

impl Irqcr {
    #[doc = "IRQi Detection Sense Select"]
    #[inline(always)]
    pub fn irqmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        irqcr::Irqmd,
        irqcr::Irqmd,
        Irqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            irqcr::Irqmd,
            irqcr::Irqmd,
            Irqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQi Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn fclksel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        irqcr::Fclksel,
        irqcr::Fclksel,
        Irqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            irqcr::Fclksel,
            irqcr::Fclksel,
            Irqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQi Digital Filter Enable"]
    #[inline(always)]
    pub fn flten(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        irqcr::Flten,
        irqcr::Flten,
        Irqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            irqcr::Flten,
            irqcr::Flten,
            Irqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Irqcr {
    #[inline(always)]
    fn default() -> Irqcr {
        <crate::RegValueT<Irqcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irqcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqmd_SPEC;
    pub type Irqmd = crate::EnumBitfieldStruct<u8, Irqmd_SPEC>;
    impl Irqmd {
        #[doc = "Falling edge"]
        pub const _00: Self = Self::new(0);

        #[doc = "Rising edge"]
        pub const _01: Self = Self::new(1);

        #[doc = "Rising and falling edges"]
        pub const _10: Self = Self::new(2);

        #[doc = "Low level"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fclksel_SPEC;
    pub type Fclksel = crate::EnumBitfieldStruct<u8, Fclksel_SPEC>;
    impl Fclksel {
        #[doc = "PCLKB"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLKB/8"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLKB/32"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLKB/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flten_SPEC;
    pub type Flten = crate::EnumBitfieldStruct<u8, Flten_SPEC>;
    impl Flten {
        #[doc = "Digital filter is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Digital filter is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmicr_SPEC;
impl crate::sealed::RegSpec for Nmicr_SPEC {
    type DataType = u8;
}

#[doc = "NMI Pin Interrupt Control Register"]
pub type Nmicr = crate::RegValueT<Nmicr_SPEC>;

impl Nmicr {
    #[doc = "NMI Detection Set"]
    #[inline(always)]
    pub fn nmimd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmicr::Nmimd,
        nmicr::Nmimd,
        Nmicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmicr::Nmimd,
            nmicr::Nmimd,
            Nmicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn nfclksel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        nmicr::Nfclksel,
        nmicr::Nfclksel,
        Nmicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            nmicr::Nfclksel,
            nmicr::Nfclksel,
            Nmicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Digital Filter Enable"]
    #[inline(always)]
    pub fn nflten(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nmicr::Nflten,
        nmicr::Nflten,
        Nmicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmicr::Nflten,
            nmicr::Nflten,
            Nmicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmicr {
    #[inline(always)]
    fn default() -> Nmicr {
        <crate::RegValueT<Nmicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmimd_SPEC;
    pub type Nmimd = crate::EnumBitfieldStruct<u8, Nmimd_SPEC>;
    impl Nmimd {
        #[doc = "Falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfclksel_SPEC;
    pub type Nfclksel = crate::EnumBitfieldStruct<u8, Nfclksel_SPEC>;
    impl Nfclksel {
        #[doc = "PCLKB"]
        pub const _00: Self = Self::new(0);

        #[doc = "PCLKB/8"]
        pub const _01: Self = Self::new(1);

        #[doc = "PCLKB/32"]
        pub const _10: Self = Self::new(2);

        #[doc = "PCLKB/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nflten_SPEC;
    pub type Nflten = crate::EnumBitfieldStruct<u8, Nflten_SPEC>;
    impl Nflten {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmier_SPEC;
impl crate::sealed::RegSpec for Nmier_SPEC {
    type DataType = u16;
}

#[doc = "Non-Maskable Interrupt Enable Register"]
pub type Nmier = crate::RegValueT<Nmier_SPEC>;

impl Nmier {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn iwdten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmier::Iwdten,
        nmier::Iwdten,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmier::Iwdten,
            nmier::Iwdten,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "WDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn wdten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        nmier::Wdten,
        nmier::Wdten,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            nmier::Wdten,
            nmier::Wdten,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage monitor 1 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd1en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmier::Lvd1En,
        nmier::Lvd1En,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmier::Lvd1En,
            nmier::Lvd1En,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage monitor 2 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd2en(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nmier::Lvd2En,
        nmier::Lvd2En,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nmier::Lvd2En,
            nmier::Lvd2En,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Interrupt Enable"]
    #[inline(always)]
    pub fn nmien(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nmier::Nmien,
        nmier::Nmien,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmier::Nmien,
            nmier::Nmien,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn rpeen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nmier::Rpeen,
        nmier::Rpeen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nmier::Rpeen,
            nmier::Rpeen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Slave MPU Error Interrupt Enable"]
    #[inline(always)]
    pub fn bussen(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        nmier::Bussen,
        nmier::Bussen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            nmier::Bussen,
            nmier::Bussen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Master MPU Error Interrupt Enable"]
    #[inline(always)]
    pub fn busmen(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        nmier::Busmen,
        nmier::Busmen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            nmier::Busmen,
            nmier::Busmen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Stack Pointer Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn speen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        nmier::Speen,
        nmier::Speen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            nmier::Speen,
            nmier::Speen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmier {
    #[inline(always)]
    fn default() -> Nmier {
        <crate::RegValueT<Nmier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdten_SPEC;
    pub type Iwdten = crate::EnumBitfieldStruct<u8, Iwdten_SPEC>;
    impl Iwdten {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdten_SPEC;
    pub type Wdten = crate::EnumBitfieldStruct<u8, Wdten_SPEC>;
    impl Wdten {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1En_SPEC;
    pub type Lvd1En = crate::EnumBitfieldStruct<u8, Lvd1En_SPEC>;
    impl Lvd1En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2En_SPEC;
    pub type Lvd2En = crate::EnumBitfieldStruct<u8, Lvd2En_SPEC>;
    impl Lvd2En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmien_SPEC;
    pub type Nmien = crate::EnumBitfieldStruct<u8, Nmien_SPEC>;
    impl Nmien {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpeen_SPEC;
    pub type Rpeen = crate::EnumBitfieldStruct<u8, Rpeen_SPEC>;
    impl Rpeen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussen_SPEC;
    pub type Bussen = crate::EnumBitfieldStruct<u8, Bussen_SPEC>;
    impl Bussen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmen_SPEC;
    pub type Busmen = crate::EnumBitfieldStruct<u8, Busmen_SPEC>;
    impl Busmen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Speen_SPEC;
    pub type Speen = crate::EnumBitfieldStruct<u8, Speen_SPEC>;
    impl Speen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmiclr_SPEC;
impl crate::sealed::RegSpec for Nmiclr_SPEC {
    type DataType = u16;
}

#[doc = "Non-Maskable Interrupt Status Clear Register"]
pub type Nmiclr = crate::RegValueT<Nmiclr_SPEC>;

impl Nmiclr {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn iwdtclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmiclr::Iwdtclr,
        nmiclr::Iwdtclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmiclr::Iwdtclr,
            nmiclr::Iwdtclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "WDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn wdtclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        nmiclr::Wdtclr,
        nmiclr::Wdtclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            nmiclr::Wdtclr,
            nmiclr::Wdtclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd1clr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmiclr::Lvd1Clr,
        nmiclr::Lvd1Clr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmiclr::Lvd1Clr,
            nmiclr::Lvd1Clr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd2clr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nmiclr::Lvd2Clr,
        nmiclr::Lvd2Clr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nmiclr::Lvd2Clr,
            nmiclr::Lvd2Clr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn nmiclr(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nmiclr::Nmiclr,
        nmiclr::Nmiclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmiclr::Nmiclr,
            nmiclr::Nmiclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM Parity Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn rpeclr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nmiclr::Rpeclr,
        nmiclr::Rpeclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nmiclr::Rpeclr,
            nmiclr::Rpeclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Slave MPU Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn bussclr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        nmiclr::Bussclr,
        nmiclr::Bussclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            nmiclr::Bussclr,
            nmiclr::Bussclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Master MPU Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn busmclr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        nmiclr::Busmclr,
        nmiclr::Busmclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            nmiclr::Busmclr,
            nmiclr::Busmclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Stack Pointer Monitor Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn speclr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        nmiclr::Speclr,
        nmiclr::Speclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            nmiclr::Speclr,
            nmiclr::Speclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmiclr {
    #[inline(always)]
    fn default() -> Nmiclr {
        <crate::RegValueT<Nmiclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmiclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtclr_SPEC;
    pub type Iwdtclr = crate::EnumBitfieldStruct<u8, Iwdtclr_SPEC>;
    impl Iwdtclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.IWDTST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtclr_SPEC;
    pub type Wdtclr = crate::EnumBitfieldStruct<u8, Wdtclr_SPEC>;
    impl Wdtclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.WDTST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Clr_SPEC;
    pub type Lvd1Clr = crate::EnumBitfieldStruct<u8, Lvd1Clr_SPEC>;
    impl Lvd1Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.LVD1ST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Clr_SPEC;
    pub type Lvd2Clr = crate::EnumBitfieldStruct<u8, Lvd2Clr_SPEC>;
    impl Lvd2Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.LVD2ST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmiclr_SPEC;
    pub type Nmiclr = crate::EnumBitfieldStruct<u8, Nmiclr_SPEC>;
    impl Nmiclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.NMIST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpeclr_SPEC;
    pub type Rpeclr = crate::EnumBitfieldStruct<u8, Rpeclr_SPEC>;
    impl Rpeclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.RPEST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussclr_SPEC;
    pub type Bussclr = crate::EnumBitfieldStruct<u8, Bussclr_SPEC>;
    impl Bussclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.BUSSST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmclr_SPEC;
    pub type Busmclr = crate::EnumBitfieldStruct<u8, Busmclr_SPEC>;
    impl Busmclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.BUSMST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Speclr_SPEC;
    pub type Speclr = crate::EnumBitfieldStruct<u8, Speclr_SPEC>;
    impl Speclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.SPEST flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisr_SPEC;
impl crate::sealed::RegSpec for Nmisr_SPEC {
    type DataType = u16;
}

#[doc = "Non-Maskable Interrupt Status Register"]
pub type Nmisr = crate::RegValueT<Nmisr_SPEC>;

impl Nmisr {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn iwdtst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nmisr::Iwdtst,
        nmisr::Iwdtst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmisr::Iwdtst,
            nmisr::Iwdtst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "WDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn wdtst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        nmisr::Wdtst,
        nmisr::Wdtst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            nmisr::Wdtst,
            nmisr::Wdtst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 1 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd1st(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmisr::Lvd1St,
        nmisr::Lvd1St,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmisr::Lvd1St,
            nmisr::Lvd1St,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd2st(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nmisr::Lvd2St,
        nmisr::Lvd2St,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nmisr::Lvd2St,
            nmisr::Lvd2St,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "NMI Pin Interrupt Status Flag"]
    #[inline(always)]
    pub fn nmist(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nmisr::Nmist,
        nmisr::Nmist,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmisr::Nmist,
            nmisr::Nmist,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SRAM Parity Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn rpest(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nmisr::Rpest,
        nmisr::Rpest,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nmisr::Rpest,
            nmisr::Rpest,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bus Slave MPU Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn bussst(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        nmisr::Bussst,
        nmisr::Bussst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            nmisr::Bussst,
            nmisr::Bussst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bus Master MPU Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn busmst(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        nmisr::Busmst,
        nmisr::Busmst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            nmisr::Busmst,
            nmisr::Busmst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CPU Stack Pointer Monitor Interrupt Status Flag"]
    #[inline(always)]
    pub fn spest(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        nmisr::Spest,
        nmisr::Spest,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            nmisr::Spest,
            nmisr::Spest,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nmisr {
    #[inline(always)]
    fn default() -> Nmisr {
        <crate::RegValueT<Nmisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtst_SPEC;
    pub type Iwdtst = crate::EnumBitfieldStruct<u8, Iwdtst_SPEC>;
    impl Iwdtst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtst_SPEC;
    pub type Wdtst = crate::EnumBitfieldStruct<u8, Wdtst_SPEC>;
    impl Wdtst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1St_SPEC;
    pub type Lvd1St = crate::EnumBitfieldStruct<u8, Lvd1St_SPEC>;
    impl Lvd1St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2St_SPEC;
    pub type Lvd2St = crate::EnumBitfieldStruct<u8, Lvd2St_SPEC>;
    impl Lvd2St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmist_SPEC;
    pub type Nmist = crate::EnumBitfieldStruct<u8, Nmist_SPEC>;
    impl Nmist {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpest_SPEC;
    pub type Rpest = crate::EnumBitfieldStruct<u8, Rpest_SPEC>;
    impl Rpest {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussst_SPEC;
    pub type Bussst = crate::EnumBitfieldStruct<u8, Bussst_SPEC>;
    impl Bussst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmst_SPEC;
    pub type Busmst = crate::EnumBitfieldStruct<u8, Busmst_SPEC>;
    impl Busmst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spest_SPEC;
    pub type Spest = crate::EnumBitfieldStruct<u8, Spest_SPEC>;
    impl Spest {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupen_SPEC;
impl crate::sealed::RegSpec for Wupen_SPEC {
    type DataType = u32;
}

#[doc = "Wake Up Interrupt Enable Register"]
pub type Wupen = crate::RegValueT<Wupen_SPEC>;

impl Wupen {
    #[doc = "IRQ Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn irqwupen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        wupen::Irqwupen,
        wupen::Irqwupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            wupen::Irqwupen,
            wupen::Irqwupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IWDT Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iwdtwupen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        wupen::Iwdtwupen,
        wupen::Iwdtwupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            wupen::Iwdtwupen,
            wupen::Iwdtwupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn keywupen(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        wupen::Keywupen,
        wupen::Keywupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            wupen::Keywupen,
            wupen::Keywupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD1 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvd1wupen(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        wupen::Lvd1Wupen,
        wupen::Lvd1Wupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            wupen::Lvd1Wupen,
            wupen::Lvd1Wupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvd2wupen(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        wupen::Lvd2Wupen,
        wupen::Lvd2Wupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            wupen::Lvd2Wupen,
            wupen::Lvd2Wupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt1udwupen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        wupen::Agt1Udwupen,
        wupen::Agt1Udwupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            wupen::Agt1Udwupen,
            wupen::Agt1Udwupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt1cawupen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        wupen::Agt1Cawupen,
        wupen::Agt1Cawupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            wupen::Agt1Cawupen,
            wupen::Agt1Cawupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt1cbwupen(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        wupen::Agt1Cbwupen,
        wupen::Agt1Cbwupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            wupen::Agt1Cbwupen,
            wupen::Agt1Cbwupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wupen {
    #[inline(always)]
    fn default() -> Wupen {
        <crate::RegValueT<Wupen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wupen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen_SPEC;
    pub type Irqwupen = crate::EnumBitfieldStruct<u8, Irqwupen_SPEC>;
    impl Irqwupen {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtwupen_SPEC;
    pub type Iwdtwupen = crate::EnumBitfieldStruct<u8, Iwdtwupen_SPEC>;
    impl Iwdtwupen {
        #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Keywupen_SPEC;
    pub type Keywupen = crate::EnumBitfieldStruct<u8, Keywupen_SPEC>;
    impl Keywupen {
        #[doc = "Software Standby/Snooze Mode returns by KEY interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by KEY interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Wupen_SPEC;
    pub type Lvd1Wupen = crate::EnumBitfieldStruct<u8, Lvd1Wupen_SPEC>;
    impl Lvd1Wupen {
        #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Wupen_SPEC;
    pub type Lvd2Wupen = crate::EnumBitfieldStruct<u8, Lvd2Wupen_SPEC>;
    impl Lvd2Wupen {
        #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Udwupen_SPEC;
    pub type Agt1Udwupen = crate::EnumBitfieldStruct<u8, Agt1Udwupen_SPEC>;
    impl Agt1Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cawupen_SPEC;
    pub type Agt1Cawupen = crate::EnumBitfieldStruct<u8, Agt1Cawupen_SPEC>;
    impl Agt1Cawupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cbwupen_SPEC;
    pub type Agt1Cbwupen = crate::EnumBitfieldStruct<u8, Agt1Cbwupen_SPEC>;
    impl Agt1Cbwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ielen_SPEC;
impl crate::sealed::RegSpec for Ielen_SPEC {
    type DataType = u8;
}

#[doc = "ICU event Enable Register"]
pub type Ielen = crate::RegValueT<Ielen_SPEC>;

impl Ielen {
    #[doc = "Parts Asynchronous Interrupts Enable (when LPOPTEN bit = 1)"]
    #[inline(always)]
    pub fn ielen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ielen::Ielen,
        ielen::Ielen,
        Ielen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ielen::Ielen,
            ielen::Ielen,
            Ielen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ielen {
    #[inline(always)]
    fn default() -> Ielen {
        <crate::RegValueT<Ielen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ielen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ielen_SPEC;
    pub type Ielen = crate::EnumBitfieldStruct<u8, Ielen_SPEC>;
    impl Ielen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Selsr0_SPEC;
impl crate::sealed::RegSpec for Selsr0_SPEC {
    type DataType = u16;
}

#[doc = "SYS Event Link Setting Register"]
pub type Selsr0 = crate::RegValueT<Selsr0_SPEC>;

impl NoBitfieldReg<Selsr0_SPEC> for Selsr0 {}
impl ::core::default::Default for Selsr0 {
    #[inline(always)]
    fn default() -> Selsr0 {
        <crate::RegValueT<Selsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ielsr_SPEC;
impl crate::sealed::RegSpec for Ielsr_SPEC {
    type DataType = u32;
}

#[doc = "ICU Event Link Setting Register %s"]
pub type Ielsr = crate::RegValueT<Ielsr_SPEC>;

impl NoBitfieldReg<Ielsr_SPEC> for Ielsr {}
impl ::core::default::Default for Ielsr {
    #[inline(always)]
    fn default() -> Ielsr {
        <crate::RegValueT<Ielsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
