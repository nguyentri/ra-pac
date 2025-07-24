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
#[doc = r"Gateway CPU Agent"]
unsafe impl ::core::marker::Send for super::Gwca0Ns {}
unsafe impl ::core::marker::Sync for super::Gwca0Ns {}
impl super::Gwca0Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Mode Configuration Register"]
    #[inline(always)]
    pub const fn gwmc(&self) -> &'static crate::common::Reg<self::Gwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Mode Status Register"]
    #[inline(always)]
    pub const fn gwms(&self) -> &'static crate::common::Reg<self::Gwms_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwms_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "IPV Remapping Configuration Register \\[802.1Q\\]"]
    #[inline(always)]
    pub const fn gwirc(&self) -> &'static crate::common::Reg<self::Gwirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "RX Descriptor Queue Security Configuration Register"]
    #[inline(always)]
    pub const fn gwrdqsc(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "RX Descriptor Queue Control Register"]
    #[inline(always)]
    pub const fn gwrdqc(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "RX Descriptor Queue Arbitration Control Register"]
    #[inline(always)]
    pub const fn gwrdqac(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "RX General Configuration Register"]
    #[inline(always)]
    pub const fn gwrgc(&self) -> &'static crate::common::Reg<self::Gwrgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Reception Maximum Frame Size Configuration Register %s"]
    #[inline(always)]
    pub const fn gwrmfsc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc2(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc3(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc4(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc5(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc6(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrmfsc7(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }

    #[doc = "Reception Descriptor Queue %s Depth Configuration Register"]
    #[inline(always)]
    pub const fn gwrdqdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60usize))
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc2(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc3(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc4(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc5(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc6(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqdc7(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }

    #[doc = "RX Descriptor Queue %s Monitoring Register"]
    #[inline(always)]
    pub const fn gwrdqm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn gwrdqm0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqm1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqm2(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqm3(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqm4(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqm5(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqm6(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqm7(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }

    #[doc = "RX Descriptor Queue %s Max Level Monitoring Register"]
    #[inline(always)]
    pub const fn gwrdqmlm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa0usize))
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm2(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm3(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm4(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm5(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm6(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrdqmlm7(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }

    #[doc = "Multicast Table Initialization Register Monitoring Register"]
    #[inline(always)]
    pub const fn gwmtirm(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmtirm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwmtirm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Multicast Table Learning Setting Register"]
    #[inline(always)]
    pub const fn gwmstls(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmstls_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwmstls_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Multicast Table Learning Result Register"]
    #[inline(always)]
    pub const fn gwmstlr(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmstlr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwmstlr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Multicast Table Searching Setting Register"]
    #[inline(always)]
    pub const fn gwmstss(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmstss_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwmstss_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "Multicast Table Searching Result Register"]
    #[inline(always)]
    pub const fn gwmstsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmstsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwmstsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "MAC Address Configuration Register 0"]
    #[inline(always)]
    pub const fn gwmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "MAC Address Configuration Register 1"]
    #[inline(always)]
    pub const fn gwmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "VLAN Control Configuration Register"]
    #[inline(always)]
    pub const fn gwvcc(&self) -> &'static crate::common::Reg<self::Gwvcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwvcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "VLAN TAG Configuration Register"]
    #[inline(always)]
    pub const fn gwvtc(&self) -> &'static crate::common::Reg<self::Gwvtc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwvtc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "Transmission TAG Filtering Configuration Register"]
    #[inline(always)]
    pub const fn gwttfc(
        &self,
    ) -> &'static crate::common::Reg<self::Gwttfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwttfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[doc = "Timestamp Descriptor Chain %s Address Configuration Register 0"]
    #[inline(always)]
    pub const fn gwtdcac0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwtdcac0_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }
    #[inline(always)]
    pub const fn gwtdcac00(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtdcac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtdcac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwtdcac10(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtdcac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtdcac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }

    #[doc = "Timestamp Descriptor Chain %s Address Configuration Register 1"]
    #[inline(always)]
    pub const fn gwtdcac1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwtdcac1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x144usize))
        }
    }
    #[inline(always)]
    pub const fn gwtdcac01(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtdcac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtdcac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwtdcac11(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtdcac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtdcac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14cusize),
            )
        }
    }

    #[doc = "Timestamp Descriptor Chain %s Configuration Register"]
    #[inline(always)]
    pub const fn gwtsdcc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwtsdcc_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x160usize))
        }
    }
    #[inline(always)]
    pub const fn gwtsdcc0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtsdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwtsdcc1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtsdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }

    #[doc = "Timestamp Number Monitoring Register"]
    #[inline(always)]
    pub const fn gwtsnm(&self) -> &'static crate::common::Reg<self::Gwtsnm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtsnm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "Timestamp Maximum Number Monitoring Register"]
    #[inline(always)]
    pub const fn gwtsmnm(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsmnm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtsmnm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[doc = "AXI Control Register"]
    #[inline(always)]
    pub const fn gwac(&self) -> &'static crate::common::Reg<self::Gwac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[doc = "Descriptor Chain Base Address Configuration Register 0"]
    #[inline(always)]
    pub const fn gwdcbac0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcbac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcbac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(404usize),
            )
        }
    }

    #[doc = "Descriptor Chain Base Address Configuration Register 1"]
    #[inline(always)]
    pub const fn gwdcbac1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcbac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcbac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(408usize),
            )
        }
    }

    #[doc = "Maximum Descriptor Number Configuration Register"]
    #[inline(always)]
    pub const fn gwmdnc(
        &self,
    ) -> &'static crate::common::Reg<self::Gwmdnc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwmdnc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "Transmission Request Configuration Register"]
    #[inline(always)]
    pub const fn gwtrc0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtrc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtrc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Transmission Request Configuration Register"]
    #[inline(always)]
    pub const fn gwtrc1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtrc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtrc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "Transmission Pause Configuration Register p (p = 0 to 1)"]
    #[inline(always)]
    pub const fn gwtpcp(
        &self,
    ) -> &'static crate::common::Reg<self::GwtpCp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GwtpCp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "AXI RAM Initialization Register Monitoring Register"]
    #[inline(always)]
    pub const fn gwarirm(
        &self,
    ) -> &'static crate::common::Reg<self::Gwarirm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwarirm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(896usize),
            )
        }
    }

    #[doc = "Descriptor Chain %s Configuration Register"]
    #[inline(always)]
    pub const fn gwdcc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW>,
        64,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }
    #[inline(always)]
    pub const fn gwdcc0(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc1(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc2(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc3(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc4(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc5(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x414usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc6(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc7(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x41cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc8(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc9(&self) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc10(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc11(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc12(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc13(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x434usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc14(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x438usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc15(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x43cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc16(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc17(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc18(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc19(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc20(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc21(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x454usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc22(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x458usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc23(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x45cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc24(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc25(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc26(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc27(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x46cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc28(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x470usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc29(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x474usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc30(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x478usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc31(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x47cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc32(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x480usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc33(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x484usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc34(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x488usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc35(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc36(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x490usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc37(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x494usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc38(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x498usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc39(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x49cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc40(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc41(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc42(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc43(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc44(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc45(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc46(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc47(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc48(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc49(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc50(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc51(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc52(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc53(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc54(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc55(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc56(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc57(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc58(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc59(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc60(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc61(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc62(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwdcc63(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4fcusize),
            )
        }
    }

    #[doc = "AXI Address RAM Searching Setting Register"]
    #[inline(always)]
    pub const fn gwaarss(
        &self,
    ) -> &'static crate::common::Reg<self::Gwaarss_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwaarss_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2048usize),
            )
        }
    }

    #[doc = "AXI Address RAM Searching Result Register 0"]
    #[inline(always)]
    pub const fn gwaarsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwaarsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwaarsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2052usize),
            )
        }
    }

    #[doc = "AXI Address RAM Searching Result Register 1"]
    #[inline(always)]
    pub const fn gwaarsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwaarsr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwaarsr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2056usize),
            )
        }
    }

    #[doc = "Incremental Data Area %s Used Area Size Register"]
    #[inline(always)]
    pub const fn gwidauas(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwidauas_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x840usize))
        }
    }
    #[inline(always)]
    pub const fn gwidauas0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidauas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidauas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidauas1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidauas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidauas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidauas2(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidauas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidauas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x848usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidauas3(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidauas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidauas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84cusize),
            )
        }
    }

    #[doc = "Incremental Data Area %s Size Monitoring Register"]
    #[inline(always)]
    pub const fn gwidasm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwidasm_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x880usize))
        }
    }
    #[inline(always)]
    pub const fn gwidasm0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x880usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasm1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x884usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasm2(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x888usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasm3(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x88cusize),
            )
        }
    }

    #[doc = "Incremental Data Area %s Start Address Monitoring Register 0"]
    #[inline(always)]
    pub const fn gwidasam0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwidasam0_SPEC, crate::common::R>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x900usize))
        }
    }
    #[inline(always)]
    pub const fn gwidasam00(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x900usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasam10(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x908usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasam20(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x910usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasam30(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x918usize),
            )
        }
    }

    #[doc = "Incremental Data Area %s Start Address Monitoring Register 1"]
    #[inline(always)]
    pub const fn gwidasam1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwidasam1_SPEC, crate::common::R>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x904usize))
        }
    }
    #[inline(always)]
    pub const fn gwidasam01(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x904usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasam11(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x90cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasam21(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x914usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidasam31(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidasam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidasam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x91cusize),
            )
        }
    }

    #[doc = "Incremental Data Area %s Current Address Monitoring Register 0"]
    #[inline(always)]
    pub const fn gwidacam0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwidacam0_SPEC, crate::common::R>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x980usize))
        }
    }
    #[inline(always)]
    pub const fn gwidacam00(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x980usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidacam10(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x988usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidacam20(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x990usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidacam30(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x998usize),
            )
        }
    }

    #[doc = "Incremental Data Area %s Current Address Monitoring Register 1"]
    #[inline(always)]
    pub const fn gwidacam1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwidacam1_SPEC, crate::common::R>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x984usize))
        }
    }
    #[inline(always)]
    pub const fn gwidacam01(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x984usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidacam11(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x98cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidacam21(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x994usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidacam31(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidacam1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwidacam1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x99cusize),
            )
        }
    }

    #[doc = "Global Rate Limiter Configuration Register"]
    #[inline(always)]
    pub const fn gwgrlc(
        &self,
    ) -> &'static crate::common::Reg<self::Gwgrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwgrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2560usize),
            )
        }
    }

    #[doc = "Global Rate Limiter Upper Limit Configuration Register"]
    #[inline(always)]
    pub const fn gwgrlulc(
        &self,
    ) -> &'static crate::common::Reg<self::Gwgrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwgrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2564usize),
            )
        }
    }

    #[doc = "Rate Limiter %s Configuration Register"]
    #[inline(always)]
    pub const fn gwrlc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa80usize))
        }
    }
    #[inline(always)]
    pub const fn gwrlc0(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlc1(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlc2(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlc3(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlc4(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlc5(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlc6(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlc7(&self) -> &'static crate::common::Reg<self::Gwrlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab8usize),
            )
        }
    }

    #[doc = "Rate Limiter %s Upper Limit Configuration Register"]
    #[inline(always)]
    pub const fn gwrlulc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa84usize))
        }
    }
    #[inline(always)]
    pub const fn gwrlulc0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlulc1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlulc2(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlulc3(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlulc4(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlulc5(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xaacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlulc6(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xab4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwrlulc7(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrlulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwrlulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xabcusize),
            )
        }
    }

    #[doc = "Interrupt Delay Prescaler Configuration Register"]
    #[inline(always)]
    pub const fn gwidpc(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2944usize),
            )
        }
    }

    #[doc = "Received Data Counter Register"]
    #[inline(always)]
    pub const fn gwrdcn(&self) -> &'static crate::common::Reg<self::Gwrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

    #[doc = "Transmitted Data Counter Register"]
    #[inline(always)]
    pub const fn gwtdcn(&self) -> &'static crate::common::Reg<self::Gwtdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4100usize),
            )
        }
    }

    #[doc = "Timestamp Counter Register"]
    #[inline(always)]
    pub const fn gwtscn(&self) -> &'static crate::common::Reg<self::Gwtscn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtscn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4104usize),
            )
        }
    }

    #[doc = "Timestamp Overflow Error Counter Register"]
    #[inline(always)]
    pub const fn gwtsovfecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsovfecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtsovfecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4108usize),
            )
        }
    }

    #[doc = "Under Minimum Frame Size Error Counter Register"]
    #[inline(always)]
    pub const fn gwusmfsecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwusmfsecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwusmfsecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4112usize),
            )
        }
    }

    #[doc = "TAG Filtering Error Counter Register"]
    #[inline(always)]
    pub const fn gwtfecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtfecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtfecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

    #[doc = "Sequence Error Counter Register"]
    #[inline(always)]
    pub const fn gwseqecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwseqecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwseqecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4120usize),
            )
        }
    }

    #[doc = "TX Descriptor Number Error Counter Register"]
    #[inline(always)]
    pub const fn gwtxdnecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtxdnecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtxdnecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4128usize),
            )
        }
    }

    #[doc = "Frame Size Error Counter Register"]
    #[inline(always)]
    pub const fn gwfsecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwfsecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwfsecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4132usize),
            )
        }
    }

    #[doc = "Timestamp Descriptor Full Error Counter Register"]
    #[inline(always)]
    pub const fn gwtdfecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtdfecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtdfecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4136usize),
            )
        }
    }

    #[doc = "Timestamp Descriptor Number Error Counter Register"]
    #[inline(always)]
    pub const fn gwtsdnecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsdnecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtsdnecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4140usize),
            )
        }
    }

    #[doc = "Descriptor Queue Overflow Error Counter Register"]
    #[inline(always)]
    pub const fn gwdqoecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdqoecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwdqoecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4144usize),
            )
        }
    }

    #[doc = "Descriptor Queue Security Error Counter Register"]
    #[inline(always)]
    pub const fn gwdqsecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdqsecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwdqsecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4148usize),
            )
        }
    }

    #[doc = "Descriptor Full Error Counter Register"]
    #[inline(always)]
    pub const fn gwdfecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdfecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwdfecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4152usize),
            )
        }
    }

    #[doc = "Descriptor Security Error Counter Register"]
    #[inline(always)]
    pub const fn gwdsecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdsecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwdsecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4156usize),
            )
        }
    }

    #[doc = "Data Size Error Counter Register"]
    #[inline(always)]
    pub const fn gwdszecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdszecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwdszecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4160usize),
            )
        }
    }

    #[doc = "Descriptor Chain Type Error Counter Register"]
    #[inline(always)]
    pub const fn gwdctecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdctecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwdctecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4164usize),
            )
        }
    }

    #[doc = "RX Descriptor Number Error Counter Register"]
    #[inline(always)]
    pub const fn gwrxdnecn(
        &self,
    ) -> &'static crate::common::Reg<self::Gwrxdnecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwrxdnecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4168usize),
            )
        }
    }

    #[doc = "Data Interrupt Status Register"]
    #[inline(always)]
    pub const fn gwdis0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[doc = "Data Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gwdie0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4356usize),
            )
        }
    }

    #[doc = "Data Interrupt Disable Register"]
    #[inline(always)]
    pub const fn gwdid0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdid0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdid0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4360usize),
            )
        }
    }

    #[doc = "Data Interrupt Delayed Status Register"]
    #[inline(always)]
    pub const fn gwdids0(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdids0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdids0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4364usize),
            )
        }
    }

    #[doc = "Data Interrupt Status Register"]
    #[inline(always)]
    pub const fn gwdis1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4368usize),
            )
        }
    }

    #[doc = "Data Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gwdie1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4372usize),
            )
        }
    }

    #[doc = "Data Interrupt Disable Register"]
    #[inline(always)]
    pub const fn gwdid1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdid1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdid1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4376usize),
            )
        }
    }

    #[doc = "Data Interrupt Delayed Status Register"]
    #[inline(always)]
    pub const fn gwdids1(
        &self,
    ) -> &'static crate::common::Reg<self::Gwdids1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwdids1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4380usize),
            )
        }
    }

    #[doc = "Timestamp Data Interrupt Status Register"]
    #[inline(always)]
    pub const fn gwtsdis(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsdis_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtsdis_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4480usize),
            )
        }
    }

    #[doc = "Timestamp Data Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gwtsdie(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsdie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwtsdie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4484usize),
            )
        }
    }

    #[doc = "Timestamp Data Interrupt Disable Register"]
    #[inline(always)]
    pub const fn gwtsdid(
        &self,
    ) -> &'static crate::common::Reg<self::Gwtsdid_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gwtsdid_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4488usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn gweis0(
        &self,
    ) -> &'static crate::common::Reg<self::Gweis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4496usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn gweie0(
        &self,
    ) -> &'static crate::common::Reg<self::Gweie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4500usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn gweid0(&self) -> &'static crate::common::Reg<self::Gweid0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gweid0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4504usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn gweis1(
        &self,
    ) -> &'static crate::common::Reg<self::Gweis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4512usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn gweie1(
        &self,
    ) -> &'static crate::common::Reg<self::Gweie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4516usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn gweid1(&self) -> &'static crate::common::Reg<self::Gweid1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gweid1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4520usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn gweis20(
        &self,
    ) -> &'static crate::common::Reg<self::Gweis20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweis20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4608usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn gweie20(
        &self,
    ) -> &'static crate::common::Reg<self::Gweie20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweie20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4612usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn gweid20(
        &self,
    ) -> &'static crate::common::Reg<self::Gweid20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweid20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4616usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn gweis21(
        &self,
    ) -> &'static crate::common::Reg<self::Gweis21_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweis21_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4624usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn gweie21(
        &self,
    ) -> &'static crate::common::Reg<self::Gweie21_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweie21_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4628usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn gweid21(
        &self,
    ) -> &'static crate::common::Reg<self::Gweid21_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweid21_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4632usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 3"]
    #[inline(always)]
    pub const fn gweis3(
        &self,
    ) -> &'static crate::common::Reg<self::Gweis3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweis3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4736usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 3"]
    #[inline(always)]
    pub const fn gweie3(
        &self,
    ) -> &'static crate::common::Reg<self::Gweie3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweie3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4740usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 3"]
    #[inline(always)]
    pub const fn gweid3(&self) -> &'static crate::common::Reg<self::Gweid3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gweid3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4744usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 4"]
    #[inline(always)]
    pub const fn gweis4(
        &self,
    ) -> &'static crate::common::Reg<self::Gweis4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweis4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4752usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 4"]
    #[inline(always)]
    pub const fn gweie4(
        &self,
    ) -> &'static crate::common::Reg<self::Gweie4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweie4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4756usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 4"]
    #[inline(always)]
    pub const fn gweid4(&self) -> &'static crate::common::Reg<self::Gweid4_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gweid4_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4760usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 5"]
    #[inline(always)]
    pub const fn gweis5(
        &self,
    ) -> &'static crate::common::Reg<self::Gweis5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweis5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4768usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 5"]
    #[inline(always)]
    pub const fn gweie5(
        &self,
    ) -> &'static crate::common::Reg<self::Gweie5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gweie5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4772usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 5"]
    #[inline(always)]
    pub const fn gweid5(&self) -> &'static crate::common::Reg<self::Gweid5_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gweid5_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4776usize),
            )
        }
    }

    #[doc = "Interrupt Delay %s Configuration Register"]
    #[inline(always)]
    pub const fn gwidc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gwidc_SPEC, crate::common::RW>,
        64,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc000usize))
        }
    }
    #[inline(always)]
    pub const fn gwidc0(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc000usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc1(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc2(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc008usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc3(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc00cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc4(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc010usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc5(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc014usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc6(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc018usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc7(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc01cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc8(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc020usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc9(&self) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc024usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc10(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc028usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc11(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc02cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc12(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc030usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc13(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc14(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc038usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc15(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc03cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc16(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc040usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc17(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc044usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc18(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc048usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc19(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc04cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc20(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc050usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc21(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc054usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc22(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc058usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc23(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc05cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc24(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc060usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc25(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc26(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc068usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc27(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc06cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc28(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc070usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc29(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc074usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc30(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc078usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc31(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc07cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc32(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc080usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc33(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc084usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc34(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc088usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc35(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc08cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc36(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc090usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc37(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc094usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc38(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc098usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc39(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc09cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc40(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc41(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc42(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc43(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc44(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc45(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc46(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc47(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc48(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc49(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc50(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc51(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc52(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc53(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc54(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc55(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc56(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc57(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc58(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc59(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc60(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc61(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc62(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gwidc63(
        &self,
    ) -> &'static crate::common::Reg<self::Gwidc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gwidc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xc0fcusize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmc_SPEC;
impl crate::sealed::RegSpec for Gwmc_SPEC {
    type DataType = u32;
}

#[doc = "Mode Configuration Register"]
pub type Gwmc = crate::RegValueT<Gwmc_SPEC>;

impl Gwmc {
    #[doc = "Operating Mode Command"]
    #[inline(always)]
    pub fn opc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gwmc::Opc,
        gwmc::Opc,
        Gwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gwmc::Opc,
            gwmc::Opc,
            Gwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwmc {
    #[inline(always)]
    fn default() -> Gwmc {
        <crate::RegValueT<Gwmc_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod gwmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opc_SPEC;
    pub type Opc = crate::EnumBitfieldStruct<u8, Opc_SPEC>;
    impl Opc {
        #[doc = "Enter RESET mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Enter DISABLE mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Enter CONFIG mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Enter OPERATION mode"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwms_SPEC;
impl crate::sealed::RegSpec for Gwms_SPEC {
    type DataType = u32;
}

#[doc = "Mode Status Register"]
pub type Gwms = crate::RegValueT<Gwms_SPEC>;

impl Gwms {
    #[doc = "Operating Mode Status"]
    #[inline(always)]
    pub fn ops(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, gwms::Ops, gwms::Ops, Gwms_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gwms::Ops,
            gwms::Ops,
            Gwms_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwms {
    #[inline(always)]
    fn default() -> Gwms {
        <crate::RegValueT<Gwms_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod gwms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ops_SPEC;
    pub type Ops = crate::EnumBitfieldStruct<u8, Ops_SPEC>;
    impl Ops {
        #[doc = "RESET mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "DISABLE mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "CONFIG mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "OPERATION mode"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwirc_SPEC;
impl crate::sealed::RegSpec for Gwirc_SPEC {
    type DataType = u32;
}

#[doc = "IPV Remapping Configuration Register \\[802.1Q\\]"]
pub type Gwirc = crate::RegValueT<Gwirc_SPEC>;

impl Gwirc {
    #[doc = "IPV remapping 0"]
    #[inline(always)]
    pub fn ipvr0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV remapping 1"]
    #[inline(always)]
    pub fn ipvr1(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV remapping 2"]
    #[inline(always)]
    pub fn ipvr2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV remapping 3"]
    #[inline(always)]
    pub fn ipvr3(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV remapping 4"]
    #[inline(always)]
    pub fn ipvr4(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV remapping 5"]
    #[inline(always)]
    pub fn ipvr5(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV remapping 6"]
    #[inline(always)]
    pub fn ipvr6(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV remapping 7"]
    #[inline(always)]
    pub fn ipvr7(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Gwirc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Gwirc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwirc {
    #[inline(always)]
    fn default() -> Gwirc {
        <crate::RegValueT<Gwirc_SPEC> as RegisterValue<_>>::new(1985229328)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrdqsc_SPEC;
impl crate::sealed::RegSpec for Gwrdqsc_SPEC {
    type DataType = u32;
}

#[doc = "RX Descriptor Queue Security Configuration Register"]
pub type Gwrdqsc = crate::RegValueT<Gwrdqsc_SPEC>;

impl Gwrdqsc {
    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl0,
        gwrdqsc::Rdqsl0,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl0,
            gwrdqsc::Rdqsl0,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl1,
        gwrdqsc::Rdqsl1,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl1,
            gwrdqsc::Rdqsl1,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl2,
        gwrdqsc::Rdqsl2,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl2,
            gwrdqsc::Rdqsl2,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl3,
        gwrdqsc::Rdqsl3,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl3,
            gwrdqsc::Rdqsl3,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl4,
        gwrdqsc::Rdqsl4,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl4,
            gwrdqsc::Rdqsl4,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl5,
        gwrdqsc::Rdqsl5,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl5,
            gwrdqsc::Rdqsl5,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl6,
        gwrdqsc::Rdqsl6,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl6,
            gwrdqsc::Rdqsl6,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Security Level (The variable i corresponds to the bit number.)"]
    #[inline(always)]
    pub fn rdqsl7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gwrdqsc::Rdqsl7,
        gwrdqsc::Rdqsl7,
        Gwrdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gwrdqsc::Rdqsl7,
            gwrdqsc::Rdqsl7,
            Gwrdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwrdqsc {
    #[inline(always)]
    fn default() -> Gwrdqsc {
        <crate::RegValueT<Gwrdqsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwrdqsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl0_SPEC;
    pub type Rdqsl0 = crate::EnumBitfieldStruct<u8, Rdqsl0_SPEC>;
    impl Rdqsl0 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl1_SPEC;
    pub type Rdqsl1 = crate::EnumBitfieldStruct<u8, Rdqsl1_SPEC>;
    impl Rdqsl1 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl2_SPEC;
    pub type Rdqsl2 = crate::EnumBitfieldStruct<u8, Rdqsl2_SPEC>;
    impl Rdqsl2 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl3_SPEC;
    pub type Rdqsl3 = crate::EnumBitfieldStruct<u8, Rdqsl3_SPEC>;
    impl Rdqsl3 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl4_SPEC;
    pub type Rdqsl4 = crate::EnumBitfieldStruct<u8, Rdqsl4_SPEC>;
    impl Rdqsl4 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl5_SPEC;
    pub type Rdqsl5 = crate::EnumBitfieldStruct<u8, Rdqsl5_SPEC>;
    impl Rdqsl5 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl6_SPEC;
    pub type Rdqsl6 = crate::EnumBitfieldStruct<u8, Rdqsl6_SPEC>;
    impl Rdqsl6 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqsl7_SPEC;
    pub type Rdqsl7 = crate::EnumBitfieldStruct<u8, Rdqsl7_SPEC>;
    impl Rdqsl7 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrdqc_SPEC;
impl crate::sealed::RegSpec for Gwrdqc_SPEC {
    type DataType = u32;
}

#[doc = "RX Descriptor Queue Control Register"]
pub type Gwrdqc = crate::RegValueT<Gwrdqc_SPEC>;

impl Gwrdqc {
    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwrdqc::Rdqd0,
        gwrdqc::Rdqd0,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwrdqc::Rdqd0,
            gwrdqc::Rdqd0,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gwrdqc::Rdqd1,
        gwrdqc::Rdqd1,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gwrdqc::Rdqd1,
            gwrdqc::Rdqd1,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gwrdqc::Rdqd2,
        gwrdqc::Rdqd2,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gwrdqc::Rdqd2,
            gwrdqc::Rdqd2,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gwrdqc::Rdqd3,
        gwrdqc::Rdqd3,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gwrdqc::Rdqd3,
            gwrdqc::Rdqd3,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gwrdqc::Rdqd4,
        gwrdqc::Rdqd4,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gwrdqc::Rdqd4,
            gwrdqc::Rdqd4,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gwrdqc::Rdqd5,
        gwrdqc::Rdqd5,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gwrdqc::Rdqd5,
            gwrdqc::Rdqd5,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gwrdqc::Rdqd6,
        gwrdqc::Rdqd6,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gwrdqc::Rdqd6,
            gwrdqc::Rdqd6,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqd7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gwrdqc::Rdqd7,
        gwrdqc::Rdqd7,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gwrdqc::Rdqd7,
            gwrdqc::Rdqd7,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gwrdqc::Rdqp0,
        gwrdqc::Rdqp0,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gwrdqc::Rdqp0,
            gwrdqc::Rdqp0,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gwrdqc::Rdqp1,
        gwrdqc::Rdqp1,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gwrdqc::Rdqp1,
            gwrdqc::Rdqp1,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gwrdqc::Rdqp2,
        gwrdqc::Rdqp2,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gwrdqc::Rdqp2,
            gwrdqc::Rdqp2,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gwrdqc::Rdqp3,
        gwrdqc::Rdqp3,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gwrdqc::Rdqp3,
            gwrdqc::Rdqp3,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gwrdqc::Rdqp4,
        gwrdqc::Rdqp4,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gwrdqc::Rdqp4,
            gwrdqc::Rdqp4,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gwrdqc::Rdqp5,
        gwrdqc::Rdqp5,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gwrdqc::Rdqp5,
            gwrdqc::Rdqp5,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gwrdqc::Rdqp6,
        gwrdqc::Rdqp6,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gwrdqc::Rdqp6,
            gwrdqc::Rdqp6,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue i Pause (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn rdqp7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gwrdqc::Rdqp7,
        gwrdqc::Rdqp7,
        Gwrdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gwrdqc::Rdqp7,
            gwrdqc::Rdqp7,
            Gwrdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwrdqc {
    #[inline(always)]
    fn default() -> Gwrdqc {
        <crate::RegValueT<Gwrdqc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwrdqc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd0_SPEC;
    pub type Rdqd0 = crate::EnumBitfieldStruct<u8, Rdqd0_SPEC>;
    impl Rdqd0 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd1_SPEC;
    pub type Rdqd1 = crate::EnumBitfieldStruct<u8, Rdqd1_SPEC>;
    impl Rdqd1 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd2_SPEC;
    pub type Rdqd2 = crate::EnumBitfieldStruct<u8, Rdqd2_SPEC>;
    impl Rdqd2 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd3_SPEC;
    pub type Rdqd3 = crate::EnumBitfieldStruct<u8, Rdqd3_SPEC>;
    impl Rdqd3 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd4_SPEC;
    pub type Rdqd4 = crate::EnumBitfieldStruct<u8, Rdqd4_SPEC>;
    impl Rdqd4 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd5_SPEC;
    pub type Rdqd5 = crate::EnumBitfieldStruct<u8, Rdqd5_SPEC>;
    impl Rdqd5 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd6_SPEC;
    pub type Rdqd6 = crate::EnumBitfieldStruct<u8, Rdqd6_SPEC>;
    impl Rdqd6 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqd7_SPEC;
    pub type Rdqd7 = crate::EnumBitfieldStruct<u8, Rdqd7_SPEC>;
    impl Rdqd7 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp0_SPEC;
    pub type Rdqp0 = crate::EnumBitfieldStruct<u8, Rdqp0_SPEC>;
    impl Rdqp0 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp1_SPEC;
    pub type Rdqp1 = crate::EnumBitfieldStruct<u8, Rdqp1_SPEC>;
    impl Rdqp1 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp2_SPEC;
    pub type Rdqp2 = crate::EnumBitfieldStruct<u8, Rdqp2_SPEC>;
    impl Rdqp2 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp3_SPEC;
    pub type Rdqp3 = crate::EnumBitfieldStruct<u8, Rdqp3_SPEC>;
    impl Rdqp3 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp4_SPEC;
    pub type Rdqp4 = crate::EnumBitfieldStruct<u8, Rdqp4_SPEC>;
    impl Rdqp4 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp5_SPEC;
    pub type Rdqp5 = crate::EnumBitfieldStruct<u8, Rdqp5_SPEC>;
    impl Rdqp5 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp6_SPEC;
    pub type Rdqp6 = crate::EnumBitfieldStruct<u8, Rdqp6_SPEC>;
    impl Rdqp6 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqp7_SPEC;
    pub type Rdqp7 = crate::EnumBitfieldStruct<u8, Rdqp7_SPEC>;
    impl Rdqp7 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrdqac_SPEC;
impl crate::sealed::RegSpec for Gwrdqac_SPEC {
    type DataType = u32;
}

#[doc = "RX Descriptor Queue Arbitration Control Register"]
pub type Gwrdqac = crate::RegValueT<Gwrdqac_SPEC>;

impl Gwrdqac {
    #[doc = "RX Descriptor Queue 0 Arbitration"]
    #[inline(always)]
    pub fn rdqa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        gwrdqac::Rdqa0,
        gwrdqac::Rdqa0,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            gwrdqac::Rdqa0,
            gwrdqac::Rdqa0,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue 1 Arbitration"]
    #[inline(always)]
    pub fn rdqa1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        gwrdqac::Rdqa1,
        gwrdqac::Rdqa1,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            gwrdqac::Rdqa1,
            gwrdqac::Rdqa1,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue 2 Arbitration"]
    #[inline(always)]
    pub fn rdqa2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        gwrdqac::Rdqa2,
        gwrdqac::Rdqa2,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            gwrdqac::Rdqa2,
            gwrdqac::Rdqa2,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue 3 Arbitration"]
    #[inline(always)]
    pub fn rdqa3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        gwrdqac::Rdqa3,
        gwrdqac::Rdqa3,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            gwrdqac::Rdqa3,
            gwrdqac::Rdqa3,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue 4 Arbitration"]
    #[inline(always)]
    pub fn rdqa4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        gwrdqac::Rdqa4,
        gwrdqac::Rdqa4,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            gwrdqac::Rdqa4,
            gwrdqac::Rdqa4,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue 5 Arbitration"]
    #[inline(always)]
    pub fn rdqa5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        gwrdqac::Rdqa5,
        gwrdqac::Rdqa5,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            gwrdqac::Rdqa5,
            gwrdqac::Rdqa5,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue 6 Arbitration"]
    #[inline(always)]
    pub fn rdqa6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        gwrdqac::Rdqa6,
        gwrdqac::Rdqa6,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            gwrdqac::Rdqa6,
            gwrdqac::Rdqa6,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Queue 7 Arbitration"]
    #[inline(always)]
    pub fn rdqa7(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        gwrdqac::Rdqa7,
        gwrdqac::Rdqa7,
        Gwrdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            gwrdqac::Rdqa7,
            gwrdqac::Rdqa7,
            Gwrdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwrdqac {
    #[inline(always)]
    fn default() -> Gwrdqac {
        <crate::RegValueT<Gwrdqac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwrdqac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa0_SPEC;
    pub type Rdqa0 = crate::EnumBitfieldStruct<u8, Rdqa0_SPEC>;
    impl Rdqa0 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa1_SPEC;
    pub type Rdqa1 = crate::EnumBitfieldStruct<u8, Rdqa1_SPEC>;
    impl Rdqa1 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa2_SPEC;
    pub type Rdqa2 = crate::EnumBitfieldStruct<u8, Rdqa2_SPEC>;
    impl Rdqa2 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa3_SPEC;
    pub type Rdqa3 = crate::EnumBitfieldStruct<u8, Rdqa3_SPEC>;
    impl Rdqa3 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa4_SPEC;
    pub type Rdqa4 = crate::EnumBitfieldStruct<u8, Rdqa4_SPEC>;
    impl Rdqa4 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa5_SPEC;
    pub type Rdqa5 = crate::EnumBitfieldStruct<u8, Rdqa5_SPEC>;
    impl Rdqa5 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa6_SPEC;
    pub type Rdqa6 = crate::EnumBitfieldStruct<u8, Rdqa6_SPEC>;
    impl Rdqa6 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdqa7_SPEC;
    pub type Rdqa7 = crate::EnumBitfieldStruct<u8, Rdqa7_SPEC>;
    impl Rdqa7 {
        #[doc = "Queue i strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrgc_SPEC;
impl crate::sealed::RegSpec for Gwrgc_SPEC {
    type DataType = u32;
}

#[doc = "RX General Configuration Register"]
pub type Gwrgc = crate::RegValueT<Gwrgc_SPEC>;

impl Gwrgc {
    #[doc = "Receive CRC Pass Through \\[802.3\\]"]
    #[inline(always)]
    pub fn rcpt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwrgc::Rcpt,
        gwrgc::Rcpt,
        Gwrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwrgc::Rcpt,
            gwrgc::Rcpt,
            Gwrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwrgc {
    #[inline(always)]
    fn default() -> Gwrgc {
        <crate::RegValueT<Gwrgc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwrgc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcpt_SPEC;
    pub type Rcpt = crate::EnumBitfieldStruct<u8, Rcpt_SPEC>;
    impl Rcpt {
        #[doc = "FCS is not passed to GWCPU. If a descriptor is received from forwarding engine with FDESCR.FI equal to 1, GWCA remove the last 4 bytes of the Frame Data (FCS)."]
        pub const _0: Self = Self::new(0);

        #[doc = "FCS is passed to GWCPU GWCA don’t remove FCS from the Frame which has one (a descriptor is received from forwarding engine with FDESCR.FI equal to 1) if the frame has not been modified by the switch (No update due to VLAN tagging/un-tagging and no update requested by L3 routing \\[FWD\\])."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrmfsc_SPEC;
impl crate::sealed::RegSpec for Gwrmfsc_SPEC {
    type DataType = u32;
}

#[doc = "Reception Maximum Frame Size Configuration Register %s"]
pub type Gwrmfsc = crate::RegValueT<Gwrmfsc_SPEC>;

impl Gwrmfsc {
    #[doc = "Maximum Frame Size"]
    #[inline(always)]
    pub fn mfs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwrmfsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwrmfsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwrmfsc {
    #[inline(always)]
    fn default() -> Gwrmfsc {
        <crate::RegValueT<Gwrmfsc_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrdqdc_SPEC;
impl crate::sealed::RegSpec for Gwrdqdc_SPEC {
    type DataType = u32;
}

#[doc = "Reception Descriptor Queue %s Depth Configuration Register"]
pub type Gwrdqdc = crate::RegValueT<Gwrdqdc_SPEC>;

impl Gwrdqdc {
    #[doc = "Descriptor Queue Depth"]
    #[inline(always)]
    pub fn dqd(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Gwrdqdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Gwrdqdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwrdqdc {
    #[inline(always)]
    fn default() -> Gwrdqdc {
        <crate::RegValueT<Gwrdqdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrdqm_SPEC;
impl crate::sealed::RegSpec for Gwrdqm_SPEC {
    type DataType = u32;
}

#[doc = "RX Descriptor Queue %s Monitoring Register"]
pub type Gwrdqm = crate::RegValueT<Gwrdqm_SPEC>;

impl Gwrdqm {
    #[doc = "Descriptor Number in Queue"]
    #[inline(always)]
    pub fn dnq(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Gwrdqm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Gwrdqm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwrdqm {
    #[inline(always)]
    fn default() -> Gwrdqm {
        <crate::RegValueT<Gwrdqm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrdqmlm_SPEC;
impl crate::sealed::RegSpec for Gwrdqmlm_SPEC {
    type DataType = u32;
}

#[doc = "RX Descriptor Queue %s Max Level Monitoring Register"]
pub type Gwrdqmlm = crate::RegValueT<Gwrdqmlm_SPEC>;

impl Gwrdqmlm {
    #[doc = "Descriptor Max Level in Queue"]
    #[inline(always)]
    pub fn dmlq(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Gwrdqmlm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Gwrdqmlm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwrdqmlm {
    #[inline(always)]
    fn default() -> Gwrdqmlm {
        <crate::RegValueT<Gwrdqmlm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmtirm_SPEC;
impl crate::sealed::RegSpec for Gwmtirm_SPEC {
    type DataType = u32;
}

#[doc = "Multicast Table Initialization Register Monitoring Register"]
pub type Gwmtirm = crate::RegValueT<Gwmtirm_SPEC>;

impl Gwmtirm {
    #[doc = "Multicast Table Initialization Ongoing"]
    #[inline(always)]
    pub fn mtiog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Gwmtirm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gwmtirm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Multicast Table Ready"]
    #[inline(always)]
    pub fn mtr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gwmtirm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gwmtirm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gwmtirm {
    #[inline(always)]
    fn default() -> Gwmtirm {
        <crate::RegValueT<Gwmtirm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmstls_SPEC;
impl crate::sealed::RegSpec for Gwmstls_SPEC {
    type DataType = u32;
}

#[doc = "Multicast Table Learning Setting Register"]
pub type Gwmstls = crate::RegValueT<Gwmstls_SPEC>;

impl Gwmstls {
    #[doc = "Multicast Next Descriptor Chain Number Learn"]
    #[inline(always)]
    pub fn mnrcnl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Gwmstls_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Gwmstls_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multicast Number Learn"]
    #[inline(always)]
    pub fn mnl(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Gwmstls_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Gwmstls_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multicast Setting Entry Number Learn"]
    #[inline(always)]
    pub fn msenl(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Gwmstls_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Gwmstls_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwmstls {
    #[inline(always)]
    fn default() -> Gwmstls {
        <crate::RegValueT<Gwmstls_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmstlr_SPEC;
impl crate::sealed::RegSpec for Gwmstlr_SPEC {
    type DataType = u32;
}

#[doc = "Multicast Table Learning Result Register"]
pub type Gwmstlr = crate::RegValueT<Gwmstlr_SPEC>;

impl Gwmstlr {
    #[doc = "Multicast Table Learning Fail"]
    #[inline(always)]
    pub fn mtlf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwmstlr::Mtlf,
        gwmstlr::Mtlf,
        Gwmstlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwmstlr::Mtlf,
            gwmstlr::Mtlf,
            Gwmstlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Table Learning"]
    #[inline(always)]
    pub fn mtl(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gwmstlr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gwmstlr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gwmstlr {
    #[inline(always)]
    fn default() -> Gwmstlr {
        <crate::RegValueT<Gwmstlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwmstlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtlf_SPEC;
    pub type Mtlf = crate::EnumBitfieldStruct<u8, Mtlf_SPEC>;
    impl Mtlf {
        #[doc = "Entry learning did not fail because the Multicast table is not ready."]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed the Multicast table is not ready."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmstss_SPEC;
impl crate::sealed::RegSpec for Gwmstss_SPEC {
    type DataType = u32;
}

#[doc = "Multicast Table Searching Setting Register"]
pub type Gwmstss = crate::RegValueT<Gwmstss_SPEC>;

impl Gwmstss {
    #[doc = "Multicast Setting Entry Number Search"]
    #[inline(always)]
    pub fn msens(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Gwmstss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Gwmstss_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwmstss {
    #[inline(always)]
    fn default() -> Gwmstss {
        <crate::RegValueT<Gwmstss_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmstsr_SPEC;
impl crate::sealed::RegSpec for Gwmstsr_SPEC {
    type DataType = u32;
}

#[doc = "Multicast Table Searching Result Register"]
pub type Gwmstsr = crate::RegValueT<Gwmstsr_SPEC>;

impl Gwmstsr {
    #[doc = "Multicast Next RX Descriptor Chain Number Result"]
    #[inline(always)]
    pub fn mnrcnr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Gwmstsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Gwmstsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multicast Number Result"]
    #[inline(always)]
    pub fn mnr(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Gwmstsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Gwmstsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Multicast Table Searching"]
    #[inline(always)]
    pub fn mts(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gwmstsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gwmstsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gwmstsr {
    #[inline(always)]
    fn default() -> Gwmstsr {
        <crate::RegValueT<Gwmstsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmac0_SPEC;
impl crate::sealed::RegSpec for Gwmac0_SPEC {
    type DataType = u32;
}

#[doc = "MAC Address Configuration Register 0"]
pub type Gwmac0 = crate::RegValueT<Gwmac0_SPEC>;

impl Gwmac0 {
    #[doc = "MAC Address Upper Part"]
    #[inline(always)]
    pub fn mau(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwmac0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwmac0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwmac0 {
    #[inline(always)]
    fn default() -> Gwmac0 {
        <crate::RegValueT<Gwmac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmac1_SPEC;
impl crate::sealed::RegSpec for Gwmac1_SPEC {
    type DataType = u32;
}

#[doc = "MAC Address Configuration Register 1"]
pub type Gwmac1 = crate::RegValueT<Gwmac1_SPEC>;

impl Gwmac1 {
    #[doc = "MAC Address Lower Part"]
    #[inline(always)]
    pub fn mal(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwmac1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gwmac1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwmac1 {
    #[inline(always)]
    fn default() -> Gwmac1 {
        <crate::RegValueT<Gwmac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwvcc_SPEC;
impl crate::sealed::RegSpec for Gwvcc_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Control Configuration Register"]
pub type Gwvcc = crate::RegValueT<Gwvcc_SPEC>;

impl Gwvcc {
    #[doc = "VLAN Ingress Mode"]
    #[inline(always)]
    pub fn vim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwvcc::Vim,
        gwvcc::Vim,
        Gwvcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwvcc::Vim,
            gwvcc::Vim,
            Gwvcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Egress Mode"]
    #[inline(always)]
    pub fn vem(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        gwvcc::Vem,
        gwvcc::Vem,
        Gwvcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            gwvcc::Vem,
            gwvcc::Vem,
            Gwvcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwvcc {
    #[inline(always)]
    fn default() -> Gwvcc {
        <crate::RegValueT<Gwvcc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwvcc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vim_SPEC;
    pub type Vim = crate::EnumBitfieldStruct<u8, Vim_SPEC>;
    impl Vim {
        #[doc = "Incoming VLAN mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Port based VLAN mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vem_SPEC;
    pub type Vem = crate::EnumBitfieldStruct<u8, Vem_SPEC>;
    impl Vem {
        #[doc = "No VLAN mode, frames are transmitted with no VLAN."]
        pub const _000: Self = Self::new(0);

        #[doc = "C-TAG VLAN mode, frames are transmitted with ingress C-TAG if there is one stored in the Local RAM."]
        pub const _001: Self = Self::new(1);

        #[doc = "HW C-TAG VLAN mode, frames are transmitted with the C-TAG stored in local RAM."]
        pub const _010: Self = Self::new(2);

        #[doc = "SC-TAG VLAN mode, frames are transmitted with ingress C-TAG and S-TAG if they were stored in the Local RAM."]
        pub const _011: Self = Self::new(3);

        #[doc = "HW SC-TAG VLAN mode, frames are transmitted with the C-TAG and S-TAG stored in local RAM."]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwvtc_SPEC;
impl crate::sealed::RegSpec for Gwvtc_SPEC {
    type DataType = u32;
}

#[doc = "VLAN TAG Configuration Register"]
pub type Gwvtc = crate::RegValueT<Gwvtc_SPEC>;

impl Gwvtc {
    #[doc = "C-TAG VLAN"]
    #[inline(always)]
    pub fn ctv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Gwvtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Gwvtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "C-TAG PCP"]
    #[inline(always)]
    pub fn ctp(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Gwvtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Gwvtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "C-TAG DEI"]
    #[inline(always)]
    pub fn ctd(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gwvtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gwvtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "S-TAG VLAN"]
    #[inline(always)]
    pub fn stv(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Gwvtc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Gwvtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "S-TAG PCP"]
    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Gwvtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Gwvtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "S-TAG DEI"]
    #[inline(always)]
    pub fn std(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gwvtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gwvtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gwvtc {
    #[inline(always)]
    fn default() -> Gwvtc {
        <crate::RegValueT<Gwvtc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwttfc_SPEC;
impl crate::sealed::RegSpec for Gwttfc_SPEC {
    type DataType = u32;
}

#[doc = "Transmission TAG Filtering Configuration Register"]
pub type Gwttfc = crate::RegValueT<Gwttfc_SPEC>;

impl Gwttfc {
    #[doc = "No Tag"]
    #[inline(always)]
    pub fn nt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwttfc::Nt,
        gwttfc::Nt,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwttfc::Nt,
            gwttfc::Nt,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "R-TAG"]
    #[inline(always)]
    pub fn rt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gwttfc::Rt,
        gwttfc::Rt,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gwttfc::Rt,
            gwttfc::Rt,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CoS-TAG"]
    #[inline(always)]
    pub fn cst(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gwttfc::Cst,
        gwttfc::Cst,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gwttfc::Cst,
            gwttfc::Cst,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CoSR-TAG"]
    #[inline(always)]
    pub fn csrt(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gwttfc::Csrt,
        gwttfc::Csrt,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gwttfc::Csrt,
            gwttfc::Csrt,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-TAG"]
    #[inline(always)]
    pub fn ct(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gwttfc::Ct,
        gwttfc::Ct,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gwttfc::Ct,
            gwttfc::Ct,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CR-TAG"]
    #[inline(always)]
    pub fn crt(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gwttfc::Crt,
        gwttfc::Crt,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gwttfc::Crt,
            gwttfc::Crt,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SC-TAG"]
    #[inline(always)]
    pub fn sct(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gwttfc::Sct,
        gwttfc::Sct,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gwttfc::Sct,
            gwttfc::Sct,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCR-TAG"]
    #[inline(always)]
    pub fn scrt(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gwttfc::Scrt,
        gwttfc::Scrt,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gwttfc::Scrt,
            gwttfc::Scrt,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Unknown TAG"]
    #[inline(always)]
    pub fn ut(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gwttfc::Ut,
        gwttfc::Ut,
        Gwttfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gwttfc::Ut,
            gwttfc::Ut,
            Gwttfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwttfc {
    #[inline(always)]
    fn default() -> Gwttfc {
        <crate::RegValueT<Gwttfc_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod gwttfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nt_SPEC;
    pub type Nt = crate::EnumBitfieldStruct<u8, Nt_SPEC>;
    impl Nt {
        #[doc = "No Tag frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "No Tag frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rt_SPEC;
    pub type Rt = crate::EnumBitfieldStruct<u8, Rt_SPEC>;
    impl Rt {
        #[doc = "R-TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "R-TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cst_SPEC;
    pub type Cst = crate::EnumBitfieldStruct<u8, Cst_SPEC>;
    impl Cst {
        #[doc = "CoS-TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "CoS-TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csrt_SPEC;
    pub type Csrt = crate::EnumBitfieldStruct<u8, Csrt_SPEC>;
    impl Csrt {
        #[doc = "CoSR-TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "CoSR-TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ct_SPEC;
    pub type Ct = crate::EnumBitfieldStruct<u8, Ct_SPEC>;
    impl Ct {
        #[doc = "C-TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crt_SPEC;
    pub type Crt = crate::EnumBitfieldStruct<u8, Crt_SPEC>;
    impl Crt {
        #[doc = "CR-TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "CR-TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sct_SPEC;
    pub type Sct = crate::EnumBitfieldStruct<u8, Sct_SPEC>;
    impl Sct {
        #[doc = "SC-TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "SC-TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scrt_SPEC;
    pub type Scrt = crate::EnumBitfieldStruct<u8, Scrt_SPEC>;
    impl Scrt {
        #[doc = "SCR-TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCR-TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ut_SPEC;
    pub type Ut = crate::EnumBitfieldStruct<u8, Ut_SPEC>;
    impl Ut {
        #[doc = "Unknown Tag frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown Tag frame rejected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtdcac0_SPEC;
impl crate::sealed::RegSpec for Gwtdcac0_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Descriptor Chain %s Address Configuration Register 0"]
pub type Gwtdcac0 = crate::RegValueT<Gwtdcac0_SPEC>;

impl Gwtdcac0 {
    #[doc = "TS Descriptor Chain s Current Address Upper Part"]
    #[inline(always)]
    pub fn tsccau(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gwtdcac0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gwtdcac0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtdcac0 {
    #[inline(always)]
    fn default() -> Gwtdcac0 {
        <crate::RegValueT<Gwtdcac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtdcac1_SPEC;
impl crate::sealed::RegSpec for Gwtdcac1_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Descriptor Chain %s Address Configuration Register 1"]
pub type Gwtdcac1 = crate::RegValueT<Gwtdcac1_SPEC>;

impl Gwtdcac1 {
    #[doc = "TS Descriptor Chain s Current Address Lower Part"]
    #[inline(always)]
    pub fn tsccal(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwtdcac1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Gwtdcac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwtdcac1 {
    #[inline(always)]
    fn default() -> Gwtdcac1 {
        <crate::RegValueT<Gwtdcac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsdcc_SPEC;
impl crate::sealed::RegSpec for Gwtsdcc_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Descriptor Chain %s Configuration Register"]
pub type Gwtsdcc = crate::RegValueT<Gwtsdcc_SPEC>;

impl Gwtsdcc {
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub fn te(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gwtsdcc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gwtsdcc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Chain Select"]
    #[inline(always)]
    pub fn dcs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gwtsdcc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gwtsdcc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OS ID"]
    #[inline(always)]
    pub fn osid(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Gwtsdcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Gwtsdcc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtsdcc {
    #[inline(always)]
    fn default() -> Gwtsdcc {
        <crate::RegValueT<Gwtsdcc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsnm_SPEC;
impl crate::sealed::RegSpec for Gwtsnm_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Number Monitoring Register"]
pub type Gwtsnm = crate::RegValueT<Gwtsnm_SPEC>;

impl Gwtsnm {
    #[doc = "Timestamp Number in Timestamp RAM"]
    #[inline(always)]
    pub fn tntr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Gwtsnm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Gwtsnm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtsnm {
    #[inline(always)]
    fn default() -> Gwtsnm {
        <crate::RegValueT<Gwtsnm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsmnm_SPEC;
impl crate::sealed::RegSpec for Gwtsmnm_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Maximum Number Monitoring Register"]
pub type Gwtsmnm = crate::RegValueT<Gwtsmnm_SPEC>;

impl Gwtsmnm {
    #[doc = "Timestamp Maximum Number in Timestamp RAM"]
    #[inline(always)]
    pub fn tmntr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Gwtsmnm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Gwtsmnm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtsmnm {
    #[inline(always)]
    fn default() -> Gwtsmnm {
        <crate::RegValueT<Gwtsmnm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwac_SPEC;
impl crate::sealed::RegSpec for Gwac_SPEC {
    type DataType = u32;
}

#[doc = "AXI Control Register"]
pub type Gwac = crate::RegValueT<Gwac_SPEC>;

impl Gwac {
    #[doc = "AXI Master Pause Request"]
    #[inline(always)]
    pub fn ampr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwac::Ampr,
        gwac::Ampr,
        Gwac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwac::Ampr,
            gwac::Ampr,
            Gwac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AXI Master Paused"]
    #[inline(always)]
    pub fn amp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gwac::Amp,
        gwac::Amp,
        Gwac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gwac::Amp,
            gwac::Amp,
            Gwac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwac {
    #[inline(always)]
    fn default() -> Gwac {
        <crate::RegValueT<Gwac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampr_SPEC;
    pub type Ampr = crate::EnumBitfieldStruct<u8, Ampr_SPEC>;
    impl Ampr {
        #[doc = "AXI master pause not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "AXI master pause requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amp_SPEC;
    pub type Amp = crate::EnumBitfieldStruct<u8, Amp_SPEC>;
    impl Amp {
        #[doc = "AXI master not paused (transaction ongoing)"]
        pub const _0: Self = Self::new(0);

        #[doc = "AXI master paused (no transaction ongoing)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdcbac0_SPEC;
impl crate::sealed::RegSpec for Gwdcbac0_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Chain Base Address Configuration Register 0"]
pub type Gwdcbac0 = crate::RegValueT<Gwdcbac0_SPEC>;

impl Gwdcbac0 {
    #[doc = "Descriptor Chain Base Address Upper Part"]
    #[inline(always)]
    pub fn dcbau(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gwdcbac0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gwdcbac0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdcbac0 {
    #[inline(always)]
    fn default() -> Gwdcbac0 {
        <crate::RegValueT<Gwdcbac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdcbac1_SPEC;
impl crate::sealed::RegSpec for Gwdcbac1_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Chain Base Address Configuration Register 1"]
pub type Gwdcbac1 = crate::RegValueT<Gwdcbac1_SPEC>;

impl Gwdcbac1 {
    #[doc = "Descriptor Chain Base Address Lower Part"]
    #[inline(always)]
    pub fn dcbal(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwdcbac1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Gwdcbac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwdcbac1 {
    #[inline(always)]
    fn default() -> Gwdcbac1 {
        <crate::RegValueT<Gwdcbac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwmdnc_SPEC;
impl crate::sealed::RegSpec for Gwmdnc_SPEC {
    type DataType = u32;
}

#[doc = "Maximum Descriptor Number Configuration Register"]
pub type Gwmdnc = crate::RegValueT<Gwmdnc_SPEC>;

impl Gwmdnc {
    #[doc = "RX Descriptor Maximum Number"]
    #[inline(always)]
    pub fn rxdmn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Gwmdnc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Gwmdnc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TX Descriptor Maximum Number"]
    #[inline(always)]
    pub fn txdmn(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Gwmdnc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Gwmdnc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timestamp Descriptor Maximum Number"]
    #[inline(always)]
    pub fn tsdmn(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Gwmdnc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Gwmdnc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwmdnc {
    #[inline(always)]
    fn default() -> Gwmdnc {
        <crate::RegValueT<Gwmdnc_SPEC> as RegisterValue<_>>::new(65793)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtrc0_SPEC;
impl crate::sealed::RegSpec for Gwtrc0_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Request Configuration Register"]
pub type Gwtrc0 = crate::RegValueT<Gwtrc0_SPEC>;

impl NoBitfieldReg<Gwtrc0_SPEC> for Gwtrc0 {}
impl ::core::default::Default for Gwtrc0 {
    #[inline(always)]
    fn default() -> Gwtrc0 {
        <crate::RegValueT<Gwtrc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtrc1_SPEC;
impl crate::sealed::RegSpec for Gwtrc1_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Request Configuration Register"]
pub type Gwtrc1 = crate::RegValueT<Gwtrc1_SPEC>;

impl NoBitfieldReg<Gwtrc1_SPEC> for Gwtrc1 {}
impl ::core::default::Default for Gwtrc1 {
    #[inline(always)]
    fn default() -> Gwtrc1 {
        <crate::RegValueT<Gwtrc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GwtpCp_SPEC;
impl crate::sealed::RegSpec for GwtpCp_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Pause Configuration Register p (p = 0 to 1)"]
pub type GwtpCp = crate::RegValueT<GwtpCp_SPEC>;

impl GwtpCp {
    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwtpcp::Pppl0,
        gwtpcp::Pppl0,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwtpcp::Pppl0,
            gwtpcp::Pppl0,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gwtpcp::Pppl1,
        gwtpcp::Pppl1,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gwtpcp::Pppl1,
            gwtpcp::Pppl1,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gwtpcp::Pppl2,
        gwtpcp::Pppl2,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gwtpcp::Pppl2,
            gwtpcp::Pppl2,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gwtpcp::Pppl3,
        gwtpcp::Pppl3,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gwtpcp::Pppl3,
            gwtpcp::Pppl3,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gwtpcp::Pppl4,
        gwtpcp::Pppl4,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gwtpcp::Pppl4,
            gwtpcp::Pppl4,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gwtpcp::Pppl5,
        gwtpcp::Pppl5,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gwtpcp::Pppl5,
            gwtpcp::Pppl5,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gwtpcp::Pppl6,
        gwtpcp::Pppl6,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gwtpcp::Pppl6,
            gwtpcp::Pppl6,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Per Priority Pause Level i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn pppl7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gwtpcp::Pppl7,
        gwtpcp::Pppl7,
        GwtpCp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gwtpcp::Pppl7,
            gwtpcp::Pppl7,
            GwtpCp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GwtpCp {
    #[inline(always)]
    fn default() -> GwtpCp {
        <crate::RegValueT<GwtpCp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwtpcp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl0_SPEC;
    pub type Pppl0 = crate::EnumBitfieldStruct<u8, Pppl0_SPEC>;
    impl Pppl0 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl1_SPEC;
    pub type Pppl1 = crate::EnumBitfieldStruct<u8, Pppl1_SPEC>;
    impl Pppl1 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl2_SPEC;
    pub type Pppl2 = crate::EnumBitfieldStruct<u8, Pppl2_SPEC>;
    impl Pppl2 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl3_SPEC;
    pub type Pppl3 = crate::EnumBitfieldStruct<u8, Pppl3_SPEC>;
    impl Pppl3 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl4_SPEC;
    pub type Pppl4 = crate::EnumBitfieldStruct<u8, Pppl4_SPEC>;
    impl Pppl4 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl5_SPEC;
    pub type Pppl5 = crate::EnumBitfieldStruct<u8, Pppl5_SPEC>;
    impl Pppl5 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl6_SPEC;
    pub type Pppl6 = crate::EnumBitfieldStruct<u8, Pppl6_SPEC>;
    impl Pppl6 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pppl7_SPEC;
    pub type Pppl7 = crate::EnumBitfieldStruct<u8, Pppl7_SPEC>;
    impl Pppl7 {
        #[doc = "Transmission queues with their priority GWDCCi.DCPi set to i are not paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmission queues with the priority GWDCCi.DCPi set to i are paused when pause level p is set (PAUSE\\[2\\]\\[p\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwarirm_SPEC;
impl crate::sealed::RegSpec for Gwarirm_SPEC {
    type DataType = u32;
}

#[doc = "AXI RAM Initialization Register Monitoring Register"]
pub type Gwarirm = crate::RegValueT<Gwarirm_SPEC>;

impl Gwarirm {
    #[doc = "AXI RAM Initialization Ongoing"]
    #[inline(always)]
    pub fn ariog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Gwarirm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gwarirm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI RAM Ready"]
    #[inline(always)]
    pub fn arr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gwarirm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gwarirm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gwarirm {
    #[inline(always)]
    fn default() -> Gwarirm {
        <crate::RegValueT<Gwarirm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdcc_SPEC;
impl crate::sealed::RegSpec for Gwdcc_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Chain %s Configuration Register"]
pub type Gwdcc = crate::RegValueT<Gwdcc_SPEC>;

impl Gwdcc {
    #[doc = "Synchronization Mode"]
    #[inline(always)]
    pub fn sm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gwdcc::Sm,
        gwdcc::Sm,
        Gwdcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gwdcc::Sm,
            gwdcc::Sm,
            Gwdcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Extended Descriptor Enable"]
    #[inline(always)]
    pub fn ede(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gwdcc::Ede,
        gwdcc::Ede,
        Gwdcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gwdcc::Ede,
            gwdcc::Ede,
            Gwdcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable Timestamp Storage"]
    #[inline(always)]
    pub fn ets(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gwdcc::Ets,
        gwdcc::Ets,
        Gwdcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gwdcc::Ets,
            gwdcc::Ets,
            Gwdcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Security Level"]
    #[inline(always)]
    pub fn sl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gwdcc::Sl,
        gwdcc::Sl,
        Gwdcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gwdcc::Sl,
            gwdcc::Sl,
            Gwdcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue Type"]
    #[inline(always)]
    pub fn dqt(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gwdcc::Dqt,
        gwdcc::Dqt,
        Gwdcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gwdcc::Dqt,
            gwdcc::Dqt,
            Gwdcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Chain Priority"]
    #[inline(always)]
    pub fn dcp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        gwdcc::Dcp,
        gwdcc::Dcp,
        Gwdcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            gwdcc::Dcp,
            gwdcc::Dcp,
            Gwdcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Base Address Load Request"]
    #[inline(always)]
    pub fn balr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gwdcc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gwdcc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OS ID"]
    #[inline(always)]
    pub fn osid(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Gwdcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Gwdcc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdcc {
    #[inline(always)]
    fn default() -> Gwdcc {
        <crate::RegValueT<Gwdcc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwdcc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sm_SPEC;
    pub type Sm = crate::EnumBitfieldStruct<u8, Sm_SPEC>;
    impl Sm {
        #[doc = "Normal mode (full descriptor write back)"]
        pub const _00: Self = Self::new(0);

        #[doc = "No-write-back mode (no descriptor write back)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Keep-DT mode (no update of DT field at descriptor write back)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ede_SPEC;
    pub type Ede = crate::EnumBitfieldStruct<u8, Ede_SPEC>;
    impl Ede {
        #[doc = "Basic Descriptor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extended Descriptor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ets_SPEC;
    pub type Ets = crate::EnumBitfieldStruct<u8, Ets_SPEC>;
    impl Ets {
        #[doc = "Timestamp is not added in the Descriptor."]
        pub const _0: Self = Self::new(0);

        #[doc = "Timestamp is added in the Descriptor."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl_SPEC;
    pub type Sl = crate::EnumBitfieldStruct<u8, Sl_SPEC>;
    impl Sl {
        #[doc = "Chain i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Chain i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqt_SPEC;
    pub type Dqt = crate::EnumBitfieldStruct<u8, Dqt_SPEC>;
    impl Dqt {
        #[doc = "Descriptor queue i is a reception queue."]
        pub const _0: Self = Self::new(0);

        #[doc = "Descriptor queue i is a transmission queue."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcp_SPEC;
    pub type Dcp = crate::EnumBitfieldStruct<u8, Dcp_SPEC>;
    impl Dcp {
        #[doc = "lowest priority"]
        pub const _000: Self = Self::new(0);

        #[doc = "highest priority"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwaarss_SPEC;
impl crate::sealed::RegSpec for Gwaarss_SPEC {
    type DataType = u32;
}

#[doc = "AXI Address RAM Searching Setting Register"]
pub type Gwaarss = crate::RegValueT<Gwaarss_SPEC>;

impl Gwaarss {
    #[doc = "AXI Address RAM Address"]
    #[inline(always)]
    pub fn aara(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Gwaarss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Gwaarss_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwaarss {
    #[inline(always)]
    fn default() -> Gwaarss {
        <crate::RegValueT<Gwaarss_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwaarsr0_SPEC;
impl crate::sealed::RegSpec for Gwaarsr0_SPEC {
    type DataType = u32;
}

#[doc = "AXI Address RAM Searching Result Register 0"]
pub type Gwaarsr0 = crate::RegValueT<Gwaarsr0_SPEC>;

impl Gwaarsr0 {
    #[doc = "AXI Current Address Result Upper Part"]
    #[inline(always)]
    pub fn acaru(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gwaarsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gwaarsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "AXI Address RAM Searching"]
    #[inline(always)]
    pub fn aars(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Gwaarsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gwaarsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gwaarsr0 {
    #[inline(always)]
    fn default() -> Gwaarsr0 {
        <crate::RegValueT<Gwaarsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwaarsr1_SPEC;
impl crate::sealed::RegSpec for Gwaarsr1_SPEC {
    type DataType = u32;
}

#[doc = "AXI Address RAM Searching Result Register 1"]
pub type Gwaarsr1 = crate::RegValueT<Gwaarsr1_SPEC>;

impl Gwaarsr1 {
    #[doc = "AXI Current Address Result Lower Part"]
    #[inline(always)]
    pub fn acarl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwaarsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gwaarsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwaarsr1 {
    #[inline(always)]
    fn default() -> Gwaarsr1 {
        <crate::RegValueT<Gwaarsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidauas_SPEC;
impl crate::sealed::RegSpec for Gwidauas_SPEC {
    type DataType = u32;
}

#[doc = "Incremental Data Area %s Used Area Size Register"]
pub type Gwidauas = crate::RegValueT<Gwidauas_SPEC>;

impl Gwidauas {
    #[doc = "Incremental Data Area Used Size of RX Descriptor Chain i"]
    #[inline(always)]
    pub fn idauas(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Gwidauas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Gwidauas_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwidauas {
    #[inline(always)]
    fn default() -> Gwidauas {
        <crate::RegValueT<Gwidauas_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidasm_SPEC;
impl crate::sealed::RegSpec for Gwidasm_SPEC {
    type DataType = u32;
}

#[doc = "Incremental Data Area %s Size Monitoring Register"]
pub type Gwidasm = crate::RegValueT<Gwidasm_SPEC>;

impl Gwidasm {
    #[doc = "Incremental Data Area Size of RX Descriptor Chain i"]
    #[inline(always)]
    pub fn idas(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Gwidasm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Gwidasm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwidasm {
    #[inline(always)]
    fn default() -> Gwidasm {
        <crate::RegValueT<Gwidasm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidasam0_SPEC;
impl crate::sealed::RegSpec for Gwidasam0_SPEC {
    type DataType = u32;
}

#[doc = "Incremental Data Area %s Start Address Monitoring Register 0"]
pub type Gwidasam0 = crate::RegValueT<Gwidasam0_SPEC>;

impl Gwidasam0 {
    #[doc = "Incremental Data Area Start Address Upper Part of RX Descriptor Chain i"]
    #[inline(always)]
    pub fn idasau(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gwidasam0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gwidasam0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwidasam0 {
    #[inline(always)]
    fn default() -> Gwidasam0 {
        <crate::RegValueT<Gwidasam0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidasam1_SPEC;
impl crate::sealed::RegSpec for Gwidasam1_SPEC {
    type DataType = u32;
}

#[doc = "Incremental Data Area %s Start Address Monitoring Register 1"]
pub type Gwidasam1 = crate::RegValueT<Gwidasam1_SPEC>;

impl Gwidasam1 {
    #[doc = "Incremental Data Area Start Address Lower Part of RX Descriptor Chain i"]
    #[inline(always)]
    pub fn idasal(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwidasam1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Gwidasam1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwidasam1 {
    #[inline(always)]
    fn default() -> Gwidasam1 {
        <crate::RegValueT<Gwidasam1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidacam0_SPEC;
impl crate::sealed::RegSpec for Gwidacam0_SPEC {
    type DataType = u32;
}

#[doc = "Incremental Data Area %s Current Address Monitoring Register 0"]
pub type Gwidacam0 = crate::RegValueT<Gwidacam0_SPEC>;

impl Gwidacam0 {
    #[doc = "Incremental Data Area Current Address Upper Part of RX Descriptor Chain i"]
    #[inline(always)]
    pub fn idacau(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gwidacam0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gwidacam0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwidacam0 {
    #[inline(always)]
    fn default() -> Gwidacam0 {
        <crate::RegValueT<Gwidacam0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidacam1_SPEC;
impl crate::sealed::RegSpec for Gwidacam1_SPEC {
    type DataType = u32;
}

#[doc = "Incremental Data Area %s Current Address Monitoring Register 1"]
pub type Gwidacam1 = crate::RegValueT<Gwidacam1_SPEC>;

impl Gwidacam1 {
    #[doc = "Incremental Data Area Current Address Lower Part of RX Descriptor Chain i"]
    #[inline(always)]
    pub fn idacal(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwidacam1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Gwidacam1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwidacam1 {
    #[inline(always)]
    fn default() -> Gwidacam1 {
        <crate::RegValueT<Gwidacam1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwgrlc_SPEC;
impl crate::sealed::RegSpec for Gwgrlc_SPEC {
    type DataType = u32;
}

#[doc = "Global Rate Limiter Configuration Register"]
pub type Gwgrlc = crate::RegValueT<Gwgrlc_SPEC>;

impl Gwgrlc {
    #[doc = "Global Rate Limiter Incremental Value"]
    #[inline(always)]
    pub fn grliv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwgrlc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwgrlc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Global Rate Limiter Enable"]
    #[inline(always)]
    pub fn grle(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gwgrlc::Grle,
        gwgrlc::Grle,
        Gwgrlc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gwgrlc::Grle,
            gwgrlc::Grle,
            Gwgrlc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Global Rate Limiter Upper Limit Reached Status Flag"]
    #[inline(always)]
    pub fn grlulrs(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gwgrlc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gwgrlc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gwgrlc {
    #[inline(always)]
    fn default() -> Gwgrlc {
        <crate::RegValueT<Gwgrlc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwgrlc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grle_SPEC;
    pub type Grle = crate::EnumBitfieldStruct<u8, Grle_SPEC>;
    impl Grle {
        #[doc = "Global Rate limiter disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Global Rate limiter enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwgrlulc_SPEC;
impl crate::sealed::RegSpec for Gwgrlulc_SPEC {
    type DataType = u32;
}

#[doc = "Global Rate Limiter Upper Limit Configuration Register"]
pub type Gwgrlulc = crate::RegValueT<Gwgrlulc_SPEC>;

impl Gwgrlulc {
    #[doc = "Global Rate Limiter Upper Limit"]
    #[inline(always)]
    pub fn grlul(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Gwgrlulc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Gwgrlulc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwgrlulc {
    #[inline(always)]
    fn default() -> Gwgrlulc {
        <crate::RegValueT<Gwgrlulc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrlc_SPEC;
impl crate::sealed::RegSpec for Gwrlc_SPEC {
    type DataType = u32;
}

#[doc = "Rate Limiter %s Configuration Register"]
pub type Gwrlc = crate::RegValueT<Gwrlc_SPEC>;

impl Gwrlc {
    #[doc = "Rate Limiter Incremental Value"]
    #[inline(always)]
    pub fn rliv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Gwrlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Gwrlc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Rate Limiter Enable"]
    #[inline(always)]
    pub fn rle(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gwrlc::Rle,
        gwrlc::Rle,
        Gwrlc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gwrlc::Rle,
            gwrlc::Rle,
            Gwrlc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwrlc {
    #[inline(always)]
    fn default() -> Gwrlc {
        <crate::RegValueT<Gwrlc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwrlc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rle_SPEC;
    pub type Rle = crate::EnumBitfieldStruct<u8, Rle_SPEC>;
    impl Rle {
        #[doc = "Rate limiter i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rate limiter i enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrlulc_SPEC;
impl crate::sealed::RegSpec for Gwrlulc_SPEC {
    type DataType = u32;
}

#[doc = "Rate Limiter %s Upper Limit Configuration Register"]
pub type Gwrlulc = crate::RegValueT<Gwrlulc_SPEC>;

impl Gwrlulc {
    #[doc = "Rate Limiter Upper Limit"]
    #[inline(always)]
    pub fn rlul(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Gwrlulc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Gwrlulc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwrlulc {
    #[inline(always)]
    fn default() -> Gwrlulc {
        <crate::RegValueT<Gwrlulc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidpc_SPEC;
impl crate::sealed::RegSpec for Gwidpc_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Delay Prescaler Configuration Register"]
pub type Gwidpc = crate::RegValueT<Gwidpc_SPEC>;

impl Gwidpc {
    #[doc = "Interrupt Delay Prescaler Value"]
    #[inline(always)]
    pub fn idpv(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Gwidpc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Gwidpc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwidpc {
    #[inline(always)]
    fn default() -> Gwidpc {
        <crate::RegValueT<Gwidpc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrdcn_SPEC;
impl crate::sealed::RegSpec for Gwrdcn_SPEC {
    type DataType = u32;
}

#[doc = "Received Data Counter Register"]
pub type Gwrdcn = crate::RegValueT<Gwrdcn_SPEC>;

impl Gwrdcn {
    #[doc = "Received Data Number"]
    #[inline(always)]
    pub fn rdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwrdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gwrdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwrdcn {
    #[inline(always)]
    fn default() -> Gwrdcn {
        <crate::RegValueT<Gwrdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtdcn_SPEC;
impl crate::sealed::RegSpec for Gwtdcn_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Data Counter Register"]
pub type Gwtdcn = crate::RegValueT<Gwtdcn_SPEC>;

impl Gwtdcn {
    #[doc = "Transmitted Data Number"]
    #[inline(always)]
    pub fn tdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwtdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gwtdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtdcn {
    #[inline(always)]
    fn default() -> Gwtdcn {
        <crate::RegValueT<Gwtdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtscn_SPEC;
impl crate::sealed::RegSpec for Gwtscn_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Counter Register"]
pub type Gwtscn = crate::RegValueT<Gwtscn_SPEC>;

impl Gwtscn {
    #[doc = "Timestamp Number"]
    #[inline(always)]
    pub fn tn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gwtscn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gwtscn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtscn {
    #[inline(always)]
    fn default() -> Gwtscn {
        <crate::RegValueT<Gwtscn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsovfecn_SPEC;
impl crate::sealed::RegSpec for Gwtsovfecn_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Overflow Error Counter Register"]
pub type Gwtsovfecn = crate::RegValueT<Gwtsovfecn_SPEC>;

impl Gwtsovfecn {
    #[doc = "Timestamp Overflow Error Number"]
    #[inline(always)]
    pub fn tsovfen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwtsovfecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwtsovfecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtsovfecn {
    #[inline(always)]
    fn default() -> Gwtsovfecn {
        <crate::RegValueT<Gwtsovfecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwusmfsecn_SPEC;
impl crate::sealed::RegSpec for Gwusmfsecn_SPEC {
    type DataType = u32;
}

#[doc = "Under Minimum Frame Size Error Counter Register"]
pub type Gwusmfsecn = crate::RegValueT<Gwusmfsecn_SPEC>;

impl Gwusmfsecn {
    #[doc = "Under Switch Minimum Frame Size Error Number"]
    #[inline(always)]
    pub fn usmfsen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwusmfsecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwusmfsecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwusmfsecn {
    #[inline(always)]
    fn default() -> Gwusmfsecn {
        <crate::RegValueT<Gwusmfsecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtfecn_SPEC;
impl crate::sealed::RegSpec for Gwtfecn_SPEC {
    type DataType = u32;
}

#[doc = "TAG Filtering Error Counter Register"]
pub type Gwtfecn = crate::RegValueT<Gwtfecn_SPEC>;

impl Gwtfecn {
    #[doc = "TAG Filtering Error Number"]
    #[inline(always)]
    pub fn tfen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwtfecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwtfecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtfecn {
    #[inline(always)]
    fn default() -> Gwtfecn {
        <crate::RegValueT<Gwtfecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwseqecn_SPEC;
impl crate::sealed::RegSpec for Gwseqecn_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Error Counter Register"]
pub type Gwseqecn = crate::RegValueT<Gwseqecn_SPEC>;

impl Gwseqecn {
    #[doc = "Sequence Error Number"]
    #[inline(always)]
    pub fn seqen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwseqecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwseqecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwseqecn {
    #[inline(always)]
    fn default() -> Gwseqecn {
        <crate::RegValueT<Gwseqecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtxdnecn_SPEC;
impl crate::sealed::RegSpec for Gwtxdnecn_SPEC {
    type DataType = u32;
}

#[doc = "TX Descriptor Number Error Counter Register"]
pub type Gwtxdnecn = crate::RegValueT<Gwtxdnecn_SPEC>;

impl Gwtxdnecn {
    #[doc = "TX Descriptor Number Error Number"]
    #[inline(always)]
    pub fn txdnen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwtxdnecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwtxdnecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtxdnecn {
    #[inline(always)]
    fn default() -> Gwtxdnecn {
        <crate::RegValueT<Gwtxdnecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwfsecn_SPEC;
impl crate::sealed::RegSpec for Gwfsecn_SPEC {
    type DataType = u32;
}

#[doc = "Frame Size Error Counter Register"]
pub type Gwfsecn = crate::RegValueT<Gwfsecn_SPEC>;

impl Gwfsecn {
    #[doc = "Frame Size Error Number"]
    #[inline(always)]
    pub fn fsen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwfsecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwfsecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwfsecn {
    #[inline(always)]
    fn default() -> Gwfsecn {
        <crate::RegValueT<Gwfsecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtdfecn_SPEC;
impl crate::sealed::RegSpec for Gwtdfecn_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Descriptor Full Error Counter Register"]
pub type Gwtdfecn = crate::RegValueT<Gwtdfecn_SPEC>;

impl Gwtdfecn {
    #[doc = "Timestamp Descriptor Full Error Number"]
    #[inline(always)]
    pub fn tdfen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwtdfecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwtdfecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtdfecn {
    #[inline(always)]
    fn default() -> Gwtdfecn {
        <crate::RegValueT<Gwtdfecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsdnecn_SPEC;
impl crate::sealed::RegSpec for Gwtsdnecn_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Descriptor Number Error Counter Register"]
pub type Gwtsdnecn = crate::RegValueT<Gwtsdnecn_SPEC>;

impl Gwtsdnecn {
    #[doc = "Timestamp Descriptor Number Error Number"]
    #[inline(always)]
    pub fn tsdnen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwtsdnecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwtsdnecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtsdnecn {
    #[inline(always)]
    fn default() -> Gwtsdnecn {
        <crate::RegValueT<Gwtsdnecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdqoecn_SPEC;
impl crate::sealed::RegSpec for Gwdqoecn_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Queue Overflow Error Counter Register"]
pub type Gwdqoecn = crate::RegValueT<Gwdqoecn_SPEC>;

impl Gwdqoecn {
    #[doc = "Descriptor Queue Overflow Error Number"]
    #[inline(always)]
    pub fn dqoen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwdqoecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwdqoecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdqoecn {
    #[inline(always)]
    fn default() -> Gwdqoecn {
        <crate::RegValueT<Gwdqoecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdqsecn_SPEC;
impl crate::sealed::RegSpec for Gwdqsecn_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Queue Security Error Counter Register"]
pub type Gwdqsecn = crate::RegValueT<Gwdqsecn_SPEC>;

impl Gwdqsecn {
    #[doc = "Descriptor Queue Security Error Number"]
    #[inline(always)]
    pub fn dqsen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwdqsecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwdqsecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdqsecn {
    #[inline(always)]
    fn default() -> Gwdqsecn {
        <crate::RegValueT<Gwdqsecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdfecn_SPEC;
impl crate::sealed::RegSpec for Gwdfecn_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Full Error Counter Register"]
pub type Gwdfecn = crate::RegValueT<Gwdfecn_SPEC>;

impl Gwdfecn {
    #[doc = "Descriptor Full Error Number"]
    #[inline(always)]
    pub fn dfen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwdfecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwdfecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdfecn {
    #[inline(always)]
    fn default() -> Gwdfecn {
        <crate::RegValueT<Gwdfecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdsecn_SPEC;
impl crate::sealed::RegSpec for Gwdsecn_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Security Error Counter Register"]
pub type Gwdsecn = crate::RegValueT<Gwdsecn_SPEC>;

impl Gwdsecn {
    #[doc = "Descriptor Security Error Number"]
    #[inline(always)]
    pub fn dsen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwdsecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwdsecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdsecn {
    #[inline(always)]
    fn default() -> Gwdsecn {
        <crate::RegValueT<Gwdsecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdszecn_SPEC;
impl crate::sealed::RegSpec for Gwdszecn_SPEC {
    type DataType = u32;
}

#[doc = "Data Size Error Counter Register"]
pub type Gwdszecn = crate::RegValueT<Gwdszecn_SPEC>;

impl Gwdszecn {
    #[doc = "Data Size Error Number"]
    #[inline(always)]
    pub fn dszen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwdszecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwdszecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdszecn {
    #[inline(always)]
    fn default() -> Gwdszecn {
        <crate::RegValueT<Gwdszecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdctecn_SPEC;
impl crate::sealed::RegSpec for Gwdctecn_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Chain Type Error Counter Register"]
pub type Gwdctecn = crate::RegValueT<Gwdctecn_SPEC>;

impl Gwdctecn {
    #[doc = "Descriptor Chain Type Error Number"]
    #[inline(always)]
    pub fn dcten(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwdctecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwdctecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwdctecn {
    #[inline(always)]
    fn default() -> Gwdctecn {
        <crate::RegValueT<Gwdctecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwrxdnecn_SPEC;
impl crate::sealed::RegSpec for Gwrxdnecn_SPEC {
    type DataType = u32;
}

#[doc = "RX Descriptor Number Error Counter Register"]
pub type Gwrxdnecn = crate::RegValueT<Gwrxdnecn_SPEC>;

impl Gwrxdnecn {
    #[doc = "RX Descriptor Number Error Number"]
    #[inline(always)]
    pub fn rxdnen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gwrxdnecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gwrxdnecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwrxdnecn {
    #[inline(always)]
    fn default() -> Gwrxdnecn {
        <crate::RegValueT<Gwrxdnecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdis0_SPEC;
impl crate::sealed::RegSpec for Gwdis0_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Status Register"]
pub type Gwdis0 = crate::RegValueT<Gwdis0_SPEC>;

impl NoBitfieldReg<Gwdis0_SPEC> for Gwdis0 {}
impl ::core::default::Default for Gwdis0 {
    #[inline(always)]
    fn default() -> Gwdis0 {
        <crate::RegValueT<Gwdis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdie0_SPEC;
impl crate::sealed::RegSpec for Gwdie0_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Enable Register"]
pub type Gwdie0 = crate::RegValueT<Gwdie0_SPEC>;

impl NoBitfieldReg<Gwdie0_SPEC> for Gwdie0 {}
impl ::core::default::Default for Gwdie0 {
    #[inline(always)]
    fn default() -> Gwdie0 {
        <crate::RegValueT<Gwdie0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdid0_SPEC;
impl crate::sealed::RegSpec for Gwdid0_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Disable Register"]
pub type Gwdid0 = crate::RegValueT<Gwdid0_SPEC>;

impl NoBitfieldReg<Gwdid0_SPEC> for Gwdid0 {}
impl ::core::default::Default for Gwdid0 {
    #[inline(always)]
    fn default() -> Gwdid0 {
        <crate::RegValueT<Gwdid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdids0_SPEC;
impl crate::sealed::RegSpec for Gwdids0_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Delayed Status Register"]
pub type Gwdids0 = crate::RegValueT<Gwdids0_SPEC>;

impl NoBitfieldReg<Gwdids0_SPEC> for Gwdids0 {}
impl ::core::default::Default for Gwdids0 {
    #[inline(always)]
    fn default() -> Gwdids0 {
        <crate::RegValueT<Gwdids0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdis1_SPEC;
impl crate::sealed::RegSpec for Gwdis1_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Status Register"]
pub type Gwdis1 = crate::RegValueT<Gwdis1_SPEC>;

impl NoBitfieldReg<Gwdis1_SPEC> for Gwdis1 {}
impl ::core::default::Default for Gwdis1 {
    #[inline(always)]
    fn default() -> Gwdis1 {
        <crate::RegValueT<Gwdis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdie1_SPEC;
impl crate::sealed::RegSpec for Gwdie1_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Enable Register"]
pub type Gwdie1 = crate::RegValueT<Gwdie1_SPEC>;

impl NoBitfieldReg<Gwdie1_SPEC> for Gwdie1 {}
impl ::core::default::Default for Gwdie1 {
    #[inline(always)]
    fn default() -> Gwdie1 {
        <crate::RegValueT<Gwdie1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdid1_SPEC;
impl crate::sealed::RegSpec for Gwdid1_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Disable Register"]
pub type Gwdid1 = crate::RegValueT<Gwdid1_SPEC>;

impl NoBitfieldReg<Gwdid1_SPEC> for Gwdid1 {}
impl ::core::default::Default for Gwdid1 {
    #[inline(always)]
    fn default() -> Gwdid1 {
        <crate::RegValueT<Gwdid1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwdids1_SPEC;
impl crate::sealed::RegSpec for Gwdids1_SPEC {
    type DataType = u32;
}

#[doc = "Data Interrupt Delayed Status Register"]
pub type Gwdids1 = crate::RegValueT<Gwdids1_SPEC>;

impl NoBitfieldReg<Gwdids1_SPEC> for Gwdids1 {}
impl ::core::default::Default for Gwdids1 {
    #[inline(always)]
    fn default() -> Gwdids1 {
        <crate::RegValueT<Gwdids1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsdis_SPEC;
impl crate::sealed::RegSpec for Gwtsdis_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Data Interrupt Status Register"]
pub type Gwtsdis = crate::RegValueT<Gwtsdis_SPEC>;

impl Gwtsdis {
    #[doc = "Timestamp Data i Interrupt Status Flag"]
    #[inline(always)]
    pub fn tsdis1_to_tsdis0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Gwtsdis_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Gwtsdis_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtsdis {
    #[inline(always)]
    fn default() -> Gwtsdis {
        <crate::RegValueT<Gwtsdis_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsdie_SPEC;
impl crate::sealed::RegSpec for Gwtsdie_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Data Interrupt Enable Register"]
pub type Gwtsdie = crate::RegValueT<Gwtsdie_SPEC>;

impl Gwtsdie {
    #[doc = "Timestamp Data i Interrupt Enable"]
    #[inline(always)]
    pub fn tsdie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gwtsdie::Tsdie0,
        gwtsdie::Tsdie0,
        Gwtsdie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gwtsdie::Tsdie0,
            gwtsdie::Tsdie0,
            Gwtsdie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Data i Interrupt Enable"]
    #[inline(always)]
    pub fn tsdie1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gwtsdie::Tsdie1,
        gwtsdie::Tsdie1,
        Gwtsdie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gwtsdie::Tsdie1,
            gwtsdie::Tsdie1,
            Gwtsdie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gwtsdie {
    #[inline(always)]
    fn default() -> Gwtsdie {
        <crate::RegValueT<Gwtsdie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gwtsdie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsdie0_SPEC;
    pub type Tsdie0 = crate::EnumBitfieldStruct<u8, Tsdie0_SPEC>;
    impl Tsdie0 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsdie1_SPEC;
    pub type Tsdie1 = crate::EnumBitfieldStruct<u8, Tsdie1_SPEC>;
    impl Tsdie1 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwtsdid_SPEC;
impl crate::sealed::RegSpec for Gwtsdid_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Data Interrupt Disable Register"]
pub type Gwtsdid = crate::RegValueT<Gwtsdid_SPEC>;

impl Gwtsdid {
    #[doc = "Timestamp Data i Interrupt Disable"]
    #[inline(always)]
    pub fn tsdid1_to_tsdid0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Gwtsdid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Gwtsdid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwtsdid {
    #[inline(always)]
    fn default() -> Gwtsdid {
        <crate::RegValueT<Gwtsdid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweis0_SPEC;
impl crate::sealed::RegSpec for Gweis0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 0"]
pub type Gweis0 = crate::RegValueT<Gweis0_SPEC>;

impl Gweis0 {
    #[doc = "AXI Error Status Flag"]
    #[inline(always)]
    pub fn aes(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Timestamp Overflow Error Status Flag"]
    #[inline(always)]
    pub fn tsovfes(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Under Switch Minimum Frame Size Error Status Flag"]
    #[inline(always)]
    pub fn usmfses(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TAG Filtering Error Status Flag"]
    #[inline(always)]
    pub fn tfes(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Sequence Error Status Flag"]
    #[inline(always)]
    pub fn seqes(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Descriptor Number Error Status Flag"]
    #[inline(always)]
    pub fn txdnes(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Timestamp Hardware Error Status Flag"]
    #[inline(always)]
    pub fn tshes(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Size Error Status Flag i"]
    #[inline(always)]
    pub fn fses7_to_fses0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gweis0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timestamp Descriptor i Full Error Status Flag"]
    #[inline(always)]
    pub fn tdfes1_to_tdfes0(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,Gweis0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timestamp Descriptor i Number Error Status Flag"]
    #[inline(always)]
    pub fn tsdnes1_to_tsdnes0(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Gweis0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Gweis0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweis0 {
    #[inline(always)]
    fn default() -> Gweis0 {
        <crate::RegValueT<Gweis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweie0_SPEC;
impl crate::sealed::RegSpec for Gweie0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 0"]
pub type Gweie0 = crate::RegValueT<Gweie0_SPEC>;

impl Gweie0 {
    #[doc = "AXI Error Enable"]
    #[inline(always)]
    pub fn aee(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gweie0::Aee,
        gweie0::Aee,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gweie0::Aee,
            gweie0::Aee,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Overflow Error Enable"]
    #[inline(always)]
    pub fn tsovfee(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gweie0::Tsovfee,
        gweie0::Tsovfee,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gweie0::Tsovfee,
            gweie0::Tsovfee,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Under Minimum Frame Size Error Enable"]
    #[inline(always)]
    pub fn usmfsee(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gweie0::Usmfsee,
        gweie0::Usmfsee,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gweie0::Usmfsee,
            gweie0::Usmfsee,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAG Filtering Error Enable"]
    #[inline(always)]
    pub fn tfee(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gweie0::Tfee,
        gweie0::Tfee,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gweie0::Tfee,
            gweie0::Tfee,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Error Enable"]
    #[inline(always)]
    pub fn seqee(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gweie0::Seqee,
        gweie0::Seqee,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gweie0::Seqee,
            gweie0::Seqee,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Number Error Enable"]
    #[inline(always)]
    pub fn txdnee(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gweie0::Txdnee,
        gweie0::Txdnee,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gweie0::Txdnee,
            gweie0::Txdnee,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Hardware Error Enable"]
    #[inline(always)]
    pub fn tshee(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gweie0::Tshee,
        gweie0::Tshee,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gweie0::Tshee,
            gweie0::Tshee,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gweie0::Fsee0,
        gweie0::Fsee0,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gweie0::Fsee0,
            gweie0::Fsee0,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gweie0::Fsee1,
        gweie0::Fsee1,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gweie0::Fsee1,
            gweie0::Fsee1,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gweie0::Fsee2,
        gweie0::Fsee2,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gweie0::Fsee2,
            gweie0::Fsee2,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gweie0::Fsee3,
        gweie0::Fsee3,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gweie0::Fsee3,
            gweie0::Fsee3,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gweie0::Fsee4,
        gweie0::Fsee4,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gweie0::Fsee4,
            gweie0::Fsee4,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gweie0::Fsee5,
        gweie0::Fsee5,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gweie0::Fsee5,
            gweie0::Fsee5,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gweie0::Fsee6,
        gweie0::Fsee6,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gweie0::Fsee6,
            gweie0::Fsee6,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable i (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn fsee7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gweie0::Fsee7,
        gweie0::Fsee7,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gweie0::Fsee7,
            gweie0::Fsee7,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Descriptor i Full Error Enable"]
    #[inline(always)]
    pub fn tdfee0(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gweie0::Tdfee0,
        gweie0::Tdfee0,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gweie0::Tdfee0,
            gweie0::Tdfee0,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Descriptor i Full Error Enable"]
    #[inline(always)]
    pub fn tdfee1(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        gweie0::Tdfee1,
        gweie0::Tdfee1,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            gweie0::Tdfee1,
            gweie0::Tdfee1,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Descriptor i Number Error Enable"]
    #[inline(always)]
    pub fn tsdnee0(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        gweie0::Tsdnee0,
        gweie0::Tsdnee0,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            gweie0::Tsdnee0,
            gweie0::Tsdnee0,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Descriptor i Number Error Enable"]
    #[inline(always)]
    pub fn tsdnee1(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gweie0::Tsdnee1,
        gweie0::Tsdnee1,
        Gweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gweie0::Tsdnee1,
            gweie0::Tsdnee1,
            Gweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gweie0 {
    #[inline(always)]
    fn default() -> Gweie0 {
        <crate::RegValueT<Gweie0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gweie0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aee_SPEC;
    pub type Aee = crate::EnumBitfieldStruct<u8, Aee_SPEC>;
    impl Aee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsovfee_SPEC;
    pub type Tsovfee = crate::EnumBitfieldStruct<u8, Tsovfee_SPEC>;
    impl Tsovfee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usmfsee_SPEC;
    pub type Usmfsee = crate::EnumBitfieldStruct<u8, Usmfsee_SPEC>;
    impl Usmfsee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfee_SPEC;
    pub type Tfee = crate::EnumBitfieldStruct<u8, Tfee_SPEC>;
    impl Tfee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seqee_SPEC;
    pub type Seqee = crate::EnumBitfieldStruct<u8, Seqee_SPEC>;
    impl Seqee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txdnee_SPEC;
    pub type Txdnee = crate::EnumBitfieldStruct<u8, Txdnee_SPEC>;
    impl Txdnee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tshee_SPEC;
    pub type Tshee = crate::EnumBitfieldStruct<u8, Tshee_SPEC>;
    impl Tshee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee0_SPEC;
    pub type Fsee0 = crate::EnumBitfieldStruct<u8, Fsee0_SPEC>;
    impl Fsee0 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee1_SPEC;
    pub type Fsee1 = crate::EnumBitfieldStruct<u8, Fsee1_SPEC>;
    impl Fsee1 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee2_SPEC;
    pub type Fsee2 = crate::EnumBitfieldStruct<u8, Fsee2_SPEC>;
    impl Fsee2 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee3_SPEC;
    pub type Fsee3 = crate::EnumBitfieldStruct<u8, Fsee3_SPEC>;
    impl Fsee3 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee4_SPEC;
    pub type Fsee4 = crate::EnumBitfieldStruct<u8, Fsee4_SPEC>;
    impl Fsee4 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee5_SPEC;
    pub type Fsee5 = crate::EnumBitfieldStruct<u8, Fsee5_SPEC>;
    impl Fsee5 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee6_SPEC;
    pub type Fsee6 = crate::EnumBitfieldStruct<u8, Fsee6_SPEC>;
    impl Fsee6 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee7_SPEC;
    pub type Fsee7 = crate::EnumBitfieldStruct<u8, Fsee7_SPEC>;
    impl Fsee7 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdfee0_SPEC;
    pub type Tdfee0 = crate::EnumBitfieldStruct<u8, Tdfee0_SPEC>;
    impl Tdfee0 {
        #[doc = "Interrupt disabled for descriptor queue i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for descriptor queue i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdfee1_SPEC;
    pub type Tdfee1 = crate::EnumBitfieldStruct<u8, Tdfee1_SPEC>;
    impl Tdfee1 {
        #[doc = "Interrupt disabled for descriptor queue i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for descriptor queue i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsdnee0_SPEC;
    pub type Tsdnee0 = crate::EnumBitfieldStruct<u8, Tsdnee0_SPEC>;
    impl Tsdnee0 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsdnee1_SPEC;
    pub type Tsdnee1 = crate::EnumBitfieldStruct<u8, Tsdnee1_SPEC>;
    impl Tsdnee1 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweid0_SPEC;
impl crate::sealed::RegSpec for Gweid0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 0"]
pub type Gweid0 = crate::RegValueT<Gweid0_SPEC>;

impl Gweid0 {
    #[doc = "AXI Error Disable"]
    #[inline(always)]
    pub fn aed(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gweid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Timestamp Overflow Error Disable"]
    #[inline(always)]
    pub fn tsovfed(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gweid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Under Minimum Frame Size Error Disable"]
    #[inline(always)]
    pub fn usmfsed(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gweid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TAG Filtering Error Disable"]
    #[inline(always)]
    pub fn tfed(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gweid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Sequence Error Disable"]
    #[inline(always)]
    pub fn seqed(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gweid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Descriptor Number Error Disable"]
    #[inline(always)]
    pub fn txdned(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gweid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Timestamp Hardware Full Error Disable"]
    #[inline(always)]
    pub fn tshed(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gweid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Size Error Disable i"]
    #[inline(always)]
    pub fn fsed7_to_fsed0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gweid0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Timestamp Descriptor i Full Error Disable"]
    #[inline(always)]
    pub fn tdfed1_to_tdfed0(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,Gweid0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Timestamp Descriptor i Number Error Disable"]
    #[inline(always)]
    pub fn tsdned1_to_tsdned0(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Gweid0_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Gweid0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweid0 {
    #[inline(always)]
    fn default() -> Gweid0 {
        <crate::RegValueT<Gweid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweis1_SPEC;
impl crate::sealed::RegSpec for Gweis1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 1"]
pub type Gweis1 = crate::RegValueT<Gweis1_SPEC>;

impl Gweis1 {
    #[doc = "Descriptor Queue i Overflow Error Status Flag"]
    #[inline(always)]
    pub fn dqoes7_to_dqoes0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gweis1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gweis1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Descriptor Queue i Security Error Status Flag (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn dqses7_to_dqses0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gweis1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gweis1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweis1 {
    #[inline(always)]
    fn default() -> Gweis1 {
        <crate::RegValueT<Gweis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweie1_SPEC;
impl crate::sealed::RegSpec for Gweie1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 1"]
pub type Gweie1 = crate::RegValueT<Gweie1_SPEC>;

impl Gweie1 {
    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gweie1::Dqoee0,
        gweie1::Dqoee0,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gweie1::Dqoee0,
            gweie1::Dqoee0,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gweie1::Dqoee1,
        gweie1::Dqoee1,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gweie1::Dqoee1,
            gweie1::Dqoee1,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gweie1::Dqoee2,
        gweie1::Dqoee2,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gweie1::Dqoee2,
            gweie1::Dqoee2,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gweie1::Dqoee3,
        gweie1::Dqoee3,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gweie1::Dqoee3,
            gweie1::Dqoee3,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gweie1::Dqoee4,
        gweie1::Dqoee4,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gweie1::Dqoee4,
            gweie1::Dqoee4,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gweie1::Dqoee5,
        gweie1::Dqoee5,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gweie1::Dqoee5,
            gweie1::Dqoee5,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gweie1::Dqoee6,
        gweie1::Dqoee6,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gweie1::Dqoee6,
            gweie1::Dqoee6,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Overflow Error Enable"]
    #[inline(always)]
    pub fn dqoee7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gweie1::Dqoee7,
        gweie1::Dqoee7,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gweie1::Dqoee7,
            gweie1::Dqoee7,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gweie1::Dqsee0,
        gweie1::Dqsee0,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gweie1::Dqsee0,
            gweie1::Dqsee0,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gweie1::Dqsee1,
        gweie1::Dqsee1,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gweie1::Dqsee1,
            gweie1::Dqsee1,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gweie1::Dqsee2,
        gweie1::Dqsee2,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gweie1::Dqsee2,
            gweie1::Dqsee2,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gweie1::Dqsee3,
        gweie1::Dqsee3,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gweie1::Dqsee3,
            gweie1::Dqsee3,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gweie1::Dqsee4,
        gweie1::Dqsee4,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gweie1::Dqsee4,
            gweie1::Dqsee4,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gweie1::Dqsee5,
        gweie1::Dqsee5,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gweie1::Dqsee5,
            gweie1::Dqsee5,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gweie1::Dqsee6,
        gweie1::Dqsee6,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gweie1::Dqsee6,
            gweie1::Dqsee6,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue i Security Error Enable"]
    #[inline(always)]
    pub fn dqsee7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gweie1::Dqsee7,
        gweie1::Dqsee7,
        Gweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gweie1::Dqsee7,
            gweie1::Dqsee7,
            Gweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gweie1 {
    #[inline(always)]
    fn default() -> Gweie1 {
        <crate::RegValueT<Gweie1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gweie1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee0_SPEC;
    pub type Dqoee0 = crate::EnumBitfieldStruct<u8, Dqoee0_SPEC>;
    impl Dqoee0 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee1_SPEC;
    pub type Dqoee1 = crate::EnumBitfieldStruct<u8, Dqoee1_SPEC>;
    impl Dqoee1 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee2_SPEC;
    pub type Dqoee2 = crate::EnumBitfieldStruct<u8, Dqoee2_SPEC>;
    impl Dqoee2 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee3_SPEC;
    pub type Dqoee3 = crate::EnumBitfieldStruct<u8, Dqoee3_SPEC>;
    impl Dqoee3 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee4_SPEC;
    pub type Dqoee4 = crate::EnumBitfieldStruct<u8, Dqoee4_SPEC>;
    impl Dqoee4 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee5_SPEC;
    pub type Dqoee5 = crate::EnumBitfieldStruct<u8, Dqoee5_SPEC>;
    impl Dqoee5 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee6_SPEC;
    pub type Dqoee6 = crate::EnumBitfieldStruct<u8, Dqoee6_SPEC>;
    impl Dqoee6 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee7_SPEC;
    pub type Dqoee7 = crate::EnumBitfieldStruct<u8, Dqoee7_SPEC>;
    impl Dqoee7 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee0_SPEC;
    pub type Dqsee0 = crate::EnumBitfieldStruct<u8, Dqsee0_SPEC>;
    impl Dqsee0 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee1_SPEC;
    pub type Dqsee1 = crate::EnumBitfieldStruct<u8, Dqsee1_SPEC>;
    impl Dqsee1 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee2_SPEC;
    pub type Dqsee2 = crate::EnumBitfieldStruct<u8, Dqsee2_SPEC>;
    impl Dqsee2 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee3_SPEC;
    pub type Dqsee3 = crate::EnumBitfieldStruct<u8, Dqsee3_SPEC>;
    impl Dqsee3 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee4_SPEC;
    pub type Dqsee4 = crate::EnumBitfieldStruct<u8, Dqsee4_SPEC>;
    impl Dqsee4 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee5_SPEC;
    pub type Dqsee5 = crate::EnumBitfieldStruct<u8, Dqsee5_SPEC>;
    impl Dqsee5 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee6_SPEC;
    pub type Dqsee6 = crate::EnumBitfieldStruct<u8, Dqsee6_SPEC>;
    impl Dqsee6 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee7_SPEC;
    pub type Dqsee7 = crate::EnumBitfieldStruct<u8, Dqsee7_SPEC>;
    impl Dqsee7 {
        #[doc = "Interrupt disabled for error q"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for error q"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweid1_SPEC;
impl crate::sealed::RegSpec for Gweid1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 1"]
pub type Gweid1 = crate::RegValueT<Gweid1_SPEC>;

impl Gweid1 {
    #[doc = "Descriptor Queue i Overflow Error Disable"]
    #[inline(always)]
    pub fn dqoed7_to_dqoed0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gweid1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gweid1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Descriptor Queue i Security Error Disable (The variable i corresponds to the bit position number.)"]
    #[inline(always)]
    pub fn dqsed7_to_dqsed0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gweid1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gweid1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweid1 {
    #[inline(always)]
    fn default() -> Gweid1 {
        <crate::RegValueT<Gweid1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweis20_SPEC;
impl crate::sealed::RegSpec for Gweis20_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 2"]
pub type Gweis20 = crate::RegValueT<Gweis20_SPEC>;

impl NoBitfieldReg<Gweis20_SPEC> for Gweis20 {}
impl ::core::default::Default for Gweis20 {
    #[inline(always)]
    fn default() -> Gweis20 {
        <crate::RegValueT<Gweis20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweie20_SPEC;
impl crate::sealed::RegSpec for Gweie20_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 2"]
pub type Gweie20 = crate::RegValueT<Gweie20_SPEC>;

impl NoBitfieldReg<Gweie20_SPEC> for Gweie20 {}
impl ::core::default::Default for Gweie20 {
    #[inline(always)]
    fn default() -> Gweie20 {
        <crate::RegValueT<Gweie20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweid20_SPEC;
impl crate::sealed::RegSpec for Gweid20_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 2"]
pub type Gweid20 = crate::RegValueT<Gweid20_SPEC>;

impl NoBitfieldReg<Gweid20_SPEC> for Gweid20 {}
impl ::core::default::Default for Gweid20 {
    #[inline(always)]
    fn default() -> Gweid20 {
        <crate::RegValueT<Gweid20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweis21_SPEC;
impl crate::sealed::RegSpec for Gweis21_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 2"]
pub type Gweis21 = crate::RegValueT<Gweis21_SPEC>;

impl NoBitfieldReg<Gweis21_SPEC> for Gweis21 {}
impl ::core::default::Default for Gweis21 {
    #[inline(always)]
    fn default() -> Gweis21 {
        <crate::RegValueT<Gweis21_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweie21_SPEC;
impl crate::sealed::RegSpec for Gweie21_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 2"]
pub type Gweie21 = crate::RegValueT<Gweie21_SPEC>;

impl NoBitfieldReg<Gweie21_SPEC> for Gweie21 {}
impl ::core::default::Default for Gweie21 {
    #[inline(always)]
    fn default() -> Gweie21 {
        <crate::RegValueT<Gweie21_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweid21_SPEC;
impl crate::sealed::RegSpec for Gweid21_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 2"]
pub type Gweid21 = crate::RegValueT<Gweid21_SPEC>;

impl NoBitfieldReg<Gweid21_SPEC> for Gweid21 {}
impl ::core::default::Default for Gweid21 {
    #[inline(always)]
    fn default() -> Gweid21 {
        <crate::RegValueT<Gweid21_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweis3_SPEC;
impl crate::sealed::RegSpec for Gweis3_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 3"]
pub type Gweis3 = crate::RegValueT<Gweis3_SPEC>;

impl Gweis3 {
    #[doc = "Incremental Area i Overflow Error Status Flag"]
    #[inline(always)]
    pub fn iaoes3_to_iaoes0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Gweis3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Gweis3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweis3 {
    #[inline(always)]
    fn default() -> Gweis3 {
        <crate::RegValueT<Gweis3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweie3_SPEC;
impl crate::sealed::RegSpec for Gweie3_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 3"]
pub type Gweie3 = crate::RegValueT<Gweie3_SPEC>;

impl Gweie3 {
    #[doc = "Incremental Area i Overflow Error Enable"]
    #[inline(always)]
    pub fn iaoee0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gweie3::Iaoee0,
        gweie3::Iaoee0,
        Gweie3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gweie3::Iaoee0,
            gweie3::Iaoee0,
            Gweie3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Incremental Area i Overflow Error Enable"]
    #[inline(always)]
    pub fn iaoee1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gweie3::Iaoee1,
        gweie3::Iaoee1,
        Gweie3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gweie3::Iaoee1,
            gweie3::Iaoee1,
            Gweie3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Incremental Area i Overflow Error Enable"]
    #[inline(always)]
    pub fn iaoee2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gweie3::Iaoee2,
        gweie3::Iaoee2,
        Gweie3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gweie3::Iaoee2,
            gweie3::Iaoee2,
            Gweie3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Incremental Area i Overflow Error Enable"]
    #[inline(always)]
    pub fn iaoee3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gweie3::Iaoee3,
        gweie3::Iaoee3,
        Gweie3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gweie3::Iaoee3,
            gweie3::Iaoee3,
            Gweie3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gweie3 {
    #[inline(always)]
    fn default() -> Gweie3 {
        <crate::RegValueT<Gweie3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gweie3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iaoee0_SPEC;
    pub type Iaoee0 = crate::EnumBitfieldStruct<u8, Iaoee0_SPEC>;
    impl Iaoee0 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iaoee1_SPEC;
    pub type Iaoee1 = crate::EnumBitfieldStruct<u8, Iaoee1_SPEC>;
    impl Iaoee1 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iaoee2_SPEC;
    pub type Iaoee2 = crate::EnumBitfieldStruct<u8, Iaoee2_SPEC>;
    impl Iaoee2 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iaoee3_SPEC;
    pub type Iaoee3 = crate::EnumBitfieldStruct<u8, Iaoee3_SPEC>;
    impl Iaoee3 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweid3_SPEC;
impl crate::sealed::RegSpec for Gweid3_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 3"]
pub type Gweid3 = crate::RegValueT<Gweid3_SPEC>;

impl Gweid3 {
    #[doc = "Incremental Area i Overflow Error Disable"]
    #[inline(always)]
    pub fn iaoed3_to_iaoed0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Gweid3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Gweid3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweid3 {
    #[inline(always)]
    fn default() -> Gweid3 {
        <crate::RegValueT<Gweid3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweis4_SPEC;
impl crate::sealed::RegSpec for Gweis4_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 4"]
pub type Gweis4 = crate::RegValueT<Gweis4_SPEC>;

impl Gweis4 {
    #[doc = "Descriptor Security Error Status Flag"]
    #[inline(always)]
    pub fn dsses(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Gweis4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gweis4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Security Error Interrupt Overflow Status Flag"]
    #[inline(always)]
    pub fn dsseios(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Gweis4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gweis4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Security Error Chain Number"]
    #[inline(always)]
    pub fn dssecn(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Gweis4_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Gweis4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Data Size Error Status Flag"]
    #[inline(always)]
    pub fn dses(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Gweis4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gweis4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Data Size Error Interrupt Overflow Status Flag"]
    #[inline(always)]
    pub fn dseios(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gweis4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gweis4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Data Size Error Chain Number"]
    #[inline(always)]
    pub fn dsecn(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Gweis4_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Gweis4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweis4 {
    #[inline(always)]
    fn default() -> Gweis4 {
        <crate::RegValueT<Gweis4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweie4_SPEC;
impl crate::sealed::RegSpec for Gweie4_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 4"]
pub type Gweie4 = crate::RegValueT<Gweie4_SPEC>;

impl Gweie4 {
    #[doc = "Descriptor Security Error Enable"]
    #[inline(always)]
    pub fn dssee(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gweie4::Dssee,
        gweie4::Dssee,
        Gweie4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gweie4::Dssee,
            gweie4::Dssee,
            Gweie4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Security Error Interrupt Overflow Enable"]
    #[inline(always)]
    pub fn dsseioe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gweie4::Dsseioe,
        gweie4::Dsseioe,
        Gweie4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gweie4::Dsseioe,
            gweie4::Dsseioe,
            Gweie4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Size Error Enable"]
    #[inline(always)]
    pub fn dsee(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gweie4::Dsee,
        gweie4::Dsee,
        Gweie4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gweie4::Dsee,
            gweie4::Dsee,
            Gweie4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Size Error Interrupt Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn dseioe(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gweie4::Dseioe,
        gweie4::Dseioe,
        Gweie4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gweie4::Dseioe,
            gweie4::Dseioe,
            Gweie4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gweie4 {
    #[inline(always)]
    fn default() -> Gweie4 {
        <crate::RegValueT<Gweie4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gweie4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dssee_SPEC;
    pub type Dssee = crate::EnumBitfieldStruct<u8, Dssee_SPEC>;
    impl Dssee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsseioe_SPEC;
    pub type Dsseioe = crate::EnumBitfieldStruct<u8, Dsseioe_SPEC>;
    impl Dsseioe {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsee_SPEC;
    pub type Dsee = crate::EnumBitfieldStruct<u8, Dsee_SPEC>;
    impl Dsee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dseioe_SPEC;
    pub type Dseioe = crate::EnumBitfieldStruct<u8, Dseioe_SPEC>;
    impl Dseioe {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweid4_SPEC;
impl crate::sealed::RegSpec for Gweid4_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 4"]
pub type Gweid4 = crate::RegValueT<Gweid4_SPEC>;

impl Gweid4 {
    #[doc = "Descriptor Security Error Disable"]
    #[inline(always)]
    pub fn dssed(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gweid4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gweid4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Security Error Interrupt Overflow Disable"]
    #[inline(always)]
    pub fn dsseiod(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Gweid4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gweid4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Data Size Error Disable"]
    #[inline(always)]
    pub fn dsed(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gweid4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gweid4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Data Size Error Interrupt Overflow Disable"]
    #[inline(always)]
    pub fn dseiod(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gweid4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gweid4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gweid4 {
    #[inline(always)]
    fn default() -> Gweid4 {
        <crate::RegValueT<Gweid4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweis5_SPEC;
impl crate::sealed::RegSpec for Gweis5_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 5"]
pub type Gweis5 = crate::RegValueT<Gweis5_SPEC>;

impl Gweis5 {
    #[doc = "Descriptor Chain Type Error Status Flag"]
    #[inline(always)]
    pub fn dctes(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Gweis5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gweis5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Chain Type Error Interrupt Overflow Status Flag"]
    #[inline(always)]
    pub fn dcteios(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Gweis5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gweis5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Chain Type Error Chain Number"]
    #[inline(always)]
    pub fn dctecn(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Gweis5_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Gweis5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "RX Descriptor Number Error Status Flag"]
    #[inline(always)]
    pub fn rxdnes(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Gweis5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gweis5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Descriptor Number Error Interrupt Overflow Status Flag"]
    #[inline(always)]
    pub fn rxdneios(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gweis5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gweis5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Descriptor Number Error Chain Number"]
    #[inline(always)]
    pub fn rxdnecn(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Gweis5_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Gweis5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gweis5 {
    #[inline(always)]
    fn default() -> Gweis5 {
        <crate::RegValueT<Gweis5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweie5_SPEC;
impl crate::sealed::RegSpec for Gweie5_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 5"]
pub type Gweie5 = crate::RegValueT<Gweie5_SPEC>;

impl Gweie5 {
    #[doc = "Descriptor Chain Type Error Enable"]
    #[inline(always)]
    pub fn dctee(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gweie5::Dctee,
        gweie5::Dctee,
        Gweie5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gweie5::Dctee,
            gweie5::Dctee,
            Gweie5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Chain Type Error Interrupt Overflow Enable"]
    #[inline(always)]
    pub fn dcteioe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gweie5::Dcteioe,
        gweie5::Dcteioe,
        Gweie5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gweie5::Dcteioe,
            gweie5::Dcteioe,
            Gweie5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Number Error Enable"]
    #[inline(always)]
    pub fn rxdnee(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gweie5::Rxdnee,
        gweie5::Rxdnee,
        Gweie5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gweie5::Rxdnee,
            gweie5::Rxdnee,
            Gweie5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RX Descriptor Number Error Interrupt Overflow Enable"]
    #[inline(always)]
    pub fn rxdneioe(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gweie5::Rxdneioe,
        gweie5::Rxdneioe,
        Gweie5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gweie5::Rxdneioe,
            gweie5::Rxdneioe,
            Gweie5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gweie5 {
    #[inline(always)]
    fn default() -> Gweie5 {
        <crate::RegValueT<Gweie5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gweie5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dctee_SPEC;
    pub type Dctee = crate::EnumBitfieldStruct<u8, Dctee_SPEC>;
    impl Dctee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcteioe_SPEC;
    pub type Dcteioe = crate::EnumBitfieldStruct<u8, Dcteioe_SPEC>;
    impl Dcteioe {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdnee_SPEC;
    pub type Rxdnee = crate::EnumBitfieldStruct<u8, Rxdnee_SPEC>;
    impl Rxdnee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdneioe_SPEC;
    pub type Rxdneioe = crate::EnumBitfieldStruct<u8, Rxdneioe_SPEC>;
    impl Rxdneioe {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gweid5_SPEC;
impl crate::sealed::RegSpec for Gweid5_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 5"]
pub type Gweid5 = crate::RegValueT<Gweid5_SPEC>;

impl Gweid5 {
    #[doc = "Descriptor Chain Type Error Disable"]
    #[inline(always)]
    pub fn dcted(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gweid5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gweid5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Chain Type Error Interrupt Overflow Disable"]
    #[inline(always)]
    pub fn dcteiod(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Gweid5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gweid5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Descriptor Number Error Disable"]
    #[inline(always)]
    pub fn rxdned(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Gweid5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gweid5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Descriptor Number Error Interrupt Overflow Disable"]
    #[inline(always)]
    pub fn rxdneiod(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gweid5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gweid5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gweid5 {
    #[inline(always)]
    fn default() -> Gweid5 {
        <crate::RegValueT<Gweid5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwidc_SPEC;
impl crate::sealed::RegSpec for Gwidc_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Delay %s Configuration Register"]
pub type Gwidc = crate::RegValueT<Gwidc_SPEC>;

impl Gwidc {
    #[doc = "Interrupt Delay Value"]
    #[inline(always)]
    pub fn idv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Gwidc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Gwidc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gwidc {
    #[inline(always)]
    fn default() -> Gwidc {
        <crate::RegValueT<Gwidc_SPEC> as RegisterValue<_>>::new(0)
    }
}
