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
#[doc = r"Event Link Controller"]
unsafe impl ::core::marker::Send for super::Elc {}
unsafe impl ::core::marker::Sync for super::Elc {}
impl super::Elc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Event Link Controller Register"]
    #[inline(always)]
    pub const fn elcr(&self) -> &'static crate::common::Reg<self::Elcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub const fn elsegr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Elsegr_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4usize))
        }
    }
    #[inline(always)]
    pub const fn elsegr0(
        &self,
    ) -> &'static crate::common::Reg<self::Elsegr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsegr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsegr1(
        &self,
    ) -> &'static crate::common::Reg<self::Elsegr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsegr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsegr2(
        &self,
    ) -> &'static crate::common::Reg<self::Elsegr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsegr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsegr3(
        &self,
    ) -> &'static crate::common::Reg<self::Elsegr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsegr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }

    #[doc = "Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn elsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Elsr_SPEC, crate::common::RW>,
        53,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }
    #[inline(always)]
    pub const fn elsr0(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr1(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr2(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr3(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr4(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr5(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr6(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr7(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr8(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr9(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr10(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr11(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr12(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr13(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr14(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr15(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr16(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr17(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr18(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr19(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr20(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr21(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr22(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr23(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr24(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr25(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr26(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr27(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr28(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr29(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr30(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr31(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr32(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr33(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr34(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr35(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr36(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr37(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr38(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr39(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr40(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr41(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr42(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr43(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr44(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr45(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr46(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr47(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr48(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr49(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr50(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr51(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn elsr52(&self) -> &'static crate::common::Reg<self::Elsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xf0usize),
            )
        }
    }

    #[doc = "Event Link Controller Security Attribution Register A"]
    #[inline(always)]
    pub const fn elcsara(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Event Link Controller Security Attribution Register B"]
    #[inline(always)]
    pub const fn elcsarb(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Event Link Controller Security Attribution Register C"]
    #[inline(always)]
    pub const fn elcsarc(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsarc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsarc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Event Link Controller Privilege Attribution Register A"]
    #[inline(always)]
    pub const fn elcpara(
        &self,
    ) -> &'static crate::common::Reg<self::Elcpara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcpara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Event Link Controller Privilege Attribution Register B"]
    #[inline(always)]
    pub const fn elcparb(
        &self,
    ) -> &'static crate::common::Reg<self::Elcparb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcparb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "Event Link Controller Privilege Attribution Register C"]
    #[inline(always)]
    pub const fn elcparc(
        &self,
    ) -> &'static crate::common::Reg<self::Elcparc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcparc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcr_SPEC;
impl crate::sealed::RegSpec for Elcr_SPEC {
    type DataType = u8;
}

#[doc = "Event Link Controller Register"]
pub type Elcr = crate::RegValueT<Elcr_SPEC>;

impl Elcr {
    #[doc = "All Event Link Enable"]
    #[inline(always)]
    pub fn elcon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        elcr::Elcon,
        elcr::Elcon,
        Elcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            elcr::Elcon,
            elcr::Elcon,
            Elcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcr {
    #[inline(always)]
    fn default() -> Elcr {
        <crate::RegValueT<Elcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elcon_SPEC;
    pub type Elcon = crate::EnumBitfieldStruct<u8, Elcon_SPEC>;
    impl Elcon {
        #[doc = "ELC function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "ELC function is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsegr_SPEC;
impl crate::sealed::RegSpec for Elsegr_SPEC {
    type DataType = u8;
}

#[doc = "Event Link Software Event Generation Register %s"]
pub type Elsegr = crate::RegValueT<Elsegr_SPEC>;

impl Elsegr {
    #[doc = "Software Event Generation"]
    #[inline(always)]
    pub fn seg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elsegr::Seg,
        elsegr::Seg,
        Elsegr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elsegr::Seg,
            elsegr::Seg,
            Elsegr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SEG Bit Write Enable"]
    #[inline(always)]
    pub fn we(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        elsegr::We,
        elsegr::We,
        Elsegr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            elsegr::We,
            elsegr::We,
            Elsegr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELSEGR Register Write Disable"]
    #[inline(always)]
    pub fn wi(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        elsegr::Wi,
        elsegr::Wi,
        Elsegr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            elsegr::Wi,
            elsegr::Wi,
            Elsegr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elsegr {
    #[inline(always)]
    fn default() -> Elsegr {
        <crate::RegValueT<Elsegr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod elsegr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seg_SPEC;
    pub type Seg = crate::EnumBitfieldStruct<u8, Seg_SPEC>;
    impl Seg {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Software event is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct We_SPEC;
    pub type We = crate::EnumBitfieldStruct<u8, We_SPEC>;
    impl We {
        #[doc = "Write to SEG bit disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to SEG bit enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wi_SPEC;
    pub type Wi = crate::EnumBitfieldStruct<u8, Wi_SPEC>;
    impl Wi {
        #[doc = "Write to ELSEGR register enabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Write to ELSEGR register disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr_SPEC;
impl crate::sealed::RegSpec for Elsr_SPEC {
    type DataType = u16;
}

#[doc = "Event Link Setting Register %s"]
pub type Elsr = crate::RegValueT<Elsr_SPEC>;

impl Elsr {
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub fn els(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Elsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Elsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elsr {
    #[inline(always)]
    fn default() -> Elsr {
        <crate::RegValueT<Elsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsara_SPEC;
impl crate::sealed::RegSpec for Elcsara_SPEC {
    type DataType = u32;
}

#[doc = "Event Link Controller Security Attribution Register A"]
pub type Elcsara = crate::RegValueT<Elcsara_SPEC>;

impl Elcsara {
    #[doc = "Event Link Controller Register Security Attribution"]
    #[inline(always)]
    pub fn elcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elcsara::Elcr,
        elcsara::Elcr,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elcsara::Elcr,
            elcsara::Elcr,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 0 Security Attribution"]
    #[inline(always)]
    pub fn elsegr0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        elcsara::Elsegr0,
        elcsara::Elsegr0,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            elcsara::Elsegr0,
            elcsara::Elsegr0,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 1 Security Attribution"]
    #[inline(always)]
    pub fn elsegr1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        elcsara::Elsegr1,
        elcsara::Elsegr1,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elcsara::Elsegr1,
            elcsara::Elsegr1,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 2 Security Attribution"]
    #[inline(always)]
    pub fn elsegr2(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        elcsara::Elsegr2,
        elcsara::Elsegr2,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            elcsara::Elsegr2,
            elcsara::Elsegr2,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 3 Security Attribution"]
    #[inline(always)]
    pub fn elsegr3(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        elcsara::Elsegr3,
        elcsara::Elsegr3,
        Elcsara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            elcsara::Elsegr3,
            elcsara::Elsegr3,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcsara {
    #[inline(always)]
    fn default() -> Elcsara {
        <crate::RegValueT<Elcsara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcsara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elcr_SPEC;
    pub type Elcr = crate::EnumBitfieldStruct<u8, Elcr_SPEC>;
    impl Elcr {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr0_SPEC;
    pub type Elsegr0 = crate::EnumBitfieldStruct<u8, Elsegr0_SPEC>;
    impl Elsegr0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr1_SPEC;
    pub type Elsegr1 = crate::EnumBitfieldStruct<u8, Elsegr1_SPEC>;
    impl Elsegr1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr2_SPEC;
    pub type Elsegr2 = crate::EnumBitfieldStruct<u8, Elsegr2_SPEC>;
    impl Elsegr2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr3_SPEC;
    pub type Elsegr3 = crate::EnumBitfieldStruct<u8, Elsegr3_SPEC>;
    impl Elsegr3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsarb_SPEC;
impl crate::sealed::RegSpec for Elcsarb_SPEC {
    type DataType = u32;
}

#[doc = "Event Link Controller Security Attribution Register B"]
pub type Elcsarb = crate::RegValueT<Elcsarb_SPEC>;

impl Elcsarb {
    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elcsarb::Elsr0,
        elcsarb::Elsr0,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elcsarb::Elsr0,
            elcsarb::Elsr0,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        elcsarb::Elsr1,
        elcsarb::Elsr1,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            elcsarb::Elsr1,
            elcsarb::Elsr1,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        elcsarb::Elsr2,
        elcsarb::Elsr2,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elcsarb::Elsr2,
            elcsarb::Elsr2,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        elcsarb::Elsr3,
        elcsarb::Elsr3,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            elcsarb::Elsr3,
            elcsarb::Elsr3,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        elcsarb::Elsr4,
        elcsarb::Elsr4,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            elcsarb::Elsr4,
            elcsarb::Elsr4,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        elcsarb::Elsr5,
        elcsarb::Elsr5,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            elcsarb::Elsr5,
            elcsarb::Elsr5,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        elcsarb::Elsr6,
        elcsarb::Elsr6,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            elcsarb::Elsr6,
            elcsarb::Elsr6,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        elcsarb::Elsr7,
        elcsarb::Elsr7,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            elcsarb::Elsr7,
            elcsarb::Elsr7,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        elcsarb::Elsr12,
        elcsarb::Elsr12,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            elcsarb::Elsr12,
            elcsarb::Elsr12,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        elcsarb::Elsr13,
        elcsarb::Elsr13,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            elcsarb::Elsr13,
            elcsarb::Elsr13,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        elcsarb::Elsr14,
        elcsarb::Elsr14,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            elcsarb::Elsr14,
            elcsarb::Elsr14,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        elcsarb::Elsr15,
        elcsarb::Elsr15,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            elcsarb::Elsr15,
            elcsarb::Elsr15,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        elcsarb::Elsr16,
        elcsarb::Elsr16,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            elcsarb::Elsr16,
            elcsarb::Elsr16,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        elcsarb::Elsr17,
        elcsarb::Elsr17,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            elcsarb::Elsr17,
            elcsarb::Elsr17,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        elcsarb::Elsr19,
        elcsarb::Elsr19,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            elcsarb::Elsr19,
            elcsarb::Elsr19,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        elcsarb::Elsr20,
        elcsarb::Elsr20,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            elcsarb::Elsr20,
            elcsarb::Elsr20,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        elcsarb::Elsr21,
        elcsarb::Elsr21,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            elcsarb::Elsr21,
            elcsarb::Elsr21,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        elcsarb::Elsr22,
        elcsarb::Elsr22,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            elcsarb::Elsr22,
            elcsarb::Elsr22,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        elcsarb::Elsr23,
        elcsarb::Elsr23,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            elcsarb::Elsr23,
            elcsarb::Elsr23,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        elcsarb::Elsr24,
        elcsarb::Elsr24,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            elcsarb::Elsr24,
            elcsarb::Elsr24,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        elcsarb::Elsr25,
        elcsarb::Elsr25,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            elcsarb::Elsr25,
            elcsarb::Elsr25,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        elcsarb::Elsr26,
        elcsarb::Elsr26,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            elcsarb::Elsr26,
            elcsarb::Elsr26,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        elcsarb::Elsr27,
        elcsarb::Elsr27,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            elcsarb::Elsr27,
            elcsarb::Elsr27,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        elcsarb::Elsr30,
        elcsarb::Elsr30,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            elcsarb::Elsr30,
            elcsarb::Elsr30,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        elcsarb::Elsr31,
        elcsarb::Elsr31,
        Elcsarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            elcsarb::Elsr31,
            elcsarb::Elsr31,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcsarb {
    #[inline(always)]
    fn default() -> Elcsarb {
        <crate::RegValueT<Elcsarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcsarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr0_SPEC;
    pub type Elsr0 = crate::EnumBitfieldStruct<u8, Elsr0_SPEC>;
    impl Elsr0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr1_SPEC;
    pub type Elsr1 = crate::EnumBitfieldStruct<u8, Elsr1_SPEC>;
    impl Elsr1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr2_SPEC;
    pub type Elsr2 = crate::EnumBitfieldStruct<u8, Elsr2_SPEC>;
    impl Elsr2 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr3_SPEC;
    pub type Elsr3 = crate::EnumBitfieldStruct<u8, Elsr3_SPEC>;
    impl Elsr3 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr4_SPEC;
    pub type Elsr4 = crate::EnumBitfieldStruct<u8, Elsr4_SPEC>;
    impl Elsr4 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr5_SPEC;
    pub type Elsr5 = crate::EnumBitfieldStruct<u8, Elsr5_SPEC>;
    impl Elsr5 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr6_SPEC;
    pub type Elsr6 = crate::EnumBitfieldStruct<u8, Elsr6_SPEC>;
    impl Elsr6 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr7_SPEC;
    pub type Elsr7 = crate::EnumBitfieldStruct<u8, Elsr7_SPEC>;
    impl Elsr7 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr12_SPEC;
    pub type Elsr12 = crate::EnumBitfieldStruct<u8, Elsr12_SPEC>;
    impl Elsr12 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr13_SPEC;
    pub type Elsr13 = crate::EnumBitfieldStruct<u8, Elsr13_SPEC>;
    impl Elsr13 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr14_SPEC;
    pub type Elsr14 = crate::EnumBitfieldStruct<u8, Elsr14_SPEC>;
    impl Elsr14 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr15_SPEC;
    pub type Elsr15 = crate::EnumBitfieldStruct<u8, Elsr15_SPEC>;
    impl Elsr15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr16_SPEC;
    pub type Elsr16 = crate::EnumBitfieldStruct<u8, Elsr16_SPEC>;
    impl Elsr16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr17_SPEC;
    pub type Elsr17 = crate::EnumBitfieldStruct<u8, Elsr17_SPEC>;
    impl Elsr17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr19_SPEC;
    pub type Elsr19 = crate::EnumBitfieldStruct<u8, Elsr19_SPEC>;
    impl Elsr19 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr20_SPEC;
    pub type Elsr20 = crate::EnumBitfieldStruct<u8, Elsr20_SPEC>;
    impl Elsr20 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr21_SPEC;
    pub type Elsr21 = crate::EnumBitfieldStruct<u8, Elsr21_SPEC>;
    impl Elsr21 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr22_SPEC;
    pub type Elsr22 = crate::EnumBitfieldStruct<u8, Elsr22_SPEC>;
    impl Elsr22 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr23_SPEC;
    pub type Elsr23 = crate::EnumBitfieldStruct<u8, Elsr23_SPEC>;
    impl Elsr23 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr24_SPEC;
    pub type Elsr24 = crate::EnumBitfieldStruct<u8, Elsr24_SPEC>;
    impl Elsr24 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr25_SPEC;
    pub type Elsr25 = crate::EnumBitfieldStruct<u8, Elsr25_SPEC>;
    impl Elsr25 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr26_SPEC;
    pub type Elsr26 = crate::EnumBitfieldStruct<u8, Elsr26_SPEC>;
    impl Elsr26 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr27_SPEC;
    pub type Elsr27 = crate::EnumBitfieldStruct<u8, Elsr27_SPEC>;
    impl Elsr27 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr30_SPEC;
    pub type Elsr30 = crate::EnumBitfieldStruct<u8, Elsr30_SPEC>;
    impl Elsr30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr31_SPEC;
    pub type Elsr31 = crate::EnumBitfieldStruct<u8, Elsr31_SPEC>;
    impl Elsr31 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsarc_SPEC;
impl crate::sealed::RegSpec for Elcsarc_SPEC {
    type DataType = u32;
}

#[doc = "Event Link Controller Security Attribution Register C"]
pub type Elcsarc = crate::RegValueT<Elcsarc_SPEC>;

impl Elcsarc {
    #[doc = "Event Link Setting Register n Security Attribution"]
    #[inline(always)]
    pub fn elsr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fffff,
        1,
        0,
        elcsarc::Elsr,
        elcsarc::Elsr,
        Elcsarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fffff,
            1,
            0,
            elcsarc::Elsr,
            elcsarc::Elsr,
            Elcsarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcsarc {
    #[inline(always)]
    fn default() -> Elcsarc {
        <crate::RegValueT<Elcsarc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcsarc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr_SPEC;
    pub type Elsr = crate::EnumBitfieldStruct<u8, Elsr_SPEC>;
    impl Elsr {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcpara_SPEC;
impl crate::sealed::RegSpec for Elcpara_SPEC {
    type DataType = u32;
}

#[doc = "Event Link Controller Privilege Attribution Register A"]
pub type Elcpara = crate::RegValueT<Elcpara_SPEC>;

impl Elcpara {
    #[doc = "Event Link Controller Register Privilege Attribution"]
    #[inline(always)]
    pub fn elcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elcpara::Elcr,
        elcpara::Elcr,
        Elcpara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elcpara::Elcr,
            elcpara::Elcr,
            Elcpara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 0 Privilege Attribution"]
    #[inline(always)]
    pub fn elsegr0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        elcpara::Elsegr0,
        elcpara::Elsegr0,
        Elcpara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            elcpara::Elsegr0,
            elcpara::Elsegr0,
            Elcpara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 1 Privilege Attribution"]
    #[inline(always)]
    pub fn elsegr1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        elcpara::Elsegr1,
        elcpara::Elsegr1,
        Elcpara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elcpara::Elsegr1,
            elcpara::Elsegr1,
            Elcpara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 2 Privilege Attribution"]
    #[inline(always)]
    pub fn elsegr2(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        elcpara::Elsegr2,
        elcpara::Elsegr2,
        Elcpara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            elcpara::Elsegr2,
            elcpara::Elsegr2,
            Elcpara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Software Event Generation Register 3 Privilege Attribution"]
    #[inline(always)]
    pub fn elsegr3(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        elcpara::Elsegr3,
        elcpara::Elsegr3,
        Elcpara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            elcpara::Elsegr3,
            elcpara::Elsegr3,
            Elcpara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcpara {
    #[inline(always)]
    fn default() -> Elcpara {
        <crate::RegValueT<Elcpara_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod elcpara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elcr_SPEC;
    pub type Elcr = crate::EnumBitfieldStruct<u8, Elcr_SPEC>;
    impl Elcr {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr0_SPEC;
    pub type Elsegr0 = crate::EnumBitfieldStruct<u8, Elsegr0_SPEC>;
    impl Elsegr0 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr1_SPEC;
    pub type Elsegr1 = crate::EnumBitfieldStruct<u8, Elsegr1_SPEC>;
    impl Elsegr1 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr2_SPEC;
    pub type Elsegr2 = crate::EnumBitfieldStruct<u8, Elsegr2_SPEC>;
    impl Elsegr2 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr3_SPEC;
    pub type Elsegr3 = crate::EnumBitfieldStruct<u8, Elsegr3_SPEC>;
    impl Elsegr3 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcparb_SPEC;
impl crate::sealed::RegSpec for Elcparb_SPEC {
    type DataType = u32;
}

#[doc = "Event Link Controller Privilege Attribution Register B"]
pub type Elcparb = crate::RegValueT<Elcparb_SPEC>;

impl Elcparb {
    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elcparb::Elsr0,
        elcparb::Elsr0,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elcparb::Elsr0,
            elcparb::Elsr0,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        elcparb::Elsr1,
        elcparb::Elsr1,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            elcparb::Elsr1,
            elcparb::Elsr1,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        elcparb::Elsr2,
        elcparb::Elsr2,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elcparb::Elsr2,
            elcparb::Elsr2,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        elcparb::Elsr3,
        elcparb::Elsr3,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            elcparb::Elsr3,
            elcparb::Elsr3,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        elcparb::Elsr4,
        elcparb::Elsr4,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            elcparb::Elsr4,
            elcparb::Elsr4,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        elcparb::Elsr5,
        elcparb::Elsr5,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            elcparb::Elsr5,
            elcparb::Elsr5,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        elcparb::Elsr6,
        elcparb::Elsr6,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            elcparb::Elsr6,
            elcparb::Elsr6,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        elcparb::Elsr7,
        elcparb::Elsr7,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            elcparb::Elsr7,
            elcparb::Elsr7,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        elcparb::Elsr12,
        elcparb::Elsr12,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            elcparb::Elsr12,
            elcparb::Elsr12,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        elcparb::Elsr13,
        elcparb::Elsr13,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            elcparb::Elsr13,
            elcparb::Elsr13,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        elcparb::Elsr14,
        elcparb::Elsr14,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            elcparb::Elsr14,
            elcparb::Elsr14,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        elcparb::Elsr15,
        elcparb::Elsr15,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            elcparb::Elsr15,
            elcparb::Elsr15,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        elcparb::Elsr16,
        elcparb::Elsr16,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            elcparb::Elsr16,
            elcparb::Elsr16,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        elcparb::Elsr17,
        elcparb::Elsr17,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            elcparb::Elsr17,
            elcparb::Elsr17,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        elcparb::Elsr19,
        elcparb::Elsr19,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            elcparb::Elsr19,
            elcparb::Elsr19,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        elcparb::Elsr20,
        elcparb::Elsr20,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            elcparb::Elsr20,
            elcparb::Elsr20,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        elcparb::Elsr21,
        elcparb::Elsr21,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            elcparb::Elsr21,
            elcparb::Elsr21,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        elcparb::Elsr22,
        elcparb::Elsr22,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            elcparb::Elsr22,
            elcparb::Elsr22,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        elcparb::Elsr23,
        elcparb::Elsr23,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            elcparb::Elsr23,
            elcparb::Elsr23,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        elcparb::Elsr24,
        elcparb::Elsr24,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            elcparb::Elsr24,
            elcparb::Elsr24,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        elcparb::Elsr25,
        elcparb::Elsr25,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            elcparb::Elsr25,
            elcparb::Elsr25,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        elcparb::Elsr26,
        elcparb::Elsr26,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            elcparb::Elsr26,
            elcparb::Elsr26,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        elcparb::Elsr27,
        elcparb::Elsr27,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            elcparb::Elsr27,
            elcparb::Elsr27,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        elcparb::Elsr30,
        elcparb::Elsr30,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            elcparb::Elsr30,
            elcparb::Elsr30,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        elcparb::Elsr31,
        elcparb::Elsr31,
        Elcparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            elcparb::Elsr31,
            elcparb::Elsr31,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcparb {
    #[inline(always)]
    fn default() -> Elcparb {
        <crate::RegValueT<Elcparb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod elcparb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr0_SPEC;
    pub type Elsr0 = crate::EnumBitfieldStruct<u8, Elsr0_SPEC>;
    impl Elsr0 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr1_SPEC;
    pub type Elsr1 = crate::EnumBitfieldStruct<u8, Elsr1_SPEC>;
    impl Elsr1 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr2_SPEC;
    pub type Elsr2 = crate::EnumBitfieldStruct<u8, Elsr2_SPEC>;
    impl Elsr2 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr3_SPEC;
    pub type Elsr3 = crate::EnumBitfieldStruct<u8, Elsr3_SPEC>;
    impl Elsr3 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr4_SPEC;
    pub type Elsr4 = crate::EnumBitfieldStruct<u8, Elsr4_SPEC>;
    impl Elsr4 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr5_SPEC;
    pub type Elsr5 = crate::EnumBitfieldStruct<u8, Elsr5_SPEC>;
    impl Elsr5 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr6_SPEC;
    pub type Elsr6 = crate::EnumBitfieldStruct<u8, Elsr6_SPEC>;
    impl Elsr6 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr7_SPEC;
    pub type Elsr7 = crate::EnumBitfieldStruct<u8, Elsr7_SPEC>;
    impl Elsr7 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr12_SPEC;
    pub type Elsr12 = crate::EnumBitfieldStruct<u8, Elsr12_SPEC>;
    impl Elsr12 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr13_SPEC;
    pub type Elsr13 = crate::EnumBitfieldStruct<u8, Elsr13_SPEC>;
    impl Elsr13 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr14_SPEC;
    pub type Elsr14 = crate::EnumBitfieldStruct<u8, Elsr14_SPEC>;
    impl Elsr14 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr15_SPEC;
    pub type Elsr15 = crate::EnumBitfieldStruct<u8, Elsr15_SPEC>;
    impl Elsr15 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr16_SPEC;
    pub type Elsr16 = crate::EnumBitfieldStruct<u8, Elsr16_SPEC>;
    impl Elsr16 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr17_SPEC;
    pub type Elsr17 = crate::EnumBitfieldStruct<u8, Elsr17_SPEC>;
    impl Elsr17 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr19_SPEC;
    pub type Elsr19 = crate::EnumBitfieldStruct<u8, Elsr19_SPEC>;
    impl Elsr19 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr20_SPEC;
    pub type Elsr20 = crate::EnumBitfieldStruct<u8, Elsr20_SPEC>;
    impl Elsr20 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr21_SPEC;
    pub type Elsr21 = crate::EnumBitfieldStruct<u8, Elsr21_SPEC>;
    impl Elsr21 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr22_SPEC;
    pub type Elsr22 = crate::EnumBitfieldStruct<u8, Elsr22_SPEC>;
    impl Elsr22 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr23_SPEC;
    pub type Elsr23 = crate::EnumBitfieldStruct<u8, Elsr23_SPEC>;
    impl Elsr23 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr24_SPEC;
    pub type Elsr24 = crate::EnumBitfieldStruct<u8, Elsr24_SPEC>;
    impl Elsr24 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr25_SPEC;
    pub type Elsr25 = crate::EnumBitfieldStruct<u8, Elsr25_SPEC>;
    impl Elsr25 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr26_SPEC;
    pub type Elsr26 = crate::EnumBitfieldStruct<u8, Elsr26_SPEC>;
    impl Elsr26 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr27_SPEC;
    pub type Elsr27 = crate::EnumBitfieldStruct<u8, Elsr27_SPEC>;
    impl Elsr27 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr30_SPEC;
    pub type Elsr30 = crate::EnumBitfieldStruct<u8, Elsr30_SPEC>;
    impl Elsr30 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr31_SPEC;
    pub type Elsr31 = crate::EnumBitfieldStruct<u8, Elsr31_SPEC>;
    impl Elsr31 {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcparc_SPEC;
impl crate::sealed::RegSpec for Elcparc_SPEC {
    type DataType = u32;
}

#[doc = "Event Link Controller Privilege Attribution Register C"]
pub type Elcparc = crate::RegValueT<Elcparc_SPEC>;

impl Elcparc {
    #[doc = "Event Link Setting Register n Privilege Attribution"]
    #[inline(always)]
    pub fn elsr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fffff,
        1,
        0,
        elcparc::Elsr,
        elcparc::Elsr,
        Elcparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fffff,
            1,
            0,
            elcparc::Elsr,
            elcparc::Elsr,
            Elcparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elcparc {
    #[inline(always)]
    fn default() -> Elcparc {
        <crate::RegValueT<Elcparc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod elcparc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr_SPEC;
    pub type Elsr = crate::EnumBitfieldStruct<u8, Elsr_SPEC>;
    impl Elsr {
        #[doc = "Privileged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unprivileged"]
        pub const _1: Self = Self::new(1);
    }
}
