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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:54 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Graphics LCD Controller"]
unsafe impl ::core::marker::Send for super::Glcdc {}
unsafe impl ::core::marker::Sync for super::Glcdc {}
impl super::Glcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gr1_clut0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn gr1_clut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }

    #[inline(always)]
    pub const fn gr2_clut0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x800usize))
        }
    }

    #[inline(always)]
    pub const fn gr2_clut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc00usize))
        }
    }

    #[inline(always)]
    pub const fn bg_en(&self) -> &'static crate::common::Reg<self::BgEn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgEn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn bg_bgc(&self) -> &'static crate::common::Reg<self::BgBgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgBgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bg_mon(&self) -> &'static crate::common::Reg<self::BgMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::BgMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4120usize),
            )
        }
    }

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
    pub const fn gam_latch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLatch_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1300usize))
        }
    }

    #[inline(always)]
    pub const fn gam_sw(&self) -> &'static crate::common::Reg<self::GamSw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamSw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4868usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gam_lut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut1_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1308usize))
        }
    }

    #[inline(always)]
    pub const fn gam_lut2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut2_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x130cusize))
        }
    }

    #[inline(always)]
    pub const fn gam_lut3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut3_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1310usize))
        }
    }

    #[inline(always)]
    pub const fn gam_lut4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut4_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1314usize))
        }
    }

    #[inline(always)]
    pub const fn gam_lut5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut5_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1318usize))
        }
    }

    #[inline(always)]
    pub const fn gam_lut6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut6_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x131cusize))
        }
    }

    #[inline(always)]
    pub const fn gam_lut7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut7_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1320usize))
        }
    }

    #[inline(always)]
    pub const fn gam_lut8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut8_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1324usize))
        }
    }

    #[inline(always)]
    pub const fn gam_area1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea1_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1328usize))
        }
    }

    #[inline(always)]
    pub const fn gam_area2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea2_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x132cusize))
        }
    }

    #[inline(always)]
    pub const fn gam_area3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea3_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1330usize))
        }
    }

    #[inline(always)]
    pub const fn gam_area4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea4_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1334usize))
        }
    }

    #[inline(always)]
    pub const fn gam_area5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea5_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1338usize))
        }
    }

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

    #[inline(always)]
    pub const fn tcon_vlatch(
        &self,
    ) -> &'static crate::common::Reg<self::TconVlatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconVlatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5120usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn tcon_stv1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconStv1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1408usize))
        }
    }

    #[inline(always)]
    pub const fn tcon_stv2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconStv2_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140cusize))
        }
    }

    #[inline(always)]
    pub const fn tcon_sth1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconSth1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1418usize))
        }
    }

    #[inline(always)]
    pub const fn tcon_sth2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconSth2_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x141cusize))
        }
    }

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
pub struct Gr1Clut0_SPEC;
impl crate::sealed::RegSpec for Gr1Clut0_SPEC {
    type DataType = u32;
}

pub type Gr1Clut0 = crate::RegValueT<Gr1Clut0_SPEC>;

impl Gr1Clut0 {
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr1Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr1Clut0 {
    #[inline(always)]
    fn default() -> Gr1Clut0 {
        <crate::RegValueT<Gr1Clut0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr1Clut1_SPEC;
impl crate::sealed::RegSpec for Gr1Clut1_SPEC {
    type DataType = u32;
}

pub type Gr1Clut1 = crate::RegValueT<Gr1Clut1_SPEC>;

impl Gr1Clut1 {
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr1Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr1Clut1 {
    #[inline(always)]
    fn default() -> Gr1Clut1 {
        <crate::RegValueT<Gr1Clut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr2Clut0_SPEC;
impl crate::sealed::RegSpec for Gr2Clut0_SPEC {
    type DataType = u32;
}

pub type Gr2Clut0 = crate::RegValueT<Gr2Clut0_SPEC>;

impl Gr2Clut0 {
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr2Clut0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr2Clut0 {
    #[inline(always)]
    fn default() -> Gr2Clut0 {
        <crate::RegValueT<Gr2Clut0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr2Clut1_SPEC;
impl crate::sealed::RegSpec for Gr2Clut1_SPEC {
    type DataType = u32;
}

pub type Gr2Clut1 = crate::RegValueT<Gr2Clut1_SPEC>;

impl Gr2Clut1 {
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Gr2Clut1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr2Clut1 {
    #[inline(always)]
    fn default() -> Gr2Clut1 {
        <crate::RegValueT<Gr2Clut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgEn_SPEC;
impl crate::sealed::RegSpec for BgEn_SPEC {
    type DataType = u32;
}

pub type BgEn = crate::RegValueT<BgEn_SPEC>;

impl BgEn {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, BgEn_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,BgEn_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgPeri_SPEC;
impl crate::sealed::RegSpec for BgPeri_SPEC {
    type DataType = u32;
}

pub type BgPeri = crate::RegValueT<BgPeri_SPEC>;

impl BgPeri {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, BgPeri_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,BgPeri_SPEC,crate::common::RW>::from_register(self,0)
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
    pub type Fh = crate::EnumBitfieldStruct<u8, Fh_SPEC>;
    impl Fh {
        pub const FH: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fv_SPEC;
    pub type Fv = crate::EnumBitfieldStruct<u8, Fv_SPEC>;
    impl Fv {
        pub const FV: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgSync_SPEC;
impl crate::sealed::RegSpec for BgSync_SPEC {
    type DataType = u32;
}

pub type BgSync = crate::RegValueT<BgSync_SPEC>;

impl BgSync {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, BgSync_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xfff,1,0,u16,u16,BgSync_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0_X_0: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vp_SPEC;
    pub type Vp = crate::EnumBitfieldStruct<u8, Vp_SPEC>;
    impl Vp {
        pub const _0_X_0: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgVsize_SPEC;
impl crate::sealed::RegSpec for BgVsize_SPEC {
    type DataType = u32;
}

pub type BgVsize = crate::RegValueT<BgVsize_SPEC>;

impl BgVsize {
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        bg_vsize::Vw,
        bg_vsize::Vw,
        BgVsize_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            bg_vsize::Vw,
            bg_vsize::Vw,
            BgVsize_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        bg_vsize::Vp,
        bg_vsize::Vp,
        BgVsize_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            bg_vsize::Vp,
            bg_vsize::Vp,
            BgVsize_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, BgVsize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgVsize {
    #[inline(always)]
    fn default() -> BgVsize {
        <crate::RegValueT<BgVsize_SPEC> as RegisterValue<_>>::new(458768)
    }
}
pub mod bg_vsize {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vw_SPEC;
    pub type Vw = crate::EnumBitfieldStruct<u8, Vw_SPEC>;
    impl Vw {
        pub const VW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vp_SPEC;
    pub type Vp = crate::EnumBitfieldStruct<u8, Vp_SPEC>;
    impl Vp {
        pub const VP: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgHsize_SPEC;
impl crate::sealed::RegSpec for BgHsize_SPEC {
    type DataType = u32;
}

pub type BgHsize = crate::RegValueT<BgHsize_SPEC>;

impl BgHsize {
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        bg_hsize::Hw,
        bg_hsize::Hw,
        BgHsize_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            bg_hsize::Hw,
            bg_hsize::Hw,
            BgHsize_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        bg_hsize::Hp,
        bg_hsize::Hp,
        BgHsize_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            bg_hsize::Hp,
            bg_hsize::Hp,
            BgHsize_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, BgHsize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgHsize {
    #[inline(always)]
    fn default() -> BgHsize {
        <crate::RegValueT<BgHsize_SPEC> as RegisterValue<_>>::new(393232)
    }
}
pub mod bg_hsize {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hw_SPEC;
    pub type Hw = crate::EnumBitfieldStruct<u8, Hw_SPEC>;
    impl Hw {
        pub const HW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hp_SPEC;
    pub type Hp = crate::EnumBitfieldStruct<u8, Hp_SPEC>;
    impl Hp {
        pub const HP: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgBgc_SPEC;
impl crate::sealed::RegSpec for BgBgc_SPEC {
    type DataType = u32;
}

pub type BgBgc = crate::RegValueT<BgBgc_SPEC>;

impl BgBgc {
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,BgBgc_SPEC,crate::common::RW>::from_register(self,0)
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

pub type BgMon = crate::RegValueT<BgMon_SPEC>;

impl BgMon {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, BgMon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,BgMon_SPEC,crate::common::R>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrVen_SPEC;
impl crate::sealed::RegSpec for GrVen_SPEC {
    type DataType = u32;
}

pub type GrVen = crate::RegValueT<GrVen_SPEC>;

impl GrVen {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, GrVen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,GrVen_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlmrd_SPEC;
impl crate::sealed::RegSpec for GrFlmrd_SPEC {
    type DataType = u32;
}

pub type GrFlmrd = crate::RegValueT<GrFlmrd_SPEC>;

impl GrFlmrd {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, GrFlmrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,GrFlmrd_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm1_SPEC;
impl crate::sealed::RegSpec for GrFlm1_SPEC {
    type DataType = u32;
}

pub type GrFlm1 = crate::RegValueT<GrFlm1_SPEC>;

impl GrFlm1 {
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
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gr_flm1::Bstmd,
            gr_flm1::Bstmd,
            GrFlm1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, u32, GrFlm1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fffffff,1,0,u32,u32,GrFlm1_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _11: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm2_SPEC;
impl crate::sealed::RegSpec for GrFlm2_SPEC {
    type DataType = u32;
}

pub type GrFlm2 = crate::RegValueT<GrFlm2_SPEC>;

impl GrFlm2 {
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

pub type GrFlm3 = crate::RegValueT<GrFlm3_SPEC>;

impl GrFlm3 {
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

pub type GrFlm5 = crate::RegValueT<GrFlm5_SPEC>;

impl GrFlm5 {
    #[inline(always)]
    pub fn datanum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        gr_flm5::Datanum,
        gr_flm5::Datanum,
        GrFlm5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            gr_flm5::Datanum,
            gr_flm5::Datanum,
            GrFlm5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lnnum(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gr_flm5::Lnnum,
        gr_flm5::Lnnum,
        GrFlm5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gr_flm5::Lnnum,
            gr_flm5::Lnnum,
            GrFlm5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrFlm5 {
    #[inline(always)]
    fn default() -> GrFlm5 {
        <crate::RegValueT<GrFlm5_SPEC> as RegisterValue<_>>::new(983040)
    }
}
pub mod gr_flm5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datanum_SPEC;
    pub type Datanum = crate::EnumBitfieldStruct<u8, Datanum_SPEC>;
    impl Datanum {
        pub const DATAUM: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lnnum_SPEC;
    pub type Lnnum = crate::EnumBitfieldStruct<u8, Lnnum_SPEC>;
    impl Lnnum {
        pub const LNNUM: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm6_SPEC;
impl crate::sealed::RegSpec for GrFlm6_SPEC {
    type DataType = u32;
}

pub type GrFlm6 = crate::RegValueT<GrFlm6_SPEC>;

impl GrFlm6 {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, GrFlm6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, GrFlm6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
        pub const _111: Self = Self::new(7);

        pub const _110: Self = Self::new(6);

        pub const _101: Self = Self::new(5);

        pub const _100: Self = Self::new(4);

        pub const _011: Self = Self::new(3);

        pub const _010: Self = Self::new(2);

        pub const _001: Self = Self::new(1);

        pub const _000: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb1_SPEC;
impl crate::sealed::RegSpec for GrAb1_SPEC {
    type DataType = u32;
}

pub type GrAb1 = crate::RegValueT<GrAb1_SPEC>;

impl GrAb1 {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7ffff, 1, 0, u32, u32, GrAb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7ffff,1,0,u32,u32,GrAb1_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _11: Self = Self::new(3);

        pub const _10: Self = Self::new(2);

        pub const _01: Self = Self::new(1);

        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcdispon_SPEC;
    pub type Grcdispon = crate::EnumBitfieldStruct<u8, Grcdispon_SPEC>;
    impl Grcdispon {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcdispon_SPEC;
    pub type Arcdispon = crate::EnumBitfieldStruct<u8, Arcdispon_SPEC>;
    impl Arcdispon {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcon_SPEC;
    pub type Arcon = crate::EnumBitfieldStruct<u8, Arcon_SPEC>;
    impl Arcon {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb2_SPEC;
impl crate::sealed::RegSpec for GrAb2_SPEC {
    type DataType = u32;
}

pub type GrAb2 = crate::RegValueT<GrAb2_SPEC>;

impl GrAb2 {
    #[inline(always)]
    pub fn grcvw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gr_ab2::Grcvw,
        gr_ab2::Grcvw,
        GrAb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gr_ab2::Grcvw,
            gr_ab2::Grcvw,
            GrAb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grcvs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gr_ab2::Grcvs,
        gr_ab2::Grcvs,
        GrAb2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gr_ab2::Grcvs,
            gr_ab2::Grcvs,
            GrAb2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb2 {
    #[inline(always)]
    fn default() -> GrAb2 {
        <crate::RegValueT<GrAb2_SPEC> as RegisterValue<_>>::new(393232)
    }
}
pub mod gr_ab2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcvw_SPEC;
    pub type Grcvw = crate::EnumBitfieldStruct<u8, Grcvw_SPEC>;
    impl Grcvw {
        pub const GRCVW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcvs_SPEC;
    pub type Grcvs = crate::EnumBitfieldStruct<u8, Grcvs_SPEC>;
    impl Grcvs {
        pub const GRCVS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb3_SPEC;
impl crate::sealed::RegSpec for GrAb3_SPEC {
    type DataType = u32;
}

pub type GrAb3 = crate::RegValueT<GrAb3_SPEC>;

impl GrAb3 {
    #[inline(always)]
    pub fn grchw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gr_ab3::Grchw,
        gr_ab3::Grchw,
        GrAb3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gr_ab3::Grchw,
            gr_ab3::Grchw,
            GrAb3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grchs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gr_ab3::Grchs,
        gr_ab3::Grchs,
        GrAb3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gr_ab3::Grchs,
            gr_ab3::Grchs,
            GrAb3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb3 {
    #[inline(always)]
    fn default() -> GrAb3 {
        <crate::RegValueT<GrAb3_SPEC> as RegisterValue<_>>::new(327696)
    }
}
pub mod gr_ab3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grchw_SPEC;
    pub type Grchw = crate::EnumBitfieldStruct<u8, Grchw_SPEC>;
    impl Grchw {
        pub const GRCHW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grchs_SPEC;
    pub type Grchs = crate::EnumBitfieldStruct<u8, Grchs_SPEC>;
    impl Grchs {
        pub const GRCHS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb4_SPEC;
impl crate::sealed::RegSpec for GrAb4_SPEC {
    type DataType = u32;
}

pub type GrAb4 = crate::RegValueT<GrAb4_SPEC>;

impl GrAb4 {
    #[inline(always)]
    pub fn arcvw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gr_ab4::Arcvw,
        gr_ab4::Arcvw,
        GrAb4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gr_ab4::Arcvw,
            gr_ab4::Arcvw,
            GrAb4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn arcvs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gr_ab4::Arcvs,
        gr_ab4::Arcvs,
        GrAb4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gr_ab4::Arcvs,
            gr_ab4::Arcvs,
            GrAb4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb4 {
    #[inline(always)]
    fn default() -> GrAb4 {
        <crate::RegValueT<GrAb4_SPEC> as RegisterValue<_>>::new(393232)
    }
}
pub mod gr_ab4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcvw_SPEC;
    pub type Arcvw = crate::EnumBitfieldStruct<u8, Arcvw_SPEC>;
    impl Arcvw {
        pub const ARCVW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcvs_SPEC;
    pub type Arcvs = crate::EnumBitfieldStruct<u8, Arcvs_SPEC>;
    impl Arcvs {
        pub const ARCVS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb5_SPEC;
impl crate::sealed::RegSpec for GrAb5_SPEC {
    type DataType = u32;
}

pub type GrAb5 = crate::RegValueT<GrAb5_SPEC>;

impl GrAb5 {
    #[inline(always)]
    pub fn archw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gr_ab5::Archw,
        gr_ab5::Archw,
        GrAb5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gr_ab5::Archw,
            gr_ab5::Archw,
            GrAb5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn archs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gr_ab5::Archs,
        gr_ab5::Archs,
        GrAb5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gr_ab5::Archs,
            gr_ab5::Archs,
            GrAb5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GrAb5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb5 {
    #[inline(always)]
    fn default() -> GrAb5 {
        <crate::RegValueT<GrAb5_SPEC> as RegisterValue<_>>::new(327696)
    }
}
pub mod gr_ab5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Archw_SPEC;
    pub type Archw = crate::EnumBitfieldStruct<u8, Archw_SPEC>;
    impl Archw {
        pub const ARCHW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Archs_SPEC;
    pub type Archs = crate::EnumBitfieldStruct<u8, Archs_SPEC>;
    impl Archs {
        pub const ARCHS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb6_SPEC;
impl crate::sealed::RegSpec for GrAb6_SPEC {
    type DataType = u32;
}

pub type GrAb6 = crate::RegValueT<GrAb6_SPEC>;

impl GrAb6 {
    #[inline(always)]
    pub fn arcrate(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        gr_ab6::Arcrate,
        gr_ab6::Arcrate,
        GrAb6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            gr_ab6::Arcrate,
            gr_ab6::Arcrate,
            GrAb6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn arccoef(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, GrAb6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, u8, GrAb6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x7f,1,0,u8,u8,GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb6 {
    #[inline(always)]
    fn default() -> GrAb6 {
        <crate::RegValueT<GrAb6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcrate_SPEC;
    pub type Arcrate = crate::EnumBitfieldStruct<u8, Arcrate_SPEC>;
    impl Arcrate {
        pub const ARCRATE: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb7_SPEC;
impl crate::sealed::RegSpec for GrAb7_SPEC {
    type DataType = u32;
}

pub type GrAb7 = crate::RegValueT<GrAb7_SPEC>;

impl GrAb7 {
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

    #[inline(always)]
    pub fn arcdef(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrAb7_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb8_SPEC;
impl crate::sealed::RegSpec for GrAb8_SPEC {
    type DataType = u32;
}

pub type GrAb8 = crate::RegValueT<GrAb8_SPEC>;

impl GrAb8 {
    #[inline(always)]
    pub fn ckkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ckkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ckkg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrAb8_SPEC,crate::common::RW>::from_register(self,0)
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

pub type GrAb9 = crate::RegValueT<GrAb9_SPEC>;

impl GrAb9 {
    #[inline(always)]
    pub fn ckr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ckb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ckg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type GrBase = crate::RegValueT<GrBase_SPEC>;

impl GrBase {
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,GrBase_SPEC,crate::common::RW>::from_register(self,0)
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

pub type GrClutint = crate::RegValueT<GrClutint_SPEC>;

impl GrClutint {
    #[inline(always)]
    pub fn line(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gr_clutint::Line,
        gr_clutint::Line,
        GrClutint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gr_clutint::Line,
            gr_clutint::Line,
            GrClutint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, GrClutint_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,GrClutint_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Line_SPEC;
    pub type Line = crate::EnumBitfieldStruct<u8, Line_SPEC>;
    impl Line {
        pub const LINE: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrMon_SPEC;
impl crate::sealed::RegSpec for GrMon_SPEC {
    type DataType = u32;
}

pub type GrMon = crate::RegValueT<GrMon_SPEC>;

impl GrMon {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, GrMon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,GrMon_SPEC,crate::common::R>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Undflst_SPEC;
    pub type Undflst = crate::EnumBitfieldStruct<u8, Undflst_SPEC>;
    impl Undflst {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLatch_SPEC;
impl crate::sealed::RegSpec for GamLatch_SPEC {
    type DataType = u32;
}

pub type GamLatch = crate::RegValueT<GamLatch_SPEC>;

impl GamLatch {
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gam_latch::Ven,
        gam_latch::Ven,
        GamLatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gam_latch::Ven,
            gam_latch::Ven,
            GamLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, GamLatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            GamLatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GamLatch {
    #[inline(always)]
    fn default() -> GamLatch {
        <crate::RegValueT<GamLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamSw_SPEC;
impl crate::sealed::RegSpec for GamSw_SPEC {
    type DataType = u32;
}

pub type GamSw = crate::RegValueT<GamSw_SPEC>;

impl GamSw {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, GamSw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,GamSw_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut1_SPEC;
impl crate::sealed::RegSpec for GamLut1_SPEC {
    type DataType = u32;
}

pub type GamLut1 = crate::RegValueT<GamLut1_SPEC>;

impl GamLut1 {
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut1::Gain01,
        gam_lut1::Gain01,
        GamLut1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut1::Gain01,
            gam_lut1::Gain01,
            GamLut1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut1::Gain00,
        gam_lut1::Gain00,
        GamLut1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut1::Gain00,
            gam_lut1::Gain00,
            GamLut1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut1 {
    #[inline(always)]
    fn default() -> GamLut1 {
        <crate::RegValueT<GamLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain01_SPEC;
    pub type Gain01 = crate::EnumBitfieldStruct<u8, Gain01_SPEC>;
    impl Gain01 {
        pub const GAIN_01: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain00_SPEC;
    pub type Gain00 = crate::EnumBitfieldStruct<u8, Gain00_SPEC>;
    impl Gain00 {
        pub const GAIN_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut2_SPEC;
impl crate::sealed::RegSpec for GamLut2_SPEC {
    type DataType = u32;
}

pub type GamLut2 = crate::RegValueT<GamLut2_SPEC>;

impl GamLut2 {
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut2::Gain03,
        gam_lut2::Gain03,
        GamLut2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut2::Gain03,
            gam_lut2::Gain03,
            GamLut2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut2::Gain02,
        gam_lut2::Gain02,
        GamLut2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut2::Gain02,
            gam_lut2::Gain02,
            GamLut2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut2 {
    #[inline(always)]
    fn default() -> GamLut2 {
        <crate::RegValueT<GamLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain03_SPEC;
    pub type Gain03 = crate::EnumBitfieldStruct<u8, Gain03_SPEC>;
    impl Gain03 {
        pub const GAIN_03: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain02_SPEC;
    pub type Gain02 = crate::EnumBitfieldStruct<u8, Gain02_SPEC>;
    impl Gain02 {
        pub const GAIN_02: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut3_SPEC;
impl crate::sealed::RegSpec for GamLut3_SPEC {
    type DataType = u32;
}

pub type GamLut3 = crate::RegValueT<GamLut3_SPEC>;

impl GamLut3 {
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut3::Gain05,
        gam_lut3::Gain05,
        GamLut3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut3::Gain05,
            gam_lut3::Gain05,
            GamLut3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut3::Gain04,
        gam_lut3::Gain04,
        GamLut3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut3::Gain04,
            gam_lut3::Gain04,
            GamLut3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut3 {
    #[inline(always)]
    fn default() -> GamLut3 {
        <crate::RegValueT<GamLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain05_SPEC;
    pub type Gain05 = crate::EnumBitfieldStruct<u8, Gain05_SPEC>;
    impl Gain05 {
        pub const GAIN_05: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain04_SPEC;
    pub type Gain04 = crate::EnumBitfieldStruct<u8, Gain04_SPEC>;
    impl Gain04 {
        pub const GAIN_04: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut4_SPEC;
impl crate::sealed::RegSpec for GamLut4_SPEC {
    type DataType = u32;
}

pub type GamLut4 = crate::RegValueT<GamLut4_SPEC>;

impl GamLut4 {
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut4::Gain07,
        gam_lut4::Gain07,
        GamLut4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut4::Gain07,
            gam_lut4::Gain07,
            GamLut4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut4::Gain06,
        gam_lut4::Gain06,
        GamLut4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut4::Gain06,
            gam_lut4::Gain06,
            GamLut4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut4 {
    #[inline(always)]
    fn default() -> GamLut4 {
        <crate::RegValueT<GamLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain07_SPEC;
    pub type Gain07 = crate::EnumBitfieldStruct<u8, Gain07_SPEC>;
    impl Gain07 {
        pub const GAIN_07: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain06_SPEC;
    pub type Gain06 = crate::EnumBitfieldStruct<u8, Gain06_SPEC>;
    impl Gain06 {
        pub const GAIN_06: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut5_SPEC;
impl crate::sealed::RegSpec for GamLut5_SPEC {
    type DataType = u32;
}

pub type GamLut5 = crate::RegValueT<GamLut5_SPEC>;

impl GamLut5 {
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut5::Gain09,
        gam_lut5::Gain09,
        GamLut5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut5::Gain09,
            gam_lut5::Gain09,
            GamLut5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut5::Gain08,
        gam_lut5::Gain08,
        GamLut5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut5::Gain08,
            gam_lut5::Gain08,
            GamLut5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut5 {
    #[inline(always)]
    fn default() -> GamLut5 {
        <crate::RegValueT<GamLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain09_SPEC;
    pub type Gain09 = crate::EnumBitfieldStruct<u8, Gain09_SPEC>;
    impl Gain09 {
        pub const GAIN_09: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain08_SPEC;
    pub type Gain08 = crate::EnumBitfieldStruct<u8, Gain08_SPEC>;
    impl Gain08 {
        pub const GAIN_08: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut6_SPEC;
impl crate::sealed::RegSpec for GamLut6_SPEC {
    type DataType = u32;
}

pub type GamLut6 = crate::RegValueT<GamLut6_SPEC>;

impl GamLut6 {
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut6::Gain11,
        gam_lut6::Gain11,
        GamLut6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut6::Gain11,
            gam_lut6::Gain11,
            GamLut6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut6::Gain10,
        gam_lut6::Gain10,
        GamLut6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut6::Gain10,
            gam_lut6::Gain10,
            GamLut6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut6 {
    #[inline(always)]
    fn default() -> GamLut6 {
        <crate::RegValueT<GamLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain11_SPEC;
    pub type Gain11 = crate::EnumBitfieldStruct<u8, Gain11_SPEC>;
    impl Gain11 {
        pub const GAIN_11: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain10_SPEC;
    pub type Gain10 = crate::EnumBitfieldStruct<u8, Gain10_SPEC>;
    impl Gain10 {
        pub const GAIN_10: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut7_SPEC;
impl crate::sealed::RegSpec for GamLut7_SPEC {
    type DataType = u32;
}

pub type GamLut7 = crate::RegValueT<GamLut7_SPEC>;

impl GamLut7 {
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut7::Gain13,
        gam_lut7::Gain13,
        GamLut7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut7::Gain13,
            gam_lut7::Gain13,
            GamLut7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain12(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut7::Gain12,
        gam_lut7::Gain12,
        GamLut7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut7::Gain12,
            gam_lut7::Gain12,
            GamLut7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut7 {
    #[inline(always)]
    fn default() -> GamLut7 {
        <crate::RegValueT<GamLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut7 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain13_SPEC;
    pub type Gain13 = crate::EnumBitfieldStruct<u8, Gain13_SPEC>;
    impl Gain13 {
        pub const GAIN_13: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain12_SPEC;
    pub type Gain12 = crate::EnumBitfieldStruct<u8, Gain12_SPEC>;
    impl Gain12 {
        pub const GAIN_12: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut8_SPEC;
impl crate::sealed::RegSpec for GamLut8_SPEC {
    type DataType = u32;
}

pub type GamLut8 = crate::RegValueT<GamLut8_SPEC>;

impl GamLut8 {
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut8::Gain15,
        gam_lut8::Gain15,
        GamLut8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut8::Gain15,
            gam_lut8::Gain15,
            GamLut8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut8::Gain14,
        gam_lut8::Gain14,
        GamLut8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut8::Gain14,
            gam_lut8::Gain14,
            GamLut8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, GamLut8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,GamLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut8 {
    #[inline(always)]
    fn default() -> GamLut8 {
        <crate::RegValueT<GamLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut8 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain15_SPEC;
    pub type Gain15 = crate::EnumBitfieldStruct<u8, Gain15_SPEC>;
    impl Gain15 {
        pub const GAIN_15: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain14_SPEC;
    pub type Gain14 = crate::EnumBitfieldStruct<u8, Gain14_SPEC>;
    impl Gain14 {
        pub const GAIN_14: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea1_SPEC;
impl crate::sealed::RegSpec for GamArea1_SPEC {
    type DataType = u32;
}

pub type GamArea1 = crate::RegValueT<GamArea1_SPEC>;

impl GamArea1 {
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea1 {
    #[inline(always)]
    fn default() -> GamArea1 {
        <crate::RegValueT<GamArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea2_SPEC;
impl crate::sealed::RegSpec for GamArea2_SPEC {
    type DataType = u32;
}

pub type GamArea2 = crate::RegValueT<GamArea2_SPEC>;

impl GamArea2 {
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea2 {
    #[inline(always)]
    fn default() -> GamArea2 {
        <crate::RegValueT<GamArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea3_SPEC;
impl crate::sealed::RegSpec for GamArea3_SPEC {
    type DataType = u32;
}

pub type GamArea3 = crate::RegValueT<GamArea3_SPEC>;

impl GamArea3 {
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea3 {
    #[inline(always)]
    fn default() -> GamArea3 {
        <crate::RegValueT<GamArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea4_SPEC;
impl crate::sealed::RegSpec for GamArea4_SPEC {
    type DataType = u32;
}

pub type GamArea4 = crate::RegValueT<GamArea4_SPEC>;

impl GamArea4 {
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea4 {
    #[inline(always)]
    fn default() -> GamArea4 {
        <crate::RegValueT<GamArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea5_SPEC;
impl crate::sealed::RegSpec for GamArea5_SPEC {
    type DataType = u32;
}

pub type GamArea5 = crate::RegValueT<GamArea5_SPEC>;

impl GamArea5 {
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GamArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, GamArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, GamArea5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, GamArea5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea5 {
    #[inline(always)]
    fn default() -> GamArea5 {
        <crate::RegValueT<GamArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutVlatch_SPEC;
impl crate::sealed::RegSpec for OutVlatch_SPEC {
    type DataType = u32;
}

pub type OutVlatch = crate::RegValueT<OutVlatch_SPEC>;

impl OutVlatch {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7fffffff,
        1,
        0,
        u32,
        u32,
        OutVlatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet_SPEC;
impl crate::sealed::RegSpec for OutSet_SPEC {
    type DataType = u32;
}

pub type OutSet = crate::RegValueT<OutSet_SPEC>;

impl OutSet {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, OutSet_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,OutSet_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _11: Self = Self::new(3);

        pub const _10: Self = Self::new(2);

        pub const _01: Self = Self::new(1);

        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirsel_SPEC;
    pub type Dirsel = crate::EnumBitfieldStruct<u8, Dirsel_SPEC>;
    impl Dirsel {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frqsel_SPEC;
    pub type Frqsel = crate::EnumBitfieldStruct<u8, Frqsel_SPEC>;
    impl Frqsel {
        pub const _11: Self = Self::new(3);

        pub const _10: Self = Self::new(2);

        pub const _01: Self = Self::new(1);

        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        pub const _11: Self = Self::new(3);

        pub const _10: Self = Self::new(2);

        pub const _01: Self = Self::new(1);

        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swapon_SPEC;
    pub type Swapon = crate::EnumBitfieldStruct<u8, Swapon_SPEC>;
    impl Swapon {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Endianon_SPEC;
    pub type Endianon = crate::EnumBitfieldStruct<u8, Endianon_SPEC>;
    impl Endianon {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBright1_SPEC;
impl crate::sealed::RegSpec for OutBright1_SPEC {
    type DataType = u32;
}

pub type OutBright1 = crate::RegValueT<OutBright1_SPEC>;

impl OutBright1 {
    #[inline(always)]
    pub fn brtg(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        u32,
        OutBright1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            u32,
            OutBright1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type OutBright2 = crate::RegValueT<OutBright2_SPEC>;

impl OutBright2 {
    #[inline(always)]
    pub fn brtr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn brtb(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, u8, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3f,1,0,u8,u8,OutBright2_SPEC,crate::common::RW>::from_register(self,0)
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

pub type OutContrast = crate::RegValueT<OutContrast_SPEC>;

impl OutContrast {
    #[inline(always)]
    pub fn contr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        out_contrast::Contr,
        out_contrast::Contr,
        OutContrast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            out_contrast::Contr,
            out_contrast::Contr,
            OutContrast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn contb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        out_contrast::Contb,
        out_contrast::Contb,
        OutContrast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            out_contrast::Contb,
            out_contrast::Contb,
            OutContrast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn contg(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        out_contrast::Contg,
        out_contrast::Contg,
        OutContrast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            out_contrast::Contg,
            out_contrast::Contg,
            OutContrast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, OutContrast_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutContrast {
    #[inline(always)]
    fn default() -> OutContrast {
        <crate::RegValueT<OutContrast_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_contrast {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Contr_SPEC;
    pub type Contr = crate::EnumBitfieldStruct<u8, Contr_SPEC>;
    impl Contr {
        pub const CONTR: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Contb_SPEC;
    pub type Contb = crate::EnumBitfieldStruct<u8, Contb_SPEC>;
    impl Contb {
        pub const CONTB: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Contg_SPEC;
    pub type Contg = crate::EnumBitfieldStruct<u8, Contg_SPEC>;
    impl Contg {
        pub const CONTG: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPdtha_SPEC;
impl crate::sealed::RegSpec for OutPdtha_SPEC {
    type DataType = u32;
}

pub type OutPdtha = crate::RegValueT<OutPdtha_SPEC>;

impl OutPdtha {
    #[inline(always)]
    pub fn pd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pb(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pa(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }

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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, u16, OutPdtha_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3ff,1,0,u16,u16,OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _11: Self = Self::new(3);

        pub const _10: Self = Self::new(2);

        pub const _01: Self = Self::new(1);

        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        pub const _11: Self = Self::new(3);

        pub const _10: Self = Self::new(2);

        pub const _01: Self = Self::new(1);

        pub const _00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClkphase_SPEC;
impl crate::sealed::RegSpec for OutClkphase_SPEC {
    type DataType = u32;
}

pub type OutClkphase = crate::RegValueT<OutClkphase_SPEC>;

impl OutClkphase {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, OutClkphase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon2Edge_SPEC;
    pub type Tcon2Edge = crate::EnumBitfieldStruct<u8, Tcon2Edge_SPEC>;
    impl Tcon2Edge {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon1Edge_SPEC;
    pub type Tcon1Edge = crate::EnumBitfieldStruct<u8, Tcon1Edge_SPEC>;
    impl Tcon1Edge {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon0Edge_SPEC;
    pub type Tcon0Edge = crate::EnumBitfieldStruct<u8, Tcon0Edge_SPEC>;
    impl Tcon0Edge {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdedge_SPEC;
    pub type Lcdedge = crate::EnumBitfieldStruct<u8, Lcdedge_SPEC>;
    impl Lcdedge {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frontgam_SPEC;
    pub type Frontgam = crate::EnumBitfieldStruct<u8, Frontgam_SPEC>;
    impl Frontgam {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconVlatch_SPEC;
impl crate::sealed::RegSpec for TconVlatch_SPEC {
    type DataType = u32;
}

pub type TconVlatch = crate::RegValueT<TconVlatch_SPEC>;

impl TconVlatch {
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TconVlatch_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TconVlatch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, TconVlatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,TconVlatch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconVlatch {
    #[inline(always)]
    fn default() -> TconVlatch {
        <crate::RegValueT<TconVlatch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconTim_SPEC;
impl crate::sealed::RegSpec for TconTim_SPEC {
    type DataType = u32;
}

pub type TconTim = crate::RegValueT<TconTim_SPEC>;

impl TconTim {
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        tcon_tim::Offset,
        tcon_tim::Offset,
        TconTim_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tcon_tim::Offset,
            tcon_tim::Offset,
            TconTim_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn half(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        tcon_tim::Half,
        tcon_tim::Half,
        TconTim_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            tcon_tim::Half,
            tcon_tim::Half,
            TconTim_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, TconTim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconTim {
    #[inline(always)]
    fn default() -> TconTim {
        <crate::RegValueT<TconTim_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_tim {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Offset_SPEC;
    pub type Offset = crate::EnumBitfieldStruct<u8, Offset_SPEC>;
    impl Offset {
        pub const OFFSET: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Half_SPEC;
    pub type Half = crate::EnumBitfieldStruct<u8, Half_SPEC>;
    impl Half {
        pub const HALF: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStv1_SPEC;
impl crate::sealed::RegSpec for TconStv1_SPEC {
    type DataType = u32;
}

pub type TconStv1 = crate::RegValueT<TconStv1_SPEC>;

impl TconStv1 {
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        tcon_stv1::Vw,
        tcon_stv1::Vw,
        TconStv1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tcon_stv1::Vw,
            tcon_stv1::Vw,
            TconStv1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        tcon_stv1::Vs,
        tcon_stv1::Vs,
        TconStv1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            tcon_stv1::Vs,
            tcon_stv1::Vs,
            TconStv1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, TconStv1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,TconStv1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStv1 {
    #[inline(always)]
    fn default() -> TconStv1 {
        <crate::RegValueT<TconStv1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stv1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vw_SPEC;
    pub type Vw = crate::EnumBitfieldStruct<u8, Vw_SPEC>;
    impl Vw {
        pub const VW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vs_SPEC;
    pub type Vs = crate::EnumBitfieldStruct<u8, Vs_SPEC>;
    impl Vs {
        pub const VS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStv2_SPEC;
impl crate::sealed::RegSpec for TconStv2_SPEC {
    type DataType = u32;
}

pub type TconStv2 = crate::RegValueT<TconStv2_SPEC>;

impl TconStv2 {
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_stv2::Sel,
        tcon_stv2::Sel,
        TconStv2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_stv2::Sel,
            tcon_stv2::Sel,
            TconStv2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_stv2::Inv,
        tcon_stv2::Inv,
        TconStv2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_stv2::Inv,
            tcon_stv2::Inv,
            TconStv2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, TconStv2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,TconStv2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStv2 {
    #[inline(always)]
    fn default() -> TconStv2 {
        <crate::RegValueT<TconStv2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stv2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        pub const _111: Self = Self::new(7);

        pub const _110: Self = Self::new(6);

        pub const _101: Self = Self::new(5);

        pub const _100: Self = Self::new(4);

        pub const _011: Self = Self::new(3);

        pub const _010: Self = Self::new(2);

        pub const _001: Self = Self::new(1);

        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSth1_SPEC;
impl crate::sealed::RegSpec for TconSth1_SPEC {
    type DataType = u32;
}

pub type TconSth1 = crate::RegValueT<TconSth1_SPEC>;

impl TconSth1 {
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        tcon_sth1::Hw,
        tcon_sth1::Hw,
        TconSth1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tcon_sth1::Hw,
            tcon_sth1::Hw,
            TconSth1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        tcon_sth1::Hs,
        tcon_sth1::Hs,
        TconSth1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            tcon_sth1::Hs,
            tcon_sth1::Hs,
            TconSth1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, TconSth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,TconSth1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSth1 {
    #[inline(always)]
    fn default() -> TconSth1 {
        <crate::RegValueT<TconSth1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_sth1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hw_SPEC;
    pub type Hw = crate::EnumBitfieldStruct<u8, Hw_SPEC>;
    impl Hw {
        pub const HW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hs_SPEC;
    pub type Hs = crate::EnumBitfieldStruct<u8, Hs_SPEC>;
    impl Hs {
        pub const HS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSth2_SPEC;
impl crate::sealed::RegSpec for TconSth2_SPEC {
    type DataType = u32;
}

pub type TconSth2 = crate::RegValueT<TconSth2_SPEC>;

impl TconSth2 {
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tcon_sth2::Sel,
        tcon_sth2::Sel,
        TconSth2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tcon_sth2::Sel,
            tcon_sth2::Sel,
            TconSth2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tcon_sth2::Inv,
        tcon_sth2::Inv,
        TconSth2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tcon_sth2::Inv,
            tcon_sth2::Inv,
            TconSth2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tcon_sth2::Hssel,
        tcon_sth2::Hssel,
        TconSth2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tcon_sth2::Hssel,
            tcon_sth2::Hssel,
            TconSth2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, TconSth2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,TconSth2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSth2 {
    #[inline(always)]
    fn default() -> TconSth2 {
        <crate::RegValueT<TconSth2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_sth2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        pub const _111: Self = Self::new(7);

        pub const _110: Self = Self::new(6);

        pub const _101: Self = Self::new(5);

        pub const _100: Self = Self::new(4);

        pub const _011: Self = Self::new(3);

        pub const _010: Self = Self::new(2);

        pub const _001: Self = Self::new(1);

        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hssel_SPEC;
    pub type Hssel = crate::EnumBitfieldStruct<u8, Hssel_SPEC>;
    impl Hssel {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconDe_SPEC;
impl crate::sealed::RegSpec for TconDe_SPEC {
    type DataType = u32;
}

pub type TconDe = crate::RegValueT<TconDe_SPEC>;

impl TconDe {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, TconDe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,TconDe_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntDtcten_SPEC;
impl crate::sealed::RegSpec for SyscntDtcten_SPEC {
    type DataType = u32;
}

pub type SyscntDtcten = crate::RegValueT<SyscntDtcten_SPEC>;

impl SyscntDtcten {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfdtc_SPEC;
    pub type L1Undfdtc = crate::EnumBitfieldStruct<u8, L1Undfdtc_SPEC>;
    impl L1Undfdtc {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfdtc_SPEC;
    pub type L2Undfdtc = crate::EnumBitfieldStruct<u8, L2Undfdtc_SPEC>;
    impl L2Undfdtc {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntInten_SPEC;
impl crate::sealed::RegSpec for SyscntInten_SPEC {
    type DataType = u32;
}

pub type SyscntInten = crate::RegValueT<SyscntInten_SPEC>;

impl SyscntInten {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfinten_SPEC;
    pub type L1Undfinten = crate::EnumBitfieldStruct<u8, L1Undfinten_SPEC>;
    impl L1Undfinten {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfinten_SPEC;
    pub type L2Undfinten = crate::EnumBitfieldStruct<u8, L2Undfinten_SPEC>;
    impl L2Undfinten {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStclr_SPEC;
impl crate::sealed::RegSpec for SyscntStclr_SPEC {
    type DataType = u32;
}

pub type SyscntStclr = crate::RegValueT<SyscntStclr_SPEC>;

impl SyscntStclr {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfclr_SPEC;
    pub type L1Undfclr = crate::EnumBitfieldStruct<u8, L1Undfclr_SPEC>;
    impl L1Undfclr {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfclr_SPEC;
    pub type L2Undfclr = crate::EnumBitfieldStruct<u8, L2Undfclr_SPEC>;
    impl L2Undfclr {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStmon_SPEC;
impl crate::sealed::RegSpec for SyscntStmon_SPEC {
    type DataType = u32;
}

pub type SyscntStmon = crate::RegValueT<SyscntStmon_SPEC>;

impl SyscntStmon {
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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undf_SPEC;
    pub type L1Undf = crate::EnumBitfieldStruct<u8, L1Undf_SPEC>;
    impl L1Undf {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undf_SPEC;
    pub type L2Undf = crate::EnumBitfieldStruct<u8, L2Undf_SPEC>;
    impl L2Undf {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntPanelClk_SPEC;
impl crate::sealed::RegSpec for SyscntPanelClk_SPEC {
    type DataType = u32;
}

pub type SyscntPanelClk = crate::RegValueT<SyscntPanelClk_SPEC>;

impl SyscntPanelClk {
    #[inline(always)]
    pub fn dcdr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }

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

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pixsel_SPEC;
    pub type Pixsel = crate::EnumBitfieldStruct<u8, Pixsel_SPEC>;
    impl Pixsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
