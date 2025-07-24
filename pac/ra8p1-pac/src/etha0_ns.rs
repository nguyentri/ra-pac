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
#[doc = r"Ethernet Agent"]
unsafe impl ::core::marker::Send for super::Etha0Ns {}
unsafe impl ::core::marker::Sync for super::Etha0Ns {}
impl super::Etha0Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Mode Configuration Register"]
    #[inline(always)]
    pub const fn eamc(&self) -> &'static crate::common::Reg<self::Eamc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eamc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Mode Status Register"]
    #[inline(always)]
    pub const fn eams(&self) -> &'static crate::common::Reg<self::Eams_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eams_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "IPV Remapping Configuration Register \\[802.1Q\\]"]
    #[inline(always)]
    pub const fn eairc(&self) -> &'static crate::common::Reg<self::Eairc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eairc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "TX Descriptor Queue Security Configuration Register"]
    #[inline(always)]
    pub const fn eatdqsc(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "TX Descriptor Queue Configuration Register"]
    #[inline(always)]
    pub const fn eatdqc(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "TX Descriptor Queue Arbitration Configuration Register"]
    #[inline(always)]
    pub const fn eatdqac(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "TX Preemption Configuration Register"]
    #[inline(always)]
    pub const fn eatpec(
        &self,
    ) -> &'static crate::common::Reg<self::Eatpec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatpec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Transmission Maximum Frame Size Configuration Register %s"]
    #[inline(always)]
    pub const fn eatmfsc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn eatmfsc0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatmfsc1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatmfsc2(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatmfsc3(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatmfsc4(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatmfsc5(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatmfsc6(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatmfsc7(
        &self,
    ) -> &'static crate::common::Reg<self::Eatmfsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatmfsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }

    #[doc = "Transmission Descriptor Queue Depth Configuration Register %s"]
    #[inline(always)]
    pub const fn eatdqdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60usize))
        }
    }
    #[inline(always)]
    pub const fn eatdqdc0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqdc1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqdc2(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqdc3(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqdc4(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqdc5(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqdc6(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqdc7(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }

    #[doc = "Transmission Descriptor Queue %s Monitoring Register"]
    #[inline(always)]
    pub const fn eatdqm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eatdqm_SPEC, crate::common::R>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn eatdqm0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqm1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqm2(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqm3(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqm4(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqm5(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqm6(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqm7(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }

    #[doc = "Transmission Descriptor Queue %s Max Level Monitoring Register"]
    #[inline(always)]
    pub const fn eatdqmlm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa0usize))
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm2(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm3(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm4(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm5(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm6(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatdqmlm7(
        &self,
    ) -> &'static crate::common::Reg<self::Eatdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }

    #[doc = "Cut-Through Queue Configuration Register"]
    #[inline(always)]
    pub const fn eactqc(
        &self,
    ) -> &'static crate::common::Reg<self::Eactqc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eactqc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Cut-Through Descriptor Queue Depth Configuration Register"]
    #[inline(always)]
    pub const fn eactdqdc(
        &self,
    ) -> &'static crate::common::Reg<self::Eactdqdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eactdqdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Cut-Through Descriptor Queue Monitoring Register"]
    #[inline(always)]
    pub const fn eactdqm(
        &self,
    ) -> &'static crate::common::Reg<self::Eactdqm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eactdqm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Cut-Through Descriptor Queue Max Level Monitoring Register"]
    #[inline(always)]
    pub const fn eactdqmlm(
        &self,
    ) -> &'static crate::common::Reg<self::Eactdqmlm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eactdqmlm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "VLAN Control Configuration Register"]
    #[inline(always)]
    pub const fn eavcc(&self) -> &'static crate::common::Reg<self::Eavcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eavcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "VLAN TAG Configuration Register"]
    #[inline(always)]
    pub const fn eavtc(&self) -> &'static crate::common::Reg<self::Eavtc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eavtc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "Reception TAG Filtering Configuration Register"]
    #[inline(always)]
    pub const fn eartfc(
        &self,
    ) -> &'static crate::common::Reg<self::Eartfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eartfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[doc = "CBS Admin Enable Configuration Register"]
    #[inline(always)]
    pub const fn eacaec(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "CBS Configuration Register"]
    #[inline(always)]
    pub const fn eacc(&self) -> &'static crate::common::Reg<self::Eacc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "CBS Admin Increment Value Configuration Register %s"]
    #[inline(always)]
    pub const fn eacaivc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x220usize))
        }
    }
    #[inline(always)]
    pub const fn eacaivc0(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacaivc1(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacaivc2(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacaivc3(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacaivc4(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacaivc5(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacaivc6(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacaivc7(
        &self,
    ) -> &'static crate::common::Reg<self::Eacaivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eacaivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23cusize),
            )
        }
    }

    #[doc = "CBS Admin Upper Limit Configuration Register q"]
    #[inline(always)]
    pub const fn eacaulcq(
        &self,
    ) -> &'static crate::common::Reg<self::EacaulCq_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EacaulCq_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(576usize),
            )
        }
    }

    #[doc = "CBS Oper Enable Monitoring Register"]
    #[inline(always)]
    pub const fn eacoem(&self) -> &'static crate::common::Reg<self::Eacoem_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoem_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(608usize),
            )
        }
    }

    #[doc = "CBS Oper Increment Value Monitoring Register %s"]
    #[inline(always)]
    pub const fn eacoivm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eacoivm_SPEC, crate::common::R>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }
    #[inline(always)]
    pub const fn eacoivm0(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoivm1(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoivm2(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoivm3(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x28cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoivm4(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoivm5(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoivm6(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoivm7(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoivm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoivm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x29cusize),
            )
        }
    }

    #[doc = "CBS Oper Upper Limit Monitoring Register %s"]
    #[inline(always)]
    pub const fn eacoulm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eacoulm_SPEC, crate::common::R>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a0usize))
        }
    }
    #[inline(always)]
    pub const fn eacoulm0(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoulm1(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoulm2(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoulm3(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoulm4(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoulm5(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoulm6(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eacoulm7(
        &self,
    ) -> &'static crate::common::Reg<self::Eacoulm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacoulm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2bcusize),
            )
        }
    }

    #[doc = "CBS Gate State Monitoring Register"]
    #[inline(always)]
    pub const fn eacgsm(&self) -> &'static crate::common::Reg<self::Eacgsm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eacgsm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(704usize),
            )
        }
    }

    #[doc = "TAS Configuration Register"]
    #[inline(always)]
    pub const fn eatasc(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "TAS Initial Gate State Configuration Register"]
    #[inline(always)]
    pub const fn eatasigsc(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasigsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasigsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "TAS Entry Number Configuration Register %s"]
    #[inline(always)]
    pub const fn eatasenc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x320usize))
        }
    }
    #[inline(always)]
    pub const fn eatasenc0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc2(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc3(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc4(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc5(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc6(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc7(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenc8(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }

    #[doc = "TAS Cut-Through Entry Number Configuration Register"]
    #[inline(always)]
    pub const fn eatasctenc(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasctenc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasctenc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(832usize),
            )
        }
    }

    #[doc = "TAS Entry Number Monitoring Register %s"]
    #[inline(always)]
    pub const fn eatasenm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Eatasenm_SPEC, crate::common::R>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x360usize))
        }
    }
    #[inline(always)]
    pub const fn eatasenm0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x360usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x364usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm2(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x368usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm3(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x36cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm4(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x370usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm5(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x374usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm6(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x378usize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm7(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x37cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn eatasenm8(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x380usize),
            )
        }
    }

    #[doc = "TAS Cut-Through Entry Number Monitoring Register"]
    #[inline(always)]
    pub const fn eatasctenm(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasctenm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasctenm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(896usize),
            )
        }
    }

    #[doc = "TAS Cycle Start Time Configuration Register 0"]
    #[inline(always)]
    pub const fn eatascstc0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatascstc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatascstc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(928usize),
            )
        }
    }

    #[doc = "TAS Cycle Start Time Configuration Register 1"]
    #[inline(always)]
    pub const fn eatascstc1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatascstc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatascstc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(932usize),
            )
        }
    }

    #[doc = "TAS Cycle Start Time Monitoring Register 0"]
    #[inline(always)]
    pub const fn eatascstm0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatascstm0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatascstm0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(936usize),
            )
        }
    }

    #[doc = "TAS Cycle Start Time Monitoring Register 1"]
    #[inline(always)]
    pub const fn eatascstm1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatascstm1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatascstm1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(940usize),
            )
        }
    }

    #[doc = "TAS Cycle Time Configuration Register"]
    #[inline(always)]
    pub const fn eatasctc(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasctc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasctc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(944usize),
            )
        }
    }

    #[doc = "TAS Cycle Time Monitoring Register"]
    #[inline(always)]
    pub const fn eatasctm(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasctm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasctm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(948usize),
            )
        }
    }

    #[doc = "TAS Gate Learn Register 0"]
    #[inline(always)]
    pub const fn eatasgl0(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasgl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasgl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(960usize),
            )
        }
    }

    #[doc = "TAS Gate Learn Register 1"]
    #[inline(always)]
    pub const fn eatasgl1(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasgl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasgl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(964usize),
            )
        }
    }

    #[doc = "TAS Gate Learn Result Register"]
    #[inline(always)]
    pub const fn eatasglr(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasglr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasglr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(968usize),
            )
        }
    }

    #[doc = "TAS Gate Read Register"]
    #[inline(always)]
    pub const fn eatasgr(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasgr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasgr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(976usize),
            )
        }
    }

    #[doc = "TAS Gate Read Result Register"]
    #[inline(always)]
    pub const fn eatasgrr(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasgrr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatasgrr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(980usize),
            )
        }
    }

    #[doc = "TAS Hardware Calibration Configuration Register"]
    #[inline(always)]
    pub const fn eatashcc(
        &self,
    ) -> &'static crate::common::Reg<self::Eatashcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatashcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(992usize),
            )
        }
    }

    #[doc = "TAS RAM Initialization Register Monitoring Register"]
    #[inline(always)]
    pub const fn eatasrirm(
        &self,
    ) -> &'static crate::common::Reg<self::Eatasrirm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eatasrirm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(996usize),
            )
        }
    }

    #[doc = "TAS Status Monitoring Register"]
    #[inline(always)]
    pub const fn eatassm(
        &self,
    ) -> &'static crate::common::Reg<self::Eatassm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatassm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1000usize),
            )
        }
    }

    #[doc = "Under Minimum Frame Size Error Counter Register"]
    #[inline(always)]
    pub const fn eausmfsecn(
        &self,
    ) -> &'static crate::common::Reg<self::Eausmfsecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eausmfsecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "TAG Filtering Error Counter Register"]
    #[inline(always)]
    pub const fn eatfecn(
        &self,
    ) -> &'static crate::common::Reg<self::Eatfecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eatfecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "Frame Size Error Counter Register"]
    #[inline(always)]
    pub const fn eafsecn(
        &self,
    ) -> &'static crate::common::Reg<self::Eafsecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eafsecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1032usize),
            )
        }
    }

    #[doc = "Descriptor Queue Overflow Error Counter Register"]
    #[inline(always)]
    pub const fn eadqoecn(
        &self,
    ) -> &'static crate::common::Reg<self::Eadqoecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eadqoecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1036usize),
            )
        }
    }

    #[doc = "Descriptor Queue Security Error Counter Register"]
    #[inline(always)]
    pub const fn eadqsecn(
        &self,
    ) -> &'static crate::common::Reg<self::Eadqsecn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eadqsecn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn eaeis0(
        &self,
    ) -> &'static crate::common::Reg<self::Eaeis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eaeis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn eaeie0(
        &self,
    ) -> &'static crate::common::Reg<self::Eaeie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eaeie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn eaeid0(&self) -> &'static crate::common::Reg<self::Eaeid0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eaeid0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn eaeis1(
        &self,
    ) -> &'static crate::common::Reg<self::Eaeis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eaeis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1296usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn eaeie1(
        &self,
    ) -> &'static crate::common::Reg<self::Eaeie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eaeie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn eaeid1(&self) -> &'static crate::common::Reg<self::Eaeid1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eaeid1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1304usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn eaeis2(
        &self,
    ) -> &'static crate::common::Reg<self::Eaeis2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eaeis2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1312usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn eaeie2(
        &self,
    ) -> &'static crate::common::Reg<self::Eaeie2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eaeie2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1316usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn eaeid2(&self) -> &'static crate::common::Reg<self::Eaeid2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Eaeid2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1320usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eamc_SPEC;
impl crate::sealed::RegSpec for Eamc_SPEC {
    type DataType = u32;
}

#[doc = "Mode Configuration Register"]
pub type Eamc = crate::RegValueT<Eamc_SPEC>;

impl Eamc {
    #[doc = "Operating Mode Command"]
    #[inline(always)]
    pub fn opc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        eamc::Opc,
        eamc::Opc,
        Eamc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            eamc::Opc,
            eamc::Opc,
            Eamc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eamc {
    #[inline(always)]
    fn default() -> Eamc {
        <crate::RegValueT<Eamc_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod eamc {

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
pub struct Eams_SPEC;
impl crate::sealed::RegSpec for Eams_SPEC {
    type DataType = u32;
}

#[doc = "Mode Status Register"]
pub type Eams = crate::RegValueT<Eams_SPEC>;

impl Eams {
    #[doc = "Operating Mode Status Flag"]
    #[inline(always)]
    pub fn ops(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, eams::Ops, eams::Ops, Eams_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            eams::Ops,
            eams::Ops,
            Eams_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eams {
    #[inline(always)]
    fn default() -> Eams {
        <crate::RegValueT<Eams_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod eams {

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
pub struct Eairc_SPEC;
impl crate::sealed::RegSpec for Eairc_SPEC {
    type DataType = u32;
}

#[doc = "IPV Remapping Configuration Register \\[802.1Q\\]"]
pub type Eairc = crate::RegValueT<Eairc_SPEC>;

impl Eairc {
    #[doc = "IPV Remapping 0"]
    #[inline(always)]
    pub fn ipvr0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Remapping 1"]
    #[inline(always)]
    pub fn ipvr1(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Remapping 2"]
    #[inline(always)]
    pub fn ipvr2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Remapping 3"]
    #[inline(always)]
    pub fn ipvr3(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Remapping 4"]
    #[inline(always)]
    pub fn ipvr4(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Remapping 5"]
    #[inline(always)]
    pub fn ipvr5(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Remapping 6"]
    #[inline(always)]
    pub fn ipvr6(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Remapping 7"]
    #[inline(always)]
    pub fn ipvr7(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Eairc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Eairc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eairc {
    #[inline(always)]
    fn default() -> Eairc {
        <crate::RegValueT<Eairc_SPEC> as RegisterValue<_>>::new(1985229328)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatdqsc_SPEC;
impl crate::sealed::RegSpec for Eatdqsc_SPEC {
    type DataType = u32;
}

#[doc = "TX Descriptor Queue Security Configuration Register"]
pub type Eatdqsc = crate::RegValueT<Eatdqsc_SPEC>;

impl Eatdqsc {
    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl0,
        eatdqsc::Tdqsl0,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl0,
            eatdqsc::Tdqsl0,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl1,
        eatdqsc::Tdqsl1,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl1,
            eatdqsc::Tdqsl1,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl2,
        eatdqsc::Tdqsl2,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl2,
            eatdqsc::Tdqsl2,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl3,
        eatdqsc::Tdqsl3,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl3,
            eatdqsc::Tdqsl3,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl4,
        eatdqsc::Tdqsl4,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl4,
            eatdqsc::Tdqsl4,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl5,
        eatdqsc::Tdqsl5,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl5,
            eatdqsc::Tdqsl5,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl6,
        eatdqsc::Tdqsl6,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl6,
            eatdqsc::Tdqsl6,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Security Level (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqsl7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eatdqsc::Tdqsl7,
        eatdqsc::Tdqsl7,
        Eatdqsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eatdqsc::Tdqsl7,
            eatdqsc::Tdqsl7,
            Eatdqsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatdqsc {
    #[inline(always)]
    fn default() -> Eatdqsc {
        <crate::RegValueT<Eatdqsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eatdqsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl0_SPEC;
    pub type Tdqsl0 = crate::EnumBitfieldStruct<u8, Tdqsl0_SPEC>;
    impl Tdqsl0 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl1_SPEC;
    pub type Tdqsl1 = crate::EnumBitfieldStruct<u8, Tdqsl1_SPEC>;
    impl Tdqsl1 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl2_SPEC;
    pub type Tdqsl2 = crate::EnumBitfieldStruct<u8, Tdqsl2_SPEC>;
    impl Tdqsl2 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl3_SPEC;
    pub type Tdqsl3 = crate::EnumBitfieldStruct<u8, Tdqsl3_SPEC>;
    impl Tdqsl3 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl4_SPEC;
    pub type Tdqsl4 = crate::EnumBitfieldStruct<u8, Tdqsl4_SPEC>;
    impl Tdqsl4 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl5_SPEC;
    pub type Tdqsl5 = crate::EnumBitfieldStruct<u8, Tdqsl5_SPEC>;
    impl Tdqsl5 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl6_SPEC;
    pub type Tdqsl6 = crate::EnumBitfieldStruct<u8, Tdqsl6_SPEC>;
    impl Tdqsl6 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqsl7_SPEC;
    pub type Tdqsl7 = crate::EnumBitfieldStruct<u8, Tdqsl7_SPEC>;
    impl Tdqsl7 {
        #[doc = "Queue i unsecure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatdqc_SPEC;
impl crate::sealed::RegSpec for Eatdqc_SPEC {
    type DataType = u32;
}

#[doc = "TX Descriptor Queue Configuration Register"]
pub type Eatdqc = crate::RegValueT<Eatdqc_SPEC>;

impl Eatdqc {
    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eatdqc::Tdqd0,
        eatdqc::Tdqd0,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eatdqc::Tdqd0,
            eatdqc::Tdqd0,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eatdqc::Tdqd1,
        eatdqc::Tdqd1,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eatdqc::Tdqd1,
            eatdqc::Tdqd1,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eatdqc::Tdqd2,
        eatdqc::Tdqd2,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eatdqc::Tdqd2,
            eatdqc::Tdqd2,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eatdqc::Tdqd3,
        eatdqc::Tdqd3,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eatdqc::Tdqd3,
            eatdqc::Tdqd3,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eatdqc::Tdqd4,
        eatdqc::Tdqd4,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eatdqc::Tdqd4,
            eatdqc::Tdqd4,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eatdqc::Tdqd5,
        eatdqc::Tdqd5,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eatdqc::Tdqd5,
            eatdqc::Tdqd5,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eatdqc::Tdqd6,
        eatdqc::Tdqd6,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eatdqc::Tdqd6,
            eatdqc::Tdqd6,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqd7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eatdqc::Tdqd7,
        eatdqc::Tdqd7,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eatdqc::Tdqd7,
            eatdqc::Tdqd7,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Cut-Through Descriptor Queue Disable"]
    #[inline(always)]
    pub fn tctdqd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eatdqc::Tctdqd,
        eatdqc::Tctdqd,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eatdqc::Tctdqd,
            eatdqc::Tctdqd,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        eatdqc::Tdqp0,
        eatdqc::Tdqp0,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            eatdqc::Tdqp0,
            eatdqc::Tdqp0,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        eatdqc::Tdqp1,
        eatdqc::Tdqp1,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            eatdqc::Tdqp1,
            eatdqc::Tdqp1,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        eatdqc::Tdqp2,
        eatdqc::Tdqp2,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            eatdqc::Tdqp2,
            eatdqc::Tdqp2,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        eatdqc::Tdqp3,
        eatdqc::Tdqp3,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            eatdqc::Tdqp3,
            eatdqc::Tdqp3,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        eatdqc::Tdqp4,
        eatdqc::Tdqp4,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            eatdqc::Tdqp4,
            eatdqc::Tdqp4,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        eatdqc::Tdqp5,
        eatdqc::Tdqp5,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            eatdqc::Tdqp5,
            eatdqc::Tdqp5,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        eatdqc::Tdqp6,
        eatdqc::Tdqp6,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            eatdqc::Tdqp6,
            eatdqc::Tdqp6,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue n Pause (n = 0 to 7)"]
    #[inline(always)]
    pub fn tdqp7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        eatdqc::Tdqp7,
        eatdqc::Tdqp7,
        Eatdqc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            eatdqc::Tdqp7,
            eatdqc::Tdqp7,
            Eatdqc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatdqc {
    #[inline(always)]
    fn default() -> Eatdqc {
        <crate::RegValueT<Eatdqc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eatdqc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd0_SPEC;
    pub type Tdqd0 = crate::EnumBitfieldStruct<u8, Tdqd0_SPEC>;
    impl Tdqd0 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd1_SPEC;
    pub type Tdqd1 = crate::EnumBitfieldStruct<u8, Tdqd1_SPEC>;
    impl Tdqd1 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd2_SPEC;
    pub type Tdqd2 = crate::EnumBitfieldStruct<u8, Tdqd2_SPEC>;
    impl Tdqd2 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd3_SPEC;
    pub type Tdqd3 = crate::EnumBitfieldStruct<u8, Tdqd3_SPEC>;
    impl Tdqd3 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd4_SPEC;
    pub type Tdqd4 = crate::EnumBitfieldStruct<u8, Tdqd4_SPEC>;
    impl Tdqd4 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd5_SPEC;
    pub type Tdqd5 = crate::EnumBitfieldStruct<u8, Tdqd5_SPEC>;
    impl Tdqd5 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd6_SPEC;
    pub type Tdqd6 = crate::EnumBitfieldStruct<u8, Tdqd6_SPEC>;
    impl Tdqd6 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqd7_SPEC;
    pub type Tdqd7 = crate::EnumBitfieldStruct<u8, Tdqd7_SPEC>;
    impl Tdqd7 {
        #[doc = "Queue i enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tctdqd_SPEC;
    pub type Tctdqd = crate::EnumBitfieldStruct<u8, Tctdqd_SPEC>;
    impl Tctdqd {
        #[doc = "Cut-through queue enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cut-through queue disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp0_SPEC;
    pub type Tdqp0 = crate::EnumBitfieldStruct<u8, Tdqp0_SPEC>;
    impl Tdqp0 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp1_SPEC;
    pub type Tdqp1 = crate::EnumBitfieldStruct<u8, Tdqp1_SPEC>;
    impl Tdqp1 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp2_SPEC;
    pub type Tdqp2 = crate::EnumBitfieldStruct<u8, Tdqp2_SPEC>;
    impl Tdqp2 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp3_SPEC;
    pub type Tdqp3 = crate::EnumBitfieldStruct<u8, Tdqp3_SPEC>;
    impl Tdqp3 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp4_SPEC;
    pub type Tdqp4 = crate::EnumBitfieldStruct<u8, Tdqp4_SPEC>;
    impl Tdqp4 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp5_SPEC;
    pub type Tdqp5 = crate::EnumBitfieldStruct<u8, Tdqp5_SPEC>;
    impl Tdqp5 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp6_SPEC;
    pub type Tdqp6 = crate::EnumBitfieldStruct<u8, Tdqp6_SPEC>;
    impl Tdqp6 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqp7_SPEC;
    pub type Tdqp7 = crate::EnumBitfieldStruct<u8, Tdqp7_SPEC>;
    impl Tdqp7 {
        #[doc = "Queue i active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i paused"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatdqac_SPEC;
impl crate::sealed::RegSpec for Eatdqac_SPEC {
    type DataType = u32;
}

#[doc = "TX Descriptor Queue Arbitration Configuration Register"]
pub type Eatdqac = crate::RegValueT<Eatdqac_SPEC>;

impl Eatdqac {
    #[doc = "TX Descriptor Queue Arbitration 0"]
    #[inline(always)]
    pub fn tdqa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        eatdqac::Tdqa0,
        eatdqac::Tdqa0,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            eatdqac::Tdqa0,
            eatdqac::Tdqa0,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue Arbitration 1"]
    #[inline(always)]
    pub fn tdqa1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        eatdqac::Tdqa1,
        eatdqac::Tdqa1,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            eatdqac::Tdqa1,
            eatdqac::Tdqa1,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue Arbitration 2"]
    #[inline(always)]
    pub fn tdqa2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        eatdqac::Tdqa2,
        eatdqac::Tdqa2,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            eatdqac::Tdqa2,
            eatdqac::Tdqa2,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue Arbitration 3"]
    #[inline(always)]
    pub fn tdqa3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        eatdqac::Tdqa3,
        eatdqac::Tdqa3,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            eatdqac::Tdqa3,
            eatdqac::Tdqa3,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue Arbitration 4"]
    #[inline(always)]
    pub fn tdqa4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        eatdqac::Tdqa4,
        eatdqac::Tdqa4,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            eatdqac::Tdqa4,
            eatdqac::Tdqa4,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue Arbitration 5"]
    #[inline(always)]
    pub fn tdqa5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        eatdqac::Tdqa5,
        eatdqac::Tdqa5,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            eatdqac::Tdqa5,
            eatdqac::Tdqa5,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue Arbitration 6"]
    #[inline(always)]
    pub fn tdqa6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        eatdqac::Tdqa6,
        eatdqac::Tdqa6,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            eatdqac::Tdqa6,
            eatdqac::Tdqa6,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TX Descriptor Queue Arbitration 7"]
    #[inline(always)]
    pub fn tdqa7(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        eatdqac::Tdqa7,
        eatdqac::Tdqa7,
        Eatdqac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            eatdqac::Tdqa7,
            eatdqac::Tdqa7,
            Eatdqac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatdqac {
    #[inline(always)]
    fn default() -> Eatdqac {
        <crate::RegValueT<Eatdqac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eatdqac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa0_SPEC;
    pub type Tdqa0 = crate::EnumBitfieldStruct<u8, Tdqa0_SPEC>;
    impl Tdqa0 {
        #[doc = "Queue 0 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 0 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa1_SPEC;
    pub type Tdqa1 = crate::EnumBitfieldStruct<u8, Tdqa1_SPEC>;
    impl Tdqa1 {
        #[doc = "Queue 1 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 1 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa2_SPEC;
    pub type Tdqa2 = crate::EnumBitfieldStruct<u8, Tdqa2_SPEC>;
    impl Tdqa2 {
        #[doc = "Queue 2 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 2 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa3_SPEC;
    pub type Tdqa3 = crate::EnumBitfieldStruct<u8, Tdqa3_SPEC>;
    impl Tdqa3 {
        #[doc = "Queue 3 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 3 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa4_SPEC;
    pub type Tdqa4 = crate::EnumBitfieldStruct<u8, Tdqa4_SPEC>;
    impl Tdqa4 {
        #[doc = "Queue 4 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 4 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa5_SPEC;
    pub type Tdqa5 = crate::EnumBitfieldStruct<u8, Tdqa5_SPEC>;
    impl Tdqa5 {
        #[doc = "Queue 5 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 5 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa6_SPEC;
    pub type Tdqa6 = crate::EnumBitfieldStruct<u8, Tdqa6_SPEC>;
    impl Tdqa6 {
        #[doc = "Queue 6 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 6 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdqa7_SPEC;
    pub type Tdqa7 = crate::EnumBitfieldStruct<u8, Tdqa7_SPEC>;
    impl Tdqa7 {
        #[doc = "Queue 7 strict arbitration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue 7 WRR arbitration"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatpec_SPEC;
impl crate::sealed::RegSpec for Eatpec_SPEC {
    type DataType = u32;
}

#[doc = "TX Preemption Configuration Register"]
pub type Eatpec = crate::RegValueT<Eatpec_SPEC>;

impl Eatpec {
    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eatpec::Ttq0,
        eatpec::Ttq0,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eatpec::Ttq0,
            eatpec::Ttq0,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eatpec::Ttq1,
        eatpec::Ttq1,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eatpec::Ttq1,
            eatpec::Ttq1,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eatpec::Ttq2,
        eatpec::Ttq2,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eatpec::Ttq2,
            eatpec::Ttq2,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eatpec::Ttq3,
        eatpec::Ttq3,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eatpec::Ttq3,
            eatpec::Ttq3,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eatpec::Ttq4,
        eatpec::Ttq4,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eatpec::Ttq4,
            eatpec::Ttq4,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eatpec::Ttq5,
        eatpec::Ttq5,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eatpec::Ttq5,
            eatpec::Ttq5,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eatpec::Ttq6,
        eatpec::Ttq6,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eatpec::Ttq6,
            eatpec::Ttq6,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Type Queue n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ttq7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eatpec::Ttq7,
        eatpec::Ttq7,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eatpec::Ttq7,
            eatpec::Ttq7,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Additional Fragment Size"]
    #[inline(always)]
    pub fn afs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        eatpec::Afs,
        eatpec::Afs,
        Eatpec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            eatpec::Afs,
            eatpec::Afs,
            Eatpec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatpec {
    #[inline(always)]
    fn default() -> Eatpec {
        <crate::RegValueT<Eatpec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eatpec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq0_SPEC;
    pub type Ttq0 = crate::EnumBitfieldStruct<u8, Ttq0_SPEC>;
    impl Ttq0 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq1_SPEC;
    pub type Ttq1 = crate::EnumBitfieldStruct<u8, Ttq1_SPEC>;
    impl Ttq1 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq2_SPEC;
    pub type Ttq2 = crate::EnumBitfieldStruct<u8, Ttq2_SPEC>;
    impl Ttq2 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq3_SPEC;
    pub type Ttq3 = crate::EnumBitfieldStruct<u8, Ttq3_SPEC>;
    impl Ttq3 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq4_SPEC;
    pub type Ttq4 = crate::EnumBitfieldStruct<u8, Ttq4_SPEC>;
    impl Ttq4 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq5_SPEC;
    pub type Ttq5 = crate::EnumBitfieldStruct<u8, Ttq5_SPEC>;
    impl Ttq5 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq6_SPEC;
    pub type Ttq6 = crate::EnumBitfieldStruct<u8, Ttq6_SPEC>;
    impl Ttq6 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttq7_SPEC;
    pub type Ttq7 = crate::EnumBitfieldStruct<u8, Ttq7_SPEC>;
    impl Ttq7 {
        #[doc = "Queue i contains e-frames"]
        pub const _0: Self = Self::new(0);

        #[doc = "Queue i contains p-frames"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Afs_SPEC;
    pub type Afs = crate::EnumBitfieldStruct<u8, Afs_SPEC>;
    impl Afs {
        #[doc = "0 byte is added to minimum fragment size (minimum fragment size = 64 bytes)"]
        pub const _00: Self = Self::new(0);

        #[doc = "64 bytes are added to minimum fragment size (minimum fragment size = 128 bytes)"]
        pub const _01: Self = Self::new(1);

        #[doc = "128 bytes are added to minimum fragment size (minimum fragment size = 192 bytes)"]
        pub const _10: Self = Self::new(2);

        #[doc = "192 bytes are added to minimum fragment size (minimum fragment size = 256 bytes)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatmfsc_SPEC;
impl crate::sealed::RegSpec for Eatmfsc_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Maximum Frame Size Configuration Register %s"]
pub type Eatmfsc = crate::RegValueT<Eatmfsc_SPEC>;

impl Eatmfsc {
    #[doc = "Maximum Frame Size"]
    #[inline(always)]
    pub fn mfs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eatmfsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eatmfsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatmfsc {
    #[inline(always)]
    fn default() -> Eatmfsc {
        <crate::RegValueT<Eatmfsc_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatdqdc_SPEC;
impl crate::sealed::RegSpec for Eatdqdc_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Descriptor Queue Depth Configuration Register %s"]
pub type Eatdqdc = crate::RegValueT<Eatdqdc_SPEC>;

impl Eatdqdc {
    #[doc = "Descriptor Queue Depth"]
    #[inline(always)]
    pub fn dqd(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Eatdqdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Eatdqdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatdqdc {
    #[inline(always)]
    fn default() -> Eatdqdc {
        <crate::RegValueT<Eatdqdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatdqm_SPEC;
impl crate::sealed::RegSpec for Eatdqm_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Descriptor Queue %s Monitoring Register"]
pub type Eatdqm = crate::RegValueT<Eatdqm_SPEC>;

impl Eatdqm {
    #[doc = "Descriptor Number in Queue"]
    #[inline(always)]
    pub fn dnq(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Eatdqm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Eatdqm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatdqm {
    #[inline(always)]
    fn default() -> Eatdqm {
        <crate::RegValueT<Eatdqm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatdqmlm_SPEC;
impl crate::sealed::RegSpec for Eatdqmlm_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Descriptor Queue %s Max Level Monitoring Register"]
pub type Eatdqmlm = crate::RegValueT<Eatdqmlm_SPEC>;

impl Eatdqmlm {
    #[doc = "Descriptor Max Level in Queue"]
    #[inline(always)]
    pub fn dmlq(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Eatdqmlm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Eatdqmlm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatdqmlm {
    #[inline(always)]
    fn default() -> Eatdqmlm {
        <crate::RegValueT<Eatdqmlm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eactqc_SPEC;
impl crate::sealed::RegSpec for Eactqc_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Queue Configuration Register"]
pub type Eactqc = crate::RegValueT<Eactqc_SPEC>;

impl Eactqc {
    #[doc = "Cut-Through Queue Delay (clk Cycle Number)"]
    #[inline(always)]
    pub fn ctqd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eactqc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eactqc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eactqc {
    #[inline(always)]
    fn default() -> Eactqc {
        <crate::RegValueT<Eactqc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eactdqdc_SPEC;
impl crate::sealed::RegSpec for Eactdqdc_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Descriptor Queue Depth Configuration Register"]
pub type Eactdqdc = crate::RegValueT<Eactdqdc_SPEC>;

impl Eactdqdc {
    #[doc = "Cut-Through Descriptor Queue Depth"]
    #[inline(always)]
    pub fn ctdqd(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Eactdqdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Eactdqdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eactdqdc {
    #[inline(always)]
    fn default() -> Eactdqdc {
        <crate::RegValueT<Eactdqdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eactdqm_SPEC;
impl crate::sealed::RegSpec for Eactdqm_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Descriptor Queue Monitoring Register"]
pub type Eactdqm = crate::RegValueT<Eactdqm_SPEC>;

impl Eactdqm {
    #[doc = "Cut-Though Queue Descriptor Number"]
    #[inline(always)]
    pub fn ctqdn(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Eactdqm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Eactdqm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eactdqm {
    #[inline(always)]
    fn default() -> Eactdqm {
        <crate::RegValueT<Eactdqm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eactdqmlm_SPEC;
impl crate::sealed::RegSpec for Eactdqmlm_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Descriptor Queue Max Level Monitoring Register"]
pub type Eactdqmlm = crate::RegValueT<Eactdqmlm_SPEC>;

impl Eactdqmlm {
    #[doc = "Cut-Through Descriptor Max Level in Queue"]
    #[inline(always)]
    pub fn ctdmlq(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Eactdqmlm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Eactdqmlm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eactdqmlm {
    #[inline(always)]
    fn default() -> Eactdqmlm {
        <crate::RegValueT<Eactdqmlm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eavcc_SPEC;
impl crate::sealed::RegSpec for Eavcc_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Control Configuration Register"]
pub type Eavcc = crate::RegValueT<Eavcc_SPEC>;

impl Eavcc {
    #[doc = "VLAN Ingress Mode"]
    #[inline(always)]
    pub fn vim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eavcc::Vim,
        eavcc::Vim,
        Eavcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eavcc::Vim,
            eavcc::Vim,
            Eavcc_SPEC,
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
        eavcc::Vem,
        eavcc::Vem,
        Eavcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            eavcc::Vem,
            eavcc::Vem,
            Eavcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eavcc {
    #[inline(always)]
    fn default() -> Eavcc {
        <crate::RegValueT<Eavcc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eavcc {

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
        #[doc = "No VLAN mode"]
        pub const _000: Self = Self::new(0);

        #[doc = "C-TAG VLAN mode"]
        pub const _001: Self = Self::new(1);

        #[doc = "HW C-TAG VLAN mode"]
        pub const _010: Self = Self::new(2);

        #[doc = "SC-TAG VLAN mode"]
        pub const _011: Self = Self::new(3);

        #[doc = "HW SC-TAG VLAN mode"]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eavtc_SPEC;
impl crate::sealed::RegSpec for Eavtc_SPEC {
    type DataType = u32;
}

#[doc = "VLAN TAG Configuration Register"]
pub type Eavtc = crate::RegValueT<Eavtc_SPEC>;

impl Eavtc {
    #[doc = "C-TAG VLAN"]
    #[inline(always)]
    pub fn ctv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Eavtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Eavtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "C-TAG PCP"]
    #[inline(always)]
    pub fn ctp(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Eavtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Eavtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "C-TAG DEI"]
    #[inline(always)]
    pub fn ctd(self) -> crate::common::RegisterFieldBool<15, 1, 0, Eavtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Eavtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "S-TAG VLAN"]
    #[inline(always)]
    pub fn stv(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Eavtc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Eavtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "S-TAG PCP"]
    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Eavtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Eavtc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "S-TAG DEI"]
    #[inline(always)]
    pub fn std(self) -> crate::common::RegisterFieldBool<31, 1, 0, Eavtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Eavtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Eavtc {
    #[inline(always)]
    fn default() -> Eavtc {
        <crate::RegValueT<Eavtc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eartfc_SPEC;
impl crate::sealed::RegSpec for Eartfc_SPEC {
    type DataType = u32;
}

#[doc = "Reception TAG Filtering Configuration Register"]
pub type Eartfc = crate::RegValueT<Eartfc_SPEC>;

impl Eartfc {
    #[doc = "No TAG"]
    #[inline(always)]
    pub fn nt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eartfc::Nt,
        eartfc::Nt,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eartfc::Nt,
            eartfc::Nt,
            Eartfc_SPEC,
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
        eartfc::Rt,
        eartfc::Rt,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eartfc::Rt,
            eartfc::Rt,
            Eartfc_SPEC,
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
        eartfc::Cst,
        eartfc::Cst,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eartfc::Cst,
            eartfc::Cst,
            Eartfc_SPEC,
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
        eartfc::Csrt,
        eartfc::Csrt,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eartfc::Csrt,
            eartfc::Csrt,
            Eartfc_SPEC,
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
        eartfc::Ct,
        eartfc::Ct,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eartfc::Ct,
            eartfc::Ct,
            Eartfc_SPEC,
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
        eartfc::Crt,
        eartfc::Crt,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eartfc::Crt,
            eartfc::Crt,
            Eartfc_SPEC,
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
        eartfc::Sct,
        eartfc::Sct,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eartfc::Sct,
            eartfc::Sct,
            Eartfc_SPEC,
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
        eartfc::Scrt,
        eartfc::Scrt,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eartfc::Scrt,
            eartfc::Scrt,
            Eartfc_SPEC,
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
        eartfc::Ut,
        eartfc::Ut,
        Eartfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eartfc::Ut,
            eartfc::Ut,
            Eartfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eartfc {
    #[inline(always)]
    fn default() -> Eartfc {
        <crate::RegValueT<Eartfc_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod eartfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nt_SPEC;
    pub type Nt = crate::EnumBitfieldStruct<u8, Nt_SPEC>;
    impl Nt {
        #[doc = "No TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "No TAG frame rejected"]
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
        #[doc = "Unknown TAG frame passed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown TAG frame rejected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eacaec_SPEC;
impl crate::sealed::RegSpec for Eacaec_SPEC {
    type DataType = u32;
}

#[doc = "CBS Admin Enable Configuration Register"]
pub type Eacaec = crate::RegValueT<Eacaec_SPEC>;

impl Eacaec {
    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eacaec::Ce0,
        eacaec::Ce0,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eacaec::Ce0,
            eacaec::Ce0,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eacaec::Ce1,
        eacaec::Ce1,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eacaec::Ce1,
            eacaec::Ce1,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eacaec::Ce2,
        eacaec::Ce2,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eacaec::Ce2,
            eacaec::Ce2,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eacaec::Ce3,
        eacaec::Ce3,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eacaec::Ce3,
            eacaec::Ce3,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eacaec::Ce4,
        eacaec::Ce4,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eacaec::Ce4,
            eacaec::Ce4,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eacaec::Ce5,
        eacaec::Ce5,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eacaec::Ce5,
            eacaec::Ce5,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eacaec::Ce6,
        eacaec::Ce6,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eacaec::Ce6,
            eacaec::Ce6,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eacaec::Ce7,
        eacaec::Ce7,
        Eacaec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eacaec::Ce7,
            eacaec::Ce7,
            Eacaec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eacaec {
    #[inline(always)]
    fn default() -> Eacaec {
        <crate::RegValueT<Eacaec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eacaec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce0_SPEC;
    pub type Ce0 = crate::EnumBitfieldStruct<u8, Ce0_SPEC>;
    impl Ce0 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce1_SPEC;
    pub type Ce1 = crate::EnumBitfieldStruct<u8, Ce1_SPEC>;
    impl Ce1 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce2_SPEC;
    pub type Ce2 = crate::EnumBitfieldStruct<u8, Ce2_SPEC>;
    impl Ce2 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce3_SPEC;
    pub type Ce3 = crate::EnumBitfieldStruct<u8, Ce3_SPEC>;
    impl Ce3 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce4_SPEC;
    pub type Ce4 = crate::EnumBitfieldStruct<u8, Ce4_SPEC>;
    impl Ce4 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce5_SPEC;
    pub type Ce5 = crate::EnumBitfieldStruct<u8, Ce5_SPEC>;
    impl Ce5 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce6_SPEC;
    pub type Ce6 = crate::EnumBitfieldStruct<u8, Ce6_SPEC>;
    impl Ce6 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce7_SPEC;
    pub type Ce7 = crate::EnumBitfieldStruct<u8, Ce7_SPEC>;
    impl Ce7 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eacc_SPEC;
impl crate::sealed::RegSpec for Eacc_SPEC {
    type DataType = u32;
}

#[doc = "CBS Configuration Register"]
pub type Eacc = crate::RegValueT<Eacc_SPEC>;

impl Eacc {
    #[doc = "Configuration Change n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cc7_to_cc0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Eacc_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Eacc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eacc {
    #[inline(always)]
    fn default() -> Eacc {
        <crate::RegValueT<Eacc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eacaivc_SPEC;
impl crate::sealed::RegSpec for Eacaivc_SPEC {
    type DataType = u32;
}

#[doc = "CBS Admin Increment Value Configuration Register %s"]
pub type Eacaivc = crate::RegValueT<Eacaivc_SPEC>;

impl Eacaivc {
    #[doc = "Credit Increment Value"]
    #[inline(always)]
    pub fn civ(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Eacaivc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Eacaivc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eacaivc {
    #[inline(always)]
    fn default() -> Eacaivc {
        <crate::RegValueT<Eacaivc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EacaulCq_SPEC;
impl crate::sealed::RegSpec for EacaulCq_SPEC {
    type DataType = u32;
}

#[doc = "CBS Admin Upper Limit Configuration Register q"]
pub type EacaulCq = crate::RegValueT<EacaulCq_SPEC>;

impl EacaulCq {
    #[doc = "Credit Upper Limit"]
    #[inline(always)]
    pub fn cul(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, u32, EacaulCq_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            EacaulCq_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EacaulCq {
    #[inline(always)]
    fn default() -> EacaulCq {
        <crate::RegValueT<EacaulCq_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eacoem_SPEC;
impl crate::sealed::RegSpec for Eacoem_SPEC {
    type DataType = u32;
}

#[doc = "CBS Oper Enable Monitoring Register"]
pub type Eacoem = crate::RegValueT<Eacoem_SPEC>;

impl Eacoem {
    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eacoem::Ce0,
        eacoem::Ce0,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eacoem::Ce0,
            eacoem::Ce0,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eacoem::Ce1,
        eacoem::Ce1,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eacoem::Ce1,
            eacoem::Ce1,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eacoem::Ce2,
        eacoem::Ce2,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eacoem::Ce2,
            eacoem::Ce2,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eacoem::Ce3,
        eacoem::Ce3,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eacoem::Ce3,
            eacoem::Ce3,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eacoem::Ce4,
        eacoem::Ce4,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eacoem::Ce4,
            eacoem::Ce4,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eacoem::Ce5,
        eacoem::Ce5,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eacoem::Ce5,
            eacoem::Ce5,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eacoem::Ce6,
        eacoem::Ce6,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eacoem::Ce6,
            eacoem::Ce6,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn ce7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eacoem::Ce7,
        eacoem::Ce7,
        Eacoem_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eacoem::Ce7,
            eacoem::Ce7,
            Eacoem_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eacoem {
    #[inline(always)]
    fn default() -> Eacoem {
        <crate::RegValueT<Eacoem_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eacoem {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce0_SPEC;
    pub type Ce0 = crate::EnumBitfieldStruct<u8, Ce0_SPEC>;
    impl Ce0 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce1_SPEC;
    pub type Ce1 = crate::EnumBitfieldStruct<u8, Ce1_SPEC>;
    impl Ce1 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce2_SPEC;
    pub type Ce2 = crate::EnumBitfieldStruct<u8, Ce2_SPEC>;
    impl Ce2 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce3_SPEC;
    pub type Ce3 = crate::EnumBitfieldStruct<u8, Ce3_SPEC>;
    impl Ce3 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce4_SPEC;
    pub type Ce4 = crate::EnumBitfieldStruct<u8, Ce4_SPEC>;
    impl Ce4 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce5_SPEC;
    pub type Ce5 = crate::EnumBitfieldStruct<u8, Ce5_SPEC>;
    impl Ce5 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce6_SPEC;
    pub type Ce6 = crate::EnumBitfieldStruct<u8, Ce6_SPEC>;
    impl Ce6 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce7_SPEC;
    pub type Ce7 = crate::EnumBitfieldStruct<u8, Ce7_SPEC>;
    impl Ce7 {
        #[doc = "CBS for descriptor queue i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS for descriptor queue i enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eacoivm_SPEC;
impl crate::sealed::RegSpec for Eacoivm_SPEC {
    type DataType = u32;
}

#[doc = "CBS Oper Increment Value Monitoring Register %s"]
pub type Eacoivm = crate::RegValueT<Eacoivm_SPEC>;

impl Eacoivm {
    #[doc = "Credit Increment Value"]
    #[inline(always)]
    pub fn civ(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Eacoivm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Eacoivm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eacoivm {
    #[inline(always)]
    fn default() -> Eacoivm {
        <crate::RegValueT<Eacoivm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eacoulm_SPEC;
impl crate::sealed::RegSpec for Eacoulm_SPEC {
    type DataType = u32;
}

#[doc = "CBS Oper Upper Limit Monitoring Register %s"]
pub type Eacoulm = crate::RegValueT<Eacoulm_SPEC>;

impl Eacoulm {
    #[doc = "Credit Upper Limit"]
    #[inline(always)]
    pub fn cul(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, u32, Eacoulm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32,u32,Eacoulm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eacoulm {
    #[inline(always)]
    fn default() -> Eacoulm {
        <crate::RegValueT<Eacoulm_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eacgsm_SPEC;
impl crate::sealed::RegSpec for Eacgsm_SPEC {
    type DataType = u32;
}

#[doc = "CBS Gate State Monitoring Register"]
pub type Eacgsm = crate::RegValueT<Eacgsm_SPEC>;

impl Eacgsm {
    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eacgsm::Cgs0,
        eacgsm::Cgs0,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eacgsm::Cgs0,
            eacgsm::Cgs0,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eacgsm::Cgs1,
        eacgsm::Cgs1,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eacgsm::Cgs1,
            eacgsm::Cgs1,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eacgsm::Cgs2,
        eacgsm::Cgs2,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eacgsm::Cgs2,
            eacgsm::Cgs2,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eacgsm::Cgs3,
        eacgsm::Cgs3,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eacgsm::Cgs3,
            eacgsm::Cgs3,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eacgsm::Cgs4,
        eacgsm::Cgs4,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eacgsm::Cgs4,
            eacgsm::Cgs4,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eacgsm::Cgs5,
        eacgsm::Cgs5,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eacgsm::Cgs5,
            eacgsm::Cgs5,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eacgsm::Cgs6,
        eacgsm::Cgs6,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eacgsm::Cgs6,
            eacgsm::Cgs6,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CBS Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn cgs7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eacgsm::Cgs7,
        eacgsm::Cgs7,
        Eacgsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eacgsm::Cgs7,
            eacgsm::Cgs7,
            Eacgsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eacgsm {
    #[inline(always)]
    fn default() -> Eacgsm {
        <crate::RegValueT<Eacgsm_SPEC> as RegisterValue<_>>::new(255)
    }
}
pub mod eacgsm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs0_SPEC;
    pub type Cgs0 = crate::EnumBitfieldStruct<u8, Cgs0_SPEC>;
    impl Cgs0 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs1_SPEC;
    pub type Cgs1 = crate::EnumBitfieldStruct<u8, Cgs1_SPEC>;
    impl Cgs1 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs2_SPEC;
    pub type Cgs2 = crate::EnumBitfieldStruct<u8, Cgs2_SPEC>;
    impl Cgs2 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs3_SPEC;
    pub type Cgs3 = crate::EnumBitfieldStruct<u8, Cgs3_SPEC>;
    impl Cgs3 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs4_SPEC;
    pub type Cgs4 = crate::EnumBitfieldStruct<u8, Cgs4_SPEC>;
    impl Cgs4 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs5_SPEC;
    pub type Cgs5 = crate::EnumBitfieldStruct<u8, Cgs5_SPEC>;
    impl Cgs5 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs6_SPEC;
    pub type Cgs6 = crate::EnumBitfieldStruct<u8, Cgs6_SPEC>;
    impl Cgs6 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cgs7_SPEC;
    pub type Cgs7 = crate::EnumBitfieldStruct<u8, Cgs7_SPEC>;
    impl Cgs7 {
        #[doc = "CBS does not authorize queue i transmission (CBS credit negative)"]
        pub const _0: Self = Self::new(0);

        #[doc = "CBS authorizes queue i transmission (CBS credit positive)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasc_SPEC;
impl crate::sealed::RegSpec for Eatasc_SPEC {
    type DataType = u32;
}

#[doc = "TAS Configuration Register"]
pub type Eatasc = crate::RegValueT<Eatasc_SPEC>;

impl Eatasc {
    #[doc = "TAS Enable"]
    #[inline(always)]
    pub fn tase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eatasc::Tase,
        eatasc::Tase,
        Eatasc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eatasc::Tase,
            eatasc::Tase,
            Eatasc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Configuration Change"]
    #[inline(always)]
    pub fn tascc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eatasc::Tascc,
        eatasc::Tascc,
        Eatasc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eatasc::Tascc,
            eatasc::Tascc,
            Eatasc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Configuration Impossible"]
    #[inline(always)]
    pub fn tasci(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eatasc::Tasci,
        eatasc::Tasci,
        Eatasc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eatasc::Tasci,
            eatasc::Tasci,
            Eatasc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Timer Select"]
    #[inline(always)]
    pub fn tasts(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Eatasc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eatasc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TAS Configuration Address"]
    #[inline(always)]
    pub fn tasca(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Eatasc_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Eatasc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasc {
    #[inline(always)]
    fn default() -> Eatasc {
        <crate::RegValueT<Eatasc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eatasc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tase_SPEC;
    pub type Tase = crate::EnumBitfieldStruct<u8, Tase_SPEC>;
    impl Tase {
        #[doc = "TAS disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "TAS enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tascc_SPEC;
    pub type Tascc = crate::EnumBitfieldStruct<u8, Tascc_SPEC>;
    impl Tascc {
        #[doc = "TAS is not changing configuration"]
        pub const _0: Self = Self::new(0);

        #[doc = "TAS is changing configuration"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasci_SPEC;
    pub type Tasci = crate::EnumBitfieldStruct<u8, Tasci_SPEC>;
    impl Tasci {
        #[doc = "TAS configuration is possible"]
        pub const _0: Self = Self::new(0);

        #[doc = "TAS configuration is not possible"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasigsc_SPEC;
impl crate::sealed::RegSpec for Eatasigsc_SPEC {
    type DataType = u32;
}

#[doc = "TAS Initial Gate State Configuration Register"]
pub type Eatasigsc = crate::RegValueT<Eatasigsc_SPEC>;

impl Eatasigsc {
    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eatasigsc::Tasigs0,
        eatasigsc::Tasigs0,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eatasigsc::Tasigs0,
            eatasigsc::Tasigs0,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eatasigsc::Tasigs1,
        eatasigsc::Tasigs1,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eatasigsc::Tasigs1,
            eatasigsc::Tasigs1,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eatasigsc::Tasigs2,
        eatasigsc::Tasigs2,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eatasigsc::Tasigs2,
            eatasigsc::Tasigs2,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eatasigsc::Tasigs3,
        eatasigsc::Tasigs3,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eatasigsc::Tasigs3,
            eatasigsc::Tasigs3,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eatasigsc::Tasigs4,
        eatasigsc::Tasigs4,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eatasigsc::Tasigs4,
            eatasigsc::Tasigs4,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eatasigsc::Tasigs5,
        eatasigsc::Tasigs5,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eatasigsc::Tasigs5,
            eatasigsc::Tasigs5,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eatasigsc::Tasigs6,
        eatasigsc::Tasigs6,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eatasigsc::Tasigs6,
            eatasigsc::Tasigs6,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Initial Gate State n (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasigs7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eatasigsc::Tasigs7,
        eatasigsc::Tasigs7,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eatasigsc::Tasigs7,
            eatasigsc::Tasigs7,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Cut-Through Initial Gate State"]
    #[inline(always)]
    pub fn tasctigs(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eatasigsc::Tasctigs,
        eatasigsc::Tasctigs,
        Eatasigsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eatasigsc::Tasctigs,
            eatasigsc::Tasctigs,
            Eatasigsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatasigsc {
    #[inline(always)]
    fn default() -> Eatasigsc {
        <crate::RegValueT<Eatasigsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eatasigsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs0_SPEC;
    pub type Tasigs0 = crate::EnumBitfieldStruct<u8, Tasigs0_SPEC>;
    impl Tasigs0 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs1_SPEC;
    pub type Tasigs1 = crate::EnumBitfieldStruct<u8, Tasigs1_SPEC>;
    impl Tasigs1 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs2_SPEC;
    pub type Tasigs2 = crate::EnumBitfieldStruct<u8, Tasigs2_SPEC>;
    impl Tasigs2 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs3_SPEC;
    pub type Tasigs3 = crate::EnumBitfieldStruct<u8, Tasigs3_SPEC>;
    impl Tasigs3 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs4_SPEC;
    pub type Tasigs4 = crate::EnumBitfieldStruct<u8, Tasigs4_SPEC>;
    impl Tasigs4 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs5_SPEC;
    pub type Tasigs5 = crate::EnumBitfieldStruct<u8, Tasigs5_SPEC>;
    impl Tasigs5 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs6_SPEC;
    pub type Tasigs6 = crate::EnumBitfieldStruct<u8, Tasigs6_SPEC>;
    impl Tasigs6 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasigs7_SPEC;
    pub type Tasigs7 = crate::EnumBitfieldStruct<u8, Tasigs7_SPEC>;
    impl Tasigs7 {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasctigs_SPEC;
    pub type Tasctigs = crate::EnumBitfieldStruct<u8, Tasctigs_SPEC>;
    impl Tasctigs {
        #[doc = "Initial gate state is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initial gate state is opened"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasenc_SPEC;
impl crate::sealed::RegSpec for Eatasenc_SPEC {
    type DataType = u32;
}

#[doc = "TAS Entry Number Configuration Register %s"]
pub type Eatasenc = crate::RegValueT<Eatasenc_SPEC>;

impl Eatasenc {
    #[doc = "TAS Admin Entry Number"]
    #[inline(always)]
    pub fn tasaen(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Eatasenc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Eatasenc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasenc {
    #[inline(always)]
    fn default() -> Eatasenc {
        <crate::RegValueT<Eatasenc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasctenc_SPEC;
impl crate::sealed::RegSpec for Eatasctenc_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cut-Through Entry Number Configuration Register"]
pub type Eatasctenc = crate::RegValueT<Eatasctenc_SPEC>;

impl Eatasctenc {
    #[doc = "TAS Admin Cut-Through Entry Number"]
    #[inline(always)]
    pub fn tasctaen(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Eatasctenc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Eatasctenc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasctenc {
    #[inline(always)]
    fn default() -> Eatasctenc {
        <crate::RegValueT<Eatasctenc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasenm_SPEC;
impl crate::sealed::RegSpec for Eatasenm_SPEC {
    type DataType = u32;
}

#[doc = "TAS Entry Number Monitoring Register %s"]
pub type Eatasenm = crate::RegValueT<Eatasenm_SPEC>;

impl Eatasenm {
    #[doc = "TAS Oper Entry Number"]
    #[inline(always)]
    pub fn tasoen(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Eatasenm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Eatasenm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasenm {
    #[inline(always)]
    fn default() -> Eatasenm {
        <crate::RegValueT<Eatasenm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasctenm_SPEC;
impl crate::sealed::RegSpec for Eatasctenm_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cut-Through Entry Number Monitoring Register"]
pub type Eatasctenm = crate::RegValueT<Eatasctenm_SPEC>;

impl Eatasctenm {
    #[doc = "TAS Cut-Through Oper Entry Number"]
    #[inline(always)]
    pub fn tasctoen(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Eatasctenm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Eatasctenm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasctenm {
    #[inline(always)]
    fn default() -> Eatasctenm {
        <crate::RegValueT<Eatasctenm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatascstc0_SPEC;
impl crate::sealed::RegSpec for Eatascstc0_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cycle Start Time Configuration Register 0"]
pub type Eatascstc0 = crate::RegValueT<Eatascstc0_SPEC>;

impl Eatascstc0 {
    #[doc = "TAS Admin Cycle Start Time Part 0"]
    #[inline(always)]
    pub fn tasacstp0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Eatascstc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Eatascstc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatascstc0 {
    #[inline(always)]
    fn default() -> Eatascstc0 {
        <crate::RegValueT<Eatascstc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatascstc1_SPEC;
impl crate::sealed::RegSpec for Eatascstc1_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cycle Start Time Configuration Register 1"]
pub type Eatascstc1 = crate::RegValueT<Eatascstc1_SPEC>;

impl Eatascstc1 {
    #[doc = "TAS Admin Cycle Start Time Part 1"]
    #[inline(always)]
    pub fn tasacstp1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Eatascstc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Eatascstc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatascstc1 {
    #[inline(always)]
    fn default() -> Eatascstc1 {
        <crate::RegValueT<Eatascstc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatascstm0_SPEC;
impl crate::sealed::RegSpec for Eatascstm0_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cycle Start Time Monitoring Register 0"]
pub type Eatascstm0 = crate::RegValueT<Eatascstm0_SPEC>;

impl Eatascstm0 {
    #[doc = "TAS Oper Cycle Start Time Part 0"]
    #[inline(always)]
    pub fn tasocstp0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Eatascstm0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Eatascstm0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatascstm0 {
    #[inline(always)]
    fn default() -> Eatascstm0 {
        <crate::RegValueT<Eatascstm0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatascstm1_SPEC;
impl crate::sealed::RegSpec for Eatascstm1_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cycle Start Time Monitoring Register 1"]
pub type Eatascstm1 = crate::RegValueT<Eatascstm1_SPEC>;

impl Eatascstm1 {
    #[doc = "TAS Oper Cycle Start Time Part 1"]
    #[inline(always)]
    pub fn tasocstp1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Eatascstm1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Eatascstm1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatascstm1 {
    #[inline(always)]
    fn default() -> Eatascstm1 {
        <crate::RegValueT<Eatascstm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasctc_SPEC;
impl crate::sealed::RegSpec for Eatasctc_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cycle Time Configuration Register"]
pub type Eatasctc = crate::RegValueT<Eatasctc_SPEC>;

impl Eatasctc {
    #[doc = "TAS Admin Cycle Time"]
    #[inline(always)]
    pub fn tasact(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Eatasctc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Eatasctc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatasctc {
    #[inline(always)]
    fn default() -> Eatasctc {
        <crate::RegValueT<Eatasctc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasctm_SPEC;
impl crate::sealed::RegSpec for Eatasctm_SPEC {
    type DataType = u32;
}

#[doc = "TAS Cycle Time Monitoring Register"]
pub type Eatasctm = crate::RegValueT<Eatasctm_SPEC>;

impl Eatasctm {
    #[doc = "TAS Oper Cycle Time"]
    #[inline(always)]
    pub fn tasoct(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Eatasctm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Eatasctm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasctm {
    #[inline(always)]
    fn default() -> Eatasctm {
        <crate::RegValueT<Eatasctm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasgl0_SPEC;
impl crate::sealed::RegSpec for Eatasgl0_SPEC {
    type DataType = u32;
}

#[doc = "TAS Gate Learn Register 0"]
pub type Eatasgl0 = crate::RegValueT<Eatasgl0_SPEC>;

impl Eatasgl0 {
    #[doc = "TAS Gate Address Learn"]
    #[inline(always)]
    pub fn tasgal(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Eatasgl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Eatasgl0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasgl0 {
    #[inline(always)]
    fn default() -> Eatasgl0 {
        <crate::RegValueT<Eatasgl0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasgl1_SPEC;
impl crate::sealed::RegSpec for Eatasgl1_SPEC {
    type DataType = u32;
}

#[doc = "TAS Gate Learn Register 1"]
pub type Eatasgl1 = crate::RegValueT<Eatasgl1_SPEC>;

impl Eatasgl1 {
    #[doc = "TAS Gate Time Learn"]
    #[inline(always)]
    pub fn tasgtl(
        self,
    ) -> crate::common::RegisterField<0, 0xfffffff, 1, 0, u32, u32, Eatasgl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffffff,1,0,u32,u32,Eatasgl1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TAS Gate State Learn"]
    #[inline(always)]
    pub fn tasgsl(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Eatasgl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Eatasgl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasgl1 {
    #[inline(always)]
    fn default() -> Eatasgl1 {
        <crate::RegValueT<Eatasgl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasglr_SPEC;
impl crate::sealed::RegSpec for Eatasglr_SPEC {
    type DataType = u32;
}

#[doc = "TAS Gate Learn Result Register"]
pub type Eatasglr = crate::RegValueT<Eatasglr_SPEC>;

impl Eatasglr {
    #[doc = "Gate Learn"]
    #[inline(always)]
    pub fn gl(self) -> crate::common::RegisterFieldBool<31, 1, 0, Eatasglr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Eatasglr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Eatasglr {
    #[inline(always)]
    fn default() -> Eatasglr {
        <crate::RegValueT<Eatasglr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasgr_SPEC;
impl crate::sealed::RegSpec for Eatasgr_SPEC {
    type DataType = u32;
}

#[doc = "TAS Gate Read Register"]
pub type Eatasgr = crate::RegValueT<Eatasgr_SPEC>;

impl Eatasgr {
    #[doc = "TAS Gate Address Read"]
    #[inline(always)]
    pub fn tasgar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Eatasgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Eatasgr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatasgr {
    #[inline(always)]
    fn default() -> Eatasgr {
        <crate::RegValueT<Eatasgr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasgrr_SPEC;
impl crate::sealed::RegSpec for Eatasgrr_SPEC {
    type DataType = u32;
}

#[doc = "TAS Gate Read Result Register"]
pub type Eatasgrr = crate::RegValueT<Eatasgrr_SPEC>;

impl Eatasgrr {
    #[doc = "TAS Gate Time Read"]
    #[inline(always)]
    pub fn tasgtr(
        self,
    ) -> crate::common::RegisterField<0, 0xfffffff, 1, 0, u32, u32, Eatasgrr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfffffff,1,0,u32,u32,Eatasgrr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "TAS Gate State Read"]
    #[inline(always)]
    pub fn tasgsr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Eatasgrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Eatasgrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Gate Read"]
    #[inline(always)]
    pub fn gr(self) -> crate::common::RegisterFieldBool<31, 1, 0, Eatasgrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Eatasgrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Eatasgrr {
    #[inline(always)]
    fn default() -> Eatasgrr {
        <crate::RegValueT<Eatasgrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatashcc_SPEC;
impl crate::sealed::RegSpec for Eatashcc_SPEC {
    type DataType = u32;
}

#[doc = "TAS Hardware Calibration Configuration Register"]
pub type Eatashcc = crate::RegValueT<Eatashcc_SPEC>;

impl Eatashcc {
    #[doc = "TAS Jitter"]
    #[inline(always)]
    pub fn tasj(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eatashcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eatashcc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatashcc {
    #[inline(always)]
    fn default() -> Eatashcc {
        <crate::RegValueT<Eatashcc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatasrirm_SPEC;
impl crate::sealed::RegSpec for Eatasrirm_SPEC {
    type DataType = u32;
}

#[doc = "TAS RAM Initialization Register Monitoring Register"]
pub type Eatasrirm = crate::RegValueT<Eatasrirm_SPEC>;

impl Eatasrirm {
    #[doc = "TAS RAM Initialization Ongoing"]
    #[inline(always)]
    pub fn tasriog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Eatasrirm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Eatasrirm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TAS RAM Ready"]
    #[inline(always)]
    pub fn tasrr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Eatasrirm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eatasrirm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Eatasrirm {
    #[inline(always)]
    fn default() -> Eatasrirm {
        <crate::RegValueT<Eatasrirm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatassm_SPEC;
impl crate::sealed::RegSpec for Eatassm_SPEC {
    type DataType = u32;
}

#[doc = "TAS Status Monitoring Register"]
pub type Eatassm = crate::RegValueT<Eatassm_SPEC>;

impl Eatassm {
    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eatassm::Tasgs0,
        eatassm::Tasgs0,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eatassm::Tasgs0,
            eatassm::Tasgs0,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eatassm::Tasgs1,
        eatassm::Tasgs1,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eatassm::Tasgs1,
            eatassm::Tasgs1,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eatassm::Tasgs2,
        eatassm::Tasgs2,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eatassm::Tasgs2,
            eatassm::Tasgs2,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eatassm::Tasgs3,
        eatassm::Tasgs3,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eatassm::Tasgs3,
            eatassm::Tasgs3,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eatassm::Tasgs4,
        eatassm::Tasgs4,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eatassm::Tasgs4,
            eatassm::Tasgs4,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eatassm::Tasgs5,
        eatassm::Tasgs5,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eatassm::Tasgs5,
            eatassm::Tasgs5,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eatassm::Tasgs6,
        eatassm::Tasgs6,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eatassm::Tasgs6,
            eatassm::Tasgs6,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n State (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgs7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eatassm::Tasgs7,
        eatassm::Tasgs7,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eatassm::Tasgs7,
            eatassm::Tasgs7,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Cut-Through Gate State"]
    #[inline(always)]
    pub fn tasctgs(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eatassm::Tasctgs,
        eatassm::Tasctgs,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eatassm::Tasctgs,
            eatassm::Tasctgs,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TAS Scheduler Ongoing"]
    #[inline(always)]
    pub fn tasso(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        eatassm::Tasso,
        eatassm::Tasso,
        Eatassm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            eatassm::Tasso,
            eatassm::Tasso,
            Eatassm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eatassm {
    #[inline(always)]
    fn default() -> Eatassm {
        <crate::RegValueT<Eatassm_SPEC> as RegisterValue<_>>::new(511)
    }
}
pub mod eatassm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs0_SPEC;
    pub type Tasgs0 = crate::EnumBitfieldStruct<u8, Tasgs0_SPEC>;
    impl Tasgs0 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs1_SPEC;
    pub type Tasgs1 = crate::EnumBitfieldStruct<u8, Tasgs1_SPEC>;
    impl Tasgs1 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs2_SPEC;
    pub type Tasgs2 = crate::EnumBitfieldStruct<u8, Tasgs2_SPEC>;
    impl Tasgs2 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs3_SPEC;
    pub type Tasgs3 = crate::EnumBitfieldStruct<u8, Tasgs3_SPEC>;
    impl Tasgs3 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs4_SPEC;
    pub type Tasgs4 = crate::EnumBitfieldStruct<u8, Tasgs4_SPEC>;
    impl Tasgs4 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs5_SPEC;
    pub type Tasgs5 = crate::EnumBitfieldStruct<u8, Tasgs5_SPEC>;
    impl Tasgs5 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs6_SPEC;
    pub type Tasgs6 = crate::EnumBitfieldStruct<u8, Tasgs6_SPEC>;
    impl Tasgs6 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgs7_SPEC;
    pub type Tasgs7 = crate::EnumBitfieldStruct<u8, Tasgs7_SPEC>;
    impl Tasgs7 {
        #[doc = "Gate i is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Gate i is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasctgs_SPEC;
    pub type Tasctgs = crate::EnumBitfieldStruct<u8, Tasctgs_SPEC>;
    impl Tasctgs {
        #[doc = "Cut-through gate is closed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cut-through gate is opened"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasso_SPEC;
    pub type Tasso = crate::EnumBitfieldStruct<u8, Tasso_SPEC>;
    impl Tasso {
        #[doc = "TAS scheduler is not ongoing"]
        pub const _0: Self = Self::new(0);

        #[doc = "TAS scheduler is ongoing"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eausmfsecn_SPEC;
impl crate::sealed::RegSpec for Eausmfsecn_SPEC {
    type DataType = u32;
}

#[doc = "Under Minimum Frame Size Error Counter Register"]
pub type Eausmfsecn = crate::RegValueT<Eausmfsecn_SPEC>;

impl Eausmfsecn {
    #[doc = "Under Minimum Frame Size Error Number"]
    #[inline(always)]
    pub fn usmfsen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eausmfsecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eausmfsecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eausmfsecn {
    #[inline(always)]
    fn default() -> Eausmfsecn {
        <crate::RegValueT<Eausmfsecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eatfecn_SPEC;
impl crate::sealed::RegSpec for Eatfecn_SPEC {
    type DataType = u32;
}

#[doc = "TAG Filtering Error Counter Register"]
pub type Eatfecn = crate::RegValueT<Eatfecn_SPEC>;

impl Eatfecn {
    #[doc = "TAG Filtering Error Number"]
    #[inline(always)]
    pub fn tfen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eatfecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eatfecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eatfecn {
    #[inline(always)]
    fn default() -> Eatfecn {
        <crate::RegValueT<Eatfecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eafsecn_SPEC;
impl crate::sealed::RegSpec for Eafsecn_SPEC {
    type DataType = u32;
}

#[doc = "Frame Size Error Counter Register"]
pub type Eafsecn = crate::RegValueT<Eafsecn_SPEC>;

impl Eafsecn {
    #[doc = "Frame Size Error Number"]
    #[inline(always)]
    pub fn fsen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eafsecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eafsecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eafsecn {
    #[inline(always)]
    fn default() -> Eafsecn {
        <crate::RegValueT<Eafsecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eadqoecn_SPEC;
impl crate::sealed::RegSpec for Eadqoecn_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Queue Overflow Error Counter Register"]
pub type Eadqoecn = crate::RegValueT<Eadqoecn_SPEC>;

impl Eadqoecn {
    #[doc = "Descriptor Queue Overflow Error Number"]
    #[inline(always)]
    pub fn dqoen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eadqoecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eadqoecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eadqoecn {
    #[inline(always)]
    fn default() -> Eadqoecn {
        <crate::RegValueT<Eadqoecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eadqsecn_SPEC;
impl crate::sealed::RegSpec for Eadqsecn_SPEC {
    type DataType = u32;
}

#[doc = "Descriptor Queue Security Error Counter Register"]
pub type Eadqsecn = crate::RegValueT<Eadqsecn_SPEC>;

impl Eadqsecn {
    #[doc = "Descriptor Queue Security Error Number"]
    #[inline(always)]
    pub fn dqsen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Eadqsecn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Eadqsecn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eadqsecn {
    #[inline(always)]
    fn default() -> Eadqsecn {
        <crate::RegValueT<Eadqsecn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeis0_SPEC;
impl crate::sealed::RegSpec for Eaeis0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 0"]
pub type Eaeis0 = crate::RegValueT<Eaeis0_SPEC>;

impl Eaeis0 {
    #[doc = "Under Switch Minimum Frame Size Error Status Flag"]
    #[inline(always)]
    pub fn usmfses(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Eaeis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eaeis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TAG Filtering Error Status Flag"]
    #[inline(always)]
    pub fn tfes(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eaeis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eaeis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Size Error Status Flag n (n = 0 to 7)"]
    #[inline(always)]
    pub fn fses15_to_fses8(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Eaeis0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Eaeis0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eaeis0 {
    #[inline(always)]
    fn default() -> Eaeis0 {
        <crate::RegValueT<Eaeis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeie0_SPEC;
impl crate::sealed::RegSpec for Eaeie0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 0"]
pub type Eaeie0 = crate::RegValueT<Eaeie0_SPEC>;

impl Eaeie0 {
    #[doc = "Under Switch Minimum Frame Size Error Enable"]
    #[inline(always)]
    pub fn usmfsee(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eaeie0::Usmfsee,
        eaeie0::Usmfsee,
        Eaeie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eaeie0::Usmfsee,
            eaeie0::Usmfsee,
            Eaeie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAG Filtering Error Enable"]
    #[inline(always)]
    pub fn tfee(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eaeie0::Tfee,
        eaeie0::Tfee,
        Eaeie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eaeie0::Tfee,
            eaeie0::Tfee,
            Eaeie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Size Error Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn fsee7_to_fsee0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        eaeie0::Fsee7ToFsee0,
        eaeie0::Fsee7ToFsee0,
        Eaeie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            eaeie0::Fsee7ToFsee0,
            eaeie0::Fsee7ToFsee0,
            Eaeie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eaeie0 {
    #[inline(always)]
    fn default() -> Eaeie0 {
        <crate::RegValueT<Eaeie0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eaeie0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usmfsee_SPEC;
    pub type Usmfsee = crate::EnumBitfieldStruct<u8, Usmfsee_SPEC>;
    impl Usmfsee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfee_SPEC;
    pub type Tfee = crate::EnumBitfieldStruct<u8, Tfee_SPEC>;
    impl Tfee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsee7ToFsee0_SPEC;
    pub type Fsee7ToFsee0 = crate::EnumBitfieldStruct<u8, Fsee7ToFsee0_SPEC>;
    impl Fsee7ToFsee0 {
        #[doc = "Interrupt disabled for descriptor queue i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for descriptor queue i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeid0_SPEC;
impl crate::sealed::RegSpec for Eaeid0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 0"]
pub type Eaeid0 = crate::RegValueT<Eaeid0_SPEC>;

impl Eaeid0 {
    #[doc = "Under Switch Minimum Frame Size Error Disable"]
    #[inline(always)]
    pub fn usmfsed(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Eaeid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eaeid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TAG Filtering Error Disable"]
    #[inline(always)]
    pub fn tfed(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eaeid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eaeid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Size Error Disable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn fsed7_to_fsed0(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Eaeid0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Eaeid0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eaeid0 {
    #[inline(always)]
    fn default() -> Eaeid0 {
        <crate::RegValueT<Eaeid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeis1_SPEC;
impl crate::sealed::RegSpec for Eaeis1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 1"]
pub type Eaeis1 = crate::RegValueT<Eaeis1_SPEC>;

impl Eaeis1 {
    #[doc = "CBS n Upper Limit Error Status Flag (n = 0 to 7)"]
    #[inline(always)]
    pub fn cules7_to_cules0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Eaeis1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Eaeis1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TAS Gate n Error Status Flag (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasges7_to_tasges0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Eaeis1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Eaeis1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TAS Cut-Through Gate Error Status Flag"]
    #[inline(always)]
    pub fn tasctges(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Eaeis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Eaeis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Eaeis1 {
    #[inline(always)]
    fn default() -> Eaeis1 {
        <crate::RegValueT<Eaeis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeie1_SPEC;
impl crate::sealed::RegSpec for Eaeie1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 1"]
pub type Eaeie1 = crate::RegValueT<Eaeie1_SPEC>;

impl Eaeie1 {
    #[doc = "CBS n Upper Limit Error Enable (n = 0 to 7)"]
    #[inline(always)]
    pub fn culee7_to_culee0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        eaeie1::Culee7ToCulee0,
        eaeie1::Culee7ToCulee0,
        Eaeie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            eaeie1::Culee7ToCulee0,
            eaeie1::Culee7ToCulee0,
            Eaeie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Gate n Error Enable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasgee7_to_tasgee0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        eaeie1::Tasgee7ToTasgee0,
        eaeie1::Tasgee7ToTasgee0,
        Eaeie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            eaeie1::Tasgee7ToTasgee0,
            eaeie1::Tasgee7ToTasgee0,
            Eaeie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "TAS Cut-through Gate Error Enable"]
    #[inline(always)]
    pub fn tasctgee(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        eaeie1::Tasctgee,
        eaeie1::Tasctgee,
        Eaeie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            eaeie1::Tasctgee,
            eaeie1::Tasctgee,
            Eaeie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eaeie1 {
    #[inline(always)]
    fn default() -> Eaeie1 {
        <crate::RegValueT<Eaeie1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eaeie1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Culee7ToCulee0_SPEC;
    pub type Culee7ToCulee0 = crate::EnumBitfieldStruct<u8, Culee7ToCulee0_SPEC>;
    impl Culee7ToCulee0 {
        #[doc = "Interrupt disabled for descriptor queue i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for descriptor queue i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasgee7ToTasgee0_SPEC;
    pub type Tasgee7ToTasgee0 = crate::EnumBitfieldStruct<u8, Tasgee7ToTasgee0_SPEC>;
    impl Tasgee7ToTasgee0 {
        #[doc = "Interrupt disabled for descriptor queue i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for descriptor queue i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tasctgee_SPEC;
    pub type Tasctgee = crate::EnumBitfieldStruct<u8, Tasctgee_SPEC>;
    impl Tasctgee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeid1_SPEC;
impl crate::sealed::RegSpec for Eaeid1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 1"]
pub type Eaeid1 = crate::RegValueT<Eaeid1_SPEC>;

impl Eaeid1 {
    #[doc = "CBS n Upper Limit Error Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn culed7_to_culed0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Eaeid1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Eaeid1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "TAS Gate n Error Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn tasged7_to_tasged0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Eaeid1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Eaeid1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "TAS Cut-through Gate Error Disable"]
    #[inline(always)]
    pub fn tasctged(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Eaeid1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Eaeid1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Eaeid1 {
    #[inline(always)]
    fn default() -> Eaeid1 {
        <crate::RegValueT<Eaeid1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeis2_SPEC;
impl crate::sealed::RegSpec for Eaeis2_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 2"]
pub type Eaeis2 = crate::RegValueT<Eaeis2_SPEC>;

impl Eaeis2 {
    #[doc = "Descriptor Queue n Overflow Error Status Flag (n = 0 to 7)"]
    #[inline(always)]
    pub fn dqoes7_to_dqoes0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Eaeis2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Eaeis2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through Descriptor Queue Overflow Error Status Flag"]
    #[inline(always)]
    pub fn ctdqoes(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Eaeis2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eaeis2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Queue n Security Error Status Flag (n = 0 to 7)"]
    #[inline(always)]
    pub fn dqses7_to_dqses0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Eaeis2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Eaeis2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eaeis2 {
    #[inline(always)]
    fn default() -> Eaeis2 {
        <crate::RegValueT<Eaeis2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeie2_SPEC;
impl crate::sealed::RegSpec for Eaeie2_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 2"]
pub type Eaeie2 = crate::RegValueT<Eaeie2_SPEC>;

impl Eaeie2 {
    #[doc = "Descriptor Queue n Overflow Error Enable (n = 0 to 7)"]
    #[inline(always)]
    pub fn dqoee7_to_dqoee0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        eaeie2::Dqoee7ToDqoee0,
        eaeie2::Dqoee7ToDqoee0,
        Eaeie2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            eaeie2::Dqoee7ToDqoee0,
            eaeie2::Dqoee7ToDqoee0,
            Eaeie2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through Descriptor Queue Overflow Error Enable"]
    #[inline(always)]
    pub fn ctdqoee(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eaeie2::Ctdqoee,
        eaeie2::Ctdqoee,
        Eaeie2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eaeie2::Ctdqoee,
            eaeie2::Ctdqoee,
            Eaeie2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Descriptor Queue n Security Error Enable (n = 0 to 7)"]
    #[inline(always)]
    pub fn dqsee7_to_dqsee0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        eaeie2::Dqsee7ToDqsee0,
        eaeie2::Dqsee7ToDqsee0,
        Eaeie2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            eaeie2::Dqsee7ToDqsee0,
            eaeie2::Dqsee7ToDqsee0,
            Eaeie2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eaeie2 {
    #[inline(always)]
    fn default() -> Eaeie2 {
        <crate::RegValueT<Eaeie2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eaeie2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqoee7ToDqoee0_SPEC;
    pub type Dqoee7ToDqoee0 = crate::EnumBitfieldStruct<u8, Dqoee7ToDqoee0_SPEC>;
    impl Dqoee7ToDqoee0 {
        #[doc = "Interrupt disabled for descriptor queue i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for descriptor queue i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctdqoee_SPEC;
    pub type Ctdqoee = crate::EnumBitfieldStruct<u8, Ctdqoee_SPEC>;
    impl Ctdqoee {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dqsee7ToDqsee0_SPEC;
    pub type Dqsee7ToDqsee0 = crate::EnumBitfieldStruct<u8, Dqsee7ToDqsee0_SPEC>;
    impl Dqsee7ToDqsee0 {
        #[doc = "Interrupt disabled for descriptor queue i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt enabled for descriptor queue i"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eaeid2_SPEC;
impl crate::sealed::RegSpec for Eaeid2_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 2"]
pub type Eaeid2 = crate::RegValueT<Eaeid2_SPEC>;

impl Eaeid2 {
    #[doc = "Descriptor Queue n Overflow Error Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn dqoed7_to_dqoed0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Eaeid2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Eaeid2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Cut-Through Descriptor Queue Overflow Error Disable"]
    #[inline(always)]
    pub fn ctdqoed(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Eaeid2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eaeid2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Descriptor Queue n Security Error Disable (n = 0 to 7)"]
    #[inline(always)]
    pub fn dqsed7_to_dqsed0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Eaeid2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Eaeid2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Eaeid2 {
    #[inline(always)]
    fn default() -> Eaeid2 {
        <crate::RegValueT<Eaeid2_SPEC> as RegisterValue<_>>::new(0)
    }
}
