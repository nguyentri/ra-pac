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
#[doc = r"Forwarding engine"]
unsafe impl ::core::marker::Send for super::Mfwd {}
unsafe impl ::core::marker::Sync for super::Mfwd {}
impl super::Mfwd {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "General Configuration Register"]
    #[inline(always)]
    pub const fn fwgc(&self) -> &'static crate::common::Reg<self::Fwgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "TAG TPID Configuration Register 0"]
    #[inline(always)]
    pub const fn fwttc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "TAG TPID Configuration Register 1"]
    #[inline(always)]
    pub const fn fwttc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "CPU Exceptional Path Target Configuration Register"]
    #[inline(always)]
    pub const fn fwceptc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwceptc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwceptc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "CPU Exceptional Path Reason Configuration Register 0"]
    #[inline(always)]
    pub const fn fwceprc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwceprc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwceprc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "CPU Exceptional Path Reason Configuration Register 1"]
    #[inline(always)]
    pub const fn fwceprc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwceprc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwceprc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "CPU Exceptional Path Reason Configuration Register 2"]
    #[inline(always)]
    pub const fn fwceprc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwceprc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwceprc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "CPU Learning Path Target Configuration Register"]
    #[inline(always)]
    pub const fn fwclptc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwclptc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwclptc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "CPU Learning Path Reason Configuration Register"]
    #[inline(always)]
    pub const fn fwclprc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwclprc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwclprc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "CPU Mirroring Path Target Configuration Register"]
    #[inline(always)]
    pub const fn fwcmptc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcmptc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcmptc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Ethernet Mirroring Path Target Configuration Register"]
    #[inline(always)]
    pub const fn fwemptc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwemptc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwemptc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Source-Destination Mirroring Path Target Configuration Register"]
    #[inline(always)]
    pub const fn fwsdmptc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwsdmptc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwsdmptc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Source-Destination Mirroring Path Vector Configuration Register"]
    #[inline(always)]
    pub const fn fwsdmpvc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwsdmpvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwsdmpvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Port %s Level Based Watermark Configuration Register"]
    #[inline(always)]
    pub const fn fwlbwmc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwlbwmc_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn fwlbwmc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlbwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlbwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwlbwmc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlbwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlbwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwlbwmc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlbwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlbwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }

    #[doc = "Port %s Configuration Register 0"]
    #[inline(always)]
    pub const fn fwpc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpc0_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn fwpc00(&self) -> &'static crate::common::Reg<self::Fwpc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpc10(&self) -> &'static crate::common::Reg<self::Fwpc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpc20(&self) -> &'static crate::common::Reg<self::Fwpc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }

    #[doc = "Port %s Configuration Register 1"]
    #[inline(always)]
    pub const fn fwpc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpc1_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x104usize))
        }
    }
    #[inline(always)]
    pub const fn fwpc01(&self) -> &'static crate::common::Reg<self::Fwpc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpc11(&self) -> &'static crate::common::Reg<self::Fwpc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpc21(&self) -> &'static crate::common::Reg<self::Fwpc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }

    #[doc = "Port %s Configuration Register 2"]
    #[inline(always)]
    pub const fn fwpc2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpc2_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x108usize))
        }
    }
    #[inline(always)]
    pub const fn fwpc02(&self) -> &'static crate::common::Reg<self::Fwpc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpc12(&self) -> &'static crate::common::Reg<self::Fwpc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpc22(&self) -> &'static crate::common::Reg<self::Fwpc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }

    #[doc = "Cut-Through General Configuration Register i0"]
    #[inline(always)]
    pub const fn fwctgc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }
    #[inline(always)]
    pub const fn fwctgc00(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x480usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x500usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc50(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x540usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc60(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x580usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc70(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c0usize),
            )
        }
    }

    #[doc = "Cut-Through General Configuration Register i1"]
    #[inline(always)]
    pub const fn fwctgc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x404usize))
        }
    }
    #[inline(always)]
    pub const fn fwctgc01(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x484usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc41(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x504usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc51(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x544usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc61(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x584usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctgc71(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctgc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctgc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c4usize),
            )
        }
    }

    #[doc = "Cut-Through Target Configuration Register i0"]
    #[inline(always)]
    pub const fn fwcttc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x408usize))
        }
    }
    #[inline(always)]
    pub const fn fwcttc00(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x488usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc50(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x548usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc60(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x588usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc70(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5c8usize),
            )
        }
    }

    #[doc = "Cut-Through Target Configuration Register i1"]
    #[inline(always)]
    pub const fn fwcttc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40cusize))
        }
    }
    #[inline(always)]
    pub const fn fwcttc01(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc41(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc51(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc61(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc71(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ccusize),
            )
        }
    }

    #[doc = "Cut-Through Target Configuration Register i20"]
    #[inline(always)]
    pub const fn fwcttc20(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x410usize))
        }
    }
    #[inline(always)]
    pub const fn fwcttc020(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc120(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc220(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x490usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc320(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc420(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc520(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x550usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc620(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x590usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcttc720(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcttc20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcttc20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5d0usize),
            )
        }
    }

    #[doc = "Cut-Through Separation Configuration Register i0"]
    #[inline(always)]
    pub const fn fwctsc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x420usize))
        }
    }
    #[inline(always)]
    pub const fn fwctsc00(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x520usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc50(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x560usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc60(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc70(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e0usize),
            )
        }
    }

    #[doc = "Cut-Through Separation Configuration Register i1"]
    #[inline(always)]
    pub const fn fwctsc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x424usize))
        }
    }
    #[inline(always)]
    pub const fn fwctsc01(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc41(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x524usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc51(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x564usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc61(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc71(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e4usize),
            )
        }
    }

    #[doc = "Cut-Through Separation Configuration Register i2"]
    #[inline(always)]
    pub const fn fwctsc2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x428usize))
        }
    }
    #[inline(always)]
    pub const fn fwctsc02(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc32(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc42(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x528usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc52(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x568usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc62(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc72(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5e8usize),
            )
        }
    }

    #[doc = "Cut-Through Separation Configuration Register i3"]
    #[inline(always)]
    pub const fn fwctsc3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x42cusize))
        }
    }
    #[inline(always)]
    pub const fn fwctsc03(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x46cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc33(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc43(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc53(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc63(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc73(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5ecusize),
            )
        }
    }

    #[doc = "Cut-Through Separation Configuration Register i4"]
    #[inline(always)]
    pub const fn fwctsc4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW>,
        8,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x430usize))
        }
    }
    #[inline(always)]
    pub const fn fwctsc04(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x470usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc34(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc44(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x530usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc54(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x570usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc64(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctsc74(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctsc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwctsc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5f0usize),
            )
        }
    }

    #[doc = "Two-Byte Filter Configuration Register %s"]
    #[inline(always)]
    pub const fn fwtwbfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1000usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1010usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1020usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1030usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1040usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1050usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1060usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1070usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1080usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1090usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10f0usize),
            )
        }
    }

    #[doc = "Two-Byte Filter Value Configuration Register %s"]
    #[inline(always)]
    pub const fn fwtwbfvc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1014usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1024usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1044usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1054usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1074usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1084usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwtwbfvc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwtwbfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwtwbfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10f4usize),
            )
        }
    }

    #[doc = "Three-Byte Filter Configuration Register %s"]
    #[inline(always)]
    pub const fn fwthbfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1400usize))
        }
    }
    #[inline(always)]
    pub const fn fwthbfc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1430usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1440usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1450usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1460usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1470usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1480usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1490usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14f0usize),
            )
        }
    }

    #[doc = "Three-Byte Filter Value 0 Configuration Register %s"]
    #[inline(always)]
    pub const fn fwthbfv0c(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1404usize))
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1404usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1414usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1424usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1434usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1444usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1454usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1464usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1474usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1484usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1494usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv0c15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14f4usize),
            )
        }
    }

    #[doc = "Three-Byte Filter Value 1 Configuration Register %s"]
    #[inline(always)]
    pub const fn fwthbfv1c(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1408usize))
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1408usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1428usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1438usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1448usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1458usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1468usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1478usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1488usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1498usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwthbfv1c15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwthbfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwthbfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14f8usize),
            )
        }
    }

    #[doc = "Four-Byte Filter Configuration Register %s"]
    #[inline(always)]
    pub const fn fwfobfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1800usize))
        }
    }
    #[inline(always)]
    pub const fn fwfobfc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1810usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1820usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1830usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1850usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1860usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1870usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1880usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1890usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18f0usize),
            )
        }
    }

    #[doc = "Four-Byte Filter Value 0 Configuration Register %s"]
    #[inline(always)]
    pub const fn fwfobfv0c(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1804usize))
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1874usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1884usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1894usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv0c15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv0C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv0C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18f4usize),
            )
        }
    }

    #[doc = "Four-Byte Filter Value 1 Configuration Register %s"]
    #[inline(always)]
    pub const fn fwfobfv1c(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1808usize))
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1818usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1828usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1838usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1848usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1858usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1868usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1878usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1888usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1898usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfobfv1c15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfobfv1C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwfobfv1C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18f8usize),
            )
        }
    }

    #[doc = "Range Filter Configuration Register %s"]
    #[inline(always)]
    pub const fn fwrfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c00usize))
        }
    }
    #[inline(always)]
    pub const fn fwrfc0(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc1(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc2(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc3(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc4(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc5(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc6(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc7(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc8(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc9(&self) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ca0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ce0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cf0usize),
            )
        }
    }

    #[doc = "Range Filter Value Configuration Register %s"]
    #[inline(always)]
    pub const fn fwrfvc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c04usize))
        }
    }
    #[inline(always)]
    pub const fn fwrfvc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ca4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ce4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwrfvc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwrfvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwrfvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cf4usize),
            )
        }
    }

    #[doc = "Cascade Filter Configuration Register %s"]
    #[inline(always)]
    pub const fn fwcfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2000usize))
        }
    }
    #[inline(always)]
    pub const fn fwcfc0(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2000usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc1(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2040usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc2(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2080usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc3(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc4(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc5(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc6(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc7(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc8(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc9(&self) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2380usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23c0usize),
            )
        }
    }

    #[doc = "Cascade Filter Mapping Configuration Register i0"]
    #[inline(always)]
    pub const fn fwcfmc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2004usize))
        }
    }
    #[inline(always)]
    pub const fn fwcfmc00(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2044usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2084usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc50(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc60(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc70(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc80(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc90(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc100(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc110(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc120(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc130(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc140(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2384usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc150(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23c4usize),
            )
        }
    }

    #[doc = "Cascade Filter Mapping Configuration Register i1"]
    #[inline(always)]
    pub const fn fwcfmc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2008usize))
        }
    }
    #[inline(always)]
    pub const fn fwcfmc01(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2008usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2048usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2088usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc41(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc51(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc61(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc71(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc81(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc91(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc101(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc111(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc121(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc131(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc141(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2388usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc151(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23c8usize),
            )
        }
    }

    #[doc = "Cascade Filter Mapping Configuration Register i2"]
    #[inline(always)]
    pub const fn fwcfmc2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200cusize))
        }
    }
    #[inline(always)]
    pub const fn fwcfmc02(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc32(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc42(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc52(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc62(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc72(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc82(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc92(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc102(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc112(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc122(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc132(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc142(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc152(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23ccusize),
            )
        }
    }

    #[doc = "Cascade Filter Mapping Configuration Register i3"]
    #[inline(always)]
    pub const fn fwcfmc3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2010usize))
        }
    }
    #[inline(always)]
    pub const fn fwcfmc03(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2010usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2050usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2090usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc33(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc43(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc53(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc63(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2190usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc73(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc83(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc93(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc103(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc113(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc123(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc133(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2350usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc143(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2390usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc153(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23d0usize),
            )
        }
    }

    #[doc = "Cascade Filter Mapping Configuration Register i4"]
    #[inline(always)]
    pub const fn fwcfmc4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2014usize))
        }
    }
    #[inline(always)]
    pub const fn fwcfmc04(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2014usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2054usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2094usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc34(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc44(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc54(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc64(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2194usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc74(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc84(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc94(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc104(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc114(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc124(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc134(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2354usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc144(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2394usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc154(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23d4usize),
            )
        }
    }

    #[doc = "Cascade Filter Mapping Configuration Register i5"]
    #[inline(always)]
    pub const fn fwcfmc5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2018usize))
        }
    }
    #[inline(always)]
    pub const fn fwcfmc05(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2018usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2058usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2098usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc35(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc45(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc55(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2158usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc65(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2198usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc75(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc85(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc95(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc105(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc115(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc125(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc135(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2358usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc145(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2398usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc155(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23d8usize),
            )
        }
    }

    #[doc = "Cascade Filter Mapping Configuration Register i6"]
    #[inline(always)]
    pub const fn fwcfmc6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW>,
        16,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x201cusize))
        }
    }
    #[inline(always)]
    pub const fn fwcfmc06(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x201cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x205cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x209cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc36(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc46(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x211cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc56(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x215cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc66(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x219cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc76(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x21dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc86(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x221cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc96(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x225cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc106(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x229cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc116(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc126(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x231cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc136(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x235cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc146(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x239cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwcfmc156(
        &self,
    ) -> &'static crate::common::Reg<self::Fwcfmc6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwcfmc6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x23dcusize),
            )
        }
    }

    #[doc = "IPv4 Stream Configuration Register"]
    #[inline(always)]
    pub const fn fwip4sc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwip4Sc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwip4Sc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16392usize),
            )
        }
    }

    #[doc = "IPv6 Stream Configuration Register"]
    #[inline(always)]
    pub const fn fwip6sc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwip6Sc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwip6Sc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16408usize),
            )
        }
    }

    #[doc = "IPv6 Offset Configuration Register"]
    #[inline(always)]
    pub const fn fwip6oc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwip6Oc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwip6Oc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16412usize),
            )
        }
    }

    #[doc = "Layer 2 Stream Configuration Register"]
    #[inline(always)]
    pub const fn fwl2sc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl2Sc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl2Sc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16416usize),
            )
        }
    }

    #[doc = "Stream Filter Hash Equation Configuration Register"]
    #[inline(always)]
    pub const fn fwsfhec(
        &self,
    ) -> &'static crate::common::Reg<self::Fwsfhec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwsfhec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16432usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 0"]
    #[inline(always)]
    pub const fn fwshcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16448usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 1"]
    #[inline(always)]
    pub const fn fwshcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16452usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 2"]
    #[inline(always)]
    pub const fn fwshcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16456usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 3"]
    #[inline(always)]
    pub const fn fwshcr3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16460usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 4"]
    #[inline(always)]
    pub const fn fwshcr4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16464usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 5"]
    #[inline(always)]
    pub const fn fwshcr5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16468usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 6"]
    #[inline(always)]
    pub const fn fwshcr6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16472usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 7"]
    #[inline(always)]
    pub const fn fwshcr7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16476usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 8"]
    #[inline(always)]
    pub const fn fwshcr8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16480usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 9"]
    #[inline(always)]
    pub const fn fwshcr9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16484usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 10"]
    #[inline(always)]
    pub const fn fwshcr10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16488usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 11"]
    #[inline(always)]
    pub const fn fwshcr11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16492usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 12"]
    #[inline(always)]
    pub const fn fwshcr12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16496usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Register 13"]
    #[inline(always)]
    pub const fn fwshcr13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcr13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwshcr13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16500usize),
            )
        }
    }

    #[doc = "Software Hash Calculation Request Result Register"]
    #[inline(always)]
    pub const fn fwshcrr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwshcrr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwshcrr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16504usize),
            )
        }
    }

    #[doc = "L3 Hash Entry Configuration Register"]
    #[inline(always)]
    pub const fn fwlthhec(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthhec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthhec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16528usize),
            )
        }
    }

    #[doc = "L3 Hash Configuration Register"]
    #[inline(always)]
    pub const fn fwlthhc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthhc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthhc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16532usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 0"]
    #[inline(always)]
    pub const fn fwlthtl0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16544usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 1"]
    #[inline(always)]
    pub const fn fwlthtl1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16548usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 2"]
    #[inline(always)]
    pub const fn fwlthtl2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16552usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 3"]
    #[inline(always)]
    pub const fn fwlthtl3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16556usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 4"]
    #[inline(always)]
    pub const fn fwlthtl4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16560usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 5"]
    #[inline(always)]
    pub const fn fwlthtl5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16564usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 6"]
    #[inline(always)]
    pub const fn fwlthtl6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16568usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 7"]
    #[inline(always)]
    pub const fn fwlthtl7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16572usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 80"]
    #[inline(always)]
    pub const fn fwlthtl80(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl80_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl80_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16576usize),
            )
        }
    }

    #[doc = "L3 Table Learn Register 9"]
    #[inline(always)]
    pub const fn fwlthtl9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtl9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtl9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16592usize),
            )
        }
    }

    #[doc = "L3 Table Learn Result Register"]
    #[inline(always)]
    pub const fn fwlthtlr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtlr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtlr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16596usize),
            )
        }
    }

    #[doc = "L3 Table Initialization Monitoring Register"]
    #[inline(always)]
    pub const fn fwlthtim(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16608usize),
            )
        }
    }

    #[doc = "L3 Table Entry Monitoring Register"]
    #[inline(always)]
    pub const fn fwlthtem(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtem_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtem_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16612usize),
            )
        }
    }

    #[doc = "L3 Table Search Register 0"]
    #[inline(always)]
    pub const fn fwlthts0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthts0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthts0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16640usize),
            )
        }
    }

    #[doc = "L3 Table Search Register 1"]
    #[inline(always)]
    pub const fn fwlthts1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthts1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthts1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16644usize),
            )
        }
    }

    #[doc = "L3 Table Search Register 2"]
    #[inline(always)]
    pub const fn fwlthts2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthts2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthts2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16648usize),
            )
        }
    }

    #[doc = "L3 Table Search Register 3"]
    #[inline(always)]
    pub const fn fwlthts3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthts3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthts3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16652usize),
            )
        }
    }

    #[doc = "L3 Table Search Register 4"]
    #[inline(always)]
    pub const fn fwlthts4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthts4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthts4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16656usize),
            )
        }
    }

    #[doc = "L3 Table Search Result Register 0"]
    #[inline(always)]
    pub const fn fwlthtsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16672usize),
            )
        }
    }

    #[doc = "L3 Table Search Result Register 1"]
    #[inline(always)]
    pub const fn fwlthtsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtsr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtsr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16676usize),
            )
        }
    }

    #[doc = "L3 Table Search Result Register 2"]
    #[inline(always)]
    pub const fn fwlthtsr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtsr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtsr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16680usize),
            )
        }
    }

    #[doc = "L3 Table Search Result Register 3"]
    #[inline(always)]
    pub const fn fwlthtsr3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtsr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtsr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16684usize),
            )
        }
    }

    #[doc = "L3 Table Search Result Register 40"]
    #[inline(always)]
    pub const fn fwlthtsr40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtsr40_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtsr40_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16688usize),
            )
        }
    }

    #[doc = "L3 Table Search Result Register 5"]
    #[inline(always)]
    pub const fn fwlthtsr5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtsr5_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtsr5_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16704usize),
            )
        }
    }

    #[doc = "L3 Table Read Register"]
    #[inline(always)]
    pub const fn fwlthtr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwlthtr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16720usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 0"]
    #[inline(always)]
    pub const fn fwlthtrr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16724usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 1"]
    #[inline(always)]
    pub const fn fwlthtrr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16728usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 2"]
    #[inline(always)]
    pub const fn fwlthtrr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16732usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 3"]
    #[inline(always)]
    pub const fn fwlthtrr3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16736usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 4"]
    #[inline(always)]
    pub const fn fwlthtrr4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr4_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr4_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16740usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 5"]
    #[inline(always)]
    pub const fn fwlthtrr5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr5_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr5_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16744usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 6"]
    #[inline(always)]
    pub const fn fwlthtrr6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr6_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr6_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16748usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 7"]
    #[inline(always)]
    pub const fn fwlthtrr7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr7_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr7_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16752usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 8"]
    #[inline(always)]
    pub const fn fwlthtrr8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr8_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr8_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16756usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 90"]
    #[inline(always)]
    pub const fn fwlthtrr90(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr90_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr90_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16768usize),
            )
        }
    }

    #[doc = "L3 Table Read Result Register 10"]
    #[inline(always)]
    pub const fn fwlthtrr10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthtrr10_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthtrr10_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16784usize),
            )
        }
    }

    #[doc = "MAC Hash Entry Configuration Register"]
    #[inline(always)]
    pub const fn fwmachec(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmachec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmachec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17952usize),
            )
        }
    }

    #[doc = "MAC Hash Configuration Register"]
    #[inline(always)]
    pub const fn fwmachc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmachc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmachc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17956usize),
            )
        }
    }

    #[doc = "MAC Table Learn Register 0"]
    #[inline(always)]
    pub const fn fwmactl0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17968usize),
            )
        }
    }

    #[doc = "MAC Table Learn Register 1"]
    #[inline(always)]
    pub const fn fwmactl1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17972usize),
            )
        }
    }

    #[doc = "MAC Table Learn Register 2"]
    #[inline(always)]
    pub const fn fwmactl2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactl2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactl2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17976usize),
            )
        }
    }

    #[doc = "MAC Table Learn Register 3"]
    #[inline(always)]
    pub const fn fwmactl3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactl3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactl3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17980usize),
            )
        }
    }

    #[doc = "MAC Table Learn Register 40"]
    #[inline(always)]
    pub const fn fwmactl40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactl40_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactl40_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17984usize),
            )
        }
    }

    #[doc = "MAC Table Learn Register 5"]
    #[inline(always)]
    pub const fn fwmactl5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactl5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactl5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18000usize),
            )
        }
    }

    #[doc = "MAC Table Learn Result Register"]
    #[inline(always)]
    pub const fn fwmactlr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactlr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactlr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18004usize),
            )
        }
    }

    #[doc = "MAC Table Initialization Monitoring Register"]
    #[inline(always)]
    pub const fn fwmactim(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18016usize),
            )
        }
    }

    #[doc = "MAC Table Entry Monitoring Register"]
    #[inline(always)]
    pub const fn fwmactem(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactem_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactem_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18020usize),
            )
        }
    }

    #[doc = "MAC Table Search Register 0"]
    #[inline(always)]
    pub const fn fwmacts0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmacts0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmacts0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18032usize),
            )
        }
    }

    #[doc = "MAC Table Search Register 1"]
    #[inline(always)]
    pub const fn fwmacts1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmacts1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmacts1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18036usize),
            )
        }
    }

    #[doc = "MAC Table Search Result Register 0"]
    #[inline(always)]
    pub const fn fwmactsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18040usize),
            )
        }
    }

    #[doc = "MAC Table Search Result Register 1"]
    #[inline(always)]
    pub const fn fwmactsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactsr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactsr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18044usize),
            )
        }
    }

    #[doc = "MAC Table Search Result Register 20"]
    #[inline(always)]
    pub const fn fwmactsr20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactsr20_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactsr20_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18048usize),
            )
        }
    }

    #[doc = "MAC Table Search Result Register 3"]
    #[inline(always)]
    pub const fn fwmactsr3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactsr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactsr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18064usize),
            )
        }
    }

    #[doc = "MAC Table Read Register"]
    #[inline(always)]
    pub const fn fwmactr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmactr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18080usize),
            )
        }
    }

    #[doc = "MAC Table Read Result Register 0"]
    #[inline(always)]
    pub const fn fwmactrr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactrr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactrr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18084usize),
            )
        }
    }

    #[doc = "MAC Table Read Result Register 1"]
    #[inline(always)]
    pub const fn fwmactrr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactrr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactrr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18088usize),
            )
        }
    }

    #[doc = "MAC Table Read Result Register 2"]
    #[inline(always)]
    pub const fn fwmactrr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactrr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactrr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18092usize),
            )
        }
    }

    #[doc = "MAC Table Read Result Register 3"]
    #[inline(always)]
    pub const fn fwmactrr3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactrr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactrr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18096usize),
            )
        }
    }

    #[doc = "MAC Table Read Result Register 4"]
    #[inline(always)]
    pub const fn fwmactrr4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactrr4_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactrr4_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18100usize),
            )
        }
    }

    #[doc = "MAC Table Read Result Register 50"]
    #[inline(always)]
    pub const fn fwmactrr50(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactrr50_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactrr50_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18112usize),
            )
        }
    }

    #[doc = "MAC Table Read Result Register 6"]
    #[inline(always)]
    pub const fn fwmactrr6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmactrr6_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmactrr6_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18128usize),
            )
        }
    }

    #[doc = "MAC Aging US Prescaler Configuration Register"]
    #[inline(always)]
    pub const fn fwmacaguspc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmacaguspc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmacaguspc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18560usize),
            )
        }
    }

    #[doc = "MAC Aging Configuration Register"]
    #[inline(always)]
    pub const fn fwmacagc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmacagc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmacagc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18564usize),
            )
        }
    }

    #[doc = "MAC Aging Monitoring Register 0"]
    #[inline(always)]
    pub const fn fwmacagm0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmacagm0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmacagm0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18568usize),
            )
        }
    }

    #[doc = "MAC Aging Monitoring Register 1"]
    #[inline(always)]
    pub const fn fwmacagm1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmacagm1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmacagm1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18572usize),
            )
        }
    }

    #[doc = "VLAN Table Entry Configuration Register"]
    #[inline(always)]
    pub const fn fwvlantec(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlantec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18688usize),
            )
        }
    }

    #[doc = "VLAN Table Learn Register 0"]
    #[inline(always)]
    pub const fn fwvlantl0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlantl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18704usize),
            )
        }
    }

    #[doc = "VLAN Table Learn Register 1"]
    #[inline(always)]
    pub const fn fwvlantl1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlantl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18708usize),
            )
        }
    }

    #[doc = "VLAN Table Learn Register 2"]
    #[inline(always)]
    pub const fn fwvlantl2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantl2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlantl2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18712usize),
            )
        }
    }

    #[doc = "VLAN Table Learn Register 30"]
    #[inline(always)]
    pub const fn fwvlantl30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantl30_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlantl30_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18720usize),
            )
        }
    }

    #[doc = "VLAN Table Learn Register 4"]
    #[inline(always)]
    pub const fn fwvlantl4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantl4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlantl4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18736usize),
            )
        }
    }

    #[doc = "VLAN Table Learn Result Register"]
    #[inline(always)]
    pub const fn fwvlantlr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantlr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwvlantlr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18740usize),
            )
        }
    }

    #[doc = "VLAN Table Initialization Monitoring Register"]
    #[inline(always)]
    pub const fn fwvlantim(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlantim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18752usize),
            )
        }
    }

    #[doc = "VLAN Table Entry Monitoring Register"]
    #[inline(always)]
    pub const fn fwvlantem(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantem_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwvlantem_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18756usize),
            )
        }
    }

    #[doc = "VLAN Table Search Register"]
    #[inline(always)]
    pub const fn fwvlants(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlants_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwvlants_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18768usize),
            )
        }
    }

    #[doc = "VLAN Table Search Result Register 0"]
    #[inline(always)]
    pub const fn fwvlantsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwvlantsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18772usize),
            )
        }
    }

    #[doc = "VLAN Table Search Result Register 1"]
    #[inline(always)]
    pub const fn fwvlantsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantsr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwvlantsr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18776usize),
            )
        }
    }

    #[doc = "VLAN Table Search Result Register 20"]
    #[inline(always)]
    pub const fn fwvlantsr20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantsr20_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwvlantsr20_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18784usize),
            )
        }
    }

    #[doc = "VLAN Table Search Result Register 3"]
    #[inline(always)]
    pub const fn fwvlantsr3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwvlantsr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwvlantsr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(18800usize),
            )
        }
    }

    #[doc = "Port %s Port Based Forwarding Configuration Register"]
    #[inline(always)]
    pub const fn fwpbfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpbfc_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4a00usize))
        }
    }
    #[inline(always)]
    pub const fn fwpbfc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbfc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbfc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpbfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a20usize),
            )
        }
    }

    #[doc = "Port %s Port Based Forwarding CSD Configuration Register 0"]
    #[inline(always)]
    pub const fn fwpbfcsdc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpbfcsdc0_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4a04usize))
        }
    }
    #[inline(always)]
    pub const fn fwpbfcsdc00(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfcsdc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpbfcsdc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbfcsdc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfcsdc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpbfcsdc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbfcsdc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfcsdc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpbfcsdc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4a24usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Learn Register 0"]
    #[inline(always)]
    pub const fn fwl23url0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Url0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Url0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19968usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Learn Register 1"]
    #[inline(always)]
    pub const fn fwl23url1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Url1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Url1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19972usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Learn Register 2"]
    #[inline(always)]
    pub const fn fwl23url2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Url2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Url2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19976usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Learn Register 3"]
    #[inline(always)]
    pub const fn fwl23url3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Url3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Url3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19980usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Learn Result Register"]
    #[inline(always)]
    pub const fn fwl23urlr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urlr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urlr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(19984usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Table Initialization Monitoring Register"]
    #[inline(always)]
    pub const fn fwl23utim(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Utim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Utim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20000usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Read Register"]
    #[inline(always)]
    pub const fn fwl23urr(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20016usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Read Result Register 0"]
    #[inline(always)]
    pub const fn fwl23urrr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urrr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urrr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(20020usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Read Result Register 1"]
    #[inline(always)]
    pub const fn fwl23urrr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urrr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urrr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(20024usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Read Result Register 2"]
    #[inline(always)]
    pub const fn fwl23urrr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urrr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urrr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(20028usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Rule Read Result Register 3"]
    #[inline(always)]
    pub const fn fwl23urrr3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urrr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urrr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(20032usize),
            )
        }
    }

    #[doc = "Layer 2/Layer 3 Update Remapping Configuration Register %s"]
    #[inline(always)]
    pub const fn fwl23urmc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW>,
        32,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4f00usize))
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwl23urmc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwl23Urmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwl23Urmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4f7cusize),
            )
        }
    }

    #[doc = "PSFP MSDU Filter Global Configuration Register %s"]
    #[inline(always)]
    pub const fn fwpmfgc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5000usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5000usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5008usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x500cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5010usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5014usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5018usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x501cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5020usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5024usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5028usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x502cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5030usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5038usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfgc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmfgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x503cusize),
            )
        }
    }

    #[doc = "PSFP Meter %s Filter Configuration Register"]
    #[inline(always)]
    pub const fn fwpmtrfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5600usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5640usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5660usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5680usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5700usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5720usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5740usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5760usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5780usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5820usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5860usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5880usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5900usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5920usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5940usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5960usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5980usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59e0usize),
            )
        }
    }

    #[doc = "PSFP Meter %s CBS Configuration Register"]
    #[inline(always)]
    pub const fn fwpmtrcbsc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5604usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5644usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5664usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5684usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5704usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5724usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5744usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5764usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5784usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5884usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5904usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5924usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5944usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5964usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5984usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcbsc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcbsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcbsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59e4usize),
            )
        }
    }

    #[doc = "PSFP Meter %s CIR Configuration Register"]
    #[inline(always)]
    pub const fn fwpmtrcirc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5608usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5648usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5668usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5688usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5708usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5728usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5748usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5768usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5788usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x57e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5828usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5848usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5868usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5888usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5908usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5928usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5948usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5968usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5988usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrcirc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrcirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrcirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x59e8usize),
            )
        }
    }

    #[doc = "PSFP Meter %s EBS Configuration Register"]
    #[inline(always)]
    pub const fn fwpmtrebsc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW>,
        8,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x560cusize))
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x560cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x562cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x564cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x566cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x568cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrebsc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrebsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrebsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56ecusize),
            )
        }
    }

    #[doc = "PSFP Meter %s EIR Configuration Register"]
    #[inline(always)]
    pub const fn fwpmtreirc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW>,
        8,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5610usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5650usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5670usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x5690usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtreirc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtreirc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwpmtreirc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x56f0usize),
            )
        }
    }

    #[doc = "PSFP Meter %s Filter Monitoring Register"]
    #[inline(always)]
    pub const fn fwpmtrfm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5614usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5654usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5674usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5694usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x56b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x56d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x56f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5714usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5734usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5754usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5774usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5794usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x57b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x57d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x57f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5874usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5894usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x58b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x58d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x58f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5914usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5934usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5954usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5974usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x5994usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x59b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x59d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmtrfm31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmtrfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmtrfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x59f4usize),
            )
        }
    }

    #[doc = "FRER Table Learn Register 0"]
    #[inline(always)]
    pub const fn fwftl0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwftl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24576usize),
            )
        }
    }

    #[doc = "FRER Table Learn Register 1"]
    #[inline(always)]
    pub const fn fwftl1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwftl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24580usize),
            )
        }
    }

    #[doc = "FRER Table Learn Result Register"]
    #[inline(always)]
    pub const fn fwftlr(&self) -> &'static crate::common::Reg<self::Fwftlr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwftlr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24584usize),
            )
        }
    }

    #[doc = "FRER Timeout Configuration Register"]
    #[inline(always)]
    pub const fn fwftoc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftoc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwftoc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24592usize),
            )
        }
    }

    #[doc = "FRER Timeout Prescaler Configuration Register 0"]
    #[inline(always)]
    pub const fn fwftopc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftopc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwftopc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24596usize),
            )
        }
    }

    #[doc = "FRER Table Initialization Monitoring Register"]
    #[inline(always)]
    pub const fn fwftim(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwftim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24608usize),
            )
        }
    }

    #[doc = "FRER Table Read Register"]
    #[inline(always)]
    pub const fn fwftr(&self) -> &'static crate::common::Reg<self::Fwftr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwftr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24624usize),
            )
        }
    }

    #[doc = "FRER Table Read Result Register 0"]
    #[inline(always)]
    pub const fn fwftrr0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftrr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwftrr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24628usize),
            )
        }
    }

    #[doc = "FRER Table Read Result Register 1"]
    #[inline(always)]
    pub const fn fwftrr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftrr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwftrr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24632usize),
            )
        }
    }

    #[doc = "FRER Table Read Result Register 2"]
    #[inline(always)]
    pub const fn fwftrr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwftrr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwftrr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24636usize),
            )
        }
    }

    #[doc = "Sequence Number Generation %s Configuration Register"]
    #[inline(always)]
    pub const fn fwseqngc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW>,
        32,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6100usize))
        }
    }
    #[inline(always)]
    pub const fn fwseqngc0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6158usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6190usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6198usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngc31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwseqngc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x61f8usize),
            )
        }
    }

    #[doc = "Sequence Number Generation %s Monitoring Register"]
    #[inline(always)]
    pub const fn fwseqngm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R>,
        32,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6104usize))
        }
    }
    #[inline(always)]
    pub const fn fwseqngm0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x610cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x611cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x612cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x613cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x614cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x615cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6164usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x616cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x617cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x618cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6194usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x619cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61acusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61bcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61ccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61dcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61ecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61f4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwseqngm31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqngm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwseqngm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61fcusize),
            )
        }
    }

    #[doc = "Sequence Number Reset Configuration Register"]
    #[inline(always)]
    pub const fn fwseqnrc(
        &self,
    ) -> &'static crate::common::Reg<self::Fwseqnrc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Fwseqnrc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(25088usize),
            )
        }
    }

    #[doc = "Port %s Cut-Through Forwarded Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwctfdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctfdcn_SPEC, crate::common::R>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6300usize))
        }
    }
    #[inline(always)]
    pub const fn fwctfdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwctfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctfdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwctfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6320usize),
            )
        }
    }

    #[doc = "Port %s Layer 3 Forwarded Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwlthfdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwlthfdcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6304usize))
        }
    }
    #[inline(always)]
    pub const fn fwlthfdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwlthfdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwlthfdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6344usize),
            )
        }
    }

    #[doc = "Port %s Layer 2 Forwarded Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwltwfdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwltwfdcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x630cusize))
        }
    }
    #[inline(always)]
    pub const fn fwltwfdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwltwfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwltwfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x630cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwltwfdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwltwfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwltwfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x632cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwltwfdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwltwfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwltwfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x634cusize),
            )
        }
    }

    #[doc = "Port %s Port Based Forwarded Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwpbfdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpbfdcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6310usize))
        }
    }
    #[inline(always)]
    pub const fn fwpbfdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpbfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbfdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpbfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbfdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpbfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6350usize),
            )
        }
    }

    #[doc = "Port %s MAC Hardware Learn Counter Register"]
    #[inline(always)]
    pub const fn fwmhlcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwmhlcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6314usize))
        }
    }
    #[inline(always)]
    pub const fn fwmhlcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmhlcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmhlcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwmhlcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmhlcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmhlcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwmhlcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmhlcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwmhlcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6354usize),
            )
        }
    }

    #[doc = "Port 2 Direct Descriptor Forwarded Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwddfdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwddfdcn2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwddfdcn2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(25408usize),
            )
        }
    }

    #[doc = "Port %s Watermark Rejected Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwwmrdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwwmrdcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6504usize))
        }
    }
    #[inline(always)]
    pub const fn fwwmrdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwwmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwwmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6504usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwwmrdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwwmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwwmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6524usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwwmrdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwwmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwwmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6544usize),
            )
        }
    }

    #[doc = "Port %s Cut-Through Rejected Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwctrdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwctrdcn_SPEC, crate::common::R>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6508usize))
        }
    }
    #[inline(always)]
    pub const fn fwctrdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwctrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwctrdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwctrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwctrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6528usize),
            )
        }
    }

    #[doc = "Port %s Layer 3 Rejected Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwlthrdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwlthrdcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x650cusize))
        }
    }
    #[inline(always)]
    pub const fn fwlthrdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x650cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwlthrdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x652cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwlthrdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwlthrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwlthrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x654cusize),
            )
        }
    }

    #[doc = "Port %s Layer 2 Rejected Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwltwrdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwltwrdcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6514usize))
        }
    }
    #[inline(always)]
    pub const fn fwltwrdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwltwrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwltwrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6514usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwltwrdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwltwrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwltwrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6534usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwltwrdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwltwrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwltwrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6554usize),
            )
        }
    }

    #[doc = "Port %s Port Based Rejected Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwpbrdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpbrdcn_SPEC, crate::common::R>,
        3,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6518usize))
        }
    }
    #[inline(always)]
    pub const fn fwpbrdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpbrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6518usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbrdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpbrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6538usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpbrdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpbrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpbrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6558usize),
            )
        }
    }

    #[doc = "Port 2 Direct Descriptor Rejected Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwddrdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwddrdcn2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwddrdcn2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(25928usize),
            )
        }
    }

    #[doc = "PSFP MSDU %s Filtered Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwpmfdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6700usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6700usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6704usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6708usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x670cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6710usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6714usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6718usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x671cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6720usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6724usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6728usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x672cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6730usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6734usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6738usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmfdcn15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmfdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmfdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x673cusize),
            )
        }
    }

    #[doc = "PSFP Meter %s Green Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwpmgdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6800usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6810usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6820usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6830usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6850usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6860usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6870usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6880usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6890usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68f0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6900usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6910usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6920usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6930usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6940usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6950usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6960usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6970usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6980usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6990usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmgdcn31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmgdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmgdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69f0usize),
            )
        }
    }

    #[doc = "PSFP Meter %s Yellow Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwpmydcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6804usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmydcn7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmydcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmydcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6874usize),
            )
        }
    }

    #[doc = "PSFP Meter %s Red Descriptor Counter Register"]
    #[inline(always)]
    pub const fn fwpmrdcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6808usize))
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6818usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6828usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6838usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6848usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6858usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6868usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6878usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6888usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6898usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x68f8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6908usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6918usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6928usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6938usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6948usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6958usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6968usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6978usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6988usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6998usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwpmrdcn31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwpmrdcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwpmrdcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x69f8usize),
            )
        }
    }

    #[doc = "FRER %s Passed Packet Counter Register"]
    #[inline(always)]
    pub const fn fwfrppcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R>,
        128,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6a00usize))
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6aa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6aa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ab0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ab8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ac0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ac8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ad0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ad8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ae0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ae8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6af0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6af8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn32(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn33(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn34(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn35(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn36(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn37(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn38(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn39(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn41(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn42(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn43(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn44(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn45(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn46(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn47(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn48(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn49(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn50(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn51(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn52(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ba0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn53(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ba8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn54(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn55(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn56(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn57(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn58(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn59(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn60(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6be0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn61(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6be8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn62(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn63(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn64(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn65(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn66(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn67(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn68(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn69(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn70(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn71(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn72(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn73(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn74(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn75(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn76(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn77(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn78(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn79(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn80(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn81(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn82(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn83(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn84(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ca0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn85(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ca8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn86(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn87(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cb8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn88(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn89(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn90(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn91(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn92(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ce0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn93(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ce8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn94(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cf0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn95(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cf8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn96(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn97(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn98(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn99(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn100(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn101(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn102(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn103(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn104(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn105(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn106(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn107(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn108(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn109(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn110(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn111(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn112(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn113(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn114(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn115(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn116(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6da0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn117(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6da8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn118(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6db0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn119(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6db8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn120(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dc0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn121(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dc8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn122(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn123(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn124(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6de0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn125(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6de8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn126(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6df0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrppcn127(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrppcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrppcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6df8usize),
            )
        }
    }

    #[doc = "FRER %s Discarded Packet Counter Register"]
    #[inline(always)]
    pub const fn fwfrdpcn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R>,
        128,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6a04usize))
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn1(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn2(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn3(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn4(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn5(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn6(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn7(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn8(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn9(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn10(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn11(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn12(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn13(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn14(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn15(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn16(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn17(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn18(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn19(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6a9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn20(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6aa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn21(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6aacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn22(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ab4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn23(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6abcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn24(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ac4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn25(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6accusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn26(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ad4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn27(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6adcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn28(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ae4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn29(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6aecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn30(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6af4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn31(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6afcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn32(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn33(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn34(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn35(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn36(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn37(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn38(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn39(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn40(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn41(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn42(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn43(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn44(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn45(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn46(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn47(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn48(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn49(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn50(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn51(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6b9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn52(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ba4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn53(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn54(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn55(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn56(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn57(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn58(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn59(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn60(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6be4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn61(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6becusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn62(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn63(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6bfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn64(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn65(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn66(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn67(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn68(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn69(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn70(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn71(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn72(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn73(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn74(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn75(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn76(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn77(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn78(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn79(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn80(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn81(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn82(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn83(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6c9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn84(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ca4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn85(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn86(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cb4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn87(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn88(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn89(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn90(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn91(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn92(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ce4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn93(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cecusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn94(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cf4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn95(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cfcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn96(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d04usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn97(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d0cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn98(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn99(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn100(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn101(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn102(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn103(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d3cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn104(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn105(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn106(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn107(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d5cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn108(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn109(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d6cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn110(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn111(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn112(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn113(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn114(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn115(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6d9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn116(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6da4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn117(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn118(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6db4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn119(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dbcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn120(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dc4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn121(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn122(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn123(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6ddcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn124(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6de4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn125(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6decusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn126(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6df4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwfrdpcn127(
        &self,
    ) -> &'static crate::common::Reg<self::Fwfrdpcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fwfrdpcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x6dfcusize),
            )
        }
    }

    #[doc = "Port %s Error Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn fweis0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fweis0_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x7900usize))
        }
    }
    #[inline(always)]
    pub const fn fweis00(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7900usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fweis01(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7910usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fweis02(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7920usize),
            )
        }
    }

    #[doc = "Port %s Error Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn fweie0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fweie0_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x7904usize))
        }
    }
    #[inline(always)]
    pub const fn fweie00(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7904usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fweie01(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7914usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fweie02(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7924usize),
            )
        }
    }

    #[doc = "Port %s Error Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn fweid0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fweid0_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x7908usize))
        }
    }
    #[inline(always)]
    pub const fn fweid00(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7908usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fweid01(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7918usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fweid02(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7928usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn fweis1(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31232usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn fweie1(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31236usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn fweid1(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31240usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn fweis2(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31248usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn fweie2(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31252usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn fweid2(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31256usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 5"]
    #[inline(always)]
    pub const fn fweis5(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31296usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 5"]
    #[inline(always)]
    pub const fn fweie5(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31300usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 5"]
    #[inline(always)]
    pub const fn fweid5(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31304usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 60"]
    #[inline(always)]
    pub const fn fweis60(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis60_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis60_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31312usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 60"]
    #[inline(always)]
    pub const fn fweie60(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie60_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie60_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31316usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 60"]
    #[inline(always)]
    pub const fn fweid60(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid60_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid60_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31320usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 61"]
    #[inline(always)]
    pub const fn fweis61(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis61_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis61_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31328usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 61"]
    #[inline(always)]
    pub const fn fweie61(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie61_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie61_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31332usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 61"]
    #[inline(always)]
    pub const fn fweid61(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid61_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid61_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31336usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 62"]
    #[inline(always)]
    pub const fn fweis62(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis62_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis62_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31344usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 62"]
    #[inline(always)]
    pub const fn fweie62(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie62_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie62_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31348usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 62"]
    #[inline(always)]
    pub const fn fweid62(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid62_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid62_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31352usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 63"]
    #[inline(always)]
    pub const fn fweis63(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis63_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis63_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31360usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 63"]
    #[inline(always)]
    pub const fn fweie63(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie63_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie63_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31364usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 63"]
    #[inline(always)]
    pub const fn fweid63(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid63_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid63_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31368usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 70"]
    #[inline(always)]
    pub const fn fweis70(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis70_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis70_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31376usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 70"]
    #[inline(always)]
    pub const fn fweie70(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie70_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie70_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31380usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 70"]
    #[inline(always)]
    pub const fn fweid70(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid70_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid70_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31384usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 71"]
    #[inline(always)]
    pub const fn fweis71(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis71_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis71_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31392usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 71"]
    #[inline(always)]
    pub const fn fweie71(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie71_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie71_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31396usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 71"]
    #[inline(always)]
    pub const fn fweid71(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid71_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid71_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31400usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 72"]
    #[inline(always)]
    pub const fn fweis72(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis72_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis72_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31408usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 72"]
    #[inline(always)]
    pub const fn fweie72(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie72_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie72_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31412usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 72"]
    #[inline(always)]
    pub const fn fweid72(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid72_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid72_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31416usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 73"]
    #[inline(always)]
    pub const fn fweis73(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis73_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis73_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31424usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 73"]
    #[inline(always)]
    pub const fn fweie73(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie73_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie73_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31428usize),
            )
        }
    }

    #[doc = "Forwarding Engine Error Interrupt Disable 73"]
    #[inline(always)]
    pub const fn fweid73(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid73_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid73_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31432usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 80"]
    #[inline(always)]
    pub const fn fweis80(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis80_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis80_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31440usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 80"]
    #[inline(always)]
    pub const fn fweie80(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie80_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie80_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31444usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 80"]
    #[inline(always)]
    pub const fn fweid80(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid80_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid80_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31448usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 81"]
    #[inline(always)]
    pub const fn fweis81(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis81_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis81_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31456usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 81"]
    #[inline(always)]
    pub const fn fweie81(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie81_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie81_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31460usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 81"]
    #[inline(always)]
    pub const fn fweid81(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid81_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid81_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31464usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 82"]
    #[inline(always)]
    pub const fn fweis82(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis82_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis82_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31472usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 82"]
    #[inline(always)]
    pub const fn fweie82(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie82_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie82_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31476usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 82"]
    #[inline(always)]
    pub const fn fweid82(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid82_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid82_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31480usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 83"]
    #[inline(always)]
    pub const fn fweis83(
        &self,
    ) -> &'static crate::common::Reg<self::Fweis83_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweis83_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31488usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 83"]
    #[inline(always)]
    pub const fn fweie83(
        &self,
    ) -> &'static crate::common::Reg<self::Fweie83_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweie83_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31492usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 83"]
    #[inline(always)]
    pub const fn fweid83(
        &self,
    ) -> &'static crate::common::Reg<self::Fweid83_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fweid83_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31496usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn fwmis0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31744usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn fwmie0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31748usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn fwmid0(
        &self,
    ) -> &'static crate::common::Reg<self::Fwmid0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwmid0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(31752usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwgc_SPEC;
impl crate::sealed::RegSpec for Fwgc_SPEC {
    type DataType = u32;
}

#[doc = "General Configuration Register"]
pub type Fwgc = crate::RegValueT<Fwgc_SPEC>;

impl Fwgc {
    #[doc = "Switch VLAN Mode"]
    #[inline(always)]
    pub fn svm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fwgc::Svm,
        fwgc::Svm,
        Fwgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fwgc::Svm,
            fwgc::Svm,
            Fwgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwgc {
    #[inline(always)]
    fn default() -> Fwgc {
        <crate::RegValueT<Fwgc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwgc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svm_SPEC;
    pub type Svm = crate::EnumBitfieldStruct<u8, Svm_SPEC>;
    impl Svm {
        #[doc = "No VLAN mode (VLAN not used for forwarding)"]
        pub const _00: Self = Self::new(0);

        #[doc = "C-TAG mode (C-TAG used for forwarding)"]
        pub const _01: Self = Self::new(1);

        #[doc = "SC-TAG mode (S-TAG used for forwarding)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwttc0_SPEC;
impl crate::sealed::RegSpec for Fwttc0_SPEC {
    type DataType = u32;
}

#[doc = "TAG TPID Configuration Register 0"]
pub type Fwttc0 = crate::RegValueT<Fwttc0_SPEC>;

impl Fwttc0 {
    #[doc = "C-TAG TPID \\[801.2Q\\]"]
    #[inline(always)]
    pub fn ctt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwttc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwttc0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "S-TAG TPID \\[801.2Q\\]"]
    #[inline(always)]
    pub fn stt(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Fwttc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Fwttc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwttc0 {
    #[inline(always)]
    fn default() -> Fwttc0 {
        <crate::RegValueT<Fwttc0_SPEC> as RegisterValue<_>>::new(2290647296)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwttc1_SPEC;
impl crate::sealed::RegSpec for Fwttc1_SPEC {
    type DataType = u32;
}

#[doc = "TAG TPID Configuration Register 1"]
pub type Fwttc1 = crate::RegValueT<Fwttc1_SPEC>;

impl Fwttc1 {
    #[doc = "R-TAG TPID \\[801.2CB\\]"]
    #[inline(always)]
    pub fn rtt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwttc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwttc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwttc1 {
    #[inline(always)]
    fn default() -> Fwttc1 {
        <crate::RegValueT<Fwttc1_SPEC> as RegisterValue<_>>::new(61889)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwceptc_SPEC;
impl crate::sealed::RegSpec for Fwceptc_SPEC {
    type DataType = u32;
}

#[doc = "CPU Exceptional Path Target Configuration Register"]
pub type Fwceptc = crate::RegValueT<Fwceptc_SPEC>;

impl Fwceptc {
    #[doc = "Exceptional Path CPU Sub Destination"]
    #[inline(always)]
    pub fn epcsd(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwceptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwceptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Exceptional Path Internal Priority Value"]
    #[inline(always)]
    pub fn epipv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwceptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwceptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Exceptional Path CPU Select"]
    #[inline(always)]
    pub fn epcs(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fwceptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fwceptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Exceptional Path Security Level"]
    #[inline(always)]
    pub fn epsl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwceptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fwceptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwceptc {
    #[inline(always)]
    fn default() -> Fwceptc {
        <crate::RegValueT<Fwceptc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwceprc0_SPEC;
impl crate::sealed::RegSpec for Fwceprc0_SPEC {
    type DataType = u32;
}

#[doc = "CPU Exceptional Path Reason Configuration Register 0"]
pub type Fwceprc0 = crate::RegValueT<Fwceprc0_SPEC>;

impl Fwceprc0 {
    #[doc = "Ethernet PHY Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn ephyeef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwceprc0::Ephyeef,
        fwceprc0::Ephyeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwceprc0::Ephyeef,
            fwceprc0::Ephyeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet PCH CRC Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn epcrceef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwceprc0::Epcrceef,
        fwceprc0::Epcrceef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwceprc0::Epcrceef,
            fwceprc0::Epcrceef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet Nibble Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn enibeef(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwceprc0::Enibeef,
        fwceprc0::Enibeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwceprc0::Enibeef,
            fwceprc0::Enibeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet FCS Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn efcseef(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwceprc0::Efcseef,
        fwceprc0::Efcseef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwceprc0::Efcseef,
            fwceprc0::Efcseef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet Final Fragment Missing Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn effmeef(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwceprc0::Effmeef,
        fwceprc0::Effmeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwceprc0::Effmeef,
            fwceprc0::Effmeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet C-Fragment SMD Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn ecfseef(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwceprc0::Ecfseef,
        fwceprc0::Ecfseef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwceprc0::Ecfseef,
            fwceprc0::Ecfseef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet C-Fragment FRAG_COUNT Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn ecffceef(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwceprc0::Ecffceef,
        fwceprc0::Ecffceef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwceprc0::Ecffceef,
            fwceprc0::Ecffceef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet RMAC Frame Filtered Exceptional Forwarding"]
    #[inline(always)]
    pub fn erffef(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwceprc0::Erffef,
        fwceprc0::Erffef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwceprc0::Erffef,
            fwceprc0::Erffef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet Reception Partially Out of Operation Exceptional Forwarding"]
    #[inline(always)]
    pub fn erpooef(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwceprc0::Erpooef,
        fwceprc0::Erpooef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwceprc0::Erpooef,
            fwceprc0::Erpooef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet Buffer Overflow Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn eboeef(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fwceprc0::Eboeef,
        fwceprc0::Eboeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fwceprc0::Eboeef,
            fwceprc0::Eboeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet Undersize Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn eueef(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fwceprc0::Eueef,
        fwceprc0::Eueef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fwceprc0::Eueef,
            fwceprc0::Eueef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet Oversize Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn eoeef(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fwceprc0::Eoeef,
        fwceprc0::Eoeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fwceprc0::Eoeef,
            fwceprc0::Eoeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet TAG Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn etfef(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fwceprc0::Etfef,
        fwceprc0::Etfef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fwceprc0::Etfef,
            fwceprc0::Etfef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA AXI Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn gaxeef(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwceprc0::Gaxeef,
        fwceprc0::Gaxeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwceprc0::Gaxeef,
            fwceprc0::Gaxeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA Sequence Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn gseqeef(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fwceprc0::Gseqeef,
        fwceprc0::Gseqeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fwceprc0::Gseqeef,
            fwceprc0::Gseqeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA TAG Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn gtfef(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fwceprc0::Gtfef,
        fwceprc0::Gtfef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fwceprc0::Gtfef,
            fwceprc0::Gtfef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "GWCA Descriptor Number Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn gdneef(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fwceprc0::Gdneef,
        fwceprc0::Gdneef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fwceprc0::Gdneef,
            fwceprc0::Gdneef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor Error Exceptional Forwarding"]
    #[inline(always)]
    pub fn ddeef(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        fwceprc0::Ddeef,
        fwceprc0::Ddeef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            fwceprc0::Ddeef,
            fwceprc0::Ddeef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor Format Security Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn ddfsfef(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        fwceprc0::Ddfsfef,
        fwceprc0::Ddfsfef,
        Fwceprc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            fwceprc0::Ddfsfef,
            fwceprc0::Ddfsfef,
            Fwceprc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwceprc0 {
    #[inline(always)]
    fn default() -> Fwceprc0 {
        <crate::RegValueT<Fwceprc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwceprc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ephyeef_SPEC;
    pub type Ephyeef = crate::EnumBitfieldStruct<u8, Ephyeef_SPEC>;
    impl Ephyeef {
        #[doc = "Frames received with PHY Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with PHY Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epcrceef_SPEC;
    pub type Epcrceef = crate::EnumBitfieldStruct<u8, Epcrceef_SPEC>;
    impl Epcrceef {
        #[doc = "Frames received with PCH CRC Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with PCH CRC Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enibeef_SPEC;
    pub type Enibeef = crate::EnumBitfieldStruct<u8, Enibeef_SPEC>;
    impl Enibeef {
        #[doc = "Frames received with Nibble Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Nibble Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Efcseef_SPEC;
    pub type Efcseef = crate::EnumBitfieldStruct<u8, Efcseef_SPEC>;
    impl Efcseef {
        #[doc = "Frames received with FCS Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with FCS Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Effmeef_SPEC;
    pub type Effmeef = crate::EnumBitfieldStruct<u8, Effmeef_SPEC>;
    impl Effmeef {
        #[doc = "Frames received with Final Fragment Missing Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Final Fragment Missing Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecfseef_SPEC;
    pub type Ecfseef = crate::EnumBitfieldStruct<u8, Ecfseef_SPEC>;
    impl Ecfseef {
        #[doc = "Frames received with C-Fragment SMD Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with C-Fragment SMD Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecffceef_SPEC;
    pub type Ecffceef = crate::EnumBitfieldStruct<u8, Ecffceef_SPEC>;
    impl Ecffceef {
        #[doc = "Frames received with C-Fragment FRAG_COUNT Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with C-Fragment FRAG_COUNT Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erffef_SPEC;
    pub type Erffef = crate::EnumBitfieldStruct<u8, Erffef_SPEC>;
    impl Erffef {
        #[doc = "Frames received with RMAC Frame Filtered flag are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with RMAC Frame Filtered flag are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erpooef_SPEC;
    pub type Erpooef = crate::EnumBitfieldStruct<u8, Erpooef_SPEC>;
    impl Erpooef {
        #[doc = "Frames received with Reception Partially Out of Operation flag are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Reception Partially Out of Operation flag are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eboeef_SPEC;
    pub type Eboeef = crate::EnumBitfieldStruct<u8, Eboeef_SPEC>;
    impl Eboeef {
        #[doc = "Frames received with Buffer Overflow Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Buffer Overflow Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eueef_SPEC;
    pub type Eueef = crate::EnumBitfieldStruct<u8, Eueef_SPEC>;
    impl Eueef {
        #[doc = "Frames received with Undersize Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Undersize Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoeef_SPEC;
    pub type Eoeef = crate::EnumBitfieldStruct<u8, Eoeef_SPEC>;
    impl Eoeef {
        #[doc = "Frames received with Oversize Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Oversize Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Etfef_SPEC;
    pub type Etfef = crate::EnumBitfieldStruct<u8, Etfef_SPEC>;
    impl Etfef {
        #[doc = "Frames received with TAG Filtering flag are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with TAG Filtering flag are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaxeef_SPEC;
    pub type Gaxeef = crate::EnumBitfieldStruct<u8, Gaxeef_SPEC>;
    impl Gaxeef {
        #[doc = "Frames received with AXI Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with AXI Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gseqeef_SPEC;
    pub type Gseqeef = crate::EnumBitfieldStruct<u8, Gseqeef_SPEC>;
    impl Gseqeef {
        #[doc = "Frames received with Sequence Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Sequence Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gtfef_SPEC;
    pub type Gtfef = crate::EnumBitfieldStruct<u8, Gtfef_SPEC>;
    impl Gtfef {
        #[doc = "Frames received with TAG Filtering flag are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with TAG Filtering flag are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gdneef_SPEC;
    pub type Gdneef = crate::EnumBitfieldStruct<u8, Gdneef_SPEC>;
    impl Gdneef {
        #[doc = "Frames received with Descriptor Number Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with Descriptor Number Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddeef_SPEC;
    pub type Ddeef = crate::EnumBitfieldStruct<u8, Ddeef_SPEC>;
    impl Ddeef {
        #[doc = "Frames with Direct Descriptor Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames with Direct Descriptor Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddfsfef_SPEC;
    pub type Ddfsfef = crate::EnumBitfieldStruct<u8, Ddfsfef_SPEC>;
    impl Ddfsfef {
        #[doc = "Frames with Direct Descriptor Format Security Error are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames with Direct Descriptor Format Security Error are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwceprc1_SPEC;
impl crate::sealed::RegSpec for Fwceprc1_SPEC {
    type DataType = u32;
}

#[doc = "CPU Exceptional Path Reason Configuration Register 1"]
pub type Fwceprc1 = crate::RegValueT<Fwceprc1_SPEC>;

impl Fwceprc1 {
    #[doc = "MSDU Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fmsdufef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwceprc1::Fmsdufef,
        fwceprc1::Fmsdufef,
        Fwceprc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwceprc1::Fmsdufef,
            fwceprc1::Fmsdufef,
            Fwceprc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Meter Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fmtrfef(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwceprc1::Fmtrfef,
        fwceprc1::Fmtrfef,
        Fwceprc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwceprc1::Fmtrfef,
            fwceprc1::Fmtrfef,
            Fwceprc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Individual FRER Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fiffef(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwceprc1::Fiffef,
        fwceprc1::Fiffef,
        Fwceprc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwceprc1::Fiffef,
            fwceprc1::Fiffef,
            Fwceprc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sequence FRER Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fsffef(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fwceprc1::Fsffef,
        fwceprc1::Fsffef,
        Fwceprc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fwceprc1::Fsffef,
            fwceprc1::Fsffef,
            Fwceprc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwceprc1 {
    #[inline(always)]
    fn default() -> Fwceprc1 {
        <crate::RegValueT<Fwceprc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwceprc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmsdufef_SPEC;
    pub type Fmsdufef = crate::EnumBitfieldStruct<u8, Fmsdufef_SPEC>;
    impl Fmsdufef {
        #[doc = "Frames filtered by PSFP MSDU filter are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered by PSFP MSDU filter are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmtrfef_SPEC;
    pub type Fmtrfef = crate::EnumBitfieldStruct<u8, Fmtrfef_SPEC>;
    impl Fmtrfef {
        #[doc = "Frames filtered by PSFP Meter filter are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered by PSFP Meter filter are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fiffef_SPEC;
    pub type Fiffef = crate::EnumBitfieldStruct<u8, Fiffef_SPEC>;
    impl Fiffef {
        #[doc = "Frames filtered by FRER individual recovery are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered by FRER individual recovery are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsffef_SPEC;
    pub type Fsffef = crate::EnumBitfieldStruct<u8, Fsffef_SPEC>;
    impl Fsffef {
        #[doc = "Frames filtered by FRER Sequence recovery are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered by FRER Sequence recovery are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwceprc2_SPEC;
impl crate::sealed::RegSpec for Fwceprc2_SPEC {
    type DataType = u32;
}

#[doc = "CPU Exceptional Path Reason Configuration Register 2"]
pub type Fwceprc2 = crate::RegValueT<Fwceprc2_SPEC>;

impl Fwceprc2 {
    #[doc = "Layer 3 Unknown Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn flthufef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwceprc2::Flthufef,
        fwceprc2::Flthufef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwceprc2::Flthufef,
            fwceprc2::Flthufef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Destination MAC Unknown Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fdmacufef(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwceprc2::Fdmacufef,
        fwceprc2::Fdmacufef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwceprc2::Fdmacufef,
            fwceprc2::Fdmacufef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Source MAC Unknown Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fsmacufef(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwceprc2::Fsmacufef,
        fwceprc2::Fsmacufef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwceprc2::Fsmacufef,
            fwceprc2::Fsmacufef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Unknown Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fvlanufef(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwceprc2::Fvlanufef,
        fwceprc2::Fvlanufef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwceprc2::Fvlanufef,
            fwceprc2::Fvlanufef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor No Target Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fddntfef(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwceprc2::Fddntfef,
        fwceprc2::Fddntfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwceprc2::Fddntfef,
            fwceprc2::Fddntfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 No Target Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn flthntfef(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fwceprc2::Flthntfef,
        fwceprc2::Flthntfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fwceprc2::Flthntfef,
            fwceprc2::Flthntfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 No Target Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fltwntfef(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fwceprc2::Fltwntfef,
        fwceprc2::Fltwntfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fwceprc2::Fltwntfef,
            fwceprc2::Fltwntfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Based No Target Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fpbntfef(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fwceprc2::Fpbntfef,
        fwceprc2::Fpbntfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fwceprc2::Fpbntfef,
            fwceprc2::Fpbntfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 Source Lock Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn flthslfef(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwceprc2::Flthslfef,
        fwceprc2::Flthslfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwceprc2::Flthslfef,
            fwceprc2::Flthslfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Destination MAC Source Lock Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fdmacslfef(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        fwceprc2::Fdmacslfef,
        fwceprc2::Fdmacslfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            fwceprc2::Fdmacslfef,
            fwceprc2::Fdmacslfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Source MAC Source Lock Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fsmacslfef(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fwceprc2::Fsmacslfef,
        fwceprc2::Fsmacslfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fwceprc2::Fsmacslfef,
            fwceprc2::Fsmacslfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Source Lock Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fvlanslfef(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fwceprc2::Fvlanslfef,
        fwceprc2::Fvlanslfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fwceprc2::Fvlanslfef,
            fwceprc2::Fvlanslfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Filtering Exceptional Forwarding"]
    #[inline(always)]
    pub fn fwmfef(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        fwceprc2::Fwmfef,
        fwceprc2::Fwmfef,
        Fwceprc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            fwceprc2::Fwmfef,
            fwceprc2::Fwmfef,
            Fwceprc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwceprc2 {
    #[inline(always)]
    fn default() -> Fwceprc2 {
        <crate::RegValueT<Fwceprc2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwceprc2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flthufef_SPEC;
    pub type Flthufef = crate::EnumBitfieldStruct<u8, Flthufef_SPEC>;
    impl Flthufef {
        #[doc = "Frames filtered because their stream ID is unknown are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because their stream ID is unknown are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdmacufef_SPEC;
    pub type Fdmacufef = crate::EnumBitfieldStruct<u8, Fdmacufef_SPEC>;
    impl Fdmacufef {
        #[doc = "Frames filtered because their Destination MAC is unknown are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because their Destination MAC is unknown are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsmacufef_SPEC;
    pub type Fsmacufef = crate::EnumBitfieldStruct<u8, Fsmacufef_SPEC>;
    impl Fsmacufef {
        #[doc = "Frames filtered because their Source MAC is unknown are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because their Source MAC is unknown are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fvlanufef_SPEC;
    pub type Fvlanufef = crate::EnumBitfieldStruct<u8, Fvlanufef_SPEC>;
    impl Fvlanufef {
        #[doc = "Frames filtered because their VLAN is unknown are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because their VLAN is unknown are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fddntfef_SPEC;
    pub type Fddntfef = crate::EnumBitfieldStruct<u8, Fddntfef_SPEC>;
    impl Fddntfef {
        #[doc = "Frames filtered because no target during direct forwarding is detected are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because no target during direct forwarding is detected are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flthntfef_SPEC;
    pub type Flthntfef = crate::EnumBitfieldStruct<u8, Flthntfef_SPEC>;
    impl Flthntfef {
        #[doc = "Frames filtered because no target during Layer 3 forwarding is detected are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because no target during Layer 3 forwarding is detected are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fltwntfef_SPEC;
    pub type Fltwntfef = crate::EnumBitfieldStruct<u8, Fltwntfef_SPEC>;
    impl Fltwntfef {
        #[doc = "Frames filtered because the combination on VLAN destination vector and destination MAC destination vector is null are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because the combination on VLAN destination vector and destination MAC destination vector is null are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fpbntfef_SPEC;
    pub type Fpbntfef = crate::EnumBitfieldStruct<u8, Fpbntfef_SPEC>;
    impl Fpbntfef {
        #[doc = "Frames filtered because no target during Port Based forwarding is detected are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because no target during Port Based forwarding is detected are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flthslfef_SPEC;
    pub type Flthslfef = crate::EnumBitfieldStruct<u8, Flthslfef_SPEC>;
    impl Flthslfef {
        #[doc = "Frames filtered because of Layer 3 source lock are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because of Layer 3 source lock are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdmacslfef_SPEC;
    pub type Fdmacslfef = crate::EnumBitfieldStruct<u8, Fdmacslfef_SPEC>;
    impl Fdmacslfef {
        #[doc = "Frames filtered because of Destination MAC source lock are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because of Destination MAC source lock are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsmacslfef_SPEC;
    pub type Fsmacslfef = crate::EnumBitfieldStruct<u8, Fsmacslfef_SPEC>;
    impl Fsmacslfef {
        #[doc = "Frames filtered because of Source MAC source lock are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because of Source MAC source lock are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fvlanslfef_SPEC;
    pub type Fvlanslfef = crate::EnumBitfieldStruct<u8, Fvlanslfef_SPEC>;
    impl Fvlanslfef {
        #[doc = "Frames filtered because of VLAN source lock are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because of VLAN source lock are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fwmfef_SPEC;
    pub type Fwmfef = crate::EnumBitfieldStruct<u8, Fwmfef_SPEC>;
    impl Fwmfef {
        #[doc = "Frames filtered because of watermark are discarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames filtered because of watermark are forwarded to exceptional path."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwclptc_SPEC;
impl crate::sealed::RegSpec for Fwclptc_SPEC {
    type DataType = u32;
}

#[doc = "CPU Learning Path Target Configuration Register"]
pub type Fwclptc = crate::RegValueT<Fwclptc_SPEC>;

impl Fwclptc {
    #[doc = "Learning Path CPU Sub Destination"]
    #[inline(always)]
    pub fn lpcsd(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwclptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwclptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Learning Path Internal Priority Value"]
    #[inline(always)]
    pub fn lpipv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwclptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwclptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Learning Path CPU Select"]
    #[inline(always)]
    pub fn lpcs(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fwclptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fwclptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Learning Path Security Level"]
    #[inline(always)]
    pub fn lpsl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwclptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fwclptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwclptc {
    #[inline(always)]
    fn default() -> Fwclptc {
        <crate::RegValueT<Fwclptc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwclprc_SPEC;
impl crate::sealed::RegSpec for Fwclprc_SPEC {
    type DataType = u32;
}

#[doc = "CPU Learning Path Reason Configuration Register"]
pub type Fwclprc = crate::RegValueT<Fwclprc_SPEC>;

impl Fwclprc {
    #[doc = "Unknown Stream ID Learning Forwarding"]
    #[inline(always)]
    pub fn usidlf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwclprc::Usidlf,
        fwclprc::Usidlf,
        Fwclprc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwclprc::Usidlf,
            fwclprc::Usidlf,
            Fwclprc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Unknown Destination MAC Learning Forwarding"]
    #[inline(always)]
    pub fn udmaclf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwclprc::Udmaclf,
        fwclprc::Udmaclf,
        Fwclprc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwclprc::Udmaclf,
            fwclprc::Udmaclf,
            Fwclprc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Unknown Source MAC Learning Forwarding"]
    #[inline(always)]
    pub fn usmaclf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwclprc::Usmaclf,
        fwclprc::Usmaclf,
        Fwclprc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwclprc::Usmaclf,
            fwclprc::Usmaclf,
            Fwclprc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Unknown Port for Source MAC Learning Forwarding"]
    #[inline(always)]
    pub fn upsmaclf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwclprc::Upsmaclf,
        fwclprc::Upsmaclf,
        Fwclprc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwclprc::Upsmaclf,
            fwclprc::Upsmaclf,
            Fwclprc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Unknown VLAN Learning Forwarding"]
    #[inline(always)]
    pub fn uvlanlf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwclprc::Uvlanlf,
        fwclprc::Uvlanlf,
        Fwclprc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwclprc::Uvlanlf,
            fwclprc::Uvlanlf,
            Fwclprc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwclprc {
    #[inline(always)]
    fn default() -> Fwclprc {
        <crate::RegValueT<Fwclprc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwclprc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usidlf_SPEC;
    pub type Usidlf = crate::EnumBitfieldStruct<u8, Usidlf_SPEC>;
    impl Usidlf {
        #[doc = "Frames received with an unknown stream ID are not forwarded to Learning path."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with an unknown stream ID are forwarded to Learning path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udmaclf_SPEC;
    pub type Udmaclf = crate::EnumBitfieldStruct<u8, Udmaclf_SPEC>;
    impl Udmaclf {
        #[doc = "Frames received with an unknown destination MAC address are not forwarded to Learning path."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with an unknown destination MAC address are forwarded to Learning path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usmaclf_SPEC;
    pub type Usmaclf = crate::EnumBitfieldStruct<u8, Usmaclf_SPEC>;
    impl Usmaclf {
        #[doc = "Frames received with an unknown Source MAC are not forwarded to Learning path."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with an unknown Source MAC are forwarded to Learning path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Upsmaclf_SPEC;
    pub type Upsmaclf = crate::EnumBitfieldStruct<u8, Upsmaclf_SPEC>;
    impl Upsmaclf {
        #[doc = "Frames received with an unknown port for Source MAC are not forwarded to Learning path."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with an unknown port for Source MAC are forwarded to Learning path."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uvlanlf_SPEC;
    pub type Uvlanlf = crate::EnumBitfieldStruct<u8, Uvlanlf_SPEC>;
    impl Uvlanlf {
        #[doc = "Frames received with an unknown VLAN are not forwarded to Learning path."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames received with an unknown VLAN are forwarded to Learning path."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcmptc_SPEC;
impl crate::sealed::RegSpec for Fwcmptc_SPEC {
    type DataType = u32;
}

#[doc = "CPU Mirroring Path Target Configuration Register"]
pub type Fwcmptc = crate::RegValueT<Fwcmptc_SPEC>;

impl Fwcmptc {
    #[doc = "CPU Mirroring Path CPU Sub Destination"]
    #[inline(always)]
    pub fn cmpcsd(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwcmptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwcmptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CPU Mirroring Path Internal Priority Value"]
    #[inline(always)]
    pub fn cmpipv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwcmptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwcmptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CPU Mirroring Path Internal Priority Update"]
    #[inline(always)]
    pub fn cmpipu(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fwcmptc::Cmpipu,
        fwcmptc::Cmpipu,
        Fwcmptc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fwcmptc::Cmpipu,
            fwcmptc::Cmpipu,
            Fwcmptc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Mirroring Path CPU Select"]
    #[inline(always)]
    pub fn cmpcs(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fwcmptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fwcmptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CPU Mirroring Path Security Level"]
    #[inline(always)]
    pub fn cmpsl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwcmptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fwcmptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcmptc {
    #[inline(always)]
    fn default() -> Fwcmptc {
        <crate::RegValueT<Fwcmptc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwcmptc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpipu_SPEC;
    pub type Cmpipu = crate::EnumBitfieldStruct<u8, Cmpipu_SPEC>;
    impl Cmpipu {
        #[doc = "Frames mirrored to CPU path are sent with the same priority as for forwarding."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames mirrored to CPU path are sent with FWCMPTC.CMPIPV priority."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwemptc_SPEC;
impl crate::sealed::RegSpec for Fwemptc_SPEC {
    type DataType = u32;
}

#[doc = "Ethernet Mirroring Path Target Configuration Register"]
pub type Fwemptc = crate::RegValueT<Fwemptc_SPEC>;

impl Fwemptc {
    #[doc = "Ethernet Mirroring Path Internal Priority Value"]
    #[inline(always)]
    pub fn empipv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwemptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwemptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ethernet Mirroring Path Internal Priority Update"]
    #[inline(always)]
    pub fn empipu(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fwemptc::Empipu,
        fwemptc::Empipu,
        Fwemptc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fwemptc::Empipu,
            fwemptc::Empipu,
            Fwemptc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet Mirroring Path Port Select"]
    #[inline(always)]
    pub fn empps(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fwemptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fwemptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ethernet Mirroring Path Security Level"]
    #[inline(always)]
    pub fn empsl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwemptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fwemptc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwemptc {
    #[inline(always)]
    fn default() -> Fwemptc {
        <crate::RegValueT<Fwemptc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwemptc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Empipu_SPEC;
    pub type Empipu = crate::EnumBitfieldStruct<u8, Empipu_SPEC>;
    impl Empipu {
        #[doc = "Frames mirrored to Ethernet path are sent with the same priority as for forwarding."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames mirrored to Ethernet path are sent with FWEMPTC.EMPIPV priority."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwsdmptc_SPEC;
impl crate::sealed::RegSpec for Fwsdmptc_SPEC {
    type DataType = u32;
}

#[doc = "Source-Destination Mirroring Path Target Configuration Register"]
pub type Fwsdmptc = crate::RegValueT<Fwsdmptc_SPEC>;

impl Fwsdmptc {
    #[doc = "Source-Destination Mirroring Path CPU Sub Destination"]
    #[inline(always)]
    pub fn sdmpcsd(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwsdmptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwsdmptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Source-Destination Mirroring Path Internal Priority Value"]
    #[inline(always)]
    pub fn sdmpipv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwsdmptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwsdmptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Source-Destination Mirroring Path Internal Priority Update"]
    #[inline(always)]
    pub fn sdmpipu(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fwsdmptc::Sdmpipu,
        fwsdmptc::Sdmpipu,
        Fwsdmptc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fwsdmptc::Sdmpipu,
            fwsdmptc::Sdmpipu,
            Fwsdmptc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Source-Destination Mirroring Path Port Select"]
    #[inline(always)]
    pub fn sdmpps(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Fwsdmptc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Fwsdmptc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Source-Destination Mirroring Path Security Level"]
    #[inline(always)]
    pub fn sdmpsl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwsdmptc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Fwsdmptc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwsdmptc {
    #[inline(always)]
    fn default() -> Fwsdmptc {
        <crate::RegValueT<Fwsdmptc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwsdmptc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdmpipu_SPEC;
    pub type Sdmpipu = crate::EnumBitfieldStruct<u8, Sdmpipu_SPEC>;
    impl Sdmpipu {
        #[doc = "Frames mirrored to Source-Destination path are sent with the same priority as for forwarding."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames mirrored to Source-Destination path are sent with FWSDMPTC.SDMPIPV priority."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwsdmpvc_SPEC;
impl crate::sealed::RegSpec for Fwsdmpvc_SPEC {
    type DataType = u32;
}

#[doc = "Source-Destination Mirroring Path Vector Configuration Register"]
pub type Fwsdmpvc = crate::RegValueT<Fwsdmpvc_SPEC>;

impl Fwsdmpvc {
    #[doc = "Source-Destination Mirroring Destination Vector"]
    #[inline(always)]
    pub fn sdmdv(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwsdmpvc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwsdmpvc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Source-Destination Mirroring Source Vector"]
    #[inline(always)]
    pub fn sdmsv(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwsdmpvc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwsdmpvc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwsdmpvc {
    #[inline(always)]
    fn default() -> Fwsdmpvc {
        <crate::RegValueT<Fwsdmpvc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlbwmc_SPEC;
impl crate::sealed::RegSpec for Fwlbwmc_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Level Based Watermark Configuration Register"]
pub type Fwlbwmc = crate::RegValueT<Fwlbwmc_SPEC>;

impl Fwlbwmc {
    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr0,
        fwlbwmc::Wmclpr0,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr0,
            fwlbwmc::Wmclpr0,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr1,
        fwlbwmc::Wmclpr1,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr1,
            fwlbwmc::Wmclpr1,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr2,
        fwlbwmc::Wmclpr2,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr2,
            fwlbwmc::Wmclpr2,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr3,
        fwlbwmc::Wmclpr3,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr3,
            fwlbwmc::Wmclpr3,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr4,
        fwlbwmc::Wmclpr4,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr4,
            fwlbwmc::Wmclpr4,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr5,
        fwlbwmc::Wmclpr5,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr5,
            fwlbwmc::Wmclpr5,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr6,
        fwlbwmc::Wmclpr6,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr6,
            fwlbwmc::Wmclpr6,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr7,
        fwlbwmc::Wmclpr7,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr7,
            fwlbwmc::Wmclpr7,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr8,
        fwlbwmc::Wmclpr8,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr8,
            fwlbwmc::Wmclpr8,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr9,
        fwlbwmc::Wmclpr9,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr9,
            fwlbwmc::Wmclpr9,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr10,
        fwlbwmc::Wmclpr10,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr10,
            fwlbwmc::Wmclpr10,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr11,
        fwlbwmc::Wmclpr11,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr11,
            fwlbwmc::Wmclpr11,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr12,
        fwlbwmc::Wmclpr12,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr12,
            fwlbwmc::Wmclpr12,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr13,
        fwlbwmc::Wmclpr13,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr13,
            fwlbwmc::Wmclpr13,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr14,
        fwlbwmc::Wmclpr14,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr14,
            fwlbwmc::Wmclpr14,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmclpr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fwlbwmc::Wmclpr15,
        fwlbwmc::Wmclpr15,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fwlbwmc::Wmclpr15,
            fwlbwmc::Wmclpr15,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr0,
        fwlbwmc::Wmflpr0,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr0,
            fwlbwmc::Wmflpr0,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr1,
        fwlbwmc::Wmflpr1,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr1,
            fwlbwmc::Wmflpr1,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr2,
        fwlbwmc::Wmflpr2,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr2,
            fwlbwmc::Wmflpr2,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr3,
        fwlbwmc::Wmflpr3,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr3,
            fwlbwmc::Wmflpr3,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr4,
        fwlbwmc::Wmflpr4,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr4,
            fwlbwmc::Wmflpr4,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr5,
        fwlbwmc::Wmflpr5,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr5,
            fwlbwmc::Wmflpr5,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr6,
        fwlbwmc::Wmflpr6,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr6,
            fwlbwmc::Wmflpr6,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr7,
        fwlbwmc::Wmflpr7,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr7,
            fwlbwmc::Wmflpr7,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr8(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr8,
        fwlbwmc::Wmflpr8,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr8,
            fwlbwmc::Wmflpr8,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr9(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr9,
        fwlbwmc::Wmflpr9,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr9,
            fwlbwmc::Wmflpr9,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr10(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr10,
        fwlbwmc::Wmflpr10,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr10,
            fwlbwmc::Wmflpr10,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr11(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr11,
        fwlbwmc::Wmflpr11,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr11,
            fwlbwmc::Wmflpr11,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr12(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr12,
        fwlbwmc::Wmflpr12,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr12,
            fwlbwmc::Wmflpr12,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr13(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr13,
        fwlbwmc::Wmflpr13,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr13,
            fwlbwmc::Wmflpr13,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr14(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr14,
        fwlbwmc::Wmflpr14,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr14,
            fwlbwmc::Wmflpr14,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Level Priority n Reject (n = 0 to 15)"]
    #[inline(always)]
    pub fn wmflpr15(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        fwlbwmc::Wmflpr15,
        fwlbwmc::Wmflpr15,
        Fwlbwmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            fwlbwmc::Wmflpr15,
            fwlbwmc::Wmflpr15,
            Fwlbwmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlbwmc {
    #[inline(always)]
    fn default() -> Fwlbwmc {
        <crate::RegValueT<Fwlbwmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwlbwmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr0_SPEC;
    pub type Wmclpr0 = crate::EnumBitfieldStruct<u8, Wmclpr0_SPEC>;
    impl Wmclpr0 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr1_SPEC;
    pub type Wmclpr1 = crate::EnumBitfieldStruct<u8, Wmclpr1_SPEC>;
    impl Wmclpr1 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr2_SPEC;
    pub type Wmclpr2 = crate::EnumBitfieldStruct<u8, Wmclpr2_SPEC>;
    impl Wmclpr2 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr3_SPEC;
    pub type Wmclpr3 = crate::EnumBitfieldStruct<u8, Wmclpr3_SPEC>;
    impl Wmclpr3 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr4_SPEC;
    pub type Wmclpr4 = crate::EnumBitfieldStruct<u8, Wmclpr4_SPEC>;
    impl Wmclpr4 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr5_SPEC;
    pub type Wmclpr5 = crate::EnumBitfieldStruct<u8, Wmclpr5_SPEC>;
    impl Wmclpr5 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr6_SPEC;
    pub type Wmclpr6 = crate::EnumBitfieldStruct<u8, Wmclpr6_SPEC>;
    impl Wmclpr6 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr7_SPEC;
    pub type Wmclpr7 = crate::EnumBitfieldStruct<u8, Wmclpr7_SPEC>;
    impl Wmclpr7 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr8_SPEC;
    pub type Wmclpr8 = crate::EnumBitfieldStruct<u8, Wmclpr8_SPEC>;
    impl Wmclpr8 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr9_SPEC;
    pub type Wmclpr9 = crate::EnumBitfieldStruct<u8, Wmclpr9_SPEC>;
    impl Wmclpr9 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr10_SPEC;
    pub type Wmclpr10 = crate::EnumBitfieldStruct<u8, Wmclpr10_SPEC>;
    impl Wmclpr10 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr11_SPEC;
    pub type Wmclpr11 = crate::EnumBitfieldStruct<u8, Wmclpr11_SPEC>;
    impl Wmclpr11 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr12_SPEC;
    pub type Wmclpr12 = crate::EnumBitfieldStruct<u8, Wmclpr12_SPEC>;
    impl Wmclpr12 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr13_SPEC;
    pub type Wmclpr13 = crate::EnumBitfieldStruct<u8, Wmclpr13_SPEC>;
    impl Wmclpr13 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr14_SPEC;
    pub type Wmclpr14 = crate::EnumBitfieldStruct<u8, Wmclpr14_SPEC>;
    impl Wmclpr14 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmclpr15_SPEC;
    pub type Wmclpr15 = crate::EnumBitfieldStruct<u8, Wmclpr15_SPEC>;
    impl Wmclpr15 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark critical level is set for port i (WM.CREITICAL\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr0_SPEC;
    pub type Wmflpr0 = crate::EnumBitfieldStruct<u8, Wmflpr0_SPEC>;
    impl Wmflpr0 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr1_SPEC;
    pub type Wmflpr1 = crate::EnumBitfieldStruct<u8, Wmflpr1_SPEC>;
    impl Wmflpr1 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr2_SPEC;
    pub type Wmflpr2 = crate::EnumBitfieldStruct<u8, Wmflpr2_SPEC>;
    impl Wmflpr2 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr3_SPEC;
    pub type Wmflpr3 = crate::EnumBitfieldStruct<u8, Wmflpr3_SPEC>;
    impl Wmflpr3 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr4_SPEC;
    pub type Wmflpr4 = crate::EnumBitfieldStruct<u8, Wmflpr4_SPEC>;
    impl Wmflpr4 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr5_SPEC;
    pub type Wmflpr5 = crate::EnumBitfieldStruct<u8, Wmflpr5_SPEC>;
    impl Wmflpr5 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr6_SPEC;
    pub type Wmflpr6 = crate::EnumBitfieldStruct<u8, Wmflpr6_SPEC>;
    impl Wmflpr6 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr7_SPEC;
    pub type Wmflpr7 = crate::EnumBitfieldStruct<u8, Wmflpr7_SPEC>;
    impl Wmflpr7 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr8_SPEC;
    pub type Wmflpr8 = crate::EnumBitfieldStruct<u8, Wmflpr8_SPEC>;
    impl Wmflpr8 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr9_SPEC;
    pub type Wmflpr9 = crate::EnumBitfieldStruct<u8, Wmflpr9_SPEC>;
    impl Wmflpr9 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr10_SPEC;
    pub type Wmflpr10 = crate::EnumBitfieldStruct<u8, Wmflpr10_SPEC>;
    impl Wmflpr10 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr11_SPEC;
    pub type Wmflpr11 = crate::EnumBitfieldStruct<u8, Wmflpr11_SPEC>;
    impl Wmflpr11 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr12_SPEC;
    pub type Wmflpr12 = crate::EnumBitfieldStruct<u8, Wmflpr12_SPEC>;
    impl Wmflpr12 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr13_SPEC;
    pub type Wmflpr13 = crate::EnumBitfieldStruct<u8, Wmflpr13_SPEC>;
    impl Wmflpr13 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr14_SPEC;
    pub type Wmflpr14 = crate::EnumBitfieldStruct<u8, Wmflpr14_SPEC>;
    impl Wmflpr14 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmflpr15_SPEC;
    pub type Wmflpr15 = crate::EnumBitfieldStruct<u8, Wmflpr15_SPEC>;
    impl Wmflpr15 {
        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are forwarded."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames coming for port i and that should be forwarded with {DEI,IPV} equal to j are discarded when watermark flush level is set for port i (WM.FLUSH\\[i\\] is set \\[COMA\\])."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpc0_SPEC;
impl crate::sealed::RegSpec for Fwpc0_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Configuration Register 0"]
pub type Fwpc0 = crate::RegValueT<Fwpc0_SPEC>;

impl Fwpc0 {
    #[doc = "L3 Table Active"]
    #[inline(always)]
    pub fn lthta(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwpc0::Lthta,
        fwpc0::Lthta,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwpc0::Lthta,
            fwpc0::Lthta,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "L3 Reject Unknown Streams"]
    #[inline(always)]
    pub fn lthrus(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwpc0::Lthrus,
        fwpc0::Lthrus,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwpc0::Lthrus,
            fwpc0::Lthrus,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "L3 Reject Unknown Secure Streams"]
    #[inline(always)]
    pub fn lthruss(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwpc0::Lthruss,
        fwpc0::Lthruss,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwpc0::Lthruss,
            fwpc0::Lthruss,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 UDP Enable"]
    #[inline(always)]
    pub fn ip4ue(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwpc0::Ip4Ue,
        fwpc0::Ip4Ue,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwpc0::Ip4Ue,
            fwpc0::Ip4Ue,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 TCP Enable"]
    #[inline(always)]
    pub fn ip4te(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwpc0::Ip4Te,
        fwpc0::Ip4Te,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwpc0::Ip4Te,
            fwpc0::Ip4Te,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Other Enable"]
    #[inline(always)]
    pub fn ip4oe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwpc0::Ip4Oe,
        fwpc0::Ip4Oe,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwpc0::Ip4Oe,
            fwpc0::Ip4Oe,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 UDP Enable"]
    #[inline(always)]
    pub fn ip6ue(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwpc0::Ip6Ue,
        fwpc0::Ip6Ue,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwpc0::Ip6Ue,
            fwpc0::Ip6Ue,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 TCP Enable"]
    #[inline(always)]
    pub fn ip6te(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwpc0::Ip6Te,
        fwpc0::Ip6Te,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwpc0::Ip6Te,
            fwpc0::Ip6Te,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Other Enable"]
    #[inline(always)]
    pub fn ip6oe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwpc0::Ip6Oe,
        fwpc0::Ip6Oe,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwpc0::Ip6Oe,
            fwpc0::Ip6Oe,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "L2 Stream Enable"]
    #[inline(always)]
    pub fn l2se(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fwpc0::L2Se,
        fwpc0::L2Se,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fwpc0::L2Se,
            fwpc0::L2Se,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Destination Search Active"]
    #[inline(always)]
    pub fn macdsa(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fwpc0::Macdsa,
        fwpc0::Macdsa,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fwpc0::Macdsa,
            fwpc0::Macdsa,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Reject Unknown Destination Addresses"]
    #[inline(always)]
    pub fn macruda(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fwpc0::Macruda,
        fwpc0::Macruda,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fwpc0::Macruda,
            fwpc0::Macruda,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Reject Unknown Destination Secure Addresses"]
    #[inline(always)]
    pub fn macrudsa(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        fwpc0::Macrudsa,
        fwpc0::Macrudsa,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            fwpc0::Macrudsa,
            fwpc0::Macrudsa,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Source Search Active"]
    #[inline(always)]
    pub fn macssa(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fwpc0::Macssa,
        fwpc0::Macssa,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fwpc0::Macssa,
            fwpc0::Macssa,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Reject Unknown Source Addresses"]
    #[inline(always)]
    pub fn macrusa(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        fwpc0::Macrusa,
        fwpc0::Macrusa,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            fwpc0::Macrusa,
            fwpc0::Macrusa,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Reject Unknown Source Secure Addresses"]
    #[inline(always)]
    pub fn macrussa(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        fwpc0::Macrussa,
        fwpc0::Macrussa,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            fwpc0::Macrussa,
            fwpc0::Macrussa,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Hardware Learning Active"]
    #[inline(always)]
    pub fn machla(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        fwpc0::Machla,
        fwpc0::Machla,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            fwpc0::Machla,
            fwpc0::Machla,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Hardware Migration Active"]
    #[inline(always)]
    pub fn machma(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        fwpc0::Machma,
        fwpc0::Machma,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            fwpc0::Machma,
            fwpc0::Machma,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Search Active"]
    #[inline(always)]
    pub fn vlansa(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        fwpc0::Vlansa,
        fwpc0::Vlansa,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            fwpc0::Vlansa,
            fwpc0::Vlansa,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Reject Unknown"]
    #[inline(always)]
    pub fn vlanru(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        fwpc0::Vlanru,
        fwpc0::Vlanru,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            fwpc0::Vlanru,
            fwpc0::Vlanru,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Reject Unknown Secure"]
    #[inline(always)]
    pub fn vlanrus(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        fwpc0::Vlanrus,
        fwpc0::Vlanrus,
        Fwpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            fwpc0::Vlanrus,
            fwpc0::Vlanrus,
            Fwpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwpc0 {
    #[inline(always)]
    fn default() -> Fwpc0 {
        <crate::RegValueT<Fwpc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwpc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthta_SPEC;
    pub type Lthta = crate::EnumBitfieldStruct<u8, Lthta_SPEC>;
    impl Lthta {
        #[doc = "L3 table is disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "L3 table is enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthrus_SPEC;
    pub type Lthrus = crate::EnumBitfieldStruct<u8, Lthrus_SPEC>;
    impl Lthrus {
        #[doc = "Unknown streams coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown streams coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthruss_SPEC;
    pub type Lthruss = crate::EnumBitfieldStruct<u8, Lthruss_SPEC>;
    impl Lthruss {
        #[doc = "Unknown secure streams coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown secure streams coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Ue_SPEC;
    pub type Ip4Ue = crate::EnumBitfieldStruct<u8, Ip4Ue_SPEC>;
    impl Ip4Ue {
        #[doc = "IPv4/UDP detection disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv4/UDP detection enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Te_SPEC;
    pub type Ip4Te = crate::EnumBitfieldStruct<u8, Ip4Te_SPEC>;
    impl Ip4Te {
        #[doc = "IPv4/TCP detection disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv4/TCP detection enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Oe_SPEC;
    pub type Ip4Oe = crate::EnumBitfieldStruct<u8, Ip4Oe_SPEC>;
    impl Ip4Oe {
        #[doc = "non-TCP non-UDP IPv4 detection disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "non-TCP non-UDP IPv4 detection enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Ue_SPEC;
    pub type Ip6Ue = crate::EnumBitfieldStruct<u8, Ip6Ue_SPEC>;
    impl Ip6Ue {
        #[doc = "IPv6/UDP detection disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6/UDP detection enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Te_SPEC;
    pub type Ip6Te = crate::EnumBitfieldStruct<u8, Ip6Te_SPEC>;
    impl Ip6Te {
        #[doc = "IPv6/TCP detection disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6/TCP detection enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Oe_SPEC;
    pub type Ip6Oe = crate::EnumBitfieldStruct<u8, Ip6Oe_SPEC>;
    impl Ip6Oe {
        #[doc = "non-TCP non-UDP IPv6 detection disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "non-TCP non-UDP IPv6 detection enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Se_SPEC;
    pub type L2Se = crate::EnumBitfieldStruct<u8, L2Se_SPEC>;
    impl L2Se {
        #[doc = "L2 stream disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "L2 stream enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macdsa_SPEC;
    pub type Macdsa = crate::EnumBitfieldStruct<u8, Macdsa_SPEC>;
    impl Macdsa {
        #[doc = "MAC Destination search is disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Destination search is enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macruda_SPEC;
    pub type Macruda = crate::EnumBitfieldStruct<u8, Macruda_SPEC>;
    impl Macruda {
        #[doc = "Unknown MAC Destination addresses coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown MAC Destination addresses coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macrudsa_SPEC;
    pub type Macrudsa = crate::EnumBitfieldStruct<u8, Macrudsa_SPEC>;
    impl Macrudsa {
        #[doc = "Unknown secure MAC Destination addresses coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown secure MAC Destination addresses coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macssa_SPEC;
    pub type Macssa = crate::EnumBitfieldStruct<u8, Macssa_SPEC>;
    impl Macssa {
        #[doc = "MAC Source search is disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Source search is enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macrusa_SPEC;
    pub type Macrusa = crate::EnumBitfieldStruct<u8, Macrusa_SPEC>;
    impl Macrusa {
        #[doc = "Unknown MAC Source addresses coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown MAC Source addresses coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macrussa_SPEC;
    pub type Macrussa = crate::EnumBitfieldStruct<u8, Macrussa_SPEC>;
    impl Macrussa {
        #[doc = "Unknown secure MAC Source addresses coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown secure MAC Source addresses coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Machla_SPEC;
    pub type Machla = crate::EnumBitfieldStruct<u8, Machla_SPEC>;
    impl Machla {
        #[doc = "Unknown unicast MAC Source addresses coming from source port i are not added to the MAC table."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown unicast MAC Source addresses coming from source port i are added to the MAC table."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Machma_SPEC;
    pub type Machma = crate::EnumBitfieldStruct<u8, Machma_SPEC>;
    impl Machma {
        #[doc = "Known unicast MAC Source addresses coming from source port i with a wrong source port in MAC.DV are not overwritten."]
        pub const _0: Self = Self::new(0);

        #[doc = "Known unicast MAC Source addresses coming from source port i with a wrong source port in MAC.DV are overwritten."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlansa_SPEC;
    pub type Vlansa = crate::EnumBitfieldStruct<u8, Vlansa_SPEC>;
    impl Vlansa {
        #[doc = "VLAN search is disabled for source port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "VLAN search is enabled for source port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlanru_SPEC;
    pub type Vlanru = crate::EnumBitfieldStruct<u8, Vlanru_SPEC>;
    impl Vlanru {
        #[doc = "Unknown VLAN coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown VLAN coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlanrus_SPEC;
    pub type Vlanrus = crate::EnumBitfieldStruct<u8, Vlanrus_SPEC>;
    impl Vlanrus {
        #[doc = "Unknown secure VLAN coming from source port i are not rejected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Unknown secure VLAN coming from source port i are rejected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpc1_SPEC;
impl crate::sealed::RegSpec for Fwpc1_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Configuration Register 1"]
pub type Fwpc1 = crate::RegValueT<Fwpc1_SPEC>;

impl Fwpc1 {
    #[doc = "Direct Descriptor Enable"]
    #[inline(always)]
    pub fn dde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwpc1::Dde,
        fwpc1::Dde,
        Fwpc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwpc1::Dde,
            fwpc1::Dde,
            Fwpc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor Security Level"]
    #[inline(always)]
    pub fn ddsl(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwpc1::Ddsl,
        fwpc1::Ddsl,
        Fwpc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwpc1::Ddsl,
            fwpc1::Ddsl,
            Fwpc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 Forwarding Mask n (n = 0 to 2)"]
    #[inline(always)]
    pub fn lthfm0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwpc1::Lthfm0,
        fwpc1::Lthfm0,
        Fwpc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwpc1::Lthfm0,
            fwpc1::Lthfm0,
            Fwpc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 Forwarding Mask n (n = 0 to 2)"]
    #[inline(always)]
    pub fn lthfm1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwpc1::Lthfm1,
        fwpc1::Lthfm1,
        Fwpc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwpc1::Lthfm1,
            fwpc1::Lthfm1,
            Fwpc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 Forwarding Mask n (n = 0 to 2)"]
    #[inline(always)]
    pub fn lthfm2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fwpc1::Lthfm2,
        fwpc1::Lthfm2,
        Fwpc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fwpc1::Lthfm2,
            fwpc1::Lthfm2,
            Fwpc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwpc1 {
    #[inline(always)]
    fn default() -> Fwpc1 {
        <crate::RegValueT<Fwpc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwpc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dde_SPEC;
    pub type Dde = crate::EnumBitfieldStruct<u8, Dde_SPEC>;
    impl Dde {
        #[doc = "Direct Descriptor disabled for port i"]
        pub const _0: Self = Self::new(0);

        #[doc = "Direct Descriptor enabled for port i"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddsl_SPEC;
    pub type Ddsl = crate::EnumBitfieldStruct<u8, Ddsl_SPEC>;
    impl Ddsl {
        #[doc = "Secured Direct Descriptor discarded for port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "Secured Direct Descriptor forwarded for port i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthfm0_SPEC;
    pub type Lthfm0 = crate::EnumBitfieldStruct<u8, Lthfm0_SPEC>;
    impl Lthfm0 {
        #[doc = "Frames forwarded by Layer 3 forwarding can be forwarded to port j."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames forwarded by Layer 3 forwarding cannot be forwarded to port j."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthfm1_SPEC;
    pub type Lthfm1 = crate::EnumBitfieldStruct<u8, Lthfm1_SPEC>;
    impl Lthfm1 {
        #[doc = "Frames forwarded by Layer 3 forwarding can be forwarded to port j."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames forwarded by Layer 3 forwarding cannot be forwarded to port j."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthfm2_SPEC;
    pub type Lthfm2 = crate::EnumBitfieldStruct<u8, Lthfm2_SPEC>;
    impl Lthfm2 {
        #[doc = "Frames forwarded by Layer 3 forwarding can be forwarded to port j."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames forwarded by Layer 3 forwarding cannot be forwarded to port j."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpc2_SPEC;
impl crate::sealed::RegSpec for Fwpc2_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Configuration Register 2"]
pub type Fwpc2 = crate::RegValueT<Fwpc2_SPEC>;

impl Fwpc2 {
    #[doc = "Layer 2 Forwarding Mask n (n = 0 to 2)"]
    #[inline(always)]
    pub fn ltwfm0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwpc2::Ltwfm0,
        fwpc2::Ltwfm0,
        Fwpc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwpc2::Ltwfm0,
            fwpc2::Ltwfm0,
            Fwpc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Forwarding Mask n (n = 0 to 2)"]
    #[inline(always)]
    pub fn ltwfm1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwpc2::Ltwfm1,
        fwpc2::Ltwfm1,
        Fwpc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwpc2::Ltwfm1,
            fwpc2::Ltwfm1,
            Fwpc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Forwarding Mask n (n = 0 to 2)"]
    #[inline(always)]
    pub fn ltwfm2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fwpc2::Ltwfm2,
        fwpc2::Ltwfm2,
        Fwpc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fwpc2::Ltwfm2,
            fwpc2::Ltwfm2,
            Fwpc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwpc2 {
    #[inline(always)]
    fn default() -> Fwpc2 {
        <crate::RegValueT<Fwpc2_SPEC> as RegisterValue<_>>::new(8323072)
    }
}
pub mod fwpc2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwfm0_SPEC;
    pub type Ltwfm0 = crate::EnumBitfieldStruct<u8, Ltwfm0_SPEC>;
    impl Ltwfm0 {
        #[doc = "Frames forwarded by Layer 2 forwarding can be forwarded to port j."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames forwarded by Layer 2 forwarding cannot be forwarded to port j."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwfm1_SPEC;
    pub type Ltwfm1 = crate::EnumBitfieldStruct<u8, Ltwfm1_SPEC>;
    impl Ltwfm1 {
        #[doc = "Frames forwarded by Layer 2 forwarding can be forwarded to port j."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames forwarded by Layer 2 forwarding cannot be forwarded to port j."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwfm2_SPEC;
    pub type Ltwfm2 = crate::EnumBitfieldStruct<u8, Ltwfm2_SPEC>;
    impl Ltwfm2 {
        #[doc = "Frames forwarded by Layer 2 forwarding can be forwarded to port j."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frames forwarded by Layer 2 forwarding cannot be forwarded to port j."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctgc0_SPEC;
impl crate::sealed::RegSpec for Fwctgc0_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through General Configuration Register i0"]
pub type Fwctgc0 = crate::RegValueT<Fwctgc0_SPEC>;

impl Fwctgc0 {
    #[doc = "Cut-Through MAC Destination Enable"]
    #[inline(always)]
    pub fn ctmde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwctgc0::Ctmde,
        fwctgc0::Ctmde,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwctgc0::Ctmde,
            fwctgc0::Ctmde,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through MAC Source Enable"]
    #[inline(always)]
    pub fn ctmse(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwctgc0::Ctmse,
        fwctgc0::Ctmse,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwctgc0::Ctmse,
            fwctgc0::Ctmse,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through C-TAG VLAN Enable"]
    #[inline(always)]
    pub fn ctcve(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwctgc0::Ctcve,
        fwctgc0::Ctcve,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwctgc0::Ctcve,
            fwctgc0::Ctcve,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through C-TAG PCP Enable"]
    #[inline(always)]
    pub fn ctcpe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwctgc0::Ctcpe,
        fwctgc0::Ctcpe,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwctgc0::Ctcpe,
            fwctgc0::Ctcpe,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through C-TAG DEI Enable"]
    #[inline(always)]
    pub fn ctcde(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwctgc0::Ctcde,
        fwctgc0::Ctcde,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwctgc0::Ctcde,
            fwctgc0::Ctcde,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through S-TAG VLAN Enable"]
    #[inline(always)]
    pub fn ctsve(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwctgc0::Ctsve,
        fwctgc0::Ctsve,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwctgc0::Ctsve,
            fwctgc0::Ctsve,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through S-TAG PCP Enable"]
    #[inline(always)]
    pub fn ctspe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwctgc0::Ctspe,
        fwctgc0::Ctspe,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwctgc0::Ctspe,
            fwctgc0::Ctspe,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through S-TAG DEI Enable"]
    #[inline(always)]
    pub fn ctsde(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwctgc0::Ctsde,
        fwctgc0::Ctsde,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwctgc0::Ctsde,
            fwctgc0::Ctsde,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through Ethernet Type Enable"]
    #[inline(always)]
    pub fn ctete(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwctgc0::Ctete,
        fwctgc0::Ctete,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwctgc0::Ctete,
            fwctgc0::Ctete,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through FCS In"]
    #[inline(always)]
    pub fn ctfi(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fwctgc0::Ctfi,
        fwctgc0::Ctfi,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fwctgc0::Ctfi,
            fwctgc0::Ctfi,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through VLAN Control \\[GWCA\\] \\[ETHA\\]"]
    #[inline(always)]
    pub fn ctvctrl(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        fwctgc0::Ctvctrl,
        fwctgc0::Ctvctrl,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            fwctgc0::Ctvctrl,
            fwctgc0::Ctvctrl,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through R-TAG In \\[GWCA\\] \\[ETHA\\]"]
    #[inline(always)]
    pub fn ctrtgi(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        fwctgc0::Ctrtgi,
        fwctgc0::Ctrtgi,
        Fwctgc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            fwctgc0::Ctrtgi,
            fwctgc0::Ctrtgi,
            Fwctgc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwctgc0 {
    #[inline(always)]
    fn default() -> Fwctgc0 {
        <crate::RegValueT<Fwctgc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwctgc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctmde_SPEC;
    pub type Ctmde = crate::EnumBitfieldStruct<u8, Ctmde_SPEC>;
    impl Ctmde {
        #[doc = "MAC destination not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC destination included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctmse_SPEC;
    pub type Ctmse = crate::EnumBitfieldStruct<u8, Ctmse_SPEC>;
    impl Ctmse {
        #[doc = "MAC Source not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Source included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctcve_SPEC;
    pub type Ctcve = crate::EnumBitfieldStruct<u8, Ctcve_SPEC>;
    impl Ctcve {
        #[doc = "C-TAG VLAN not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG VLAN included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctcpe_SPEC;
    pub type Ctcpe = crate::EnumBitfieldStruct<u8, Ctcpe_SPEC>;
    impl Ctcpe {
        #[doc = "C-TAG PCP not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG PCP included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctcde_SPEC;
    pub type Ctcde = crate::EnumBitfieldStruct<u8, Ctcde_SPEC>;
    impl Ctcde {
        #[doc = "C-TAG DEI not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG DEI included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsve_SPEC;
    pub type Ctsve = crate::EnumBitfieldStruct<u8, Ctsve_SPEC>;
    impl Ctsve {
        #[doc = "S-TAG VLAN not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG VLAN included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspe_SPEC;
    pub type Ctspe = crate::EnumBitfieldStruct<u8, Ctspe_SPEC>;
    impl Ctspe {
        #[doc = "S-TAG PCP not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG PCP included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsde_SPEC;
    pub type Ctsde = crate::EnumBitfieldStruct<u8, Ctsde_SPEC>;
    impl Ctsde {
        #[doc = "S-TAG DEI not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG DEI included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctete_SPEC;
    pub type Ctete = crate::EnumBitfieldStruct<u8, Ctete_SPEC>;
    impl Ctete {
        #[doc = "Ethernet Type not included in Cut-Through separation for separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "Ethernet Type included in Cut-Through separation for separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctfi_SPEC;
    pub type Ctfi = crate::EnumBitfieldStruct<u8, Ctfi_SPEC>;
    impl Ctfi {
        #[doc = "The frame matching separation rule i as no FCS included."]
        pub const _0: Self = Self::new(0);

        #[doc = "The frame matching separation rule i as an FCS included."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctvctrl_SPEC;
    pub type Ctvctrl = crate::EnumBitfieldStruct<u8, Ctvctrl_SPEC>;
    impl Ctvctrl {
        #[doc = "The ingress matching separation rule i frame is a No TAG frame."]
        pub const _00: Self = Self::new(0);

        #[doc = "The ingress matching separation rule i frame is a C-TAG frame."]
        pub const _01: Self = Self::new(1);

        #[doc = "The ingress matching separation rule i frame is a SC-TAG frame."]
        pub const _10: Self = Self::new(2);

        #[doc = "The ingress matching separation rule i frame is a CoS TAG frame."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrtgi_SPEC;
    pub type Ctrtgi = crate::EnumBitfieldStruct<u8, Ctrtgi_SPEC>;
    impl Ctrtgi {
        #[doc = "R-TAG is not included in cut-through frame."]
        pub const _0: Self = Self::new(0);

        #[doc = "R-TAG is included in cut-through frame."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctgc1_SPEC;
impl crate::sealed::RegSpec for Fwctgc1_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through General Configuration Register i1"]
pub type Fwctgc1 = crate::RegValueT<Fwctgc1_SPEC>;

impl Fwctgc1 {
    #[doc = "Cut-Through Maximum time"]
    #[inline(always)]
    pub fn ctmt(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffffff, 1, 0, u32, u32, Fwctgc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffffff,1,0,u32,u32,Fwctgc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwctgc1 {
    #[inline(always)]
    fn default() -> Fwctgc1 {
        <crate::RegValueT<Fwctgc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcttc0_SPEC;
impl crate::sealed::RegSpec for Fwcttc0_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Target Configuration Register i0"]
pub type Fwcttc0 = crate::RegValueT<Fwcttc0_SPEC>;

impl Fwcttc0 {
    #[doc = "Cut-through Destination Vector"]
    #[inline(always)]
    pub fn ctdv(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwcttc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwcttc0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through Destination n Forwarding Mode (n = 0 to 1)"]
    #[inline(always)]
    pub fn ctdfm0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwcttc0::Ctdfm0,
        fwcttc0::Ctdfm0,
        Fwcttc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwcttc0::Ctdfm0,
            fwcttc0::Ctdfm0,
            Fwcttc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-Through Destination n Forwarding Mode (n = 0 to 1)"]
    #[inline(always)]
    pub fn ctdfm1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwcttc0::Ctdfm1,
        fwcttc0::Ctdfm1,
        Fwcttc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwcttc0::Ctdfm1,
            fwcttc0::Ctdfm1,
            Fwcttc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwcttc0 {
    #[inline(always)]
    fn default() -> Fwcttc0 {
        <crate::RegValueT<Fwcttc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwcttc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctdfm0_SPEC;
    pub type Ctdfm0 = crate::EnumBitfieldStruct<u8, Ctdfm0_SPEC>;
    impl Ctdfm0 {
        #[doc = "Frame is forwarded in Cut-through mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame is forwarded in Store and forward mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctdfm1_SPEC;
    pub type Ctdfm1 = crate::EnumBitfieldStruct<u8, Ctdfm1_SPEC>;
    impl Ctdfm1 {
        #[doc = "Frame is forwarded in Cut-through mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame is forwarded in Store and forward mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcttc1_SPEC;
impl crate::sealed::RegSpec for Fwcttc1_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Target Configuration Register i1"]
pub type Fwcttc1 = crate::RegValueT<Fwcttc1_SPEC>;

impl Fwcttc1 {
    #[doc = "Cut-through Internal Priority Value"]
    #[inline(always)]
    pub fn ctipv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwcttc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwcttc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-through Internal Priority Update"]
    #[inline(always)]
    pub fn ctipu(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fwcttc1::Ctipu,
        fwcttc1::Ctipu,
        Fwcttc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fwcttc1::Ctipu,
            fwcttc1::Ctipu,
            Fwcttc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-through CPU Mirroring Enable"]
    #[inline(always)]
    pub fn ctcme(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwcttc1::Ctcme,
        fwcttc1::Ctcme,
        Fwcttc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwcttc1::Ctcme,
            fwcttc1::Ctcme,
            Fwcttc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cut-through Ethernet Mirroring Enable"]
    #[inline(always)]
    pub fn cteme(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwcttc1::Cteme,
        fwcttc1::Cteme,
        Fwcttc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwcttc1::Cteme,
            fwcttc1::Cteme,
            Fwcttc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwcttc1 {
    #[inline(always)]
    fn default() -> Fwcttc1 {
        <crate::RegValueT<Fwcttc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwcttc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctipu_SPEC;
    pub type Ctipu = crate::EnumBitfieldStruct<u8, Ctipu_SPEC>;
    impl Ctipu {
        #[doc = "Use the output descriptor priority to forwarding frames matching separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "Use FWCTTCi1.CTIPVi priority to forwarding frames matching separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctcme_SPEC;
    pub type Ctcme = crate::EnumBitfieldStruct<u8, Ctcme_SPEC>;
    impl Ctcme {
        #[doc = "CPU mirroring is disabled for frames matching separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU mirroring is enabled for frames matching separation rule i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cteme_SPEC;
    pub type Cteme = crate::EnumBitfieldStruct<u8, Cteme_SPEC>;
    impl Cteme {
        #[doc = "Ethernet mirroring is disabled for frames matching separation rule i."]
        pub const _0: Self = Self::new(0);

        #[doc = "Ethernet mirroring is enabled for frames matching separation rule i."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcttc20_SPEC;
impl crate::sealed::RegSpec for Fwcttc20_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Target Configuration Register i20"]
pub type Fwcttc20 = crate::RegValueT<Fwcttc20_SPEC>;

impl Fwcttc20 {
    #[doc = "Cut-Through CPU Sub Destination"]
    #[inline(always)]
    pub fn ctcsd(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwcttc20_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwcttc20_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwcttc20 {
    #[inline(always)]
    fn default() -> Fwcttc20 {
        <crate::RegValueT<Fwcttc20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctsc0_SPEC;
impl crate::sealed::RegSpec for Fwctsc0_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Separation Configuration Register i0"]
pub type Fwctsc0 = crate::RegValueT<Fwctsc0_SPEC>;

impl Fwctsc0 {
    #[doc = "Cut-Through Destination MAC Address Upper Part"]
    #[inline(always)]
    pub fn ctdmau(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwctsc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwctsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwctsc0 {
    #[inline(always)]
    fn default() -> Fwctsc0 {
        <crate::RegValueT<Fwctsc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctsc1_SPEC;
impl crate::sealed::RegSpec for Fwctsc1_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Separation Configuration Register i1"]
pub type Fwctsc1 = crate::RegValueT<Fwctsc1_SPEC>;

impl Fwctsc1 {
    #[doc = "Cut-Through Source MAC Address Upper Part"]
    #[inline(always)]
    pub fn ctsmau(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwctsc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwctsc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through Destination MAC Address Lower Part"]
    #[inline(always)]
    pub fn ctdmal(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Fwctsc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Fwctsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwctsc1 {
    #[inline(always)]
    fn default() -> Fwctsc1 {
        <crate::RegValueT<Fwctsc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctsc2_SPEC;
impl crate::sealed::RegSpec for Fwctsc2_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Separation Configuration Register i2"]
pub type Fwctsc2 = crate::RegValueT<Fwctsc2_SPEC>;

impl Fwctsc2 {
    #[doc = "Cut-Through Source MAC Address Lower Part"]
    #[inline(always)]
    pub fn ctsmal(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwctsc2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwctsc2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwctsc2 {
    #[inline(always)]
    fn default() -> Fwctsc2 {
        <crate::RegValueT<Fwctsc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctsc3_SPEC;
impl crate::sealed::RegSpec for Fwctsc3_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Separation Configuration Register i3"]
pub type Fwctsc3 = crate::RegValueT<Fwctsc3_SPEC>;

impl Fwctsc3 {
    #[doc = "Cut-Through C-TAG VLAN"]
    #[inline(always)]
    pub fn ctcv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Fwctsc3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Fwctsc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through C-TAG PCP"]
    #[inline(always)]
    pub fn ctcp(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwctsc3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwctsc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through C-TAG DEI"]
    #[inline(always)]
    pub fn ctcd(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwctsc3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwctsc3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Cut-Through S-TAG VLAN"]
    #[inline(always)]
    pub fn ctsv(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Fwctsc3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Fwctsc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through S-TAG PCP"]
    #[inline(always)]
    pub fn ctsp(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Fwctsc3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Fwctsc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through S-TAG DEI"]
    #[inline(always)]
    pub fn ctsd(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwctsc3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fwctsc3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwctsc3 {
    #[inline(always)]
    fn default() -> Fwctsc3 {
        <crate::RegValueT<Fwctsc3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctsc4_SPEC;
impl crate::sealed::RegSpec for Fwctsc4_SPEC {
    type DataType = u32;
}

#[doc = "Cut-Through Separation Configuration Register i4"]
pub type Fwctsc4 = crate::RegValueT<Fwctsc4_SPEC>;

impl Fwctsc4 {
    #[doc = "Cut-Through Ethernet Type"]
    #[inline(always)]
    pub fn ctet(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwctsc4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwctsc4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cut-Through Source Port Number"]
    #[inline(always)]
    pub fn ctspn(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fwctsc4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fwctsc4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwctsc4 {
    #[inline(always)]
    fn default() -> Fwctsc4 {
        <crate::RegValueT<Fwctsc4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwtwbfc_SPEC;
impl crate::sealed::RegSpec for Fwtwbfc_SPEC {
    type DataType = u32;
}

#[doc = "Two-Byte Filter Configuration Register %s"]
pub type Fwtwbfc = crate::RegValueT<Fwtwbfc_SPEC>;

impl Fwtwbfc {
    #[doc = "Two-Byte Filter Unit Mode"]
    #[inline(always)]
    pub fn twbfum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fwtwbfc::Twbfum,
        fwtwbfc::Twbfum,
        Fwtwbfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fwtwbfc::Twbfum,
            fwtwbfc::Twbfum,
            Fwtwbfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Two-Byte Filtering Mode"]
    #[inline(always)]
    pub fn twbfm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwtwbfc::Twbfm,
        fwtwbfc::Twbfm,
        Fwtwbfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwtwbfc::Twbfm,
            fwtwbfc::Twbfm,
            Fwtwbfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Two-Byte Filter Offset Value"]
    #[inline(always)]
    pub fn twbfov(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Fwtwbfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Fwtwbfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwtwbfc {
    #[inline(always)]
    fn default() -> Fwtwbfc {
        <crate::RegValueT<Fwtwbfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwtwbfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Twbfum_SPEC;
    pub type Twbfum = crate::EnumBitfieldStruct<u8, Twbfum_SPEC>;
    impl Twbfum {
        #[doc = "Mask mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Expand mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Precise mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Twbfm_SPEC;
    pub type Twbfm = crate::EnumBitfieldStruct<u8, Twbfm_SPEC>;
    impl Twbfm {
        #[doc = "Offset filtering"]
        pub const _0: Self = Self::new(0);

        #[doc = "VLAN TAG filtering"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwtwbfvc_SPEC;
impl crate::sealed::RegSpec for Fwtwbfvc_SPEC {
    type DataType = u32;
}

#[doc = "Two-Byte Filter Value Configuration Register %s"]
pub type Fwtwbfvc = crate::RegValueT<Fwtwbfvc_SPEC>;

impl Fwtwbfvc {
    #[doc = "Two-Byte Filter Value 0"]
    #[inline(always)]
    pub fn twbfv0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwtwbfvc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwtwbfvc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Two-Byte Filter Value 1"]
    #[inline(always)]
    pub fn twbfv1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Fwtwbfvc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Fwtwbfvc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwtwbfvc {
    #[inline(always)]
    fn default() -> Fwtwbfvc {
        <crate::RegValueT<Fwtwbfvc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwthbfc_SPEC;
impl crate::sealed::RegSpec for Fwthbfc_SPEC {
    type DataType = u32;
}

#[doc = "Three-Byte Filter Configuration Register %s"]
pub type Fwthbfc = crate::RegValueT<Fwthbfc_SPEC>;

impl Fwthbfc {
    #[doc = "Three-Byte Filter Unit Mode"]
    #[inline(always)]
    pub fn thbfum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fwthbfc::Thbfum,
        fwthbfc::Thbfum,
        Fwthbfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fwthbfc::Thbfum,
            fwthbfc::Thbfum,
            Fwthbfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Three-Byte Filter Offset Value"]
    #[inline(always)]
    pub fn thbfov(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Fwthbfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Fwthbfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwthbfc {
    #[inline(always)]
    fn default() -> Fwthbfc {
        <crate::RegValueT<Fwthbfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwthbfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thbfum_SPEC;
    pub type Thbfum = crate::EnumBitfieldStruct<u8, Thbfum_SPEC>;
    impl Thbfum {
        #[doc = "Mask mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Expand mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Precise mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwthbfv0C_SPEC;
impl crate::sealed::RegSpec for Fwthbfv0C_SPEC {
    type DataType = u32;
}

#[doc = "Three-Byte Filter Value 0 Configuration Register %s"]
pub type Fwthbfv0C = crate::RegValueT<Fwthbfv0C_SPEC>;

impl Fwthbfv0C {
    #[doc = "Three-Byte Filter Value 0"]
    #[inline(always)]
    pub fn thbfv0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Fwthbfv0C_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Fwthbfv0C_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwthbfv0C {
    #[inline(always)]
    fn default() -> Fwthbfv0C {
        <crate::RegValueT<Fwthbfv0C_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwthbfv1C_SPEC;
impl crate::sealed::RegSpec for Fwthbfv1C_SPEC {
    type DataType = u32;
}

#[doc = "Three-Byte Filter Value 1 Configuration Register %s"]
pub type Fwthbfv1C = crate::RegValueT<Fwthbfv1C_SPEC>;

impl Fwthbfv1C {
    #[doc = "Three-Byte Filter Value 1"]
    #[inline(always)]
    pub fn thbfv1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Fwthbfv1C_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Fwthbfv1C_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwthbfv1C {
    #[inline(always)]
    fn default() -> Fwthbfv1C {
        <crate::RegValueT<Fwthbfv1C_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwfobfc_SPEC;
impl crate::sealed::RegSpec for Fwfobfc_SPEC {
    type DataType = u32;
}

#[doc = "Four-Byte Filter Configuration Register %s"]
pub type Fwfobfc = crate::RegValueT<Fwfobfc_SPEC>;

impl Fwfobfc {
    #[doc = "Four-Byte Filter Unit Mode"]
    #[inline(always)]
    pub fn fobfum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fwfobfc::Fobfum,
        fwfobfc::Fobfum,
        Fwfobfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fwfobfc::Fobfum,
            fwfobfc::Fobfum,
            Fwfobfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Four-Byte Filter Offset Value"]
    #[inline(always)]
    pub fn fobfov(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Fwfobfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Fwfobfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwfobfc {
    #[inline(always)]
    fn default() -> Fwfobfc {
        <crate::RegValueT<Fwfobfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwfobfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fobfum_SPEC;
    pub type Fobfum = crate::EnumBitfieldStruct<u8, Fobfum_SPEC>;
    impl Fobfum {
        #[doc = "Mask mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "Expand mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Precise mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwfobfv0C_SPEC;
impl crate::sealed::RegSpec for Fwfobfv0C_SPEC {
    type DataType = u32;
}

#[doc = "Four-Byte Filter Value 0 Configuration Register %s"]
pub type Fwfobfv0C = crate::RegValueT<Fwfobfv0C_SPEC>;

impl Fwfobfv0C {
    #[doc = "Four-Byte Filter Value 0"]
    #[inline(always)]
    pub fn fobfv0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Fwfobfv0C_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwfobfv0C_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwfobfv0C {
    #[inline(always)]
    fn default() -> Fwfobfv0C {
        <crate::RegValueT<Fwfobfv0C_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwfobfv1C_SPEC;
impl crate::sealed::RegSpec for Fwfobfv1C_SPEC {
    type DataType = u32;
}

#[doc = "Four-Byte Filter Value 1 Configuration Register %s"]
pub type Fwfobfv1C = crate::RegValueT<Fwfobfv1C_SPEC>;

impl Fwfobfv1C {
    #[doc = "Four-Byte Filter Value 1"]
    #[inline(always)]
    pub fn fobfv1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Fwfobfv1C_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwfobfv1C_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwfobfv1C {
    #[inline(always)]
    fn default() -> Fwfobfv1C {
        <crate::RegValueT<Fwfobfv1C_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwrfc_SPEC;
impl crate::sealed::RegSpec for Fwrfc_SPEC {
    type DataType = u32;
}

#[doc = "Range Filter Configuration Register %s"]
pub type Fwrfc = crate::RegValueT<Fwrfc_SPEC>;

impl Fwrfc {
    #[doc = "Range Filtering Mode"]
    #[inline(always)]
    pub fn rfm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwrfc::Rfm,
        fwrfc::Rfm,
        Fwrfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwrfc::Rfm,
            fwrfc::Rfm,
            Fwrfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Range Filter Offset Value"]
    #[inline(always)]
    pub fn rfov(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Fwrfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Fwrfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwrfc {
    #[inline(always)]
    fn default() -> Fwrfc {
        <crate::RegValueT<Fwrfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwrfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfm_SPEC;
    pub type Rfm = crate::EnumBitfieldStruct<u8, Rfm_SPEC>;
    impl Rfm {
        #[doc = "Offset filtering"]
        pub const _0: Self = Self::new(0);

        #[doc = "VLAN TAG filtering"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwrfvc_SPEC;
impl crate::sealed::RegSpec for Fwrfvc_SPEC {
    type DataType = u32;
}

#[doc = "Range Filter Value Configuration Register %s"]
pub type Fwrfvc = crate::RegValueT<Fwrfvc_SPEC>;

impl Fwrfvc {
    #[doc = "Range Filter Start Value 0"]
    #[inline(always)]
    pub fn rfsv0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwrfvc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwrfvc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Range Filter Start Value 1"]
    #[inline(always)]
    pub fn rfsv1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fwrfvc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fwrfvc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Range Filter Range Value"]
    #[inline(always)]
    pub fn rfrv(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Fwrfvc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Fwrfvc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwrfvc {
    #[inline(always)]
    fn default() -> Fwrfvc {
        <crate::RegValueT<Fwrfvc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfc_SPEC;
impl crate::sealed::RegSpec for Fwcfc_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Configuration Register %s"]
pub type Fwcfc = crate::RegValueT<Fwcfc_SPEC>;

impl Fwcfc {
    #[doc = "Cascade Filter E-Frame Filter Valid n (n = 0 to 2)"]
    #[inline(always)]
    pub fn cfeffv0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwcfc::Cfeffv0,
        fwcfc::Cfeffv0,
        Fwcfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwcfc::Cfeffv0,
            fwcfc::Cfeffv0,
            Fwcfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cascade Filter E-Frame Filter Valid n (n = 0 to 2)"]
    #[inline(always)]
    pub fn cfeffv1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwcfc::Cfeffv1,
        fwcfc::Cfeffv1,
        Fwcfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwcfc::Cfeffv1,
            fwcfc::Cfeffv1,
            Fwcfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cascade Filter E-Frame Filter Valid n (n = 0 to 2)"]
    #[inline(always)]
    pub fn cfeffv2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwcfc::Cfeffv2,
        fwcfc::Cfeffv2,
        Fwcfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwcfc::Cfeffv2,
            fwcfc::Cfeffv2,
            Fwcfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cascade Filter E-Frame Filter Valid n (n = 0 to 1)"]
    #[inline(always)]
    pub fn cfpffv0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwcfc::Cfpffv0,
        fwcfc::Cfpffv0,
        Fwcfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwcfc::Cfpffv0,
            fwcfc::Cfpffv0,
            Fwcfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cascade Filter E-Frame Filter Valid n (n = 0 to 1)"]
    #[inline(always)]
    pub fn cfpffv1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwcfc::Cfpffv1,
        fwcfc::Cfpffv1,
        Fwcfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwcfc::Cfpffv1,
            fwcfc::Cfpffv1,
            Fwcfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwcfc {
    #[inline(always)]
    fn default() -> Fwcfc {
        <crate::RegValueT<Fwcfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwcfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfeffv0_SPEC;
    pub type Cfeffv0 = crate::EnumBitfieldStruct<u8, Cfeffv0_SPEC>;
    impl Cfeffv0 {
        #[doc = "Cascade filter i is disabled for port j e-frames (All slow ports frames are e-frames) \\[GWCA\\]."]
        pub const _0: Self = Self::new(0);

        #[doc = "Cascade filter i is enabled for port j e-frames (All slow ports frames are e-frames) \\[GWCA\\]."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfeffv1_SPEC;
    pub type Cfeffv1 = crate::EnumBitfieldStruct<u8, Cfeffv1_SPEC>;
    impl Cfeffv1 {
        #[doc = "Cascade filter i is disabled for port j e-frames (All slow ports frames are e-frames) \\[GWCA\\]."]
        pub const _0: Self = Self::new(0);

        #[doc = "Cascade filter i is enabled for port j e-frames (All slow ports frames are e-frames) \\[GWCA\\]."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfeffv2_SPEC;
    pub type Cfeffv2 = crate::EnumBitfieldStruct<u8, Cfeffv2_SPEC>;
    impl Cfeffv2 {
        #[doc = "Cascade filter i is disabled for port j e-frames (All slow ports frames are e-frames) \\[GWCA\\]."]
        pub const _0: Self = Self::new(0);

        #[doc = "Cascade filter i is enabled for port j e-frames (All slow ports frames are e-frames) \\[GWCA\\]."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfpffv0_SPEC;
    pub type Cfpffv0 = crate::EnumBitfieldStruct<u8, Cfpffv0_SPEC>;
    impl Cfpffv0 {
        #[doc = "Cascade filter i is disabled for port j p-frames."]
        pub const _0: Self = Self::new(0);

        #[doc = "Cascade filter i is enabled for port j p-frames."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfpffv1_SPEC;
    pub type Cfpffv1 = crate::EnumBitfieldStruct<u8, Cfpffv1_SPEC>;
    impl Cfpffv1 {
        #[doc = "Cascade filter i is disabled for port j p-frames."]
        pub const _0: Self = Self::new(0);

        #[doc = "Cascade filter i is enabled for port j p-frames."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfmc0_SPEC;
impl crate::sealed::RegSpec for Fwcfmc0_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Mapping Configuration Register i0"]
pub type Fwcfmc0 = crate::RegValueT<Fwcfmc0_SPEC>;

impl Fwcfmc0 {
    #[doc = "Cascade Filter Filter Number"]
    #[inline(always)]
    pub fn cffn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwcfmc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwcfmc0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cascade Filter Valid"]
    #[inline(always)]
    pub fn cffv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwcfmc0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwcfmc0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcfmc0 {
    #[inline(always)]
    fn default() -> Fwcfmc0 {
        <crate::RegValueT<Fwcfmc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfmc1_SPEC;
impl crate::sealed::RegSpec for Fwcfmc1_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Mapping Configuration Register i1"]
pub type Fwcfmc1 = crate::RegValueT<Fwcfmc1_SPEC>;

impl Fwcfmc1 {
    #[doc = "Cascade Filter Filter Number"]
    #[inline(always)]
    pub fn cffn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwcfmc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwcfmc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cascade Filter Valid"]
    #[inline(always)]
    pub fn cffv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwcfmc1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwcfmc1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcfmc1 {
    #[inline(always)]
    fn default() -> Fwcfmc1 {
        <crate::RegValueT<Fwcfmc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfmc2_SPEC;
impl crate::sealed::RegSpec for Fwcfmc2_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Mapping Configuration Register i2"]
pub type Fwcfmc2 = crate::RegValueT<Fwcfmc2_SPEC>;

impl Fwcfmc2 {
    #[doc = "Cascade Filter Filter Number"]
    #[inline(always)]
    pub fn cffn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwcfmc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwcfmc2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cascade Filter Valid"]
    #[inline(always)]
    pub fn cffv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwcfmc2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwcfmc2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcfmc2 {
    #[inline(always)]
    fn default() -> Fwcfmc2 {
        <crate::RegValueT<Fwcfmc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfmc3_SPEC;
impl crate::sealed::RegSpec for Fwcfmc3_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Mapping Configuration Register i3"]
pub type Fwcfmc3 = crate::RegValueT<Fwcfmc3_SPEC>;

impl Fwcfmc3 {
    #[doc = "Cascade Filter Filter Number"]
    #[inline(always)]
    pub fn cffn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwcfmc3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwcfmc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cascade Filter Valid"]
    #[inline(always)]
    pub fn cffv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwcfmc3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwcfmc3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcfmc3 {
    #[inline(always)]
    fn default() -> Fwcfmc3 {
        <crate::RegValueT<Fwcfmc3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfmc4_SPEC;
impl crate::sealed::RegSpec for Fwcfmc4_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Mapping Configuration Register i4"]
pub type Fwcfmc4 = crate::RegValueT<Fwcfmc4_SPEC>;

impl Fwcfmc4 {
    #[doc = "Cascade Filter Filter Number"]
    #[inline(always)]
    pub fn cffn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwcfmc4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwcfmc4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cascade Filter Valid"]
    #[inline(always)]
    pub fn cffv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwcfmc4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwcfmc4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcfmc4 {
    #[inline(always)]
    fn default() -> Fwcfmc4 {
        <crate::RegValueT<Fwcfmc4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfmc5_SPEC;
impl crate::sealed::RegSpec for Fwcfmc5_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Mapping Configuration Register i5"]
pub type Fwcfmc5 = crate::RegValueT<Fwcfmc5_SPEC>;

impl Fwcfmc5 {
    #[doc = "Cascade Filter Filter Number"]
    #[inline(always)]
    pub fn cffn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwcfmc5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwcfmc5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cascade Filter Valid"]
    #[inline(always)]
    pub fn cffv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwcfmc5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwcfmc5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcfmc5 {
    #[inline(always)]
    fn default() -> Fwcfmc5 {
        <crate::RegValueT<Fwcfmc5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwcfmc6_SPEC;
impl crate::sealed::RegSpec for Fwcfmc6_SPEC {
    type DataType = u32;
}

#[doc = "Cascade Filter Mapping Configuration Register i6"]
pub type Fwcfmc6 = crate::RegValueT<Fwcfmc6_SPEC>;

impl Fwcfmc6 {
    #[doc = "Cascade Filter Filter Number"]
    #[inline(always)]
    pub fn cffn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwcfmc6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwcfmc6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cascade Filter Valid"]
    #[inline(always)]
    pub fn cffv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwcfmc6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fwcfmc6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwcfmc6 {
    #[inline(always)]
    fn default() -> Fwcfmc6 {
        <crate::RegValueT<Fwcfmc6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwip4Sc_SPEC;
impl crate::sealed::RegSpec for Fwip4Sc_SPEC {
    type DataType = u32;
}

#[doc = "IPv4 Stream Configuration Register"]
pub type Fwip4Sc = crate::RegValueT<Fwip4Sc_SPEC>;

impl Fwip4Sc {
    #[doc = "IPv4 Include MAC Destination in Hash"]
    #[inline(always)]
    pub fn ip4imdh(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwip4sc::Ip4Imdh,
        fwip4sc::Ip4Imdh,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwip4sc::Ip4Imdh,
            fwip4sc::Ip4Imdh,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include MAC Source in Hash"]
    #[inline(always)]
    pub fn ip4imsh(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwip4sc::Ip4Imsh,
        fwip4sc::Ip4Imsh,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwip4sc::Ip4Imsh,
            fwip4sc::Ip4Imsh,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include S-TAG VLAN ID in Hash"]
    #[inline(always)]
    pub fn ip4isvh(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwip4sc::Ip4Isvh,
        fwip4sc::Ip4Isvh,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwip4sc::Ip4Isvh,
            fwip4sc::Ip4Isvh,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include S-TAG PCP in Hash"]
    #[inline(always)]
    pub fn ip4isph(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwip4sc::Ip4Isph,
        fwip4sc::Ip4Isph,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwip4sc::Ip4Isph,
            fwip4sc::Ip4Isph,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include S-TAG DEI in Hash"]
    #[inline(always)]
    pub fn ip4isdh(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwip4sc::Ip4Isdh,
        fwip4sc::Ip4Isdh,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwip4sc::Ip4Isdh,
            fwip4sc::Ip4Isdh,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include C-TAG VLAN ID in Hash"]
    #[inline(always)]
    pub fn ip4icvh(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwip4sc::Ip4Icvh,
        fwip4sc::Ip4Icvh,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwip4sc::Ip4Icvh,
            fwip4sc::Ip4Icvh,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include C-TAG PCP in Hash"]
    #[inline(always)]
    pub fn ip4icph(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwip4sc::Ip4Icph,
        fwip4sc::Ip4Icph,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwip4sc::Ip4Icph,
            fwip4sc::Ip4Icph,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include C-TAG DEI in Hash"]
    #[inline(always)]
    pub fn ip4icdh(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwip4sc::Ip4Icdh,
        fwip4sc::Ip4Icdh,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwip4sc::Ip4Icdh,
            fwip4sc::Ip4Icdh,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include IP Source in Hash"]
    #[inline(always)]
    pub fn ip4iish(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwip4sc::Ip4Iish,
        fwip4sc::Ip4Iish,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwip4sc::Ip4Iish,
            fwip4sc::Ip4Iish,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include IP Destination in Hash"]
    #[inline(always)]
    pub fn ip4iidh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fwip4sc::Ip4Iidh,
        fwip4sc::Ip4Iidh,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fwip4sc::Ip4Iidh,
            fwip4sc::Ip4Iidh,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include Protocol in Hash"]
    #[inline(always)]
    pub fn ip4iph(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fwip4sc::Ip4Iph,
        fwip4sc::Ip4Iph,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fwip4sc::Ip4Iph,
            fwip4sc::Ip4Iph,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include Source Port in Hash"]
    #[inline(always)]
    pub fn ip4ispth(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fwip4sc::Ip4Ispth,
        fwip4sc::Ip4Ispth,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fwip4sc::Ip4Ispth,
            fwip4sc::Ip4Ispth,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include Destination Port in Hash"]
    #[inline(always)]
    pub fn ip4idpth(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fwip4sc::Ip4Idpth,
        fwip4sc::Ip4Idpth,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fwip4sc::Ip4Idpth,
            fwip4sc::Ip4Idpth,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include S-TAG VLAN ID in Stream"]
    #[inline(always)]
    pub fn ip4isvs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwip4sc::Ip4Isvs,
        fwip4sc::Ip4Isvs,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwip4sc::Ip4Isvs,
            fwip4sc::Ip4Isvs,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include S-TAG PCP in Stream"]
    #[inline(always)]
    pub fn ip4isps(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwip4sc::Ip4Isps,
        fwip4sc::Ip4Isps,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwip4sc::Ip4Isps,
            fwip4sc::Ip4Isps,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include S-TAG DEI in Stream"]
    #[inline(always)]
    pub fn ip4isds(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fwip4sc::Ip4Isds,
        fwip4sc::Ip4Isds,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fwip4sc::Ip4Isds,
            fwip4sc::Ip4Isds,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include C-TAG VLAN ID in Stream"]
    #[inline(always)]
    pub fn ip4icvs(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        fwip4sc::Ip4Icvs,
        fwip4sc::Ip4Icvs,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            fwip4sc::Ip4Icvs,
            fwip4sc::Ip4Icvs,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include C-TAG PCP in Stream"]
    #[inline(always)]
    pub fn ip4icps(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fwip4sc::Ip4Icps,
        fwip4sc::Ip4Icps,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fwip4sc::Ip4Icps,
            fwip4sc::Ip4Icps,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include C-TAG DEI in Stream"]
    #[inline(always)]
    pub fn ip4icds(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fwip4sc::Ip4Icds,
        fwip4sc::Ip4Icds,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fwip4sc::Ip4Icds,
            fwip4sc::Ip4Icds,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include IP Source in Stream"]
    #[inline(always)]
    pub fn ip4iiss(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        fwip4sc::Ip4Iiss,
        fwip4sc::Ip4Iiss,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            fwip4sc::Ip4Iiss,
            fwip4sc::Ip4Iiss,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include IP Destination in Stream"]
    #[inline(always)]
    pub fn ip4iids(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fwip4sc::Ip4Iids,
        fwip4sc::Ip4Iids,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fwip4sc::Ip4Iids,
            fwip4sc::Ip4Iids,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Include Destination Port in Stream"]
    #[inline(always)]
    pub fn ip4idpts(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        fwip4sc::Ip4Idpts,
        fwip4sc::Ip4Idpts,
        Fwip4Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            fwip4sc::Ip4Idpts,
            fwip4sc::Ip4Idpts,
            Fwip4Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwip4Sc {
    #[inline(always)]
    fn default() -> Fwip4Sc {
        <crate::RegValueT<Fwip4Sc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwip4sc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Imdh_SPEC;
    pub type Ip4Imdh = crate::EnumBitfieldStruct<u8, Ip4Imdh_SPEC>;
    impl Ip4Imdh {
        #[doc = "MAC Destination address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Destination address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Imsh_SPEC;
    pub type Ip4Imsh = crate::EnumBitfieldStruct<u8, Ip4Imsh_SPEC>;
    impl Ip4Imsh {
        #[doc = "MAC Source address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Source address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Isvh_SPEC;
    pub type Ip4Isvh = crate::EnumBitfieldStruct<u8, Ip4Isvh_SPEC>;
    impl Ip4Isvh {
        #[doc = "S-TAG VLAN ID is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG VLAN ID is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Isph_SPEC;
    pub type Ip4Isph = crate::EnumBitfieldStruct<u8, Ip4Isph_SPEC>;
    impl Ip4Isph {
        #[doc = "S-TAG PCP is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG PCP is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Isdh_SPEC;
    pub type Ip4Isdh = crate::EnumBitfieldStruct<u8, Ip4Isdh_SPEC>;
    impl Ip4Isdh {
        #[doc = "S-TAG DEI is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG DEI is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Icvh_SPEC;
    pub type Ip4Icvh = crate::EnumBitfieldStruct<u8, Ip4Icvh_SPEC>;
    impl Ip4Icvh {
        #[doc = "C-TAG VLAN ID is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG VLAN ID is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Icph_SPEC;
    pub type Ip4Icph = crate::EnumBitfieldStruct<u8, Ip4Icph_SPEC>;
    impl Ip4Icph {
        #[doc = "C-TAG PCP is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG PCP is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Icdh_SPEC;
    pub type Ip4Icdh = crate::EnumBitfieldStruct<u8, Ip4Icdh_SPEC>;
    impl Ip4Icdh {
        #[doc = "C-TAG DEI is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG DEI is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Iish_SPEC;
    pub type Ip4Iish = crate::EnumBitfieldStruct<u8, Ip4Iish_SPEC>;
    impl Ip4Iish {
        #[doc = "IPv4 Source IP address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv4 Source IP address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Iidh_SPEC;
    pub type Ip4Iidh = crate::EnumBitfieldStruct<u8, Ip4Iidh_SPEC>;
    impl Ip4Iidh {
        #[doc = "IPv4 Destination IP address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "IP v4 Destination IP address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Iph_SPEC;
    pub type Ip4Iph = crate::EnumBitfieldStruct<u8, Ip4Iph_SPEC>;
    impl Ip4Iph {
        #[doc = "IPv4 Protocol is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv4 Protocol is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Ispth_SPEC;
    pub type Ip4Ispth = crate::EnumBitfieldStruct<u8, Ip4Ispth_SPEC>;
    impl Ip4Ispth {
        #[doc = "TCP/UDP Source port is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "TCP/UDP Source port is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Idpth_SPEC;
    pub type Ip4Idpth = crate::EnumBitfieldStruct<u8, Ip4Idpth_SPEC>;
    impl Ip4Idpth {
        #[doc = "TCP/UDP Destination port is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "TCP/UDP Destination port is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Isvs_SPEC;
    pub type Ip4Isvs = crate::EnumBitfieldStruct<u8, Ip4Isvs_SPEC>;
    impl Ip4Isvs {
        #[doc = "S-TAG VLAN ID is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG VLAN ID is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Isps_SPEC;
    pub type Ip4Isps = crate::EnumBitfieldStruct<u8, Ip4Isps_SPEC>;
    impl Ip4Isps {
        #[doc = "S-TAG PCP is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG PCP is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Isds_SPEC;
    pub type Ip4Isds = crate::EnumBitfieldStruct<u8, Ip4Isds_SPEC>;
    impl Ip4Isds {
        #[doc = "S-TAG DEI is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG DEI is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Icvs_SPEC;
    pub type Ip4Icvs = crate::EnumBitfieldStruct<u8, Ip4Icvs_SPEC>;
    impl Ip4Icvs {
        #[doc = "C-TAG VLAN ID is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG VLAN ID is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Icps_SPEC;
    pub type Ip4Icps = crate::EnumBitfieldStruct<u8, Ip4Icps_SPEC>;
    impl Ip4Icps {
        #[doc = "C-TAG PCP is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG PCP is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Icds_SPEC;
    pub type Ip4Icds = crate::EnumBitfieldStruct<u8, Ip4Icds_SPEC>;
    impl Ip4Icds {
        #[doc = "C-TAG DEI is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG DEI is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Iiss_SPEC;
    pub type Ip4Iiss = crate::EnumBitfieldStruct<u8, Ip4Iiss_SPEC>;
    impl Ip4Iiss {
        #[doc = "IPv4 Source IP address is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv4 Source IP address is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Iids_SPEC;
    pub type Ip4Iids = crate::EnumBitfieldStruct<u8, Ip4Iids_SPEC>;
    impl Ip4Iids {
        #[doc = "IPv4 Destination IP address is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv4 Destination IP address is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Idpts_SPEC;
    pub type Ip4Idpts = crate::EnumBitfieldStruct<u8, Ip4Idpts_SPEC>;
    impl Ip4Idpts {
        #[doc = "TCP/UDP Destination port is not included in L3 IPv4 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "TCP/UDP Destination port is included in L3 IPv4 stream ID"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwip6Sc_SPEC;
impl crate::sealed::RegSpec for Fwip6Sc_SPEC {
    type DataType = u32;
}

#[doc = "IPv6 Stream Configuration Register"]
pub type Fwip6Sc = crate::RegValueT<Fwip6Sc_SPEC>;

impl Fwip6Sc {
    #[doc = "IPv6 Include MAC Destination in Hash"]
    #[inline(always)]
    pub fn ip6imdh(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwip6sc::Ip6Imdh,
        fwip6sc::Ip6Imdh,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwip6sc::Ip6Imdh,
            fwip6sc::Ip6Imdh,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include MAC Source in Hash"]
    #[inline(always)]
    pub fn ip6imsh(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwip6sc::Ip6Imsh,
        fwip6sc::Ip6Imsh,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwip6sc::Ip6Imsh,
            fwip6sc::Ip6Imsh,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include S-TAG VLAN ID in Hash"]
    #[inline(always)]
    pub fn ip6isvh(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwip6sc::Ip6Isvh,
        fwip6sc::Ip6Isvh,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwip6sc::Ip6Isvh,
            fwip6sc::Ip6Isvh,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include S-TAG PCP in Hash"]
    #[inline(always)]
    pub fn ip6isph(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwip6sc::Ip6Isph,
        fwip6sc::Ip6Isph,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwip6sc::Ip6Isph,
            fwip6sc::Ip6Isph,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include S-TAG DEI in Hash"]
    #[inline(always)]
    pub fn ip6isdh(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwip6sc::Ip6Isdh,
        fwip6sc::Ip6Isdh,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwip6sc::Ip6Isdh,
            fwip6sc::Ip6Isdh,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include C-TAG VLAN ID in Hash"]
    #[inline(always)]
    pub fn ip6icvh(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwip6sc::Ip6Icvh,
        fwip6sc::Ip6Icvh,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwip6sc::Ip6Icvh,
            fwip6sc::Ip6Icvh,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include C-TAG PCP in Hash"]
    #[inline(always)]
    pub fn ip6icph(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwip6sc::Ip6Icph,
        fwip6sc::Ip6Icph,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwip6sc::Ip6Icph,
            fwip6sc::Ip6Icph,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include C-TAG DEI in Hash"]
    #[inline(always)]
    pub fn ip6icdh(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwip6sc::Ip6Icdh,
        fwip6sc::Ip6Icdh,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwip6sc::Ip6Icdh,
            fwip6sc::Ip6Icdh,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include IP Source in Hash"]
    #[inline(always)]
    pub fn ip6iish(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fwip6sc::Ip6Iish,
        fwip6sc::Ip6Iish,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fwip6sc::Ip6Iish,
            fwip6sc::Ip6Iish,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include IP Destination in Hash"]
    #[inline(always)]
    pub fn ip6iidh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fwip6sc::Ip6Iidh,
        fwip6sc::Ip6Iidh,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fwip6sc::Ip6Iidh,
            fwip6sc::Ip6Iidh,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include Protocol in Hash (Next Header)"]
    #[inline(always)]
    pub fn ip6iph(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fwip6sc::Ip6Iph,
        fwip6sc::Ip6Iph,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fwip6sc::Ip6Iph,
            fwip6sc::Ip6Iph,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include Source Port in Hash"]
    #[inline(always)]
    pub fn ip6ispth(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fwip6sc::Ip6Ispth,
        fwip6sc::Ip6Ispth,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fwip6sc::Ip6Ispth,
            fwip6sc::Ip6Ispth,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include Destination Port in Hash"]
    #[inline(always)]
    pub fn ip6idpth(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fwip6sc::Ip6Idpth,
        fwip6sc::Ip6Idpth,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fwip6sc::Ip6Idpth,
            fwip6sc::Ip6Idpth,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include S-TAG VLAN ID in Stream"]
    #[inline(always)]
    pub fn ip6isvs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwip6sc::Ip6Isvs,
        fwip6sc::Ip6Isvs,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwip6sc::Ip6Isvs,
            fwip6sc::Ip6Isvs,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include S-TAG PCP in Stream"]
    #[inline(always)]
    pub fn ip6isps(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwip6sc::Ip6Isps,
        fwip6sc::Ip6Isps,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwip6sc::Ip6Isps,
            fwip6sc::Ip6Isps,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include S-TAG DEI in Stream"]
    #[inline(always)]
    pub fn ip6isds(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fwip6sc::Ip6Isds,
        fwip6sc::Ip6Isds,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fwip6sc::Ip6Isds,
            fwip6sc::Ip6Isds,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include C-TAG VLAN ID in Stream"]
    #[inline(always)]
    pub fn ip6icvs(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        fwip6sc::Ip6Icvs,
        fwip6sc::Ip6Icvs,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            fwip6sc::Ip6Icvs,
            fwip6sc::Ip6Icvs,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include C-TAG PCP in Stream"]
    #[inline(always)]
    pub fn ip6icps(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fwip6sc::Ip6Icps,
        fwip6sc::Ip6Icps,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fwip6sc::Ip6Icps,
            fwip6sc::Ip6Icps,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include C-TAG DEI in Stream"]
    #[inline(always)]
    pub fn ip6icds(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fwip6sc::Ip6Icds,
        fwip6sc::Ip6Icds,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fwip6sc::Ip6Icds,
            fwip6sc::Ip6Icds,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include IP 0 in Stream"]
    #[inline(always)]
    pub fn ip6ii0s(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        fwip6sc::Ip6Ii0S,
        fwip6sc::Ip6Ii0S,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            fwip6sc::Ip6Ii0S,
            fwip6sc::Ip6Ii0S,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include IP 1 in Stream"]
    #[inline(always)]
    pub fn ip6ii1s(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fwip6sc::Ip6Ii1S,
        fwip6sc::Ip6Ii1S,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fwip6sc::Ip6Ii1S,
            fwip6sc::Ip6Ii1S,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Include Destination Port in Stream"]
    #[inline(always)]
    pub fn ip6idpts(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        fwip6sc::Ip6Idpts,
        fwip6sc::Ip6Idpts,
        Fwip6Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            fwip6sc::Ip6Idpts,
            fwip6sc::Ip6Idpts,
            Fwip6Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwip6Sc {
    #[inline(always)]
    fn default() -> Fwip6Sc {
        <crate::RegValueT<Fwip6Sc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwip6sc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Imdh_SPEC;
    pub type Ip6Imdh = crate::EnumBitfieldStruct<u8, Ip6Imdh_SPEC>;
    impl Ip6Imdh {
        #[doc = "MAC Destination address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Destination address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Imsh_SPEC;
    pub type Ip6Imsh = crate::EnumBitfieldStruct<u8, Ip6Imsh_SPEC>;
    impl Ip6Imsh {
        #[doc = "MAC Source address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Source address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Isvh_SPEC;
    pub type Ip6Isvh = crate::EnumBitfieldStruct<u8, Ip6Isvh_SPEC>;
    impl Ip6Isvh {
        #[doc = "S-TAG VLAN ID is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG VLAN ID is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Isph_SPEC;
    pub type Ip6Isph = crate::EnumBitfieldStruct<u8, Ip6Isph_SPEC>;
    impl Ip6Isph {
        #[doc = "S-TAG PCP is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG PCP is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Isdh_SPEC;
    pub type Ip6Isdh = crate::EnumBitfieldStruct<u8, Ip6Isdh_SPEC>;
    impl Ip6Isdh {
        #[doc = "S-TAG DEI is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG DEI is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Icvh_SPEC;
    pub type Ip6Icvh = crate::EnumBitfieldStruct<u8, Ip6Icvh_SPEC>;
    impl Ip6Icvh {
        #[doc = "C-TAG VLAN ID is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG VLAN ID is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Icph_SPEC;
    pub type Ip6Icph = crate::EnumBitfieldStruct<u8, Ip6Icph_SPEC>;
    impl Ip6Icph {
        #[doc = "C-TAG PCP is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG PCP is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Icdh_SPEC;
    pub type Ip6Icdh = crate::EnumBitfieldStruct<u8, Ip6Icdh_SPEC>;
    impl Ip6Icdh {
        #[doc = "C-TAG DEI is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG DEI is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Iish_SPEC;
    pub type Ip6Iish = crate::EnumBitfieldStruct<u8, Ip6Iish_SPEC>;
    impl Ip6Iish {
        #[doc = "IPv6 Source IP address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 Source IP address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Iidh_SPEC;
    pub type Ip6Iidh = crate::EnumBitfieldStruct<u8, Ip6Iidh_SPEC>;
    impl Ip6Iidh {
        #[doc = "IPv6 Destination IP address is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 Destination IP address is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Iph_SPEC;
    pub type Ip6Iph = crate::EnumBitfieldStruct<u8, Ip6Iph_SPEC>;
    impl Ip6Iph {
        #[doc = "IPv6 Protocol is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 Protocol is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Ispth_SPEC;
    pub type Ip6Ispth = crate::EnumBitfieldStruct<u8, Ip6Ispth_SPEC>;
    impl Ip6Ispth {
        #[doc = "TCP/UDP Source port is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "TCP/UDP Source port is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Idpth_SPEC;
    pub type Ip6Idpth = crate::EnumBitfieldStruct<u8, Ip6Idpth_SPEC>;
    impl Ip6Idpth {
        #[doc = "TCP/UDP Destination port is not included in hash calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "TCP/UDP Destination port is included in hash calculation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Isvs_SPEC;
    pub type Ip6Isvs = crate::EnumBitfieldStruct<u8, Ip6Isvs_SPEC>;
    impl Ip6Isvs {
        #[doc = "S-TAG VLAN ID is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG VLAN ID is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Isps_SPEC;
    pub type Ip6Isps = crate::EnumBitfieldStruct<u8, Ip6Isps_SPEC>;
    impl Ip6Isps {
        #[doc = "S-TAG PCP is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG PCP is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Isds_SPEC;
    pub type Ip6Isds = crate::EnumBitfieldStruct<u8, Ip6Isds_SPEC>;
    impl Ip6Isds {
        #[doc = "S-TAG DEI is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-TAG DEI is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Icvs_SPEC;
    pub type Ip6Icvs = crate::EnumBitfieldStruct<u8, Ip6Icvs_SPEC>;
    impl Ip6Icvs {
        #[doc = "C-TAG VLAN ID is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG VLAN ID is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Icps_SPEC;
    pub type Ip6Icps = crate::EnumBitfieldStruct<u8, Ip6Icps_SPEC>;
    impl Ip6Icps {
        #[doc = "C-TAG PCP is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG PCP is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Icds_SPEC;
    pub type Ip6Icds = crate::EnumBitfieldStruct<u8, Ip6Icds_SPEC>;
    impl Ip6Icds {
        #[doc = "C-TAG DEI is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-TAG DEI is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Ii0S_SPEC;
    pub type Ip6Ii0S = crate::EnumBitfieldStruct<u8, Ip6Ii0S_SPEC>;
    impl Ip6Ii0S {
        #[doc = "IPv6 IP address part 0 is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 IP address part 0 is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Ii1S_SPEC;
    pub type Ip6Ii1S = crate::EnumBitfieldStruct<u8, Ip6Ii1S_SPEC>;
    impl Ip6Ii1S {
        #[doc = "IPv6 IP address part 1 is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 IP address part 1 is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Idpts_SPEC;
    pub type Ip6Idpts = crate::EnumBitfieldStruct<u8, Ip6Idpts_SPEC>;
    impl Ip6Idpts {
        #[doc = "TCP/UDP Destination port is not included in L3 IPv6 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "TCP/UDP Destination port is included in L3 IPv6 stream ID"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwip6Oc_SPEC;
impl crate::sealed::RegSpec for Fwip6Oc_SPEC {
    type DataType = u32;
}

#[doc = "IPv6 Offset Configuration Register"]
pub type Fwip6Oc = crate::RegValueT<Fwip6Oc_SPEC>;

impl Fwip6Oc {
    #[doc = "IPv6 IP Offset mode 0"]
    #[inline(always)]
    pub fn ip6ipom0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwip6oc::Ip6Ipom0,
        fwip6oc::Ip6Ipom0,
        Fwip6Oc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwip6oc::Ip6Ipom0,
            fwip6oc::Ip6Ipom0,
            Fwip6Oc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 IP Offset 0"]
    #[inline(always)]
    pub fn ip6ipo0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Fwip6Oc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Fwip6Oc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPv6 IP Offset mode 1"]
    #[inline(always)]
    pub fn ip6ipom1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwip6oc::Ip6Ipom1,
        fwip6oc::Ip6Ipom1,
        Fwip6Oc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwip6oc::Ip6Ipom1,
            fwip6oc::Ip6Ipom1,
            Fwip6Oc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 IP Offset 1"]
    #[inline(always)]
    pub fn ip6ipo1(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Fwip6Oc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Fwip6Oc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwip6Oc {
    #[inline(always)]
    fn default() -> Fwip6Oc {
        <crate::RegValueT<Fwip6Oc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwip6oc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Ipom0_SPEC;
    pub type Ip6Ipom0 = crate::EnumBitfieldStruct<u8, Ip6Ipom0_SPEC>;
    impl Ip6Ipom0 {
        #[doc = "IPv6 IP address part 0 is extracted from IP source address"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 IP address part 0 is extracted from IP destination address"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Ipom1_SPEC;
    pub type Ip6Ipom1 = crate::EnumBitfieldStruct<u8, Ip6Ipom1_SPEC>;
    impl Ip6Ipom1 {
        #[doc = "IPv6 IP address part 1 is extracted from IP source address"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 IP address part 1 is extracted from IP destination address"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl2Sc_SPEC;
impl crate::sealed::RegSpec for Fwl2Sc_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2 Stream Configuration Register"]
pub type Fwl2Sc = crate::RegValueT<Fwl2Sc_SPEC>;

impl Fwl2Sc {
    #[doc = "Layer 2 Include MAC Destination in Stream"]
    #[inline(always)]
    pub fn l2imds(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwl2sc::L2Imds,
        fwl2sc::L2Imds,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwl2sc::L2Imds,
            fwl2sc::L2Imds,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Include MAC Source in Stream"]
    #[inline(always)]
    pub fn l2imss(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwl2sc::L2Imss,
        fwl2sc::L2Imss,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwl2sc::L2Imss,
            fwl2sc::L2Imss,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Include S-TAG VLAN ID in Stream"]
    #[inline(always)]
    pub fn l2isvs(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwl2sc::L2Isvs,
        fwl2sc::L2Isvs,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwl2sc::L2Isvs,
            fwl2sc::L2Isvs,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Include S-TAG PCP ID in Stream"]
    #[inline(always)]
    pub fn l2isps(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwl2sc::L2Isps,
        fwl2sc::L2Isps,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwl2sc::L2Isps,
            fwl2sc::L2Isps,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Include S-TAG DEI in Stream"]
    #[inline(always)]
    pub fn l2isds(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwl2sc::L2Isds,
        fwl2sc::L2Isds,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwl2sc::L2Isds,
            fwl2sc::L2Isds,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Include C-TAG VLAN ID in Stream"]
    #[inline(always)]
    pub fn l2icvs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fwl2sc::L2Icvs,
        fwl2sc::L2Icvs,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fwl2sc::L2Icvs,
            fwl2sc::L2Icvs,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Include C-TAG PCP ID in Stream"]
    #[inline(always)]
    pub fn l2icps(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fwl2sc::L2Icps,
        fwl2sc::L2Icps,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fwl2sc::L2Icps,
            fwl2sc::L2Icps,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Include C-TAG DEI in Stream"]
    #[inline(always)]
    pub fn l2icds(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fwl2sc::L2Icds,
        fwl2sc::L2Icds,
        Fwl2Sc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fwl2sc::L2Icds,
            fwl2sc::L2Icds,
            Fwl2Sc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwl2Sc {
    #[inline(always)]
    fn default() -> Fwl2Sc {
        <crate::RegValueT<Fwl2Sc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwl2sc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Imds_SPEC;
    pub type L2Imds = crate::EnumBitfieldStruct<u8, L2Imds_SPEC>;
    impl L2Imds {
        #[doc = "MAC Destination address is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Destination address is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Imss_SPEC;
    pub type L2Imss = crate::EnumBitfieldStruct<u8, L2Imss_SPEC>;
    impl L2Imss {
        #[doc = "MAC Source address is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Source address is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Isvs_SPEC;
    pub type L2Isvs = crate::EnumBitfieldStruct<u8, L2Isvs_SPEC>;
    impl L2Isvs {
        #[doc = "MAC S-TAG VLAN ID is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC S-TAG VLAN ID is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Isps_SPEC;
    pub type L2Isps = crate::EnumBitfieldStruct<u8, L2Isps_SPEC>;
    impl L2Isps {
        #[doc = "MAC S-TAG PCP is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC S-TAG PCP is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Isds_SPEC;
    pub type L2Isds = crate::EnumBitfieldStruct<u8, L2Isds_SPEC>;
    impl L2Isds {
        #[doc = "MAC S-TAG DEI is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC S-TAG DEI is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Icvs_SPEC;
    pub type L2Icvs = crate::EnumBitfieldStruct<u8, L2Icvs_SPEC>;
    impl L2Icvs {
        #[doc = "MAC C-TAG VLAN ID is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC C-TAG VLAN ID is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Icps_SPEC;
    pub type L2Icps = crate::EnumBitfieldStruct<u8, L2Icps_SPEC>;
    impl L2Icps {
        #[doc = "MAC C-TAG PCP is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC C-TAG PCP is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Icds_SPEC;
    pub type L2Icds = crate::EnumBitfieldStruct<u8, L2Icds_SPEC>;
    impl L2Icds {
        #[doc = "MAC C-TAG DEI is not included in L2 stream ID"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC C-TAG DEI is included in L2 stream ID"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwsfhec_SPEC;
impl crate::sealed::RegSpec for Fwsfhec_SPEC {
    type DataType = u32;
}

#[doc = "Stream Filter Hash Equation Configuration Register"]
pub type Fwsfhec = crate::RegValueT<Fwsfhec_SPEC>;

impl Fwsfhec {
    #[doc = "IPv4 Stream Filter Hash Equation n (n = 0 to 15)"]
    #[inline(always)]
    pub fn ip4he15_to_ip4he0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwsfhec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwsfhec_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPv6 Stream Filter Hash Equation n (n = 0 to 15)"]
    #[inline(always)]
    pub fn ip6he15_to_ip6he0(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Fwsfhec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Fwsfhec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwsfhec {
    #[inline(always)]
    fn default() -> Fwsfhec {
        <crate::RegValueT<Fwsfhec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr0_SPEC;
impl crate::sealed::RegSpec for Fwshcr0_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 0"]
pub type Fwshcr0 = crate::RegValueT<Fwshcr0_SPEC>;

impl Fwshcr0 {
    #[doc = "Software Hash Calculation MAC Destination Part 0"]
    #[inline(always)]
    pub fn shcmdp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwshcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr0 {
    #[inline(always)]
    fn default() -> Fwshcr0 {
        <crate::RegValueT<Fwshcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr1_SPEC;
impl crate::sealed::RegSpec for Fwshcr1_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 1"]
pub type Fwshcr1 = crate::RegValueT<Fwshcr1_SPEC>;

impl Fwshcr1 {
    #[doc = "Software Hash Calculation MAC Source Part 0"]
    #[inline(always)]
    pub fn shcmsp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwshcr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwshcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Hash Calculation MAC Destination Part 1"]
    #[inline(always)]
    pub fn shcmdp1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Fwshcr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Fwshcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr1 {
    #[inline(always)]
    fn default() -> Fwshcr1 {
        <crate::RegValueT<Fwshcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr2_SPEC;
impl crate::sealed::RegSpec for Fwshcr2_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 2"]
pub type Fwshcr2 = crate::RegValueT<Fwshcr2_SPEC>;

impl Fwshcr2 {
    #[doc = "Software Hash Calculation MAC Source Part 1"]
    #[inline(always)]
    pub fn shcmsp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwshcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr2 {
    #[inline(always)]
    fn default() -> Fwshcr2 {
        <crate::RegValueT<Fwshcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr3_SPEC;
impl crate::sealed::RegSpec for Fwshcr3_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 3"]
pub type Fwshcr3 = crate::RegValueT<Fwshcr3_SPEC>;

impl Fwshcr3 {
    #[doc = "Software Hash Calculation C-TAG VLAN"]
    #[inline(always)]
    pub fn shccv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Fwshcr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Fwshcr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Hash Calculation C-TAG DEI"]
    #[inline(always)]
    pub fn shccd(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Fwshcr3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Fwshcr3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Software Hash Calculation C-TAG PCP"]
    #[inline(always)]
    pub fn shccp(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, Fwshcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,Fwshcr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Hash Calculation S-TAG VLANs"]
    #[inline(always)]
    pub fn shcsv(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Fwshcr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Fwshcr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Hash Calculation S-TAG DEI"]
    #[inline(always)]
    pub fn shcsd(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Fwshcr3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Fwshcr3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Software Hash Calculation S-TAG PCP"]
    #[inline(always)]
    pub fn shcsp(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Fwshcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Fwshcr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr3 {
    #[inline(always)]
    fn default() -> Fwshcr3 {
        <crate::RegValueT<Fwshcr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr4_SPEC;
impl crate::sealed::RegSpec for Fwshcr4_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 4"]
pub type Fwshcr4 = crate::RegValueT<Fwshcr4_SPEC>;

impl Fwshcr4 {
    #[doc = "Software Hash Calculation Protocol (NextHeader for IPv6)"]
    #[inline(always)]
    pub fn shcp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwshcr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwshcr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Hash Calculation Frame Format"]
    #[inline(always)]
    pub fn shcff(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwshcr4::Shcff,
        fwshcr4::Shcff,
        Fwshcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwshcr4::Shcff,
            fwshcr4::Shcff,
            Fwshcr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwshcr4 {
    #[inline(always)]
    fn default() -> Fwshcr4 {
        <crate::RegValueT<Fwshcr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwshcr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shcff_SPEC;
    pub type Shcff = crate::EnumBitfieldStruct<u8, Shcff_SPEC>;
    impl Shcff {
        #[doc = "IPv4 Hash Calculation"]
        pub const _0: Self = Self::new(0);

        #[doc = "IPv6 Hash Calculation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr5_SPEC;
impl crate::sealed::RegSpec for Fwshcr5_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 5"]
pub type Fwshcr5 = crate::RegValueT<Fwshcr5_SPEC>;

impl Fwshcr5 {
    #[doc = "Software Hash Calculation IP Source Part 0"]
    #[inline(always)]
    pub fn shcisp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwshcr5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr5 {
    #[inline(always)]
    fn default() -> Fwshcr5 {
        <crate::RegValueT<Fwshcr5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr6_SPEC;
impl crate::sealed::RegSpec for Fwshcr6_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 6"]
pub type Fwshcr6 = crate::RegValueT<Fwshcr6_SPEC>;

impl Fwshcr6 {
    #[doc = "Software Hash Calculation IP Source Part 1"]
    #[inline(always)]
    pub fn shcisp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwshcr6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr6 {
    #[inline(always)]
    fn default() -> Fwshcr6 {
        <crate::RegValueT<Fwshcr6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr7_SPEC;
impl crate::sealed::RegSpec for Fwshcr7_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 7"]
pub type Fwshcr7 = crate::RegValueT<Fwshcr7_SPEC>;

impl Fwshcr7 {
    #[doc = "Software Hash Calculation IP Source Part 2"]
    #[inline(always)]
    pub fn shcisp2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwshcr7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr7 {
    #[inline(always)]
    fn default() -> Fwshcr7 {
        <crate::RegValueT<Fwshcr7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr8_SPEC;
impl crate::sealed::RegSpec for Fwshcr8_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 8"]
pub type Fwshcr8 = crate::RegValueT<Fwshcr8_SPEC>;

impl Fwshcr8 {
    #[doc = "Software Hash Calculation IP Source Part 3"]
    #[inline(always)]
    pub fn shcisp3(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwshcr8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr8 {
    #[inline(always)]
    fn default() -> Fwshcr8 {
        <crate::RegValueT<Fwshcr8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr9_SPEC;
impl crate::sealed::RegSpec for Fwshcr9_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 9"]
pub type Fwshcr9 = crate::RegValueT<Fwshcr9_SPEC>;

impl Fwshcr9 {
    #[doc = "Software Hash Calculation IP Destination Part 0"]
    #[inline(always)]
    pub fn shcidp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr9_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwshcr9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr9 {
    #[inline(always)]
    fn default() -> Fwshcr9 {
        <crate::RegValueT<Fwshcr9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr10_SPEC;
impl crate::sealed::RegSpec for Fwshcr10_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 10"]
pub type Fwshcr10 = crate::RegValueT<Fwshcr10_SPEC>;

impl Fwshcr10 {
    #[doc = "Software Hash Calculation IP Source Destination Part 1"]
    #[inline(always)]
    pub fn shcidp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwshcr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwshcr10 {
    #[inline(always)]
    fn default() -> Fwshcr10 {
        <crate::RegValueT<Fwshcr10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr11_SPEC;
impl crate::sealed::RegSpec for Fwshcr11_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 11"]
pub type Fwshcr11 = crate::RegValueT<Fwshcr11_SPEC>;

impl Fwshcr11 {
    #[doc = "Software Hash Calculation IP Destination Part 2"]
    #[inline(always)]
    pub fn shcidp2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwshcr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwshcr11 {
    #[inline(always)]
    fn default() -> Fwshcr11 {
        <crate::RegValueT<Fwshcr11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr12_SPEC;
impl crate::sealed::RegSpec for Fwshcr12_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 12"]
pub type Fwshcr12 = crate::RegValueT<Fwshcr12_SPEC>;

impl Fwshcr12 {
    #[doc = "Software Hash Calculation IP Destination Part 3"]
    #[inline(always)]
    pub fn shcidp3(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwshcr12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwshcr12_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwshcr12 {
    #[inline(always)]
    fn default() -> Fwshcr12 {
        <crate::RegValueT<Fwshcr12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcr13_SPEC;
impl crate::sealed::RegSpec for Fwshcr13_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Register 13"]
pub type Fwshcr13 = crate::RegValueT<Fwshcr13_SPEC>;

impl Fwshcr13 {
    #[doc = "Software Hash Calculation Destination Port"]
    #[inline(always)]
    pub fn shcdp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwshcr13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwshcr13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Hash Calculation Source Port"]
    #[inline(always)]
    pub fn shcsp(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Fwshcr13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Fwshcr13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwshcr13 {
    #[inline(always)]
    fn default() -> Fwshcr13 {
        <crate::RegValueT<Fwshcr13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwshcrr_SPEC;
impl crate::sealed::RegSpec for Fwshcrr_SPEC {
    type DataType = u32;
}

#[doc = "Software Hash Calculation Request Result Register"]
pub type Fwshcrr = crate::RegValueT<Fwshcrr_SPEC>;

impl Fwshcrr {
    #[doc = "Software Hash Calculation Result"]
    #[inline(always)]
    pub fn shcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwshcrr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwshcrr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Software Hash Calculation"]
    #[inline(always)]
    pub fn shc(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fwshcrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fwshcrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwshcrr {
    #[inline(always)]
    fn default() -> Fwshcrr {
        <crate::RegValueT<Fwshcrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthhec_SPEC;
impl crate::sealed::RegSpec for Fwlthhec_SPEC {
    type DataType = u32;
}

#[doc = "L3 Hash Entry Configuration Register"]
pub type Fwlthhec = crate::RegValueT<Fwlthhec_SPEC>;

impl Fwlthhec {
    #[doc = "L3 Hash Maximum Collision"]
    #[inline(always)]
    pub fn lthhmc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwlthhec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwlthhec_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Hash Maximum Unsecure Entry"]
    #[inline(always)]
    pub fn lthhmue(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Fwlthhec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Fwlthhec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthhec {
    #[inline(always)]
    fn default() -> Fwlthhec {
        <crate::RegValueT<Fwlthhec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthhc_SPEC;
impl crate::sealed::RegSpec for Fwlthhc_SPEC {
    type DataType = u32;
}

#[doc = "L3 Hash Configuration Register"]
pub type Fwlthhc = crate::RegValueT<Fwlthhc_SPEC>;

impl Fwlthhc {
    #[doc = "L3 Hash Equation n (n = 0 to 7)"]
    #[inline(always)]
    pub fn lthhe7_to_lthhe0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwlthhc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwlthhc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthhc {
    #[inline(always)]
    fn default() -> Fwlthhc {
        <crate::RegValueT<Fwlthhc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl0_SPEC;
impl crate::sealed::RegSpec for Fwlthtl0_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 0"]
pub type Fwlthtl0 = crate::RegValueT<Fwlthtl0_SPEC>;

impl Fwlthtl0 {
    #[doc = "L3 Stream Learn Part 0"]
    #[inline(always)]
    pub fn lthslp0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwlthtl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwlthtl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Security Level Learn"]
    #[inline(always)]
    pub fn lthsll(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwlthtl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwlthtl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "L3 Entry Delete"]
    #[inline(always)]
    pub fn lthed(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwlthtl0::Lthed,
        fwlthtl0::Lthed,
        Fwlthtl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwlthtl0::Lthed,
            fwlthtl0::Lthed,
            Fwlthtl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtl0 {
    #[inline(always)]
    fn default() -> Fwlthtl0 {
        <crate::RegValueT<Fwlthtl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwlthtl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthed_SPEC;
    pub type Lthed = crate::EnumBitfieldStruct<u8, Lthed_SPEC>;
    impl Lthed {
        #[doc = "Learn/overwrite the set stream ID in L3 table."]
        pub const _0: Self = Self::new(0);

        #[doc = "Delete the set stream ID in L3 table."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl1_SPEC;
impl crate::sealed::RegSpec for Fwlthtl1_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 1"]
pub type Fwlthtl1 = crate::RegValueT<Fwlthtl1_SPEC>;

impl Fwlthtl1 {
    #[doc = "L3 Stream Learn Part 1"]
    #[inline(always)]
    pub fn lthslp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtl1 {
    #[inline(always)]
    fn default() -> Fwlthtl1 {
        <crate::RegValueT<Fwlthtl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl2_SPEC;
impl crate::sealed::RegSpec for Fwlthtl2_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 2"]
pub type Fwlthtl2 = crate::RegValueT<Fwlthtl2_SPEC>;

impl Fwlthtl2 {
    #[doc = "L3 Stream Learn Part 2"]
    #[inline(always)]
    pub fn lthslp2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtl2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtl2 {
    #[inline(always)]
    fn default() -> Fwlthtl2 {
        <crate::RegValueT<Fwlthtl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl3_SPEC;
impl crate::sealed::RegSpec for Fwlthtl3_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 3"]
pub type Fwlthtl3 = crate::RegValueT<Fwlthtl3_SPEC>;

impl Fwlthtl3 {
    #[doc = "L3 Stream Learn Part 3"]
    #[inline(always)]
    pub fn lthslp3(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtl3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtl3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtl3 {
    #[inline(always)]
    fn default() -> Fwlthtl3 {
        <crate::RegValueT<Fwlthtl3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl4_SPEC;
impl crate::sealed::RegSpec for Fwlthtl4_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 4"]
pub type Fwlthtl4 = crate::RegValueT<Fwlthtl4_SPEC>;

impl Fwlthtl4 {
    #[doc = "L3 Stream Learn Part 4"]
    #[inline(always)]
    pub fn lthslp4(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtl4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtl4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtl4 {
    #[inline(always)]
    fn default() -> Fwlthtl4 {
        <crate::RegValueT<Fwlthtl4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl5_SPEC;
impl crate::sealed::RegSpec for Fwlthtl5_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 5"]
pub type Fwlthtl5 = crate::RegValueT<Fwlthtl5_SPEC>;

impl Fwlthtl5 {
    #[doc = "L3 MSDU Number Learn"]
    #[inline(always)]
    pub fn lthmsdunl(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Fwlthtl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Fwlthtl5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 MSDU Valid Learn"]
    #[inline(always)]
    pub fn lthmsduvl(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtl5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtl5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtl5 {
    #[inline(always)]
    fn default() -> Fwlthtl5 {
        <crate::RegValueT<Fwlthtl5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl6_SPEC;
impl crate::sealed::RegSpec for Fwlthtl6_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 6"]
pub type Fwlthtl6 = crate::RegValueT<Fwlthtl6_SPEC>;

impl Fwlthtl6 {
    #[doc = "L3 FRER Number Learn"]
    #[inline(always)]
    pub fn lthfrernl(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwlthtl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwlthtl6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 FRER Valid Learn"]
    #[inline(always)]
    pub fn lthfrervl(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwlthtl6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Fwlthtl6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Meter Number Learn"]
    #[inline(always)]
    pub fn lthmtrnl(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Fwlthtl6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Fwlthtl6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Meter Valid Learn"]
    #[inline(always)]
    pub fn lthmtrvl(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtl6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtl6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtl6 {
    #[inline(always)]
    fn default() -> Fwlthtl6 {
        <crate::RegValueT<Fwlthtl6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl7_SPEC;
impl crate::sealed::RegSpec for Fwlthtl7_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 7"]
pub type Fwlthtl7 = crate::RegValueT<Fwlthtl7_SPEC>;

impl Fwlthtl7 {
    #[doc = "L3 Routing Number Learn"]
    #[inline(always)]
    pub fn lthrnl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwlthtl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwlthtl7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Routing Valid Learn"]
    #[inline(always)]
    pub fn lthrvl(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwlthtl7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Fwlthtl7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Source Lock Vector Learn"]
    #[inline(always)]
    pub fn lthslvl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwlthtl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwlthtl7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtl7 {
    #[inline(always)]
    fn default() -> Fwlthtl7 {
        <crate::RegValueT<Fwlthtl7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl80_SPEC;
impl crate::sealed::RegSpec for Fwlthtl80_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 80"]
pub type Fwlthtl80 = crate::RegValueT<Fwlthtl80_SPEC>;

impl Fwlthtl80 {
    #[doc = "L3 CPU Sub-Destination Learn"]
    #[inline(always)]
    pub fn lthcsdl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwlthtl80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwlthtl80_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtl80 {
    #[inline(always)]
    fn default() -> Fwlthtl80 {
        <crate::RegValueT<Fwlthtl80_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtl9_SPEC;
impl crate::sealed::RegSpec for Fwlthtl9_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Register 9"]
pub type Fwlthtl9 = crate::RegValueT<Fwlthtl9_SPEC>;

impl Fwlthtl9 {
    #[doc = "L3 Destination Vector Learn"]
    #[inline(always)]
    pub fn lthdvl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwlthtl9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwlthtl9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Internal Priority Value Learn"]
    #[inline(always)]
    pub fn lthipvl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwlthtl9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwlthtl9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Internal Priority Update Learn"]
    #[inline(always)]
    pub fn lthipul(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwlthtl9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Fwlthtl9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 Ethernet Mirroring Enable Learn"]
    #[inline(always)]
    pub fn lthemel(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwlthtl9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Fwlthtl9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "L3 CPU Mirroring Enable Learn"]
    #[inline(always)]
    pub fn lthcmel(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwlthtl9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Fwlthtl9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtl9 {
    #[inline(always)]
    fn default() -> Fwlthtl9 {
        <crate::RegValueT<Fwlthtl9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtlr_SPEC;
impl crate::sealed::RegSpec for Fwlthtlr_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Learn Result Register"]
pub type Fwlthtlr = crate::RegValueT<Fwlthtlr_SPEC>;

impl Fwlthtlr {
    #[doc = "L3 Learn Fail"]
    #[inline(always)]
    pub fn lthlf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwlthtlr::Lthlf,
        fwlthtlr::Lthlf,
        Fwlthtlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwlthtlr::Lthlf,
            fwlthtlr::Lthlf,
            Fwlthtlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "L3 Learn Security Fail"]
    #[inline(always)]
    pub fn lthlsf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwlthtlr::Lthlsf,
        fwlthtlr::Lthlsf,
        Fwlthtlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwlthtlr::Lthlsf,
            fwlthtlr::Lthlsf,
            Fwlthtlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "L3 Learn Overwrite"]
    #[inline(always)]
    pub fn lthlo(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwlthtlr::Lthlo,
        fwlthtlr::Lthlo,
        Fwlthtlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwlthtlr::Lthlo,
            fwlthtlr::Lthlo,
            Fwlthtlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "L3 Learn Collision Number"]
    #[inline(always)]
    pub fn lthlcn(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Fwlthtlr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Fwlthtlr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Table Learn"]
    #[inline(always)]
    pub fn lthtl(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtlr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fwlthtlr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwlthtlr {
    #[inline(always)]
    fn default() -> Fwlthtlr {
        <crate::RegValueT<Fwlthtlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwlthtlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthlf_SPEC;
    pub type Lthlf = crate::EnumBitfieldStruct<u8, Lthlf_SPEC>;
    impl Lthlf {
        #[doc = "Entry learning did not fail. (See .)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed. (See .)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthlsf_SPEC;
    pub type Lthlsf = crate::EnumBitfieldStruct<u8, Lthlsf_SPEC>;
    impl Lthlsf {
        #[doc = "Entry learning did not fail. (See .)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed. (See .)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthlo_SPEC;
    pub type Lthlo = crate::EnumBitfieldStruct<u8, Lthlo_SPEC>;
    impl Lthlo {
        #[doc = "The entry learning did not overwrite an existing entry."]
        pub const _0: Self = Self::new(0);

        #[doc = "The entry learning overwrote an existing entry."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtim_SPEC;
impl crate::sealed::RegSpec for Fwlthtim_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Initialization Monitoring Register"]
pub type Fwlthtim = crate::RegValueT<Fwlthtim_SPEC>;

impl Fwlthtim {
    #[doc = "L3 Table Initialization Ongoing"]
    #[inline(always)]
    pub fn lthtiog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwlthtim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fwlthtim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "L3 Table Ready"]
    #[inline(always)]
    pub fn lthtr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fwlthtim_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fwlthtim_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwlthtim {
    #[inline(always)]
    fn default() -> Fwlthtim {
        <crate::RegValueT<Fwlthtim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtem_SPEC;
impl crate::sealed::RegSpec for Fwlthtem_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Entry Monitoring Register"]
pub type Fwlthtem = crate::RegValueT<Fwlthtem_SPEC>;

impl Fwlthtem {
    #[doc = "L3 Table Entry Number"]
    #[inline(always)]
    pub fn lthten(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Fwlthtem_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Fwlthtem_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Table Unsecure Entry Number"]
    #[inline(always)]
    pub fn lthtuen(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Fwlthtem_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Fwlthtem_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtem {
    #[inline(always)]
    fn default() -> Fwlthtem {
        <crate::RegValueT<Fwlthtem_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthts0_SPEC;
impl crate::sealed::RegSpec for Fwlthts0_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Register 0"]
pub type Fwlthts0 = crate::RegValueT<Fwlthts0_SPEC>;

impl Fwlthts0 {
    #[doc = "L3 Stream Search Part 0"]
    #[inline(always)]
    pub fn lthssp0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwlthts0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwlthts0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthts0 {
    #[inline(always)]
    fn default() -> Fwlthts0 {
        <crate::RegValueT<Fwlthts0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthts1_SPEC;
impl crate::sealed::RegSpec for Fwlthts1_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Register 1"]
pub type Fwlthts1 = crate::RegValueT<Fwlthts1_SPEC>;

impl Fwlthts1 {
    #[doc = "L3 Stream Search Part 1"]
    #[inline(always)]
    pub fn lthssp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthts1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthts1 {
    #[inline(always)]
    fn default() -> Fwlthts1 {
        <crate::RegValueT<Fwlthts1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthts2_SPEC;
impl crate::sealed::RegSpec for Fwlthts2_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Register 2"]
pub type Fwlthts2 = crate::RegValueT<Fwlthts2_SPEC>;

impl Fwlthts2 {
    #[doc = "L3 Stream Search Part 2"]
    #[inline(always)]
    pub fn lthssp2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthts2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthts2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthts2 {
    #[inline(always)]
    fn default() -> Fwlthts2 {
        <crate::RegValueT<Fwlthts2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthts3_SPEC;
impl crate::sealed::RegSpec for Fwlthts3_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Register 3"]
pub type Fwlthts3 = crate::RegValueT<Fwlthts3_SPEC>;

impl Fwlthts3 {
    #[doc = "L3 Stream Search Part 3"]
    #[inline(always)]
    pub fn lthssp3(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthts3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthts3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthts3 {
    #[inline(always)]
    fn default() -> Fwlthts3 {
        <crate::RegValueT<Fwlthts3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthts4_SPEC;
impl crate::sealed::RegSpec for Fwlthts4_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Register 4"]
pub type Fwlthts4 = crate::RegValueT<Fwlthts4_SPEC>;

impl Fwlthts4 {
    #[doc = "L3 Stream Search Part 4"]
    #[inline(always)]
    pub fn lthssp4(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthts4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthts4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthts4 {
    #[inline(always)]
    fn default() -> Fwlthts4 {
        <crate::RegValueT<Fwlthts4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtsr0_SPEC;
impl crate::sealed::RegSpec for Fwlthtsr0_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Result Register 0"]
pub type Fwlthtsr0 = crate::RegValueT<Fwlthtsr0_SPEC>;

impl Fwlthtsr0 {
    #[doc = "L3 Search Not found"]
    #[inline(always)]
    pub fn lthsnf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwlthtsr0::Lthsnf,
        fwlthtsr0::Lthsnf,
        Fwlthtsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwlthtsr0::Lthsnf,
            fwlthtsr0::Lthsnf,
            Fwlthtsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "L3 Security Level Search"]
    #[inline(always)]
    pub fn lthsls(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwlthtsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwlthtsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "L3 Search Collision Number"]
    #[inline(always)]
    pub fn lthscn(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Fwlthtsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Fwlthtsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Table Search"]
    #[inline(always)]
    pub fn lthts(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtsr0 {
    #[inline(always)]
    fn default() -> Fwlthtsr0 {
        <crate::RegValueT<Fwlthtsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwlthtsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthsnf_SPEC;
    pub type Lthsnf = crate::EnumBitfieldStruct<u8, Lthsnf_SPEC>;
    impl Lthsnf {
        #[doc = "Stream ID found in L3 Table."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stream ID not found in L3 Table."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtsr1_SPEC;
impl crate::sealed::RegSpec for Fwlthtsr1_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Result Register 1"]
pub type Fwlthtsr1 = crate::RegValueT<Fwlthtsr1_SPEC>;

impl Fwlthtsr1 {
    #[doc = "L3 MSDU Number Search"]
    #[inline(always)]
    pub fn lthmsduns(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Fwlthtsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Fwlthtsr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 MSDU Valid Search"]
    #[inline(always)]
    pub fn lthmsduvs(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtsr1 {
    #[inline(always)]
    fn default() -> Fwlthtsr1 {
        <crate::RegValueT<Fwlthtsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtsr2_SPEC;
impl crate::sealed::RegSpec for Fwlthtsr2_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Result Register 2"]
pub type Fwlthtsr2 = crate::RegValueT<Fwlthtsr2_SPEC>;

impl Fwlthtsr2 {
    #[doc = "L3 FRER Number Search"]
    #[inline(always)]
    pub fn lthfrerns(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwlthtsr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwlthtsr2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 FRER Valid Search"]
    #[inline(always)]
    pub fn lthfrervs(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwlthtsr2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Fwlthtsr2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Meter Number Search"]
    #[inline(always)]
    pub fn lthmtrns(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Fwlthtsr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Fwlthtsr2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Meter Valid Search"]
    #[inline(always)]
    pub fn lthmtrvs(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtsr2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtsr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtsr2 {
    #[inline(always)]
    fn default() -> Fwlthtsr2 {
        <crate::RegValueT<Fwlthtsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtsr3_SPEC;
impl crate::sealed::RegSpec for Fwlthtsr3_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Result Register 3"]
pub type Fwlthtsr3 = crate::RegValueT<Fwlthtsr3_SPEC>;

impl Fwlthtsr3 {
    #[doc = "L3 Routing Number Search"]
    #[inline(always)]
    pub fn lthrns(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwlthtsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwlthtsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Routing Valid Search"]
    #[inline(always)]
    pub fn lthrvs(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwlthtsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Fwlthtsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Source Lock Vector Search"]
    #[inline(always)]
    pub fn lthslvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwlthtsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwlthtsr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtsr3 {
    #[inline(always)]
    fn default() -> Fwlthtsr3 {
        <crate::RegValueT<Fwlthtsr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtsr40_SPEC;
impl crate::sealed::RegSpec for Fwlthtsr40_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Result Register 40"]
pub type Fwlthtsr40 = crate::RegValueT<Fwlthtsr40_SPEC>;

impl Fwlthtsr40 {
    #[doc = "L3 CPU Sub-Destination Search"]
    #[inline(always)]
    pub fn lthcsds(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwlthtsr40_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwlthtsr40_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtsr40 {
    #[inline(always)]
    fn default() -> Fwlthtsr40 {
        <crate::RegValueT<Fwlthtsr40_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtsr5_SPEC;
impl crate::sealed::RegSpec for Fwlthtsr5_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Search Result Register 5"]
pub type Fwlthtsr5 = crate::RegValueT<Fwlthtsr5_SPEC>;

impl Fwlthtsr5 {
    #[doc = "L3 Destination Vector Search"]
    #[inline(always)]
    pub fn lthdvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwlthtsr5_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwlthtsr5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Internal Priority Value Search"]
    #[inline(always)]
    pub fn lthipvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwlthtsr5_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwlthtsr5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Internal Priority Update Search"]
    #[inline(always)]
    pub fn lthipus(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwlthtsr5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Fwlthtsr5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Ethernet Mirroring Enable Search"]
    #[inline(always)]
    pub fn lthemes(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwlthtsr5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Fwlthtsr5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 CPU Mirroring Enable Search"]
    #[inline(always)]
    pub fn lthcmes(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwlthtsr5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Fwlthtsr5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtsr5 {
    #[inline(always)]
    fn default() -> Fwlthtsr5 {
        <crate::RegValueT<Fwlthtsr5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtr_SPEC;
impl crate::sealed::RegSpec for Fwlthtr_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Register"]
pub type Fwlthtr = crate::RegValueT<Fwlthtr_SPEC>;

impl Fwlthtr {
    #[doc = "L3 Address Read"]
    #[inline(always)]
    pub fn lthar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwlthtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwlthtr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtr {
    #[inline(always)]
    fn default() -> Fwlthtr {
        <crate::RegValueT<Fwlthtr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr0_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr0_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 0"]
pub type Fwlthtrr0 = crate::RegValueT<Fwlthtrr0_SPEC>;

impl Fwlthtrr0 {
    #[doc = "L3 Entry Valid Read"]
    #[inline(always)]
    pub fn lthevr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fwlthtrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fwlthtrr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "L3 Table Read"]
    #[inline(always)]
    pub fn lthtr(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtrr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtrr0 {
    #[inline(always)]
    fn default() -> Fwlthtrr0 {
        <crate::RegValueT<Fwlthtrr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr1_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr1_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 1"]
pub type Fwlthtrr1 = crate::RegValueT<Fwlthtrr1_SPEC>;

impl Fwlthtrr1 {
    #[doc = "L3 Stream Read Part 0"]
    #[inline(always)]
    pub fn lthsrp0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwlthtrr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwlthtrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Security Level Read"]
    #[inline(always)]
    pub fn lthslr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwlthtrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwlthtrr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwlthtrr1 {
    #[inline(always)]
    fn default() -> Fwlthtrr1 {
        <crate::RegValueT<Fwlthtrr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr2_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr2_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 2"]
pub type Fwlthtrr2 = crate::RegValueT<Fwlthtrr2_SPEC>;

impl Fwlthtrr2 {
    #[doc = "L3 Stream Read Part 1"]
    #[inline(always)]
    pub fn lthsrp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtrr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtrr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtrr2 {
    #[inline(always)]
    fn default() -> Fwlthtrr2 {
        <crate::RegValueT<Fwlthtrr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr3_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr3_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 3"]
pub type Fwlthtrr3 = crate::RegValueT<Fwlthtrr3_SPEC>;

impl Fwlthtrr3 {
    #[doc = "L3 Stream Read Part 2"]
    #[inline(always)]
    pub fn lthsrp2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtrr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtrr3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtrr3 {
    #[inline(always)]
    fn default() -> Fwlthtrr3 {
        <crate::RegValueT<Fwlthtrr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr4_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr4_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 4"]
pub type Fwlthtrr4 = crate::RegValueT<Fwlthtrr4_SPEC>;

impl Fwlthtrr4 {
    #[doc = "L3 Stream Read Part 3"]
    #[inline(always)]
    pub fn lthsrp3(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtrr4_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtrr4_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtrr4 {
    #[inline(always)]
    fn default() -> Fwlthtrr4 {
        <crate::RegValueT<Fwlthtrr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr5_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr5_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 5"]
pub type Fwlthtrr5 = crate::RegValueT<Fwlthtrr5_SPEC>;

impl Fwlthtrr5 {
    #[doc = "L3 Stream Read Part 4"]
    #[inline(always)]
    pub fn lthsrp4(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthtrr5_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthtrr5_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthtrr5 {
    #[inline(always)]
    fn default() -> Fwlthtrr5 {
        <crate::RegValueT<Fwlthtrr5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr6_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr6_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 6"]
pub type Fwlthtrr6 = crate::RegValueT<Fwlthtrr6_SPEC>;

impl Fwlthtrr6 {
    #[doc = "L3 MSDU Number Read"]
    #[inline(always)]
    pub fn lthmsdunr(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Fwlthtrr6_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Fwlthtrr6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 MSDU Valid Read"]
    #[inline(always)]
    pub fn lthmsduvr(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtrr6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtrr6_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtrr6 {
    #[inline(always)]
    fn default() -> Fwlthtrr6 {
        <crate::RegValueT<Fwlthtrr6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr7_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr7_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 7"]
pub type Fwlthtrr7 = crate::RegValueT<Fwlthtrr7_SPEC>;

impl Fwlthtrr7 {
    #[doc = "L3 FRER Number Read"]
    #[inline(always)]
    pub fn lthfrernr(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwlthtrr7_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwlthtrr7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 FRER Valid Read"]
    #[inline(always)]
    pub fn lthfrervr(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwlthtrr7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Fwlthtrr7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Meter Number Read"]
    #[inline(always)]
    pub fn lthmtrnr(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Fwlthtrr7_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Fwlthtrr7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Meter Valid Read"]
    #[inline(always)]
    pub fn lthmtrvr(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwlthtrr7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwlthtrr7_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtrr7 {
    #[inline(always)]
    fn default() -> Fwlthtrr7 {
        <crate::RegValueT<Fwlthtrr7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr8_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr8_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 8"]
pub type Fwlthtrr8 = crate::RegValueT<Fwlthtrr8_SPEC>;

impl Fwlthtrr8 {
    #[doc = "L3 Routing Number Read"]
    #[inline(always)]
    pub fn lthrnr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwlthtrr8_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwlthtrr8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Routing Valid Read"]
    #[inline(always)]
    pub fn lthrvr(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwlthtrr8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Fwlthtrr8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Source Lock Vector Read"]
    #[inline(always)]
    pub fn lthslvr(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwlthtrr8_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwlthtrr8_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtrr8 {
    #[inline(always)]
    fn default() -> Fwlthtrr8 {
        <crate::RegValueT<Fwlthtrr8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr90_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr90_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 90"]
pub type Fwlthtrr90 = crate::RegValueT<Fwlthtrr90_SPEC>;

impl Fwlthtrr90 {
    #[doc = "L3 CPU Sub-Destination Read"]
    #[inline(always)]
    pub fn lthcsdr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwlthtrr90_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwlthtrr90_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtrr90 {
    #[inline(always)]
    fn default() -> Fwlthtrr90 {
        <crate::RegValueT<Fwlthtrr90_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthtrr10_SPEC;
impl crate::sealed::RegSpec for Fwlthtrr10_SPEC {
    type DataType = u32;
}

#[doc = "L3 Table Read Result Register 10"]
pub type Fwlthtrr10 = crate::RegValueT<Fwlthtrr10_SPEC>;

impl Fwlthtrr10 {
    #[doc = "L3 Destination Vector Read"]
    #[inline(always)]
    pub fn lthdvr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwlthtrr10_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwlthtrr10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Internal Priority Value Read"]
    #[inline(always)]
    pub fn lthipvr(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwlthtrr10_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwlthtrr10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Internal Priority Update Read"]
    #[inline(always)]
    pub fn lthipur(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwlthtrr10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Fwlthtrr10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 Ethernet Mirroring Enable Read"]
    #[inline(always)]
    pub fn lthemer(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwlthtrr10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Fwlthtrr10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "L3 CPU Mirroring Enable Read"]
    #[inline(always)]
    pub fn lthcmer(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwlthtrr10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Fwlthtrr10_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthtrr10 {
    #[inline(always)]
    fn default() -> Fwlthtrr10 {
        <crate::RegValueT<Fwlthtrr10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmachec_SPEC;
impl crate::sealed::RegSpec for Fwmachec_SPEC {
    type DataType = u32;
}

#[doc = "MAC Hash Entry Configuration Register"]
pub type Fwmachec = crate::RegValueT<Fwmachec_SPEC>;

impl Fwmachec {
    #[doc = "MAC Hash Maximum Collision"]
    #[inline(always)]
    pub fn machmc(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fwmachec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fwmachec_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC Hash Maximum Unsecure Entry"]
    #[inline(always)]
    pub fn machmue(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Fwmachec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Fwmachec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmachec {
    #[inline(always)]
    fn default() -> Fwmachec {
        <crate::RegValueT<Fwmachec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmachc_SPEC;
impl crate::sealed::RegSpec for Fwmachc_SPEC {
    type DataType = u32;
}

#[doc = "MAC Hash Configuration Register"]
pub type Fwmachc = crate::RegValueT<Fwmachc_SPEC>;

impl Fwmachc {
    #[doc = "MAC Hash Equation n (n = 0 to 10)"]
    #[inline(always)]
    pub fn mache10_to_mache0(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fwmachc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fwmachc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmachc {
    #[inline(always)]
    fn default() -> Fwmachc {
        <crate::RegValueT<Fwmachc_SPEC> as RegisterValue<_>>::new(2047)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactl0_SPEC;
impl crate::sealed::RegSpec for Fwmactl0_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Learn Register 0"]
pub type Fwmactl0 = crate::RegValueT<Fwmactl0_SPEC>;

impl Fwmactl0 {
    #[doc = "MAC Security Level Learn"]
    #[inline(always)]
    pub fn macsll(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwmactl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwmactl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Dynamic Entry Learn"]
    #[inline(always)]
    pub fn macdel(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Fwmactl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Fwmactl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Hardware Learning Disable Learn"]
    #[inline(always)]
    pub fn machldl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fwmactl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Fwmactl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC Entry Delete"]
    #[inline(always)]
    pub fn maced(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwmactl0::Maced,
        fwmactl0::Maced,
        Fwmactl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwmactl0::Maced,
            fwmactl0::Maced,
            Fwmactl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwmactl0 {
    #[inline(always)]
    fn default() -> Fwmactl0 {
        <crate::RegValueT<Fwmactl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwmactl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Maced_SPEC;
    pub type Maced = crate::EnumBitfieldStruct<u8, Maced_SPEC>;
    impl Maced {
        #[doc = "Learn/overwrite the set MAC address in MAC table."]
        pub const _0: Self = Self::new(0);

        #[doc = "Delete the set MAC address in MAC table."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactl1_SPEC;
impl crate::sealed::RegSpec for Fwmactl1_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Learn Register 1"]
pub type Fwmactl1 = crate::RegValueT<Fwmactl1_SPEC>;

impl Fwmactl1 {
    #[doc = "MAC MAC address Learn Part 0"]
    #[inline(always)]
    pub fn macmalp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwmactl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwmactl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactl1 {
    #[inline(always)]
    fn default() -> Fwmactl1 {
        <crate::RegValueT<Fwmactl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactl2_SPEC;
impl crate::sealed::RegSpec for Fwmactl2_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Learn Register 2"]
pub type Fwmactl2 = crate::RegValueT<Fwmactl2_SPEC>;

impl Fwmactl2 {
    #[doc = "MAC MAC address Learn Part 1"]
    #[inline(always)]
    pub fn macmalp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwmactl2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwmactl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwmactl2 {
    #[inline(always)]
    fn default() -> Fwmactl2 {
        <crate::RegValueT<Fwmactl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactl3_SPEC;
impl crate::sealed::RegSpec for Fwmactl3_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Learn Register 3"]
pub type Fwmactl3 = crate::RegValueT<Fwmactl3_SPEC>;

impl Fwmactl3 {
    #[doc = "MAC Source Source Lock Vector Learn"]
    #[inline(always)]
    pub fn macsslvl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwmactl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwmactl3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC Destination Source Lock Vector Learn"]
    #[inline(always)]
    pub fn macdslvl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwmactl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwmactl3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactl3 {
    #[inline(always)]
    fn default() -> Fwmactl3 {
        <crate::RegValueT<Fwmactl3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactl40_SPEC;
impl crate::sealed::RegSpec for Fwmactl40_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Learn Register 40"]
pub type Fwmactl40 = crate::RegValueT<Fwmactl40_SPEC>;

impl Fwmactl40 {
    #[doc = "MAC CPU Sub-Destination Learn"]
    #[inline(always)]
    pub fn maccsdl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwmactl40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwmactl40_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactl40 {
    #[inline(always)]
    fn default() -> Fwmactl40 {
        <crate::RegValueT<Fwmactl40_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactl5_SPEC;
impl crate::sealed::RegSpec for Fwmactl5_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Learn Register 5"]
pub type Fwmactl5 = crate::RegValueT<Fwmactl5_SPEC>;

impl Fwmactl5 {
    #[doc = "MAC Destination Vector Learn"]
    #[inline(always)]
    pub fn macdvl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwmactl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwmactl5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC Internal Priority Value Learn"]
    #[inline(always)]
    pub fn macipvl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwmactl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwmactl5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC Internal Priority Update Learn"]
    #[inline(always)]
    pub fn macipul(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwmactl5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Fwmactl5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC Ethernet Mirroring Enable Learn"]
    #[inline(always)]
    pub fn macemel(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwmactl5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Fwmactl5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC CPU Mirroring Enable Learn"]
    #[inline(always)]
    pub fn maccmel(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwmactl5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Fwmactl5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactl5 {
    #[inline(always)]
    fn default() -> Fwmactl5 {
        <crate::RegValueT<Fwmactl5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactlr_SPEC;
impl crate::sealed::RegSpec for Fwmactlr_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Learn Result Register"]
pub type Fwmactlr = crate::RegValueT<Fwmactlr_SPEC>;

impl Fwmactlr {
    #[doc = "MAC Learn Fail"]
    #[inline(always)]
    pub fn maclf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwmactlr::Maclf,
        fwmactlr::Maclf,
        Fwmactlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwmactlr::Maclf,
            fwmactlr::Maclf,
            Fwmactlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MAC Learn Security Fail"]
    #[inline(always)]
    pub fn maclsf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwmactlr::Maclsf,
        fwmactlr::Maclsf,
        Fwmactlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwmactlr::Maclsf,
            fwmactlr::Maclsf,
            Fwmactlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MAC Learn Overwrite"]
    #[inline(always)]
    pub fn maclo(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwmactlr::Maclo,
        fwmactlr::Maclo,
        Fwmactlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwmactlr::Maclo,
            fwmactlr::Maclo,
            Fwmactlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MAC Learn Collision Number"]
    #[inline(always)]
    pub fn maclcn(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, Fwmactlr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,Fwmactlr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Table Learn"]
    #[inline(always)]
    pub fn mactl(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwmactlr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fwmactlr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwmactlr {
    #[inline(always)]
    fn default() -> Fwmactlr {
        <crate::RegValueT<Fwmactlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwmactlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Maclf_SPEC;
    pub type Maclf = crate::EnumBitfieldStruct<u8, Maclf_SPEC>;
    impl Maclf {
        #[doc = "Entry learning did not fail. (See )."]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed. (See )."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Maclsf_SPEC;
    pub type Maclsf = crate::EnumBitfieldStruct<u8, Maclsf_SPEC>;
    impl Maclsf {
        #[doc = "Entry learning did not fail because of a security error. (See )."]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed because of a security error. (See )."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Maclo_SPEC;
    pub type Maclo = crate::EnumBitfieldStruct<u8, Maclo_SPEC>;
    impl Maclo {
        #[doc = "The entry learning did not overwrite an existing entry."]
        pub const _0: Self = Self::new(0);

        #[doc = "The entry learning overwrote an existing entry."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactim_SPEC;
impl crate::sealed::RegSpec for Fwmactim_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Initialization Monitoring Register"]
pub type Fwmactim = crate::RegValueT<Fwmactim_SPEC>;

impl Fwmactim {
    #[doc = "MAC Table Initialization Ongoing"]
    #[inline(always)]
    pub fn mactiog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwmactim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fwmactim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Table Ready"]
    #[inline(always)]
    pub fn mactr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fwmactim_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fwmactim_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwmactim {
    #[inline(always)]
    fn default() -> Fwmactim {
        <crate::RegValueT<Fwmactim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactem_SPEC;
impl crate::sealed::RegSpec for Fwmactem_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Entry Monitoring Register"]
pub type Fwmactem = crate::RegValueT<Fwmactem_SPEC>;

impl Fwmactem {
    #[doc = "MAC Table Entry Number"]
    #[inline(always)]
    pub fn macten(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Fwmactem_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Fwmactem_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Table Unsecure Entry Number"]
    #[inline(always)]
    pub fn mactuen(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Fwmactem_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Fwmactem_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactem {
    #[inline(always)]
    fn default() -> Fwmactem {
        <crate::RegValueT<Fwmactem_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmacts0_SPEC;
impl crate::sealed::RegSpec for Fwmacts0_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Search Register 0"]
pub type Fwmacts0 = crate::RegValueT<Fwmacts0_SPEC>;

impl Fwmacts0 {
    #[doc = "MAC MAC Address Search Part 0"]
    #[inline(always)]
    pub fn macmasp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwmacts0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwmacts0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmacts0 {
    #[inline(always)]
    fn default() -> Fwmacts0 {
        <crate::RegValueT<Fwmacts0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmacts1_SPEC;
impl crate::sealed::RegSpec for Fwmacts1_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Search Register 1"]
pub type Fwmacts1 = crate::RegValueT<Fwmacts1_SPEC>;

impl Fwmacts1 {
    #[doc = "MAC MAC Address Search Part 1"]
    #[inline(always)]
    pub fn macmasp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwmacts1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwmacts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwmacts1 {
    #[inline(always)]
    fn default() -> Fwmacts1 {
        <crate::RegValueT<Fwmacts1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactsr0_SPEC;
impl crate::sealed::RegSpec for Fwmactsr0_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Search Result Register 0"]
pub type Fwmactsr0 = crate::RegValueT<Fwmactsr0_SPEC>;

impl Fwmactsr0 {
    #[doc = "MAC Search Not found"]
    #[inline(always)]
    pub fn macsnf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwmactsr0::Macsnf,
        fwmactsr0::Macsnf,
        Fwmactsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwmactsr0::Macsnf,
            fwmactsr0::Macsnf,
            Fwmactsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MAC Security Level Search"]
    #[inline(always)]
    pub fn macsls(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwmactsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwmactsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Dynamic Entry Search"]
    #[inline(always)]
    pub fn macdes(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Fwmactsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Fwmactsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Hardware Learning Disable Search"]
    #[inline(always)]
    pub fn machlds(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fwmactsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,Fwmactsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Search Collision Number"]
    #[inline(always)]
    pub fn macscn(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, Fwmactsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,Fwmactsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Table Search"]
    #[inline(always)]
    pub fn macts(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwmactsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwmactsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactsr0 {
    #[inline(always)]
    fn default() -> Fwmactsr0 {
        <crate::RegValueT<Fwmactsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwmactsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macsnf_SPEC;
    pub type Macsnf = crate::EnumBitfieldStruct<u8, Macsnf_SPEC>;
    impl Macsnf {
        #[doc = "MAC address found in MAC Table."]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC address not found in MAC Table."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactsr1_SPEC;
impl crate::sealed::RegSpec for Fwmactsr1_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Search Result Register 1"]
pub type Fwmactsr1 = crate::RegValueT<Fwmactsr1_SPEC>;

impl Fwmactsr1 {
    #[doc = "MAC Source Source Lock Vector Search"]
    #[inline(always)]
    pub fn macsslvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwmactsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwmactsr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Destination Source Lock Vector Search"]
    #[inline(always)]
    pub fn macdslvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwmactsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwmactsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactsr1 {
    #[inline(always)]
    fn default() -> Fwmactsr1 {
        <crate::RegValueT<Fwmactsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactsr20_SPEC;
impl crate::sealed::RegSpec for Fwmactsr20_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Search Result Register 20"]
pub type Fwmactsr20 = crate::RegValueT<Fwmactsr20_SPEC>;

impl Fwmactsr20 {
    #[doc = "MAC CPU Sub-Destination Search"]
    #[inline(always)]
    pub fn maccsds(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwmactsr20_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwmactsr20_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactsr20 {
    #[inline(always)]
    fn default() -> Fwmactsr20 {
        <crate::RegValueT<Fwmactsr20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactsr3_SPEC;
impl crate::sealed::RegSpec for Fwmactsr3_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Search Result Register 3"]
pub type Fwmactsr3 = crate::RegValueT<Fwmactsr3_SPEC>;

impl Fwmactsr3 {
    #[doc = "MAC Destination Vector Search"]
    #[inline(always)]
    pub fn macdvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwmactsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwmactsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Internal Priority Value Search"]
    #[inline(always)]
    pub fn macipvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwmactsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwmactsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Internal Priority Update Search"]
    #[inline(always)]
    pub fn macipus(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwmactsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Fwmactsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Ethernet Mirroring Enable Search"]
    #[inline(always)]
    pub fn macemes(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwmactsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Fwmactsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC CPU Mirroring Enable Search"]
    #[inline(always)]
    pub fn maccmes(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwmactsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Fwmactsr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactsr3 {
    #[inline(always)]
    fn default() -> Fwmactsr3 {
        <crate::RegValueT<Fwmactsr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactr_SPEC;
impl crate::sealed::RegSpec for Fwmactr_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Register"]
pub type Fwmactr = crate::RegValueT<Fwmactr_SPEC>;

impl Fwmactr {
    #[doc = "MAC Address Read"]
    #[inline(always)]
    pub fn macar(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fwmactr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fwmactr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactr {
    #[inline(always)]
    fn default() -> Fwmactr {
        <crate::RegValueT<Fwmactr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactrr0_SPEC;
impl crate::sealed::RegSpec for Fwmactrr0_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Result Register 0"]
pub type Fwmactrr0 = crate::RegValueT<Fwmactrr0_SPEC>;

impl Fwmactrr0 {
    #[doc = "MAC Entry Valid Read"]
    #[inline(always)]
    pub fn macevr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwmactrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fwmactrr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Table Read"]
    #[inline(always)]
    pub fn mactr(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwmactrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwmactrr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactrr0 {
    #[inline(always)]
    fn default() -> Fwmactrr0 {
        <crate::RegValueT<Fwmactrr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactrr1_SPEC;
impl crate::sealed::RegSpec for Fwmactrr1_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Result Register 1"]
pub type Fwmactrr1 = crate::RegValueT<Fwmactrr1_SPEC>;

impl Fwmactrr1 {
    #[doc = "MAC Security Level Read"]
    #[inline(always)]
    pub fn macslr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwmactrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwmactrr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Dynamic Entry Read"]
    #[inline(always)]
    pub fn macder(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Fwmactrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Fwmactrr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Hardware Learn Disable Read"]
    #[inline(always)]
    pub fn machldr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fwmactrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,Fwmactrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Aging Bit Read"]
    #[inline(always)]
    pub fn macabr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Fwmactrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,Fwmactrr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactrr1 {
    #[inline(always)]
    fn default() -> Fwmactrr1 {
        <crate::RegValueT<Fwmactrr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactrr2_SPEC;
impl crate::sealed::RegSpec for Fwmactrr2_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Result Register 2"]
pub type Fwmactrr2 = crate::RegValueT<Fwmactrr2_SPEC>;

impl Fwmactrr2 {
    #[doc = "MAC MAC address Read Part 0"]
    #[inline(always)]
    pub fn macmarp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwmactrr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwmactrr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactrr2 {
    #[inline(always)]
    fn default() -> Fwmactrr2 {
        <crate::RegValueT<Fwmactrr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactrr3_SPEC;
impl crate::sealed::RegSpec for Fwmactrr3_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Result Register 3"]
pub type Fwmactrr3 = crate::RegValueT<Fwmactrr3_SPEC>;

impl Fwmactrr3 {
    #[doc = "MAC MAC Address Read Part 1"]
    #[inline(always)]
    pub fn macmarp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwmactrr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwmactrr3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwmactrr3 {
    #[inline(always)]
    fn default() -> Fwmactrr3 {
        <crate::RegValueT<Fwmactrr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactrr4_SPEC;
impl crate::sealed::RegSpec for Fwmactrr4_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Result Register 4"]
pub type Fwmactrr4 = crate::RegValueT<Fwmactrr4_SPEC>;

impl Fwmactrr4 {
    #[doc = "MAC Source Source Lock Vector Read"]
    #[inline(always)]
    pub fn macsslvr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwmactrr4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwmactrr4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Destination Source Lock Vector Read"]
    #[inline(always)]
    pub fn macdslvr(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwmactrr4_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwmactrr4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactrr4 {
    #[inline(always)]
    fn default() -> Fwmactrr4 {
        <crate::RegValueT<Fwmactrr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactrr50_SPEC;
impl crate::sealed::RegSpec for Fwmactrr50_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Result Register 50"]
pub type Fwmactrr50 = crate::RegValueT<Fwmactrr50_SPEC>;

impl Fwmactrr50 {
    #[doc = "MAC CPU Sub-Destination Read"]
    #[inline(always)]
    pub fn maccsdr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwmactrr50_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwmactrr50_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactrr50 {
    #[inline(always)]
    fn default() -> Fwmactrr50 {
        <crate::RegValueT<Fwmactrr50_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmactrr6_SPEC;
impl crate::sealed::RegSpec for Fwmactrr6_SPEC {
    type DataType = u32;
}

#[doc = "MAC Table Read Result Register 6"]
pub type Fwmactrr6 = crate::RegValueT<Fwmactrr6_SPEC>;

impl Fwmactrr6 {
    #[doc = "MAC Destination Vector Read"]
    #[inline(always)]
    pub fn macdvr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwmactrr6_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwmactrr6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Internal Priority Value Read"]
    #[inline(always)]
    pub fn macipvr(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwmactrr6_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwmactrr6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Internal Priority Update Read"]
    #[inline(always)]
    pub fn macipur(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwmactrr6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Fwmactrr6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC Ethernet Mirroring Enable Read"]
    #[inline(always)]
    pub fn macemer(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwmactrr6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Fwmactrr6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "MAC CPU Mirroring Enable Read"]
    #[inline(always)]
    pub fn maccmer(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwmactrr6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Fwmactrr6_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmactrr6 {
    #[inline(always)]
    fn default() -> Fwmactrr6 {
        <crate::RegValueT<Fwmactrr6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmacaguspc_SPEC;
impl crate::sealed::RegSpec for Fwmacaguspc_SPEC {
    type DataType = u32;
}

#[doc = "MAC Aging US Prescaler Configuration Register"]
pub type Fwmacaguspc = crate::RegValueT<Fwmacaguspc_SPEC>;

impl Fwmacaguspc {
    #[doc = "MAC Aging US prescaler"]
    #[inline(always)]
    pub fn macagusp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Fwmacaguspc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Fwmacaguspc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmacaguspc {
    #[inline(always)]
    fn default() -> Fwmacaguspc {
        <crate::RegValueT<Fwmacaguspc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmacagc_SPEC;
impl crate::sealed::RegSpec for Fwmacagc_SPEC {
    type DataType = u32;
}

#[doc = "MAC Aging Configuration Register"]
pub type Fwmacagc = crate::RegValueT<Fwmacagc_SPEC>;

impl Fwmacagc {
    #[doc = "MAC Aging Time"]
    #[inline(always)]
    pub fn macagt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwmacagc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwmacagc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MAC Aging Enable"]
    #[inline(always)]
    pub fn macage(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwmacagc::Macage,
        fwmacagc::Macage,
        Fwmacagc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwmacagc::Macage,
            fwmacagc::Macage,
            Fwmacagc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Aging Security Level"]
    #[inline(always)]
    pub fn macagsl(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwmacagc::Macagsl,
        fwmacagc::Macagsl,
        Fwmacagc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwmacagc::Macagsl,
            fwmacagc::Macagsl,
            Fwmacagc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Aging Polling Mode"]
    #[inline(always)]
    pub fn macagpm(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fwmacagc::Macagpm,
        fwmacagc::Macagpm,
        Fwmacagc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fwmacagc::Macagpm,
            fwmacagc::Macagpm,
            Fwmacagc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Dynamic Entry Suppression"]
    #[inline(always)]
    pub fn macdes(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwmacagc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fwmacagc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Aging Ongoing"]
    #[inline(always)]
    pub fn macagog(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        fwmacagc::Macagog,
        fwmacagc::Macagog,
        Fwmacagc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            fwmacagc::Macagog,
            fwmacagc::Macagog,
            Fwmacagc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MAC Dynamic Entry Suppression Ongoing"]
    #[inline(always)]
    pub fn macdesog(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        fwmacagc::Macdesog,
        fwmacagc::Macdesog,
        Fwmacagc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            fwmacagc::Macdesog,
            fwmacagc::Macdesog,
            Fwmacagc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwmacagc {
    #[inline(always)]
    fn default() -> Fwmacagc {
        <crate::RegValueT<Fwmacagc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwmacagc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macage_SPEC;
    pub type Macage = crate::EnumBitfieldStruct<u8, Macage_SPEC>;
    impl Macage {
        #[doc = "MAC aging is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC aging is enabled and will happen every FWMACAGC.MACAGT seconds."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macagsl_SPEC;
    pub type Macagsl = crate::EnumBitfieldStruct<u8, Macagsl_SPEC>;
    impl Macagsl {
        #[doc = "Only unsecure entries will be aged (MAC.SL == 0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "All entries will be aged."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macagpm_SPEC;
    pub type Macagpm = crate::EnumBitfieldStruct<u8, Macagpm_SPEC>;
    impl Macagpm {
        #[doc = "MAC aging polling mode disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC aging polling mode enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macagog_SPEC;
    pub type Macagog = crate::EnumBitfieldStruct<u8, Macagog_SPEC>;
    impl Macagog {
        #[doc = "MAC aging is not ongoing."]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC aging is ongoing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macdesog_SPEC;
    pub type Macdesog = crate::EnumBitfieldStruct<u8, Macdesog_SPEC>;
    impl Macdesog {
        #[doc = "MAC Dynamic Entry Suppression is not ongoing."]
        pub const _0: Self = Self::new(0);

        #[doc = "MAC Dynamic Entry Suppression is ongoing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmacagm0_SPEC;
impl crate::sealed::RegSpec for Fwmacagm0_SPEC {
    type DataType = u32;
}

#[doc = "MAC Aging Monitoring Register 0"]
pub type Fwmacagm0 = crate::RegValueT<Fwmacagm0_SPEC>;

impl Fwmacagm0 {
    #[doc = "Aged MAC Address Part 0"]
    #[inline(always)]
    pub fn agmacap0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwmacagm0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwmacagm0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmacagm0 {
    #[inline(always)]
    fn default() -> Fwmacagm0 {
        <crate::RegValueT<Fwmacagm0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmacagm1_SPEC;
impl crate::sealed::RegSpec for Fwmacagm1_SPEC {
    type DataType = u32;
}

#[doc = "MAC Aging Monitoring Register 1"]
pub type Fwmacagm1 = crate::RegValueT<Fwmacagm1_SPEC>;

impl Fwmacagm1 {
    #[doc = "Aged MAC Address Part 1"]
    #[inline(always)]
    pub fn agmacap1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwmacagm1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwmacagm1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwmacagm1 {
    #[inline(always)]
    fn default() -> Fwmacagm1 {
        <crate::RegValueT<Fwmacagm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantec_SPEC;
impl crate::sealed::RegSpec for Fwvlantec_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Entry Configuration Register"]
pub type Fwvlantec = crate::RegValueT<Fwvlantec_SPEC>;

impl Fwvlantec {
    #[doc = "VLAN Table Maximum Unsecure Entry"]
    #[inline(always)]
    pub fn vlantmue(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Fwvlantec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Fwvlantec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantec {
    #[inline(always)]
    fn default() -> Fwvlantec {
        <crate::RegValueT<Fwvlantec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantl0_SPEC;
impl crate::sealed::RegSpec for Fwvlantl0_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Learn Register 0"]
pub type Fwvlantl0 = crate::RegValueT<Fwvlantl0_SPEC>;

impl Fwvlantl0 {
    #[doc = "VLAN Security Level Learn"]
    #[inline(always)]
    pub fn vlansll(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwvlantl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Fwvlantl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VLAN Hardware Learning Disable Learn"]
    #[inline(always)]
    pub fn vlanhldl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fwvlantl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Fwvlantl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VLAN Entry Delete"]
    #[inline(always)]
    pub fn vlaned(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwvlantl0::Vlaned,
        fwvlantl0::Vlaned,
        Fwvlantl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwvlantl0::Vlaned,
            fwvlantl0::Vlaned,
            Fwvlantl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwvlantl0 {
    #[inline(always)]
    fn default() -> Fwvlantl0 {
        <crate::RegValueT<Fwvlantl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwvlantl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlaned_SPEC;
    pub type Vlaned = crate::EnumBitfieldStruct<u8, Vlaned_SPEC>;
    impl Vlaned {
        #[doc = "Learn/overwrite the set VLAN address in VLAN table."]
        pub const _0: Self = Self::new(0);

        #[doc = "Delete the set VLAN address in VLAN table."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantl1_SPEC;
impl crate::sealed::RegSpec for Fwvlantl1_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Learn Register 1"]
pub type Fwvlantl1 = crate::RegValueT<Fwvlantl1_SPEC>;

impl Fwvlantl1 {
    #[doc = "VLAN VID Learn"]
    #[inline(always)]
    pub fn vlanvidl(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Fwvlantl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Fwvlantl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantl1 {
    #[inline(always)]
    fn default() -> Fwvlantl1 {
        <crate::RegValueT<Fwvlantl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantl2_SPEC;
impl crate::sealed::RegSpec for Fwvlantl2_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Learn Register 2"]
pub type Fwvlantl2 = crate::RegValueT<Fwvlantl2_SPEC>;

impl Fwvlantl2 {
    #[doc = "VLAN Source Lock Vector Learn"]
    #[inline(always)]
    pub fn vlanslvl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwvlantl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwvlantl2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantl2 {
    #[inline(always)]
    fn default() -> Fwvlantl2 {
        <crate::RegValueT<Fwvlantl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantl30_SPEC;
impl crate::sealed::RegSpec for Fwvlantl30_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Learn Register 30"]
pub type Fwvlantl30 = crate::RegValueT<Fwvlantl30_SPEC>;

impl Fwvlantl30 {
    #[doc = "VLAN CPU Sub-Destination Learn"]
    #[inline(always)]
    pub fn vlancsdl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwvlantl30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwvlantl30_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantl30 {
    #[inline(always)]
    fn default() -> Fwvlantl30 {
        <crate::RegValueT<Fwvlantl30_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantl4_SPEC;
impl crate::sealed::RegSpec for Fwvlantl4_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Learn Register 4"]
pub type Fwvlantl4 = crate::RegValueT<Fwvlantl4_SPEC>;

impl Fwvlantl4 {
    #[doc = "VLAN Destination Vector Learn"]
    #[inline(always)]
    pub fn vlandvl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwvlantl4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwvlantl4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VLAN Internal Priority Value Learn"]
    #[inline(always)]
    pub fn vlanipvl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwvlantl4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwvlantl4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VLAN Internal Priority Update Learn"]
    #[inline(always)]
    pub fn vlanipul(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwvlantl4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Fwvlantl4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VLAN Ethernet Mirroring Enable Learn"]
    #[inline(always)]
    pub fn vlanemel(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwvlantl4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Fwvlantl4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VLAN CPU Mirroring Enable Learn"]
    #[inline(always)]
    pub fn vlancmel(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwvlantl4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Fwvlantl4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantl4 {
    #[inline(always)]
    fn default() -> Fwvlantl4 {
        <crate::RegValueT<Fwvlantl4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantlr_SPEC;
impl crate::sealed::RegSpec for Fwvlantlr_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Learn Result Register"]
pub type Fwvlantlr = crate::RegValueT<Fwvlantlr_SPEC>;

impl Fwvlantlr {
    #[doc = "VLAN Learn Fail"]
    #[inline(always)]
    pub fn vlanlf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwvlantlr::Vlanlf,
        fwvlantlr::Vlanlf,
        Fwvlantlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwvlantlr::Vlanlf,
            fwvlantlr::Vlanlf,
            Fwvlantlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Learn Security Fail"]
    #[inline(always)]
    pub fn vlanlsf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwvlantlr::Vlanlsf,
        fwvlantlr::Vlanlsf,
        Fwvlantlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwvlantlr::Vlanlsf,
            fwvlantlr::Vlanlsf,
            Fwvlantlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Learn Overwrite"]
    #[inline(always)]
    pub fn vlanlo(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwvlantlr::Vlanlo,
        fwvlantlr::Vlanlo,
        Fwvlantlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwvlantlr::Vlanlo,
            fwvlantlr::Vlanlo,
            Fwvlantlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Table Learn"]
    #[inline(always)]
    pub fn vlantl(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwvlantlr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwvlantlr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantlr {
    #[inline(always)]
    fn default() -> Fwvlantlr {
        <crate::RegValueT<Fwvlantlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwvlantlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlanlf_SPEC;
    pub type Vlanlf = crate::EnumBitfieldStruct<u8, Vlanlf_SPEC>;
    impl Vlanlf {
        #[doc = "Entry learning did not fail. (See .)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed. (See .)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlanlsf_SPEC;
    pub type Vlanlsf = crate::EnumBitfieldStruct<u8, Vlanlsf_SPEC>;
    impl Vlanlsf {
        #[doc = "Entry learning did not fail because of a security error. (See .)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed because of a security error. (See .)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlanlo_SPEC;
    pub type Vlanlo = crate::EnumBitfieldStruct<u8, Vlanlo_SPEC>;
    impl Vlanlo {
        #[doc = "The entry learning did not overwrite an existing entry."]
        pub const _0: Self = Self::new(0);

        #[doc = "The entry learning overwrote an existing entry."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantim_SPEC;
impl crate::sealed::RegSpec for Fwvlantim_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Initialization Monitoring Register"]
pub type Fwvlantim = crate::RegValueT<Fwvlantim_SPEC>;

impl Fwvlantim {
    #[doc = "VLAN Table Initialization Ongoing"]
    #[inline(always)]
    pub fn vlantiog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwvlantim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Fwvlantim_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VLAN Table Ready"]
    #[inline(always)]
    pub fn vlantr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fwvlantim_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fwvlantim_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwvlantim {
    #[inline(always)]
    fn default() -> Fwvlantim {
        <crate::RegValueT<Fwvlantim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantem_SPEC;
impl crate::sealed::RegSpec for Fwvlantem_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Entry Monitoring Register"]
pub type Fwvlantem = crate::RegValueT<Fwvlantem_SPEC>;

impl Fwvlantem {
    #[doc = "VLAN Table Entry Number"]
    #[inline(always)]
    pub fn vlanten(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Fwvlantem_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Fwvlantem_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VLAN Table Unsecure Entry Number"]
    #[inline(always)]
    pub fn vlantuen(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Fwvlantem_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Fwvlantem_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantem {
    #[inline(always)]
    fn default() -> Fwvlantem {
        <crate::RegValueT<Fwvlantem_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlants_SPEC;
impl crate::sealed::RegSpec for Fwvlants_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Search Register"]
pub type Fwvlants = crate::RegValueT<Fwvlants_SPEC>;

impl Fwvlants {
    #[doc = "VLAN VID Search"]
    #[inline(always)]
    pub fn vlanvids(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Fwvlants_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Fwvlants_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlants {
    #[inline(always)]
    fn default() -> Fwvlants {
        <crate::RegValueT<Fwvlants_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantsr0_SPEC;
impl crate::sealed::RegSpec for Fwvlantsr0_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Search Result Register 0"]
pub type Fwvlantsr0 = crate::RegValueT<Fwvlantsr0_SPEC>;

impl Fwvlantsr0 {
    #[doc = "VLAN Search Not found"]
    #[inline(always)]
    pub fn vlansnf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fwvlantsr0::Vlansnf,
        fwvlantsr0::Vlansnf,
        Fwvlantsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fwvlantsr0::Vlansnf,
            fwvlantsr0::Vlansnf,
            Fwvlantsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Security Level Search"]
    #[inline(always)]
    pub fn vlansls(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwvlantsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Fwvlantsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VLAN Hardware Learning Disable Search"]
    #[inline(always)]
    pub fn vlanhlds(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fwvlantsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,Fwvlantsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VLAN Table Search"]
    #[inline(always)]
    pub fn vlants(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwvlantsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwvlantsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantsr0 {
    #[inline(always)]
    fn default() -> Fwvlantsr0 {
        <crate::RegValueT<Fwvlantsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwvlantsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlansnf_SPEC;
    pub type Vlansnf = crate::EnumBitfieldStruct<u8, Vlansnf_SPEC>;
    impl Vlansnf {
        #[doc = "VLAN found in VLAN Table."]
        pub const _0: Self = Self::new(0);

        #[doc = "VLAN not found in VLAN Table."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantsr1_SPEC;
impl crate::sealed::RegSpec for Fwvlantsr1_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Search Result Register 1"]
pub type Fwvlantsr1 = crate::RegValueT<Fwvlantsr1_SPEC>;

impl Fwvlantsr1 {
    #[doc = "VLAN Source Lock Vector Search"]
    #[inline(always)]
    pub fn vlanslvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwvlantsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwvlantsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantsr1 {
    #[inline(always)]
    fn default() -> Fwvlantsr1 {
        <crate::RegValueT<Fwvlantsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantsr20_SPEC;
impl crate::sealed::RegSpec for Fwvlantsr20_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Search Result Register 20"]
pub type Fwvlantsr20 = crate::RegValueT<Fwvlantsr20_SPEC>;

impl Fwvlantsr20 {
    #[doc = "VLAN CPU Sub-Destination Search"]
    #[inline(always)]
    pub fn vlancsds(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwvlantsr20_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwvlantsr20_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantsr20 {
    #[inline(always)]
    fn default() -> Fwvlantsr20 {
        <crate::RegValueT<Fwvlantsr20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwvlantsr3_SPEC;
impl crate::sealed::RegSpec for Fwvlantsr3_SPEC {
    type DataType = u32;
}

#[doc = "VLAN Table Search Result Register 3"]
pub type Fwvlantsr3 = crate::RegValueT<Fwvlantsr3_SPEC>;

impl Fwvlantsr3 {
    #[doc = "VLAN Destination Vector Search"]
    #[inline(always)]
    pub fn vlandvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwvlantsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwvlantsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VLAN Internal Priority Value Search"]
    #[inline(always)]
    pub fn vlanipvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwvlantsr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwvlantsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VLAN Internal Priority Update Search"]
    #[inline(always)]
    pub fn vlanipus(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwvlantsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Fwvlantsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VLAN Ethernet Mirroring Enable Search"]
    #[inline(always)]
    pub fn vlanemes(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwvlantsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Fwvlantsr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VLAN CPU Mirroring Enable Search"]
    #[inline(always)]
    pub fn vlancmes(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwvlantsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Fwvlantsr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwvlantsr3 {
    #[inline(always)]
    fn default() -> Fwvlantsr3 {
        <crate::RegValueT<Fwvlantsr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpbfc_SPEC;
impl crate::sealed::RegSpec for Fwpbfc_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Port Based Forwarding Configuration Register"]
pub type Fwpbfc = crate::RegValueT<Fwpbfc_SPEC>;

impl Fwpbfc {
    #[doc = "Port Based Destination Vector"]
    #[inline(always)]
    pub fn pbdv(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwpbfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwpbfc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Port Based Internal Priority Value"]
    #[inline(always)]
    pub fn pbipv(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwpbfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwpbfc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Port Based Internal Priority Update"]
    #[inline(always)]
    pub fn pbipu(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwpbfc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Fwpbfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Based Ethernet Mirroring Enabled"]
    #[inline(always)]
    pub fn pbeme(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwpbfc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Fwpbfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Based CPU Mirroring Enabled"]
    #[inline(always)]
    pub fn pbcme(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwpbfc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Fwpbfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Based Security Level"]
    #[inline(always)]
    pub fn pbsl(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Fwpbfc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Fwpbfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IPv4 Priority Decode Enable"]
    #[inline(always)]
    pub fn ip4pde(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fwpbfc::Ip4Pde,
        fwpbfc::Ip4Pde,
        Fwpbfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fwpbfc::Ip4Pde,
            fwpbfc::Ip4Pde,
            Fwpbfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv4 Priority Decode Mode"]
    #[inline(always)]
    pub fn ip4pdm(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        fwpbfc::Ip4Pdm,
        fwpbfc::Ip4Pdm,
        Fwpbfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            fwpbfc::Ip4Pdm,
            fwpbfc::Ip4Pdm,
            Fwpbfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IPv6 Priority Decode Enable"]
    #[inline(always)]
    pub fn ip6pde(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        fwpbfc::Ip6Pde,
        fwpbfc::Ip6Pde,
        Fwpbfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            fwpbfc::Ip6Pde,
            fwpbfc::Ip6Pde,
            Fwpbfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Force All Input Frame Priority Enable"]
    #[inline(always)]
    pub fn faifp(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        fwpbfc::Faifp,
        fwpbfc::Faifp,
        Fwpbfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            fwpbfc::Faifp,
            fwpbfc::Faifp,
            Fwpbfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwpbfc {
    #[inline(always)]
    fn default() -> Fwpbfc {
        <crate::RegValueT<Fwpbfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwpbfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Pde_SPEC;
    pub type Ip4Pde = crate::EnumBitfieldStruct<u8, Ip4Pde_SPEC>;
    impl Ip4Pde {
        #[doc = "For port i, Input frame priority is taken from VLAN TAGs for port i."]
        pub const _0: Self = Self::new(0);

        #[doc = "For port i, If an IPv4 frame is detected, input frame priority is taken from precedence ToS. Else Input frame priority is taken from VLAN TAGs."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip4Pdm_SPEC;
    pub type Ip4Pdm = crate::EnumBitfieldStruct<u8, Ip4Pdm_SPEC>;
    impl Ip4Pdm {
        #[doc = "For port i, Input frame priority is taken from precedent field in ToS."]
        pub const _0: Self = Self::new(0);

        #[doc = "For port i, Input frame priority is decoded from DSCP field in ToS."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ip6Pde_SPEC;
    pub type Ip6Pde = crate::EnumBitfieldStruct<u8, Ip6Pde_SPEC>;
    impl Ip6Pde {
        #[doc = "For port i, Input frame priority is taken from VLAN TAGs."]
        pub const _0: Self = Self::new(0);

        #[doc = "For port i, If an IPv6 frame is detected, input frame priority is decoded from DSCP field in ToS. Else Input frame priority is taken from VLAN TAGs."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Faifp_SPEC;
    pub type Faifp = crate::EnumBitfieldStruct<u8, Faifp_SPEC>;
    impl Faifp {
        #[doc = "For port i, Input frame priority is given by frame decoding."]
        pub const _0: Self = Self::new(0);

        #[doc = "For port i, Input frame priority is forced to FWPBFCi.PBIPV."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpbfcsdc0_SPEC;
impl crate::sealed::RegSpec for Fwpbfcsdc0_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Port Based Forwarding CSD Configuration Register 0"]
pub type Fwpbfcsdc0 = crate::RegValueT<Fwpbfcsdc0_SPEC>;

impl Fwpbfcsdc0 {
    #[doc = "Port Based CPU Sub Destination"]
    #[inline(always)]
    pub fn pbcsd(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fwpbfcsdc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fwpbfcsdc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpbfcsdc0 {
    #[inline(always)]
    fn default() -> Fwpbfcsdc0 {
        <crate::RegValueT<Fwpbfcsdc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Url0_SPEC;
impl crate::sealed::RegSpec for Fwl23Url0_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Learn Register 0"]
pub type Fwl23Url0 = crate::RegValueT<Fwl23Url0_SPEC>;

impl Fwl23Url0 {
    #[doc = "Layer 2/Layer 3 Update Routing Number Learn"]
    #[inline(always)]
    pub fn l23urnl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwl23Url0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwl23Url0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update Routing Port Valid Learn"]
    #[inline(always)]
    pub fn l23urpvl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fwl23Url0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fwl23Url0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Url0 {
    #[inline(always)]
    fn default() -> Fwl23Url0 {
        <crate::RegValueT<Fwl23Url0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Url1_SPEC;
impl crate::sealed::RegSpec for Fwl23Url1_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Learn Register 1"]
pub type Fwl23Url1 = crate::RegValueT<Fwl23Url1_SPEC>;

impl Fwl23Url1 {
    #[doc = "Layer 2/Layer 3 Update MAC Destination Address Learn Part 0"]
    #[inline(always)]
    pub fn l23umdalp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwl23Url1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update Time To Live Update Learn"]
    #[inline(always)]
    pub fn l23uttlul(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update MAC Destination Address Update Learn"]
    #[inline(always)]
    pub fn l23umdaul(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update MAC Source Address Update Learn"]
    #[inline(always)]
    pub fn l23umsaul(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update C-TAG VID Update Learn"]
    #[inline(always)]
    pub fn l23ucvidul(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update C-TAG PCP Update Learn"]
    #[inline(always)]
    pub fn l23ucpcpul(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update C-TAG DEI Update Learn"]
    #[inline(always)]
    pub fn l23ucdeiul(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update S-TAG VID Update Learn"]
    #[inline(always)]
    pub fn l23usvidul(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update S-TAG PCP Update Learn"]
    #[inline(always)]
    pub fn l23uspcpul(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update S-TAG DEI Update Learn"]
    #[inline(always)]
    pub fn l23usdeiul(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwl23Url1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update R-TAG Update Learn"]
    #[inline(always)]
    pub fn l23urtul(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, u8, u8, Fwl23Url1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x3,1,0,u8,u8,Fwl23Url1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Url1 {
    #[inline(always)]
    fn default() -> Fwl23Url1 {
        <crate::RegValueT<Fwl23Url1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Url2_SPEC;
impl crate::sealed::RegSpec for Fwl23Url2_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Learn Register 2"]
pub type Fwl23Url2 = crate::RegValueT<Fwl23Url2_SPEC>;

impl Fwl23Url2 {
    #[doc = "Layer 2/Layer 3 Update MAC Destination Address Learn Part 1"]
    #[inline(always)]
    pub fn l23umdalp1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Fwl23Url2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwl23Url2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwl23Url2 {
    #[inline(always)]
    fn default() -> Fwl23Url2 {
        <crate::RegValueT<Fwl23Url2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Url3_SPEC;
impl crate::sealed::RegSpec for Fwl23Url3_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Learn Register 3"]
pub type Fwl23Url3 = crate::RegValueT<Fwl23Url3_SPEC>;

impl Fwl23Url3 {
    #[doc = "Layer 2/Layer 3 Update C-TAG VID Learn"]
    #[inline(always)]
    pub fn l23ucvidl(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Fwl23Url3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Fwl23Url3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update C-TAG PCP Learn"]
    #[inline(always)]
    pub fn l23ucpcpl(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwl23Url3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwl23Url3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update C-TAG DEI Learn"]
    #[inline(always)]
    pub fn l23ucdeil(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwl23Url3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Fwl23Url3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update S-TAG VID Learn"]
    #[inline(always)]
    pub fn l23usvidl(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Fwl23Url3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Fwl23Url3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update S-TAG PCP Learn"]
    #[inline(always)]
    pub fn l23uspcpl(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Fwl23Url3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Fwl23Url3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update S-TAG DEI Learn"]
    #[inline(always)]
    pub fn l23usdeil(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwl23Url3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Fwl23Url3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Url3 {
    #[inline(always)]
    fn default() -> Fwl23Url3 {
        <crate::RegValueT<Fwl23Url3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Urlr_SPEC;
impl crate::sealed::RegSpec for Fwl23Urlr_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Learn Result Register"]
pub type Fwl23Urlr = crate::RegValueT<Fwl23Urlr_SPEC>;

impl Fwl23Urlr {
    #[doc = "Layer 2/Layer 3 Update Learn Fail"]
    #[inline(always)]
    pub fn l23ulf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwl23urlr::L23Ulf,
        fwl23urlr::L23Ulf,
        Fwl23Urlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwl23urlr::L23Ulf,
            fwl23urlr::L23Ulf,
            Fwl23Urlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2/Layer 3 Update Rule Learn"]
    #[inline(always)]
    pub fn l23url(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwl23Urlr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwl23Urlr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Urlr {
    #[inline(always)]
    fn default() -> Fwl23Urlr {
        <crate::RegValueT<Fwl23Urlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwl23urlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L23Ulf_SPEC;
    pub type L23Ulf = crate::EnumBitfieldStruct<u8, L23Ulf_SPEC>;
    impl L23Ulf {
        #[doc = "Entry learning did not fail because the Layer 2/Layer 3 table is ready."]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed because the Layer 2/Layer 3 table is not ready."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Utim_SPEC;
impl crate::sealed::RegSpec for Fwl23Utim_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Table Initialization Monitoring Register"]
pub type Fwl23Utim = crate::RegValueT<Fwl23Utim_SPEC>;

impl Fwl23Utim {
    #[doc = "Layer 2/Layer 3 Update Table Initialization Ongoing"]
    #[inline(always)]
    pub fn l23utiog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwl23Utim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Fwl23Utim_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update Table Ready"]
    #[inline(always)]
    pub fn l23utr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fwl23Utim_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fwl23Utim_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwl23Utim {
    #[inline(always)]
    fn default() -> Fwl23Utim {
        <crate::RegValueT<Fwl23Utim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Urr_SPEC;
impl crate::sealed::RegSpec for Fwl23Urr_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Read Register"]
pub type Fwl23Urr = crate::RegValueT<Fwl23Urr_SPEC>;

impl Fwl23Urr {
    #[doc = "Layer 2/Layer 3 Routing Number Read"]
    #[inline(always)]
    pub fn l23rnr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwl23Urr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwl23Urr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Urr {
    #[inline(always)]
    fn default() -> Fwl23Urr {
        <crate::RegValueT<Fwl23Urr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Urrr0_SPEC;
impl crate::sealed::RegSpec for Fwl23Urrr0_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Read Result Register 0"]
pub type Fwl23Urrr0 = crate::RegValueT<Fwl23Urrr0_SPEC>;

impl Fwl23Urrr0 {
    #[doc = "Layer 2/Layer 3 Update Routing Port Valid Read"]
    #[inline(always)]
    pub fn l23urpvr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fwl23Urrr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fwl23Urrr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update Rule Read"]
    #[inline(always)]
    pub fn l23urr(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwl23Urrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwl23Urrr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Urrr0 {
    #[inline(always)]
    fn default() -> Fwl23Urrr0 {
        <crate::RegValueT<Fwl23Urrr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Urrr1_SPEC;
impl crate::sealed::RegSpec for Fwl23Urrr1_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Read Result Register 1"]
pub type Fwl23Urrr1 = crate::RegValueT<Fwl23Urrr1_SPEC>;

impl Fwl23Urrr1 {
    #[doc = "Layer 2/Layer 3 MAC Destination Address Read Part 0"]
    #[inline(always)]
    pub fn l23umdarp0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwl23Urrr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Time To Live Update Read"]
    #[inline(always)]
    pub fn l23uttlur(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 MAC Destination Address Update Read"]
    #[inline(always)]
    pub fn l23umdaur(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 MAC Source Address Update Read"]
    #[inline(always)]
    pub fn l23umsaur(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 C-TAG VID Update Read"]
    #[inline(always)]
    pub fn l23ucvidur(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 C-TAG PCP Update Read"]
    #[inline(always)]
    pub fn l23ucpcpur(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 C-TAG DEI Update Read"]
    #[inline(always)]
    pub fn l23ucdeiur(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 S-TAG VID Update Read"]
    #[inline(always)]
    pub fn l23usvidur(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 S-TAG PCP Update Read"]
    #[inline(always)]
    pub fn l23uspcpur(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 S-TAG DEI Update Read"]
    #[inline(always)]
    pub fn l23usdeiur(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fwl23Urrr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 R-TAG Update Read"]
    #[inline(always)]
    pub fn l23urtur(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, u8, u8, Fwl23Urrr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x3,1,0,u8,u8,Fwl23Urrr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Urrr1 {
    #[inline(always)]
    fn default() -> Fwl23Urrr1 {
        <crate::RegValueT<Fwl23Urrr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Urrr2_SPEC;
impl crate::sealed::RegSpec for Fwl23Urrr2_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Read Result Register 2"]
pub type Fwl23Urrr2 = crate::RegValueT<Fwl23Urrr2_SPEC>;

impl Fwl23Urrr2 {
    #[doc = "Layer 2/Layer 3 Update MAC Destination Address Read Part 1"]
    #[inline(always)]
    pub fn l23umdarp1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Fwl23Urrr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwl23Urrr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwl23Urrr2 {
    #[inline(always)]
    fn default() -> Fwl23Urrr2 {
        <crate::RegValueT<Fwl23Urrr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Urrr3_SPEC;
impl crate::sealed::RegSpec for Fwl23Urrr3_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Rule Read Result Register 3"]
pub type Fwl23Urrr3 = crate::RegValueT<Fwl23Urrr3_SPEC>;

impl Fwl23Urrr3 {
    #[doc = "Layer 2/Layer 3 Update MAC C-TAG VID Read"]
    #[inline(always)]
    pub fn l23ucvidr(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Fwl23Urrr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Fwl23Urrr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update MAC C-TAG PCP Read"]
    #[inline(always)]
    pub fn l23ucpcpr(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Fwl23Urrr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Fwl23Urrr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update MAC C-TAG DEI Read"]
    #[inline(always)]
    pub fn l23ucdeir(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fwl23Urrr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Fwl23Urrr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update MAC S-TAG VID Read"]
    #[inline(always)]
    pub fn l23usvidr(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Fwl23Urrr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Fwl23Urrr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update MAC S-TAG PCP Read"]
    #[inline(always)]
    pub fn l23uspcpr(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Fwl23Urrr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Fwl23Urrr3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Layer 2/Layer 3 Update MAC S-TAG DEI Read"]
    #[inline(always)]
    pub fn l23usdeir(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fwl23Urrr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Fwl23Urrr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwl23Urrr3 {
    #[inline(always)]
    fn default() -> Fwl23Urrr3 {
        <crate::RegValueT<Fwl23Urrr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwl23Urmc_SPEC;
impl crate::sealed::RegSpec for Fwl23Urmc_SPEC {
    type DataType = u32;
}

#[doc = "Layer 2/Layer 3 Update Remapping Configuration Register %s"]
pub type Fwl23Urmc = crate::RegValueT<Fwl23Urmc_SPEC>;

impl Fwl23Urmc {
    #[doc = "Remapping Rule Number"]
    #[inline(always)]
    pub fn rmrn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwl23Urmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwl23Urmc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Remapping Destination Port Number"]
    #[inline(always)]
    pub fn rmdpn(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Fwl23Urmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Fwl23Urmc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Remapping New Rule Number"]
    #[inline(always)]
    pub fn rmnrn(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Fwl23Urmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Fwl23Urmc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Remapping Enable"]
    #[inline(always)]
    pub fn rme(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        fwl23urmc::Rme,
        fwl23urmc::Rme,
        Fwl23Urmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            fwl23urmc::Rme,
            fwl23urmc::Rme,
            Fwl23Urmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwl23Urmc {
    #[inline(always)]
    fn default() -> Fwl23Urmc {
        <crate::RegValueT<Fwl23Urmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwl23urmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rme_SPEC;
    pub type Rme = crate::EnumBitfieldStruct<u8, Rme_SPEC>;
    impl Rme {
        #[doc = "Remapping rule i disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Remapping rule i enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmfgc_SPEC;
impl crate::sealed::RegSpec for Fwpmfgc_SPEC {
    type DataType = u32;
}

#[doc = "PSFP MSDU Filter Global Configuration Register %s"]
pub type Fwpmfgc = crate::RegValueT<Fwpmfgc_SPEC>;

impl Fwpmfgc {
    #[doc = "MSDU Value"]
    #[inline(always)]
    pub fn msduv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwpmfgc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwpmfgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MSDU Filter Mode"]
    #[inline(always)]
    pub fn mfm(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        fwpmfgc::Mfm,
        fwpmfgc::Mfm,
        Fwpmfgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            fwpmfgc::Mfm,
            fwpmfgc::Mfm,
            Fwpmfgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwpmfgc {
    #[inline(always)]
    fn default() -> Fwpmfgc {
        <crate::RegValueT<Fwpmfgc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwpmfgc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mfm_SPEC;
    pub type Mfm = crate::EnumBitfieldStruct<u8, Mfm_SPEC>;
    impl Mfm {
        #[doc = "Normal mode: Any frame linked to MSDU filter i thanks to L3 table received with LDESCR.TPL \\[GWCA\\] bigger than FWPMPCi.MSDUV will be filtered."]
        pub const _0: Self = Self::new(0);

        #[doc = "Throttle mode: Any frame linked to MSDU filter i thanks to L3 table received with LDESCR.TPL \\[GWCA\\] bigger than FWPMPCi.MSDUV will be filtered. Any frame linked to MSDU filter i thanks to L3 table received when FWEIS2.PMFS\\[i\\] is set will be filtered."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmtrfc_SPEC;
impl crate::sealed::RegSpec for Fwpmtrfc_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s Filter Configuration Register"]
pub type Fwpmtrfc = crate::RegValueT<Fwpmtrfc_SPEC>;

impl Fwpmtrfc {
    #[doc = "Meter Filter Enable"]
    #[inline(always)]
    pub fn mtrfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwpmtrfc::Mtrfe,
        fwpmtrfc::Mtrfe,
        Fwpmtrfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwpmtrfc::Mtrfe,
            fwpmtrfc::Mtrfe,
            Fwpmtrfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Meter Filter Mode"]
    #[inline(always)]
    pub fn mtrfm(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        fwpmtrfc::Mtrfm,
        fwpmtrfc::Mtrfm,
        Fwpmtrfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            fwpmtrfc::Mtrfm,
            fwpmtrfc::Mtrfm,
            Fwpmtrfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Meter Filter Red Frame Drop"]
    #[inline(always)]
    pub fn mtrfrfd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwpmtrfc::Mtrfrfd,
        fwpmtrfc::Mtrfrfd,
        Fwpmtrfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwpmtrfc::Mtrfrfd,
            fwpmtrfc::Mtrfrfd,
            Fwpmtrfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Meter Coupling Flag"]
    #[inline(always)]
    pub fn mtrcf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fwpmtrfc::Mtrcf,
        fwpmtrfc::Mtrcf,
        Fwpmtrfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fwpmtrfc::Mtrcf,
            fwpmtrfc::Mtrcf,
            Fwpmtrfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Meter Color Mode n (n = 0 to 15)"]
    #[inline(always)]
    pub fn mtrcm15_to_mtrcm0(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Fwpmtrfc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Fwpmtrfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmtrfc {
    #[inline(always)]
    fn default() -> Fwpmtrfc {
        <crate::RegValueT<Fwpmtrfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwpmtrfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtrfe_SPEC;
    pub type Mtrfe = crate::EnumBitfieldStruct<u8, Mtrfe_SPEC>;
    impl Mtrfe {
        #[doc = "Meter i disabled (Pass through all frames.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Meter i enabled (Reject target stream frames by meter filter.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtrfm_SPEC;
    pub type Mtrfm = crate::EnumBitfieldStruct<u8, Mtrfm_SPEC>;
    impl Mtrfm {
        #[doc = "Normal mode: Any frame linked to Meter filter i thanks to L3 table received when not enough token is available will be red."]
        pub const _00: Self = Self::new(0);

        #[doc = "Throttle mode: Any frame linked to Meter filter i thanks to L3 table received when not enough token is available or when FWEIS5.PMRFS\\[i\\] is set will be red."]
        pub const _01: Self = Self::new(1);

        #[doc = "ATS mode: Any frame linked to Meter filter i thanks to L3 table received when not enough token is available will be stored in ATS RAM until enough tokens are available. When enough tokens are available, frame will be forwarded as yellow or green."]
        pub const _10: Self = Self::new(2);

        #[doc = "ATS throttle mode: Any frame linked to Meter filter i thanks to L3 table received when not enough token is available will be stored in ATS RAM until enough tokens are available. When enough tokens are available, frame will be forwarded as yellow or green if FWEIS5.PMRFS\\[i\\] is not set and will be red if FWEIS5.PMRFS\\[i\\] is set."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtrfrfd_SPEC;
    pub type Mtrfrfd = crate::EnumBitfieldStruct<u8, Mtrfrfd_SPEC>;
    impl Mtrfrfd {
        #[doc = "Red frames are not dropped by meter i."]
        pub const _0: Self = Self::new(0);

        #[doc = "Red frames are dropped by meter i."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtrcf_SPEC;
    pub type Mtrcf = crate::EnumBitfieldStruct<u8, Mtrcf_SPEC>;
    impl Mtrcf {
        #[doc = "When green bucket is full, CIR tokens are lost."]
        pub const _0: Self = Self::new(0);

        #[doc = "When green bucket is full, CIR tokens are added to the yellow bucket."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmtrcbsc_SPEC;
impl crate::sealed::RegSpec for Fwpmtrcbsc_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s CBS Configuration Register"]
pub type Fwpmtrcbsc = crate::RegValueT<Fwpmtrcbsc_SPEC>;

impl Fwpmtrcbsc {
    #[doc = "CBS"]
    #[inline(always)]
    pub fn cbs(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, u32, Fwpmtrcbsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32,u32,Fwpmtrcbsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmtrcbsc {
    #[inline(always)]
    fn default() -> Fwpmtrcbsc {
        <crate::RegValueT<Fwpmtrcbsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmtrcirc_SPEC;
impl crate::sealed::RegSpec for Fwpmtrcirc_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s CIR Configuration Register"]
pub type Fwpmtrcirc = crate::RegValueT<Fwpmtrcirc_SPEC>;

impl Fwpmtrcirc {
    #[doc = "CIR"]
    #[inline(always)]
    pub fn cir(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Fwpmtrcirc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Fwpmtrcirc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmtrcirc {
    #[inline(always)]
    fn default() -> Fwpmtrcirc {
        <crate::RegValueT<Fwpmtrcirc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmtrebsc_SPEC;
impl crate::sealed::RegSpec for Fwpmtrebsc_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s EBS Configuration Register"]
pub type Fwpmtrebsc = crate::RegValueT<Fwpmtrebsc_SPEC>;

impl Fwpmtrebsc {
    #[doc = "EBS"]
    #[inline(always)]
    pub fn ebs(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, u32, Fwpmtrebsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32,u32,Fwpmtrebsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmtrebsc {
    #[inline(always)]
    fn default() -> Fwpmtrebsc {
        <crate::RegValueT<Fwpmtrebsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmtreirc_SPEC;
impl crate::sealed::RegSpec for Fwpmtreirc_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s EIR Configuration Register"]
pub type Fwpmtreirc = crate::RegValueT<Fwpmtreirc_SPEC>;

impl Fwpmtreirc {
    #[doc = "EIR"]
    #[inline(always)]
    pub fn eir(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Fwpmtreirc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Fwpmtreirc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmtreirc {
    #[inline(always)]
    fn default() -> Fwpmtreirc {
        <crate::RegValueT<Fwpmtreirc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmtrfm_SPEC;
impl crate::sealed::RegSpec for Fwpmtrfm_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s Filter Monitoring Register"]
pub type Fwpmtrfm = crate::RegValueT<Fwpmtrfm_SPEC>;

impl Fwpmtrfm {
    #[doc = "Meter ATS RAM Descriptor Number"]
    #[inline(always)]
    pub fn mtrardn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Fwpmtrfm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Fwpmtrfm_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Meter ATS RAM Descriptor Number Maximum Number"]
    #[inline(always)]
    pub fn mtrardnmn(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Fwpmtrfm_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Fwpmtrfm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmtrfm {
    #[inline(always)]
    fn default() -> Fwpmtrfm {
        <crate::RegValueT<Fwpmtrfm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftl0_SPEC;
impl crate::sealed::RegSpec for Fwftl0_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Learn Register 0"]
pub type Fwftl0 = crate::RegValueT<Fwftl0_SPEC>;

impl Fwftl0 {
    #[doc = "FRER Entry Address Learn"]
    #[inline(always)]
    pub fn feal(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwftl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwftl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FRER Sequence Recovery Pointer Learn"]
    #[inline(always)]
    pub fn fsrpl(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Fwftl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Fwftl0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwftl0 {
    #[inline(always)]
    fn default() -> Fwftl0 {
        <crate::RegValueT<Fwftl0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftl1_SPEC;
impl crate::sealed::RegSpec for Fwftl1_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Learn Register 1"]
pub type Fwftl1 = crate::RegValueT<Fwftl1_SPEC>;

impl Fwftl1 {
    #[doc = "FRER Sequence History Length Learn"]
    #[inline(always)]
    pub fn fshll(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Fwftl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Fwftl1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FRER Take No Sequence Learn"]
    #[inline(always)]
    pub fn ftnsl(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwftl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwftl1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FRER Sequence Recovery Pointer Valid Learn"]
    #[inline(always)]
    pub fn fsrpvl(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Fwftl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Fwftl1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FRER Sequence Recovery Remaining Ticks Learn"]
    #[inline(always)]
    pub fn fsrrtl(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Fwftl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Fwftl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwftl1 {
    #[inline(always)]
    fn default() -> Fwftl1 {
        <crate::RegValueT<Fwftl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftlr_SPEC;
impl crate::sealed::RegSpec for Fwftlr_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Learn Result Register"]
pub type Fwftlr = crate::RegValueT<Fwftlr_SPEC>;

impl Fwftlr {
    #[doc = "FRER Learn Fail"]
    #[inline(always)]
    pub fn flf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwftlr::Flf,
        fwftlr::Flf,
        Fwftlr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwftlr::Flf,
            fwftlr::Flf,
            Fwftlr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FRER Table Learn"]
    #[inline(always)]
    pub fn ftl(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fwftlr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fwftlr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwftlr {
    #[inline(always)]
    fn default() -> Fwftlr {
        <crate::RegValueT<Fwftlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwftlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flf_SPEC;
    pub type Flf = crate::EnumBitfieldStruct<u8, Flf_SPEC>;
    impl Flf {
        #[doc = "Entry learning did not fail because the FRER RAM is ready."]
        pub const _0: Self = Self::new(0);

        #[doc = "Entry learning failed because the FRER RAM is not ready."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftoc_SPEC;
impl crate::sealed::RegSpec for Fwftoc_SPEC {
    type DataType = u32;
}

#[doc = "FRER Timeout Configuration Register"]
pub type Fwftoc = crate::RegValueT<Fwftoc_SPEC>;

impl Fwftoc {
    #[doc = "Timeout Time (ms)"]
    #[inline(always)]
    pub fn tot(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwftoc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwftoc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timeout Check Enable"]
    #[inline(always)]
    pub fn toce(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwftoc::Toce,
        fwftoc::Toce,
        Fwftoc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwftoc::Toce,
            fwftoc::Toce,
            Fwftoc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout Ongoing"]
    #[inline(always)]
    pub fn toog(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwftoc::Toog,
        fwftoc::Toog,
        Fwftoc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwftoc::Toog,
            fwftoc::Toog,
            Fwftoc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwftoc {
    #[inline(always)]
    fn default() -> Fwftoc {
        <crate::RegValueT<Fwftoc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwftoc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toce_SPEC;
    pub type Toce = crate::EnumBitfieldStruct<u8, Toce_SPEC>;
    impl Toce {
        #[doc = "Timeout check is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Timeout check is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toog_SPEC;
    pub type Toog = crate::EnumBitfieldStruct<u8, Toog_SPEC>;
    impl Toog {
        #[doc = "Timeout check is not ongoing."]
        pub const _0: Self = Self::new(0);

        #[doc = "Timeout check is ongoing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftopc_SPEC;
impl crate::sealed::RegSpec for Fwftopc_SPEC {
    type DataType = u32;
}

#[doc = "FRER Timeout Prescaler Configuration Register 0"]
pub type Fwftopc = crate::RegValueT<Fwftopc_SPEC>;

impl Fwftopc {
    #[doc = "Microsecond Prescaler"]
    #[inline(always)]
    pub fn usp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Fwftopc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Fwftopc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwftopc {
    #[inline(always)]
    fn default() -> Fwftopc {
        <crate::RegValueT<Fwftopc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftim_SPEC;
impl crate::sealed::RegSpec for Fwftim_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Initialization Monitoring Register"]
pub type Fwftim = crate::RegValueT<Fwftim_SPEC>;

impl Fwftim {
    #[doc = "FRER Table Initialization Ongoing"]
    #[inline(always)]
    pub fn ftiog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwftim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fwftim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FRER Table Ready"]
    #[inline(always)]
    pub fn ftr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Fwftim_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fwftim_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwftim {
    #[inline(always)]
    fn default() -> Fwftim {
        <crate::RegValueT<Fwftim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftr_SPEC;
impl crate::sealed::RegSpec for Fwftr_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Read Register"]
pub type Fwftr = crate::RegValueT<Fwftr_SPEC>;

impl Fwftr {
    #[doc = "FRER Entry Address Read"]
    #[inline(always)]
    pub fn fear(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Fwftr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Fwftr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwftr {
    #[inline(always)]
    fn default() -> Fwftr {
        <crate::RegValueT<Fwftr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftrr0_SPEC;
impl crate::sealed::RegSpec for Fwftrr0_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Read Result Register 0"]
pub type Fwftrr0 = crate::RegValueT<Fwftrr0_SPEC>;

impl Fwftrr0 {
    #[doc = "FRER Sequence History Length Read"]
    #[inline(always)]
    pub fn fshlr(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Fwftrr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Fwftrr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "FRER Take No Sequence Read"]
    #[inline(always)]
    pub fn ftnsr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fwftrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fwftrr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FRER Sequence Recovery Pointer Valid Read"]
    #[inline(always)]
    pub fn fsrpvr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Fwftrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Fwftrr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FRER Set Recovery Remaining Ticks Read"]
    #[inline(always)]
    pub fn fsrrtr(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Fwftrr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Fwftrr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "FRER Table Read"]
    #[inline(always)]
    pub fn ftr(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fwftrr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fwftrr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwftrr0 {
    #[inline(always)]
    fn default() -> Fwftrr0 {
        <crate::RegValueT<Fwftrr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftrr1_SPEC;
impl crate::sealed::RegSpec for Fwftrr1_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Read Result Register 1"]
pub type Fwftrr1 = crate::RegValueT<Fwftrr1_SPEC>;

impl Fwftrr1 {
    #[doc = "FRER Sequence History Read"]
    #[inline(always)]
    pub fn fshr(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Fwftrr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Fwftrr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "FRER Sequence Recovery Pointer Read"]
    #[inline(always)]
    pub fn fsrpr(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Fwftrr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Fwftrr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwftrr1 {
    #[inline(always)]
    fn default() -> Fwftrr1 {
        <crate::RegValueT<Fwftrr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwftrr2_SPEC;
impl crate::sealed::RegSpec for Fwftrr2_SPEC {
    type DataType = u32;
}

#[doc = "FRER Table Read Result Register 2"]
pub type Fwftrr2 = crate::RegValueT<Fwftrr2_SPEC>;

impl Fwftrr2 {
    #[doc = "FRER Recovery Sequence Number Read"]
    #[inline(always)]
    pub fn frsnr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwftrr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwftrr2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "FRER Recovery Remaining Ticks Read"]
    #[inline(always)]
    pub fn frrtr(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Fwftrr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Fwftrr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwftrr2 {
    #[inline(always)]
    fn default() -> Fwftrr2 {
        <crate::RegValueT<Fwftrr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwseqngc_SPEC;
impl crate::sealed::RegSpec for Fwseqngc_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Number Generation %s Configuration Register"]
pub type Fwseqngc = crate::RegValueT<Fwseqngc_SPEC>;

impl Fwseqngc {
    #[doc = "SEQuence Number Generation Routing Number"]
    #[inline(always)]
    pub fn seqngrn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fwseqngc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fwseqngc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SEQuence Number Generation Enable"]
    #[inline(always)]
    pub fn seqnge(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fwseqngc::Seqnge,
        fwseqngc::Seqnge,
        Fwseqngc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fwseqngc::Seqnge,
            fwseqngc::Seqnge,
            Fwseqngc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwseqngc {
    #[inline(always)]
    fn default() -> Fwseqngc {
        <crate::RegValueT<Fwseqngc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwseqngc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seqnge_SPEC;
    pub type Seqnge = crate::EnumBitfieldStruct<u8, Seqnge_SPEC>;
    impl Seqnge {
        #[doc = "Sequence number generation disabled for routing number FWSEQNGCi.SEQNGRN"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sequence number generation enabled for routing number FWSEQNGCi.SEQNGRN"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwseqngm_SPEC;
impl crate::sealed::RegSpec for Fwseqngm_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Number Generation %s Monitoring Register"]
pub type Fwseqngm = crate::RegValueT<Fwseqngm_SPEC>;

impl Fwseqngm {
    #[doc = "SEQuence Number"]
    #[inline(always)]
    pub fn seqn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwseqngm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwseqngm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwseqngm {
    #[inline(always)]
    fn default() -> Fwseqngm {
        <crate::RegValueT<Fwseqngm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwseqnrc_SPEC;
impl crate::sealed::RegSpec for Fwseqnrc_SPEC {
    type DataType = u32;
}

#[doc = "Sequence Number Reset Configuration Register"]
pub type Fwseqnrc = crate::RegValueT<Fwseqnrc_SPEC>;

impl Fwseqnrc {
    #[doc = "Sequence Number Generation Reset n (n = 0 to 31)"]
    #[inline(always)]
    pub fn seqnr31_to_seqnr0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwseqnrc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwseqnrc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwseqnrc {
    #[inline(always)]
    fn default() -> Fwseqnrc {
        <crate::RegValueT<Fwseqnrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctfdcn_SPEC;
impl crate::sealed::RegSpec for Fwctfdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Cut-Through Forwarded Descriptor Counter Register"]
pub type Fwctfdcn = crate::RegValueT<Fwctfdcn_SPEC>;

impl Fwctfdcn {
    #[doc = "Cut-Through Forwarded Descriptor Number"]
    #[inline(always)]
    pub fn ctfdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwctfdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwctfdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwctfdcn {
    #[inline(always)]
    fn default() -> Fwctfdcn {
        <crate::RegValueT<Fwctfdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthfdcn_SPEC;
impl crate::sealed::RegSpec for Fwlthfdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Layer 3 Forwarded Descriptor Counter Register"]
pub type Fwlthfdcn = crate::RegValueT<Fwlthfdcn_SPEC>;

impl Fwlthfdcn {
    #[doc = "Layer 3 Forwarded Descriptor Number"]
    #[inline(always)]
    pub fn lthfdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwlthfdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwlthfdcn_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwlthfdcn {
    #[inline(always)]
    fn default() -> Fwlthfdcn {
        <crate::RegValueT<Fwlthfdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwltwfdcn_SPEC;
impl crate::sealed::RegSpec for Fwltwfdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Layer 2 Forwarded Descriptor Counter Register"]
pub type Fwltwfdcn = crate::RegValueT<Fwltwfdcn_SPEC>;

impl Fwltwfdcn {
    #[doc = "Layer 2 Forwarded Descriptor Number"]
    #[inline(always)]
    pub fn ltwfdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwltwfdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwltwfdcn_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwltwfdcn {
    #[inline(always)]
    fn default() -> Fwltwfdcn {
        <crate::RegValueT<Fwltwfdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpbfdcn_SPEC;
impl crate::sealed::RegSpec for Fwpbfdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Port Based Forwarded Descriptor Counter Register"]
pub type Fwpbfdcn = crate::RegValueT<Fwpbfdcn_SPEC>;

impl Fwpbfdcn {
    #[doc = "Port Based Forwarded Descriptor Number"]
    #[inline(always)]
    pub fn pbfdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwpbfdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwpbfdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpbfdcn {
    #[inline(always)]
    fn default() -> Fwpbfdcn {
        <crate::RegValueT<Fwpbfdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmhlcn_SPEC;
impl crate::sealed::RegSpec for Fwmhlcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s MAC Hardware Learn Counter Register"]
pub type Fwmhlcn = crate::RegValueT<Fwmhlcn_SPEC>;

impl Fwmhlcn {
    #[doc = "MAC Hardware Learn Number"]
    #[inline(always)]
    pub fn mhln(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwmhlcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fwmhlcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwmhlcn {
    #[inline(always)]
    fn default() -> Fwmhlcn {
        <crate::RegValueT<Fwmhlcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwddfdcn2_SPEC;
impl crate::sealed::RegSpec for Fwddfdcn2_SPEC {
    type DataType = u32;
}

#[doc = "Port 2 Direct Descriptor Forwarded Descriptor Counter Register"]
pub type Fwddfdcn2 = crate::RegValueT<Fwddfdcn2_SPEC>;

impl Fwddfdcn2 {
    #[doc = "Direct Descriptor Forwarded Descriptor Number"]
    #[inline(always)]
    pub fn ddfdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fwddfdcn2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fwddfdcn2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwddfdcn2 {
    #[inline(always)]
    fn default() -> Fwddfdcn2 {
        <crate::RegValueT<Fwddfdcn2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwwmrdcn_SPEC;
impl crate::sealed::RegSpec for Fwwmrdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Watermark Rejected Descriptor Counter Register"]
pub type Fwwmrdcn = crate::RegValueT<Fwwmrdcn_SPEC>;

impl Fwwmrdcn {
    #[doc = "Watermark rejected Descriptor Number"]
    #[inline(always)]
    pub fn wmrdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwwmrdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwwmrdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwwmrdcn {
    #[inline(always)]
    fn default() -> Fwwmrdcn {
        <crate::RegValueT<Fwwmrdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwctrdcn_SPEC;
impl crate::sealed::RegSpec for Fwctrdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Cut-Through Rejected Descriptor Counter Register"]
pub type Fwctrdcn = crate::RegValueT<Fwctrdcn_SPEC>;

impl Fwctrdcn {
    #[doc = "Cut-through rejected Descriptor Number"]
    #[inline(always)]
    pub fn ctrdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwctrdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwctrdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwctrdcn {
    #[inline(always)]
    fn default() -> Fwctrdcn {
        <crate::RegValueT<Fwctrdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwlthrdcn_SPEC;
impl crate::sealed::RegSpec for Fwlthrdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Layer 3 Rejected Descriptor Counter Register"]
pub type Fwlthrdcn = crate::RegValueT<Fwlthrdcn_SPEC>;

impl Fwlthrdcn {
    #[doc = "Layer 3 rejected Descriptor Number"]
    #[inline(always)]
    pub fn lthrdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwlthrdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwlthrdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwlthrdcn {
    #[inline(always)]
    fn default() -> Fwlthrdcn {
        <crate::RegValueT<Fwlthrdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwltwrdcn_SPEC;
impl crate::sealed::RegSpec for Fwltwrdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Layer 2 Rejected Descriptor Counter Register"]
pub type Fwltwrdcn = crate::RegValueT<Fwltwrdcn_SPEC>;

impl Fwltwrdcn {
    #[doc = "Layer 2 rejected Descriptor Number"]
    #[inline(always)]
    pub fn ltwrdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwltwrdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwltwrdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwltwrdcn {
    #[inline(always)]
    fn default() -> Fwltwrdcn {
        <crate::RegValueT<Fwltwrdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpbrdcn_SPEC;
impl crate::sealed::RegSpec for Fwpbrdcn_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Port Based Rejected Descriptor Counter Register"]
pub type Fwpbrdcn = crate::RegValueT<Fwpbrdcn_SPEC>;

impl Fwpbrdcn {
    #[doc = "Port Based rejected Descriptor Number"]
    #[inline(always)]
    pub fn pbrdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwpbrdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwpbrdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpbrdcn {
    #[inline(always)]
    fn default() -> Fwpbrdcn {
        <crate::RegValueT<Fwpbrdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwddrdcn2_SPEC;
impl crate::sealed::RegSpec for Fwddrdcn2_SPEC {
    type DataType = u32;
}

#[doc = "Port 2 Direct Descriptor Rejected Descriptor Counter Register"]
pub type Fwddrdcn2 = crate::RegValueT<Fwddrdcn2_SPEC>;

impl Fwddrdcn2 {
    #[doc = "Direct Descriptor rejected Descriptor Number"]
    #[inline(always)]
    pub fn ddrdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwddrdcn2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwddrdcn2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwddrdcn2 {
    #[inline(always)]
    fn default() -> Fwddrdcn2 {
        <crate::RegValueT<Fwddrdcn2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmfdcn_SPEC;
impl crate::sealed::RegSpec for Fwpmfdcn_SPEC {
    type DataType = u32;
}

#[doc = "PSFP MSDU %s Filtered Descriptor Counter Register"]
pub type Fwpmfdcn = crate::RegValueT<Fwpmfdcn_SPEC>;

impl Fwpmfdcn {
    #[doc = "PSFP MSDU Filtered Descriptor Number"]
    #[inline(always)]
    pub fn pmfdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwpmfdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwpmfdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmfdcn {
    #[inline(always)]
    fn default() -> Fwpmfdcn {
        <crate::RegValueT<Fwpmfdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmgdcn_SPEC;
impl crate::sealed::RegSpec for Fwpmgdcn_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s Green Descriptor Counter Register"]
pub type Fwpmgdcn = crate::RegValueT<Fwpmgdcn_SPEC>;

impl Fwpmgdcn {
    #[doc = "PSFP Meter Green Descriptor Number"]
    #[inline(always)]
    pub fn pmgdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwpmgdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwpmgdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmgdcn {
    #[inline(always)]
    fn default() -> Fwpmgdcn {
        <crate::RegValueT<Fwpmgdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmydcn_SPEC;
impl crate::sealed::RegSpec for Fwpmydcn_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s Yellow Descriptor Counter Register"]
pub type Fwpmydcn = crate::RegValueT<Fwpmydcn_SPEC>;

impl Fwpmydcn {
    #[doc = "PSFP Meter Yellow Descriptor Number"]
    #[inline(always)]
    pub fn pmydn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwpmydcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwpmydcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmydcn {
    #[inline(always)]
    fn default() -> Fwpmydcn {
        <crate::RegValueT<Fwpmydcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwpmrdcn_SPEC;
impl crate::sealed::RegSpec for Fwpmrdcn_SPEC {
    type DataType = u32;
}

#[doc = "PSFP Meter %s Red Descriptor Counter Register"]
pub type Fwpmrdcn = crate::RegValueT<Fwpmrdcn_SPEC>;

impl Fwpmrdcn {
    #[doc = "PSFP Meter Red Descriptor Number"]
    #[inline(always)]
    pub fn pmrdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwpmrdcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwpmrdcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwpmrdcn {
    #[inline(always)]
    fn default() -> Fwpmrdcn {
        <crate::RegValueT<Fwpmrdcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwfrppcn_SPEC;
impl crate::sealed::RegSpec for Fwfrppcn_SPEC {
    type DataType = u32;
}

#[doc = "FRER %s Passed Packet Counter Register"]
pub type Fwfrppcn = crate::RegValueT<Fwfrppcn_SPEC>;

impl Fwfrppcn {
    #[doc = "Passed Packet Count"]
    #[inline(always)]
    pub fn ppc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwfrppcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwfrppcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwfrppcn {
    #[inline(always)]
    fn default() -> Fwfrppcn {
        <crate::RegValueT<Fwfrppcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwfrdpcn_SPEC;
impl crate::sealed::RegSpec for Fwfrdpcn_SPEC {
    type DataType = u32;
}

#[doc = "FRER %s Discarded Packet Counter Register"]
pub type Fwfrdpcn = crate::RegValueT<Fwfrdpcn_SPEC>;

impl Fwfrdpcn {
    #[doc = "Discarded Packet Count"]
    #[inline(always)]
    pub fn dpc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwfrdpcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwfrdpcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwfrdpcn {
    #[inline(always)]
    fn default() -> Fwfrdpcn {
        <crate::RegValueT<Fwfrdpcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis0_SPEC;
impl crate::sealed::RegSpec for Fweis0_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Error Interrupt Status Register 0"]
pub type Fweis0 = crate::RegValueT<Fweis0_SPEC>;

impl Fweis0 {
    #[doc = "Layer 3 Source Port Filtering Status Flag"]
    #[inline(always)]
    pub fn lthspfs(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 3 No Target Filtering Status Flag"]
    #[inline(always)]
    pub fn lthntfs(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 3 Unknown Filtering Status Flag"]
    #[inline(always)]
    pub fn lthufs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Destination Source Port Filtering Status Flag"]
    #[inline(always)]
    pub fn ltwdspfs(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Source Source Port Filtering Status Flag"]
    #[inline(always)]
    pub fn ltwsspfs(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 VLAN Source Port Filtering Status Flag"]
    #[inline(always)]
    pub fn ltwvspfs(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 No Target Filtering Status Flag"]
    #[inline(always)]
    pub fn ltwntfs(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Source Unknown Filtering Status Flag"]
    #[inline(always)]
    pub fn ltwsufs(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Destination Unknown Filtering Status Flag"]
    #[inline(always)]
    pub fn ltwdufs(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 VLAN Unknown Filtering Status Flag"]
    #[inline(always)]
    pub fn ltwvufs(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Based No Target Filtering Status Flag"]
    #[inline(always)]
    pub fn pbntfs(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source MAC Hardware Learning Fail Status Flag"]
    #[inline(always)]
    pub fn smhlfs(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source MAC Hardware Migration Fail Status Flag"]
    #[inline(always)]
    pub fn smhmfs(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Critical Filtering Status Flag"]
    #[inline(always)]
    pub fn wmcfs(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Flush Filtering Status Flag"]
    #[inline(always)]
    pub fn wmffs(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark IPV Secure Filtering Status Flag"]
    #[inline(always)]
    pub fn wmisfs(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark IPV Unsecure Filtering Status Flag"]
    #[inline(always)]
    pub fn wmiufs(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Direct Descriptor Error Status Flag"]
    #[inline(always)]
    pub fn ddes(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Direct Descriptor Security Error Status Flag"]
    #[inline(always)]
    pub fn ddses(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Direct Descriptor No Target Filtering Status Flag"]
    #[inline(always)]
    pub fn ddntfs(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Fweis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Fweis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fweis0 {
    #[inline(always)]
    fn default() -> Fweis0 {
        <crate::RegValueT<Fweis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie0_SPEC;
impl crate::sealed::RegSpec for Fweie0_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Error Interrupt Enable Register 0"]
pub type Fweie0 = crate::RegValueT<Fweie0_SPEC>;

impl Fweie0 {
    #[doc = "Layer 3 Source Port Filtering Enable"]
    #[inline(always)]
    pub fn lthspfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fweie0::Lthspfe,
        fweie0::Lthspfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fweie0::Lthspfe,
            fweie0::Lthspfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 No Target Filtering Enable"]
    #[inline(always)]
    pub fn lthntfe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fweie0::Lthntfe,
        fweie0::Lthntfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fweie0::Lthntfe,
            fweie0::Lthntfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 3 Unknown Filtering Enable"]
    #[inline(always)]
    pub fn lthufe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fweie0::Lthufe,
        fweie0::Lthufe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fweie0::Lthufe,
            fweie0::Lthufe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Destination Source Port Filtering Enable"]
    #[inline(always)]
    pub fn ltwdspfe(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fweie0::Ltwdspfe,
        fweie0::Ltwdspfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fweie0::Ltwdspfe,
            fweie0::Ltwdspfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Source Source Port Filtering Enable"]
    #[inline(always)]
    pub fn ltwsspfe(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fweie0::Ltwsspfe,
        fweie0::Ltwsspfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fweie0::Ltwsspfe,
            fweie0::Ltwsspfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 VLAN Source Port Filtering Enable"]
    #[inline(always)]
    pub fn ltwvspfe(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fweie0::Ltwvspfe,
        fweie0::Ltwvspfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fweie0::Ltwvspfe,
            fweie0::Ltwvspfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 No Target Filtering Enable"]
    #[inline(always)]
    pub fn ltwntfe(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        fweie0::Ltwntfe,
        fweie0::Ltwntfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            fweie0::Ltwntfe,
            fweie0::Ltwntfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Source Unknown Filtering Enable"]
    #[inline(always)]
    pub fn ltwsufe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        fweie0::Ltwsufe,
        fweie0::Ltwsufe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            fweie0::Ltwsufe,
            fweie0::Ltwsufe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 Destination Unknown Filtering Enable"]
    #[inline(always)]
    pub fn ltwdufe(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fweie0::Ltwdufe,
        fweie0::Ltwdufe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fweie0::Ltwdufe,
            fweie0::Ltwdufe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Layer 2 VLAN Unknown Filtering Enable"]
    #[inline(always)]
    pub fn ltwvufe(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        fweie0::Ltwvufe,
        fweie0::Ltwvufe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            fweie0::Ltwvufe,
            fweie0::Ltwvufe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Port Based No Target Filtering Enable"]
    #[inline(always)]
    pub fn pbntfe(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fweie0::Pbntfe,
        fweie0::Pbntfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fweie0::Pbntfe,
            fweie0::Pbntfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Source MAC Hardware Learning Fail Enable"]
    #[inline(always)]
    pub fn smhlfe(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        fweie0::Smhlfe,
        fweie0::Smhlfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            fweie0::Smhlfe,
            fweie0::Smhlfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Source MAC Hardware Migration Fail Enable"]
    #[inline(always)]
    pub fn smhmfe(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        fweie0::Smhmfe,
        fweie0::Smhmfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            fweie0::Smhmfe,
            fweie0::Smhmfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Critical Filtering Enable"]
    #[inline(always)]
    pub fn wmcfe(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        fweie0::Wmcfe,
        fweie0::Wmcfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            fweie0::Wmcfe,
            fweie0::Wmcfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark Flush Filtering Enable"]
    #[inline(always)]
    pub fn wmffe(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fweie0::Wmffe,
        fweie0::Wmffe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fweie0::Wmffe,
            fweie0::Wmffe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark IPV Secure Filtering Enable"]
    #[inline(always)]
    pub fn wmisfe(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        fweie0::Wmisfe,
        fweie0::Wmisfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            fweie0::Wmisfe,
            fweie0::Wmisfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Watermark IPV Unsecure Filtering Enable"]
    #[inline(always)]
    pub fn wmiufe(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        fweie0::Wmiufe,
        fweie0::Wmiufe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            fweie0::Wmiufe,
            fweie0::Wmiufe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor Error Enable"]
    #[inline(always)]
    pub fn ddee(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        fweie0::Ddee,
        fweie0::Ddee,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            fweie0::Ddee,
            fweie0::Ddee,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor Format Error Enable"]
    #[inline(always)]
    pub fn ddfee(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        fweie0::Ddfee,
        fweie0::Ddfee,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            fweie0::Ddfee,
            fweie0::Ddfee,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor Security Error Enable"]
    #[inline(always)]
    pub fn ddsee(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        fweie0::Ddsee,
        fweie0::Ddsee,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            fweie0::Ddsee,
            fweie0::Ddsee,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direct Descriptor No Target Filtering Enable"]
    #[inline(always)]
    pub fn ddntfe(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        fweie0::Ddntfe,
        fweie0::Ddntfe,
        Fweie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            fweie0::Ddntfe,
            fweie0::Ddntfe,
            Fweie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fweie0 {
    #[inline(always)]
    fn default() -> Fweie0 {
        <crate::RegValueT<Fweie0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fweie0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthspfe_SPEC;
    pub type Lthspfe = crate::EnumBitfieldStruct<u8, Lthspfe_SPEC>;
    impl Lthspfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthntfe_SPEC;
    pub type Lthntfe = crate::EnumBitfieldStruct<u8, Lthntfe_SPEC>;
    impl Lthntfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthufe_SPEC;
    pub type Lthufe = crate::EnumBitfieldStruct<u8, Lthufe_SPEC>;
    impl Lthufe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwdspfe_SPEC;
    pub type Ltwdspfe = crate::EnumBitfieldStruct<u8, Ltwdspfe_SPEC>;
    impl Ltwdspfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwsspfe_SPEC;
    pub type Ltwsspfe = crate::EnumBitfieldStruct<u8, Ltwsspfe_SPEC>;
    impl Ltwsspfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwvspfe_SPEC;
    pub type Ltwvspfe = crate::EnumBitfieldStruct<u8, Ltwvspfe_SPEC>;
    impl Ltwvspfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwntfe_SPEC;
    pub type Ltwntfe = crate::EnumBitfieldStruct<u8, Ltwntfe_SPEC>;
    impl Ltwntfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwsufe_SPEC;
    pub type Ltwsufe = crate::EnumBitfieldStruct<u8, Ltwsufe_SPEC>;
    impl Ltwsufe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwdufe_SPEC;
    pub type Ltwdufe = crate::EnumBitfieldStruct<u8, Ltwdufe_SPEC>;
    impl Ltwdufe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ltwvufe_SPEC;
    pub type Ltwvufe = crate::EnumBitfieldStruct<u8, Ltwvufe_SPEC>;
    impl Ltwvufe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pbntfe_SPEC;
    pub type Pbntfe = crate::EnumBitfieldStruct<u8, Pbntfe_SPEC>;
    impl Pbntfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smhlfe_SPEC;
    pub type Smhlfe = crate::EnumBitfieldStruct<u8, Smhlfe_SPEC>;
    impl Smhlfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smhmfe_SPEC;
    pub type Smhmfe = crate::EnumBitfieldStruct<u8, Smhmfe_SPEC>;
    impl Smhmfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmcfe_SPEC;
    pub type Wmcfe = crate::EnumBitfieldStruct<u8, Wmcfe_SPEC>;
    impl Wmcfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmffe_SPEC;
    pub type Wmffe = crate::EnumBitfieldStruct<u8, Wmffe_SPEC>;
    impl Wmffe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmisfe_SPEC;
    pub type Wmisfe = crate::EnumBitfieldStruct<u8, Wmisfe_SPEC>;
    impl Wmisfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wmiufe_SPEC;
    pub type Wmiufe = crate::EnumBitfieldStruct<u8, Wmiufe_SPEC>;
    impl Wmiufe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddee_SPEC;
    pub type Ddee = crate::EnumBitfieldStruct<u8, Ddee_SPEC>;
    impl Ddee {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddfee_SPEC;
    pub type Ddfee = crate::EnumBitfieldStruct<u8, Ddfee_SPEC>;
    impl Ddfee {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddsee_SPEC;
    pub type Ddsee = crate::EnumBitfieldStruct<u8, Ddsee_SPEC>;
    impl Ddsee {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddntfe_SPEC;
    pub type Ddntfe = crate::EnumBitfieldStruct<u8, Ddntfe_SPEC>;
    impl Ddntfe {
        #[doc = "Interrupt i disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt i Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid0_SPEC;
impl crate::sealed::RegSpec for Fweid0_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Error Interrupt Disable Register 0"]
pub type Fweid0 = crate::RegValueT<Fweid0_SPEC>;

impl Fweid0 {
    #[doc = "Layer 3 Source Port Filtering Disable"]
    #[inline(always)]
    pub fn lthspfd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 3 No Target Filtering Disable"]
    #[inline(always)]
    pub fn lthntfd(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 3 Unknown Filtering Disable"]
    #[inline(always)]
    pub fn lthufd(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Destination Source Port Filtering Disable"]
    #[inline(always)]
    pub fn ltwdspfd(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Source Source Port Filtering Disable"]
    #[inline(always)]
    pub fn ltwsspfd(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 VLAN Source Port Filtering Disable"]
    #[inline(always)]
    pub fn ltwvspfd(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 No Target Filtering Disable"]
    #[inline(always)]
    pub fn ltwntfd(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Source Unknown Filtering Disable"]
    #[inline(always)]
    pub fn ltwsufd(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 Destination Unknown Filtering Disable"]
    #[inline(always)]
    pub fn ltwdufd(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2 VLAN Unknown Filtering Disable"]
    #[inline(always)]
    pub fn ltwvufd(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Based No Target Filtering Disable"]
    #[inline(always)]
    pub fn pbntfd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source MAC Hardware Learning Fail Disable"]
    #[inline(always)]
    pub fn smhlfd(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source MAC Hardware Migration Fail Disable"]
    #[inline(always)]
    pub fn smhmfd(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Critical Filtering Disable"]
    #[inline(always)]
    pub fn wmcfd(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Flush Filtering Disable"]
    #[inline(always)]
    pub fn wmffd(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark IPV Secure Filtering Disable"]
    #[inline(always)]
    pub fn wmisfd(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark IPV Unsecure Filtering Disable"]
    #[inline(always)]
    pub fn wmiufd(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Direct Descriptor Error Disable"]
    #[inline(always)]
    pub fn dded(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Direct Descriptor Format Error Disable"]
    #[inline(always)]
    pub fn ddfed(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Direct Descriptor Security Error Disable"]
    #[inline(always)]
    pub fn ddsed(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Direct Descriptor No Target Filtering Disable"]
    #[inline(always)]
    pub fn ddntfd(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Fweid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Fweid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fweid0 {
    #[inline(always)]
    fn default() -> Fweid0 {
        <crate::RegValueT<Fweid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis1_SPEC;
impl crate::sealed::RegSpec for Fweis1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 1"]
pub type Fweis1 = crate::RegValueT<Fweis1_SPEC>;

impl Fweis1 {
    #[doc = "L3 Table Security Error Status Flag"]
    #[inline(always)]
    pub fn lthtses(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fweis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fweis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Table Security Error Status Flag"]
    #[inline(always)]
    pub fn mactses(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Fweis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Fweis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VLAN Table Security Error Status Flag"]
    #[inline(always)]
    pub fn vlantses(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Fweis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fweis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fweis1 {
    #[inline(always)]
    fn default() -> Fweis1 {
        <crate::RegValueT<Fweis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie1_SPEC;
impl crate::sealed::RegSpec for Fweie1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 1"]
pub type Fweie1 = crate::RegValueT<Fweie1_SPEC>;

impl Fweie1 {
    #[doc = "L3 Table Security Error Enable"]
    #[inline(always)]
    pub fn lthtsee(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fweie1::Lthtsee,
        fweie1::Lthtsee,
        Fweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fweie1::Lthtsee,
            fweie1::Lthtsee,
            Fweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Table Security Error Enable"]
    #[inline(always)]
    pub fn mactsee(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fweie1::Mactsee,
        fweie1::Mactsee,
        Fweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fweie1::Mactsee,
            fweie1::Mactsee,
            Fweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Table Security Error Enable"]
    #[inline(always)]
    pub fn vlantsee(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fweie1::Vlantsee,
        fweie1::Vlantsee,
        Fweie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fweie1::Vlantsee,
            fweie1::Vlantsee,
            Fweie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fweie1 {
    #[inline(always)]
    fn default() -> Fweie1 {
        <crate::RegValueT<Fweie1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fweie1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthtsee_SPEC;
    pub type Lthtsee = crate::EnumBitfieldStruct<u8, Lthtsee_SPEC>;
    impl Lthtsee {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mactsee_SPEC;
    pub type Mactsee = crate::EnumBitfieldStruct<u8, Mactsee_SPEC>;
    impl Mactsee {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlantsee_SPEC;
    pub type Vlantsee = crate::EnumBitfieldStruct<u8, Vlantsee_SPEC>;
    impl Vlantsee {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid1_SPEC;
impl crate::sealed::RegSpec for Fweid1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 1"]
pub type Fweid1 = crate::RegValueT<Fweid1_SPEC>;

impl Fweid1 {
    #[doc = "L3 Table Security Error Disable"]
    #[inline(always)]
    pub fn lthtsed(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fweid1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fweid1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Table Security Error Disable"]
    #[inline(always)]
    pub fn mactsed(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Fweid1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Fweid1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VLAN Table Security Error Disable"]
    #[inline(always)]
    pub fn vlantsed(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Fweid1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fweid1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Layer 2/Layer 3 Update Error Disable"]
    #[inline(always)]
    pub fn l23ueed(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fweid1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fweid1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fweid1 {
    #[inline(always)]
    fn default() -> Fweid1 {
        <crate::RegValueT<Fweid1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis2_SPEC;
impl crate::sealed::RegSpec for Fweis2_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 2"]
pub type Fweis2 = crate::RegValueT<Fweis2_SPEC>;

impl Fweis2 {
    #[doc = "PSFP MSDU n Filtering Status Flag (n = 0 to 15)"]
    #[inline(always)]
    pub fn pmfs15_to_pmfs0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fweis2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fweis2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis2 {
    #[inline(always)]
    fn default() -> Fweis2 {
        <crate::RegValueT<Fweis2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie2_SPEC;
impl crate::sealed::RegSpec for Fweie2_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 2"]
pub type Fweie2 = crate::RegValueT<Fweie2_SPEC>;

impl Fweie2 {
    #[doc = "PSFP MSDU n Filtering Enable (n = 0 to 15)"]
    #[inline(always)]
    pub fn pmfe15_to_pmfe0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        fweie2::Pmfe15ToPmfe0,
        fweie2::Pmfe15ToPmfe0,
        Fweie2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            fweie2::Pmfe15ToPmfe0,
            fweie2::Pmfe15ToPmfe0,
            Fweie2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fweie2 {
    #[inline(always)]
    fn default() -> Fweie2 {
        <crate::RegValueT<Fweie2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fweie2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmfe15ToPmfe0_SPEC;
    pub type Pmfe15ToPmfe0 = crate::EnumBitfieldStruct<u8, Pmfe15ToPmfe0_SPEC>;
    impl Pmfe15ToPmfe0 {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid2_SPEC;
impl crate::sealed::RegSpec for Fweid2_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 2"]
pub type Fweid2 = crate::RegValueT<Fweid2_SPEC>;

impl Fweid2 {
    #[doc = "PSFP MSDU n Filtering Disable (n = 0 to 15)"]
    #[inline(always)]
    pub fn pmfd15_to_pmfd0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fweid2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fweid2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid2 {
    #[inline(always)]
    fn default() -> Fweid2 {
        <crate::RegValueT<Fweid2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis5_SPEC;
impl crate::sealed::RegSpec for Fweis5_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 5"]
pub type Fweis5 = crate::RegValueT<Fweis5_SPEC>;

impl Fweis5 {
    #[doc = "PSFP Meter n Filtering Status Flag (n = 0 to 31)"]
    #[inline(always)]
    pub fn pmrfs31_to_pmrfs0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis5 {
    #[inline(always)]
    fn default() -> Fweis5 {
        <crate::RegValueT<Fweis5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie5_SPEC;
impl crate::sealed::RegSpec for Fweie5_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 5"]
pub type Fweie5 = crate::RegValueT<Fweie5_SPEC>;

impl Fweie5 {
    #[doc = "PSFP Meter n Filtering Enable (n = 0 to 31)"]
    #[inline(always)]
    pub fn pmrfe31_to_pmrfe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie5 {
    #[inline(always)]
    fn default() -> Fweie5 {
        <crate::RegValueT<Fweie5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid5_SPEC;
impl crate::sealed::RegSpec for Fweid5_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 5"]
pub type Fweid5 = crate::RegValueT<Fweid5_SPEC>;

impl Fweid5 {
    #[doc = "PSFP Meter n Filtering Disable (n = 0 to 31)"]
    #[inline(always)]
    pub fn pmrfd31_to_pmrfd0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid5 {
    #[inline(always)]
    fn default() -> Fweid5 {
        <crate::RegValueT<Fweid5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis60_SPEC;
impl crate::sealed::RegSpec for Fweis60_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 60"]
pub type Fweis60 = crate::RegValueT<Fweis60_SPEC>;

impl Fweis60 {
    #[doc = "FRER n Filtering Status Flag (n = 96 to 127)"]
    #[inline(always)]
    pub fn ffs31_to_ffs0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis60_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis60 {
    #[inline(always)]
    fn default() -> Fweis60 {
        <crate::RegValueT<Fweis60_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie60_SPEC;
impl crate::sealed::RegSpec for Fweie60_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 60"]
pub type Fweie60 = crate::RegValueT<Fweie60_SPEC>;

impl Fweie60 {
    #[doc = "FRER n Filtering Enable (n = 96 to 127)"]
    #[inline(always)]
    pub fn ffe31_to_ffe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie60_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie60 {
    #[inline(always)]
    fn default() -> Fweie60 {
        <crate::RegValueT<Fweie60_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid60_SPEC;
impl crate::sealed::RegSpec for Fweid60_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 60"]
pub type Fweid60 = crate::RegValueT<Fweid60_SPEC>;

impl Fweid60 {
    #[doc = "FRER n Filtering Disable (n = 96 to 127)"]
    #[inline(always)]
    pub fn ffd31_to_ffd0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid60_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid60 {
    #[inline(always)]
    fn default() -> Fweid60 {
        <crate::RegValueT<Fweid60_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis61_SPEC;
impl crate::sealed::RegSpec for Fweis61_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 61"]
pub type Fweis61 = crate::RegValueT<Fweis61_SPEC>;

impl Fweis61 {
    #[doc = "FRER n Filtering Status Flag (n = 64 to 95)"]
    #[inline(always)]
    pub fn ffs31_to_ffs0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis61_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis61 {
    #[inline(always)]
    fn default() -> Fweis61 {
        <crate::RegValueT<Fweis61_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie61_SPEC;
impl crate::sealed::RegSpec for Fweie61_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 61"]
pub type Fweie61 = crate::RegValueT<Fweie61_SPEC>;

impl Fweie61 {
    #[doc = "FRER n Filtering Enable (n = 64 to 95)"]
    #[inline(always)]
    pub fn ffe31_to_ffe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie61_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie61 {
    #[inline(always)]
    fn default() -> Fweie61 {
        <crate::RegValueT<Fweie61_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid61_SPEC;
impl crate::sealed::RegSpec for Fweid61_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 61"]
pub type Fweid61 = crate::RegValueT<Fweid61_SPEC>;

impl Fweid61 {
    #[doc = "FRER n Filtering Disable (n = 64 to 95)"]
    #[inline(always)]
    pub fn ffd31_to_ffd0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid61_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid61 {
    #[inline(always)]
    fn default() -> Fweid61 {
        <crate::RegValueT<Fweid61_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis62_SPEC;
impl crate::sealed::RegSpec for Fweis62_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 62"]
pub type Fweis62 = crate::RegValueT<Fweis62_SPEC>;

impl Fweis62 {
    #[doc = "FRER n Filtering Status Flag (n = 32 to 63)"]
    #[inline(always)]
    pub fn ffs31_to_ffs0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis62_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis62_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis62 {
    #[inline(always)]
    fn default() -> Fweis62 {
        <crate::RegValueT<Fweis62_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie62_SPEC;
impl crate::sealed::RegSpec for Fweie62_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 62"]
pub type Fweie62 = crate::RegValueT<Fweie62_SPEC>;

impl Fweie62 {
    #[doc = "FRER n Filtering Enable (n = 32 to 63)"]
    #[inline(always)]
    pub fn ffe31_to_ffe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie62_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie62_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie62 {
    #[inline(always)]
    fn default() -> Fweie62 {
        <crate::RegValueT<Fweie62_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid62_SPEC;
impl crate::sealed::RegSpec for Fweid62_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 62"]
pub type Fweid62 = crate::RegValueT<Fweid62_SPEC>;

impl Fweid62 {
    #[doc = "FRER n Filtering Disable (n = 32 to 63)"]
    #[inline(always)]
    pub fn ffd31_to_ffd0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid62_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid62_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid62 {
    #[inline(always)]
    fn default() -> Fweid62 {
        <crate::RegValueT<Fweid62_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis63_SPEC;
impl crate::sealed::RegSpec for Fweis63_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 63"]
pub type Fweis63 = crate::RegValueT<Fweis63_SPEC>;

impl Fweis63 {
    #[doc = "FRER n Filtering Status Flag (n = 0 to 31)"]
    #[inline(always)]
    pub fn ffs31_to_ffs0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis63_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis63_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis63 {
    #[inline(always)]
    fn default() -> Fweis63 {
        <crate::RegValueT<Fweis63_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie63_SPEC;
impl crate::sealed::RegSpec for Fweie63_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 63"]
pub type Fweie63 = crate::RegValueT<Fweie63_SPEC>;

impl Fweie63 {
    #[doc = "FRER n Filtering Enable (n = 0 to 31)"]
    #[inline(always)]
    pub fn ffe31_to_ffe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie63_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie63_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie63 {
    #[inline(always)]
    fn default() -> Fweie63 {
        <crate::RegValueT<Fweie63_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid63_SPEC;
impl crate::sealed::RegSpec for Fweid63_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 63"]
pub type Fweid63 = crate::RegValueT<Fweid63_SPEC>;

impl Fweid63 {
    #[doc = "FRER n Filtering Disable (n = 0 to 31)"]
    #[inline(always)]
    pub fn ffd31_to_ffd0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid63_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid63_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid63 {
    #[inline(always)]
    fn default() -> Fweid63 {
        <crate::RegValueT<Fweid63_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis70_SPEC;
impl crate::sealed::RegSpec for Fweis70_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 70"]
pub type Fweis70 = crate::RegValueT<Fweis70_SPEC>;

impl Fweis70 {
    #[doc = "FRER n Out Of Range Status Flag (n = 96 to 127)"]
    #[inline(always)]
    pub fn foors31_to_foors0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis70_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis70 {
    #[inline(always)]
    fn default() -> Fweis70 {
        <crate::RegValueT<Fweis70_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie70_SPEC;
impl crate::sealed::RegSpec for Fweie70_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 70"]
pub type Fweie70 = crate::RegValueT<Fweie70_SPEC>;

impl Fweie70 {
    #[doc = "FRER n Out Of Range Enable (n = 96 to 127)"]
    #[inline(always)]
    pub fn foore31_to_foore0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie70_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie70 {
    #[inline(always)]
    fn default() -> Fweie70 {
        <crate::RegValueT<Fweie70_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid70_SPEC;
impl crate::sealed::RegSpec for Fweid70_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 70"]
pub type Fweid70 = crate::RegValueT<Fweid70_SPEC>;

impl Fweid70 {
    #[doc = "FRER n Out Of Range Disable (n = 96 to 127)"]
    #[inline(always)]
    pub fn foord31_to_foord0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid70_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid70 {
    #[inline(always)]
    fn default() -> Fweid70 {
        <crate::RegValueT<Fweid70_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis71_SPEC;
impl crate::sealed::RegSpec for Fweis71_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 71"]
pub type Fweis71 = crate::RegValueT<Fweis71_SPEC>;

impl Fweis71 {
    #[doc = "FRER n Out Of Range Status Flag (n = 64 to 95)"]
    #[inline(always)]
    pub fn foors31_to_foors0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis71_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis71 {
    #[inline(always)]
    fn default() -> Fweis71 {
        <crate::RegValueT<Fweis71_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie71_SPEC;
impl crate::sealed::RegSpec for Fweie71_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 71"]
pub type Fweie71 = crate::RegValueT<Fweie71_SPEC>;

impl Fweie71 {
    #[doc = "FRER n Out Of Range Enable (n = 64 to 95)"]
    #[inline(always)]
    pub fn foore31_to_foore0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie71_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie71 {
    #[inline(always)]
    fn default() -> Fweie71 {
        <crate::RegValueT<Fweie71_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid71_SPEC;
impl crate::sealed::RegSpec for Fweid71_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 71"]
pub type Fweid71 = crate::RegValueT<Fweid71_SPEC>;

impl Fweid71 {
    #[doc = "FRER n Out Of Range Disable (n = 64 to 95)"]
    #[inline(always)]
    pub fn foord31_to_foord0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid71_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid71 {
    #[inline(always)]
    fn default() -> Fweid71 {
        <crate::RegValueT<Fweid71_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis72_SPEC;
impl crate::sealed::RegSpec for Fweis72_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 72"]
pub type Fweis72 = crate::RegValueT<Fweis72_SPEC>;

impl Fweis72 {
    #[doc = "FRER n Out Of Range Status Flag (n = 32 to 63)"]
    #[inline(always)]
    pub fn foors31_to_foors0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis72_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis72_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis72 {
    #[inline(always)]
    fn default() -> Fweis72 {
        <crate::RegValueT<Fweis72_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie72_SPEC;
impl crate::sealed::RegSpec for Fweie72_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 72"]
pub type Fweie72 = crate::RegValueT<Fweie72_SPEC>;

impl Fweie72 {
    #[doc = "FRER n Out Of Range Enable (n = 32 to 63)"]
    #[inline(always)]
    pub fn foore31_to_foore0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie72_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie72_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie72 {
    #[inline(always)]
    fn default() -> Fweie72 {
        <crate::RegValueT<Fweie72_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid72_SPEC;
impl crate::sealed::RegSpec for Fweid72_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 72"]
pub type Fweid72 = crate::RegValueT<Fweid72_SPEC>;

impl Fweid72 {
    #[doc = "FRER n Out Of Range Disable (n = 32 to 63)"]
    #[inline(always)]
    pub fn foord31_to_foord0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid72_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid72_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid72 {
    #[inline(always)]
    fn default() -> Fweid72 {
        <crate::RegValueT<Fweid72_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis73_SPEC;
impl crate::sealed::RegSpec for Fweis73_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 73"]
pub type Fweis73 = crate::RegValueT<Fweis73_SPEC>;

impl Fweis73 {
    #[doc = "FRER n Out Of Range Status Flag (n = 0 to 31)"]
    #[inline(always)]
    pub fn foors31_to_foors0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis73_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis73_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis73 {
    #[inline(always)]
    fn default() -> Fweis73 {
        <crate::RegValueT<Fweis73_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie73_SPEC;
impl crate::sealed::RegSpec for Fweie73_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 73"]
pub type Fweie73 = crate::RegValueT<Fweie73_SPEC>;

impl Fweie73 {
    #[doc = "FRER n Out Of Range Enable (n = 0 to 31)"]
    #[inline(always)]
    pub fn foore31_to_foore0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie73_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie73_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie73 {
    #[inline(always)]
    fn default() -> Fweie73 {
        <crate::RegValueT<Fweie73_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid73_SPEC;
impl crate::sealed::RegSpec for Fweid73_SPEC {
    type DataType = u32;
}

#[doc = "Forwarding Engine Error Interrupt Disable 73"]
pub type Fweid73 = crate::RegValueT<Fweid73_SPEC>;

impl Fweid73 {
    #[doc = "FRER n Out Of Range Disable (n = 0 to 31)"]
    #[inline(always)]
    pub fn foord31_to_foord0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid73_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid73_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid73 {
    #[inline(always)]
    fn default() -> Fweid73 {
        <crate::RegValueT<Fweid73_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis80_SPEC;
impl crate::sealed::RegSpec for Fweis80_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 80"]
pub type Fweis80 = crate::RegValueT<Fweis80_SPEC>;

impl Fweis80 {
    #[doc = "Timeout Status Flag n (n = 96 to 127)"]
    #[inline(always)]
    pub fn tos31_to_tos0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis80_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis80 {
    #[inline(always)]
    fn default() -> Fweis80 {
        <crate::RegValueT<Fweis80_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie80_SPEC;
impl crate::sealed::RegSpec for Fweie80_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 80"]
pub type Fweie80 = crate::RegValueT<Fweie80_SPEC>;

impl Fweie80 {
    #[doc = "Timeout Enable n (n = 96 to 127)"]
    #[inline(always)]
    pub fn toe31_to_toe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie80_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie80 {
    #[inline(always)]
    fn default() -> Fweie80 {
        <crate::RegValueT<Fweie80_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid80_SPEC;
impl crate::sealed::RegSpec for Fweid80_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 80"]
pub type Fweid80 = crate::RegValueT<Fweid80_SPEC>;

impl Fweid80 {
    #[doc = "Timeout Disable n (n = 96 to 127)"]
    #[inline(always)]
    pub fn tod31_to_tod0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid80_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid80 {
    #[inline(always)]
    fn default() -> Fweid80 {
        <crate::RegValueT<Fweid80_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis81_SPEC;
impl crate::sealed::RegSpec for Fweis81_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 81"]
pub type Fweis81 = crate::RegValueT<Fweis81_SPEC>;

impl Fweis81 {
    #[doc = "Timeout Status Flag n (n = 64 to 95)"]
    #[inline(always)]
    pub fn tos31_to_tos0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis81_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis81 {
    #[inline(always)]
    fn default() -> Fweis81 {
        <crate::RegValueT<Fweis81_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie81_SPEC;
impl crate::sealed::RegSpec for Fweie81_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 81"]
pub type Fweie81 = crate::RegValueT<Fweie81_SPEC>;

impl Fweie81 {
    #[doc = "Timeout Enable n (n = 64 to 95)"]
    #[inline(always)]
    pub fn toe31_to_toe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie81_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie81 {
    #[inline(always)]
    fn default() -> Fweie81 {
        <crate::RegValueT<Fweie81_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid81_SPEC;
impl crate::sealed::RegSpec for Fweid81_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 81"]
pub type Fweid81 = crate::RegValueT<Fweid81_SPEC>;

impl Fweid81 {
    #[doc = "Timeout Disable n (n = 64 to 95)"]
    #[inline(always)]
    pub fn tod31_to_tod0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid81_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid81 {
    #[inline(always)]
    fn default() -> Fweid81 {
        <crate::RegValueT<Fweid81_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis82_SPEC;
impl crate::sealed::RegSpec for Fweis82_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 82"]
pub type Fweis82 = crate::RegValueT<Fweis82_SPEC>;

impl Fweis82 {
    #[doc = "Timeout Status Flag n (n = 32 to 63)"]
    #[inline(always)]
    pub fn tos31_to_tos0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis82_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis82_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis82 {
    #[inline(always)]
    fn default() -> Fweis82 {
        <crate::RegValueT<Fweis82_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie82_SPEC;
impl crate::sealed::RegSpec for Fweie82_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 82"]
pub type Fweie82 = crate::RegValueT<Fweie82_SPEC>;

impl Fweie82 {
    #[doc = "Timeout Enable n (n = 32 to 63)"]
    #[inline(always)]
    pub fn toe31_to_toe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie82_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie82_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie82 {
    #[inline(always)]
    fn default() -> Fweie82 {
        <crate::RegValueT<Fweie82_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid82_SPEC;
impl crate::sealed::RegSpec for Fweid82_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 82"]
pub type Fweid82 = crate::RegValueT<Fweid82_SPEC>;

impl Fweid82 {
    #[doc = "Timeout Disable n (n = 32 to 63)"]
    #[inline(always)]
    pub fn tod31_to_tod0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid82_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid82_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid82 {
    #[inline(always)]
    fn default() -> Fweid82 {
        <crate::RegValueT<Fweid82_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweis83_SPEC;
impl crate::sealed::RegSpec for Fweis83_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 83"]
pub type Fweis83 = crate::RegValueT<Fweis83_SPEC>;

impl Fweis83 {
    #[doc = "Timeout Status Flag n (n = 0 to 31)"]
    #[inline(always)]
    pub fn tos31_to_tos0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweis83_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweis83_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweis83 {
    #[inline(always)]
    fn default() -> Fweis83 {
        <crate::RegValueT<Fweis83_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweie83_SPEC;
impl crate::sealed::RegSpec for Fweie83_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 83"]
pub type Fweie83 = crate::RegValueT<Fweie83_SPEC>;

impl Fweie83 {
    #[doc = "Timeout Enable n (n = 0 to 31)"]
    #[inline(always)]
    pub fn toe31_to_toe0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweie83_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweie83_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweie83 {
    #[inline(always)]
    fn default() -> Fweie83 {
        <crate::RegValueT<Fweie83_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fweid83_SPEC;
impl crate::sealed::RegSpec for Fweid83_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 83"]
pub type Fweid83 = crate::RegValueT<Fweid83_SPEC>;

impl Fweid83 {
    #[doc = "Timeout Disable n (n = 0 to 31)"]
    #[inline(always)]
    pub fn tod31_to_tod0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fweid83_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fweid83_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fweid83 {
    #[inline(always)]
    fn default() -> Fweid83 {
        <crate::RegValueT<Fweid83_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmis0_SPEC;
impl crate::sealed::RegSpec for Fwmis0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Status Register 0"]
pub type Fwmis0 = crate::RegValueT<Fwmis0_SPEC>;

impl Fwmis0 {
    #[doc = "L3 Table Full Status Flag"]
    #[inline(always)]
    pub fn lthtfs(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fwmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Table Full Status Flag"]
    #[inline(always)]
    pub fn mactfs(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Fwmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fwmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VLAN Table Full Status Flag"]
    #[inline(always)]
    pub fn vlantfs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Fwmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fwmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Address Deleted Aging Status Flag"]
    #[inline(always)]
    pub fn macadas(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Fwmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Fwmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwmis0 {
    #[inline(always)]
    fn default() -> Fwmis0 {
        <crate::RegValueT<Fwmis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmie0_SPEC;
impl crate::sealed::RegSpec for Fwmie0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Enable Register 0"]
pub type Fwmie0 = crate::RegValueT<Fwmie0_SPEC>;

impl Fwmie0 {
    #[doc = "L3 Table Full Enable"]
    #[inline(always)]
    pub fn lthtfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fwmie0::Lthtfe,
        fwmie0::Lthtfe,
        Fwmie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fwmie0::Lthtfe,
            fwmie0::Lthtfe,
            Fwmie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Table Full Enable"]
    #[inline(always)]
    pub fn mactfe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fwmie0::Mactfe,
        fwmie0::Mactfe,
        Fwmie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fwmie0::Mactfe,
            fwmie0::Mactfe,
            Fwmie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VLAN Table Full Enable"]
    #[inline(always)]
    pub fn vlantfe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fwmie0::Vlantfe,
        fwmie0::Vlantfe,
        Fwmie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fwmie0::Vlantfe,
            fwmie0::Vlantfe,
            Fwmie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MAC Address Deleted Aging Enable"]
    #[inline(always)]
    pub fn macadae(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        fwmie0::Macadae,
        fwmie0::Macadae,
        Fwmie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            fwmie0::Macadae,
            fwmie0::Macadae,
            Fwmie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fwmie0 {
    #[inline(always)]
    fn default() -> Fwmie0 {
        <crate::RegValueT<Fwmie0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fwmie0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lthtfe_SPEC;
    pub type Lthtfe = crate::EnumBitfieldStruct<u8, Lthtfe_SPEC>;
    impl Lthtfe {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mactfe_SPEC;
    pub type Mactfe = crate::EnumBitfieldStruct<u8, Mactfe_SPEC>;
    impl Mactfe {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlantfe_SPEC;
    pub type Vlantfe = crate::EnumBitfieldStruct<u8, Vlantfe_SPEC>;
    impl Vlantfe {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macadae_SPEC;
    pub type Macadae = crate::EnumBitfieldStruct<u8, Macadae_SPEC>;
    impl Macadae {
        #[doc = "Interrupt disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwmid0_SPEC;
impl crate::sealed::RegSpec for Fwmid0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Disable Register 0"]
pub type Fwmid0 = crate::RegValueT<Fwmid0_SPEC>;

impl Fwmid0 {
    #[doc = "L3 Table Full Disable"]
    #[inline(always)]
    pub fn lthtfd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fwmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fwmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Table Full Disable"]
    #[inline(always)]
    pub fn mactfd(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Fwmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fwmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VLAN Table Full Disable"]
    #[inline(always)]
    pub fn vlantfd(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Fwmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fwmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MAC Address Deleted Aging Disable"]
    #[inline(always)]
    pub fn macadad(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Fwmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Fwmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fwmid0 {
    #[inline(always)]
    fn default() -> Fwmid0 {
        <crate::RegValueT<Fwmid0_SPEC> as RegisterValue<_>>::new(0)
    }
}
