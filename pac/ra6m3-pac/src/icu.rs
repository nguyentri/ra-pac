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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:32 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Interrupt Controller"]
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
        16,
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
    #[inline(always)]
    pub const fn irqcr8(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr9(&self) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr10(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xausize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr11(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr12(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr13(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr14(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr15(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfusize),
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
    pub const fn nmiclr(&self) -> &'static crate::common::Reg<self::Nmiclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Nmiclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
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

    #[doc = "INT Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn ielsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ielsr_SPEC, crate::common::RW>,
        96,
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
    #[inline(always)]
    pub const fn ielsr32(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x380usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr33(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x384usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr34(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x388usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr35(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr36(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x390usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr37(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x394usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr38(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x398usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr39(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x39cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr40(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr41(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr42(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr43(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr44(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr45(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr46(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr47(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr48(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr49(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr50(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr51(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr52(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr53(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr54(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr55(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr56(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr57(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr58(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr59(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr60(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr61(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr62(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr63(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3fcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr64(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr65(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr66(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr67(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr68(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr69(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x414usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr70(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr71(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x41cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr72(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr73(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr74(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr75(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr76(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr77(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x434usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr78(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x438usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr79(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x43cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr80(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr81(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr82(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr83(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr84(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr85(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x454usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr86(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x458usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr87(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x45cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr88(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr89(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr90(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr91(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x46cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr92(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x470usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr93(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x474usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr94(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x478usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ielsr95(
        &self,
    ) -> &'static crate::common::Reg<self::Ielsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ielsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x47cusize),
            )
        }
    }

    #[doc = "DMAC Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn delsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Delsr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }
    #[inline(always)]
    pub const fn delsr0(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr1(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr2(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr3(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr4(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr5(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr6(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn delsr7(&self) -> &'static crate::common::Reg<self::Delsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Delsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
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

    #[doc = "Wake Up interrupt enable register"]
    #[inline(always)]
    pub const fn wupen(&self) -> &'static crate::common::Reg<self::Wupen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wupen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
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
    #[doc = "IRQ Digital Filter Enable"]
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

    #[doc = "IRQ Digital Filter Sampling Clock"]
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

    #[doc = "IRQ Detection Sense Select"]
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
}
impl ::core::default::Default for Irqcr {
    #[inline(always)]
    fn default() -> Irqcr {
        <crate::RegValueT<Irqcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irqcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flten_SPEC;
    pub type Flten = crate::EnumBitfieldStruct<u8, Flten_SPEC>;
    impl Flten {
        #[doc = "Digital filter is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Digital filter is enabled."]
        pub const _1: Self = Self::new(1);
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
    #[doc = "MPU Stack Error Interrupt Status Flag"]
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

    #[doc = "MPU Bus Master Error Interrupt Status Flag"]
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

    #[doc = "MPU Bus Slave Error Interrupt Status Flag"]
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

    #[doc = "RAM ECC Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn reccst(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        nmisr::Reccst,
        nmisr::Reccst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            nmisr::Reccst,
            nmisr::Reccst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RAM Parity Error Interrupt Status Flag"]
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

    #[doc = "NMI Status Flag"]
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

    #[doc = "Oscillation Stop Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn ostst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        nmisr::Ostst,
        nmisr::Ostst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            nmisr::Ostst,
            nmisr::Ostst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage-Monitoring 2 Interrupt Status Flag"]
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

    #[doc = "Voltage-Monitoring 1 Interrupt Status Flag"]
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

    #[doc = "WDT Underflow/Refresh Error Status Flag"]
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

    #[doc = "IWDT Underflow/Refresh Error Status Flag"]
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
}
impl ::core::default::Default for Nmisr {
    #[inline(always)]
    fn default() -> Nmisr {
        <crate::RegValueT<Nmisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spest_SPEC;
    pub type Spest = crate::EnumBitfieldStruct<u8, Spest_SPEC>;
    impl Spest {
        #[doc = "MPU Stack Error interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "MPU Stack Error interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmst_SPEC;
    pub type Busmst = crate::EnumBitfieldStruct<u8, Busmst_SPEC>;
    impl Busmst {
        #[doc = "MPU Bus Master Error interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "MPU Bus Master Error interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussst_SPEC;
    pub type Bussst = crate::EnumBitfieldStruct<u8, Bussst_SPEC>;
    impl Bussst {
        #[doc = "MPU Bus Slave Error interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "MPU Bus Slave Error interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reccst_SPEC;
    pub type Reccst = crate::EnumBitfieldStruct<u8, Reccst_SPEC>;
    impl Reccst {
        #[doc = "RAM ECC Error interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "RAM ECC Error interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpest_SPEC;
    pub type Rpest = crate::EnumBitfieldStruct<u8, Rpest_SPEC>;
    impl Rpest {
        #[doc = "RAM Parity Error interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "RAM Parity Error interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmist_SPEC;
    pub type Nmist = crate::EnumBitfieldStruct<u8, Nmist_SPEC>;
    impl Nmist {
        #[doc = "NMI pin interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "NMI pin interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostst_SPEC;
    pub type Ostst = crate::EnumBitfieldStruct<u8, Ostst_SPEC>;
    impl Ostst {
        #[doc = "Oscillation stop detection interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillation stop detection interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2St_SPEC;
    pub type Lvd2St = crate::EnumBitfieldStruct<u8, Lvd2St_SPEC>;
    impl Lvd2St {
        #[doc = "Voltage-monitoring 2 interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage-monitoring 2 interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1St_SPEC;
    pub type Lvd1St = crate::EnumBitfieldStruct<u8, Lvd1St_SPEC>;
    impl Lvd1St {
        #[doc = "Voltage-monitoring 1 interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage-monitoring 1 interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtst_SPEC;
    pub type Wdtst = crate::EnumBitfieldStruct<u8, Wdtst_SPEC>;
    impl Wdtst {
        #[doc = "WDT underflow/refresh error interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "WDT underflow/refresh error interrupt is requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtst_SPEC;
    pub type Iwdtst = crate::EnumBitfieldStruct<u8, Iwdtst_SPEC>;
    impl Iwdtst {
        #[doc = "IWDT underflow/refresh error interrupt is not requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "IWDT underflow/refresh error interrupt is requested."]
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
    #[doc = "MPU Stack Error Interrupt Enable"]
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

    #[doc = "MPU Bus Master Error Interrupt Enable"]
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

    #[doc = "MPU Bus Slave Error Interrupt Enable"]
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

    #[doc = "RAM ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn reccen(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        nmier::Reccen,
        nmier::Reccen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            nmier::Reccen,
            nmier::Reccen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RAM Parity Error Interrupt Enable"]
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

    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn osten(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        nmier::Osten,
        nmier::Osten,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            nmier::Osten,
            nmier::Osten,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage-Monitoring 2 Interrupt Enable"]
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

    #[doc = "Voltage-Monitoring 1 Interrupt Enable"]
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
}
impl ::core::default::Default for Nmier {
    #[inline(always)]
    fn default() -> Nmier {
        <crate::RegValueT<Nmier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Speen_SPEC;
    pub type Speen = crate::EnumBitfieldStruct<u8, Speen_SPEC>;
    impl Speen {
        #[doc = "MPU Stack Error interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "MPU Stack Error interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmen_SPEC;
    pub type Busmen = crate::EnumBitfieldStruct<u8, Busmen_SPEC>;
    impl Busmen {
        #[doc = "MPU Bus Master Error interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "MPU Bus Master Error interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussen_SPEC;
    pub type Bussen = crate::EnumBitfieldStruct<u8, Bussen_SPEC>;
    impl Bussen {
        #[doc = "MPU Bus Slave Error interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "MPU Bus Slave Error interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reccen_SPEC;
    pub type Reccen = crate::EnumBitfieldStruct<u8, Reccen_SPEC>;
    impl Reccen {
        #[doc = "RAM ECC Error interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "RAM ECC Error interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpeen_SPEC;
    pub type Rpeen = crate::EnumBitfieldStruct<u8, Rpeen_SPEC>;
    impl Rpeen {
        #[doc = "RAM Parity Error interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "RAM Parity Error interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmien_SPEC;
    pub type Nmien = crate::EnumBitfieldStruct<u8, Nmien_SPEC>;
    impl Nmien {
        #[doc = "NMI pin interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "NMI pin interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Osten_SPEC;
    pub type Osten = crate::EnumBitfieldStruct<u8, Osten_SPEC>;
    impl Osten {
        #[doc = "Oscillation stop detection interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Oscillation stop detection interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2En_SPEC;
    pub type Lvd2En = crate::EnumBitfieldStruct<u8, Lvd2En_SPEC>;
    impl Lvd2En {
        #[doc = "Voltage-monitoring 2 interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage-monitoring 2 interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1En_SPEC;
    pub type Lvd1En = crate::EnumBitfieldStruct<u8, Lvd1En_SPEC>;
    impl Lvd1En {
        #[doc = "Voltage-monitoring 1 interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Voltage-monitoring 1 interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdten_SPEC;
    pub type Wdten = crate::EnumBitfieldStruct<u8, Wdten_SPEC>;
    impl Wdten {
        #[doc = "WDT underflow/refresh error interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "WDT underflow/refresh error interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdten_SPEC;
    pub type Iwdten = crate::EnumBitfieldStruct<u8, Iwdten_SPEC>;
    impl Iwdten {
        #[doc = "IWDT underflow/refresh error interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "IWDT underflow/refresh error interrupt is enabled."]
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
    #[doc = "SPEST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            nmiclr::Speclr,
            nmiclr::Speclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "BUSMST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            nmiclr::Busmclr,
            nmiclr::Busmclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "BUSSST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            nmiclr::Bussclr,
            nmiclr::Bussclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "RECCST Clear"]
    #[inline(always)]
    pub fn reccclr(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        nmiclr::Reccclr,
        nmiclr::Reccclr,
        Nmiclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            nmiclr::Reccclr,
            nmiclr::Reccclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "RPEST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nmiclr::Rpeclr,
            nmiclr::Rpeclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "NMIST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nmiclr::Nmiclr,
            nmiclr::Nmiclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "OSTST Clear"]
    #[inline(always)]
    pub fn ostclr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        nmiclr::Ostclr,
        nmiclr::Ostclr,
        Nmiclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            nmiclr::Ostclr,
            nmiclr::Ostclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "LVD2ST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nmiclr::Lvd2Clr,
            nmiclr::Lvd2Clr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "LVD1ST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmiclr::Lvd1Clr,
            nmiclr::Lvd1Clr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "WDTST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            nmiclr::Wdtclr,
            nmiclr::Wdtclr,
            Nmiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "IWDTST Clear"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nmiclr::Iwdtclr,
            nmiclr::Iwdtclr,
            Nmiclr_SPEC,
            crate::common::W,
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
    pub struct Speclr_SPEC;
    pub type Speclr = crate::EnumBitfieldStruct<u8, Speclr_SPEC>;
    impl Speclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.SPEST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmclr_SPEC;
    pub type Busmclr = crate::EnumBitfieldStruct<u8, Busmclr_SPEC>;
    impl Busmclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.BUSMST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussclr_SPEC;
    pub type Bussclr = crate::EnumBitfieldStruct<u8, Bussclr_SPEC>;
    impl Bussclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.BUSSST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reccclr_SPEC;
    pub type Reccclr = crate::EnumBitfieldStruct<u8, Reccclr_SPEC>;
    impl Reccclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.RECCST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpeclr_SPEC;
    pub type Rpeclr = crate::EnumBitfieldStruct<u8, Rpeclr_SPEC>;
    impl Rpeclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.RPEST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmiclr_SPEC;
    pub type Nmiclr = crate::EnumBitfieldStruct<u8, Nmiclr_SPEC>;
    impl Nmiclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.NMIST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostclr_SPEC;
    pub type Ostclr = crate::EnumBitfieldStruct<u8, Ostclr_SPEC>;
    impl Ostclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.OSTST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Clr_SPEC;
    pub type Lvd2Clr = crate::EnumBitfieldStruct<u8, Lvd2Clr_SPEC>;
    impl Lvd2Clr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.LVD2ST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Clr_SPEC;
    pub type Lvd1Clr = crate::EnumBitfieldStruct<u8, Lvd1Clr_SPEC>;
    impl Lvd1Clr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.LVD1ST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtclr_SPEC;
    pub type Wdtclr = crate::EnumBitfieldStruct<u8, Wdtclr_SPEC>;
    impl Wdtclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.WDTST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtclr_SPEC;
    pub type Iwdtclr = crate::EnumBitfieldStruct<u8, Iwdtclr_SPEC>;
    impl Iwdtclr {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.IWDTST flag."]
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

    #[doc = "NMI Digital Filter Sampling Clock"]
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
}
impl ::core::default::Default for Nmicr {
    #[inline(always)]
    fn default() -> Nmicr {
        <crate::RegValueT<Nmicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nflten_SPEC;
    pub type Nflten = crate::EnumBitfieldStruct<u8, Nflten_SPEC>;
    impl Nflten {
        #[doc = "Digital filter is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Digital filter is enabled."]
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
    pub struct Nmimd_SPEC;
    pub type Nmimd = crate::EnumBitfieldStruct<u8, Nmimd_SPEC>;
    impl Nmimd {
        #[doc = "Falling edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ielsr_SPEC;
impl crate::sealed::RegSpec for Ielsr_SPEC {
    type DataType = u32;
}

#[doc = "INT Event Link Setting Register %s"]
pub type Ielsr = crate::RegValueT<Ielsr_SPEC>;

impl Ielsr {
    #[doc = "DTC Activation Enable"]
    #[inline(always)]
    pub fn dtce(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ielsr::Dtce,
        ielsr::Dtce,
        Ielsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ielsr::Dtce,
            ielsr::Dtce,
            Ielsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub fn ir(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ielsr::Ir,
        ielsr::Ir,
        Ielsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ielsr::Ir,
            ielsr::Ir,
            Ielsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event selection to NVIC"]
    #[inline(always)]
    pub fn iels(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        ielsr::Iels,
        ielsr::Iels,
        Ielsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            ielsr::Iels,
            ielsr::Iels,
            Ielsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ielsr {
    #[inline(always)]
    fn default() -> Ielsr {
        <crate::RegValueT<Ielsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ielsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtce_SPEC;
    pub type Dtce = crate::EnumBitfieldStruct<u8, Dtce_SPEC>;
    impl Dtce {
        #[doc = "DTC activation is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC activation is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ir_SPEC;
    pub type Ir = crate::EnumBitfieldStruct<u8, Ir_SPEC>;
    impl Ir {
        #[doc = "No interrupt request is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt request is generated ( \"1\" write to the IR bit is prohibited. )"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iels_SPEC;
    pub type Iels = crate::EnumBitfieldStruct<u8, Iels_SPEC>;
    impl Iels {
        #[doc = "Nothing is selected"]
        pub const _0_X_000: Self = Self::new(0);

        #[doc = "See Event Table"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Delsr_SPEC;
impl crate::sealed::RegSpec for Delsr_SPEC {
    type DataType = u32;
}

#[doc = "DMAC Event Link Setting Register %s"]
pub type Delsr = crate::RegValueT<Delsr_SPEC>;

impl Delsr {
    #[doc = "Interrupt Status Flag for DMAC"]
    #[inline(always)]
    pub fn ir(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        delsr::Ir,
        delsr::Ir,
        Delsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            delsr::Ir,
            delsr::Ir,
            Delsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMAC Event Link Select"]
    #[inline(always)]
    pub fn dels(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        delsr::Dels,
        delsr::Dels,
        Delsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            delsr::Dels,
            delsr::Dels,
            Delsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Delsr {
    #[inline(always)]
    fn default() -> Delsr {
        <crate::RegValueT<Delsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod delsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ir_SPEC;
    pub type Ir = crate::EnumBitfieldStruct<u8, Ir_SPEC>;
    impl Ir {
        #[doc = "No interrupt request is generated"]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt request is generated ( \"1\" write to the IR bit is prohibited. )"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dels_SPEC;
    pub type Dels = crate::EnumBitfieldStruct<u8, Dels_SPEC>;
    impl Dels {
        #[doc = "Nothing is selected."]
        pub const _0_X_000: Self = Self::new(0);

        #[doc = "See Event Table"]
        pub const OTHERS: Self = Self::new(0);
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

impl Selsr0 {
    #[doc = "SYS Event Link Select"]
    #[inline(always)]
    pub fn sels(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        selsr0::Sels,
        selsr0::Sels,
        Selsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            selsr0::Sels,
            selsr0::Sels,
            Selsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Selsr0 {
    #[inline(always)]
    fn default() -> Selsr0 {
        <crate::RegValueT<Selsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod selsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sels_SPEC;
    pub type Sels = crate::EnumBitfieldStruct<u8, Sels_SPEC>;
    impl Sels {
        #[doc = "Disable event output to the associated low-power mode module"]
        pub const _000000000: Self = Self::new(0);

        #[doc = "Event signal number to be linked."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupen_SPEC;
impl crate::sealed::RegSpec for Wupen_SPEC {
    type DataType = u32;
}

#[doc = "Wake Up interrupt enable register"]
pub type Wupen = crate::RegValueT<Wupen_SPEC>;

impl Wupen {
    #[doc = "IIC0 address match interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn iic0wupen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        wupen::Iic0Wupen,
        wupen::Iic0Wupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            wupen::Iic0Wupen,
            wupen::Iic0Wupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 compare match B interrupt S/W standby returns enable bit"]
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

    #[doc = "AGT1 compare match A interrupt S/W standby returns enable bit"]
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

    #[doc = "AGT1 underflow interrupt S/W standby returns enable bit"]
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

    #[doc = "USBFS interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn usbfswupen(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        wupen::Usbfswupen,
        wupen::Usbfswupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            wupen::Usbfswupen,
            wupen::Usbfswupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBHS interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn usbhswupen(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        wupen::Usbhswupen,
        wupen::Usbhswupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            wupen::Usbhswupen,
            wupen::Usbhswupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RCT period interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn rtcprdwupen(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        wupen::Rtcprdwupen,
        wupen::Rtcprdwupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            wupen::Rtcprdwupen,
            wupen::Rtcprdwupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC alarm interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn rtcalmwupen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        wupen::Rtcalmwupen,
        wupen::Rtcalmwupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            wupen::Rtcalmwupen,
            wupen::Rtcalmwupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACMPHS0 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn acmphs0wupen(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        wupen::Acmphs0Wupen,
        wupen::Acmphs0Wupen,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            wupen::Acmphs0Wupen,
            wupen::Acmphs0Wupen,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVD2 interrupt S/W standby returns enable bit"]
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

    #[doc = "LVD1 interrupt S/W standby returns enable bit"]
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

    #[doc = "Key interrupt S/W standby returns enable bit"]
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

    #[doc = "IWDT interrupt S/W standby returns enable bit"]
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

    #[doc = "IRQ15 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        wupen::Irqwupen15,
        wupen::Irqwupen15,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            wupen::Irqwupen15,
            wupen::Irqwupen15,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ14 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        wupen::Irqwupen14,
        wupen::Irqwupen14,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            wupen::Irqwupen14,
            wupen::Irqwupen14,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ13 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        wupen::Irqwupen13,
        wupen::Irqwupen13,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            wupen::Irqwupen13,
            wupen::Irqwupen13,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ12 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        wupen::Irqwupen12,
        wupen::Irqwupen12,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            wupen::Irqwupen12,
            wupen::Irqwupen12,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ11 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        wupen::Irqwupen11,
        wupen::Irqwupen11,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            wupen::Irqwupen11,
            wupen::Irqwupen11,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ10 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        wupen::Irqwupen10,
        wupen::Irqwupen10,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            wupen::Irqwupen10,
            wupen::Irqwupen10,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ9 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        wupen::Irqwupen9,
        wupen::Irqwupen9,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            wupen::Irqwupen9,
            wupen::Irqwupen9,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ8 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        wupen::Irqwupen8,
        wupen::Irqwupen8,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            wupen::Irqwupen8,
            wupen::Irqwupen8,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ7 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wupen::Irqwupen7,
        wupen::Irqwupen7,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wupen::Irqwupen7,
            wupen::Irqwupen7,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ6 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        wupen::Irqwupen6,
        wupen::Irqwupen6,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            wupen::Irqwupen6,
            wupen::Irqwupen6,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ5 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        wupen::Irqwupen5,
        wupen::Irqwupen5,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            wupen::Irqwupen5,
            wupen::Irqwupen5,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ4 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        wupen::Irqwupen4,
        wupen::Irqwupen4,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            wupen::Irqwupen4,
            wupen::Irqwupen4,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ3 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        wupen::Irqwupen3,
        wupen::Irqwupen3,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            wupen::Irqwupen3,
            wupen::Irqwupen3,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ2 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        wupen::Irqwupen2,
        wupen::Irqwupen2,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            wupen::Irqwupen2,
            wupen::Irqwupen2,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ1 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        wupen::Irqwupen1,
        wupen::Irqwupen1,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            wupen::Irqwupen1,
            wupen::Irqwupen1,
            Wupen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ0 interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn irqwupen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wupen::Irqwupen0,
        wupen::Irqwupen0,
        Wupen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wupen::Irqwupen0,
            wupen::Irqwupen0,
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
    pub struct Iic0Wupen_SPEC;
    pub type Iic0Wupen = crate::EnumBitfieldStruct<u8, Iic0Wupen_SPEC>;
    impl Iic0Wupen {
        #[doc = "S/W standby returns by IIC0 address match interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IIC0 address match interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cbwupen_SPEC;
    pub type Agt1Cbwupen = crate::EnumBitfieldStruct<u8, Agt1Cbwupen_SPEC>;
    impl Agt1Cbwupen {
        #[doc = "S/W standby returns by AGT1 compare match B interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by AGT1 compare match B interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cawupen_SPEC;
    pub type Agt1Cawupen = crate::EnumBitfieldStruct<u8, Agt1Cawupen_SPEC>;
    impl Agt1Cawupen {
        #[doc = "S/W standby returns by AGT1 compare match A interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by AGT1 compare match A interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Udwupen_SPEC;
    pub type Agt1Udwupen = crate::EnumBitfieldStruct<u8, Agt1Udwupen_SPEC>;
    impl Agt1Udwupen {
        #[doc = "S/W standby returns by AGT1 underflow interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by AGT1 underflow interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbfswupen_SPEC;
    pub type Usbfswupen = crate::EnumBitfieldStruct<u8, Usbfswupen_SPEC>;
    impl Usbfswupen {
        #[doc = "S/W standby returns by USBFS interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by USBFS interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbhswupen_SPEC;
    pub type Usbhswupen = crate::EnumBitfieldStruct<u8, Usbhswupen_SPEC>;
    impl Usbhswupen {
        #[doc = "S/W standby returns by USBHS interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by USBHS interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcprdwupen_SPEC;
    pub type Rtcprdwupen = crate::EnumBitfieldStruct<u8, Rtcprdwupen_SPEC>;
    impl Rtcprdwupen {
        #[doc = "S/W standby returns by RTC period interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by RTC period interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcalmwupen_SPEC;
    pub type Rtcalmwupen = crate::EnumBitfieldStruct<u8, Rtcalmwupen_SPEC>;
    impl Rtcalmwupen {
        #[doc = "S/W standby returns by RTC alarm interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by RTC alarm interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acmphs0Wupen_SPEC;
    pub type Acmphs0Wupen = crate::EnumBitfieldStruct<u8, Acmphs0Wupen_SPEC>;
    impl Acmphs0Wupen {
        #[doc = "S/W standby returns by ACMPHS0 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by ACMPHS0 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Wupen_SPEC;
    pub type Lvd2Wupen = crate::EnumBitfieldStruct<u8, Lvd2Wupen_SPEC>;
    impl Lvd2Wupen {
        #[doc = "S/W standby returns by LVD2 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by LVD2 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Wupen_SPEC;
    pub type Lvd1Wupen = crate::EnumBitfieldStruct<u8, Lvd1Wupen_SPEC>;
    impl Lvd1Wupen {
        #[doc = "S/W standby returns by LVD1 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by LVD1 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Keywupen_SPEC;
    pub type Keywupen = crate::EnumBitfieldStruct<u8, Keywupen_SPEC>;
    impl Keywupen {
        #[doc = "S/W standby returns by KEY interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by KEY interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtwupen_SPEC;
    pub type Iwdtwupen = crate::EnumBitfieldStruct<u8, Iwdtwupen_SPEC>;
    impl Iwdtwupen {
        #[doc = "S/W standby returns by IWDT interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IWDT interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen15_SPEC;
    pub type Irqwupen15 = crate::EnumBitfieldStruct<u8, Irqwupen15_SPEC>;
    impl Irqwupen15 {
        #[doc = "S/W standby returns by IRQ15 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ15 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen14_SPEC;
    pub type Irqwupen14 = crate::EnumBitfieldStruct<u8, Irqwupen14_SPEC>;
    impl Irqwupen14 {
        #[doc = "S/W standby returns by IRQ14 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ14 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen13_SPEC;
    pub type Irqwupen13 = crate::EnumBitfieldStruct<u8, Irqwupen13_SPEC>;
    impl Irqwupen13 {
        #[doc = "S/W standby returns by IRQ13 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ13 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen12_SPEC;
    pub type Irqwupen12 = crate::EnumBitfieldStruct<u8, Irqwupen12_SPEC>;
    impl Irqwupen12 {
        #[doc = "S/W standby returns by IRQ12 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ12 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen11_SPEC;
    pub type Irqwupen11 = crate::EnumBitfieldStruct<u8, Irqwupen11_SPEC>;
    impl Irqwupen11 {
        #[doc = "S/W standby returns by IRQ11 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ11 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen10_SPEC;
    pub type Irqwupen10 = crate::EnumBitfieldStruct<u8, Irqwupen10_SPEC>;
    impl Irqwupen10 {
        #[doc = "S/W standby returns by IRQ10 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ10 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen9_SPEC;
    pub type Irqwupen9 = crate::EnumBitfieldStruct<u8, Irqwupen9_SPEC>;
    impl Irqwupen9 {
        #[doc = "S/W standby returns by IRQ9 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ9 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen8_SPEC;
    pub type Irqwupen8 = crate::EnumBitfieldStruct<u8, Irqwupen8_SPEC>;
    impl Irqwupen8 {
        #[doc = "S/W standby returns by IRQ8 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ8 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen7_SPEC;
    pub type Irqwupen7 = crate::EnumBitfieldStruct<u8, Irqwupen7_SPEC>;
    impl Irqwupen7 {
        #[doc = "S/W standby returns by IRQ7 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ7 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen6_SPEC;
    pub type Irqwupen6 = crate::EnumBitfieldStruct<u8, Irqwupen6_SPEC>;
    impl Irqwupen6 {
        #[doc = "S/W standby returns by IRQ6 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ6 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen5_SPEC;
    pub type Irqwupen5 = crate::EnumBitfieldStruct<u8, Irqwupen5_SPEC>;
    impl Irqwupen5 {
        #[doc = "S/W standby returns by IRQ5 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ5 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen4_SPEC;
    pub type Irqwupen4 = crate::EnumBitfieldStruct<u8, Irqwupen4_SPEC>;
    impl Irqwupen4 {
        #[doc = "S/W standby returns by IRQ4 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ4 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen3_SPEC;
    pub type Irqwupen3 = crate::EnumBitfieldStruct<u8, Irqwupen3_SPEC>;
    impl Irqwupen3 {
        #[doc = "S/W standby returns by IRQ3 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ3 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen2_SPEC;
    pub type Irqwupen2 = crate::EnumBitfieldStruct<u8, Irqwupen2_SPEC>;
    impl Irqwupen2 {
        #[doc = "S/W standby returns by IRQ2 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ2 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen1_SPEC;
    pub type Irqwupen1 = crate::EnumBitfieldStruct<u8, Irqwupen1_SPEC>;
    impl Irqwupen1 {
        #[doc = "S/W standby returns by IRQ1 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ1 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen0_SPEC;
    pub type Irqwupen0 = crate::EnumBitfieldStruct<u8, Irqwupen0_SPEC>;
    impl Irqwupen0 {
        #[doc = "S/W standby returns by IRQ0 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "S/W standby returns by IRQ0 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
