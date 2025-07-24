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
#[doc = r"GLCDC_NS"]
unsafe impl ::core::marker::Send for super::GlcdcNs {}
unsafe impl ::core::marker::Sync for super::GlcdcNs {}
impl super::GlcdcNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Background Plane Setting Operation Control Register"]
    #[inline(always)]
    pub const fn bg_en(&self) -> &'static crate::common::Reg<self::BgEn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgEn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

    #[doc = "Background Plane Setting Free-Running Period Register"]
    #[inline(always)]
    pub const fn bg_peri(
        &self,
    ) -> &'static crate::common::Reg<self::BgPeri_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgPeri_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4100usize),
            )
        }
    }

    #[doc = "Background Plane Setting Synchronization Position Register"]
    #[inline(always)]
    pub const fn bg_sync(
        &self,
    ) -> &'static crate::common::Reg<self::BgSync_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgSync_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4104usize),
            )
        }
    }

    #[doc = "Background Plane Setting Full Image Vertical Size Register"]
    #[inline(always)]
    pub const fn bg_vsize(
        &self,
    ) -> &'static crate::common::Reg<self::BgVsize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgVsize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4108usize),
            )
        }
    }

    #[doc = "Background Plane Setting Full Image Horizontal Size Register"]
    #[inline(always)]
    pub const fn bg_hsize(
        &self,
    ) -> &'static crate::common::Reg<self::BgHsize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgHsize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4112usize),
            )
        }
    }

    #[doc = "Background Plane Setting Background Color Register"]
    #[inline(always)]
    pub const fn bg_bgc(&self) -> &'static crate::common::Reg<self::BgBgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgBgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

    #[doc = "Background Plane Setting Status Monitor Register"]
    #[inline(always)]
    pub const fn bg_mon(&self) -> &'static crate::common::Reg<self::BgMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::BgMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4120usize),
            )
        }
    }

    #[doc = "Graphics %s Register Update Control Register"]
    #[inline(always)]
    pub const fn gr_ven(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrVen_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1100usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ven(
        &self,
    ) -> &'static crate::common::Reg<self::GrVen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrVen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ven(
        &self,
    ) -> &'static crate::common::Reg<self::GrVen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrVen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1200usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Read Control Register"]
    #[inline(always)]
    pub const fn gr_flmrd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlmrd_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1104usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flmrd(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlmrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlmrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flmrd(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlmrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlmrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1204usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 1"]
    #[inline(always)]
    pub const fn gr_flm1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm1_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1108usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm1(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm1(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1208usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 2"]
    #[inline(always)]
    pub const fn gr_flm2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm2_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x110cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm2(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm2(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120cusize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 3"]
    #[inline(always)]
    pub const fn gr_flm3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm3_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1110usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm3(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm3(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1210usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 5"]
    #[inline(always)]
    pub const fn gr_flm5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm5_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1118usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm5(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm5(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1218usize),
            )
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 6"]
    #[inline(always)]
    pub const fn gr_flm6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm6_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x111cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_flm6(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x111cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_flm6(
        &self,
    ) -> &'static crate::common::Reg<self::GrFlm6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrFlm6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x121cusize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 1"]
    #[inline(always)]
    pub const fn gr_ab1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb1_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1120usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab1(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab1(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1220usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 2"]
    #[inline(always)]
    pub const fn gr_ab2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb2_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1124usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab2(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab2(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1224usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 3"]
    #[inline(always)]
    pub const fn gr_ab3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb3_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1128usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab3(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab3(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1228usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 4"]
    #[inline(always)]
    pub const fn gr_ab4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb4_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x112cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab4(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x112cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab4(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x122cusize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 5"]
    #[inline(always)]
    pub const fn gr_ab5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb5_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1130usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab5(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab5(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1230usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 6"]
    #[inline(always)]
    pub const fn gr_ab6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb6_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1134usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab6(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab6(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1234usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 7"]
    #[inline(always)]
    pub const fn gr_ab7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb7_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1138usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab7(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab7(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1238usize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 8"]
    #[inline(always)]
    pub const fn gr_ab8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb8_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x113cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab8(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x113cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab8(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x123cusize),
            )
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 9"]
    #[inline(always)]
    pub const fn gr_ab9(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb9_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1140usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_ab9(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_ab9(
        &self,
    ) -> &'static crate::common::Reg<self::GrAb9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrAb9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1240usize),
            )
        }
    }

    #[doc = "Graphics %s Background Color Control Register"]
    #[inline(always)]
    pub const fn gr_base(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrBase_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x114cusize))
        }
    }
    #[inline(always)]
    pub const fn gr1_base(
        &self,
    ) -> &'static crate::common::Reg<self::GrBase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrBase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_base(
        &self,
    ) -> &'static crate::common::Reg<self::GrBase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrBase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124cusize),
            )
        }
    }

    #[doc = "Graphics %s CLUT Table Interrupt Control Register"]
    #[inline(always)]
    pub const fn gr_clutint(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrClutint_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1150usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_clutint(
        &self,
    ) -> &'static crate::common::Reg<self::GrClutint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrClutint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_clutint(
        &self,
    ) -> &'static crate::common::Reg<self::GrClutint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GrClutint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1250usize),
            )
        }
    }

    #[doc = "Graphics %s Status Monitor Register"]
    #[inline(always)]
    pub const fn gr_mon(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrMon_SPEC, crate::common::R>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1154usize))
        }
    }
    #[inline(always)]
    pub const fn gr1_mon(&self) -> &'static crate::common::Reg<self::GrMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn gr2_mon(&self) -> &'static crate::common::Reg<self::GrMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1254usize),
            )
        }
    }

    #[doc = "Gamma G Register Update Control Register"]
    #[inline(always)]
    pub const fn gamg_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4864usize),
            )
        }
    }

    #[doc = "Gamma Correction Block Function Switch Register"]
    #[inline(always)]
    pub const fn gam_sw(&self) -> &'static crate::common::Reg<self::GamSw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamSw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4868usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gamg_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4872usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gamg_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4876usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gamg_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4880usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gamg_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4884usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gamg_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4888usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gamg_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4892usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gamg_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4896usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gamg_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GamgLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4900usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gamg_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4904usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gamg_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4908usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gamg_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4912usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gamg_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4916usize),
            )
        }
    }

    #[doc = "Gamma G Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gamg_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GamgArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamgArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4920usize),
            )
        }
    }

    #[doc = "Gamma B Register Update Control Register"]
    #[inline(always)]
    pub const fn gamb_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GambLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4928usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gamb_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4936usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gamb_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4940usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gamb_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4944usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gamb_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4948usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gamb_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4952usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gamb_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4956usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gamb_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4960usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gamb_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GambLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4964usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gamb_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4968usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gamb_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4972usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gamb_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4976usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gamb_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4980usize),
            )
        }
    }

    #[doc = "Gamma B Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gamb_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GambArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GambArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4984usize),
            )
        }
    }

    #[doc = "Gamma R Register Update Control Register"]
    #[inline(always)]
    pub const fn gamr_latch(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4992usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gamr_lut1(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5000usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gamr_lut2(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5004usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gamr_lut3(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5008usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gamr_lut4(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5012usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gamr_lut5(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5016usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gamr_lut6(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5020usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gamr_lut7(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5024usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gamr_lut8(
        &self,
    ) -> &'static crate::common::Reg<self::GamrLut8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrLut8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5028usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gamr_area1(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5032usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gamr_area2(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5036usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gamr_area3(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5040usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gamr_area4(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5044usize),
            )
        }
    }

    #[doc = "Gamma R Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gamr_area5(
        &self,
    ) -> &'static crate::common::Reg<self::GamrArea5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamrArea5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5048usize),
            )
        }
    }

    #[doc = "Output Control Block Register Update Control Register"]
    #[inline(always)]
    pub const fn out_vlatch(
        &self,
    ) -> &'static crate::common::Reg<self::OutVlatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutVlatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5056usize),
            )
        }
    }

    #[doc = "Output Control Block Output Interface Register"]
    #[inline(always)]
    pub const fn out_set(
        &self,
    ) -> &'static crate::common::Reg<self::OutSet_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutSet_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5060usize),
            )
        }
    }

    #[doc = "Output Control Block Brightness Correction Register 1"]
    #[inline(always)]
    pub const fn out_bright1(
        &self,
    ) -> &'static crate::common::Reg<self::OutBright1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutBright1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5064usize),
            )
        }
    }

    #[doc = "Output Control Block Brightness Correction Register 2"]
    #[inline(always)]
    pub const fn out_bright2(
        &self,
    ) -> &'static crate::common::Reg<self::OutBright2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutBright2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5068usize),
            )
        }
    }

    #[doc = "Output Control Block Contrast Correction Register"]
    #[inline(always)]
    pub const fn out_contrast(
        &self,
    ) -> &'static crate::common::Reg<self::OutContrast_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutContrast_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5072usize),
            )
        }
    }

    #[doc = "Output Control Block Panel Dither Correction Register"]
    #[inline(always)]
    pub const fn out_pdtha(
        &self,
    ) -> &'static crate::common::Reg<self::OutPdtha_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutPdtha_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5076usize),
            )
        }
    }

    #[doc = "Output Control Block Output Phase Control Register"]
    #[inline(always)]
    pub const fn out_clkphase(
        &self,
    ) -> &'static crate::common::Reg<self::OutClkphase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutClkphase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5092usize),
            )
        }
    }

    #[doc = "TCON Reference Timing Setting Register"]
    #[inline(always)]
    pub const fn tcon_tim(
        &self,
    ) -> &'static crate::common::Reg<self::TconTim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconTim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5124usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_stva1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStva1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStva1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5128usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_stva2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStva2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStva2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5132usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_stvb1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStvb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStvb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5136usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_stvb2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStvb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStvb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5140usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_stha1(
        &self,
    ) -> &'static crate::common::Reg<self::TconStha1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStha1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5144usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_stha2(
        &self,
    ) -> &'static crate::common::Reg<self::TconStha2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconStha2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5148usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x1"]
    #[inline(always)]
    pub const fn tcon_sthb1(
        &self,
    ) -> &'static crate::common::Reg<self::TconSthb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSthb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5152usize),
            )
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register x2"]
    #[inline(always)]
    pub const fn tcon_sthb2(
        &self,
    ) -> &'static crate::common::Reg<self::TconSthb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconSthb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5156usize),
            )
        }
    }

    #[doc = "TCON Data Enable Polarity Setting Register"]
    #[inline(always)]
    pub const fn tcon_de(
        &self,
    ) -> &'static crate::common::Reg<self::TconDe_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconDe_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5160usize),
            )
        }
    }

    #[doc = "System Control Block State Detection Control Register"]
    #[inline(always)]
    pub const fn syscnt_dtcten(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntDtcten_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntDtcten_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5184usize),
            )
        }
    }

    #[doc = "System Control Block Interrupt Request Enable Control Register"]
    #[inline(always)]
    pub const fn syscnt_inten(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntInten_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntInten_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5188usize),
            )
        }
    }

    #[doc = "System Control Block Status Clear Register"]
    #[inline(always)]
    pub const fn syscnt_stclr(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntStclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntStclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5192usize),
            )
        }
    }

    #[doc = "System Control Block Status Monitor Register"]
    #[inline(always)]
    pub const fn syscnt_stmon(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntStmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SyscntStmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5196usize),
            )
        }
    }

    #[doc = "System Control Block Version and Panel Clock Control Register"]
    #[inline(always)]
    pub const fn syscnt_panel_clk(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntPanelClk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntPanelClk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5200usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgEn_SPEC;
impl crate::sealed::RegSpec for BgEn_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Operation Control Register"]
pub type BgEn = crate::RegValueT<BgEn_SPEC>;

impl BgEn {
    #[doc = "Background Plane Operation Enable"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bg_en::En,
        bg_en::En,
        BgEn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bg_en::En,
            bg_en::En,
            BgEn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control of GLCDC Internal Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bg_en::Ven,
        bg_en::Ven,
        BgEn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bg_en::Ven,
            bg_en::Ven,
            BgEn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Software Reset Control"]
    #[inline(always)]
    pub fn swrst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bg_en::Swrst,
        bg_en::Swrst,
        BgEn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bg_en::Swrst,
            bg_en::Swrst,
            BgEn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BgEn {
    #[inline(always)]
    fn default() -> BgEn {
        <crate::RegValueT<BgEn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bg_en {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable background plane operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable background plane operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable GLCDC register values from being reflected in internal operations at start of screen generation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC register values to be reflected in internal operations at start of screen generation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "Place entire module in software reset state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Release entire module from software reset state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgPeri_SPEC;
impl crate::sealed::RegSpec for BgPeri_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Free-Running Period Register"]
pub type BgPeri = crate::RegValueT<BgPeri_SPEC>;

impl BgPeri {
    #[doc = "Background Plane Horizontal Synchronization Signal Period"]
    #[inline(always)]
    pub fn fh(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        bg_peri::Fh,
        bg_peri::Fh,
        BgPeri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            bg_peri::Fh,
            bg_peri::Fh,
            BgPeri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Background Plane Vertical Synchronization Signal Period"]
    #[inline(always)]
    pub fn fv(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        bg_peri::Fv,
        bg_peri::Fv,
        BgPeri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            bg_peri::Fv,
            bg_peri::Fv,
            BgPeri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BgPeri {
    #[inline(always)]
    fn default() -> BgPeri {
        <crate::RegValueT<BgPeri_SPEC> as RegisterValue<_>>::new(1507351)
    }
}
pub mod bg_peri {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fh_SPEC;
    pub type Fh = crate::EnumBitfieldStruct<u16, Fh_SPEC>;
    impl Fh {
        #[doc = "24 cycles (pixels)"]
        pub const _0_X_017: Self = Self::new(23);

        #[doc = "1024 cycles (pixels)"]
        pub const _0_X_3_FF: Self = Self::new(1023);

        #[doc = "2048 cycles (pixels)"]
        pub const _0_X_7_FF: Self = Self::new(2047);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fv_SPEC;
    pub type Fv = crate::EnumBitfieldStruct<u16, Fv_SPEC>;
    impl Fv {
        #[doc = "20 lines"]
        pub const _0_X_013: Self = Self::new(19);

        #[doc = "1024 lines"]
        pub const _0_X_3_FF: Self = Self::new(1023);

        #[doc = "2048 lines"]
        pub const _0_X_7_FF: Self = Self::new(2047);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgSync_SPEC;
impl crate::sealed::RegSpec for BgSync_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Synchronization Position Register"]
pub type BgSync = crate::RegValueT<BgSync_SPEC>;

impl BgSync {
    #[doc = "Background Plane Horizontal Synchronization Signal Assertion Position"]
    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        bg_sync::Hp,
        bg_sync::Hp,
        BgSync_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            bg_sync::Hp,
            bg_sync::Hp,
            BgSync_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Background Plane Vertical Synchronization Assertion Position"]
    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        bg_sync::Vp,
        bg_sync::Vp,
        BgSync_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            bg_sync::Vp,
            bg_sync::Vp,
            BgSync_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BgSync {
    #[inline(always)]
    fn default() -> BgSync {
        <crate::RegValueT<BgSync_SPEC> as RegisterValue<_>>::new(65537)
    }
}
pub mod bg_sync {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hp_SPEC;
    pub type Hp = crate::EnumBitfieldStruct<u8, Hp_SPEC>;
    impl Hp {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "HPth cycle (pixel)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vp_SPEC;
    pub type Vp = crate::EnumBitfieldStruct<u8, Vp_SPEC>;
    impl Vp {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "VPth line"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgVsize_SPEC;
impl crate::sealed::RegSpec for BgVsize_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Full Image Vertical Size Register"]
pub type BgVsize = crate::RegValueT<BgVsize_SPEC>;

impl BgVsize {
    #[doc = "Background Plane Vertical Valid Pixel Width"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Vertical Valid Pixel Start Position"]
    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgVsize {
    #[inline(always)]
    fn default() -> BgVsize {
        <crate::RegValueT<BgVsize_SPEC> as RegisterValue<_>>::new(458768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgHsize_SPEC;
impl crate::sealed::RegSpec for BgHsize_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Full Image Horizontal Size Register"]
pub type BgHsize = crate::RegValueT<BgHsize_SPEC>;

impl BgHsize {
    #[doc = "Background Plane Horizontal Valid Pixel Width"]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Horizontal Valid Pixel Start Position"]
    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgHsize {
    #[inline(always)]
    fn default() -> BgHsize {
        <crate::RegValueT<BgHsize_SPEC> as RegisterValue<_>>::new(393232)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgBgc_SPEC;
impl crate::sealed::RegSpec for BgBgc_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Background Color Register"]
pub type BgBgc = crate::RegValueT<BgBgc_SPEC>;

impl BgBgc {
    #[doc = "Background Plane Valid Pixel Area B Value"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Valid Pixel Area G Value"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Plane Valid Pixel Area R Value"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgBgc {
    #[inline(always)]
    fn default() -> BgBgc {
        <crate::RegValueT<BgBgc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgMon_SPEC;
impl crate::sealed::RegSpec for BgMon_SPEC {
    type DataType = u32;
}

#[doc = "Background Plane Setting Status Monitor Register"]
pub type BgMon = crate::RegValueT<BgMon_SPEC>;

impl BgMon {
    #[doc = "Background Plane Operation Monitor"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bg_mon::En,
        bg_mon::En,
        BgMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bg_mon::En,
            bg_mon::En,
            BgMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Entire Module Internal Operation Reflection Control Signal Monitor"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bg_mon::Ven,
        bg_mon::Ven,
        BgMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bg_mon::Ven,
            bg_mon::Ven,
            BgMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Entire Module SW Reset State Monitor"]
    #[inline(always)]
    pub fn swrst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bg_mon::Swrst,
        bg_mon::Swrst,
        BgMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bg_mon::Swrst,
            bg_mon::Swrst,
            BgMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BgMon {
    #[inline(always)]
    fn default() -> BgMon {
        <crate::RegValueT<BgMon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bg_mon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Operation is stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation is in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Signal for controlling reflection of the register values to internal operations on assertion of vertical synchronization signal is negated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Signal for controlling reflection of the register values to internal operations on assertion of vertical synchronization signal is asserted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "Entire module is in software reset state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Entire module is released from software reset state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrVen_SPEC;
impl crate::sealed::RegSpec for GrVen_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Register Update Control Register"]
pub type GrVen = crate::RegValueT<GrVen_SPEC>;

impl GrVen {
    #[doc = "This bit is cleared to 0 by an internal source."]
    #[inline(always)]
    pub fn pven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_ven::Pven,
        gr_ven::Pven,
        GrVen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_ven::Pven,
            gr_ven::Pven,
            GrVen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrVen {
    #[inline(always)]
    fn default() -> GrVen {
        <crate::RegValueT<GrVen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ven {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pven_SPEC;
    pub type Pven = crate::EnumBitfieldStruct<u8, Pven_SPEC>;
    impl Pven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlmrd_SPEC;
impl crate::sealed::RegSpec for GrFlmrd_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Read Control Register"]
pub type GrFlmrd = crate::RegValueT<GrFlmrd_SPEC>;

impl GrFlmrd {
    #[doc = "Graphics Data Read Enable"]
    #[inline(always)]
    pub fn renb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_flmrd::Renb,
        gr_flmrd::Renb,
        GrFlmrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_flmrd::Renb,
            gr_flmrd::Renb,
            GrFlmrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrFlmrd {
    #[inline(always)]
    fn default() -> GrFlmrd {
        <crate::RegValueT<GrFlmrd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_flmrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Renb_SPEC;
    pub type Renb = crate::EnumBitfieldStruct<u8, Renb_SPEC>;
    impl Renb {
        #[doc = "Disable reading"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reading"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm1_SPEC;
impl crate::sealed::RegSpec for GrFlm1_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 1"]
pub type GrFlm1 = crate::RegValueT<GrFlm1_SPEC>;

impl GrFlm1 {
    #[doc = "Burst Transfer Control for Graphics Data Access"]
    #[inline(always)]
    pub fn bstmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gr_flm1::Bstmd,
        gr_flm1::Bstmd,
        GrFlm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gr_flm1::Bstmd,
            gr_flm1::Bstmd,
            GrFlm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrFlm1 {
    #[inline(always)]
    fn default() -> GrFlm1 {
        <crate::RegValueT<GrFlm1_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod gr_flm1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bstmd_SPEC;
    pub type Bstmd = crate::EnumBitfieldStruct<u8, Bstmd_SPEC>;
    impl Bstmd {
        #[doc = "16-beat increment burst transfer (64-byte boundary)"]
        pub const _11: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm2_SPEC;
impl crate::sealed::RegSpec for GrFlm2_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 2"]
pub type GrFlm2 = crate::RegValueT<GrFlm2_SPEC>;

impl GrFlm2 {
    #[doc = "Base Address for Accessing Graphics Data"]
    #[inline(always)]
    pub fn base(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, GrFlm2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,GrFlm2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm2 {
    #[inline(always)]
    fn default() -> GrFlm2 {
        <crate::RegValueT<GrFlm2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm3_SPEC;
impl crate::sealed::RegSpec for GrFlm3_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 3"]
pub type GrFlm3 = crate::RegValueT<GrFlm3_SPEC>;

impl GrFlm3 {
    #[doc = "Macro Line Offset Address for Accessing Graphics Data"]
    #[inline(always)]
    pub fn lnoff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, GrFlm3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,GrFlm3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm3 {
    #[inline(always)]
    fn default() -> GrFlm3 {
        <crate::RegValueT<GrFlm3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm5_SPEC;
impl crate::sealed::RegSpec for GrFlm5_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 5"]
pub type GrFlm5 = crate::RegValueT<GrFlm5_SPEC>;

impl GrFlm5 {
    #[doc = "Number of Data Transfer Times Per Line for Accessing Graphics Data"]
    #[inline(always)]
    pub fn datanum(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, GrFlm5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,GrFlm5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Number of Lines Per Frame for Accessing Graphics Data"]
    #[inline(always)]
    pub fn lnnum(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrFlm5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrFlm5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm5 {
    #[inline(always)]
    fn default() -> GrFlm5 {
        <crate::RegValueT<GrFlm5_SPEC> as RegisterValue<_>>::new(983040)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm6_SPEC;
impl crate::sealed::RegSpec for GrFlm6_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Frame Buffer Control Register 6"]
pub type GrFlm6 = crate::RegValueT<GrFlm6_SPEC>;

impl GrFlm6 {
    #[doc = "Data Format for Accessing Graphics Data"]
    #[inline(always)]
    pub fn format(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        gr_flm6::Format,
        gr_flm6::Format,
        GrFlm6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            gr_flm6::Format,
            gr_flm6::Format,
            GrFlm6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrFlm6 {
    #[inline(always)]
    fn default() -> GrFlm6 {
        <crate::RegValueT<GrFlm6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_flm6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        #[doc = "RGB565 (16 bits/pixel)"]
        pub const _000: Self = Self::new(0);

        #[doc = "RGB888 (32 bits/pixel, 8 bits on the MSB side are invalid)"]
        pub const _001: Self = Self::new(1);

        #[doc = "ARGB1555 (16 bits/pixel, 1 bit of A is LUT data)"]
        pub const _010: Self = Self::new(2);

        #[doc = "ARGB4444 (16 bits/pixel)"]
        pub const _011: Self = Self::new(3);

        #[doc = "ARGB8888 (32 bits/pixel)"]
        pub const _100: Self = Self::new(4);

        #[doc = "CLUT8 (8 bits/pixel)"]
        pub const _101: Self = Self::new(5);

        #[doc = "CLUT4 (4 bits/pixel)"]
        pub const _110: Self = Self::new(6);

        #[doc = "CLUT1 (1 bit/pixel)"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb1_SPEC;
impl crate::sealed::RegSpec for GrAb1_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 1"]
pub type GrAb1 = crate::RegValueT<GrAb1_SPEC>;

impl GrAb1 {
    #[doc = "Graphics Display Plane Control"]
    #[inline(always)]
    pub fn dispsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gr_ab1::Dispsel,
        gr_ab1::Dispsel,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gr_ab1::Dispsel,
            gr_ab1::Dispsel,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics Image Area Border Display Control"]
    #[inline(always)]
    pub fn grcdispon(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gr_ab1::Grcdispon,
        gr_ab1::Grcdispon,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gr_ab1::Grcdispon,
            gr_ab1::Grcdispon,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Image Area Border Display Control for Rectangular Area Alpha Blending"]
    #[inline(always)]
    pub fn arcdispon(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gr_ab1::Arcdispon,
        gr_ab1::Arcdispon,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gr_ab1::Arcdispon,
            gr_ab1::Arcdispon,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Rectangular Area Alpha Blending Control"]
    #[inline(always)]
    pub fn arcon(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gr_ab1::Arcon,
        gr_ab1::Arcon,
        GrAb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gr_ab1::Arcon,
            gr_ab1::Arcon,
            GrAb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrAb1 {
    #[inline(always)]
    fn default() -> GrAb1 {
        <crate::RegValueT<GrAb1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dispsel_SPEC;
    pub type Dispsel = crate::EnumBitfieldStruct<u8, Dispsel_SPEC>;
    impl Dispsel {
        #[doc = "Background color display (value set in the GRn_BASE register)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Lower-layer graphics display"]
        pub const _01: Self = Self::new(1);

        #[doc = "Current graphics display"]
        pub const _10: Self = Self::new(2);

        #[doc = "Blended display of lower-layer graphics (input image from the previous stage) and current graphics (data read from the GLCDC0 and GLCDC1 bus)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcdispon_SPEC;
    pub type Grcdispon = crate::EnumBitfieldStruct<u8, Grcdispon_SPEC>;
    impl Grcdispon {
        #[doc = "Turn display off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn display on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcdispon_SPEC;
    pub type Arcdispon = crate::EnumBitfieldStruct<u8, Arcdispon_SPEC>;
    impl Arcdispon {
        #[doc = "Turn display off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn display on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcon_SPEC;
    pub type Arcon = crate::EnumBitfieldStruct<u8, Arcon_SPEC>;
    impl Arcon {
        #[doc = "Turn blending off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn blending on"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb2_SPEC;
impl crate::sealed::RegSpec for GrAb2_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 2"]
pub type GrAb2 = crate::RegValueT<GrAb2_SPEC>;

impl GrAb2 {
    #[doc = "Vertical Width of Graphics Image Area"]
    #[inline(always)]
    pub fn grcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Start Position of Graphics Image Area"]
    #[inline(always)]
    pub fn grcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb2 {
    #[inline(always)]
    fn default() -> GrAb2 {
        <crate::RegValueT<GrAb2_SPEC> as RegisterValue<_>>::new(393232)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb3_SPEC;
impl crate::sealed::RegSpec for GrAb3_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 3"]
pub type GrAb3 = crate::RegValueT<GrAb3_SPEC>;

impl GrAb3 {
    #[doc = "Horizontal Width of Graphics Image Area"]
    #[inline(always)]
    pub fn grchw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Start Position of Graphics Image Area"]
    #[inline(always)]
    pub fn grchs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb3 {
    #[inline(always)]
    fn default() -> GrAb3 {
        <crate::RegValueT<GrAb3_SPEC> as RegisterValue<_>>::new(327696)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb4_SPEC;
impl crate::sealed::RegSpec for GrAb4_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 4"]
pub type GrAb4 = crate::RegValueT<GrAb4_SPEC>;

impl GrAb4 {
    #[doc = "Vertical Width of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn arcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Start Position of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn arcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb4 {
    #[inline(always)]
    fn default() -> GrAb4 {
        <crate::RegValueT<GrAb4_SPEC> as RegisterValue<_>>::new(393232)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb5_SPEC;
impl crate::sealed::RegSpec for GrAb5_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 5"]
pub type GrAb5 = crate::RegValueT<GrAb5_SPEC>;

impl GrAb5 {
    #[doc = "Horizontal Width of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn archw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrAb5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Start Position of Rectangular Area Alpha Blending Image Area"]
    #[inline(always)]
    pub fn archs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GrAb5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb5 {
    #[inline(always)]
    fn default() -> GrAb5 {
        <crate::RegValueT<GrAb5_SPEC> as RegisterValue<_>>::new(327696)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb6_SPEC;
impl crate::sealed::RegSpec for GrAb6_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 6"]
pub type GrAb6 = crate::RegValueT<GrAb6_SPEC>;

impl GrAb6 {
    #[doc = "Frame Rate for Alpha Blending in Rectangular Area"]
    #[inline(always)]
    pub fn arcrate(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha Coefficient for Alpha Blending in Rectangular Area"]
    #[inline(always)]
    pub fn arccoef(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, GrAb6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb6 {
    #[inline(always)]
    fn default() -> GrAb6 {
        <crate::RegValueT<GrAb6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb7_SPEC;
impl crate::sealed::RegSpec for GrAb7_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 7"]
pub type GrAb7 = crate::RegValueT<GrAb7_SPEC>;

impl GrAb7 {
    #[doc = "RGB-Index Chroma-Key Processing Control"]
    #[inline(always)]
    pub fn ckon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_ab7::Ckon,
        gr_ab7::Ckon,
        GrAb7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_ab7::Ckon,
            gr_ab7::Ckon,
            GrAb7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Initial Alpha Value for Alpha Blending in Rectangular Area"]
    #[inline(always)]
    pub fn arcdef(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb7 {
    #[inline(always)]
    fn default() -> GrAb7 {
        <crate::RegValueT<GrAb7_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab7 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckon_SPEC;
    pub type Ckon = crate::EnumBitfieldStruct<u8, Ckon_SPEC>;
    impl Ckon {
        #[doc = "Disable chroma-key processing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable chroma-key processing"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb8_SPEC;
impl crate::sealed::RegSpec for GrAb8_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 8"]
pub type GrAb8 = crate::RegValueT<GrAb8_SPEC>;

impl GrAb8 {
    #[doc = "R Signal for RGB-Index Chroma-Key Processing"]
    #[inline(always)]
    pub fn ckkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B Signal for RGB-Index Chroma-Key Processing"]
    #[inline(always)]
    pub fn ckkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Signal for RGB-Index Chroma-Key Processing"]
    #[inline(always)]
    pub fn ckkg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb8 {
    #[inline(always)]
    fn default() -> GrAb8 {
        <crate::RegValueT<GrAb8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb9_SPEC;
impl crate::sealed::RegSpec for GrAb9_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Alpha Blending Control Register 9"]
pub type GrAb9 = crate::RegValueT<GrAb9_SPEC>;

impl GrAb9 {
    #[doc = "R Value after RGB-Index Chroma-Key Processing Replacement"]
    #[inline(always)]
    pub fn ckr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B Value after RGB-Index Chroma-Key Processing Replacement"]
    #[inline(always)]
    pub fn ckb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "G Value after RGB-Index Chroma-Key Processing Replacement"]
    #[inline(always)]
    pub fn ckg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "A Value after RGB-Index Chroma-Key Processing Replacement"]
    #[inline(always)]
    pub fn cka(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb9 {
    #[inline(always)]
    fn default() -> GrAb9 {
        <crate::RegValueT<GrAb9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrBase_SPEC;
impl crate::sealed::RegSpec for GrBase_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Background Color Control Register"]
pub type GrBase = crate::RegValueT<GrBase_SPEC>;

impl GrBase {
    #[doc = "Background Color R Value"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Color B Value"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Background Color G Value"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrBase {
    #[inline(always)]
    fn default() -> GrBase {
        <crate::RegValueT<GrBase_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrClutint_SPEC;
impl crate::sealed::RegSpec for GrClutint_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s CLUT Table Interrupt Control Register"]
pub type GrClutint = crate::RegValueT<GrClutint_SPEC>;

impl GrClutint {
    #[doc = "Number of Detection Lines"]
    #[inline(always)]
    pub fn line(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GrClutint_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GrClutint_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CLUT Table Control"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gr_clutint::Sel,
        gr_clutint::Sel,
        GrClutint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gr_clutint::Sel,
            gr_clutint::Sel,
            GrClutint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrClutint {
    #[inline(always)]
    fn default() -> GrClutint {
        <crate::RegValueT<GrClutint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_clutint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Select CLUT table 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select CLUT table 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrMon_SPEC;
impl crate::sealed::RegSpec for GrMon_SPEC {
    type DataType = u32;
}

#[doc = "Graphics %s Status Monitor Register"]
pub type GrMon = crate::RegValueT<GrMon_SPEC>;

impl GrMon {
    #[doc = "Status Monitor for Alpha Blending in Rectangular Area"]
    #[inline(always)]
    pub fn arcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gr_mon::Arcst,
        gr_mon::Arcst,
        GrMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gr_mon::Arcst,
            gr_mon::Arcst,
            GrMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Underflow Status Monitor"]
    #[inline(always)]
    pub fn undflst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gr_mon::Undflst,
        gr_mon::Undflst,
        GrMon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gr_mon::Undflst,
            gr_mon::Undflst,
            GrMon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrMon {
    #[inline(always)]
    fn default() -> GrMon {
        <crate::RegValueT<GrMon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_mon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcst_SPEC;
    pub type Arcst = crate::EnumBitfieldStruct<u8, Arcst_SPEC>;
    impl Arcst {
        #[doc = "Fade-in/fade-out not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fade-in/fade-out in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Undflst_SPEC;
    pub type Undflst = crate::EnumBitfieldStruct<u8, Undflst_SPEC>;
    impl Undflst {
        #[doc = "No underflow occurred in internal operations"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow occurred in internal operations"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLatch_SPEC;
impl crate::sealed::RegSpec for GamgLatch_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Register Update Control Register"]
pub type GamgLatch = crate::RegValueT<GamgLatch_SPEC>;

impl GamgLatch {
    #[doc = "Control of Gamma Correction × Module Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gamg_latch::Ven,
        gamg_latch::Ven,
        GamgLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gamg_latch::Ven,
            gamg_latch::Ven,
            GamgLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GamgLatch {
    #[inline(always)]
    fn default() -> GamgLatch {
        <crate::RegValueT<GamgLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gamg_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamSw_SPEC;
impl crate::sealed::RegSpec for GamSw_SPEC {
    type DataType = u32;
}

#[doc = "Gamma Correction Block Function Switch Register"]
pub type GamSw = crate::RegValueT<GamSw_SPEC>;

impl GamSw {
    #[doc = "Gamma Correction On/Off Control"]
    #[inline(always)]
    pub fn gamon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gam_sw::Gamon,
        gam_sw::Gamon,
        GamSw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gam_sw::Gamon,
            gam_sw::Gamon,
            GamSw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GamSw {
    #[inline(always)]
    fn default() -> GamSw {
        <crate::RegValueT<GamSw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_sw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gamon_SPEC;
    pub type Gamon = crate::EnumBitfieldStruct<u8, Gamon_SPEC>;
    impl Gamon {
        #[doc = "Turn off gamma correction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Turn on gamma correction"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut1_SPEC;
impl crate::sealed::RegSpec for GamgLut1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 1"]
pub type GamgLut1 = crate::RegValueT<GamgLut1_SPEC>;

impl GamgLut1 {
    #[doc = "Gain Value of Area 1"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 0"]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut1 {
    #[inline(always)]
    fn default() -> GamgLut1 {
        <crate::RegValueT<GamgLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut2_SPEC;
impl crate::sealed::RegSpec for GamgLut2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 2"]
pub type GamgLut2 = crate::RegValueT<GamgLut2_SPEC>;

impl GamgLut2 {
    #[doc = "Gain Value of Area 3"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 2"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut2 {
    #[inline(always)]
    fn default() -> GamgLut2 {
        <crate::RegValueT<GamgLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut3_SPEC;
impl crate::sealed::RegSpec for GamgLut3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 3"]
pub type GamgLut3 = crate::RegValueT<GamgLut3_SPEC>;

impl GamgLut3 {
    #[doc = "Gain Value of Area 5"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 4"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut3 {
    #[inline(always)]
    fn default() -> GamgLut3 {
        <crate::RegValueT<GamgLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut4_SPEC;
impl crate::sealed::RegSpec for GamgLut4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 4"]
pub type GamgLut4 = crate::RegValueT<GamgLut4_SPEC>;

impl GamgLut4 {
    #[doc = "Gain Value of Area 7"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 6"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut4 {
    #[inline(always)]
    fn default() -> GamgLut4 {
        <crate::RegValueT<GamgLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut5_SPEC;
impl crate::sealed::RegSpec for GamgLut5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 5"]
pub type GamgLut5 = crate::RegValueT<GamgLut5_SPEC>;

impl GamgLut5 {
    #[doc = "Gain Value of Area 9"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 8"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut5 {
    #[inline(always)]
    fn default() -> GamgLut5 {
        <crate::RegValueT<GamgLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut6_SPEC;
impl crate::sealed::RegSpec for GamgLut6_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 6"]
pub type GamgLut6 = crate::RegValueT<GamgLut6_SPEC>;

impl GamgLut6 {
    #[doc = "Gain Value of Area 11"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 10"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut6 {
    #[inline(always)]
    fn default() -> GamgLut6 {
        <crate::RegValueT<GamgLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut7_SPEC;
impl crate::sealed::RegSpec for GamgLut7_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 7"]
pub type GamgLut7 = crate::RegValueT<GamgLut7_SPEC>;

impl GamgLut7 {
    #[doc = "Gain Value of Area 13"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 12"]
    #[inline(always)]
    pub fn gain012(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut7 {
    #[inline(always)]
    fn default() -> GamgLut7 {
        <crate::RegValueT<GamgLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgLut8_SPEC;
impl crate::sealed::RegSpec for GamgLut8_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Table Setting Register 8"]
pub type GamgLut8 = crate::RegValueT<GamgLut8_SPEC>;

impl GamgLut8 {
    #[doc = "Gain Value of Area 15"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamgLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamgLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 14"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamgLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamgLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgLut8 {
    #[inline(always)]
    fn default() -> GamgLut8 {
        <crate::RegValueT<GamgLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea1_SPEC;
impl crate::sealed::RegSpec for GamgArea1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 1"]
pub type GamgArea1 = crate::RegValueT<GamgArea1_SPEC>;

impl GamgArea1 {
    #[doc = "Start Threshold of Area 3"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 2"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 1"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea1 {
    #[inline(always)]
    fn default() -> GamgArea1 {
        <crate::RegValueT<GamgArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea2_SPEC;
impl crate::sealed::RegSpec for GamgArea2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 2"]
pub type GamgArea2 = crate::RegValueT<GamgArea2_SPEC>;

impl GamgArea2 {
    #[doc = "Start Threshold of Area 6"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 5"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 4"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea2 {
    #[inline(always)]
    fn default() -> GamgArea2 {
        <crate::RegValueT<GamgArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea3_SPEC;
impl crate::sealed::RegSpec for GamgArea3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 3"]
pub type GamgArea3 = crate::RegValueT<GamgArea3_SPEC>;

impl GamgArea3 {
    #[doc = "Start Threshold of Area 9"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 8"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 7"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea3 {
    #[inline(always)]
    fn default() -> GamgArea3 {
        <crate::RegValueT<GamgArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea4_SPEC;
impl crate::sealed::RegSpec for GamgArea4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 4"]
pub type GamgArea4 = crate::RegValueT<GamgArea4_SPEC>;

impl GamgArea4 {
    #[doc = "Start Threshold of Area 12"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 11"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 10"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea4 {
    #[inline(always)]
    fn default() -> GamgArea4 {
        <crate::RegValueT<GamgArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamgArea5_SPEC;
impl crate::sealed::RegSpec for GamgArea5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma G Correction Block Area Setting Register 5"]
pub type GamgArea5 = crate::RegValueT<GamgArea5_SPEC>;

impl GamgArea5 {
    #[doc = "Start Threshold of Area 15"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamgArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamgArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 14"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamgArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamgArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 13"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamgArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamgArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamgArea5 {
    #[inline(always)]
    fn default() -> GamgArea5 {
        <crate::RegValueT<GamgArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLatch_SPEC;
impl crate::sealed::RegSpec for GambLatch_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Register Update Control Register"]
pub type GambLatch = crate::RegValueT<GambLatch_SPEC>;

impl GambLatch {
    #[doc = "Control of Gamma Correction × Module Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gamb_latch::Ven,
        gamb_latch::Ven,
        GambLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gamb_latch::Ven,
            gamb_latch::Ven,
            GambLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GambLatch {
    #[inline(always)]
    fn default() -> GambLatch {
        <crate::RegValueT<GambLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gamb_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut1_SPEC;
impl crate::sealed::RegSpec for GambLut1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 1"]
pub type GambLut1 = crate::RegValueT<GambLut1_SPEC>;

impl GambLut1 {
    #[doc = "Gain Value of Area 1"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 0"]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut1 {
    #[inline(always)]
    fn default() -> GambLut1 {
        <crate::RegValueT<GambLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut2_SPEC;
impl crate::sealed::RegSpec for GambLut2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 2"]
pub type GambLut2 = crate::RegValueT<GambLut2_SPEC>;

impl GambLut2 {
    #[doc = "Gain Value of Area 3"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 2"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut2 {
    #[inline(always)]
    fn default() -> GambLut2 {
        <crate::RegValueT<GambLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut3_SPEC;
impl crate::sealed::RegSpec for GambLut3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 3"]
pub type GambLut3 = crate::RegValueT<GambLut3_SPEC>;

impl GambLut3 {
    #[doc = "Gain Value of Area 5"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 4"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut3 {
    #[inline(always)]
    fn default() -> GambLut3 {
        <crate::RegValueT<GambLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut4_SPEC;
impl crate::sealed::RegSpec for GambLut4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 4"]
pub type GambLut4 = crate::RegValueT<GambLut4_SPEC>;

impl GambLut4 {
    #[doc = "Gain Value of Area 7"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 6"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut4 {
    #[inline(always)]
    fn default() -> GambLut4 {
        <crate::RegValueT<GambLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut5_SPEC;
impl crate::sealed::RegSpec for GambLut5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 5"]
pub type GambLut5 = crate::RegValueT<GambLut5_SPEC>;

impl GambLut5 {
    #[doc = "Gain Value of Area 9"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 8"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut5 {
    #[inline(always)]
    fn default() -> GambLut5 {
        <crate::RegValueT<GambLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut6_SPEC;
impl crate::sealed::RegSpec for GambLut6_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 6"]
pub type GambLut6 = crate::RegValueT<GambLut6_SPEC>;

impl GambLut6 {
    #[doc = "Gain Value of Area 11"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 10"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut6 {
    #[inline(always)]
    fn default() -> GambLut6 {
        <crate::RegValueT<GambLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut7_SPEC;
impl crate::sealed::RegSpec for GambLut7_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 7"]
pub type GambLut7 = crate::RegValueT<GambLut7_SPEC>;

impl GambLut7 {
    #[doc = "Gain Value of Area 13"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 12"]
    #[inline(always)]
    pub fn gain012(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut7 {
    #[inline(always)]
    fn default() -> GambLut7 {
        <crate::RegValueT<GambLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambLut8_SPEC;
impl crate::sealed::RegSpec for GambLut8_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Table Setting Register 8"]
pub type GambLut8 = crate::RegValueT<GambLut8_SPEC>;

impl GambLut8 {
    #[doc = "Gain Value of Area 15"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GambLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GambLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 14"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GambLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GambLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambLut8 {
    #[inline(always)]
    fn default() -> GambLut8 {
        <crate::RegValueT<GambLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea1_SPEC;
impl crate::sealed::RegSpec for GambArea1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 1"]
pub type GambArea1 = crate::RegValueT<GambArea1_SPEC>;

impl GambArea1 {
    #[doc = "Start Threshold of Area 3"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 2"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 1"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea1 {
    #[inline(always)]
    fn default() -> GambArea1 {
        <crate::RegValueT<GambArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea2_SPEC;
impl crate::sealed::RegSpec for GambArea2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 2"]
pub type GambArea2 = crate::RegValueT<GambArea2_SPEC>;

impl GambArea2 {
    #[doc = "Start Threshold of Area 6"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 5"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 4"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea2 {
    #[inline(always)]
    fn default() -> GambArea2 {
        <crate::RegValueT<GambArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea3_SPEC;
impl crate::sealed::RegSpec for GambArea3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 3"]
pub type GambArea3 = crate::RegValueT<GambArea3_SPEC>;

impl GambArea3 {
    #[doc = "Start Threshold of Area 9"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 8"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 7"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea3 {
    #[inline(always)]
    fn default() -> GambArea3 {
        <crate::RegValueT<GambArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea4_SPEC;
impl crate::sealed::RegSpec for GambArea4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 4"]
pub type GambArea4 = crate::RegValueT<GambArea4_SPEC>;

impl GambArea4 {
    #[doc = "Start Threshold of Area 12"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 11"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 10"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea4 {
    #[inline(always)]
    fn default() -> GambArea4 {
        <crate::RegValueT<GambArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GambArea5_SPEC;
impl crate::sealed::RegSpec for GambArea5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma B Correction Block Area Setting Register 5"]
pub type GambArea5 = crate::RegValueT<GambArea5_SPEC>;

impl GambArea5 {
    #[doc = "Start Threshold of Area 15"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GambArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GambArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 14"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GambArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GambArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 13"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GambArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GambArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GambArea5 {
    #[inline(always)]
    fn default() -> GambArea5 {
        <crate::RegValueT<GambArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLatch_SPEC;
impl crate::sealed::RegSpec for GamrLatch_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Register Update Control Register"]
pub type GamrLatch = crate::RegValueT<GamrLatch_SPEC>;

impl GamrLatch {
    #[doc = "Control of Gamma Correction × Module Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gamr_latch::Ven,
        gamr_latch::Ven,
        GamrLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gamr_latch::Ven,
            gamr_latch::Ven,
            GamrLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GamrLatch {
    #[inline(always)]
    fn default() -> GamrLatch {
        <crate::RegValueT<GamrLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gamr_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of the vertical synchronization signal (VS)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut1_SPEC;
impl crate::sealed::RegSpec for GamrLut1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 1"]
pub type GamrLut1 = crate::RegValueT<GamrLut1_SPEC>;

impl GamrLut1 {
    #[doc = "Gain Value of Area 1"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 0"]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut1 {
    #[inline(always)]
    fn default() -> GamrLut1 {
        <crate::RegValueT<GamrLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut2_SPEC;
impl crate::sealed::RegSpec for GamrLut2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 2"]
pub type GamrLut2 = crate::RegValueT<GamrLut2_SPEC>;

impl GamrLut2 {
    #[doc = "Gain Value of Area 3"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 2"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut2 {
    #[inline(always)]
    fn default() -> GamrLut2 {
        <crate::RegValueT<GamrLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut3_SPEC;
impl crate::sealed::RegSpec for GamrLut3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 3"]
pub type GamrLut3 = crate::RegValueT<GamrLut3_SPEC>;

impl GamrLut3 {
    #[doc = "Gain Value of Area 5"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 4"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut3 {
    #[inline(always)]
    fn default() -> GamrLut3 {
        <crate::RegValueT<GamrLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut4_SPEC;
impl crate::sealed::RegSpec for GamrLut4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 4"]
pub type GamrLut4 = crate::RegValueT<GamrLut4_SPEC>;

impl GamrLut4 {
    #[doc = "Gain Value of Area 7"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 6"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut4 {
    #[inline(always)]
    fn default() -> GamrLut4 {
        <crate::RegValueT<GamrLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut5_SPEC;
impl crate::sealed::RegSpec for GamrLut5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 5"]
pub type GamrLut5 = crate::RegValueT<GamrLut5_SPEC>;

impl GamrLut5 {
    #[doc = "Gain Value of Area 9"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 8"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut5 {
    #[inline(always)]
    fn default() -> GamrLut5 {
        <crate::RegValueT<GamrLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut6_SPEC;
impl crate::sealed::RegSpec for GamrLut6_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 6"]
pub type GamrLut6 = crate::RegValueT<GamrLut6_SPEC>;

impl GamrLut6 {
    #[doc = "Gain Value of Area 11"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 10"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut6 {
    #[inline(always)]
    fn default() -> GamrLut6 {
        <crate::RegValueT<GamrLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut7_SPEC;
impl crate::sealed::RegSpec for GamrLut7_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 7"]
pub type GamrLut7 = crate::RegValueT<GamrLut7_SPEC>;

impl GamrLut7 {
    #[doc = "Gain Value of Area 13"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 12"]
    #[inline(always)]
    pub fn gain012(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut7 {
    #[inline(always)]
    fn default() -> GamrLut7 {
        <crate::RegValueT<GamrLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrLut8_SPEC;
impl crate::sealed::RegSpec for GamrLut8_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Table Setting Register 8"]
pub type GamrLut8 = crate::RegValueT<GamrLut8_SPEC>;

impl GamrLut8 {
    #[doc = "Gain Value of Area 15"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, GamrLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,GamrLut8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gain Value of Area 14"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, GamrLut8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,GamrLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrLut8 {
    #[inline(always)]
    fn default() -> GamrLut8 {
        <crate::RegValueT<GamrLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea1_SPEC;
impl crate::sealed::RegSpec for GamrArea1_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 1"]
pub type GamrArea1 = crate::RegValueT<GamrArea1_SPEC>;

impl GamrArea1 {
    #[doc = "Start Threshold of Area 3"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 2"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 1"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea1 {
    #[inline(always)]
    fn default() -> GamrArea1 {
        <crate::RegValueT<GamrArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea2_SPEC;
impl crate::sealed::RegSpec for GamrArea2_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 2"]
pub type GamrArea2 = crate::RegValueT<GamrArea2_SPEC>;

impl GamrArea2 {
    #[doc = "Start Threshold of Area 6"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 5"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 4"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea2 {
    #[inline(always)]
    fn default() -> GamrArea2 {
        <crate::RegValueT<GamrArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea3_SPEC;
impl crate::sealed::RegSpec for GamrArea3_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 3"]
pub type GamrArea3 = crate::RegValueT<GamrArea3_SPEC>;

impl GamrArea3 {
    #[doc = "Start Threshold of Area 9"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 8"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 7"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea3 {
    #[inline(always)]
    fn default() -> GamrArea3 {
        <crate::RegValueT<GamrArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea4_SPEC;
impl crate::sealed::RegSpec for GamrArea4_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 4"]
pub type GamrArea4 = crate::RegValueT<GamrArea4_SPEC>;

impl GamrArea4 {
    #[doc = "Start Threshold of Area 12"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 11"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 10"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea4 {
    #[inline(always)]
    fn default() -> GamrArea4 {
        <crate::RegValueT<GamrArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamrArea5_SPEC;
impl crate::sealed::RegSpec for GamrArea5_SPEC {
    type DataType = u32;
}

#[doc = "Gamma R Correction Block Area Setting Register 5"]
pub type GamrArea5 = crate::RegValueT<GamrArea5_SPEC>;

impl GamrArea5 {
    #[doc = "Start Threshold of Area 15"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamrArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamrArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 14"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamrArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamrArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Start Threshold of Area 13"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamrArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamrArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamrArea5 {
    #[inline(always)]
    fn default() -> GamrArea5 {
        <crate::RegValueT<GamrArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutVlatch_SPEC;
impl crate::sealed::RegSpec for OutVlatch_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Register Update Control Register"]
pub type OutVlatch = crate::RegValueT<OutVlatch_SPEC>;

impl OutVlatch {
    #[doc = "Control of Output Control Module Register Value Reflection to Internal Operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        out_vlatch::Ven,
        out_vlatch::Ven,
        OutVlatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            out_vlatch::Ven,
            out_vlatch::Ven,
            OutVlatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OutVlatch {
    #[inline(always)]
    fn default() -> OutVlatch {
        <crate::RegValueT<OutVlatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_vlatch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Disable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable reflection of register values to internal operations on assertion of vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet_SPEC;
impl crate::sealed::RegSpec for OutSet_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Output Interface Register"]
pub type OutSet = crate::RegValueT<OutSet_SPEC>;

impl OutSet {
    #[doc = "Data Output Delay Control in Serial RGB Format"]
    #[inline(always)]
    pub fn phase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        out_set::Phase,
        out_set::Phase,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            out_set::Phase,
            out_set::Phase,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Scan Direction Select of Serial RGB Format"]
    #[inline(always)]
    pub fn dirsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        out_set::Dirsel,
        out_set::Dirsel,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            out_set::Dirsel,
            out_set::Dirsel,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clock Frequency Division Control"]
    #[inline(always)]
    pub fn frqsel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        out_set::Frqsel,
        out_set::Frqsel,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            out_set::Frqsel,
            out_set::Frqsel,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output Format Select"]
    #[inline(always)]
    pub fn format(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        out_set::Format,
        out_set::Format,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            out_set::Format,
            out_set::Format,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pixel Order Control"]
    #[inline(always)]
    pub fn swapon(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        out_set::Swapon,
        out_set::Swapon,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            out_set::Swapon,
            out_set::Swapon,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bit Endian Control"]
    #[inline(always)]
    pub fn endianon(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        out_set::Endianon,
        out_set::Endianon,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            out_set::Endianon,
            out_set::Endianon,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OutSet {
    #[inline(always)]
    fn default() -> OutSet {
        <crate::RegValueT<OutSet_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_set {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phase_SPEC;
    pub type Phase = crate::EnumBitfieldStruct<u8, Phase_SPEC>;
    impl Phase {
        #[doc = "0 cycle"]
        pub const _00: Self = Self::new(0);

        #[doc = "1 cycle"]
        pub const _01: Self = Self::new(1);

        #[doc = "2 cycles"]
        pub const _10: Self = Self::new(2);

        #[doc = "3 cycles"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirsel_SPEC;
    pub type Dirsel = crate::EnumBitfieldStruct<u8, Dirsel_SPEC>;
    impl Dirsel {
        #[doc = "Forward scan"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reverse scan"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frqsel_SPEC;
    pub type Frqsel = crate::EnumBitfieldStruct<u8, Frqsel_SPEC>;
    impl Frqsel {
        #[doc = "No frequency division, parallel RGB"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Quarter frequency (serial RGB)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        #[doc = "RGB888 — select RGB888 as dither output format"]
        pub const _00: Self = Self::new(0);

        #[doc = "RGB666 — select RGB666 as dither output format"]
        pub const _01: Self = Self::new(1);

        #[doc = "RGB565 — select RGB565 as dither output format"]
        pub const _10: Self = Self::new(2);

        #[doc = "Serial RGB — select RGB888 as dither output format. Select dither output format in OUT_PDTHA.FORM\\[1:0\\]"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swapon_SPEC;
    pub type Swapon = crate::EnumBitfieldStruct<u8, Swapon_SPEC>;
    impl Swapon {
        #[doc = "RGB order"]
        pub const _0: Self = Self::new(0);

        #[doc = "BGR order"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Endianon_SPEC;
    pub type Endianon = crate::EnumBitfieldStruct<u8, Endianon_SPEC>;
    impl Endianon {
        #[doc = "Descending order (little endian)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Ascending order (big endian)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBright1_SPEC;
impl crate::sealed::RegSpec for OutBright1_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Brightness Correction Register 1"]
pub type OutBright1 = crate::RegValueT<OutBright1_SPEC>;

impl OutBright1 {
    #[doc = "Brightness Adjustment of G Signal"]
    #[inline(always)]
    pub fn brtg(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutBright1 {
    #[inline(always)]
    fn default() -> OutBright1 {
        <crate::RegValueT<OutBright1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBright2_SPEC;
impl crate::sealed::RegSpec for OutBright2_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Brightness Correction Register 2"]
pub type OutBright2 = crate::RegValueT<OutBright2_SPEC>;

impl OutBright2 {
    #[doc = "Brightness Adjustment of R Signal"]
    #[inline(always)]
    pub fn brtr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Brightness Adjustment of B Signal"]
    #[inline(always)]
    pub fn brtb(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutBright2 {
    #[inline(always)]
    fn default() -> OutBright2 {
        <crate::RegValueT<OutBright2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutContrast_SPEC;
impl crate::sealed::RegSpec for OutContrast_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Contrast Correction Register"]
pub type OutContrast = crate::RegValueT<OutContrast_SPEC>;

impl OutContrast {
    #[doc = "Contrast Adjustment of R Signal"]
    #[inline(always)]
    pub fn contr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Contrast Adjustment of B Signal"]
    #[inline(always)]
    pub fn contb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Contrast Adjustment of G Signal"]
    #[inline(always)]
    pub fn contg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutContrast {
    #[inline(always)]
    fn default() -> OutContrast {
        <crate::RegValueT<OutContrast_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPdtha_SPEC;
impl crate::sealed::RegSpec for OutPdtha_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Panel Dither Correction Register"]
pub type OutPdtha = crate::RegValueT<OutPdtha_SPEC>;

impl OutPdtha {
    #[doc = "Pattern Value (D) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern Value (C) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern Value (B) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pb(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pattern Value (A) of 2×2 Pattern Dither"]
    #[inline(always)]
    pub fn pa(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Output Format Select"]
    #[inline(always)]
    pub fn form(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        out_pdtha::Form,
        out_pdtha::Form,
        OutPdtha_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            out_pdtha::Form,
            out_pdtha::Form,
            OutPdtha_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation Mode"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        out_pdtha::Sel,
        out_pdtha::Sel,
        OutPdtha_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            out_pdtha::Sel,
            out_pdtha::Sel,
            OutPdtha_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OutPdtha {
    #[inline(always)]
    fn default() -> OutPdtha {
        <crate::RegValueT<OutPdtha_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_pdtha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Form_SPEC;
    pub type Form = crate::EnumBitfieldStruct<u8, Form_SPEC>;
    impl Form {
        #[doc = "RGB888; select RGB888 or serial RGB as output interface format"]
        pub const _00: Self = Self::new(0);

        #[doc = "RGB666; select RGB666 as output interface format"]
        pub const _01: Self = Self::new(1);

        #[doc = "RGB565; select RGB565 as output interface format"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited Select output interface format in OUT_SET.FORMAT\\[1:0\\]."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Truncate"]
        pub const _00: Self = Self::new(0);

        #[doc = "Round-off"]
        pub const _01: Self = Self::new(1);

        #[doc = "2×2 pattern dither"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClkphase_SPEC;
impl crate::sealed::RegSpec for OutClkphase_SPEC {
    type DataType = u32;
}

#[doc = "Output Control Block Output Phase Control Register"]
pub type OutClkphase = crate::RegValueT<OutClkphase_SPEC>;

impl OutClkphase {
    #[doc = "LCD_TCON3 Output Phase Control"]
    #[inline(always)]
    pub fn tcon3edge(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        out_clkphase::Tcon3Edge,
        out_clkphase::Tcon3Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            out_clkphase::Tcon3Edge,
            out_clkphase::Tcon3Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_TCON2 Output Phase Control"]
    #[inline(always)]
    pub fn tcon2edge(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        out_clkphase::Tcon2Edge,
        out_clkphase::Tcon2Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            out_clkphase::Tcon2Edge,
            out_clkphase::Tcon2Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_TCON1 Output Phase Control"]
    #[inline(always)]
    pub fn tcon1edge(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        out_clkphase::Tcon1Edge,
        out_clkphase::Tcon1Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            out_clkphase::Tcon1Edge,
            out_clkphase::Tcon1Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_TCON0 Output Phase Control"]
    #[inline(always)]
    pub fn tcon0edge(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        out_clkphase::Tcon0Edge,
        out_clkphase::Tcon0Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            out_clkphase::Tcon0Edge,
            out_clkphase::Tcon0Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD_DATA Output Phase Control"]
    #[inline(always)]
    pub fn lcdedge(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        out_clkphase::Lcdedge,
        out_clkphase::Lcdedge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            out_clkphase::Lcdedge,
            out_clkphase::Lcdedge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Correction Control"]
    #[inline(always)]
    pub fn frontgam(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        out_clkphase::Frontgam,
        out_clkphase::Frontgam,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            out_clkphase::Frontgam,
            out_clkphase::Frontgam,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OutClkphase {
    #[inline(always)]
    fn default() -> OutClkphase {
        <crate::RegValueT<OutClkphase_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_clkphase {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon3Edge_SPEC;
    pub type Tcon3Edge = crate::EnumBitfieldStruct<u8, Tcon3Edge_SPEC>;
    impl Tcon3Edge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon2Edge_SPEC;
    pub type Tcon2Edge = crate::EnumBitfieldStruct<u8, Tcon2Edge_SPEC>;
    impl Tcon2Edge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon1Edge_SPEC;
    pub type Tcon1Edge = crate::EnumBitfieldStruct<u8, Tcon1Edge_SPEC>;
    impl Tcon1Edge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon0Edge_SPEC;
    pub type Tcon0Edge = crate::EnumBitfieldStruct<u8, Tcon0Edge_SPEC>;
    impl Tcon0Edge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdedge_SPEC;
    pub type Lcdedge = crate::EnumBitfieldStruct<u8, Lcdedge_SPEC>;
    impl Lcdedge {
        #[doc = "Synchronize output with rising edge of LCD_CLK"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronize output with falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frontgam_SPEC;
    pub type Frontgam = crate::EnumBitfieldStruct<u8, Frontgam_SPEC>;
    impl Frontgam {
        #[doc = "Process brightness/contrast correction followed by gamma correction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Process gamma correction followed by brightness/contrast correction"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconTim_SPEC;
impl crate::sealed::RegSpec for TconTim_SPEC {
    type DataType = u32;
}

#[doc = "TCON Reference Timing Setting Register"]
pub type TconTim = crate::RegValueT<TconTim_SPEC>;

impl TconTim {
    #[doc = "Horizontal Synchronization Signal Generation Reference Timing"]
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconTim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Synchronization Signal Generation Change Timing"]
    #[inline(always)]
    pub fn half(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconTim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconTim {
    #[inline(always)]
    fn default() -> TconTim {
        <crate::RegValueT<TconTim_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStva1_SPEC;
impl crate::sealed::RegSpec for TconStva1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x1"]
pub type TconStva1 = crate::RegValueT<TconStva1_SPEC>;

impl TconStva1 {
    #[doc = "Vertical Synchronization Signal STVx1 Second Change Timing"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconStva1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconStva1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Synchronization Signal STVx1 First Change Timing"]
    #[inline(always)]
    pub fn vs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconStva1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconStva1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStva1 {
    #[inline(always)]
    fn default() -> TconStva1 {
        <crate::RegValueT<TconStva1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStva2_SPEC;
impl crate::sealed::RegSpec for TconStva2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x2"]
pub type TconStva2 = crate::RegValueT<TconStva2_SPEC>;

impl TconStva2 {
    #[doc = "Output Signal Select Control for VSOUT/VEOUT Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stva2::Sel,
        tcon_stva2::Sel,
        TconStva2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stva2::Sel,
            tcon_stva2::Sel,
            TconStva2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Vertical Synchronization Signal STVx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stva2::Inv,
        tcon_stva2::Inv,
        TconStva2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stva2::Inv,
            tcon_stva2::Inv,
            TconStva2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconStva2 {
    #[inline(always)]
    fn default() -> TconStva2 {
        <crate::RegValueT<TconStva2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stva2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStvb1_SPEC;
impl crate::sealed::RegSpec for TconStvb1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x1"]
pub type TconStvb1 = crate::RegValueT<TconStvb1_SPEC>;

impl TconStvb1 {
    #[doc = "Vertical Synchronization Signal STVx1 Second Change Timing"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconStvb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconStvb1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vertical Synchronization Signal STVx1 First Change Timing"]
    #[inline(always)]
    pub fn vs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconStvb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconStvb1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStvb1 {
    #[inline(always)]
    fn default() -> TconStvb1 {
        <crate::RegValueT<TconStvb1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStvb2_SPEC;
impl crate::sealed::RegSpec for TconStvb2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Vertical Timing Setting Register x2"]
pub type TconStvb2 = crate::RegValueT<TconStvb2_SPEC>;

impl TconStvb2 {
    #[doc = "Output Signal Select Control for VSOUT/VEOUT Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stvb2::Sel,
        tcon_stvb2::Sel,
        TconStvb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stvb2::Sel,
            tcon_stvb2::Sel,
            TconStvb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Vertical Synchronization Signal STVx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stvb2::Inv,
        tcon_stvb2::Inv,
        TconStvb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stvb2::Inv,
            tcon_stvb2::Inv,
            TconStvb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconStvb2 {
    #[inline(always)]
    fn default() -> TconStvb2 {
        <crate::RegValueT<TconStvb2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stvb2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStha1_SPEC;
impl crate::sealed::RegSpec for TconStha1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x1"]
pub type TconStha1 = crate::RegValueT<TconStha1_SPEC>;

impl TconStha1 {
    #[doc = "Horizontal Synchronization Signal STHx1 Second Change Timing"]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconStha1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconStha1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Synchronization Signal STHx1 First Change Timing"]
    #[inline(always)]
    pub fn hs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconStha1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconStha1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStha1 {
    #[inline(always)]
    fn default() -> TconStha1 {
        <crate::RegValueT<TconStha1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStha2_SPEC;
impl crate::sealed::RegSpec for TconStha2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x2"]
pub type TconStha2 = crate::RegValueT<TconStha2_SPEC>;

impl TconStha2 {
    #[doc = "Output Signal Select Control for LCD_TCON2/LCD_TCON3 Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stha2::Sel,
        tcon_stha2::Sel,
        TconStha2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stha2::Sel,
            tcon_stha2::Sel,
            TconStha2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stha2::Inv,
        tcon_stha2::Inv,
        TconStha2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stha2::Inv,
            tcon_stha2::Inv,
            TconStha2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Reference Timing Control"]
    #[inline(always)]
    pub fn hssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tcon_stha2::Hssel,
        tcon_stha2::Hssel,
        TconStha2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tcon_stha2::Hssel,
            tcon_stha2::Hssel,
            TconStha2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconStha2 {
    #[inline(always)]
    fn default() -> TconStha2 {
        <crate::RegValueT<TconStha2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stha2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hssel_SPEC;
    pub type Hssel = crate::EnumBitfieldStruct<u8, Hssel_SPEC>;
    impl Hssel {
        #[doc = "Select input horizontal synchronization signal (HSIN) as reference for signal generation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select offset specified in TCON_TIM.OFFSET\\[10:0\\] (horizontal synchronization generation reference timing) as reference for signal generation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSthb1_SPEC;
impl crate::sealed::RegSpec for TconSthb1_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x1"]
pub type TconSthb1 = crate::RegValueT<TconSthb1_SPEC>;

impl TconSthb1 {
    #[doc = "Horizontal Synchronization Signal STHx1 Second Change Timing"]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, TconSthb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,TconSthb1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Synchronization Signal STHx1 First Change Timing"]
    #[inline(always)]
    pub fn hs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, u16, TconSthb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16,u16,TconSthb1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSthb1 {
    #[inline(always)]
    fn default() -> TconSthb1 {
        <crate::RegValueT<TconSthb1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSthb2_SPEC;
impl crate::sealed::RegSpec for TconSthb2_SPEC {
    type DataType = u32;
}

#[doc = "TCON Horizontal Timing Setting Register x2"]
pub type TconSthb2 = crate::RegValueT<TconSthb2_SPEC>;

impl TconSthb2 {
    #[doc = "Output Signal Select Control for LCD_TCON2/LCD_TCON3 Pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_sthb2::Sel,
        tcon_sthb2::Sel,
        TconSthb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_sthb2::Sel,
            tcon_sthb2::Sel,
            TconSthb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_sthb2::Inv,
        tcon_sthb2::Inv,
        TconSthb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_sthb2::Inv,
            tcon_sthb2::Inv,
            TconSthb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Synchronization Signal STHx Reference Timing Control"]
    #[inline(always)]
    pub fn hssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tcon_sthb2::Hssel,
        tcon_sthb2::Hssel,
        TconSthb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tcon_sthb2::Hssel,
            tcon_sthb2::Hssel,
            TconSthb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconSthb2 {
    #[inline(always)]
    fn default() -> TconSthb2 {
        <crate::RegValueT<TconSthb2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_sthb2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);

        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);

        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);

        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hssel_SPEC;
    pub type Hssel = crate::EnumBitfieldStruct<u8, Hssel_SPEC>;
    impl Hssel {
        #[doc = "Select input horizontal synchronization signal (HSIN) as reference for signal generation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select offset specified in TCON_TIM.OFFSET\\[10:0\\] (horizontal synchronization generation reference timing) as reference for signal generation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconDe_SPEC;
impl crate::sealed::RegSpec for TconDe_SPEC {
    type DataType = u32;
}

#[doc = "TCON Data Enable Polarity Setting Register"]
pub type TconDe = crate::RegValueT<TconDe_SPEC>;

impl TconDe {
    #[doc = "Data Enable Signal DE Polarity Inversion Control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tcon_de::Inv,
        tcon_de::Inv,
        TconDe_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcon_de::Inv,
            tcon_de::Inv,
            TconDe_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TconDe {
    #[inline(always)]
    fn default() -> TconDe {
        <crate::RegValueT<TconDe_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_de {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Do not invert"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invert"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntDtcten_SPEC;
impl crate::sealed::RegSpec for SyscntDtcten_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block State Detection Control Register"]
pub type SyscntDtcten = crate::RegValueT<SyscntDtcten_SPEC>;

impl SyscntDtcten {
    #[doc = "Specified Line Detection Control"]
    #[inline(always)]
    pub fn vposdtc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_dtcten::Vposdtc,
        syscnt_dtcten::Vposdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_dtcten::Vposdtc,
            syscnt_dtcten::Vposdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 1 Underflow Detection Control"]
    #[inline(always)]
    pub fn l1undfdtc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_dtcten::L1Undfdtc,
        syscnt_dtcten::L1Undfdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_dtcten::L1Undfdtc,
            syscnt_dtcten::L1Undfdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 2 Underflow Detection Control"]
    #[inline(always)]
    pub fn l2undfdtc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_dtcten::L2Undfdtc,
        syscnt_dtcten::L2Undfdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_dtcten::L2Undfdtc,
            syscnt_dtcten::L2Undfdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntDtcten {
    #[inline(always)]
    fn default() -> SyscntDtcten {
        <crate::RegValueT<SyscntDtcten_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_dtcten {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposdtc_SPEC;
    pub type Vposdtc = crate::EnumBitfieldStruct<u8, Vposdtc_SPEC>;
    impl Vposdtc {
        #[doc = "Disable detection of specified line"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable detection of specified line"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfdtc_SPEC;
    pub type L1Undfdtc = crate::EnumBitfieldStruct<u8, L1Undfdtc_SPEC>;
    impl L1Undfdtc {
        #[doc = "Disable detection of graphics 1 underflow"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable detection of graphics 1 underflow"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfdtc_SPEC;
    pub type L2Undfdtc = crate::EnumBitfieldStruct<u8, L2Undfdtc_SPEC>;
    impl L2Undfdtc {
        #[doc = "Disable detection of graphics 2 underflow"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable detection of graphics 2 underflow"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntInten_SPEC;
impl crate::sealed::RegSpec for SyscntInten_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Interrupt Request Enable Control Register"]
pub type SyscntInten = crate::RegValueT<SyscntInten_SPEC>;

impl SyscntInten {
    #[doc = "Interrupt Request Signal GLCDC_VPOS Enable Control"]
    #[inline(always)]
    pub fn vposinten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_inten::Vposinten,
        syscnt_inten::Vposinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_inten::Vposinten,
            syscnt_inten::Vposinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Signal GLCDC_L1UNDF Enable Control"]
    #[inline(always)]
    pub fn l1undfinten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_inten::L1Undfinten,
        syscnt_inten::L1Undfinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_inten::L1Undfinten,
            syscnt_inten::L1Undfinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Signal GLCDC_L2UNDF Enable Control"]
    #[inline(always)]
    pub fn l2undfinten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_inten::L2Undfinten,
        syscnt_inten::L2Undfinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_inten::L2Undfinten,
            syscnt_inten::L2Undfinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntInten {
    #[inline(always)]
    fn default() -> SyscntInten {
        <crate::RegValueT<SyscntInten_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_inten {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposinten_SPEC;
    pub type Vposinten = crate::EnumBitfieldStruct<u8, Vposinten_SPEC>;
    impl Vposinten {
        #[doc = "Disable GLCDC_VPOS output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC_VPOS output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfinten_SPEC;
    pub type L1Undfinten = crate::EnumBitfieldStruct<u8, L1Undfinten_SPEC>;
    impl L1Undfinten {
        #[doc = "Disable GLCDC_L1UNDF output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC_L1UNDF output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfinten_SPEC;
    pub type L2Undfinten = crate::EnumBitfieldStruct<u8, L2Undfinten_SPEC>;
    impl L2Undfinten {
        #[doc = "Disable GLCDC_L2UNDF output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable GLCDC_L2UNDF output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStclr_SPEC;
impl crate::sealed::RegSpec for SyscntStclr_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Status Clear Register"]
pub type SyscntStclr = crate::RegValueT<SyscntStclr_SPEC>;

impl SyscntStclr {
    #[doc = "Graphics 2 Specified Line Detection Flag Clear"]
    #[inline(always)]
    pub fn vposclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_stclr::Vposclr,
        syscnt_stclr::Vposclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_stclr::Vposclr,
            syscnt_stclr::Vposclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 1 Underflow Detection Flag Clear"]
    #[inline(always)]
    pub fn l1undfclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_stclr::L1Undfclr,
        syscnt_stclr::L1Undfclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_stclr::L1Undfclr,
            syscnt_stclr::L1Undfclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 2 Underflow Detection Flag Clear"]
    #[inline(always)]
    pub fn l2undfclr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_stclr::L2Undfclr,
        syscnt_stclr::L2Undfclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_stclr::L2Undfclr,
            syscnt_stclr::L2Undfclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntStclr {
    #[inline(always)]
    fn default() -> SyscntStclr {
        <crate::RegValueT<SyscntStclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_stclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposclr_SPEC;
    pub type Vposclr = crate::EnumBitfieldStruct<u8, Vposclr_SPEC>;
    impl Vposclr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the graphics 2 specified line detection flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfclr_SPEC;
    pub type L1Undfclr = crate::EnumBitfieldStruct<u8, L1Undfclr_SPEC>;
    impl L1Undfclr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the graphics 1 underflow detection flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfclr_SPEC;
    pub type L2Undfclr = crate::EnumBitfieldStruct<u8, L2Undfclr_SPEC>;
    impl L2Undfclr {
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the graphics 2 underflow detection flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStmon_SPEC;
impl crate::sealed::RegSpec for SyscntStmon_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Status Monitor Register"]
pub type SyscntStmon = crate::RegValueT<SyscntStmon_SPEC>;

impl SyscntStmon {
    #[doc = "Graphics 2 Specified Line Detection Flag"]
    #[inline(always)]
    pub fn vpos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_stmon::Vpos,
        syscnt_stmon::Vpos,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_stmon::Vpos,
            syscnt_stmon::Vpos,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 1 Underflow Detection Flag"]
    #[inline(always)]
    pub fn l1undf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_stmon::L1Undf,
        syscnt_stmon::L1Undf,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_stmon::L1Undf,
            syscnt_stmon::L1Undf,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Graphics 2 Underflow Detection Flag"]
    #[inline(always)]
    pub fn l2undf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_stmon::L2Undf,
        syscnt_stmon::L2Undf,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_stmon::L2Undf,
            syscnt_stmon::L2Undf,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntStmon {
    #[inline(always)]
    fn default() -> SyscntStmon {
        <crate::RegValueT<SyscntStmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_stmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vpos_SPEC;
    pub type Vpos = crate::EnumBitfieldStruct<u8, Vpos_SPEC>;
    impl Vpos {
        #[doc = "Specified line notification not detected in graphics 2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Specified line notification detected in graphics 2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undf_SPEC;
    pub type L1Undf = crate::EnumBitfieldStruct<u8, L1Undf_SPEC>;
    impl L1Undf {
        #[doc = "No underflow detected in graphics 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow detected in graphics 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undf_SPEC;
    pub type L2Undf = crate::EnumBitfieldStruct<u8, L2Undf_SPEC>;
    impl L2Undf {
        #[doc = "No underflow detected in graphics 2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow detected in graphics 2"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntPanelClk_SPEC;
impl crate::sealed::RegSpec for SyscntPanelClk_SPEC {
    type DataType = u32;
}

#[doc = "System Control Block Version and Panel Clock Control Register"]
pub type SyscntPanelClk = crate::RegValueT<SyscntPanelClk_SPEC>;

impl SyscntPanelClk {
    #[doc = "Clock Division Ratio Setting Control"]
    #[inline(always)]
    pub fn dcdr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Panel Clock Output Enable Control"]
    #[inline(always)]
    pub fn clken(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syscnt_panel_clk::Clken,
        syscnt_panel_clk::Clken,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syscnt_panel_clk::Clken,
            syscnt_panel_clk::Clken,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Panel Clock Supply Source Control"]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        syscnt_panel_clk::Clksel,
        syscnt_panel_clk::Clksel,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            syscnt_panel_clk::Clksel,
            syscnt_panel_clk::Clksel,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pixel Clock Select Control"]
    #[inline(always)]
    pub fn pixsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        syscnt_panel_clk::Pixsel,
        syscnt_panel_clk::Pixsel,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            syscnt_panel_clk::Pixsel,
            syscnt_panel_clk::Pixsel,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Version Information"]
    #[inline(always)]
    pub fn ver(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        SyscntPanelClk_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            SyscntPanelClk_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyscntPanelClk {
    #[inline(always)]
    fn default() -> SyscntPanelClk {
        <crate::RegValueT<SyscntPanelClk_SPEC> as RegisterValue<_>>::new(17825792)
    }
}
pub mod syscnt_panel_clk {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clken_SPEC;
    pub type Clken = crate::EnumBitfieldStruct<u8, Clken_SPEC>;
    impl Clken {
        #[doc = "Disable panel clock output"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable panel clock output Before changing the PIXSEL, CLKSEL, or DCDR bit, this bit must be set to 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "Select external clock (LCD_EXTCLK)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select LCDCLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pixsel_SPEC;
    pub type Pixsel = crate::EnumBitfieldStruct<u8, Pixsel_SPEC>;
    impl Pixsel {
        #[doc = "Select no frequency division, parallel RGB"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select quarter frequency, serial RGB This setting must have the same value as OUT_SET.FRQSEL\\[1\\]."]
        pub const _1: Self = Self::new(1);
    }
}
