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
        12,
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

    #[doc = "Wake Up Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn wupen0(
        &self,
    ) -> &'static crate::common::Reg<self::Wupen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wupen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "Wake Up Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn wupen1(
        &self,
    ) -> &'static crate::common::Reg<self::Wupen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wupen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(420usize),
            )
        }
    }

    #[doc = "ICU Event Enable Register"]
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
        36,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x380usize))
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

        #[doc = "Digital filter is enabled"]
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
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
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

    #[doc = "Main Clock Oscillation Stop Detection Interrupt Enable"]
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

    #[doc = "SRAM ECC Error Interrupt Enable"]
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

        #[doc = "Enabled"]
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
    pub struct Osten_SPEC;
    pub type Osten = crate::EnumBitfieldStruct<u8, Osten_SPEC>;
    impl Osten {
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
    pub struct Reccen_SPEC;
    pub type Reccen = crate::EnumBitfieldStruct<u8, Reccen_SPEC>;
    impl Reccen {
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

    #[doc = "Oscillation Stop Detection Interrupt Status Flag Clear"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            nmiclr::Ostclr,
            nmiclr::Ostclr,
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

    #[doc = "SRAM ECC Error Interrupt Status Flag Clear"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            nmiclr::Reccclr,
            nmiclr::Reccclr,
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

        #[doc = "Clear the NMISR.LVD2ST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostclr_SPEC;
    pub type Ostclr = crate::EnumBitfieldStruct<u8, Ostclr_SPEC>;
    impl Ostclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.OSTST flag"]
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
    pub struct Reccclr_SPEC;
    pub type Reccclr = crate::EnumBitfieldStruct<u8, Reccclr_SPEC>;
    impl Reccclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.RECCST flag"]
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

    #[doc = "Main Clock Oscillation Stop Detection Interrupt Status Flag"]
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

    #[doc = "SRAM ECC Error Interrupt Status Flag"]
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
    pub struct Ostst_SPEC;
    pub type Ostst = crate::EnumBitfieldStruct<u8, Ostst_SPEC>;
    impl Ostst {
        #[doc = "Interrupt not requested for main clock oscillation stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested for main clock oscillation stop"]
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
    pub struct Reccst_SPEC;
    pub type Reccst = crate::EnumBitfieldStruct<u8, Reccst_SPEC>;
    impl Reccst {
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

        #[doc = "Interrupt requested"]
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
pub struct Wupen0_SPEC;
impl crate::sealed::RegSpec for Wupen0_SPEC {
    type DataType = u32;
}

#[doc = "Wake Up Interrupt Enable Register 0"]
pub type Wupen0 = crate::RegValueT<Wupen0_SPEC>;

impl Wupen0 {
    #[doc = "IRQ Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn irqwupen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        wupen0::Irqwupen,
        wupen0::Irqwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            wupen0::Irqwupen,
            wupen0::Irqwupen,
            Wupen0_SPEC,
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
        wupen0::Iwdtwupen,
        wupen0::Iwdtwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            wupen0::Iwdtwupen,
            wupen0::Iwdtwupen,
            Wupen0_SPEC,
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
        wupen0::Lvd1Wupen,
        wupen0::Lvd1Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            wupen0::Lvd1Wupen,
            wupen0::Lvd1Wupen,
            Wupen0_SPEC,
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
        wupen0::Lvd2Wupen,
        wupen0::Lvd2Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            wupen0::Lvd2Wupen,
            wupen0::Lvd2Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVDVBAT Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvdvbatwupen(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        wupen0::Lvdvbatwupen,
        wupen0::Lvdvbatwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            wupen0::Lvdvbatwupen,
            wupen0::Lvdvbatwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVDVRTC Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvdvrtcwupen(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        wupen0::Lvdvrtcwupen,
        wupen0::Lvdvrtcwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            wupen0::Lvdvrtcwupen,
            wupen0::Lvdvrtcwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LVDEXLVD Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn lvdexlvdwupen(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        wupen0::Lvdexlvdwupen,
        wupen0::Lvdexlvdwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            wupen0::Lvdexlvdwupen,
            wupen0::Lvdexlvdwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm Interrupt 1 Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn rtcalm1wupen(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        wupen0::Rtcalm1Wupen,
        wupen0::Rtcalm1Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            wupen0::Rtcalm1Wupen,
            wupen0::Rtcalm1Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm Interrupt 0 Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn rtcalm0wupen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        wupen0::Rtcalm0Wupen,
        wupen0::Rtcalm0Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            wupen0::Rtcalm0Wupen,
            wupen0::Rtcalm0Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Period Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn rtcprdwupen(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        wupen0::Rtcprdwupen,
        wupen0::Rtcprdwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            wupen0::Rtcprdwupen,
            wupen0::Rtcprdwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGTW0 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agtw0udwupen(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        wupen0::Agtw0Udwupen,
        wupen0::Agtw0Udwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            wupen0::Agtw0Udwupen,
            wupen0::Agtw0Udwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGTW1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agtw1udwupen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        wupen0::Agtw1Udwupen,
        wupen0::Agtw1Udwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            wupen0::Agtw1Udwupen,
            wupen0::Agtw1Udwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGTW1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agtw1cawupen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        wupen0::Agtw1Cawupen,
        wupen0::Agtw1Cawupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            wupen0::Agtw1Cawupen,
            wupen0::Agtw1Cawupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGTW1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agtw1cbwupen(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        wupen0::Agtw1Cbwupen,
        wupen0::Agtw1Cbwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            wupen0::Agtw1Cbwupen,
            wupen0::Agtw1Cbwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn iic0wupen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        wupen0::Iic0Wupen,
        wupen0::Iic0Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            wupen0::Iic0Wupen,
            wupen0::Iic0Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wupen0 {
    #[inline(always)]
    fn default() -> Wupen0 {
        <crate::RegValueT<Wupen0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wupen0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen_SPEC;
    pub type Irqwupen = crate::EnumBitfieldStruct<u8, Irqwupen_SPEC>;
    impl Irqwupen {
        #[doc = "Software Standby/Snooze mode returns by IRQn interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IRQn interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtwupen_SPEC;
    pub type Iwdtwupen = crate::EnumBitfieldStruct<u8, Iwdtwupen_SPEC>;
    impl Iwdtwupen {
        #[doc = "Software Standby/Snooze mode returns by IWDT interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IWDT interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Wupen_SPEC;
    pub type Lvd1Wupen = crate::EnumBitfieldStruct<u8, Lvd1Wupen_SPEC>;
    impl Lvd1Wupen {
        #[doc = "Software Standby/Snooze mode returns by LVD1 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by LVD1 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Wupen_SPEC;
    pub type Lvd2Wupen = crate::EnumBitfieldStruct<u8, Lvd2Wupen_SPEC>;
    impl Lvd2Wupen {
        #[doc = "Software Standby/Snooze mode returns by LVD2 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by LVD2 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvdvbatwupen_SPEC;
    pub type Lvdvbatwupen = crate::EnumBitfieldStruct<u8, Lvdvbatwupen_SPEC>;
    impl Lvdvbatwupen {
        #[doc = "Software Standby/Snooze mode returns by LVDVBAT interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by LVDVBAT interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvdvrtcwupen_SPEC;
    pub type Lvdvrtcwupen = crate::EnumBitfieldStruct<u8, Lvdvrtcwupen_SPEC>;
    impl Lvdvrtcwupen {
        #[doc = "Software Standby/Snooze mode returns by LVDVRTC interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by LVDVRTC interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvdexlvdwupen_SPEC;
    pub type Lvdexlvdwupen = crate::EnumBitfieldStruct<u8, Lvdexlvdwupen_SPEC>;
    impl Lvdexlvdwupen {
        #[doc = "Software Standby/Snooze mode returns by LVDEXLVD interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by LVDEXLVD interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcalm1Wupen_SPEC;
    pub type Rtcalm1Wupen = crate::EnumBitfieldStruct<u8, Rtcalm1Wupen_SPEC>;
    impl Rtcalm1Wupen {
        #[doc = "Software Standby/Snooze mode returns by RTCALM1 interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by RTCALM1 interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcalm0Wupen_SPEC;
    pub type Rtcalm0Wupen = crate::EnumBitfieldStruct<u8, Rtcalm0Wupen_SPEC>;
    impl Rtcalm0Wupen {
        #[doc = "Software Standby/Snooze mode returns by RTC alarm interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by RTC alarm interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcprdwupen_SPEC;
    pub type Rtcprdwupen = crate::EnumBitfieldStruct<u8, Rtcprdwupen_SPEC>;
    impl Rtcprdwupen {
        #[doc = "Software Standby/Snooze mode returns by RTC period interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by RTC period interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agtw0Udwupen_SPEC;
    pub type Agtw0Udwupen = crate::EnumBitfieldStruct<u8, Agtw0Udwupen_SPEC>;
    impl Agtw0Udwupen {
        #[doc = "Software Standby/Snooze mode returns by AGTW0 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by AGTW0 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agtw1Udwupen_SPEC;
    pub type Agtw1Udwupen = crate::EnumBitfieldStruct<u8, Agtw1Udwupen_SPEC>;
    impl Agtw1Udwupen {
        #[doc = "Software Standby/Snooze mode returns by AGTW1 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by AGTW1 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agtw1Cawupen_SPEC;
    pub type Agtw1Cawupen = crate::EnumBitfieldStruct<u8, Agtw1Cawupen_SPEC>;
    impl Agtw1Cawupen {
        #[doc = "Software Standby/Snooze mode returns by AGTW1 compare match A interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by AGTW1 compare match A interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agtw1Cbwupen_SPEC;
    pub type Agtw1Cbwupen = crate::EnumBitfieldStruct<u8, Agtw1Cbwupen_SPEC>;
    impl Agtw1Cbwupen {
        #[doc = "Software Standby/Snooze mode returns by AGTW1 compare match B interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by AGTW1 compare match B interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iic0Wupen_SPEC;
    pub type Iic0Wupen = crate::EnumBitfieldStruct<u8, Iic0Wupen_SPEC>;
    impl Iic0Wupen {
        #[doc = "Software Standby/Snooze mode returns by IIC0 address match interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze mode returns by IIC0 address match interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupen1_SPEC;
impl crate::sealed::RegSpec for Wupen1_SPEC {
    type DataType = u32;
}

#[doc = "Wake Up Interrupt Enable Register 1"]
pub type Wupen1 = crate::RegValueT<Wupen1_SPEC>;

impl Wupen1 {
    #[doc = "AGT0 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt0udwupen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wupen1::Agt0Udwupen,
        wupen1::Agt0Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wupen1::Agt0Udwupen,
            wupen1::Agt0Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt1udwupen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        wupen1::Agt1Udwupen,
        wupen1::Agt1Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            wupen1::Agt1Udwupen,
            wupen1::Agt1Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT2 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt2udwupen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        wupen1::Agt2Udwupen,
        wupen1::Agt2Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            wupen1::Agt2Udwupen,
            wupen1::Agt2Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT3 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt3udwupen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        wupen1::Agt3Udwupen,
        wupen1::Agt3Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            wupen1::Agt3Udwupen,
            wupen1::Agt3Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT4 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt4udwupen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        wupen1::Agt4Udwupen,
        wupen1::Agt4Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            wupen1::Agt4Udwupen,
            wupen1::Agt4Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT5 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt5udwupen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        wupen1::Agt5Udwupen,
        wupen1::Agt5Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            wupen1::Agt5Udwupen,
            wupen1::Agt5Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT6 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt6udwupen(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        wupen1::Agt6Udwupen,
        wupen1::Agt6Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            wupen1::Agt6Udwupen,
            wupen1::Agt6Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT7 Underflow Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn agt7udwupen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wupen1::Agt7Udwupen,
        wupen1::Agt7Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wupen1::Agt7Udwupen,
            wupen1::Agt7Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SOSTD Interrupt Software Standby/Snooze Mode Returns Enable"]
    #[inline(always)]
    pub fn sostdwupen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        wupen1::Sostdwupen,
        wupen1::Sostdwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            wupen1::Sostdwupen,
            wupen1::Sostdwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wupen1 {
    #[inline(always)]
    fn default() -> Wupen1 {
        <crate::RegValueT<Wupen1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wupen1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt0Udwupen_SPEC;
    pub type Agt0Udwupen = crate::EnumBitfieldStruct<u8, Agt0Udwupen_SPEC>;
    impl Agt0Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT0 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT0 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Udwupen_SPEC;
    pub type Agt1Udwupen = crate::EnumBitfieldStruct<u8, Agt1Udwupen_SPEC>;
    impl Agt1Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt2Udwupen_SPEC;
    pub type Agt2Udwupen = crate::EnumBitfieldStruct<u8, Agt2Udwupen_SPEC>;
    impl Agt2Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT2 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT2 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt3Udwupen_SPEC;
    pub type Agt3Udwupen = crate::EnumBitfieldStruct<u8, Agt3Udwupen_SPEC>;
    impl Agt3Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT3 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT3 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt4Udwupen_SPEC;
    pub type Agt4Udwupen = crate::EnumBitfieldStruct<u8, Agt4Udwupen_SPEC>;
    impl Agt4Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT4 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT4 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt5Udwupen_SPEC;
    pub type Agt5Udwupen = crate::EnumBitfieldStruct<u8, Agt5Udwupen_SPEC>;
    impl Agt5Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT5 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT5 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt6Udwupen_SPEC;
    pub type Agt6Udwupen = crate::EnumBitfieldStruct<u8, Agt6Udwupen_SPEC>;
    impl Agt6Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT6 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT6 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt7Udwupen_SPEC;
    pub type Agt7Udwupen = crate::EnumBitfieldStruct<u8, Agt7Udwupen_SPEC>;
    impl Agt7Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT7 underflow interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by AGT7 underflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostdwupen_SPEC;
    pub type Sostdwupen = crate::EnumBitfieldStruct<u8, Sostdwupen_SPEC>;
    impl Sostdwupen {
        #[doc = "Software Standby/Snooze Mode returns by SOSTD interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software Standby/Snooze Mode returns by SOSTD interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ielen_SPEC;
impl crate::sealed::RegSpec for Ielen_SPEC {
    type DataType = u8;
}

#[doc = "ICU Event Enable Register"]
pub type Ielen = crate::RegValueT<Ielen_SPEC>;

impl Ielen {
    #[doc = "RTCALM0, RTCALM1, and RTCPRD Interrupts Enable (when LPOPTEN bit = 1)"]
    #[inline(always)]
    pub fn rtcinten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ielen::Rtcinten,
        ielen::Rtcinten,
        Ielen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ielen::Rtcinten,
            ielen::Rtcinten,
            Ielen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parts Asynchronous Interrupts Enable except RTC (when LPOPTEN bit = 1)"]
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
    pub struct Rtcinten_SPEC;
    pub type Rtcinten = crate::EnumBitfieldStruct<u8, Rtcinten_SPEC>;
    impl Rtcinten {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
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
