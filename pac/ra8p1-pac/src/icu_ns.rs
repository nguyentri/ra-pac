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
#[doc = r"Interrupt Controller"]
unsafe impl ::core::marker::Send for super::IcuNs {}
unsafe impl ::core::marker::Sync for super::IcuNs {}
impl super::IcuNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Non-Maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(&self) -> &'static crate::common::Reg<self::Nmier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
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
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(&self) -> &'static crate::common::Reg<self::Nmisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nmisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
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

    #[doc = "Wake Up interrupt enable register 1"]
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

    #[doc = "Deep Sleep Wake Up IRQ Enable Register 0"]
    #[inline(always)]
    pub const fn dslpwupirqen0(
        &self,
    ) -> &'static crate::common::Reg<self::Dslpwupirqen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dslpwupirqen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Deep Sleep Wake Up IRQ Enable Register 1"]
    #[inline(always)]
    pub const fn dslpwupirqen1(
        &self,
    ) -> &'static crate::common::Reg<self::Dslpwupirqen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dslpwupirqen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "Deep Sleep Wake Up IRQ Enable Register 2"]
    #[inline(always)]
    pub const fn dslpwupirqen2(
        &self,
    ) -> &'static crate::common::Reg<self::Dslpwupirqen2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dslpwupirqen2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(540usize),
            )
        }
    }

    #[doc = "DMAC Event Link Setting Register m (m = 0 to 7)"]
    #[inline(always)]
    pub const fn delsrm(
        &self,
    ) -> &'static crate::common::Reg<self::DelsRm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DelsRm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(640usize),
            )
        }
    }

    #[doc = "Interrupt Controller Unit Event Link Setting Register %s"]
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmier_SPEC;
impl crate::sealed::RegSpec for Nmier_SPEC {
    type DataType = u32;
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
    pub fn pvd1en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmier::Pvd1En,
        nmier::Pvd1En,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmier::Pvd1En,
            nmier::Pvd1En,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage monitor 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pvd2en(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nmier::Pvd2En,
        nmier::Pvd2En,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nmier::Pvd2En,
            nmier::Pvd2En,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sub Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn sosten(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        nmier::Sosten,
        nmier::Sosten,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            nmier::Sosten,
            nmier::Sosten,
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

    #[inline(always)]
    pub fn busen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        nmier::Busen,
        nmier::Busen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            nmier::Busen,
            nmier::Busen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        nmier::Cmen,
        nmier::Cmen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            nmier::Cmen,
            nmier::Cmen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Local Memory Error Interrupt Enable"]
    #[inline(always)]
    pub fn lmen(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        nmier::Lmen,
        nmier::Lmen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            nmier::Lmen,
            nmier::Lmen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn luen(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        nmier::Luen,
        nmier::Luen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            nmier::Luen,
            nmier::Luen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FPU Exception Interrupt Enable"]
    #[inline(always)]
    pub fn fpuexcen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        nmier::Fpuexcen,
        nmier::Fpuexcen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            nmier::Fpuexcen,
            nmier::Fpuexcen,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MRAM MRC read Error Interrupt Enable"]
    #[inline(always)]
    pub fn mrcrden(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        nmier::Mrcrden,
        nmier::Mrcrden,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            nmier::Mrcrden,
            nmier::Mrcrden,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MRAM MRE read Error Interrupt Enable"]
    #[inline(always)]
    pub fn mrerden(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        nmier::Mrerden,
        nmier::Mrerden,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            nmier::Mrerden,
            nmier::Mrerden,
            Nmier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPC NMI CPU mutual Interrupt Enable"]
    #[inline(always)]
    pub fn ipcen(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        nmier::Ipcen,
        nmier::Ipcen,
        Nmier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            nmier::Ipcen,
            nmier::Ipcen,
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
    pub struct Pvd1En_SPEC;
    pub type Pvd1En = crate::EnumBitfieldStruct<u8, Pvd1En_SPEC>;
    impl Pvd1En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2En_SPEC;
    pub type Pvd2En = crate::EnumBitfieldStruct<u8, Pvd2En_SPEC>;
    impl Pvd2En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sosten_SPEC;
    pub type Sosten = crate::EnumBitfieldStruct<u8, Sosten_SPEC>;
    impl Sosten {
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
    pub struct Busen_SPEC;
    pub type Busen = crate::EnumBitfieldStruct<u8, Busen_SPEC>;
    impl Busen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmen_SPEC;
    pub type Cmen = crate::EnumBitfieldStruct<u8, Cmen_SPEC>;
    impl Cmen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lmen_SPEC;
    pub type Lmen = crate::EnumBitfieldStruct<u8, Lmen_SPEC>;
    impl Lmen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Luen_SPEC;
    pub type Luen = crate::EnumBitfieldStruct<u8, Luen_SPEC>;
    impl Luen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fpuexcen_SPEC;
    pub type Fpuexcen = crate::EnumBitfieldStruct<u8, Fpuexcen_SPEC>;
    impl Fpuexcen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcrden_SPEC;
    pub type Mrcrden = crate::EnumBitfieldStruct<u8, Mrcrden_SPEC>;
    impl Mrcrden {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrerden_SPEC;
    pub type Mrerden = crate::EnumBitfieldStruct<u8, Mrerden_SPEC>;
    impl Mrerden {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ipcen_SPEC;
    pub type Ipcen = crate::EnumBitfieldStruct<u8, Ipcen_SPEC>;
    impl Ipcen {
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
    type DataType = u32;
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
    pub fn pvd1clr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmiclr::Pvd1Clr,
        nmiclr::Pvd1Clr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmiclr::Pvd1Clr,
            nmiclr::Pvd1Clr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn pvd2clr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nmiclr::Pvd2Clr,
        nmiclr::Pvd2Clr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nmiclr::Pvd2Clr,
            nmiclr::Pvd2Clr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Oscillation Stop Detection Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn sostclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        nmiclr::Sostclr,
        nmiclr::Sostclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            nmiclr::Sostclr,
            nmiclr::Sostclr,
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

    #[inline(always)]
    pub fn busclr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        nmiclr::Busclr,
        nmiclr::Busclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            nmiclr::Busclr,
            nmiclr::Busclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmclr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        nmiclr::Cmclr,
        nmiclr::Cmclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            nmiclr::Cmclr,
            nmiclr::Cmclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmclr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        nmiclr::Lmclr,
        nmiclr::Lmclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            nmiclr::Lmclr,
            nmiclr::Lmclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn luclr(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        nmiclr::Luclr,
        nmiclr::Luclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            nmiclr::Luclr,
            nmiclr::Luclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FPU Exception Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn fpuexcclr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        nmiclr::Fpuexcclr,
        nmiclr::Fpuexcclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            nmiclr::Fpuexcclr,
            nmiclr::Fpuexcclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MRAM MRC read Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn mrcrdclr(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        nmiclr::Mrcrdclr,
        nmiclr::Mrcrdclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            nmiclr::Mrcrdclr,
            nmiclr::Mrcrdclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MRAM MRE read Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn mrerdclr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        nmiclr::Mrerdclr,
        nmiclr::Mrerdclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            nmiclr::Mrerdclr,
            nmiclr::Mrerdclr,
            Nmiclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPC NMI CPU mutual Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn ipcclr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        nmiclr::Ipcclr,
        nmiclr::Ipcclr,
        Nmiclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            nmiclr::Ipcclr,
            nmiclr::Ipcclr,
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
    pub struct Pvd1Clr_SPEC;
    pub type Pvd1Clr = crate::EnumBitfieldStruct<u8, Pvd1Clr_SPEC>;
    impl Pvd1Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.PVD1ST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2Clr_SPEC;
    pub type Pvd2Clr = crate::EnumBitfieldStruct<u8, Pvd2Clr_SPEC>;
    impl Pvd2Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.PVD2ST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostclr_SPEC;
    pub type Sostclr = crate::EnumBitfieldStruct<u8, Sostclr_SPEC>;
    impl Sostclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.OSTST flag"]
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
    pub struct Busclr_SPEC;
    pub type Busclr = crate::EnumBitfieldStruct<u8, Busclr_SPEC>;
    impl Busclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.BUSST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmclr_SPEC;
    pub type Cmclr = crate::EnumBitfieldStruct<u8, Cmclr_SPEC>;
    impl Cmclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.CMST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lmclr_SPEC;
    pub type Lmclr = crate::EnumBitfieldStruct<u8, Lmclr_SPEC>;
    impl Lmclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.LMST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Luclr_SPEC;
    pub type Luclr = crate::EnumBitfieldStruct<u8, Luclr_SPEC>;
    impl Luclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.LUST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fpuexcclr_SPEC;
    pub type Fpuexcclr = crate::EnumBitfieldStruct<u8, Fpuexcclr_SPEC>;
    impl Fpuexcclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.FPUEXCST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcrdclr_SPEC;
    pub type Mrcrdclr = crate::EnumBitfieldStruct<u8, Mrcrdclr_SPEC>;
    impl Mrcrdclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.MRCRDST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrerdclr_SPEC;
    pub type Mrerdclr = crate::EnumBitfieldStruct<u8, Mrerdclr_SPEC>;
    impl Mrerdclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.MRERDST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ipcclr_SPEC;
    pub type Ipcclr = crate::EnumBitfieldStruct<u8, Ipcclr_SPEC>;
    impl Ipcclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the NMISR.IPCST flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisr_SPEC;
impl crate::sealed::RegSpec for Nmisr_SPEC {
    type DataType = u32;
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
    pub fn pvd1st(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nmisr::Pvd1St,
        nmisr::Pvd1St,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nmisr::Pvd1St,
            nmisr::Pvd1St,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Voltage Monitor 2 Interrupt Status Flag"]
    #[inline(always)]
    pub fn pvd2st(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nmisr::Pvd2St,
        nmisr::Pvd2St,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nmisr::Pvd2St,
            nmisr::Pvd2St,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sub Oscillation Stop Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn sostst(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        nmisr::Sostst,
        nmisr::Sostst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            nmisr::Sostst,
            nmisr::Sostst,
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

    #[doc = "Bus Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn busst(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        nmisr::Busst,
        nmisr::Busst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            nmisr::Busst,
            nmisr::Busst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmst(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        nmisr::Cmst,
        nmisr::Cmst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            nmisr::Cmst,
            nmisr::Cmst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmst(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        nmisr::Lmst,
        nmisr::Lmst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            nmisr::Lmst,
            nmisr::Lmst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lust(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        nmisr::Lust,
        nmisr::Lust,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            nmisr::Lust,
            nmisr::Lust,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fpuexcst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        nmisr::Fpuexcst,
        nmisr::Fpuexcst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            nmisr::Fpuexcst,
            nmisr::Fpuexcst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mrcrdst(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        nmisr::Mrcrdst,
        nmisr::Mrcrdst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            nmisr::Mrcrdst,
            nmisr::Mrcrdst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mrerdst(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        nmisr::Mrerdst,
        nmisr::Mrerdst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            nmisr::Mrerdst,
            nmisr::Mrerdst,
            Nmisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ipcst(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        nmisr::Ipcst,
        nmisr::Ipcst,
        Nmisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            nmisr::Ipcst,
            nmisr::Ipcst,
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
    pub struct Pvd1St_SPEC;
    pub type Pvd1St = crate::EnumBitfieldStruct<u8, Pvd1St_SPEC>;
    impl Pvd1St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2St_SPEC;
    pub type Pvd2St = crate::EnumBitfieldStruct<u8, Pvd2St_SPEC>;
    impl Pvd2St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostst_SPEC;
    pub type Sostst = crate::EnumBitfieldStruct<u8, Sostst_SPEC>;
    impl Sostst {
        #[doc = "Interrupt not requested for sub oscillation stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested for sub oscillation stop"]
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
    pub struct Busst_SPEC;
    pub type Busst = crate::EnumBitfieldStruct<u8, Busst_SPEC>;
    impl Busst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmst_SPEC;
    pub type Cmst = crate::EnumBitfieldStruct<u8, Cmst_SPEC>;
    impl Cmst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lmst_SPEC;
    pub type Lmst = crate::EnumBitfieldStruct<u8, Lmst_SPEC>;
    impl Lmst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lust_SPEC;
    pub type Lust = crate::EnumBitfieldStruct<u8, Lust_SPEC>;
    impl Lust {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fpuexcst_SPEC;
    pub type Fpuexcst = crate::EnumBitfieldStruct<u8, Fpuexcst_SPEC>;
    impl Fpuexcst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcrdst_SPEC;
    pub type Mrcrdst = crate::EnumBitfieldStruct<u8, Mrcrdst_SPEC>;
    impl Mrcrdst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrerdst_SPEC;
    pub type Mrerdst = crate::EnumBitfieldStruct<u8, Mrerdst_SPEC>;
    impl Mrerdst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ipcst_SPEC;
    pub type Ipcst = crate::EnumBitfieldStruct<u8, Ipcst_SPEC>;
    impl Ipcst {
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
    #[inline(always)]
    pub fn irqwupen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wupen0::Irqwupen0,
        wupen0::Irqwupen0,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wupen0::Irqwupen0,
            wupen0::Irqwupen0,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        wupen0::Irqwupen1,
        wupen0::Irqwupen1,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            wupen0::Irqwupen1,
            wupen0::Irqwupen1,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        wupen0::Irqwupen2,
        wupen0::Irqwupen2,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            wupen0::Irqwupen2,
            wupen0::Irqwupen2,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        wupen0::Irqwupen3,
        wupen0::Irqwupen3,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            wupen0::Irqwupen3,
            wupen0::Irqwupen3,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        wupen0::Irqwupen4,
        wupen0::Irqwupen4,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            wupen0::Irqwupen4,
            wupen0::Irqwupen4,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        wupen0::Irqwupen5,
        wupen0::Irqwupen5,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            wupen0::Irqwupen5,
            wupen0::Irqwupen5,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        wupen0::Irqwupen6,
        wupen0::Irqwupen6,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            wupen0::Irqwupen6,
            wupen0::Irqwupen6,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wupen0::Irqwupen7,
        wupen0::Irqwupen7,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wupen0::Irqwupen7,
            wupen0::Irqwupen7,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        wupen0::Irqwupen8,
        wupen0::Irqwupen8,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            wupen0::Irqwupen8,
            wupen0::Irqwupen8,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        wupen0::Irqwupen9,
        wupen0::Irqwupen9,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            wupen0::Irqwupen9,
            wupen0::Irqwupen9,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        wupen0::Irqwupen10,
        wupen0::Irqwupen10,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            wupen0::Irqwupen10,
            wupen0::Irqwupen10,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        wupen0::Irqwupen11,
        wupen0::Irqwupen11,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            wupen0::Irqwupen11,
            wupen0::Irqwupen11,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        wupen0::Irqwupen12,
        wupen0::Irqwupen12,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            wupen0::Irqwupen12,
            wupen0::Irqwupen12,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        wupen0::Irqwupen13,
        wupen0::Irqwupen13,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            wupen0::Irqwupen13,
            wupen0::Irqwupen13,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        wupen0::Irqwupen14,
        wupen0::Irqwupen14,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            wupen0::Irqwupen14,
            wupen0::Irqwupen14,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irqwupen15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        wupen0::Irqwupen15,
        wupen0::Irqwupen15,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            wupen0::Irqwupen15,
            wupen0::Irqwupen15,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IWDT Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
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

    #[doc = "PVD1 Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn pvd1wupen(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        wupen0::Pvd1Wupen,
        wupen0::Pvd1Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            wupen0::Pvd1Wupen,
            wupen0::Pvd1Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PVD2 Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn pvd2wupen(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        wupen0::Pvd2Wupen,
        wupen0::Pvd2Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            wupen0::Pvd2Wupen,
            wupen0::Pvd2Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBATT Monitor Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn vbattwupen(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        wupen0::Vbattwupen,
        wupen0::Vbattwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            wupen0::Vbattwupen,
            wupen0::Vbattwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Alarm Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn rtcalmwupen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        wupen0::Rtcalmwupen,
        wupen0::Rtcalmwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            wupen0::Rtcalmwupen,
            wupen0::Rtcalmwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTC Period Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
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

    #[doc = "USBHS Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn usbhswupen(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        wupen0::Usbhswupen,
        wupen0::Usbhswupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            wupen0::Usbhswupen,
            wupen0::Usbhswupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBFS Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn usbfs0wupen(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        wupen0::Usbfs0Wupen,
        wupen0::Usbfs0Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            wupen0::Usbfs0Wupen,
            wupen0::Usbfs0Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Underflow Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1udwupen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        wupen0::Agt1Udwupen,
        wupen0::Agt1Udwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            wupen0::Agt1Udwupen,
            wupen0::Agt1Udwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Compare Match A Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1cawupen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        wupen0::Agt1Cawupen,
        wupen0::Agt1Cawupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            wupen0::Agt1Cawupen,
            wupen0::Agt1Cawupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGT1 Compare Match B Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1cbwupen(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        wupen0::Agt1Cbwupen,
        wupen0::Agt1Cbwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            wupen0::Agt1Cbwupen,
            wupen0::Agt1Cbwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RIIC0 Address Match Interrupt Deep Sleep/Software Standby Mode Returns Enable bit"]
    #[inline(always)]
    pub fn riic0wupen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        wupen0::Riic0Wupen,
        wupen0::Riic0Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            wupen0::Riic0Wupen,
            wupen0::Riic0Wupen,
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
    pub struct Irqwupen0_SPEC;
    pub type Irqwupen0 = crate::EnumBitfieldStruct<u8, Irqwupen0_SPEC>;
    impl Irqwupen0 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen1_SPEC;
    pub type Irqwupen1 = crate::EnumBitfieldStruct<u8, Irqwupen1_SPEC>;
    impl Irqwupen1 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen2_SPEC;
    pub type Irqwupen2 = crate::EnumBitfieldStruct<u8, Irqwupen2_SPEC>;
    impl Irqwupen2 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen3_SPEC;
    pub type Irqwupen3 = crate::EnumBitfieldStruct<u8, Irqwupen3_SPEC>;
    impl Irqwupen3 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen4_SPEC;
    pub type Irqwupen4 = crate::EnumBitfieldStruct<u8, Irqwupen4_SPEC>;
    impl Irqwupen4 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen5_SPEC;
    pub type Irqwupen5 = crate::EnumBitfieldStruct<u8, Irqwupen5_SPEC>;
    impl Irqwupen5 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen6_SPEC;
    pub type Irqwupen6 = crate::EnumBitfieldStruct<u8, Irqwupen6_SPEC>;
    impl Irqwupen6 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen7_SPEC;
    pub type Irqwupen7 = crate::EnumBitfieldStruct<u8, Irqwupen7_SPEC>;
    impl Irqwupen7 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen8_SPEC;
    pub type Irqwupen8 = crate::EnumBitfieldStruct<u8, Irqwupen8_SPEC>;
    impl Irqwupen8 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen9_SPEC;
    pub type Irqwupen9 = crate::EnumBitfieldStruct<u8, Irqwupen9_SPEC>;
    impl Irqwupen9 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen10_SPEC;
    pub type Irqwupen10 = crate::EnumBitfieldStruct<u8, Irqwupen10_SPEC>;
    impl Irqwupen10 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen11_SPEC;
    pub type Irqwupen11 = crate::EnumBitfieldStruct<u8, Irqwupen11_SPEC>;
    impl Irqwupen11 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen12_SPEC;
    pub type Irqwupen12 = crate::EnumBitfieldStruct<u8, Irqwupen12_SPEC>;
    impl Irqwupen12 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen13_SPEC;
    pub type Irqwupen13 = crate::EnumBitfieldStruct<u8, Irqwupen13_SPEC>;
    impl Irqwupen13 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen14_SPEC;
    pub type Irqwupen14 = crate::EnumBitfieldStruct<u8, Irqwupen14_SPEC>;
    impl Irqwupen14 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen15_SPEC;
    pub type Irqwupen15 = crate::EnumBitfieldStruct<u8, Irqwupen15_SPEC>;
    impl Irqwupen15 {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtwupen_SPEC;
    pub type Iwdtwupen = crate::EnumBitfieldStruct<u8, Iwdtwupen_SPEC>;
    impl Iwdtwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by IWDT interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IWDT interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd1Wupen_SPEC;
    pub type Pvd1Wupen = crate::EnumBitfieldStruct<u8, Pvd1Wupen_SPEC>;
    impl Pvd1Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by PVD1 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by PVD1 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2Wupen_SPEC;
    pub type Pvd2Wupen = crate::EnumBitfieldStruct<u8, Pvd2Wupen_SPEC>;
    impl Pvd2Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by PVD2 interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by PVD2 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbattwupen_SPEC;
    pub type Vbattwupen = crate::EnumBitfieldStruct<u8, Vbattwupen_SPEC>;
    impl Vbattwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by VBATT monitor interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by VBATT monitor interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcalmwupen_SPEC;
    pub type Rtcalmwupen = crate::EnumBitfieldStruct<u8, Rtcalmwupen_SPEC>;
    impl Rtcalmwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by RTC alarm interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by RTC alarm interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcprdwupen_SPEC;
    pub type Rtcprdwupen = crate::EnumBitfieldStruct<u8, Rtcprdwupen_SPEC>;
    impl Rtcprdwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by RTC period interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by RTC period interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbhswupen_SPEC;
    pub type Usbhswupen = crate::EnumBitfieldStruct<u8, Usbhswupen_SPEC>;
    impl Usbhswupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by USBHS interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by USBHS interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbfs0Wupen_SPEC;
    pub type Usbfs0Wupen = crate::EnumBitfieldStruct<u8, Usbfs0Wupen_SPEC>;
    impl Usbfs0Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by USBFS interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by USBFS interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Udwupen_SPEC;
    pub type Agt1Udwupen = crate::EnumBitfieldStruct<u8, Agt1Udwupen_SPEC>;
    impl Agt1Udwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 underflow interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 underflow interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cawupen_SPEC;
    pub type Agt1Cawupen = crate::EnumBitfieldStruct<u8, Agt1Cawupen_SPEC>;
    impl Agt1Cawupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match A interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match A interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cbwupen_SPEC;
    pub type Agt1Cbwupen = crate::EnumBitfieldStruct<u8, Agt1Cbwupen_SPEC>;
    impl Agt1Cbwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match B interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match B interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Riic0Wupen_SPEC;
    pub type Riic0Wupen = crate::EnumBitfieldStruct<u8, Riic0Wupen_SPEC>;
    impl Riic0Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by IIC0 address match A interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby Mode returns by IIC0 address match A interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupen1_SPEC;
impl crate::sealed::RegSpec for Wupen1_SPEC {
    type DataType = u32;
}

#[doc = "Wake Up interrupt enable register 1"]
pub type Wupen1 = crate::RegValueT<Wupen1_SPEC>;

impl Wupen1 {
    #[doc = "Comparator-HS0 Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn comphs0wupen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        wupen1::Comphs0Wupen,
        wupen1::Comphs0Wupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            wupen1::Comphs0Wupen,
            wupen1::Comphs0Wupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sub Oscillation Stop Detection Interrupt Deep Sleep/Software Standby Returns Enable bit"]
    #[inline(always)]
    pub fn soscwupen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wupen1::Soscwupen,
        wupen1::Soscwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wupen1::Soscwupen,
            wupen1::Soscwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT0 Underflow Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp0uwupen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        wupen1::Ulp0Uwupen,
        wupen1::Ulp0Uwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            wupen1::Ulp0Uwupen,
            wupen1::Ulp0Uwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT0 Compare Match A Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp0awupen(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        wupen1::Ulp0Awupen,
        wupen1::Ulp0Awupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            wupen1::Ulp0Awupen,
            wupen1::Ulp0Awupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT0 Compare Match B Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp0bwupen(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        wupen1::Ulp0Bwupen,
        wupen1::Ulp0Bwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            wupen1::Ulp0Bwupen,
            wupen1::Ulp0Bwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I3C Wakeup Condition Detection Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn i3cwupen(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        wupen1::I3Cwupen,
        wupen1::I3Cwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            wupen1::I3Cwupen,
            wupen1::I3Cwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT1 Underflow Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp1uwupen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        wupen1::Ulp1Uwupen,
        wupen1::Ulp1Uwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            wupen1::Ulp1Uwupen,
            wupen1::Ulp1Uwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT1 Compare Match A Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp1awupen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        wupen1::Ulp1Awupen,
        wupen1::Ulp1Awupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            wupen1::Ulp1Awupen,
            wupen1::Ulp1Awupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPT1 Compare Match B Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp1bwupen(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        wupen1::Ulp1Bwupen,
        wupen1::Ulp1Bwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            wupen1::Ulp1Bwupen,
            wupen1::Ulp1Bwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PDMIF Sound Detection Interrupt Deep Sleep/Software Standby Returns Enable bit"]
    #[inline(always)]
    pub fn pdmwupen(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        wupen1::Pdmwupen,
        wupen1::Pdmwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            wupen1::Pdmwupen,
            wupen1::Pdmwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        wupen1::Irqwupen16,
        wupen1::Irqwupen16,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            wupen1::Irqwupen16,
            wupen1::Irqwupen16,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        wupen1::Irqwupen17,
        wupen1::Irqwupen17,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            wupen1::Irqwupen17,
            wupen1::Irqwupen17,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        wupen1::Irqwupen18,
        wupen1::Irqwupen18,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            wupen1::Irqwupen18,
            wupen1::Irqwupen18,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        wupen1::Irqwupen19,
        wupen1::Irqwupen19,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            wupen1::Irqwupen19,
            wupen1::Irqwupen19,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        wupen1::Irqwupen20,
        wupen1::Irqwupen20,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            wupen1::Irqwupen20,
            wupen1::Irqwupen20,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        wupen1::Irqwupen21,
        wupen1::Irqwupen21,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            wupen1::Irqwupen21,
            wupen1::Irqwupen21,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        wupen1::Irqwupen22,
        wupen1::Irqwupen22,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            wupen1::Irqwupen22,
            wupen1::Irqwupen22,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        wupen1::Irqwupen23,
        wupen1::Irqwupen23,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            wupen1::Irqwupen23,
            wupen1::Irqwupen23,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        wupen1::Irqwupen24,
        wupen1::Irqwupen24,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            wupen1::Irqwupen24,
            wupen1::Irqwupen24,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        wupen1::Irqwupen25,
        wupen1::Irqwupen25,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            wupen1::Irqwupen25,
            wupen1::Irqwupen25,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        wupen1::Irqwupen26,
        wupen1::Irqwupen26,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            wupen1::Irqwupen26,
            wupen1::Irqwupen26,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        wupen1::Irqwupen27,
        wupen1::Irqwupen27,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            wupen1::Irqwupen27,
            wupen1::Irqwupen27,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        wupen1::Irqwupen28,
        wupen1::Irqwupen28,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            wupen1::Irqwupen28,
            wupen1::Irqwupen28,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        wupen1::Irqwupen29,
        wupen1::Irqwupen29,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            wupen1::Irqwupen29,
            wupen1::Irqwupen29,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        wupen1::Irqwupen30,
        wupen1::Irqwupen30,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            wupen1::Irqwupen30,
            wupen1::Irqwupen30,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Deep Sleep/Software Standby Returns Enable bits IRQ31 to IRQ16"]
    #[inline(always)]
    pub fn irqwupen31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        wupen1::Irqwupen31,
        wupen1::Irqwupen31,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            wupen1::Irqwupen31,
            wupen1::Irqwupen31,
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
    pub struct Comphs0Wupen_SPEC;
    pub type Comphs0Wupen = crate::EnumBitfieldStruct<u8, Comphs0Wupen_SPEC>;
    impl Comphs0Wupen {
        #[doc = "Deep Sleep/Software Standby returns by Comparator-HS0 interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by Comparator-HS0 interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soscwupen_SPEC;
    pub type Soscwupen = crate::EnumBitfieldStruct<u8, Soscwupen_SPEC>;
    impl Soscwupen {
        #[doc = "Deep Sleep/Software Standby returns by Sub Oscillation Stop Detection Interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by Sub Oscillation Stop Detection Interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp0Uwupen_SPEC;
    pub type Ulp0Uwupen = crate::EnumBitfieldStruct<u8, Ulp0Uwupen_SPEC>;
    impl Ulp0Uwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Underflow interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp0Awupen_SPEC;
    pub type Ulp0Awupen = crate::EnumBitfieldStruct<u8, Ulp0Awupen_SPEC>;
    impl Ulp0Awupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match A interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match A interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp0Bwupen_SPEC;
    pub type Ulp0Bwupen = crate::EnumBitfieldStruct<u8, Ulp0Bwupen_SPEC>;
    impl Ulp0Bwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match B interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match B interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct I3Cwupen_SPEC;
    pub type I3Cwupen = crate::EnumBitfieldStruct<u8, I3Cwupen_SPEC>;
    impl I3Cwupen {
        #[doc = "Deep Sleep/Software Standby returns by I3C wake up interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by I3C wake up interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp1Uwupen_SPEC;
    pub type Ulp1Uwupen = crate::EnumBitfieldStruct<u8, Ulp1Uwupen_SPEC>;
    impl Ulp1Uwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Underflow interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Underflow interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp1Awupen_SPEC;
    pub type Ulp1Awupen = crate::EnumBitfieldStruct<u8, Ulp1Awupen_SPEC>;
    impl Ulp1Awupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match A interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match A interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp1Bwupen_SPEC;
    pub type Ulp1Bwupen = crate::EnumBitfieldStruct<u8, Ulp1Bwupen_SPEC>;
    impl Ulp1Bwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match B interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match B interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdmwupen_SPEC;
    pub type Pdmwupen = crate::EnumBitfieldStruct<u8, Pdmwupen_SPEC>;
    impl Pdmwupen {
        #[doc = "Deep Sleep/Software Standby returns by PDMIF Sound Detection interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by PDMIF Sound Detection interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen16_SPEC;
    pub type Irqwupen16 = crate::EnumBitfieldStruct<u8, Irqwupen16_SPEC>;
    impl Irqwupen16 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen17_SPEC;
    pub type Irqwupen17 = crate::EnumBitfieldStruct<u8, Irqwupen17_SPEC>;
    impl Irqwupen17 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen18_SPEC;
    pub type Irqwupen18 = crate::EnumBitfieldStruct<u8, Irqwupen18_SPEC>;
    impl Irqwupen18 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen19_SPEC;
    pub type Irqwupen19 = crate::EnumBitfieldStruct<u8, Irqwupen19_SPEC>;
    impl Irqwupen19 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen20_SPEC;
    pub type Irqwupen20 = crate::EnumBitfieldStruct<u8, Irqwupen20_SPEC>;
    impl Irqwupen20 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen21_SPEC;
    pub type Irqwupen21 = crate::EnumBitfieldStruct<u8, Irqwupen21_SPEC>;
    impl Irqwupen21 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen22_SPEC;
    pub type Irqwupen22 = crate::EnumBitfieldStruct<u8, Irqwupen22_SPEC>;
    impl Irqwupen22 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen23_SPEC;
    pub type Irqwupen23 = crate::EnumBitfieldStruct<u8, Irqwupen23_SPEC>;
    impl Irqwupen23 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen24_SPEC;
    pub type Irqwupen24 = crate::EnumBitfieldStruct<u8, Irqwupen24_SPEC>;
    impl Irqwupen24 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen25_SPEC;
    pub type Irqwupen25 = crate::EnumBitfieldStruct<u8, Irqwupen25_SPEC>;
    impl Irqwupen25 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen26_SPEC;
    pub type Irqwupen26 = crate::EnumBitfieldStruct<u8, Irqwupen26_SPEC>;
    impl Irqwupen26 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen27_SPEC;
    pub type Irqwupen27 = crate::EnumBitfieldStruct<u8, Irqwupen27_SPEC>;
    impl Irqwupen27 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen28_SPEC;
    pub type Irqwupen28 = crate::EnumBitfieldStruct<u8, Irqwupen28_SPEC>;
    impl Irqwupen28 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen29_SPEC;
    pub type Irqwupen29 = crate::EnumBitfieldStruct<u8, Irqwupen29_SPEC>;
    impl Irqwupen29 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen30_SPEC;
    pub type Irqwupen30 = crate::EnumBitfieldStruct<u8, Irqwupen30_SPEC>;
    impl Irqwupen30 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen31_SPEC;
    pub type Irqwupen31 = crate::EnumBitfieldStruct<u8, Irqwupen31_SPEC>;
    impl Irqwupen31 {
        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep/Software Standby returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dslpwupirqen0_SPEC;
impl crate::sealed::RegSpec for Dslpwupirqen0_SPEC {
    type DataType = u32;
}

#[doc = "Deep Sleep Wake Up IRQ Enable Register 0"]
pub type Dslpwupirqen0 = crate::RegValueT<Dslpwupirqen0_SPEC>;

impl Dslpwupirqen0 {
    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq00,
        dslpwupirqen0::Irq00,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq00,
            dslpwupirqen0::Irq00,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq01,
        dslpwupirqen0::Irq01,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq01,
            dslpwupirqen0::Irq01,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq02,
        dslpwupirqen0::Irq02,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq02,
            dslpwupirqen0::Irq02,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq03,
        dslpwupirqen0::Irq03,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq03,
            dslpwupirqen0::Irq03,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq04,
        dslpwupirqen0::Irq04,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq04,
            dslpwupirqen0::Irq04,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq05,
        dslpwupirqen0::Irq05,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq05,
            dslpwupirqen0::Irq05,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq06,
        dslpwupirqen0::Irq06,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq06,
            dslpwupirqen0::Irq06,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq07,
        dslpwupirqen0::Irq07,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq07,
            dslpwupirqen0::Irq07,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq08,
        dslpwupirqen0::Irq08,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq08,
            dslpwupirqen0::Irq08,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq09,
        dslpwupirqen0::Irq09,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq09,
            dslpwupirqen0::Irq09,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq10,
        dslpwupirqen0::Irq10,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq10,
            dslpwupirqen0::Irq10,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq11,
        dslpwupirqen0::Irq11,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq11,
            dslpwupirqen0::Irq11,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq12,
        dslpwupirqen0::Irq12,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq12,
            dslpwupirqen0::Irq12,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq13,
        dslpwupirqen0::Irq13,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq13,
            dslpwupirqen0::Irq13,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq14,
        dslpwupirqen0::Irq14,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq14,
            dslpwupirqen0::Irq14,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq15,
        dslpwupirqen0::Irq15,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq15,
            dslpwupirqen0::Irq15,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq16,
        dslpwupirqen0::Irq16,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq16,
            dslpwupirqen0::Irq16,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq17,
        dslpwupirqen0::Irq17,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq17,
            dslpwupirqen0::Irq17,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq18,
        dslpwupirqen0::Irq18,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq18,
            dslpwupirqen0::Irq18,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq19,
        dslpwupirqen0::Irq19,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq19,
            dslpwupirqen0::Irq19,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq20,
        dslpwupirqen0::Irq20,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq20,
            dslpwupirqen0::Irq20,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq21,
        dslpwupirqen0::Irq21,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq21,
            dslpwupirqen0::Irq21,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq22,
        dslpwupirqen0::Irq22,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq22,
            dslpwupirqen0::Irq22,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq23,
        dslpwupirqen0::Irq23,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq23,
            dslpwupirqen0::Irq23,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq24,
        dslpwupirqen0::Irq24,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq24,
            dslpwupirqen0::Irq24,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq25,
        dslpwupirqen0::Irq25,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq25,
            dslpwupirqen0::Irq25,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq26,
        dslpwupirqen0::Irq26,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq26,
            dslpwupirqen0::Irq26,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq27,
        dslpwupirqen0::Irq27,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq27,
            dslpwupirqen0::Irq27,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq28,
        dslpwupirqen0::Irq28,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq28,
            dslpwupirqen0::Irq28,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq29,
        dslpwupirqen0::Irq29,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq29,
            dslpwupirqen0::Irq29,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq30,
        dslpwupirqen0::Irq30,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq30,
            dslpwupirqen0::Irq30,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dslpwupirqen0::Irq31,
        dslpwupirqen0::Irq31,
        Dslpwupirqen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dslpwupirqen0::Irq31,
            dslpwupirqen0::Irq31,
            Dslpwupirqen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dslpwupirqen0 {
    #[inline(always)]
    fn default() -> Dslpwupirqen0 {
        <crate::RegValueT<Dslpwupirqen0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dslpwupirqen0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq00_SPEC;
    pub type Irq00 = crate::EnumBitfieldStruct<u8, Irq00_SPEC>;
    impl Irq00 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq01_SPEC;
    pub type Irq01 = crate::EnumBitfieldStruct<u8, Irq01_SPEC>;
    impl Irq01 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq02_SPEC;
    pub type Irq02 = crate::EnumBitfieldStruct<u8, Irq02_SPEC>;
    impl Irq02 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq03_SPEC;
    pub type Irq03 = crate::EnumBitfieldStruct<u8, Irq03_SPEC>;
    impl Irq03 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq04_SPEC;
    pub type Irq04 = crate::EnumBitfieldStruct<u8, Irq04_SPEC>;
    impl Irq04 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq05_SPEC;
    pub type Irq05 = crate::EnumBitfieldStruct<u8, Irq05_SPEC>;
    impl Irq05 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq06_SPEC;
    pub type Irq06 = crate::EnumBitfieldStruct<u8, Irq06_SPEC>;
    impl Irq06 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq07_SPEC;
    pub type Irq07 = crate::EnumBitfieldStruct<u8, Irq07_SPEC>;
    impl Irq07 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq08_SPEC;
    pub type Irq08 = crate::EnumBitfieldStruct<u8, Irq08_SPEC>;
    impl Irq08 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq09_SPEC;
    pub type Irq09 = crate::EnumBitfieldStruct<u8, Irq09_SPEC>;
    impl Irq09 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq10_SPEC;
    pub type Irq10 = crate::EnumBitfieldStruct<u8, Irq10_SPEC>;
    impl Irq10 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq11_SPEC;
    pub type Irq11 = crate::EnumBitfieldStruct<u8, Irq11_SPEC>;
    impl Irq11 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq12_SPEC;
    pub type Irq12 = crate::EnumBitfieldStruct<u8, Irq12_SPEC>;
    impl Irq12 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq13_SPEC;
    pub type Irq13 = crate::EnumBitfieldStruct<u8, Irq13_SPEC>;
    impl Irq13 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq14_SPEC;
    pub type Irq14 = crate::EnumBitfieldStruct<u8, Irq14_SPEC>;
    impl Irq14 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq15_SPEC;
    pub type Irq15 = crate::EnumBitfieldStruct<u8, Irq15_SPEC>;
    impl Irq15 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq16_SPEC;
    pub type Irq16 = crate::EnumBitfieldStruct<u8, Irq16_SPEC>;
    impl Irq16 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq17_SPEC;
    pub type Irq17 = crate::EnumBitfieldStruct<u8, Irq17_SPEC>;
    impl Irq17 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq18_SPEC;
    pub type Irq18 = crate::EnumBitfieldStruct<u8, Irq18_SPEC>;
    impl Irq18 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq19_SPEC;
    pub type Irq19 = crate::EnumBitfieldStruct<u8, Irq19_SPEC>;
    impl Irq19 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq20_SPEC;
    pub type Irq20 = crate::EnumBitfieldStruct<u8, Irq20_SPEC>;
    impl Irq20 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq21_SPEC;
    pub type Irq21 = crate::EnumBitfieldStruct<u8, Irq21_SPEC>;
    impl Irq21 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq22_SPEC;
    pub type Irq22 = crate::EnumBitfieldStruct<u8, Irq22_SPEC>;
    impl Irq22 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq23_SPEC;
    pub type Irq23 = crate::EnumBitfieldStruct<u8, Irq23_SPEC>;
    impl Irq23 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq24_SPEC;
    pub type Irq24 = crate::EnumBitfieldStruct<u8, Irq24_SPEC>;
    impl Irq24 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq25_SPEC;
    pub type Irq25 = crate::EnumBitfieldStruct<u8, Irq25_SPEC>;
    impl Irq25 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq26_SPEC;
    pub type Irq26 = crate::EnumBitfieldStruct<u8, Irq26_SPEC>;
    impl Irq26 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq27_SPEC;
    pub type Irq27 = crate::EnumBitfieldStruct<u8, Irq27_SPEC>;
    impl Irq27 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq28_SPEC;
    pub type Irq28 = crate::EnumBitfieldStruct<u8, Irq28_SPEC>;
    impl Irq28 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq29_SPEC;
    pub type Irq29 = crate::EnumBitfieldStruct<u8, Irq29_SPEC>;
    impl Irq29 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq30_SPEC;
    pub type Irq30 = crate::EnumBitfieldStruct<u8, Irq30_SPEC>;
    impl Irq30 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq31_SPEC;
    pub type Irq31 = crate::EnumBitfieldStruct<u8, Irq31_SPEC>;
    impl Irq31 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dslpwupirqen1_SPEC;
impl crate::sealed::RegSpec for Dslpwupirqen1_SPEC {
    type DataType = u32;
}

#[doc = "Deep Sleep Wake Up IRQ Enable Register 1"]
pub type Dslpwupirqen1 = crate::RegValueT<Dslpwupirqen1_SPEC>;

impl Dslpwupirqen1 {
    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq32,
        dslpwupirqen1::Irq32,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq32,
            dslpwupirqen1::Irq32,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq33,
        dslpwupirqen1::Irq33,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq33,
            dslpwupirqen1::Irq33,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq34,
        dslpwupirqen1::Irq34,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq34,
            dslpwupirqen1::Irq34,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq35,
        dslpwupirqen1::Irq35,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq35,
            dslpwupirqen1::Irq35,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq36,
        dslpwupirqen1::Irq36,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq36,
            dslpwupirqen1::Irq36,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq37,
        dslpwupirqen1::Irq37,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq37,
            dslpwupirqen1::Irq37,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq38,
        dslpwupirqen1::Irq38,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq38,
            dslpwupirqen1::Irq38,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq39(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq39,
        dslpwupirqen1::Irq39,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq39,
            dslpwupirqen1::Irq39,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq40,
        dslpwupirqen1::Irq40,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq40,
            dslpwupirqen1::Irq40,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq41,
        dslpwupirqen1::Irq41,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq41,
            dslpwupirqen1::Irq41,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq42(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq42,
        dslpwupirqen1::Irq42,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq42,
            dslpwupirqen1::Irq42,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq43(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq43,
        dslpwupirqen1::Irq43,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq43,
            dslpwupirqen1::Irq43,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq44(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq44,
        dslpwupirqen1::Irq44,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq44,
            dslpwupirqen1::Irq44,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq45(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq45,
        dslpwupirqen1::Irq45,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq45,
            dslpwupirqen1::Irq45,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq46(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq46,
        dslpwupirqen1::Irq46,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq46,
            dslpwupirqen1::Irq46,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq47(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq47,
        dslpwupirqen1::Irq47,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq47,
            dslpwupirqen1::Irq47,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq48(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq48,
        dslpwupirqen1::Irq48,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq48,
            dslpwupirqen1::Irq48,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq49(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq49,
        dslpwupirqen1::Irq49,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq49,
            dslpwupirqen1::Irq49,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq50(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq50,
        dslpwupirqen1::Irq50,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq50,
            dslpwupirqen1::Irq50,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq51(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq51,
        dslpwupirqen1::Irq51,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq51,
            dslpwupirqen1::Irq51,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq52(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq52,
        dslpwupirqen1::Irq52,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq52,
            dslpwupirqen1::Irq52,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq53(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq53,
        dslpwupirqen1::Irq53,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq53,
            dslpwupirqen1::Irq53,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq54(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq54,
        dslpwupirqen1::Irq54,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq54,
            dslpwupirqen1::Irq54,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq55(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq55,
        dslpwupirqen1::Irq55,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq55,
            dslpwupirqen1::Irq55,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq56(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq56,
        dslpwupirqen1::Irq56,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq56,
            dslpwupirqen1::Irq56,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq57(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq57,
        dslpwupirqen1::Irq57,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq57,
            dslpwupirqen1::Irq57,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq58(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq58,
        dslpwupirqen1::Irq58,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq58,
            dslpwupirqen1::Irq58,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq59(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq59,
        dslpwupirqen1::Irq59,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq59,
            dslpwupirqen1::Irq59,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq60(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq60,
        dslpwupirqen1::Irq60,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq60,
            dslpwupirqen1::Irq60,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq61(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq61,
        dslpwupirqen1::Irq61,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq61,
            dslpwupirqen1::Irq61,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq62(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq62,
        dslpwupirqen1::Irq62,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq62,
            dslpwupirqen1::Irq62,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq63(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dslpwupirqen1::Irq63,
        dslpwupirqen1::Irq63,
        Dslpwupirqen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dslpwupirqen1::Irq63,
            dslpwupirqen1::Irq63,
            Dslpwupirqen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dslpwupirqen1 {
    #[inline(always)]
    fn default() -> Dslpwupirqen1 {
        <crate::RegValueT<Dslpwupirqen1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dslpwupirqen1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq32_SPEC;
    pub type Irq32 = crate::EnumBitfieldStruct<u8, Irq32_SPEC>;
    impl Irq32 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq33_SPEC;
    pub type Irq33 = crate::EnumBitfieldStruct<u8, Irq33_SPEC>;
    impl Irq33 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq34_SPEC;
    pub type Irq34 = crate::EnumBitfieldStruct<u8, Irq34_SPEC>;
    impl Irq34 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq35_SPEC;
    pub type Irq35 = crate::EnumBitfieldStruct<u8, Irq35_SPEC>;
    impl Irq35 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq36_SPEC;
    pub type Irq36 = crate::EnumBitfieldStruct<u8, Irq36_SPEC>;
    impl Irq36 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq37_SPEC;
    pub type Irq37 = crate::EnumBitfieldStruct<u8, Irq37_SPEC>;
    impl Irq37 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq38_SPEC;
    pub type Irq38 = crate::EnumBitfieldStruct<u8, Irq38_SPEC>;
    impl Irq38 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq39_SPEC;
    pub type Irq39 = crate::EnumBitfieldStruct<u8, Irq39_SPEC>;
    impl Irq39 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq40_SPEC;
    pub type Irq40 = crate::EnumBitfieldStruct<u8, Irq40_SPEC>;
    impl Irq40 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq41_SPEC;
    pub type Irq41 = crate::EnumBitfieldStruct<u8, Irq41_SPEC>;
    impl Irq41 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq42_SPEC;
    pub type Irq42 = crate::EnumBitfieldStruct<u8, Irq42_SPEC>;
    impl Irq42 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq43_SPEC;
    pub type Irq43 = crate::EnumBitfieldStruct<u8, Irq43_SPEC>;
    impl Irq43 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq44_SPEC;
    pub type Irq44 = crate::EnumBitfieldStruct<u8, Irq44_SPEC>;
    impl Irq44 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq45_SPEC;
    pub type Irq45 = crate::EnumBitfieldStruct<u8, Irq45_SPEC>;
    impl Irq45 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq46_SPEC;
    pub type Irq46 = crate::EnumBitfieldStruct<u8, Irq46_SPEC>;
    impl Irq46 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq47_SPEC;
    pub type Irq47 = crate::EnumBitfieldStruct<u8, Irq47_SPEC>;
    impl Irq47 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq48_SPEC;
    pub type Irq48 = crate::EnumBitfieldStruct<u8, Irq48_SPEC>;
    impl Irq48 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq49_SPEC;
    pub type Irq49 = crate::EnumBitfieldStruct<u8, Irq49_SPEC>;
    impl Irq49 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq50_SPEC;
    pub type Irq50 = crate::EnumBitfieldStruct<u8, Irq50_SPEC>;
    impl Irq50 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq51_SPEC;
    pub type Irq51 = crate::EnumBitfieldStruct<u8, Irq51_SPEC>;
    impl Irq51 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq52_SPEC;
    pub type Irq52 = crate::EnumBitfieldStruct<u8, Irq52_SPEC>;
    impl Irq52 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq53_SPEC;
    pub type Irq53 = crate::EnumBitfieldStruct<u8, Irq53_SPEC>;
    impl Irq53 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq54_SPEC;
    pub type Irq54 = crate::EnumBitfieldStruct<u8, Irq54_SPEC>;
    impl Irq54 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq55_SPEC;
    pub type Irq55 = crate::EnumBitfieldStruct<u8, Irq55_SPEC>;
    impl Irq55 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq56_SPEC;
    pub type Irq56 = crate::EnumBitfieldStruct<u8, Irq56_SPEC>;
    impl Irq56 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq57_SPEC;
    pub type Irq57 = crate::EnumBitfieldStruct<u8, Irq57_SPEC>;
    impl Irq57 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq58_SPEC;
    pub type Irq58 = crate::EnumBitfieldStruct<u8, Irq58_SPEC>;
    impl Irq58 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq59_SPEC;
    pub type Irq59 = crate::EnumBitfieldStruct<u8, Irq59_SPEC>;
    impl Irq59 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq60_SPEC;
    pub type Irq60 = crate::EnumBitfieldStruct<u8, Irq60_SPEC>;
    impl Irq60 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq61_SPEC;
    pub type Irq61 = crate::EnumBitfieldStruct<u8, Irq61_SPEC>;
    impl Irq61 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq62_SPEC;
    pub type Irq62 = crate::EnumBitfieldStruct<u8, Irq62_SPEC>;
    impl Irq62 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq63_SPEC;
    pub type Irq63 = crate::EnumBitfieldStruct<u8, Irq63_SPEC>;
    impl Irq63 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dslpwupirqen2_SPEC;
impl crate::sealed::RegSpec for Dslpwupirqen2_SPEC {
    type DataType = u32;
}

#[doc = "Deep Sleep Wake Up IRQ Enable Register 2"]
pub type Dslpwupirqen2 = crate::RegValueT<Dslpwupirqen2_SPEC>;

impl Dslpwupirqen2 {
    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq64(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq64,
        dslpwupirqen2::Irq64,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq64,
            dslpwupirqen2::Irq64,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq65(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq65,
        dslpwupirqen2::Irq65,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq65,
            dslpwupirqen2::Irq65,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq66(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq66,
        dslpwupirqen2::Irq66,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq66,
            dslpwupirqen2::Irq66,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq67(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq67,
        dslpwupirqen2::Irq67,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq67,
            dslpwupirqen2::Irq67,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq68(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq68,
        dslpwupirqen2::Irq68,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq68,
            dslpwupirqen2::Irq68,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq69(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq69,
        dslpwupirqen2::Irq69,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq69,
            dslpwupirqen2::Irq69,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq70(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq70,
        dslpwupirqen2::Irq70,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq70,
            dslpwupirqen2::Irq70,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq71(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq71,
        dslpwupirqen2::Irq71,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq71,
            dslpwupirqen2::Irq71,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq72(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq72,
        dslpwupirqen2::Irq72,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq72,
            dslpwupirqen2::Irq72,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq73(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq73,
        dslpwupirqen2::Irq73,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq73,
            dslpwupirqen2::Irq73,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq74(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq74,
        dslpwupirqen2::Irq74,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq74,
            dslpwupirqen2::Irq74,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq75(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq75,
        dslpwupirqen2::Irq75,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq75,
            dslpwupirqen2::Irq75,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq76(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq76,
        dslpwupirqen2::Irq76,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq76,
            dslpwupirqen2::Irq76,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq77(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq77,
        dslpwupirqen2::Irq77,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq77,
            dslpwupirqen2::Irq77,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq78(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq78,
        dslpwupirqen2::Irq78,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq78,
            dslpwupirqen2::Irq78,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq79(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq79,
        dslpwupirqen2::Irq79,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq79,
            dslpwupirqen2::Irq79,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq80(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq80,
        dslpwupirqen2::Irq80,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq80,
            dslpwupirqen2::Irq80,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq81(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq81,
        dslpwupirqen2::Irq81,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq81,
            dslpwupirqen2::Irq81,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq82(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq82,
        dslpwupirqen2::Irq82,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq82,
            dslpwupirqen2::Irq82,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq83(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq83,
        dslpwupirqen2::Irq83,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq83,
            dslpwupirqen2::Irq83,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq84(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq84,
        dslpwupirqen2::Irq84,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq84,
            dslpwupirqen2::Irq84,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq85(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq85,
        dslpwupirqen2::Irq85,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq85,
            dslpwupirqen2::Irq85,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq86(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq86,
        dslpwupirqen2::Irq86,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq86,
            dslpwupirqen2::Irq86,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq87(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq87,
        dslpwupirqen2::Irq87,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq87,
            dslpwupirqen2::Irq87,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq88(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq88,
        dslpwupirqen2::Irq88,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq88,
            dslpwupirqen2::Irq88,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq89(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq89,
        dslpwupirqen2::Irq89,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq89,
            dslpwupirqen2::Irq89,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq90(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq90,
        dslpwupirqen2::Irq90,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq90,
            dslpwupirqen2::Irq90,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq91(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq91,
        dslpwupirqen2::Irq91,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq91,
            dslpwupirqen2::Irq91,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq92(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq92,
        dslpwupirqen2::Irq92,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq92,
            dslpwupirqen2::Irq92,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq93(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq93,
        dslpwupirqen2::Irq93,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq93,
            dslpwupirqen2::Irq93,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq94(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq94,
        dslpwupirqen2::Irq94,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq94,
            dslpwupirqen2::Irq94,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IRQ Deep Sleep Returns Enable bit."]
    #[inline(always)]
    pub fn irq95(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dslpwupirqen2::Irq95,
        dslpwupirqen2::Irq95,
        Dslpwupirqen2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dslpwupirqen2::Irq95,
            dslpwupirqen2::Irq95,
            Dslpwupirqen2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dslpwupirqen2 {
    #[inline(always)]
    fn default() -> Dslpwupirqen2 {
        <crate::RegValueT<Dslpwupirqen2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dslpwupirqen2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq64_SPEC;
    pub type Irq64 = crate::EnumBitfieldStruct<u8, Irq64_SPEC>;
    impl Irq64 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq65_SPEC;
    pub type Irq65 = crate::EnumBitfieldStruct<u8, Irq65_SPEC>;
    impl Irq65 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq66_SPEC;
    pub type Irq66 = crate::EnumBitfieldStruct<u8, Irq66_SPEC>;
    impl Irq66 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq67_SPEC;
    pub type Irq67 = crate::EnumBitfieldStruct<u8, Irq67_SPEC>;
    impl Irq67 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq68_SPEC;
    pub type Irq68 = crate::EnumBitfieldStruct<u8, Irq68_SPEC>;
    impl Irq68 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq69_SPEC;
    pub type Irq69 = crate::EnumBitfieldStruct<u8, Irq69_SPEC>;
    impl Irq69 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq70_SPEC;
    pub type Irq70 = crate::EnumBitfieldStruct<u8, Irq70_SPEC>;
    impl Irq70 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq71_SPEC;
    pub type Irq71 = crate::EnumBitfieldStruct<u8, Irq71_SPEC>;
    impl Irq71 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq72_SPEC;
    pub type Irq72 = crate::EnumBitfieldStruct<u8, Irq72_SPEC>;
    impl Irq72 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq73_SPEC;
    pub type Irq73 = crate::EnumBitfieldStruct<u8, Irq73_SPEC>;
    impl Irq73 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq74_SPEC;
    pub type Irq74 = crate::EnumBitfieldStruct<u8, Irq74_SPEC>;
    impl Irq74 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq75_SPEC;
    pub type Irq75 = crate::EnumBitfieldStruct<u8, Irq75_SPEC>;
    impl Irq75 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq76_SPEC;
    pub type Irq76 = crate::EnumBitfieldStruct<u8, Irq76_SPEC>;
    impl Irq76 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq77_SPEC;
    pub type Irq77 = crate::EnumBitfieldStruct<u8, Irq77_SPEC>;
    impl Irq77 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq78_SPEC;
    pub type Irq78 = crate::EnumBitfieldStruct<u8, Irq78_SPEC>;
    impl Irq78 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq79_SPEC;
    pub type Irq79 = crate::EnumBitfieldStruct<u8, Irq79_SPEC>;
    impl Irq79 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq80_SPEC;
    pub type Irq80 = crate::EnumBitfieldStruct<u8, Irq80_SPEC>;
    impl Irq80 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq81_SPEC;
    pub type Irq81 = crate::EnumBitfieldStruct<u8, Irq81_SPEC>;
    impl Irq81 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq82_SPEC;
    pub type Irq82 = crate::EnumBitfieldStruct<u8, Irq82_SPEC>;
    impl Irq82 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq83_SPEC;
    pub type Irq83 = crate::EnumBitfieldStruct<u8, Irq83_SPEC>;
    impl Irq83 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq84_SPEC;
    pub type Irq84 = crate::EnumBitfieldStruct<u8, Irq84_SPEC>;
    impl Irq84 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq85_SPEC;
    pub type Irq85 = crate::EnumBitfieldStruct<u8, Irq85_SPEC>;
    impl Irq85 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq86_SPEC;
    pub type Irq86 = crate::EnumBitfieldStruct<u8, Irq86_SPEC>;
    impl Irq86 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq87_SPEC;
    pub type Irq87 = crate::EnumBitfieldStruct<u8, Irq87_SPEC>;
    impl Irq87 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq88_SPEC;
    pub type Irq88 = crate::EnumBitfieldStruct<u8, Irq88_SPEC>;
    impl Irq88 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq89_SPEC;
    pub type Irq89 = crate::EnumBitfieldStruct<u8, Irq89_SPEC>;
    impl Irq89 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq90_SPEC;
    pub type Irq90 = crate::EnumBitfieldStruct<u8, Irq90_SPEC>;
    impl Irq90 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq91_SPEC;
    pub type Irq91 = crate::EnumBitfieldStruct<u8, Irq91_SPEC>;
    impl Irq91 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq92_SPEC;
    pub type Irq92 = crate::EnumBitfieldStruct<u8, Irq92_SPEC>;
    impl Irq92 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq93_SPEC;
    pub type Irq93 = crate::EnumBitfieldStruct<u8, Irq93_SPEC>;
    impl Irq93 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq94_SPEC;
    pub type Irq94 = crate::EnumBitfieldStruct<u8, Irq94_SPEC>;
    impl Irq94 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irq95_SPEC;
    pub type Irq95 = crate::EnumBitfieldStruct<u8, Irq95_SPEC>;
    impl Irq95 {
        #[doc = "Deep Sleep returns by IRQ event is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Deep Sleep returns by IRQ event is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DelsRm_SPEC;
impl crate::sealed::RegSpec for DelsRm_SPEC {
    type DataType = u32;
}

#[doc = "DMAC Event Link Setting Register m (m = 0 to 7)"]
pub type DelsRm = crate::RegValueT<DelsRm_SPEC>;

impl DelsRm {
    #[doc = "DMAC Event Link Select"]
    #[inline(always)]
    pub fn dels(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        delsrm::Dels,
        delsrm::Dels,
        DelsRm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            delsrm::Dels,
            delsrm::Dels,
            DelsRm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMAC Activation Request Status flag"]
    #[inline(always)]
    pub fn ir(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        delsrm::Ir,
        delsrm::Ir,
        DelsRm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            delsrm::Ir,
            delsrm::Ir,
            DelsRm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DelsRm {
    #[inline(always)]
    fn default() -> DelsRm {
        <crate::RegValueT<DelsRm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod delsrm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dels_SPEC;
    pub type Dels = crate::EnumBitfieldStruct<u8, Dels_SPEC>;
    impl Dels {
        #[doc = "Disable interrupts to the associated DMAC module"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Event signal number to be linked. For details, see ."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ir_SPEC;
    pub type Ir = crate::EnumBitfieldStruct<u8, Ir_SPEC>;
    impl Ir {
        #[doc = "No DMAC activation request occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC activation request occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ielsr_SPEC;
impl crate::sealed::RegSpec for Ielsr_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Unit Event Link Setting Register %s"]
pub type Ielsr = crate::RegValueT<Ielsr_SPEC>;

impl Ielsr {
    #[doc = "ICU Event Link Select"]
    #[inline(always)]
    pub fn iels(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        ielsr::Iels,
        ielsr::Iels,
        Ielsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            ielsr::Iels,
            ielsr::Iels,
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
}
impl ::core::default::Default for Ielsr {
    #[inline(always)]
    fn default() -> Ielsr {
        <crate::RegValueT<Ielsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ielsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iels_SPEC;
    pub type Iels = crate::EnumBitfieldStruct<u8, Iels_SPEC>;
    impl Iels {
        #[doc = "Disable interrupts to the associated NVIC or DTC module"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Event signal number to be linked. For details, see ."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ir_SPEC;
    pub type Ir = crate::EnumBitfieldStruct<u8, Ir_SPEC>;
    impl Ir {
        #[doc = "No interrupt request generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "An interrupt request is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtce_SPEC;
    pub type Dtce = crate::EnumBitfieldStruct<u8, Dtce_SPEC>;
    impl Dtce {
        #[doc = "DTC activation is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "DTC activation is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
