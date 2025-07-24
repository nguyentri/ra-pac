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
unsafe impl ::core::marker::Send for super::IcuCommonNs {}
unsafe impl ::core::marker::Sync for super::IcuCommonNs {}
impl super::IcuCommonNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "IRQ Control Register"]
    #[inline(always)]
    pub const fn irqcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Irqcr_SPEC, crate::common::RW>,
        32,
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
    #[inline(always)]
    pub const fn irqcr16(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr17(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr18(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr19(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr20(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr21(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x15usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr22(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr23(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x17usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr24(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr25(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x19usize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr26(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr27(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1busize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr28(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr29(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr30(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn irqcr31(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1fusize),
            )
        }
    }

    #[doc = "NMI Pin Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmicr(&self) -> &'static crate::common::Reg<self::Nmicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Request Select Register"]
    #[inline(always)]
    pub const fn intselr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Intselr_SPEC, crate::common::RW>,
        32,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn intselr0(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr1(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr2(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr3(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr4(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr5(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr6(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr7(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr8(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr9(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr10(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr11(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr12(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr13(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr14(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr15(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr16(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr17(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr18(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr19(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr20(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr21(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr22(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr23(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr24(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr25(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr26(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr27(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr28(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr29(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr30(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn intselr31(
        &self,
    ) -> &'static crate::common::Reg<self::Intselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
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

#[doc = "IRQ Control Register"]
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
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intselr_SPEC;
impl crate::sealed::RegSpec for Intselr_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Request Select Register"]
pub type Intselr = crate::RegValueT<Intselr_SPEC>;

impl Intselr {
    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p000(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intselr::Is32P000,
        intselr::Is32P000,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intselr::Is32P000,
            intselr::Is32P000,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p001(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        intselr::Is32P001,
        intselr::Is32P001,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            intselr::Is32P001,
            intselr::Is32P001,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p002(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        intselr::Is32P002,
        intselr::Is32P002,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            intselr::Is32P002,
            intselr::Is32P002,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p003(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        intselr::Is32P003,
        intselr::Is32P003,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            intselr::Is32P003,
            intselr::Is32P003,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p004(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        intselr::Is32P004,
        intselr::Is32P004,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            intselr::Is32P004,
            intselr::Is32P004,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p005(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        intselr::Is32P005,
        intselr::Is32P005,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            intselr::Is32P005,
            intselr::Is32P005,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p006(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        intselr::Is32P006,
        intselr::Is32P006,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            intselr::Is32P006,
            intselr::Is32P006,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p007(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        intselr::Is32P007,
        intselr::Is32P007,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            intselr::Is32P007,
            intselr::Is32P007,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p008(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        intselr::Is32P008,
        intselr::Is32P008,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            intselr::Is32P008,
            intselr::Is32P008,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p009(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        intselr::Is32P009,
        intselr::Is32P009,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            intselr::Is32P009,
            intselr::Is32P009,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p010(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        intselr::Is32P010,
        intselr::Is32P010,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            intselr::Is32P010,
            intselr::Is32P010,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p011(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        intselr::Is32P011,
        intselr::Is32P011,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            intselr::Is32P011,
            intselr::Is32P011,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p012(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        intselr::Is32P012,
        intselr::Is32P012,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            intselr::Is32P012,
            intselr::Is32P012,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p013(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        intselr::Is32P013,
        intselr::Is32P013,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            intselr::Is32P013,
            intselr::Is32P013,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p014(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        intselr::Is32P014,
        intselr::Is32P014,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            intselr::Is32P014,
            intselr::Is32P014,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p015(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        intselr::Is32P015,
        intselr::Is32P015,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            intselr::Is32P015,
            intselr::Is32P015,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p016(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        intselr::Is32P016,
        intselr::Is32P016,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            intselr::Is32P016,
            intselr::Is32P016,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p017(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        intselr::Is32P017,
        intselr::Is32P017,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            intselr::Is32P017,
            intselr::Is32P017,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p018(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        intselr::Is32P018,
        intselr::Is32P018,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            intselr::Is32P018,
            intselr::Is32P018,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p019(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        intselr::Is32P019,
        intselr::Is32P019,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            intselr::Is32P019,
            intselr::Is32P019,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p020(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        intselr::Is32P020,
        intselr::Is32P020,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            intselr::Is32P020,
            intselr::Is32P020,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p021(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        intselr::Is32P021,
        intselr::Is32P021,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            intselr::Is32P021,
            intselr::Is32P021,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p022(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        intselr::Is32P022,
        intselr::Is32P022,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            intselr::Is32P022,
            intselr::Is32P022,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p023(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        intselr::Is32P023,
        intselr::Is32P023,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            intselr::Is32P023,
            intselr::Is32P023,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p024(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        intselr::Is32P024,
        intselr::Is32P024,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            intselr::Is32P024,
            intselr::Is32P024,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p025(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        intselr::Is32P025,
        intselr::Is32P025,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            intselr::Is32P025,
            intselr::Is32P025,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p026(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        intselr::Is32P026,
        intselr::Is32P026,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            intselr::Is32P026,
            intselr::Is32P026,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p027(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        intselr::Is32P027,
        intselr::Is32P027,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            intselr::Is32P027,
            intselr::Is32P027,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p028(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        intselr::Is32P028,
        intselr::Is32P028,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            intselr::Is32P028,
            intselr::Is32P028,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p029(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        intselr::Is32P029,
        intselr::Is32P029,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            intselr::Is32P029,
            intselr::Is32P029,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p030(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        intselr::Is32P030,
        intselr::Is32P030,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            intselr::Is32P030,
            intselr::Is32P030,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit sets whether the interrupt requests and DTC requests are on the CPU0 side or the CPU1 side."]
    #[inline(always)]
    pub fn is32p031(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        intselr::Is32P031,
        intselr::Is32P031,
        Intselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            intselr::Is32P031,
            intselr::Is32P031,
            Intselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intselr {
    #[inline(always)]
    fn default() -> Intselr {
        <crate::RegValueT<Intselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P000_SPEC;
    pub type Is32P000 = crate::EnumBitfieldStruct<u8, Is32P000_SPEC>;
    impl Is32P000 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P001_SPEC;
    pub type Is32P001 = crate::EnumBitfieldStruct<u8, Is32P001_SPEC>;
    impl Is32P001 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P002_SPEC;
    pub type Is32P002 = crate::EnumBitfieldStruct<u8, Is32P002_SPEC>;
    impl Is32P002 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P003_SPEC;
    pub type Is32P003 = crate::EnumBitfieldStruct<u8, Is32P003_SPEC>;
    impl Is32P003 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P004_SPEC;
    pub type Is32P004 = crate::EnumBitfieldStruct<u8, Is32P004_SPEC>;
    impl Is32P004 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P005_SPEC;
    pub type Is32P005 = crate::EnumBitfieldStruct<u8, Is32P005_SPEC>;
    impl Is32P005 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P006_SPEC;
    pub type Is32P006 = crate::EnumBitfieldStruct<u8, Is32P006_SPEC>;
    impl Is32P006 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P007_SPEC;
    pub type Is32P007 = crate::EnumBitfieldStruct<u8, Is32P007_SPEC>;
    impl Is32P007 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P008_SPEC;
    pub type Is32P008 = crate::EnumBitfieldStruct<u8, Is32P008_SPEC>;
    impl Is32P008 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P009_SPEC;
    pub type Is32P009 = crate::EnumBitfieldStruct<u8, Is32P009_SPEC>;
    impl Is32P009 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P010_SPEC;
    pub type Is32P010 = crate::EnumBitfieldStruct<u8, Is32P010_SPEC>;
    impl Is32P010 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P011_SPEC;
    pub type Is32P011 = crate::EnumBitfieldStruct<u8, Is32P011_SPEC>;
    impl Is32P011 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P012_SPEC;
    pub type Is32P012 = crate::EnumBitfieldStruct<u8, Is32P012_SPEC>;
    impl Is32P012 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P013_SPEC;
    pub type Is32P013 = crate::EnumBitfieldStruct<u8, Is32P013_SPEC>;
    impl Is32P013 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P014_SPEC;
    pub type Is32P014 = crate::EnumBitfieldStruct<u8, Is32P014_SPEC>;
    impl Is32P014 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P015_SPEC;
    pub type Is32P015 = crate::EnumBitfieldStruct<u8, Is32P015_SPEC>;
    impl Is32P015 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P016_SPEC;
    pub type Is32P016 = crate::EnumBitfieldStruct<u8, Is32P016_SPEC>;
    impl Is32P016 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P017_SPEC;
    pub type Is32P017 = crate::EnumBitfieldStruct<u8, Is32P017_SPEC>;
    impl Is32P017 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P018_SPEC;
    pub type Is32P018 = crate::EnumBitfieldStruct<u8, Is32P018_SPEC>;
    impl Is32P018 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P019_SPEC;
    pub type Is32P019 = crate::EnumBitfieldStruct<u8, Is32P019_SPEC>;
    impl Is32P019 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P020_SPEC;
    pub type Is32P020 = crate::EnumBitfieldStruct<u8, Is32P020_SPEC>;
    impl Is32P020 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P021_SPEC;
    pub type Is32P021 = crate::EnumBitfieldStruct<u8, Is32P021_SPEC>;
    impl Is32P021 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P022_SPEC;
    pub type Is32P022 = crate::EnumBitfieldStruct<u8, Is32P022_SPEC>;
    impl Is32P022 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P023_SPEC;
    pub type Is32P023 = crate::EnumBitfieldStruct<u8, Is32P023_SPEC>;
    impl Is32P023 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P024_SPEC;
    pub type Is32P024 = crate::EnumBitfieldStruct<u8, Is32P024_SPEC>;
    impl Is32P024 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P025_SPEC;
    pub type Is32P025 = crate::EnumBitfieldStruct<u8, Is32P025_SPEC>;
    impl Is32P025 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P026_SPEC;
    pub type Is32P026 = crate::EnumBitfieldStruct<u8, Is32P026_SPEC>;
    impl Is32P026 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P027_SPEC;
    pub type Is32P027 = crate::EnumBitfieldStruct<u8, Is32P027_SPEC>;
    impl Is32P027 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P028_SPEC;
    pub type Is32P028 = crate::EnumBitfieldStruct<u8, Is32P028_SPEC>;
    impl Is32P028 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P029_SPEC;
    pub type Is32P029 = crate::EnumBitfieldStruct<u8, Is32P029_SPEC>;
    impl Is32P029 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P030_SPEC;
    pub type Is32P030 = crate::EnumBitfieldStruct<u8, Is32P030_SPEC>;
    impl Is32P030 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Is32P031_SPEC;
    pub type Is32P031 = crate::EnumBitfieldStruct<u8, Is32P031_SPEC>;
    impl Is32P031 {
        #[doc = "Selected as CPU0 factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Selected as CPU1 factor"]
        pub const _1: Self = Self::new(1);
    }
}
