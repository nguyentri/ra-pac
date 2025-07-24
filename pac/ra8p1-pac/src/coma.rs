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
#[doc = r"Common Agent"]
unsafe impl ::core::marker::Send for super::Coma {}
unsafe impl ::core::marker::Sync for super::Coma {}
impl super::Coma {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "IP Version Register"]
    #[inline(always)]
    pub const fn ripv(&self) -> &'static crate::common::Reg<self::Ripv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ripv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Reset Configuration Register"]
    #[inline(always)]
    pub const fn rrc(&self) -> &'static crate::common::Reg<self::Rrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Clock Enable Configuration Register"]
    #[inline(always)]
    pub const fn rcec(&self) -> &'static crate::common::Reg<self::Rcec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Clock Disable Configuration Register"]
    #[inline(always)]
    pub const fn rcdc(&self) -> &'static crate::common::Reg<self::Rcdc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rcdc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Buffer Pool IPV Based Watermark Configuration Register %s"]
    #[inline(always)]
    pub const fn cabpibwmc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc0(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc1(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc2(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc3(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc4(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc5(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc6(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpibwmc7(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpibwmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpibwmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }

    #[doc = "Buffer Pool Watermark Level Configuration Register"]
    #[inline(always)]
    pub const fn cabpwmlc(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpwmlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpwmlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Buffer Pointer Pause Frame Level i Configuration Register"]
    #[inline(always)]
    pub const fn cabppflci(
        &self,
    ) -> &'static crate::common::Reg<self::CabppflCi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CabppflCi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Port %s Buffer Pool Watermark Level Configuration Register"]
    #[inline(always)]
    pub const fn cabppwmlc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabppwmlc_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60usize))
        }
    }
    #[inline(always)]
    pub const fn cabppwmlc0(
        &self,
    ) -> &'static crate::common::Reg<self::Cabppwmlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabppwmlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabppwmlc1(
        &self,
    ) -> &'static crate::common::Reg<self::Cabppwmlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabppwmlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabppwmlc2(
        &self,
    ) -> &'static crate::common::Reg<self::Cabppwmlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabppwmlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }

    #[doc = "Port i Buffer Pointer Pause Frame Level %s Configuration Register"]
    #[inline(always)]
    pub const fn cabpppflc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabpppflc0_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa0usize))
        }
    }
    #[inline(always)]
    pub const fn cabpppflc00(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpppflc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpppflc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpppflc01(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpppflc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpppflc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }

    #[doc = "Port i Buffer Pointer Pause Frame Level %s Configuration Register"]
    #[inline(always)]
    pub const fn cabpppflc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabpppflc1_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa8usize))
        }
    }
    #[inline(always)]
    pub const fn cabpppflc10(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpppflc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpppflc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpppflc11(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpppflc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpppflc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }

    #[doc = "Port i Buffer Pointer Pause Frame Level %s Configuration Register"]
    #[inline(always)]
    pub const fn cabpppflc2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabpppflc2_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb0usize))
        }
    }
    #[inline(always)]
    pub const fn cabpppflc20(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpppflc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpppflc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpppflc21(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpppflc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpppflc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }

    #[doc = "Port %s Buffer Pointer Utilization Level Configuration Register i"]
    #[inline(always)]
    pub const fn cabpulc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabpulc_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn cabpulc0(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpulc1(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpulc2(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }

    #[doc = "Buffer Pool Initialization Register Monitoring Register"]
    #[inline(always)]
    pub const fn cabpirm(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpirm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cabpirm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "Buffer Pool Pointer Count Monitoring Register"]
    #[inline(always)]
    pub const fn cabppcm(
        &self,
    ) -> &'static crate::common::Reg<self::Cabppcm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabppcm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(324usize),
            )
        }
    }

    #[doc = "Buffer Pool Pointer Least Count Monitoring Register"]
    #[inline(always)]
    pub const fn cabplcm(
        &self,
    ) -> &'static crate::common::Reg<self::Cabplcm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabplcm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(328usize),
            )
        }
    }

    #[doc = "Port %s Buffer Pointer Count Monitoring Register"]
    #[inline(always)]
    pub const fn cabpcpm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabpcpm_SPEC, crate::common::R>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x180usize))
        }
    }
    #[inline(always)]
    pub const fn cabpcpm0(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpcpm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabpcpm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpcpm1(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpcpm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabpcpm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpcpm2(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpcpm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabpcpm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x188usize),
            )
        }
    }

    #[doc = "Port %s Buffer Pointer Maximum Count Monitoring Register"]
    #[inline(always)]
    pub const fn cabpmcpm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cabpmcpm_SPEC, crate::common::R>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn cabpmcpm0(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpmcpm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabpmcpm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpmcpm1(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpmcpm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabpmcpm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cabpmcpm2(
        &self,
    ) -> &'static crate::common::Reg<self::Cabpmcpm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cabpmcpm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }

    #[doc = "Rejected Descriptor Number Monitoring Register"]
    #[inline(always)]
    pub const fn cardnm(&self) -> &'static crate::common::Reg<self::Cardnm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cardnm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "Rejected Descriptor Maximum Number Monitoring Register"]
    #[inline(always)]
    pub const fn cardmnm(
        &self,
    ) -> &'static crate::common::Reg<self::Cardmnm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cardmnm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "Rejected Descriptor Counter Register"]
    #[inline(always)]
    pub const fn cardcn(&self) -> &'static crate::common::Reg<self::Cardcn_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cardcn_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn caeis0(
        &self,
    ) -> &'static crate::common::Reg<self::Caeis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caeis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn caeie0(
        &self,
    ) -> &'static crate::common::Reg<self::Caeie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caeie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn caeid0(&self) -> &'static crate::common::Reg<self::Caeid0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Caeid0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1032usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn caeis1(
        &self,
    ) -> &'static crate::common::Reg<self::Caeis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caeis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn caeie1(
        &self,
    ) -> &'static crate::common::Reg<self::Caeie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caeie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1044usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn caeid1(&self) -> &'static crate::common::Reg<self::Caeid1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Caeid1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn camis0(
        &self,
    ) -> &'static crate::common::Reg<self::Camis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Camis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1088usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn camie0(
        &self,
    ) -> &'static crate::common::Reg<self::Camie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Camie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1092usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn camid0(&self) -> &'static crate::common::Reg<self::Camid0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Camid0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1096usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn camis1(
        &self,
    ) -> &'static crate::common::Reg<self::Camis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Camis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1104usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn camie1(
        &self,
    ) -> &'static crate::common::Reg<self::Camie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Camie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1108usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn camid1(
        &self,
    ) -> &'static crate::common::Reg<self::Camid1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Camid1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1112usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ripv_SPEC;
impl crate::sealed::RegSpec for Ripv_SPEC {
    type DataType = u32;
}

#[doc = "IP Version Register"]
pub type Ripv = crate::RegValueT<Ripv_SPEC>;

impl Ripv {
    #[doc = "Top Module IP Version Number"]
    #[inline(always)]
    pub fn tipv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ripv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ripv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gateway CPU Agent IP Version Number"]
    #[inline(always)]
    pub fn gwipv(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ripv_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ripv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Forwarding Engine IP Version Number"]
    #[inline(always)]
    pub fn fwipv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Ripv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Ripv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Ethernet Agent IP Version Number"]
    #[inline(always)]
    pub fn eaipv(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ripv_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Ripv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Fabric Bus IP Version Number"]
    #[inline(always)]
    pub fn fbipv(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Ripv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Ripv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Ethernet Common Agent IP Version Number"]
    #[inline(always)]
    pub fn caipv(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Ripv_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Ripv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ripv {
    #[inline(always)]
    fn default() -> Ripv {
        <crate::RegValueT<Ripv_SPEC> as RegisterValue<_>>::new(3355443)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrc_SPEC;
impl crate::sealed::RegSpec for Rrc_SPEC {
    type DataType = u32;
}

#[doc = "Reset Configuration Register"]
pub type Rrc = crate::RegValueT<Rrc_SPEC>;

impl Rrc {
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn rr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rrc::Rr, rrc::Rr, Rrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,rrc::Rr,rrc::Rr,Rrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rrc {
    #[inline(always)]
    fn default() -> Rrc {
        <crate::RegValueT<Rrc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rr_SPEC;
    pub type Rr = crate::EnumBitfieldStruct<u8, Rr_SPEC>;
    impl Rr {
        #[doc = "ESWM not reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "ESWM reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcec_SPEC;
impl crate::sealed::RegSpec for Rcec_SPEC {
    type DataType = u32;
}

#[doc = "Clock Enable Configuration Register"]
pub type Rcec = crate::RegValueT<Rcec_SPEC>;

impl Rcec {
    #[doc = "Agent i Clock Enable (i = 0 to 2)"]
    #[inline(always)]
    pub fn ace0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcec::Ace0,
        rcec::Ace0,
        Rcec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcec::Ace0,
            rcec::Ace0,
            Rcec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Agent i Clock Enable (i = 0 to 2)"]
    #[inline(always)]
    pub fn ace1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rcec::Ace1,
        rcec::Ace1,
        Rcec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rcec::Ace1,
            rcec::Ace1,
            Rcec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Agent i Clock Enable (i = 0 to 2)"]
    #[inline(always)]
    pub fn ace2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rcec::Ace2,
        rcec::Ace2,
        Rcec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rcec::Ace2,
            rcec::Ace2,
            Rcec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn rce(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rcec::Rce,
        rcec::Rce,
        Rcec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rcec::Rce,
            rcec::Rce,
            Rcec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcec {
    #[inline(always)]
    fn default() -> Rcec {
        <crate::RegValueT<Rcec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ace0_SPEC;
    pub type Ace0 = crate::EnumBitfieldStruct<u8, Ace0_SPEC>;
    impl Ace0 {
        #[doc = "Agent i clock disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Agent i clock enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ace1_SPEC;
    pub type Ace1 = crate::EnumBitfieldStruct<u8, Ace1_SPEC>;
    impl Ace1 {
        #[doc = "Agent i clock disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Agent i clock enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ace2_SPEC;
    pub type Ace2 = crate::EnumBitfieldStruct<u8, Ace2_SPEC>;
    impl Ace2 {
        #[doc = "Agent i clock disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Agent i clock enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rce_SPEC;
    pub type Rce = crate::EnumBitfieldStruct<u8, Rce_SPEC>;
    impl Rce {
        #[doc = "ESWM clock disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "ESWM clock enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcdc_SPEC;
impl crate::sealed::RegSpec for Rcdc_SPEC {
    type DataType = u32;
}

#[doc = "Clock Disable Configuration Register"]
pub type Rcdc = crate::RegValueT<Rcdc_SPEC>;

impl Rcdc {
    #[doc = "Agent i Clock Disable (i = 0 to 2)"]
    #[inline(always)]
    pub fn acd2_to_acd0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Rcdc_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Rcdc_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Clock Disable"]
    #[inline(always)]
    pub fn rcd(self) -> crate::common::RegisterFieldBool<16, 1, 0, Rcdc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Rcdc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rcdc {
    #[inline(always)]
    fn default() -> Rcdc {
        <crate::RegValueT<Rcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpibwmc_SPEC;
impl crate::sealed::RegSpec for Cabpibwmc_SPEC {
    type DataType = u32;
}

#[doc = "Buffer Pool IPV Based Watermark Configuration Register %s"]
pub type Cabpibwmc = crate::RegValueT<Cabpibwmc_SPEC>;

impl Cabpibwmc {
    #[doc = "IPV Based Unsecure Watermark Pointer Number"]
    #[inline(always)]
    pub fn ibuwmpn(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabpibwmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabpibwmc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "IPV Based Secure Watermark Pointer Number"]
    #[inline(always)]
    pub fn ibswmpn(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabpibwmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabpibwmc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpibwmc {
    #[inline(always)]
    fn default() -> Cabpibwmc {
        <crate::RegValueT<Cabpibwmc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpwmlc_SPEC;
impl crate::sealed::RegSpec for Cabpwmlc_SPEC {
    type DataType = u32;
}

#[doc = "Buffer Pool Watermark Level Configuration Register"]
pub type Cabpwmlc = crate::RegValueT<Cabpwmlc_SPEC>;

impl Cabpwmlc {
    #[doc = "Watermark Flush Level"]
    #[inline(always)]
    pub fn wmfl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabpwmlc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabpwmlc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Watermark Critical Level"]
    #[inline(always)]
    pub fn wmcl(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabpwmlc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabpwmlc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpwmlc {
    #[inline(always)]
    fn default() -> Cabpwmlc {
        <crate::RegValueT<Cabpwmlc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CabppflCi_SPEC;
impl crate::sealed::RegSpec for CabppflCi_SPEC {
    type DataType = u32;
}

#[doc = "Buffer Pointer Pause Frame Level i Configuration Register"]
pub type CabppflCi = crate::RegValueT<CabppflCi_SPEC>;

impl CabppflCi {
    #[doc = "Pause De-Assertion Level"]
    #[inline(always)]
    pub fn pdl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, CabppflCi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,CabppflCi_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pause Assertion Level"]
    #[inline(always)]
    pub fn pal(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, CabppflCi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,CabppflCi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CabppflCi {
    #[inline(always)]
    fn default() -> CabppflCi {
        <crate::RegValueT<CabppflCi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabppwmlc_SPEC;
impl crate::sealed::RegSpec for Cabppwmlc_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Buffer Pool Watermark Level Configuration Register"]
pub type Cabppwmlc = crate::RegValueT<Cabppwmlc_SPEC>;

impl Cabppwmlc {
    #[doc = "Watermark Flush Level"]
    #[inline(always)]
    pub fn pwmfl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabppwmlc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabppwmlc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Watermark Critical Level"]
    #[inline(always)]
    pub fn pwmcl(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabppwmlc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabppwmlc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabppwmlc {
    #[inline(always)]
    fn default() -> Cabppwmlc {
        <crate::RegValueT<Cabppwmlc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpppflc0_SPEC;
impl crate::sealed::RegSpec for Cabpppflc0_SPEC {
    type DataType = u32;
}

#[doc = "Port i Buffer Pointer Pause Frame Level %s Configuration Register"]
pub type Cabpppflc0 = crate::RegValueT<Cabpppflc0_SPEC>;

impl Cabpppflc0 {
    #[doc = "Pause De-Assertion Level"]
    #[inline(always)]
    pub fn ppdl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabpppflc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabpppflc0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pause Assertion Level"]
    #[inline(always)]
    pub fn ppal(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabpppflc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabpppflc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpppflc0 {
    #[inline(always)]
    fn default() -> Cabpppflc0 {
        <crate::RegValueT<Cabpppflc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpppflc1_SPEC;
impl crate::sealed::RegSpec for Cabpppflc1_SPEC {
    type DataType = u32;
}

#[doc = "Port i Buffer Pointer Pause Frame Level %s Configuration Register"]
pub type Cabpppflc1 = crate::RegValueT<Cabpppflc1_SPEC>;

impl Cabpppflc1 {
    #[doc = "Pause De-Assertion Level"]
    #[inline(always)]
    pub fn ppdl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabpppflc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabpppflc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pause Assertion Level"]
    #[inline(always)]
    pub fn ppal(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabpppflc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabpppflc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpppflc1 {
    #[inline(always)]
    fn default() -> Cabpppflc1 {
        <crate::RegValueT<Cabpppflc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpppflc2_SPEC;
impl crate::sealed::RegSpec for Cabpppflc2_SPEC {
    type DataType = u32;
}

#[doc = "Port i Buffer Pointer Pause Frame Level %s Configuration Register"]
pub type Cabpppflc2 = crate::RegValueT<Cabpppflc2_SPEC>;

impl Cabpppflc2 {
    #[doc = "Pause De-Assertion Level"]
    #[inline(always)]
    pub fn ppdl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabpppflc2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabpppflc2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pause Assertion Level"]
    #[inline(always)]
    pub fn ppal(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabpppflc2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabpppflc2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpppflc2 {
    #[inline(always)]
    fn default() -> Cabpppflc2 {
        <crate::RegValueT<Cabpppflc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpulc_SPEC;
impl crate::sealed::RegSpec for Cabpulc_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Buffer Pointer Utilization Level Configuration Register i"]
pub type Cabpulc = crate::RegValueT<Cabpulc_SPEC>;

impl Cabpulc {
    #[doc = "Maximum Number of Pointer"]
    #[inline(always)]
    pub fn mxnpn(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabpulc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabpulc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Minimum Number of Pointer"]
    #[inline(always)]
    pub fn mnnpn(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabpulc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabpulc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpulc {
    #[inline(always)]
    fn default() -> Cabpulc {
        <crate::RegValueT<Cabpulc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpirm_SPEC;
impl crate::sealed::RegSpec for Cabpirm_SPEC {
    type DataType = u32;
}

#[doc = "Buffer Pool Initialization Register Monitoring Register"]
pub type Cabpirm = crate::RegValueT<Cabpirm_SPEC>;

impl Cabpirm {
    #[doc = "Buffer Pool Initialization Ongoing"]
    #[inline(always)]
    pub fn bpiog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cabpirm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cabpirm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Buffer Pool Ready"]
    #[inline(always)]
    pub fn bpr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cabpirm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cabpirm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cabpirm {
    #[inline(always)]
    fn default() -> Cabpirm {
        <crate::RegValueT<Cabpirm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabppcm_SPEC;
impl crate::sealed::RegSpec for Cabppcm_SPEC {
    type DataType = u32;
}

#[doc = "Buffer Pool Pointer Count Monitoring Register"]
pub type Cabppcm = crate::RegValueT<Cabppcm_SPEC>;

impl Cabppcm {
    #[doc = "Remaining Pointer Count"]
    #[inline(always)]
    pub fn rpc(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cabppcm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cabppcm_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Total Pointer Count"]
    #[inline(always)]
    pub fn tpc(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cabppcm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cabppcm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabppcm {
    #[inline(always)]
    fn default() -> Cabppcm {
        <crate::RegValueT<Cabppcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabplcm_SPEC;
impl crate::sealed::RegSpec for Cabplcm_SPEC {
    type DataType = u32;
}

#[doc = "Buffer Pool Pointer Least Count Monitoring Register"]
pub type Cabplcm = crate::RegValueT<Cabplcm_SPEC>;

impl Cabplcm {
    #[doc = "Least Remaining Pointer Count"]
    #[inline(always)]
    pub fn lrc(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cabplcm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cabplcm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabplcm {
    #[inline(always)]
    fn default() -> Cabplcm {
        <crate::RegValueT<Cabplcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpcpm_SPEC;
impl crate::sealed::RegSpec for Cabpcpm_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Buffer Pointer Count Monitoring Register"]
pub type Cabpcpm = crate::RegValueT<Cabpcpm_SPEC>;

impl Cabpcpm {
    #[doc = "Received Pointer Count"]
    #[inline(always)]
    pub fn rpcp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cabpcpm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cabpcpm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpcpm {
    #[inline(always)]
    fn default() -> Cabpcpm {
        <crate::RegValueT<Cabpcpm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cabpmcpm_SPEC;
impl crate::sealed::RegSpec for Cabpmcpm_SPEC {
    type DataType = u32;
}

#[doc = "Port %s Buffer Pointer Maximum Count Monitoring Register"]
pub type Cabpmcpm = crate::RegValueT<Cabpmcpm_SPEC>;

impl Cabpmcpm {
    #[doc = "Received Pointer Maximum Count"]
    #[inline(always)]
    pub fn rpmcp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cabpmcpm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cabpmcpm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cabpmcpm {
    #[inline(always)]
    fn default() -> Cabpmcpm {
        <crate::RegValueT<Cabpmcpm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cardnm_SPEC;
impl crate::sealed::RegSpec for Cardnm_SPEC {
    type DataType = u32;
}

#[doc = "Rejected Descriptor Number Monitoring Register"]
pub type Cardnm = crate::RegValueT<Cardnm_SPEC>;

impl Cardnm {
    #[doc = "Rejected Descriptor Number in Reject RAM"]
    #[inline(always)]
    pub fn rdnrr(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cardnm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cardnm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cardnm {
    #[inline(always)]
    fn default() -> Cardnm {
        <crate::RegValueT<Cardnm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cardmnm_SPEC;
impl crate::sealed::RegSpec for Cardmnm_SPEC {
    type DataType = u32;
}

#[doc = "Rejected Descriptor Maximum Number Monitoring Register"]
pub type Cardmnm = crate::RegValueT<Cardmnm_SPEC>;

impl Cardmnm {
    #[doc = "Rejected Descriptor Maximum Number in Reject RAM"]
    #[inline(always)]
    pub fn rdmnrr(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cardmnm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cardmnm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cardmnm {
    #[inline(always)]
    fn default() -> Cardmnm {
        <crate::RegValueT<Cardmnm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cardcn_SPEC;
impl crate::sealed::RegSpec for Cardcn_SPEC {
    type DataType = u32;
}

#[doc = "Rejected Descriptor Counter Register"]
pub type Cardcn = crate::RegValueT<Cardcn_SPEC>;

impl Cardcn {
    #[doc = "Rejected Descriptor Number"]
    #[inline(always)]
    pub fn rdn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cardcn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cardcn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cardcn {
    #[inline(always)]
    fn default() -> Cardcn {
        <crate::RegValueT<Cardcn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caeis0_SPEC;
impl crate::sealed::RegSpec for Caeis0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 0"]
pub type Caeis0 = crate::RegValueT<Caeis0_SPEC>;

impl Caeis0 {
    #[doc = "Buffer Pool Out of Pointer Status Flag"]
    #[inline(always)]
    pub fn bpops(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Caeis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Caeis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Critical Level Overtook Status Flag"]
    #[inline(always)]
    pub fn wmclos(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Caeis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Caeis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Flush Level Overtook Status Flag"]
    #[inline(always)]
    pub fn wmflos(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Caeis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Caeis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Caeis0 {
    #[inline(always)]
    fn default() -> Caeis0 {
        <crate::RegValueT<Caeis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caeie0_SPEC;
impl crate::sealed::RegSpec for Caeie0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 0"]
pub type Caeie0 = crate::RegValueT<Caeie0_SPEC>;

impl Caeie0 {
    #[doc = "Buffer Pool Out of Pointer Enable"]
    #[inline(always)]
    pub fn bpope(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Caeie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Caeie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Critical Level Overtook Enable"]
    #[inline(always)]
    pub fn wmcloe(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Caeie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Caeie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Flush Level Overtook Enable"]
    #[inline(always)]
    pub fn wmfloe(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Caeie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Caeie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Caeie0 {
    #[inline(always)]
    fn default() -> Caeie0 {
        <crate::RegValueT<Caeie0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caeid0_SPEC;
impl crate::sealed::RegSpec for Caeid0_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 0"]
pub type Caeid0 = crate::RegValueT<Caeid0_SPEC>;

impl Caeid0 {
    #[doc = "Buffer Pool Out of Pointer Disable"]
    #[inline(always)]
    pub fn bpopd(self) -> crate::common::RegisterFieldBool<8, 1, 0, Caeid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Caeid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Critical Level Overtook Disable"]
    #[inline(always)]
    pub fn wmclod(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Caeid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Caeid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Watermark Flush Level Overtook Disable"]
    #[inline(always)]
    pub fn wmflod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Caeid0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Caeid0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Caeid0 {
    #[inline(always)]
    fn default() -> Caeid0 {
        <crate::RegValueT<Caeid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caeis1_SPEC;
impl crate::sealed::RegSpec for Caeis1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register 1"]
pub type Caeis1 = crate::RegValueT<Caeis1_SPEC>;

impl Caeis1 {
    #[doc = "Port n Watermark Critical Level Overtook Status Flag (n = 0 to 2)"]
    #[inline(always)]
    pub fn pwmclos2_to_pwmclos0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Caeis1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Caeis1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Port n Watermark Flush Level Overtook Status Flag (n = 0 to 2)"]
    #[inline(always)]
    pub fn pwmflos2_to_pwmflos0(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Caeis1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Caeis1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Caeis1 {
    #[inline(always)]
    fn default() -> Caeis1 {
        <crate::RegValueT<Caeis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caeie1_SPEC;
impl crate::sealed::RegSpec for Caeie1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register 1"]
pub type Caeie1 = crate::RegValueT<Caeie1_SPEC>;

impl Caeie1 {
    #[doc = "Port n Watermark Critical Level Overtook Enable (n = 0 to 2)"]
    #[inline(always)]
    pub fn pwmcloe2_to_pwmcloe0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Caeie1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Caeie1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Port n Watermark Flush Level Overtook Enable (n = 0 to 2)"]
    #[inline(always)]
    pub fn pwmfloe2_to_pwmfloe0(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Caeie1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Caeie1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Caeie1 {
    #[inline(always)]
    fn default() -> Caeie1 {
        <crate::RegValueT<Caeie1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caeid1_SPEC;
impl crate::sealed::RegSpec for Caeid1_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register 1"]
pub type Caeid1 = crate::RegValueT<Caeid1_SPEC>;

impl Caeid1 {
    #[doc = "Port n Watermark Critical Level Overtook Disable (n = 0 to 2)"]
    #[inline(always)]
    pub fn pwmclod2_to_pwmclod0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Caeid1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Caeid1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Port n Watermark Flush Level Overtook Disable (n = 0 to 2)"]
    #[inline(always)]
    pub fn pwmflod2_to_pwmflod0(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Caeid1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Caeid1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Caeid1 {
    #[inline(always)]
    fn default() -> Caeid1 {
        <crate::RegValueT<Caeid1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camis0_SPEC;
impl crate::sealed::RegSpec for Camis0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Status Register 0"]
pub type Camis0 = crate::RegValueT<Camis0_SPEC>;

impl Camis0 {
    #[doc = "Pause Frame Status Flag n (n= 0 to 1)"]
    #[inline(always)]
    pub fn pfs1_to_pfs0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Camis0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Camis0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Camis0 {
    #[inline(always)]
    fn default() -> Camis0 {
        <crate::RegValueT<Camis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camie0_SPEC;
impl crate::sealed::RegSpec for Camie0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Enable Register 0"]
pub type Camie0 = crate::RegValueT<Camie0_SPEC>;

impl Camie0 {
    #[doc = "Pause Frame Enable n (n = 0 to 1)"]
    #[inline(always)]
    pub fn pfe1_to_pfe0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Camie0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Camie0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Camie0 {
    #[inline(always)]
    fn default() -> Camie0 {
        <crate::RegValueT<Camie0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camid0_SPEC;
impl crate::sealed::RegSpec for Camid0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Disable Register 0"]
pub type Camid0 = crate::RegValueT<Camid0_SPEC>;

impl Camid0 {
    #[doc = "Pause Frame Disable n (n = 0 to 1)"]
    #[inline(always)]
    pub fn pfd1_to_pfd0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Camid0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Camid0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Camid0 {
    #[inline(always)]
    fn default() -> Camid0 {
        <crate::RegValueT<Camid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camis1_SPEC;
impl crate::sealed::RegSpec for Camis1_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Status Register 1"]
pub type Camis1 = crate::RegValueT<Camis1_SPEC>;

impl Camis1 {
    #[doc = "Port 0 Pause Frame Status Flag 0"]
    #[inline(always)]
    pub fn ppfs00(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Camis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Camis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 0 Pause Frame Status Flag 1"]
    #[inline(always)]
    pub fn ppfs01(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Camis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Camis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 1 Pause Frame Status Flag 0"]
    #[inline(always)]
    pub fn ppfs10(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Camis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Camis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 1 Pause Frame Status Flag 1"]
    #[inline(always)]
    pub fn ppfs11(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Camis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Camis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 2 Pause Frame Status Flag 0"]
    #[inline(always)]
    pub fn ppfs20(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Camis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Camis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 2 Pause Frame Status Flag 1"]
    #[inline(always)]
    pub fn ppfs21(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Camis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Camis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Camis1 {
    #[inline(always)]
    fn default() -> Camis1 {
        <crate::RegValueT<Camis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camie1_SPEC;
impl crate::sealed::RegSpec for Camie1_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Enable Register 1"]
pub type Camie1 = crate::RegValueT<Camie1_SPEC>;

impl Camie1 {
    #[doc = "Port 0 Pause Frame Enable 0"]
    #[inline(always)]
    pub fn ppfe00(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Camie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Camie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 0 Pause Frame Enable 1"]
    #[inline(always)]
    pub fn ppfe01(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Camie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Camie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 1 Pause Frame Enable 0"]
    #[inline(always)]
    pub fn ppfe10(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Camie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Camie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 1 Pause Frame Enable 1"]
    #[inline(always)]
    pub fn ppfe11(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Camie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Camie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 2 Pause Frame Enable 0"]
    #[inline(always)]
    pub fn ppfe20(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Camie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Camie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 2 Pause Frame Enable 1"]
    #[inline(always)]
    pub fn ppfe21(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Camie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Camie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Camie1 {
    #[inline(always)]
    fn default() -> Camie1 {
        <crate::RegValueT<Camie1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camid1_SPEC;
impl crate::sealed::RegSpec for Camid1_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Disable Register 1"]
pub type Camid1 = crate::RegValueT<Camid1_SPEC>;

impl Camid1 {
    #[doc = "Port 0 Pause Frame Disable 0"]
    #[inline(always)]
    pub fn ppfd00(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Camid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Camid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 0 Pause Frame Disable 1"]
    #[inline(always)]
    pub fn ppfd01(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Camid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Camid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 1 Pause Frame Disable 0"]
    #[inline(always)]
    pub fn ppfd10(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Camid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Camid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 1 Pause Frame Disable 1"]
    #[inline(always)]
    pub fn ppfd11(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Camid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Camid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 2 Pause Frame Disable 0"]
    #[inline(always)]
    pub fn ppfd20(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Camid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Camid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Port 2 Pause Frame Disable 1"]
    #[inline(always)]
    pub fn ppfd21(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Camid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Camid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Camid1 {
    #[inline(always)]
    fn default() -> Camid1 {
        <crate::RegValueT<Camid1_SPEC> as RegisterValue<_>>::new(0)
    }
}
