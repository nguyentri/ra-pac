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
#[doc = r"Generic PTP Timer"]
unsafe impl ::core::marker::Send for super::GptpNs {}
unsafe impl ::core::marker::Sync for super::GptpNs {}
impl super::GptpNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "IP Version Register"]
    #[inline(always)]
    pub const fn ptpipv(&self) -> &'static crate::common::Reg<self::Ptpipv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpipv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Timer Enable Configuration Register"]
    #[inline(always)]
    pub const fn ptptmec(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptmec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptmec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Timer Disable Configuration Register"]
    #[inline(always)]
    pub const fn ptptmdc(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptmdc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptptmdc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Timer %s Increment Value Configuration Register"]
    #[inline(always)]
    pub const fn ptptivc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptptivc_SPEC, crate::common::RW>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }
    #[inline(always)]
    pub const fn ptptivc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptptivc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptivc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptivc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x60usize),
            )
        }
    }

    #[doc = "Timer %s Offset Value Configuration Register L"]
    #[inline(always)]
    pub const fn ptptovcl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptptovcl_SPEC, crate::common::RW>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30usize))
        }
    }
    #[inline(always)]
    pub const fn ptptovc0l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptovcl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptovcl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptptovc1l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptovcl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptovcl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }

    #[doc = "Timer %s Offset Value Configuration Register M"]
    #[inline(always)]
    pub const fn ptptovcm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptptovcm_SPEC, crate::common::RW>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x34usize))
        }
    }
    #[inline(always)]
    pub const fn ptptovcm0(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptovcm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptovcm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptptovcm1(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptovcm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptovcm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }

    #[doc = "Timer %s Offset Value Configuration Register U"]
    #[inline(always)]
    pub const fn ptptovcu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptptovcu_SPEC, crate::common::RW>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x38usize))
        }
    }
    #[inline(always)]
    pub const fn ptptovc0u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptovcu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptovcu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptptovc1u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptptovcu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptptovcu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }

    #[doc = "AVTP Timer %s Monitoring Register L"]
    #[inline(always)]
    pub const fn ptpavtptml(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpavtptml_SPEC, crate::common::R>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn ptpavtptm0l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpavtptml_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpavtptml_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpavtptm1l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpavtptml_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpavtptml_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }

    #[doc = "AVTP Timer %s Monitoring Register U"]
    #[inline(always)]
    pub const fn ptpavtptmu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpavtptmu_SPEC, crate::common::R>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x44usize))
        }
    }
    #[inline(always)]
    pub const fn ptpavtptm0u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpavtptmu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpavtptmu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpavtptm1u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpavtptmu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpavtptmu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }

    #[doc = "GPTP Timer %s Monitoring Register L"]
    #[inline(always)]
    pub const fn ptpgptptml(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpgptptml_SPEC, crate::common::R>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x50usize))
        }
    }
    #[inline(always)]
    pub const fn ptpgptptm0l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpgptptml_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpgptptml_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpgptptm1l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpgptptml_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpgptptml_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }

    #[doc = "GPTP Timer %s Monitoring Register M"]
    #[inline(always)]
    pub const fn ptpgptptmm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpgptptmm_SPEC, crate::common::R>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x54usize))
        }
    }
    #[inline(always)]
    pub const fn ptpgptptm0m(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpgptptmm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpgptptmm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpgptptm1m(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpgptptmm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpgptptmm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }

    #[doc = "GPTP Timer %s Monitoring Register U"]
    #[inline(always)]
    pub const fn ptpgptptmu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpgptptmu_SPEC, crate::common::R>,
        2,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x58usize))
        }
    }
    #[inline(always)]
    pub const fn ptpgptptm0u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpgptptmu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpgptptmu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpgptptm1u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpgptptmu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpgptptmu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }

    #[doc = "Media Clock %s Capture Configuration Register"]
    #[inline(always)]
    pub const fn ptpmccc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmccc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn ptpmccc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmccc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmccc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmccc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }

    #[doc = "Media Clock %s Capture Monitoring Register L"]
    #[inline(always)]
    pub const fn ptpmccml(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmccml_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }
    #[inline(always)]
    pub const fn ptpmccm0l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccml_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpmccml_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmccm1l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccml_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpmccml_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }

    #[doc = "Media Clock %s Capture Monitoring Register M"]
    #[inline(always)]
    pub const fn ptpmccmm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmccmm_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }
    #[inline(always)]
    pub const fn ptpmccm0m(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccmm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpmccmm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmccm1m(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccmm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpmccmm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }

    #[doc = "Media Clock %s Capture Monitoring Register U"]
    #[inline(always)]
    pub const fn ptpmccmu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmccmu_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20cusize))
        }
    }
    #[inline(always)]
    pub const fn ptpmccm0u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccmu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpmccmu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x20cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmccm1u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmccmu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpmccmu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x21cusize),
            )
        }
    }

    #[doc = "Media Clock %s Recovery Configuration Register"]
    #[inline(always)]
    pub const fn ptpmcrc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmcrc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
    #[inline(always)]
    pub const fn ptpmcrc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmcrc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }

    #[doc = "Media Clock %s Recovery Time Configuration Register L"]
    #[inline(always)]
    pub const fn ptpmcrtcl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmcrtcl_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x304usize))
        }
    }
    #[inline(always)]
    pub const fn ptpmcrtc0l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrtcl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrtcl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmcrtc1l(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrtcl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrtcl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }

    #[doc = "Media Clock %s Recovery Time Configuration Register M"]
    #[inline(always)]
    pub const fn ptpmcrtcm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmcrtcm_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x308usize))
        }
    }
    #[inline(always)]
    pub const fn ptpmcrtc0m(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrtcm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrtcm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmcrtc1m(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrtcm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrtcm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }

    #[doc = "Media Clock %s Recovery Time Configuration Register U"]
    #[inline(always)]
    pub const fn ptpmcrtcu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmcrtcu_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30cusize))
        }
    }
    #[inline(always)]
    pub const fn ptpmcrtc0u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrtcu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrtcu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmcrtc1u(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcrtcu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcrtcu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }

    #[doc = "Media Clock %s Pin Configuration Register"]
    #[inline(always)]
    pub const fn ptpmcpc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpmcpc_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }
    #[inline(always)]
    pub const fn ptpmcpc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x400usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpmcpc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpmcpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpmcpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x404usize),
            )
        }
    }

    #[doc = "Cyclic Compare %s Configuration Register 0 (c = 0 to 7)"]
    #[inline(always)]
    pub const fn ptpccc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x500usize))
        }
    }
    #[inline(always)]
    pub const fn ptpccc00(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x500usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc10(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc20(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc30(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x518usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc40(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x520usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc50(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x528usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc60(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x530usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc70(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x538usize),
            )
        }
    }

    #[doc = "Cyclic Compare %s Configuration Register 1 (c = 0 to 7)"]
    #[inline(always)]
    pub const fn ptpccc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x504usize))
        }
    }
    #[inline(always)]
    pub const fn ptpccc01(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x504usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc11(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc21(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x514usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc31(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x51cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc41(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x524usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc51(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc61(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x534usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ptpccc71(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpccc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpccc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x53cusize),
            )
        }
    }

    #[doc = "Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ptpis0(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1792usize),
            )
        }
    }

    #[doc = "Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn ptpie0(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1796usize),
            )
        }
    }

    #[doc = "Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn ptpid0(&self) -> &'static crate::common::Reg<self::Ptpid0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpid0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1800usize),
            )
        }
    }

    #[doc = "Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn ptpis1(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1808usize),
            )
        }
    }

    #[doc = "Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn ptpie1(
        &self,
    ) -> &'static crate::common::Reg<self::Ptpie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptpie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1812usize),
            )
        }
    }

    #[doc = "Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn ptpid1(&self) -> &'static crate::common::Reg<self::Ptpid1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ptpid1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1816usize),
            )
        }
    }

    #[doc = "Pulse Output Timer Configuration Register"]
    #[inline(always)]
    pub const fn potcfgr(
        &self,
    ) -> &'static crate::common::Reg<self::Potcfgr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potcfgr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

    #[doc = "Time Capture Register nL"]
    #[inline(always)]
    pub const fn potcprl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potcprl_SPEC, crate::common::R>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potcpr0l(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr1l(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr2l(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr3l(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Time Capture Register nM"]
    #[inline(always)]
    pub const fn potcprm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potcprm_SPEC, crate::common::R>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potcpr0m(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr1m(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr2m(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr3m(
        &self,
    ) -> &'static crate::common::Reg<self::Potcprm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcprm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Time Capture Register nU"]
    #[inline(always)]
    pub const fn potcpru(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potcpru_SPEC, crate::common::R>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potcpr0u(
        &self,
    ) -> &'static crate::common::Reg<self::Potcpru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcpru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr1u(
        &self,
    ) -> &'static crate::common::Reg<self::Potcpru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcpru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr2u(
        &self,
    ) -> &'static crate::common::Reg<self::Potcpru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcpru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcpr3u(
        &self,
    ) -> &'static crate::common::Reg<self::Potcpru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Potcpru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Pulse Output Timer Control Register %s"]
    #[inline(always)]
    pub const fn potcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potcr_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potcr0(&self) -> &'static crate::common::Reg<self::Potcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcr1(&self) -> &'static crate::common::Reg<self::Potcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcr2(&self) -> &'static crate::common::Reg<self::Potcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potcr3(&self) -> &'static crate::common::Reg<self::Potcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Period Setting Register nL"]
    #[inline(always)]
    pub const fn potperl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potperl_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potper0l(
        &self,
    ) -> &'static crate::common::Reg<self::Potperl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper1l(
        &self,
    ) -> &'static crate::common::Reg<self::Potperl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper2l(
        &self,
    ) -> &'static crate::common::Reg<self::Potperl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper3l(
        &self,
    ) -> &'static crate::common::Reg<self::Potperl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Period Setting Register nM"]
    #[inline(always)]
    pub const fn potperm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potperm_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potper0m(
        &self,
    ) -> &'static crate::common::Reg<self::Potperm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper1m(
        &self,
    ) -> &'static crate::common::Reg<self::Potperm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper2m(
        &self,
    ) -> &'static crate::common::Reg<self::Potperm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper3m(
        &self,
    ) -> &'static crate::common::Reg<self::Potperm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Pulse Width Setting Register %s"]
    #[inline(always)]
    pub const fn potpwr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potpwr_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potpwr0(
        &self,
    ) -> &'static crate::common::Reg<self::Potpwr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potpwr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potpwr1(
        &self,
    ) -> &'static crate::common::Reg<self::Potpwr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potpwr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potpwr2(
        &self,
    ) -> &'static crate::common::Reg<self::Potpwr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potpwr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potpwr3(
        &self,
    ) -> &'static crate::common::Reg<self::Potpwr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potpwr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Pulse Output Start Time Setting Register nL"]
    #[inline(always)]
    pub const fn potstrl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potstrl_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potstr0l(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr1l(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr2l(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr3l(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Pulse Output Start Time Setting Register nM"]
    #[inline(always)]
    pub const fn potstrm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potstrm_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1004usize))
        }
    }
    #[inline(always)]
    pub const fn potstr0m(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr1m(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1034usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr2m(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1064usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr3m(
        &self,
    ) -> &'static crate::common::Reg<self::Potstrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1094usize),
            )
        }
    }

    #[doc = "Period Setting Register nU"]
    #[inline(always)]
    pub const fn potperu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potperu_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1008usize))
        }
    }
    #[inline(always)]
    pub const fn potper0u(
        &self,
    ) -> &'static crate::common::Reg<self::Potperu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1008usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper1u(
        &self,
    ) -> &'static crate::common::Reg<self::Potperu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1038usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper2u(
        &self,
    ) -> &'static crate::common::Reg<self::Potperu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1068usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potper3u(
        &self,
    ) -> &'static crate::common::Reg<self::Potperu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potperu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1098usize),
            )
        }
    }

    #[doc = "Pulse Output Start Time Setting Register nU"]
    #[inline(always)]
    pub const fn potstru(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Potstru_SPEC, crate::common::RW>,
        4,
        0x30,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1008usize))
        }
    }
    #[inline(always)]
    pub const fn potstr0u(
        &self,
    ) -> &'static crate::common::Reg<self::Potstru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1008usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr1u(
        &self,
    ) -> &'static crate::common::Reg<self::Potstru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1038usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr2u(
        &self,
    ) -> &'static crate::common::Reg<self::Potstru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1068usize),
            )
        }
    }
    #[inline(always)]
    pub const fn potstr3u(
        &self,
    ) -> &'static crate::common::Reg<self::Potstru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Potstru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1098usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpipv_SPEC;
impl crate::sealed::RegSpec for Ptpipv_SPEC {
    type DataType = u32;
}

#[doc = "IP Version Register"]
pub type Ptpipv = crate::RegValueT<Ptpipv_SPEC>;

impl Ptpipv {
    #[doc = "IP Version"]
    #[inline(always)]
    pub fn ipv(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ptpipv_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ptpipv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpipv {
    #[inline(always)]
    fn default() -> Ptpipv {
        <crate::RegValueT<Ptpipv_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptptmec_SPEC;
impl crate::sealed::RegSpec for Ptptmec_SPEC {
    type DataType = u32;
}

#[doc = "Timer Enable Configuration Register"]
pub type Ptptmec = crate::RegValueT<Ptptmec_SPEC>;

impl Ptptmec {
    #[doc = "Timer q Enable (q = 0 to 1)"]
    #[inline(always)]
    pub fn te0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ptptmec::Te0,
        ptptmec::Te0,
        Ptptmec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ptptmec::Te0,
            ptptmec::Te0,
            Ptptmec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timer q Enable (q = 0 to 1)"]
    #[inline(always)]
    pub fn te1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ptptmec::Te1,
        ptptmec::Te1,
        Ptptmec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ptptmec::Te1,
            ptptmec::Te1,
            Ptptmec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptptmec {
    #[inline(always)]
    fn default() -> Ptptmec {
        <crate::RegValueT<Ptptmec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptptmec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te0_SPEC;
    pub type Te0 = crate::EnumBitfieldStruct<u8, Te0_SPEC>;
    impl Te0 {
        #[doc = "Timer q disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer q enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te1_SPEC;
    pub type Te1 = crate::EnumBitfieldStruct<u8, Te1_SPEC>;
    impl Te1 {
        #[doc = "Timer q disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer q enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptptmdc_SPEC;
impl crate::sealed::RegSpec for Ptptmdc_SPEC {
    type DataType = u32;
}

#[doc = "Timer Disable Configuration Register"]
pub type Ptptmdc = crate::RegValueT<Ptptmdc_SPEC>;

impl Ptptmdc {
    #[doc = "Timer n Disable (n = 0 to 1)"]
    #[inline(always)]
    pub fn td1_to_td0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Ptptmdc_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Ptptmdc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptptmdc {
    #[inline(always)]
    fn default() -> Ptptmdc {
        <crate::RegValueT<Ptptmdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptptivc_SPEC;
impl crate::sealed::RegSpec for Ptptivc_SPEC {
    type DataType = u32;
}

#[doc = "Timer %s Increment Value Configuration Register"]
pub type Ptptivc = crate::RegValueT<Ptptivc_SPEC>;

impl Ptptivc {
    #[doc = "Timer Increment Value"]
    #[inline(always)]
    pub fn tiv(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ptptivc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ptptivc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptptivc {
    #[inline(always)]
    fn default() -> Ptptivc {
        <crate::RegValueT<Ptptivc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptptovcl_SPEC;
impl crate::sealed::RegSpec for Ptptovcl_SPEC {
    type DataType = u32;
}

#[doc = "Timer %s Offset Value Configuration Register L"]
pub type Ptptovcl = crate::RegValueT<Ptptovcl_SPEC>;

impl Ptptovcl {
    #[doc = "Timer Offset Value Lower Part"]
    #[inline(always)]
    pub fn tovl(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffffff, 1, 0, u32, u32, Ptptovcl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fffffff,
            1,
            0,
            u32,
            u32,
            Ptptovcl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptptovcl {
    #[inline(always)]
    fn default() -> Ptptovcl {
        <crate::RegValueT<Ptptovcl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptptovcm_SPEC;
impl crate::sealed::RegSpec for Ptptovcm_SPEC {
    type DataType = u32;
}

#[doc = "Timer %s Offset Value Configuration Register M"]
pub type Ptptovcm = crate::RegValueT<Ptptovcm_SPEC>;

impl Ptptovcm {
    #[doc = "Timer Offset Value Middle Part"]
    #[inline(always)]
    pub fn tovm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ptptovcm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ptptovcm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptptovcm {
    #[inline(always)]
    fn default() -> Ptptovcm {
        <crate::RegValueT<Ptptovcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptptovcu_SPEC;
impl crate::sealed::RegSpec for Ptptovcu_SPEC {
    type DataType = u32;
}

#[doc = "Timer %s Offset Value Configuration Register U"]
pub type Ptptovcu = crate::RegValueT<Ptptovcu_SPEC>;

impl Ptptovcu {
    #[doc = "Timer Offset Value Upper Part"]
    #[inline(always)]
    pub fn tovu(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ptptovcu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ptptovcu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptptovcu {
    #[inline(always)]
    fn default() -> Ptptovcu {
        <crate::RegValueT<Ptptovcu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpavtptml_SPEC;
impl crate::sealed::RegSpec for Ptpavtptml_SPEC {
    type DataType = u32;
}

#[doc = "AVTP Timer %s Monitoring Register L"]
pub type Ptpavtptml = crate::RegValueT<Ptpavtptml_SPEC>;

impl Ptpavtptml {
    #[doc = "AVTP Timer Value Lower Part"]
    #[inline(always)]
    pub fn avtpl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Ptpavtptml_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ptpavtptml_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpavtptml {
    #[inline(always)]
    fn default() -> Ptpavtptml {
        <crate::RegValueT<Ptpavtptml_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpavtptmu_SPEC;
impl crate::sealed::RegSpec for Ptpavtptmu_SPEC {
    type DataType = u32;
}

#[doc = "AVTP Timer %s Monitoring Register U"]
pub type Ptpavtptmu = crate::RegValueT<Ptpavtptmu_SPEC>;

impl Ptpavtptmu {
    #[doc = "AVTP Timer Value Upper Part"]
    #[inline(always)]
    pub fn avtpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Ptpavtptmu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ptpavtptmu_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpavtptmu {
    #[inline(always)]
    fn default() -> Ptpavtptmu {
        <crate::RegValueT<Ptpavtptmu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpgptptml_SPEC;
impl crate::sealed::RegSpec for Ptpgptptml_SPEC {
    type DataType = u32;
}

#[doc = "GPTP Timer %s Monitoring Register L"]
pub type Ptpgptptml = crate::RegValueT<Ptpgptptml_SPEC>;

impl Ptpgptptml {
    #[doc = "GPTP Timer Value Lower Part"]
    #[inline(always)]
    pub fn gptpl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fffffff,
        1,
        0,
        u32,
        u32,
        Ptpgptptml_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3fffffff,
            1,
            0,
            u32,
            u32,
            Ptpgptptml_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpgptptml {
    #[inline(always)]
    fn default() -> Ptpgptptml {
        <crate::RegValueT<Ptpgptptml_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpgptptmm_SPEC;
impl crate::sealed::RegSpec for Ptpgptptmm_SPEC {
    type DataType = u32;
}

#[doc = "GPTP Timer %s Monitoring Register M"]
pub type Ptpgptptmm = crate::RegValueT<Ptpgptptmm_SPEC>;

impl Ptpgptptmm {
    #[doc = "GPTP Timer Value Middle Part"]
    #[inline(always)]
    pub fn gptpm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Ptpgptptmm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ptpgptptmm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpgptptmm {
    #[inline(always)]
    fn default() -> Ptpgptptmm {
        <crate::RegValueT<Ptpgptptmm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpgptptmu_SPEC;
impl crate::sealed::RegSpec for Ptpgptptmu_SPEC {
    type DataType = u32;
}

#[doc = "GPTP Timer %s Monitoring Register U"]
pub type Ptpgptptmu = crate::RegValueT<Ptpgptptmu_SPEC>;

impl Ptpgptptmu {
    #[doc = "GPTP Timer Value Upper Part"]
    #[inline(always)]
    pub fn gptpu(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ptpgptptmu_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ptpgptptmu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpgptptmu {
    #[inline(always)]
    fn default() -> Ptpgptptmu {
        <crate::RegValueT<Ptpgptptmu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmccc_SPEC;
impl crate::sealed::RegSpec for Ptpmccc_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Capture Configuration Register"]
pub type Ptpmccc = crate::RegValueT<Ptpmccc_SPEC>;

impl Ptpmccc {
    #[doc = "Media Clock Capture Positive Edge Enable"]
    #[inline(always)]
    pub fn mcpee(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ptpmccc::Mcpee,
        ptpmccc::Mcpee,
        Ptpmccc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ptpmccc::Mcpee,
            ptpmccc::Mcpee,
            Ptpmccc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock Capture Negative Edge Enable"]
    #[inline(always)]
    pub fn mcnee(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ptpmccc::Mcnee,
        ptpmccc::Mcnee,
        Ptpmccc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ptpmccc::Mcnee,
            ptpmccc::Mcnee,
            Ptpmccc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock Capture Timer Type Select"]
    #[inline(always)]
    pub fn mctts(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ptpmccc::Mctts,
        ptpmccc::Mctts,
        Ptpmccc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ptpmccc::Mctts,
            ptpmccc::Mctts,
            Ptpmccc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock Capture Timer Number Select"]
    #[inline(always)]
    pub fn mctns(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ptpmccc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ptpmccc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Media Clock Capture Request"]
    #[inline(always)]
    pub fn mccr(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ptpmccc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ptpmccc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ptpmccc {
    #[inline(always)]
    fn default() -> Ptpmccc {
        <crate::RegValueT<Ptpmccc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptpmccc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcpee_SPEC;
    pub type Mcpee = crate::EnumBitfieldStruct<u8, Mcpee_SPEC>;
    impl Mcpee {
        #[doc = "Timer value is not captured on positive edge of MEDIA_IN\\[m\\] signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer value is captured on positive edge of MEDIA_IN\\[m\\] signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcnee_SPEC;
    pub type Mcnee = crate::EnumBitfieldStruct<u8, Mcnee_SPEC>;
    impl Mcnee {
        #[doc = "Timer value is not captured on negative edge of MEDIA_IN\\[m\\] signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer value is captured on negative edge of MEDIA_IN\\[m\\] signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mctts_SPEC;
    pub type Mctts = crate::EnumBitfieldStruct<u8, Mctts_SPEC>;
    impl Mctts {
        #[doc = "Timer value is captured on GPTP timer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer value is captured on AVTP timer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmccml_SPEC;
impl crate::sealed::RegSpec for Ptpmccml_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Capture Monitoring Register L"]
pub type Ptpmccml = crate::RegValueT<Ptpmccml_SPEC>;

impl Ptpmccml {
    #[doc = "Media Clock Captured Timer Value Lower Part"]
    #[inline(always)]
    pub fn mcctvl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ptpmccml_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ptpmccml_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpmccml {
    #[inline(always)]
    fn default() -> Ptpmccml {
        <crate::RegValueT<Ptpmccml_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmccmm_SPEC;
impl crate::sealed::RegSpec for Ptpmccmm_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Capture Monitoring Register M"]
pub type Ptpmccmm = crate::RegValueT<Ptpmccmm_SPEC>;

impl Ptpmccmm {
    #[doc = "Media Clock Captured Timer Value Middle Part"]
    #[inline(always)]
    pub fn mcctvm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ptpmccmm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ptpmccmm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpmccmm {
    #[inline(always)]
    fn default() -> Ptpmccmm {
        <crate::RegValueT<Ptpmccmm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmccmu_SPEC;
impl crate::sealed::RegSpec for Ptpmccmu_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Capture Monitoring Register U"]
pub type Ptpmccmu = crate::RegValueT<Ptpmccmu_SPEC>;

impl Ptpmccmu {
    #[doc = "Media Clock Captured Timer Value Upper Part"]
    #[inline(always)]
    pub fn mcctvu(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ptpmccmu_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ptpmccmu_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Media Clock Positive Edge Captured"]
    #[inline(always)]
    pub fn mcpec(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ptpmccmu_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ptpmccmu_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Media Clock Negative Edge Captured"]
    #[inline(always)]
    pub fn mcnec(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ptpmccmu_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ptpmccmu_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Media Clock Software Captured"]
    #[inline(always)]
    pub fn mcswc(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ptpmccmu_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ptpmccmu_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Media Clock Capture Number"]
    #[inline(always)]
    pub fn mccn(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, Ptpmccmu_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,Ptpmccmu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpmccmu {
    #[inline(always)]
    fn default() -> Ptpmccmu {
        <crate::RegValueT<Ptpmccmu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmcrc_SPEC;
impl crate::sealed::RegSpec for Ptpmcrc_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Recovery Configuration Register"]
pub type Ptpmcrc = crate::RegValueT<Ptpmcrc_SPEC>;

impl Ptpmcrc {
    #[doc = "Media Clock Recovery Timer Type Select"]
    #[inline(always)]
    pub fn mrtts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ptpmcrc::Mrtts,
        ptpmcrc::Mrtts,
        Ptpmcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ptpmcrc::Mrtts,
            ptpmcrc::Mrtts,
            Ptpmcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock Recovery AVTP Mode Select"]
    #[inline(always)]
    pub fn mrams(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ptpmcrc::Mrams,
        ptpmcrc::Mrams,
        Ptpmcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ptpmcrc::Mrams,
            ptpmcrc::Mrams,
            Ptpmcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock Recovery Timer Number Select"]
    #[inline(always)]
    pub fn mrtns(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ptpmcrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ptpmcrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Media Clock Recovery Pulse Length"]
    #[inline(always)]
    pub fn mrpl(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ptpmcrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ptpmcrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpmcrc {
    #[inline(always)]
    fn default() -> Ptpmcrc {
        <crate::RegValueT<Ptpmcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptpmcrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrtts_SPEC;
    pub type Mrtts = crate::EnumBitfieldStruct<u8, Mrtts_SPEC>;
    impl Mrtts {
        #[doc = "Timer value is compared to GPTP timer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timer value is compared to AVTP timer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrams_SPEC;
    pub type Mrams = crate::EnumBitfieldStruct<u8, Mrams_SPEC>;
    impl Mrams {
        #[doc = "64-bit AVTP is used for comparison"]
        pub const _0: Self = Self::new(0);

        #[doc = "32-bit AVTP is used for comparison"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmcrtcl_SPEC;
impl crate::sealed::RegSpec for Ptpmcrtcl_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Recovery Time Configuration Register L"]
pub type Ptpmcrtcl = crate::RegValueT<Ptpmcrtcl_SPEC>;

impl Ptpmcrtcl {
    #[doc = "Media Clock Recovery Timer Value Lower Part"]
    #[inline(always)]
    pub fn mrtvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Ptpmcrtcl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ptpmcrtcl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpmcrtcl {
    #[inline(always)]
    fn default() -> Ptpmcrtcl {
        <crate::RegValueT<Ptpmcrtcl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmcrtcm_SPEC;
impl crate::sealed::RegSpec for Ptpmcrtcm_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Recovery Time Configuration Register M"]
pub type Ptpmcrtcm = crate::RegValueT<Ptpmcrtcm_SPEC>;

impl Ptpmcrtcm {
    #[doc = "Media Clock Recovery Timer Value Middle Part"]
    #[inline(always)]
    pub fn mrtvm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Ptpmcrtcm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ptpmcrtcm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpmcrtcm {
    #[inline(always)]
    fn default() -> Ptpmcrtcm {
        <crate::RegValueT<Ptpmcrtcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmcrtcu_SPEC;
impl crate::sealed::RegSpec for Ptpmcrtcu_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Recovery Time Configuration Register U"]
pub type Ptpmcrtcu = crate::RegValueT<Ptpmcrtcu_SPEC>;

impl Ptpmcrtcu {
    #[doc = "Media Clock Recovery Timer Value Upper Part"]
    #[inline(always)]
    pub fn mrtvu(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ptpmcrtcu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ptpmcrtcu_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Media Clock Recovery Trigger Type"]
    #[inline(always)]
    pub fn mrtt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        ptpmcrtcu::Mrtt,
        ptpmcrtcu::Mrtt,
        Ptpmcrtcu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            ptpmcrtcu::Mrtt,
            ptpmcrtcu::Mrtt,
            Ptpmcrtcu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock Recovery Number"]
    #[inline(always)]
    pub fn mcrn(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, u8, Ptpmcrtcu_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x7,1,0,u8,u8,Ptpmcrtcu_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Media Clock Recovery Buffer Clear Request"]
    #[inline(always)]
    pub fn mrbcr(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ptpmcrtcu_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Ptpmcrtcu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpmcrtcu {
    #[inline(always)]
    fn default() -> Ptpmcrtcu {
        <crate::RegValueT<Ptpmcrtcu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptpmcrtcu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrtt_SPEC;
    pub type Mrtt = crate::EnumBitfieldStruct<u8, Mrtt_SPEC>;
    impl Mrtt {
        #[doc = "When timer match, time recovery pin is set to 1"]
        pub const _00: Self = Self::new(0);

        #[doc = "When timer match, time recovery pin is set to 0"]
        pub const _01: Self = Self::new(1);

        #[doc = "When timer match, time recovery pin is inverted"]
        pub const _10: Self = Self::new(2);

        #[doc = "When timer match, a positive pulse of length PTPMCRCm.MRPL is output on time recovery pin"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpmcpc_SPEC;
impl crate::sealed::RegSpec for Ptpmcpc_SPEC {
    type DataType = u32;
}

#[doc = "Media Clock %s Pin Configuration Register"]
pub type Ptpmcpc = crate::RegValueT<Ptpmcpc_SPEC>;

impl Ptpmcpc {
    #[doc = "Pin Enable"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ptpmcpc::Pe,
        ptpmcpc::Pe,
        Ptpmcpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ptpmcpc::Pe,
            ptpmcpc::Pe,
            Ptpmcpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock Recovery Select"]
    #[inline(always)]
    pub fn mrs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ptpmcpc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ptpmcpc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ptpmcpc {
    #[inline(always)]
    fn default() -> Ptpmcpc {
        <crate::RegValueT<Ptpmcpc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptpmcpc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "MEDIA_OUT\\[p\\] fixed to 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "MEDIA_OUT\\[p\\] outputs the comparison result of PTPMCPCm.MRS Recovery logic"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpccc0_SPEC;
impl crate::sealed::RegSpec for Ptpccc0_SPEC {
    type DataType = u32;
}

#[doc = "Cyclic Compare %s Configuration Register 0 (c = 0 to 7)"]
pub type Ptpccc0 = crate::RegValueT<Ptpccc0_SPEC>;

impl Ptpccc0 {
    #[doc = "Cyclic Compare Timer Number Select"]
    #[inline(always)]
    pub fn cctns(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ptpccc0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ptpccc0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Cyclic Compare Output Pin Select"]
    #[inline(always)]
    pub fn ccops(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ptpccc0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ptpccc0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ptpccc0 {
    #[inline(always)]
    fn default() -> Ptpccc0 {
        <crate::RegValueT<Ptpccc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpccc1_SPEC;
impl crate::sealed::RegSpec for Ptpccc1_SPEC {
    type DataType = u32;
}

#[doc = "Cyclic Compare %s Configuration Register 1 (c = 0 to 7)"]
pub type Ptpccc1 = crate::RegValueT<Ptpccc1_SPEC>;

impl Ptpccc1 {
    #[doc = "Cycle Compare Value"]
    #[inline(always)]
    pub fn ccv(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ptpccc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ptpccc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpccc1 {
    #[inline(always)]
    fn default() -> Ptpccc1 {
        <crate::RegValueT<Ptpccc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpis0_SPEC;
impl crate::sealed::RegSpec for Ptpis0_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Status Register 0"]
pub type Ptpis0 = crate::RegValueT<Ptpis0_SPEC>;

impl Ptpis0 {
    #[doc = "Media Clock n Capture Status Flag (n = 0 to 1)"]
    #[inline(always)]
    pub fn mccs1_to_mccs0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Ptpis0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Ptpis0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Media Clock n Capture Overflow Error Status Flag (n = 0 to 1)"]
    #[inline(always)]
    pub fn mccoes1_to_mccoes0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Ptpis0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Ptpis0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpis0 {
    #[inline(always)]
    fn default() -> Ptpis0 {
        <crate::RegValueT<Ptpis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpie0_SPEC;
impl crate::sealed::RegSpec for Ptpie0_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Enable Register 0"]
pub type Ptpie0 = crate::RegValueT<Ptpie0_SPEC>;

impl Ptpie0 {
    #[doc = "Media Clock m Capture Enable (m = 0 to 1)"]
    #[inline(always)]
    pub fn mcce1_to_mcce0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ptpie0::Mcce1ToMcce0,
        ptpie0::Mcce1ToMcce0,
        Ptpie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ptpie0::Mcce1ToMcce0,
            ptpie0::Mcce1ToMcce0,
            Ptpie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Media Clock m Capture Overflow Error Enable (m = 0 to 1)"]
    #[inline(always)]
    pub fn mccoee1_to_mccoee0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        ptpie0::Mccoee1ToMccoee0,
        ptpie0::Mccoee1ToMccoee0,
        Ptpie0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            ptpie0::Mccoee1ToMccoee0,
            ptpie0::Mccoee1ToMccoee0,
            Ptpie0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpie0 {
    #[inline(always)]
    fn default() -> Ptpie0 {
        <crate::RegValueT<Ptpie0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptpie0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcce1ToMcce0_SPEC;
    pub type Mcce1ToMcce0 = crate::EnumBitfieldStruct<u8, Mcce1ToMcce0_SPEC>;
    impl Mcce1ToMcce0 {
        #[doc = "Interrupt m disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt m enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mccoee1ToMccoee0_SPEC;
    pub type Mccoee1ToMccoee0 = crate::EnumBitfieldStruct<u8, Mccoee1ToMccoee0_SPEC>;
    impl Mccoee1ToMccoee0 {
        #[doc = "Interrupt m disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt m enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpid0_SPEC;
impl crate::sealed::RegSpec for Ptpid0_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Disable Register 0"]
pub type Ptpid0 = crate::RegValueT<Ptpid0_SPEC>;

impl Ptpid0 {
    #[doc = "Media Clock m Capture Disable (m = 0 to 1)"]
    #[inline(always)]
    pub fn mccd1_to_mccd0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Ptpid0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Ptpid0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Media Clock m Capture Overflow Error Disable (m = 0 to 1)"]
    #[inline(always)]
    pub fn mccoed1_to_mccoed0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Ptpid0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Ptpid0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpid0 {
    #[inline(always)]
    fn default() -> Ptpid0 {
        <crate::RegValueT<Ptpid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpis1_SPEC;
impl crate::sealed::RegSpec for Ptpis1_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Status Register 1"]
pub type Ptpis1 = crate::RegValueT<Ptpis1_SPEC>;

impl Ptpis1 {
    #[doc = "Media Clock m Recovery Match Status Flag (m = 0 to 1)"]
    #[inline(always)]
    pub fn mcrms1_to_mcrms0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Ptpis1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Ptpis1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpis1 {
    #[inline(always)]
    fn default() -> Ptpis1 {
        <crate::RegValueT<Ptpis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpie1_SPEC;
impl crate::sealed::RegSpec for Ptpie1_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Enable Register 1"]
pub type Ptpie1 = crate::RegValueT<Ptpie1_SPEC>;

impl Ptpie1 {
    #[doc = "Media Clock m Recovery Match Enable (m = 0 to 1)"]
    #[inline(always)]
    pub fn mcrme1_to_mcrme0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ptpie1::Mcrme1ToMcrme0,
        ptpie1::Mcrme1ToMcrme0,
        Ptpie1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ptpie1::Mcrme1ToMcrme0,
            ptpie1::Mcrme1ToMcrme0,
            Ptpie1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptpie1 {
    #[inline(always)]
    fn default() -> Ptpie1 {
        <crate::RegValueT<Ptpie1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptpie1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcrme1ToMcrme0_SPEC;
    pub type Mcrme1ToMcrme0 = crate::EnumBitfieldStruct<u8, Mcrme1ToMcrme0_SPEC>;
    impl Mcrme1ToMcrme0 {
        #[doc = "Interrupt m disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt m enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpid1_SPEC;
impl crate::sealed::RegSpec for Ptpid1_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Disable Register 1"]
pub type Ptpid1 = crate::RegValueT<Ptpid1_SPEC>;

impl Ptpid1 {
    #[doc = "Media Clock m Recovery Match Disable (m = 0 to 1)"]
    #[inline(always)]
    pub fn mcrmd1_to_mcrmd0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Ptpid1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Ptpid1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ptpid1 {
    #[inline(always)]
    fn default() -> Ptpid1 {
        <crate::RegValueT<Ptpid1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potcfgr_SPEC;
impl crate::sealed::RegSpec for Potcfgr_SPEC {
    type DataType = u32;
}

#[doc = "Pulse Output Timer Configuration Register"]
pub type Potcfgr = crate::RegValueT<Potcfgr_SPEC>;

impl Potcfgr {
    #[doc = "Reference Timer Select"]
    #[inline(always)]
    pub fn refsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        potcfgr::Refsel,
        potcfgr::Refsel,
        Potcfgr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            potcfgr::Refsel,
            potcfgr::Refsel,
            Potcfgr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Potcfgr {
    #[inline(always)]
    fn default() -> Potcfgr {
        <crate::RegValueT<Potcfgr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod potcfgr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Refsel_SPEC;
    pub type Refsel = crate::EnumBitfieldStruct<u8, Refsel_SPEC>;
    impl Refsel {
        #[doc = "Select and use the reference time of GPTP timer 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select and use the reference time of GPTP timer 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potcprl_SPEC;
impl crate::sealed::RegSpec for Potcprl_SPEC {
    type DataType = u32;
}

#[doc = "Time Capture Register nL"]
pub type Potcprl = crate::RegValueT<Potcprl_SPEC>;

impl NoBitfieldReg<Potcprl_SPEC> for Potcprl {}
impl ::core::default::Default for Potcprl {
    #[inline(always)]
    fn default() -> Potcprl {
        <crate::RegValueT<Potcprl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potcprm_SPEC;
impl crate::sealed::RegSpec for Potcprm_SPEC {
    type DataType = u32;
}

#[doc = "Time Capture Register nM"]
pub type Potcprm = crate::RegValueT<Potcprm_SPEC>;

impl NoBitfieldReg<Potcprm_SPEC> for Potcprm {}
impl ::core::default::Default for Potcprm {
    #[inline(always)]
    fn default() -> Potcprm {
        <crate::RegValueT<Potcprm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potcpru_SPEC;
impl crate::sealed::RegSpec for Potcpru_SPEC {
    type DataType = u32;
}

#[doc = "Time Capture Register nU"]
pub type Potcpru = crate::RegValueT<Potcpru_SPEC>;

impl NoBitfieldReg<Potcpru_SPEC> for Potcpru {}
impl ::core::default::Default for Potcpru {
    #[inline(always)]
    fn default() -> Potcpru {
        <crate::RegValueT<Potcpru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potcr_SPEC;
impl crate::sealed::RegSpec for Potcr_SPEC {
    type DataType = u32;
}

#[doc = "Pulse Output Timer Control Register %s"]
pub type Potcr = crate::RegValueT<Potcr_SPEC>;

impl Potcr {
    #[doc = "Pulse Output Enable"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        potcr::Start,
        potcr::Start,
        Potcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            potcr::Start,
            potcr::Start,
            Potcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Potcr {
    #[inline(always)]
    fn default() -> Potcr {
        <crate::RegValueT<Potcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod potcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Pulse output stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output permission"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potperl_SPEC;
impl crate::sealed::RegSpec for Potperl_SPEC {
    type DataType = u32;
}

#[doc = "Period Setting Register nL"]
pub type Potperl = crate::RegValueT<Potperl_SPEC>;

impl NoBitfieldReg<Potperl_SPEC> for Potperl {}
impl ::core::default::Default for Potperl {
    #[inline(always)]
    fn default() -> Potperl {
        <crate::RegValueT<Potperl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potperm_SPEC;
impl crate::sealed::RegSpec for Potperm_SPEC {
    type DataType = u32;
}

#[doc = "Period Setting Register nM"]
pub type Potperm = crate::RegValueT<Potperm_SPEC>;

impl NoBitfieldReg<Potperm_SPEC> for Potperm {}
impl ::core::default::Default for Potperm {
    #[inline(always)]
    fn default() -> Potperm {
        <crate::RegValueT<Potperm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potpwr_SPEC;
impl crate::sealed::RegSpec for Potpwr_SPEC {
    type DataType = u32;
}

#[doc = "Pulse Width Setting Register %s"]
pub type Potpwr = crate::RegValueT<Potpwr_SPEC>;

impl NoBitfieldReg<Potpwr_SPEC> for Potpwr {}
impl ::core::default::Default for Potpwr {
    #[inline(always)]
    fn default() -> Potpwr {
        <crate::RegValueT<Potpwr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potstrl_SPEC;
impl crate::sealed::RegSpec for Potstrl_SPEC {
    type DataType = u32;
}

#[doc = "Pulse Output Start Time Setting Register nL"]
pub type Potstrl = crate::RegValueT<Potstrl_SPEC>;

impl NoBitfieldReg<Potstrl_SPEC> for Potstrl {}
impl ::core::default::Default for Potstrl {
    #[inline(always)]
    fn default() -> Potstrl {
        <crate::RegValueT<Potstrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potstrm_SPEC;
impl crate::sealed::RegSpec for Potstrm_SPEC {
    type DataType = u32;
}

#[doc = "Pulse Output Start Time Setting Register nM"]
pub type Potstrm = crate::RegValueT<Potstrm_SPEC>;

impl NoBitfieldReg<Potstrm_SPEC> for Potstrm {}
impl ::core::default::Default for Potstrm {
    #[inline(always)]
    fn default() -> Potstrm {
        <crate::RegValueT<Potstrm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potperu_SPEC;
impl crate::sealed::RegSpec for Potperu_SPEC {
    type DataType = u32;
}

#[doc = "Period Setting Register nU"]
pub type Potperu = crate::RegValueT<Potperu_SPEC>;

impl NoBitfieldReg<Potperu_SPEC> for Potperu {}
impl ::core::default::Default for Potperu {
    #[inline(always)]
    fn default() -> Potperu {
        <crate::RegValueT<Potperu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Potstru_SPEC;
impl crate::sealed::RegSpec for Potstru_SPEC {
    type DataType = u32;
}

#[doc = "Pulse Output Start Time Setting Register nU"]
pub type Potstru = crate::RegValueT<Potstru_SPEC>;

impl NoBitfieldReg<Potstru_SPEC> for Potstru {}
impl ::core::default::Default for Potstru {
    #[inline(always)]
    fn default() -> Potstru {
        <crate::RegValueT<Potstru_SPEC> as RegisterValue<_>>::new(0)
    }
}
