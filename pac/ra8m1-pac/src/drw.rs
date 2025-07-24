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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:06 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"2D Drawing Engine"]
unsafe impl ::core::marker::Send for super::Drw {}
unsafe impl ::core::marker::Sync for super::Drw {}
impl super::Drw {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Geometry Control Register"]
    #[inline(always)]
    pub const fn control(
        &self,
    ) -> &'static crate::common::Reg<self::Control_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Control_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Status Control Register"]
    #[inline(always)]
    pub const fn status(&self) -> &'static crate::common::Reg<self::Status_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Status_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Surface Control Register"]
    #[inline(always)]
    pub const fn control2(
        &self,
    ) -> &'static crate::common::Reg<self::Control2_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Control2_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Hardware Version and Feature Set ID Register"]
    #[inline(always)]
    pub const fn hwrevision(
        &self,
    ) -> &'static crate::common::Reg<self::Hwrevision_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Hwrevision_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Limiter %s Start Value Register"]
    #[inline(always)]
    pub const fn lstart(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lstart_SPEC, crate::common::W>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10usize))
        }
    }
    #[inline(always)]
    pub const fn l1start(
        &self,
    ) -> &'static crate::common::Reg<self::Lstart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lstart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l2start(
        &self,
    ) -> &'static crate::common::Reg<self::Lstart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lstart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l3start(
        &self,
    ) -> &'static crate::common::Reg<self::Lstart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lstart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l4start(
        &self,
    ) -> &'static crate::common::Reg<self::Lstart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lstart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn l5start(
        &self,
    ) -> &'static crate::common::Reg<self::Lstart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lstart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l6start(
        &self,
    ) -> &'static crate::common::Reg<self::Lstart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lstart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }

    #[doc = "Limiter %s X-Axis Increment Register"]
    #[inline(always)]
    pub const fn lxadd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lxadd_SPEC, crate::common::W>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x28usize))
        }
    }
    #[inline(always)]
    pub const fn l1xadd(&self) -> &'static crate::common::Reg<self::Lxadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lxadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l2xadd(&self) -> &'static crate::common::Reg<self::Lxadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lxadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn l3xadd(&self) -> &'static crate::common::Reg<self::Lxadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lxadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l4xadd(&self) -> &'static crate::common::Reg<self::Lxadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lxadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l5xadd(&self) -> &'static crate::common::Reg<self::Lxadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lxadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l6xadd(&self) -> &'static crate::common::Reg<self::Lxadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lxadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }

    #[doc = "Limiter %s Y-Axis Increment Register"]
    #[inline(always)]
    pub const fn lyadd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lyadd_SPEC, crate::common::W>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn l1yadd(&self) -> &'static crate::common::Reg<self::Lyadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lyadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l2yadd(&self) -> &'static crate::common::Reg<self::Lyadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lyadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l3yadd(&self) -> &'static crate::common::Reg<self::Lyadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lyadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l4yadd(&self) -> &'static crate::common::Reg<self::Lyadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lyadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn l5yadd(&self) -> &'static crate::common::Reg<self::Lyadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lyadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l6yadd(&self) -> &'static crate::common::Reg<self::Lyadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lyadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }

    #[doc = "Limiter %s Band Width Parameter Register"]
    #[inline(always)]
    pub const fn lband(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lband_SPEC, crate::common::W>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x58usize))
        }
    }
    #[inline(always)]
    pub const fn l1band(&self) -> &'static crate::common::Reg<self::Lband_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lband_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn l2band(&self) -> &'static crate::common::Reg<self::Lband_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lband_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x5cusize),
            )
        }
    }

    #[doc = "Base Color Register"]
    #[inline(always)]
    pub const fn color1(&self) -> &'static crate::common::Reg<self::Color1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Color1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Secondary Color Register"]
    #[inline(always)]
    pub const fn color2(&self) -> &'static crate::common::Reg<self::Color2_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Color2_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Pattern Register"]
    #[inline(always)]
    pub const fn pattern(
        &self,
    ) -> &'static crate::common::Reg<self::Pattern_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pattern_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Bounding Box Dimension Register"]
    #[inline(always)]
    pub const fn size(&self) -> &'static crate::common::Reg<self::Size_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Size_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Framebuffer Pitch And Spanstore Delay Register"]
    #[inline(always)]
    pub const fn pitch(&self) -> &'static crate::common::Reg<self::Pitch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pitch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Framebuffer Base Address Register"]
    #[inline(always)]
    pub const fn origin(&self) -> &'static crate::common::Reg<self::Origin_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Origin_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "U Limiter Start Value Register"]
    #[inline(always)]
    pub const fn lustart(
        &self,
    ) -> &'static crate::common::Reg<self::Lustart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lustart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "U Limiter X-Axis Increment Register"]
    #[inline(always)]
    pub const fn luxadd(&self) -> &'static crate::common::Reg<self::Luxadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Luxadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "U Limiter Y-Axis Increment Register"]
    #[inline(always)]
    pub const fn luyadd(&self) -> &'static crate::common::Reg<self::Luyadd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Luyadd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "V Limiter Start Value Integer Part Register"]
    #[inline(always)]
    pub const fn lvstarti(
        &self,
    ) -> &'static crate::common::Reg<self::Lvstarti_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lvstarti_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "V Limiter Start Value Fractional Part Register"]
    #[inline(always)]
    pub const fn lvstartf(
        &self,
    ) -> &'static crate::common::Reg<self::Lvstartf_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lvstartf_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "V Limiter X-Axis Increment Integer Part Register"]
    #[inline(always)]
    pub const fn lvxaddi(
        &self,
    ) -> &'static crate::common::Reg<self::Lvxaddi_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lvxaddi_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "V Limiter Y-Axis Increment Integer Part Register"]
    #[inline(always)]
    pub const fn lvyaddi(
        &self,
    ) -> &'static crate::common::Reg<self::Lvyaddi_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lvyaddi_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "V Limiter Increment Fractional Parts Register"]
    #[inline(always)]
    pub const fn lvyxaddf(
        &self,
    ) -> &'static crate::common::Reg<self::Lvyxaddf_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lvyxaddf_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "Texels Per Texture Line Register"]
    #[inline(always)]
    pub const fn texpitch(
        &self,
    ) -> &'static crate::common::Reg<self::Texpitch_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Texpitch_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[doc = "Texture Size or Texture Address Mask Register"]
    #[inline(always)]
    pub const fn texmask(
        &self,
    ) -> &'static crate::common::Reg<self::Texmask_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Texmask_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "Texture Base Address Register"]
    #[inline(always)]
    pub const fn texorigin(
        &self,
    ) -> &'static crate::common::Reg<self::Texorigin_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Texorigin_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[doc = "Interrupt Control Register"]
    #[inline(always)]
    pub const fn irqctl(&self) -> &'static crate::common::Reg<self::Irqctl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Irqctl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "Cache Control Register"]
    #[inline(always)]
    pub const fn cachectl(
        &self,
    ) -> &'static crate::common::Reg<self::Cachectl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cachectl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "Display List Start Address Register"]
    #[inline(always)]
    pub const fn dliststart(
        &self,
    ) -> &'static crate::common::Reg<self::Dliststart_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Dliststart_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "Performance Counter %s"]
    #[inline(always)]
    pub const fn perfcount(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Perfcount_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xccusize))
        }
    }
    #[inline(always)]
    pub const fn perfcount1(
        &self,
    ) -> &'static crate::common::Reg<self::Perfcount_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Perfcount_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xccusize),
            )
        }
    }
    #[inline(always)]
    pub const fn perfcount2(
        &self,
    ) -> &'static crate::common::Reg<self::Perfcount_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Perfcount_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }

    #[doc = "Performance Counters Control Register"]
    #[inline(always)]
    pub const fn perftrigger(
        &self,
    ) -> &'static crate::common::Reg<self::Perftrigger_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Perftrigger_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "CLUT Start Address Register"]
    #[inline(always)]
    pub const fn texcladdr(
        &self,
    ) -> &'static crate::common::Reg<self::Texcladdr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Texcladdr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[doc = "CLUT Data Register"]
    #[inline(always)]
    pub const fn texcldata(
        &self,
    ) -> &'static crate::common::Reg<self::Texcldata_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Texcldata_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "CLUT Offset Register"]
    #[inline(always)]
    pub const fn texcloffset(
        &self,
    ) -> &'static crate::common::Reg<self::Texcloffset_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Texcloffset_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "Color Key Register"]
    #[inline(always)]
    pub const fn colkey(&self) -> &'static crate::common::Reg<self::Colkey_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Colkey_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control_SPEC;
impl crate::sealed::RegSpec for Control_SPEC {
    type DataType = u32;
}

#[doc = "Geometry Control Register"]
pub type Control = crate::RegValueT<Control_SPEC>;

impl Control {
    #[doc = "Enable limiter 1"]
    #[inline(always)]
    pub fn lim1enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        control::Lim1Enable,
        control::Lim1Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            control::Lim1Enable,
            control::Lim1Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 2"]
    #[inline(always)]
    pub fn lim2enable(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        control::Lim2Enable,
        control::Lim2Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            control::Lim2Enable,
            control::Lim2Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 3"]
    #[inline(always)]
    pub fn lim3enable(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        control::Lim3Enable,
        control::Lim3Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            control::Lim3Enable,
            control::Lim3Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 4"]
    #[inline(always)]
    pub fn lim4enable(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        control::Lim4Enable,
        control::Lim4Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            control::Lim4Enable,
            control::Lim4Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 5"]
    #[inline(always)]
    pub fn lim5enable(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        control::Lim5Enable,
        control::Lim5Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            control::Lim5Enable,
            control::Lim5Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 6"]
    #[inline(always)]
    pub fn lim6enable(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        control::Lim6Enable,
        control::Lim6Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            control::Lim6Enable,
            control::Lim6Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable quadratic coupling of limiters 1 and 2"]
    #[inline(always)]
    pub fn quad1enable(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        control::Quad1Enable,
        control::Quad1Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            control::Quad1Enable,
            control::Quad1Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable quadratic coupling of limiters 3 and 4"]
    #[inline(always)]
    pub fn quad2enable(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        control::Quad2Enable,
        control::Quad2Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            control::Quad2Enable,
            control::Quad2Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable quadratic coupling of limiters 5 and 6"]
    #[inline(always)]
    pub fn quad3enable(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        control::Quad3Enable,
        control::Quad3Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            control::Quad3Enable,
            control::Quad3Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 1 threshold mode"]
    #[inline(always)]
    pub fn lim1threshold(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        control::Lim1Threshold,
        control::Lim1Threshold,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            control::Lim1Threshold,
            control::Lim1Threshold,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 2 threshold mode"]
    #[inline(always)]
    pub fn lim2threshold(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        control::Lim2Threshold,
        control::Lim2Threshold,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            control::Lim2Threshold,
            control::Lim2Threshold,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 3 threshold mode"]
    #[inline(always)]
    pub fn lim3threshold(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        control::Lim3Threshold,
        control::Lim3Threshold,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            control::Lim3Threshold,
            control::Lim3Threshold,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 4 threshold mode"]
    #[inline(always)]
    pub fn lim4threshold(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        control::Lim4Threshold,
        control::Lim4Threshold,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            control::Lim4Threshold,
            control::Lim4Threshold,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 5 threshold mode"]
    #[inline(always)]
    pub fn lim5threshold(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        control::Lim5Threshold,
        control::Lim5Threshold,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            control::Lim5Threshold,
            control::Lim5Threshold,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable limiter 6 threshold mode"]
    #[inline(always)]
    pub fn lim6threshold(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        control::Lim6Threshold,
        control::Lim6Threshold,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            control::Lim6Threshold,
            control::Lim6Threshold,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable band postprocess for limiter 1 (see L1BAND)"]
    #[inline(always)]
    pub fn band1enable(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        control::Band1Enable,
        control::Band1Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            control::Band1Enable,
            control::Band1Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Enable band postprocess for limiter 1 (see L1BAND)"]
    #[inline(always)]
    pub fn band2enable(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        control::Band2Enable,
        control::Band2Enable,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            control::Band2Enable,
            control::Band2Enable,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Combine limter 1 & 2 as union (output is called A)"]
    #[inline(always)]
    pub fn union12(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        control::Union12,
        control::Union12,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            control::Union12,
            control::Union12,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Combine limter 3 & 4 as union (output is called B)"]
    #[inline(always)]
    pub fn union34(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        control::Union34,
        control::Union34,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            control::Union34,
            control::Union34,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Combine limter 5 & 6 as union (output is called D)"]
    #[inline(always)]
    pub fn union56(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        control::Union56,
        control::Union56,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            control::Union56,
            control::Union56,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Combine outputs A & B as union (output is called C)"]
    #[inline(always)]
    pub fn unionab(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        control::Unionab,
        control::Unionab,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            control::Unionab,
            control::Unionab,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Combine outputs C & D as union (output is final)"]
    #[inline(always)]
    pub fn unioncd(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        control::Unioncd,
        control::Unioncd,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            control::Unioncd,
            control::Unioncd,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Shape is horizontally convex, only a single span per scanline"]
    #[inline(always)]
    pub fn spanabort(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        control::Spanabort,
        control::Spanabort,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            control::Spanabort,
            control::Spanabort,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Nextline span start is always equal or left to current-line span start"]
    #[inline(always)]
    pub fn spanstore(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        control::Spanstore,
        control::Spanstore,
        Control_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            control::Spanstore,
            control::Spanstore,
            Control_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Control_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Control_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Control {
    #[inline(always)]
    fn default() -> Control {
        <crate::RegValueT<Control_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod control {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim1Enable_SPEC;
    pub type Lim1Enable = crate::EnumBitfieldStruct<u8, Lim1Enable_SPEC>;
    impl Lim1Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim2Enable_SPEC;
    pub type Lim2Enable = crate::EnumBitfieldStruct<u8, Lim2Enable_SPEC>;
    impl Lim2Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim3Enable_SPEC;
    pub type Lim3Enable = crate::EnumBitfieldStruct<u8, Lim3Enable_SPEC>;
    impl Lim3Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim4Enable_SPEC;
    pub type Lim4Enable = crate::EnumBitfieldStruct<u8, Lim4Enable_SPEC>;
    impl Lim4Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim5Enable_SPEC;
    pub type Lim5Enable = crate::EnumBitfieldStruct<u8, Lim5Enable_SPEC>;
    impl Lim5Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim6Enable_SPEC;
    pub type Lim6Enable = crate::EnumBitfieldStruct<u8, Lim6Enable_SPEC>;
    impl Lim6Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Quad1Enable_SPEC;
    pub type Quad1Enable = crate::EnumBitfieldStruct<u8, Quad1Enable_SPEC>;
    impl Quad1Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Quad2Enable_SPEC;
    pub type Quad2Enable = crate::EnumBitfieldStruct<u8, Quad2Enable_SPEC>;
    impl Quad2Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Quad3Enable_SPEC;
    pub type Quad3Enable = crate::EnumBitfieldStruct<u8, Quad3Enable_SPEC>;
    impl Quad3Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim1Threshold_SPEC;
    pub type Lim1Threshold = crate::EnumBitfieldStruct<u8, Lim1Threshold_SPEC>;
    impl Lim1Threshold {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim2Threshold_SPEC;
    pub type Lim2Threshold = crate::EnumBitfieldStruct<u8, Lim2Threshold_SPEC>;
    impl Lim2Threshold {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim3Threshold_SPEC;
    pub type Lim3Threshold = crate::EnumBitfieldStruct<u8, Lim3Threshold_SPEC>;
    impl Lim3Threshold {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim4Threshold_SPEC;
    pub type Lim4Threshold = crate::EnumBitfieldStruct<u8, Lim4Threshold_SPEC>;
    impl Lim4Threshold {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim5Threshold_SPEC;
    pub type Lim5Threshold = crate::EnumBitfieldStruct<u8, Lim5Threshold_SPEC>;
    impl Lim5Threshold {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lim6Threshold_SPEC;
    pub type Lim6Threshold = crate::EnumBitfieldStruct<u8, Lim6Threshold_SPEC>;
    impl Lim6Threshold {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Band1Enable_SPEC;
    pub type Band1Enable = crate::EnumBitfieldStruct<u8, Band1Enable_SPEC>;
    impl Band1Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Band2Enable_SPEC;
    pub type Band2Enable = crate::EnumBitfieldStruct<u8, Band2Enable_SPEC>;
    impl Band2Enable {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Union12_SPEC;
    pub type Union12 = crate::EnumBitfieldStruct<u8, Union12_SPEC>;
    impl Union12 {
        #[doc = "minimum/intersect"]
        pub const _0: Self = Self::new(0);

        #[doc = "maximum/union"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Union34_SPEC;
    pub type Union34 = crate::EnumBitfieldStruct<u8, Union34_SPEC>;
    impl Union34 {
        #[doc = "minimum/intersect"]
        pub const _0: Self = Self::new(0);

        #[doc = "maximum/union"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Union56_SPEC;
    pub type Union56 = crate::EnumBitfieldStruct<u8, Union56_SPEC>;
    impl Union56 {
        #[doc = "minimum/intersect"]
        pub const _0: Self = Self::new(0);

        #[doc = "maximum/union"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Unionab_SPEC;
    pub type Unionab = crate::EnumBitfieldStruct<u8, Unionab_SPEC>;
    impl Unionab {
        #[doc = "minimum/intersect"]
        pub const _0: Self = Self::new(0);

        #[doc = "maximum/union"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Unioncd_SPEC;
    pub type Unioncd = crate::EnumBitfieldStruct<u8, Unioncd_SPEC>;
    impl Unioncd {
        #[doc = "minimum/intersect"]
        pub const _0: Self = Self::new(0);

        #[doc = "maximum/union"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spanabort_SPEC;
    pub type Spanabort = crate::EnumBitfieldStruct<u8, Spanabort_SPEC>;
    impl Spanabort {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spanstore_SPEC;
    pub type Spanstore = crate::EnumBitfieldStruct<u8, Spanstore_SPEC>;
    impl Spanstore {
        #[doc = "disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status_SPEC;
impl crate::sealed::RegSpec for Status_SPEC {
    type DataType = u32;
}

#[doc = "Status Control Register"]
pub type Status = crate::RegValueT<Status_SPEC>;

impl Status {
    #[doc = "Enumeration unit status"]
    #[inline(always)]
    pub fn busyenum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        status::Busyenum,
        status::Busyenum,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            status::Busyenum,
            status::Busyenum,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framebuffer writeback status"]
    #[inline(always)]
    pub fn busywrite(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        status::Busywrite,
        status::Busywrite,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            status::Busywrite,
            status::Busywrite,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framebuffer cache status"]
    #[inline(always)]
    pub fn cachedirty(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        status::Cachedirty,
        status::Cachedirty,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            status::Cachedirty,
            status::Cachedirty,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Display list reader status"]
    #[inline(always)]
    pub fn dlistactive(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        status::Dlistactive,
        status::Dlistactive,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            status::Dlistactive,
            status::Dlistactive,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "enumeration finished interrupt triggered"]
    #[inline(always)]
    pub fn enumirq(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        status::Enumirq,
        status::Enumirq,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            status::Enumirq,
            status::Enumirq,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "display list finished interrupt triggered"]
    #[inline(always)]
    pub fn dlistirq(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        status::Dlistirq,
        status::Dlistirq,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            status::Dlistirq,
            status::Dlistirq,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "bus error interrupt triggered"]
    #[inline(always)]
    pub fn busirq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        status::Busirq,
        status::Busirq,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            status::Busirq,
            status::Busirq,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "framebuffer bus error interrupt triggered"]
    #[inline(always)]
    pub fn buserrmfb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        status::Buserrmfb,
        status::Buserrmfb,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            status::Buserrmfb,
            status::Buserrmfb,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "texture bus error interrupt triggered"]
    #[inline(always)]
    pub fn buserrmtxmrl(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        status::Buserrmtxmrl,
        status::Buserrmtxmrl,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            status::Buserrmtxmrl,
            status::Buserrmtxmrl,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "display list bus error interrupt triggered"]
    #[inline(always)]
    pub fn buserrmdl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        status::Buserrmdl,
        status::Buserrmdl,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            status::Buserrmdl,
            status::Buserrmdl,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Status_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Status_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Status {
    #[inline(always)]
    fn default() -> Status {
        <crate::RegValueT<Status_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod status {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busyenum_SPEC;
    pub type Busyenum = crate::EnumBitfieldStruct<u8, Busyenum_SPEC>;
    impl Busyenum {
        #[doc = "enumeration unit idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "enumeration unit busy, new primitive can not be started"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busywrite_SPEC;
    pub type Busywrite = crate::EnumBitfieldStruct<u8, Busywrite_SPEC>;
    impl Busywrite {
        #[doc = "framebuffer writeback finished"]
        pub const _0: Self = Self::new(0);

        #[doc = "framebuffer writeback busy, framebuffer type can not be changed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cachedirty_SPEC;
    pub type Cachedirty = crate::EnumBitfieldStruct<u8, Cachedirty_SPEC>;
    impl Cachedirty {
        #[doc = "framebuffer cache is not dirty"]
        pub const _0: Self = Self::new(0);

        #[doc = "framebuffer cache is dirty, frame should not be flipped"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlistactive_SPEC;
    pub type Dlistactive = crate::EnumBitfieldStruct<u8, Dlistactive_SPEC>;
    impl Dlistactive {
        #[doc = "display list reader is idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "display list reader busy, no direct write access to registers allowed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enumirq_SPEC;
    pub type Enumirq = crate::EnumBitfieldStruct<u8, Enumirq_SPEC>;
    impl Enumirq {
        #[doc = "enumeration not finished or interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "enumeration finished interrupt triggered"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlistirq_SPEC;
    pub type Dlistirq = crate::EnumBitfieldStruct<u8, Dlistirq_SPEC>;
    impl Dlistirq {
        #[doc = "display list not finished or interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "display list finished interrupt triggered"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busirq_SPEC;
    pub type Busirq = crate::EnumBitfieldStruct<u8, Busirq_SPEC>;
    impl Busirq {
        #[doc = "no bus error occurred or interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "bus error interrupt triggered"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buserrmfb_SPEC;
    pub type Buserrmfb = crate::EnumBitfieldStruct<u8, Buserrmfb_SPEC>;
    impl Buserrmfb {
        #[doc = "no framebuffer bus error occured or interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "framebuffer bus error interrupt triggered"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buserrmtxmrl_SPEC;
    pub type Buserrmtxmrl = crate::EnumBitfieldStruct<u8, Buserrmtxmrl_SPEC>;
    impl Buserrmtxmrl {
        #[doc = "no texture bus error occurred or interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "texture bus error interrupt triggered"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buserrmdl_SPEC;
    pub type Buserrmdl = crate::EnumBitfieldStruct<u8, Buserrmdl_SPEC>;
    impl Buserrmdl {
        #[doc = "no display list bus error occurred or interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "display list bus error interrupt triggered"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control2_SPEC;
impl crate::sealed::RegSpec for Control2_SPEC {
    type DataType = u32;
}

#[doc = "Surface Control Register"]
pub type Control2 = crate::RegValueT<Control2_SPEC>;

impl Control2 {
    #[doc = "Pixel source is a pattern color (blend of COLOR1 and COLOR2 depending on PATTERN and pattern index)"]
    #[inline(always)]
    pub fn patternenable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        control2::Patternenable,
        control2::Patternenable,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            control2::Patternenable,
            control2::Patternenable,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pixel source is read from texture and used as an alpha to blend between COLOR1 and COLOR2"]
    #[inline(always)]
    pub fn textureenable(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        control2::Textureenable,
        control2::Textureenable,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            control2::Textureenable,
            control2::Textureenable,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Limiter 5 is used as pattern index instead of the default U limiter.Limiter 5 can be combined with limiter 6 to form a quadratic limiter which can be used to make quadratic pattern functions to draw radial patterns."]
    #[inline(always)]
    pub fn patternsourcel5(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Control2_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Alpha blend mode"]
    #[inline(always)]
    pub fn useacb(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        control2::Useacb,
        control2::Useacb,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            control2::Useacb,
            control2::Useacb,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Bit 4 and 3 of the texture buffer format.See READFORMAT above for description"]
    #[inline(always)]
    pub fn readformat32(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Control2_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Control2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Blend source factor  for alpha channel in alpha channel blending mode (USEACB = 1)"]
    #[inline(always)]
    pub fn bsfa(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        control2::Bsfa,
        control2::Bsfa,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            control2::Bsfa,
            control2::Bsfa,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Blend destinetion factor  for alpha channel in alpha channel blending mode (USEACB = 1)"]
    #[inline(always)]
    pub fn bdfa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        control2::Bdfa,
        control2::Bdfa,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            control2::Bdfa,
            control2::Bdfa,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Control2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Control2_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Blend source factorsrc factor is alpha (factor is 1 per default)"]
    #[inline(always)]
    pub fn bsf(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        control2::Bsf,
        control2::Bsf,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            control2::Bsf,
            control2::Bsf,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Blend destination factordst factor is alpha (factor is 1 per default)"]
    #[inline(always)]
    pub fn bdf(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        control2::Bdf,
        control2::Bdf,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            control2::Bdf,
            control2::Bdf,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Blend source factor is invertedsrc factor will be inverted (meaning 1-a or 1-1 depending on BSF)"]
    #[inline(always)]
    pub fn bsi(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        control2::Bsi,
        control2::Bsi,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            control2::Bsi,
            control2::Bsi,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Blend destination factor is inverteddst factor will be inverted (meaning 1-a or 1-1 depending on BDF)"]
    #[inline(always)]
    pub fn bdi(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        control2::Bdi,
        control2::Bdi,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            control2::Bdi,
            control2::Bdi,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Blend color 2 instead of framebuffer pixel"]
    #[inline(always)]
    pub fn bc2(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        control2::Bc2,
        control2::Bc2,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            control2::Bc2,
            control2::Bc2,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Calculating U limiter outside use textureThe bit describes what happens if the U limiter (x direction in texture space) calculates a U value outside of the used texture"]
    #[inline(always)]
    pub fn textureclampx(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        control2::Textureclampx,
        control2::Textureclampx,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            control2::Textureclampx,
            control2::Textureclampx,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Calculating V limiter outside use textureThe bit describes what happens if the V limiter (y direction in texture space) calculates a V value outside of the used texture"]
    #[inline(always)]
    pub fn textureclampy(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        control2::Textureclampy,
        control2::Textureclampy,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            control2::Textureclampy,
            control2::Textureclampy,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Linear filtering on texture U axis"]
    #[inline(always)]
    pub fn texturefilterx(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        control2::Texturefilterx,
        control2::Texturefilterx,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            control2::Texturefilterx,
            control2::Texturefilterx,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Linear filtering on texture V axis"]
    #[inline(always)]
    pub fn texturefiltery(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        control2::Texturefiltery,
        control2::Texturefiltery,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            control2::Texturefiltery,
            control2::Texturefiltery,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pixel format of the texture buffer{READFORMAT32,READFORMAT10}0000: 8 bpp a(8)0001: 16 bpp RGB(565)0010: 32 bpp aRGB(8888)0011: 16 bpp aRGB(4444)0100: 16 bpp aRGB(1555)0101: 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color1001: 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance1010: 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance1011: 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance 1100: 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance"]
    #[inline(always)]
    pub fn readformat10(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        control2::Readformat10,
        control2::Readformat10,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            control2::Readformat10,
            control2::Readformat10,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pixel format of the framebuffer"]
    #[inline(always)]
    pub fn writeformat10(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        control2::Writeformat10,
        control2::Writeformat10,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            control2::Writeformat10,
            control2::Writeformat10,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Writeback alpha source for framebufferSet the \'alpha source\' for the framebuffer(USEACB = 0)Blend alpha in color 2 instead of framebuffer alpha((USEACB = 1))In not alpha channel blending mode (USEACB = 0):Set the \'alpha source\' for the framebuffer.In alpha channel blending mode (USEACB = 1):Blend alpha in color 2 instead of framebuffer alpha00B:  BC2A = 1: use alpha from framebuffer as destination (DST_A)else: BC2A = 0: use alpha in color 2 as destination (DST_A)"]
    #[inline(always)]
    pub fn writealpha(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        control2::Writealpha,
        control2::Writealpha,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            control2::Writealpha,
            control2::Writealpha,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "RLE enable"]
    #[inline(always)]
    pub fn rleenable(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        control2::Rleenable,
        control2::Rleenable,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            control2::Rleenable,
            control2::Rleenable,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "CLUT enable"]
    #[inline(always)]
    pub fn clutenable(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        control2::Clutenable,
        control2::Clutenable,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            control2::Clutenable,
            control2::Clutenable,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "color keying enable"]
    #[inline(always)]
    pub fn colkeyenable(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        control2::Colkeyenable,
        control2::Colkeyenable,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            control2::Colkeyenable,
            control2::Colkeyenable,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Format of the CLUT"]
    #[inline(always)]
    pub fn clutformat(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        control2::Clutformat,
        control2::Clutformat,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            control2::Clutformat,
            control2::Clutformat,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Blend source factor inverted in alpha channel (USEACB = 1)"]
    #[inline(always)]
    pub fn bsia(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        control2::Bsia,
        control2::Bsia,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            control2::Bsia,
            control2::Bsia,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Blend destination factor inverted in alpha channel (USEACB = 1)"]
    #[inline(always)]
    pub fn bdia(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        control2::Bdia,
        control2::Bdia,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            control2::Bdia,
            control2::Bdia,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Texel width for RLE unit"]
    #[inline(always)]
    pub fn rlepixelwidth(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        control2::Rlepixelwidth,
        control2::Rlepixelwidth,
        Control2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            control2::Rlepixelwidth,
            control2::Rlepixelwidth,
            Control2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Control2 {
    #[inline(always)]
    fn default() -> Control2 {
        <crate::RegValueT<Control2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod control2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Patternenable_SPEC;
    pub type Patternenable = crate::EnumBitfieldStruct<u8, Patternenable_SPEC>;
    impl Patternenable {
        #[doc = "disabled pattern"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled pattern"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Textureenable_SPEC;
    pub type Textureenable = crate::EnumBitfieldStruct<u8, Textureenable_SPEC>;
    impl Textureenable {
        #[doc = "disabled texture"]
        pub const _0: Self = Self::new(0);

        #[doc = "enabled texture"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Useacb_SPEC;
    pub type Useacb = crate::EnumBitfieldStruct<u8, Useacb_SPEC>;
    impl Useacb {
        #[doc = "use WRITEALPHA\\[1:0\\] mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "use full alpha channel blending mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsfa_SPEC;
    pub type Bsfa = crate::EnumBitfieldStruct<u8, Bsfa_SPEC>;
    impl Bsfa {
        #[doc = "use 1.0 as blend source factor for alpha channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "use alpha as blend source factor for alpha channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bdfa_SPEC;
    pub type Bdfa = crate::EnumBitfieldStruct<u8, Bdfa_SPEC>;
    impl Bdfa {
        #[doc = "use 1.0 as blend destination factor for alpha channel"]
        pub const _0: Self = Self::new(0);

        #[doc = "use alpha as blend destination factor for alpha channel"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsf_SPEC;
    pub type Bsf = crate::EnumBitfieldStruct<u8, Bsf_SPEC>;
    impl Bsf {
        #[doc = "use 1.0 as blend source factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "use alpha as blend source factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bdf_SPEC;
    pub type Bdf = crate::EnumBitfieldStruct<u8, Bdf_SPEC>;
    impl Bdf {
        #[doc = "use 1.0 as blend destination factor"]
        pub const _0: Self = Self::new(0);

        #[doc = "use alpha as blend destination factor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsi_SPEC;
    pub type Bsi = crate::EnumBitfieldStruct<u8, Bsi_SPEC>;
    impl Bsi {
        #[doc = "use blend factor as specified through BSF"]
        pub const _0: Self = Self::new(0);

        #[doc = "invert blend source factor (1-x)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bdi_SPEC;
    pub type Bdi = crate::EnumBitfieldStruct<u8, Bdi_SPEC>;
    impl Bdi {
        #[doc = "use blend factor as specified through BDF"]
        pub const _0: Self = Self::new(0);

        #[doc = "invert blend destinationfactor (1-x)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bc2_SPEC;
    pub type Bc2 = crate::EnumBitfieldStruct<u8, Bc2_SPEC>;
    impl Bc2 {
        #[doc = "use pixel from framebuffer as destination (DST)"]
        pub const _0: Self = Self::new(0);

        #[doc = "use color 2 as destination (DST)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Textureclampx_SPEC;
    pub type Textureclampx = crate::EnumBitfieldStruct<u8, Textureclampx_SPEC>;
    impl Textureclampx {
        #[doc = "Texture wrap mode: The integer part of the calculated value from the u limiter is anded with      TEXUMASK. This results in a repetition of the texture in x/u direction."]
        pub const _0: Self = Self::new(0);

        #[doc = "Texture clamp mode: The texture color at the border of the texture is taken. This results in a     repetition of the texture border color in x/u direction."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Textureclampy_SPEC;
    pub type Textureclampy = crate::EnumBitfieldStruct<u8, Textureclampy_SPEC>;
    impl Textureclampy {
        #[doc = "Texture wrap mode:    The integer part of the calculated value from the v limiter is anded with TEXVMASK. This results  in a  repetition of the texture in y/v direction."]
        pub const _0: Self = Self::new(0);

        #[doc = "Texture clamp mode:    The texture color at the border of the texture is taken. This results in a repetition of the texture  border  color in y/v direction."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Texturefilterx_SPEC;
    pub type Texturefilterx = crate::EnumBitfieldStruct<u8, Texturefilterx_SPEC>;
    impl Texturefilterx {
        #[doc = "no filtering on texture U axis"]
        pub const _0: Self = Self::new(0);

        #[doc = "linear filtering on texture U axis"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Texturefiltery_SPEC;
    pub type Texturefiltery = crate::EnumBitfieldStruct<u8, Texturefiltery_SPEC>;
    impl Texturefiltery {
        #[doc = "no filtering on texture V axis"]
        pub const _0: Self = Self::new(0);

        #[doc = "linear filtering on texture V axis"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Readformat10_SPEC;
    pub type Readformat10 = crate::EnumBitfieldStruct<u8, Readformat10_SPEC>;
    impl Readformat10 {
        #[doc = "8 bpp a(8) (READFORMAT32=00) / 16 bpp aRGB(1555)  (READFORMAT32=01) / 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance  (READFORMAT32=11)"]
        pub const _00: Self = Self::new(0);

        #[doc = "16 bpp RGB(565)  (READFORMAT32=00) / 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color  (READFORMAT32=01) / 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance  (READFORMAT32=10)"]
        pub const _01: Self = Self::new(1);

        #[doc = "32 bpp aRGB(8888) (READFORMAT32=00)  / 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance (READFORMAT32=10)"]
        pub const _10: Self = Self::new(2);

        #[doc = "16 bpp aRGB(4444)  (READFORMAT32=00) / 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance  (READFORMAT32=10)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Writeformat10_SPEC;
    pub type Writeformat10 = crate::EnumBitfieldStruct<u8, Writeformat10_SPEC>;
    impl Writeformat10 {
        #[doc = "8bpp a(8)0"]
        pub const _00: Self = Self::new(0);

        #[doc = "16bpp RGB(565)"]
        pub const _01: Self = Self::new(1);

        #[doc = "32bpp aRGB(8888)"]
        pub const _10: Self = Self::new(2);

        #[doc = "16bpp aRGB(4444)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Writealpha_SPEC;
    pub type Writealpha = crate::EnumBitfieldStruct<u8, Writealpha_SPEC>;
    impl Writealpha {
        #[doc = "use alpha from color 2"]
        pub const _00: Self = Self::new(0);

        #[doc = "use source alpha (pixel coverage)"]
        pub const _01: Self = Self::new(1);

        #[doc = "use 0.0 as alpha"]
        pub const _10: Self = Self::new(2);

        #[doc = "use alpha from framebuffer"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rleenable_SPEC;
    pub type Rleenable = crate::EnumBitfieldStruct<u8, Rleenable_SPEC>;
    impl Rleenable {
        #[doc = "RLE disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "RLE enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clutenable_SPEC;
    pub type Clutenable = crate::EnumBitfieldStruct<u8, Clutenable_SPEC>;
    impl Clutenable {
        #[doc = "CLUT disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CLUT enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Colkeyenable_SPEC;
    pub type Colkeyenable = crate::EnumBitfieldStruct<u8, Colkeyenable_SPEC>;
    impl Colkeyenable {
        #[doc = "color keying disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "color keying enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clutformat_SPEC;
    pub type Clutformat = crate::EnumBitfieldStruct<u8, Clutformat_SPEC>;
    impl Clutformat {
        #[doc = "aRGB(8888)"]
        pub const _0: Self = Self::new(0);

        #[doc = "RGB(565)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsia_SPEC;
    pub type Bsia = crate::EnumBitfieldStruct<u8, Bsia_SPEC>;
    impl Bsia {
        #[doc = "use blend factor as specified through BSFA"]
        pub const _0: Self = Self::new(0);

        #[doc = "invert blend source factor (1-x)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bdia_SPEC;
    pub type Bdia = crate::EnumBitfieldStruct<u8, Bdia_SPEC>;
    impl Bdia {
        #[doc = "use blend factor as specified through BDFA"]
        pub const _0: Self = Self::new(0);

        #[doc = "invert blend destination factor (1-x)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rlepixelwidth_SPEC;
    pub type Rlepixelwidth = crate::EnumBitfieldStruct<u8, Rlepixelwidth_SPEC>;
    impl Rlepixelwidth {
        #[doc = "1 byte per texel"]
        pub const _00: Self = Self::new(0);

        #[doc = "2 byte per texel"]
        pub const _01: Self = Self::new(1);

        #[doc = "3 byte per texel"]
        pub const _10: Self = Self::new(2);

        #[doc = "4 byte per texel"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwrevision_SPEC;
impl crate::sealed::RegSpec for Hwrevision_SPEC {
    type DataType = u32;
}

#[doc = "Hardware Version and Feature Set ID Register"]
pub type Hwrevision = crate::RegValueT<Hwrevision_SPEC>;

impl Hwrevision {
    #[doc = "Revision number"]
    #[inline(always)]
    pub fn rev(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Hwrevision_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Display list reader feature"]
    #[inline(always)]
    pub fn dlr(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        hwrevision::Dlr,
        hwrevision::Dlr,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            hwrevision::Dlr,
            hwrevision::Dlr,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framebuffer cache feature"]
    #[inline(always)]
    pub fn fbcache(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        hwrevision::Fbcache,
        hwrevision::Fbcache,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            hwrevision::Fbcache,
            hwrevision::Fbcache,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Texture cache feature"]
    #[inline(always)]
    pub fn txcache(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        hwrevision::Txcache,
        hwrevision::Txcache,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            hwrevision::Txcache,
            hwrevision::Txcache,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Two performance counter feature"]
    #[inline(always)]
    pub fn perfcount(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        hwrevision::Perfcount,
        hwrevision::Perfcount,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            hwrevision::Perfcount,
            hwrevision::Perfcount,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Texture CLUT with 16 or 256 entries feature"]
    #[inline(always)]
    pub fn texclu(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        hwrevision::Texclu,
        hwrevision::Texclu,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            hwrevision::Texclu,
            hwrevision::Texclu,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RLE unit feature"]
    #[inline(always)]
    pub fn rleunit(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        hwrevision::Rleunit,
        hwrevision::Rleunit,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            hwrevision::Rleunit,
            hwrevision::Rleunit,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Texture CLUT feature"]
    #[inline(always)]
    pub fn texclut256(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        hwrevision::Texclut256,
        hwrevision::Texclut256,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            hwrevision::Texclut256,
            hwrevision::Texclut256,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Colorkey feature"]
    #[inline(always)]
    pub fn colorkey(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        hwrevision::Colorkey,
        hwrevision::Colorkey,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            hwrevision::Colorkey,
            hwrevision::Colorkey,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Alpha channel blending feature"]
    #[inline(always)]
    pub fn acblend(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        hwrevision::Acblend,
        hwrevision::Acblend,
        Hwrevision_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            hwrevision::Acblend,
            hwrevision::Acblend,
            Hwrevision_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Hwrevision_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Hwrevision_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Hwrevision {
    #[inline(always)]
    fn default() -> Hwrevision {
        <crate::RegValueT<Hwrevision_SPEC> as RegisterValue<_>>::new(264110080)
    }
}
pub mod hwrevision {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlr_SPEC;
    pub type Dlr = crate::EnumBitfieldStruct<u8, Dlr_SPEC>;
    impl Dlr {
        #[doc = "Display list reader unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Display list reader available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fbcache_SPEC;
    pub type Fbcache = crate::EnumBitfieldStruct<u8, Fbcache_SPEC>;
    impl Fbcache {
        #[doc = "Framebuffer cache unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Framebuffer cache available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txcache_SPEC;
    pub type Txcache = crate::EnumBitfieldStruct<u8, Txcache_SPEC>;
    impl Txcache {
        #[doc = "Texture cache unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Texture cache available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perfcount_SPEC;
    pub type Perfcount = crate::EnumBitfieldStruct<u8, Perfcount_SPEC>;
    impl Perfcount {
        #[doc = "Two performance counter unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Two performance counter available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Texclu_SPEC;
    pub type Texclu = crate::EnumBitfieldStruct<u8, Texclu_SPEC>;
    impl Texclu {
        #[doc = "Texture CLUT with 16 or 256 entries unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Texture CLUT with 16 or 256 entries available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rleunit_SPEC;
    pub type Rleunit = crate::EnumBitfieldStruct<u8, Rleunit_SPEC>;
    impl Rleunit {
        #[doc = "RLE unit unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "RLE unit available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Texclut256_SPEC;
    pub type Texclut256 = crate::EnumBitfieldStruct<u8, Texclut256_SPEC>;
    impl Texclut256 {
        #[doc = "Texture CLUT unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Texture CLUT available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Colorkey_SPEC;
    pub type Colorkey = crate::EnumBitfieldStruct<u8, Colorkey_SPEC>;
    impl Colorkey {
        #[doc = "Colorkey unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Colorkey available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acblend_SPEC;
    pub type Acblend = crate::EnumBitfieldStruct<u8, Acblend_SPEC>;
    impl Acblend {
        #[doc = "Alpha channel blending unavailable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Alpha channel blending available"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lstart_SPEC;
impl crate::sealed::RegSpec for Lstart_SPEC {
    type DataType = u32;
}

#[doc = "Limiter %s Start Value Register"]
pub type Lstart = crate::RegValueT<Lstart_SPEC>;

impl Lstart {
    #[doc = "Start value of the n\'th limiter(n=1-6)"]
    #[inline(always)]
    pub fn lstart(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lstart_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lstart_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lstart {
    #[inline(always)]
    fn default() -> Lstart {
        <crate::RegValueT<Lstart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lxadd_SPEC;
impl crate::sealed::RegSpec for Lxadd_SPEC {
    type DataType = u32;
}

#[doc = "Limiter %s X-Axis Increment Register"]
pub type Lxadd = crate::RegValueT<Lxadd_SPEC>;

impl Lxadd {
    #[doc = "X-axis increment"]
    #[inline(always)]
    pub fn lxadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lxadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lxadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lxadd {
    #[inline(always)]
    fn default() -> Lxadd {
        <crate::RegValueT<Lxadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lyadd_SPEC;
impl crate::sealed::RegSpec for Lyadd_SPEC {
    type DataType = u32;
}

#[doc = "Limiter %s Y-Axis Increment Register"]
pub type Lyadd = crate::RegValueT<Lyadd_SPEC>;

impl Lyadd {
    #[doc = "Y-axis increment"]
    #[inline(always)]
    pub fn lyadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lyadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lyadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lyadd {
    #[inline(always)]
    fn default() -> Lyadd {
        <crate::RegValueT<Lyadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lband_SPEC;
impl crate::sealed::RegSpec for Lband_SPEC {
    type DataType = u32;
}

#[doc = "Limiter %s Band Width Parameter Register"]
pub type Lband = crate::RegValueT<Lband_SPEC>;

impl Lband {
    #[doc = "Limiter m band width parameter"]
    #[inline(always)]
    pub fn lband(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lband_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lband_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lband {
    #[inline(always)]
    fn default() -> Lband {
        <crate::RegValueT<Lband_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color1_SPEC;
impl crate::sealed::RegSpec for Color1_SPEC {
    type DataType = u32;
}

#[doc = "Base Color Register"]
pub type Color1 = crate::RegValueT<Color1_SPEC>;

impl Color1 {
    #[doc = "Blue channel of color 1"]
    #[inline(always)]
    pub fn color1b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Color1_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Color1_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Green channel of color 1"]
    #[inline(always)]
    pub fn color1g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Color1_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Color1_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Red channel of color 1"]
    #[inline(always)]
    pub fn color1r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Color1_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Color1_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Alpha channel of color 1(0x00: transparent. . .  0xFF: opaque)"]
    #[inline(always)]
    pub fn color1a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Color1_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Color1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Color1 {
    #[inline(always)]
    fn default() -> Color1 {
        <crate::RegValueT<Color1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color2_SPEC;
impl crate::sealed::RegSpec for Color2_SPEC {
    type DataType = u32;
}

#[doc = "Secondary Color Register"]
pub type Color2 = crate::RegValueT<Color2_SPEC>;

impl Color2 {
    #[doc = "Blue channel of color 2"]
    #[inline(always)]
    pub fn color2b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Color2_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Color2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Green channel of color 2"]
    #[inline(always)]
    pub fn color2g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Color2_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Color2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Red channel of color 2"]
    #[inline(always)]
    pub fn color2r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Color2_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Color2_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Alpha channel of color 2(0x00: transparent. . .   0xFF: opaque)"]
    #[inline(always)]
    pub fn color2a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Color2_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Color2_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Color2 {
    #[inline(always)]
    fn default() -> Color2 {
        <crate::RegValueT<Color2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pattern_SPEC;
impl crate::sealed::RegSpec for Pattern_SPEC {
    type DataType = u32;
}

#[doc = "Pattern Register"]
pub type Pattern = crate::RegValueT<Pattern_SPEC>;

impl Pattern {
    #[doc = "Bitmap of the pattern"]
    #[inline(always)]
    pub fn pattern(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Pattern_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Pattern_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Pattern_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Pattern_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Pattern {
    #[inline(always)]
    fn default() -> Pattern {
        <crate::RegValueT<Pattern_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Size_SPEC;
impl crate::sealed::RegSpec for Size_SPEC {
    type DataType = u32;
}

#[doc = "Bounding Box Dimension Register"]
pub type Size = crate::RegValueT<Size_SPEC>;

impl Size {
    #[doc = "Width of the bounding box in pixelsvalid range: 0 to 1024"]
    #[inline(always)]
    pub fn sizex(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Size_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Size_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Height of the bounding box in pixelsvalid range: 0 to 1024"]
    #[inline(always)]
    pub fn sizey(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Size_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Size_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Size {
    #[inline(always)]
    fn default() -> Size {
        <crate::RegValueT<Size_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pitch_SPEC;
impl crate::sealed::RegSpec for Pitch_SPEC {
    type DataType = u32;
}

#[doc = "Framebuffer Pitch And Spanstore Delay Register"]
pub type Pitch = crate::RegValueT<Pitch_SPEC>;

impl Pitch {
    #[doc = "pitch of the framebuffer. A negative width can be used to render bottom-up instead of top-down"]
    #[inline(always)]
    pub fn pitch(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Pitch_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pitch_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Spanstore delay"]
    #[inline(always)]
    pub fn ssd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Pitch_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Pitch_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Pitch {
    #[inline(always)]
    fn default() -> Pitch {
        <crate::RegValueT<Pitch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Origin_SPEC;
impl crate::sealed::RegSpec for Origin_SPEC {
    type DataType = u32;
}

#[doc = "Framebuffer Base Address Register"]
pub type Origin = crate::RegValueT<Origin_SPEC>;

impl Origin {
    #[doc = "Address of the first pixel in framebuffer"]
    #[inline(always)]
    pub fn origin(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Origin_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Origin_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Origin {
    #[inline(always)]
    fn default() -> Origin {
        <crate::RegValueT<Origin_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lustart_SPEC;
impl crate::sealed::RegSpec for Lustart_SPEC {
    type DataType = u32;
}

#[doc = "U Limiter Start Value Register"]
pub type Lustart = crate::RegValueT<Lustart_SPEC>;

impl Lustart {
    #[doc = "U limiter start value"]
    #[inline(always)]
    pub fn lustart(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lustart_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lustart_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lustart {
    #[inline(always)]
    fn default() -> Lustart {
        <crate::RegValueT<Lustart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Luxadd_SPEC;
impl crate::sealed::RegSpec for Luxadd_SPEC {
    type DataType = u32;
}

#[doc = "U Limiter X-Axis Increment Register"]
pub type Luxadd = crate::RegValueT<Luxadd_SPEC>;

impl Luxadd {
    #[doc = "U limiter x-axis increment"]
    #[inline(always)]
    pub fn luxadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Luxadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Luxadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Luxadd {
    #[inline(always)]
    fn default() -> Luxadd {
        <crate::RegValueT<Luxadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Luyadd_SPEC;
impl crate::sealed::RegSpec for Luyadd_SPEC {
    type DataType = u32;
}

#[doc = "U Limiter Y-Axis Increment Register"]
pub type Luyadd = crate::RegValueT<Luyadd_SPEC>;

impl Luyadd {
    #[doc = "U limiter y-axis increment"]
    #[inline(always)]
    pub fn luyadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Luyadd_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Luyadd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Luyadd {
    #[inline(always)]
    fn default() -> Luyadd {
        <crate::RegValueT<Luyadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvstarti_SPEC;
impl crate::sealed::RegSpec for Lvstarti_SPEC {
    type DataType = u32;
}

#[doc = "V Limiter Start Value Integer Part Register"]
pub type Lvstarti = crate::RegValueT<Lvstarti_SPEC>;

impl Lvstarti {
    #[doc = "V limiter start value integer part"]
    #[inline(always)]
    pub fn lvstarti(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lvstarti_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lvstarti_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvstarti {
    #[inline(always)]
    fn default() -> Lvstarti {
        <crate::RegValueT<Lvstarti_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvstartf_SPEC;
impl crate::sealed::RegSpec for Lvstartf_SPEC {
    type DataType = u32;
}

#[doc = "V Limiter Start Value Fractional Part Register"]
pub type Lvstartf = crate::RegValueT<Lvstartf_SPEC>;

impl Lvstartf {
    #[doc = "V limiter start value fractional part"]
    #[inline(always)]
    pub fn lvstartf(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Lvstartf_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Lvstartf_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Lvstartf_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Lvstartf_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvstartf {
    #[inline(always)]
    fn default() -> Lvstartf {
        <crate::RegValueT<Lvstartf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvxaddi_SPEC;
impl crate::sealed::RegSpec for Lvxaddi_SPEC {
    type DataType = u32;
}

#[doc = "V Limiter X-Axis Increment Integer Part Register"]
pub type Lvxaddi = crate::RegValueT<Lvxaddi_SPEC>;

impl Lvxaddi {
    #[doc = "V limiter x-axis increment integer part"]
    #[inline(always)]
    pub fn lvxaddi(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lvxaddi_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lvxaddi_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvxaddi {
    #[inline(always)]
    fn default() -> Lvxaddi {
        <crate::RegValueT<Lvxaddi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvyaddi_SPEC;
impl crate::sealed::RegSpec for Lvyaddi_SPEC {
    type DataType = u32;
}

#[doc = "V Limiter Y-Axis Increment Integer Part Register"]
pub type Lvyaddi = crate::RegValueT<Lvyaddi_SPEC>;

impl Lvyaddi {
    #[doc = "V limiter y-axis increment integer part"]
    #[inline(always)]
    pub fn lvyaddi(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lvyaddi_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lvyaddi_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvyaddi {
    #[inline(always)]
    fn default() -> Lvyaddi {
        <crate::RegValueT<Lvyaddi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvyxaddf_SPEC;
impl crate::sealed::RegSpec for Lvyxaddf_SPEC {
    type DataType = u32;
}

#[doc = "V Limiter Increment Fractional Parts Register"]
pub type Lvyxaddf = crate::RegValueT<Lvyxaddf_SPEC>;

impl Lvyxaddf {
    #[doc = "V xlimiter increment fractional part"]
    #[inline(always)]
    pub fn lvxaddf(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Lvyxaddf_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Lvyxaddf_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "V y limiter increment fractional part"]
    #[inline(always)]
    pub fn lvyaddf(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Lvyxaddf_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Lvyxaddf_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvyxaddf {
    #[inline(always)]
    fn default() -> Lvyxaddf {
        <crate::RegValueT<Lvyxaddf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Texpitch_SPEC;
impl crate::sealed::RegSpec for Texpitch_SPEC {
    type DataType = u32;
}

#[doc = "Texels Per Texture Line Register"]
pub type Texpitch = crate::RegValueT<Texpitch_SPEC>;

impl Texpitch {
    #[doc = "Texels per texture linevalid range: 0 to 2048"]
    #[inline(always)]
    pub fn texpitch(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Texpitch_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Texpitch_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Texpitch {
    #[inline(always)]
    fn default() -> Texpitch {
        <crate::RegValueT<Texpitch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Texmask_SPEC;
impl crate::sealed::RegSpec for Texmask_SPEC {
    type DataType = u32;
}

#[doc = "Texture Size or Texture Address Mask Register"]
pub type Texmask = crate::RegValueT<Texmask_SPEC>;

impl Texmask {
    #[doc = "U maskSet TEXUMASK\\[10:0\\] = texture_width -1In texture wrapping mode (CONTROL2.TEXTURECLAMPX = 0): texture_width must be a power of 2.In texture clamping mode (CONTROL2.TEXTURECLAMPX = 1):all widths up to 2048 are allowed."]
    #[inline(always)]
    pub fn texumask(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Texmask_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Texmask_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "V maskSet TEXVMASK\\[20:0\\] = TEXPITCH * (texture_height - 1).In texture wrapping mode (CONTROL2.TEXTURECLAMPY = 0): texture_height must be a power of 2In texture clamping mode (CONTROL2.TEXTURECLAMPY = 1):all heights up to 1024 are allowed."]
    #[inline(always)]
    pub fn texvmask(
        self,
    ) -> crate::common::RegisterField<11, 0x1fffff, 1, 0, u32, u32, Texmask_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<11,0x1fffff,1,0,u32,u32,Texmask_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Texmask {
    #[inline(always)]
    fn default() -> Texmask {
        <crate::RegValueT<Texmask_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Texorigin_SPEC;
impl crate::sealed::RegSpec for Texorigin_SPEC {
    type DataType = u32;
}

#[doc = "Texture Base Address Register"]
pub type Texorigin = crate::RegValueT<Texorigin_SPEC>;

impl Texorigin {
    #[doc = "Texture base address"]
    #[inline(always)]
    pub fn texorigin(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Texorigin_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Texorigin_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Texorigin {
    #[inline(always)]
    fn default() -> Texorigin {
        <crate::RegValueT<Texorigin_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqctl_SPEC;
impl crate::sealed::RegSpec for Irqctl_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Control Register"]
pub type Irqctl = crate::RegValueT<Irqctl_SPEC>;

impl Irqctl {
    #[doc = "ENUMIRQ interrupt mask enable"]
    #[inline(always)]
    pub fn enumirqen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        irqctl::Enumirqen,
        irqctl::Enumirqen,
        Irqctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            irqctl::Enumirqen,
            irqctl::Enumirqen,
            Irqctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "DLISTIRQ interrupt mask enable"]
    #[inline(always)]
    pub fn dlistirqen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        irqctl::Dlistirqen,
        irqctl::Dlistirqen,
        Irqctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            irqctl::Dlistirqen,
            irqctl::Dlistirqen,
            Irqctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Clear enumeration interrupt ENUMIRQ"]
    #[inline(always)]
    pub fn enumirqclr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        irqctl::Enumirqclr,
        irqctl::Enumirqclr,
        Irqctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            irqctl::Enumirqclr,
            irqctl::Enumirqclr,
            Irqctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Clear display list interrupt DLISTIRQ"]
    #[inline(always)]
    pub fn dlistirqclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        irqctl::Dlistirqclr,
        irqctl::Dlistirqclr,
        Irqctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            irqctl::Dlistirqclr,
            irqctl::Dlistirqclr,
            Irqctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "BUSIRQ interrupt mask enable"]
    #[inline(always)]
    pub fn busirqen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        irqctl::Busirqen,
        irqctl::Busirqen,
        Irqctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            irqctl::Busirqen,
            irqctl::Busirqen,
            Irqctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Clear bus error interrupt BUSIRQ"]
    #[inline(always)]
    pub fn busirqclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        irqctl::Busirqclr,
        irqctl::Busirqclr,
        Irqctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            irqctl::Busirqclr,
            irqctl::Busirqclr,
            Irqctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Irqctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Irqctl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Irqctl {
    #[inline(always)]
    fn default() -> Irqctl {
        <crate::RegValueT<Irqctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irqctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enumirqen_SPEC;
    pub type Enumirqen = crate::EnumBitfieldStruct<u8, Enumirqen_SPEC>;
    impl Enumirqen {
        #[doc = "disable (mask) ENUMIRQ"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable (unmask) ENUMIRQ"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlistirqen_SPEC;
    pub type Dlistirqen = crate::EnumBitfieldStruct<u8, Dlistirqen_SPEC>;
    impl Dlistirqen {
        #[doc = "disable (mask) DLISTIRQ"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable (unmask) DLISTIRQ"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enumirqclr_SPEC;
    pub type Enumirqclr = crate::EnumBitfieldStruct<u8, Enumirqclr_SPEC>;
    impl Enumirqclr {
        #[doc = "no ENUMIRQCLR clear"]
        pub const _0: Self = Self::new(0);

        #[doc = "clear ENUMIRQCLR"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlistirqclr_SPEC;
    pub type Dlistirqclr = crate::EnumBitfieldStruct<u8, Dlistirqclr_SPEC>;
    impl Dlistirqclr {
        #[doc = "no DLISTRQCLR clear"]
        pub const _0: Self = Self::new(0);

        #[doc = "clear DLISTRQCLR"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busirqen_SPEC;
    pub type Busirqen = crate::EnumBitfieldStruct<u8, Busirqen_SPEC>;
    impl Busirqen {
        #[doc = "disable (mask) BUSIRQ"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable (unmask) BUSIRQ"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busirqclr_SPEC;
    pub type Busirqclr = crate::EnumBitfieldStruct<u8, Busirqclr_SPEC>;
    impl Busirqclr {
        #[doc = "no BUSIRQCLR clear"]
        pub const _0: Self = Self::new(0);

        #[doc = "clear BUSIRQCLR"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachectl_SPEC;
impl crate::sealed::RegSpec for Cachectl_SPEC {
    type DataType = u32;
}

#[doc = "Cache Control Register"]
pub type Cachectl = crate::RegValueT<Cachectl_SPEC>;

impl Cachectl {
    #[doc = "Framebuffer cache enable"]
    #[inline(always)]
    pub fn cenablefx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cachectl::Cenablefx,
        cachectl::Cenablefx,
        Cachectl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cachectl::Cenablefx,
            cachectl::Cenablefx,
            Cachectl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Flush framebuffer cache"]
    #[inline(always)]
    pub fn cflushfx(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cachectl::Cflushfx,
        cachectl::Cflushfx,
        Cachectl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cachectl::Cflushfx,
            cachectl::Cflushfx,
            Cachectl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Texture cache enable"]
    #[inline(always)]
    pub fn cenabletx(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cachectl::Cenabletx,
        cachectl::Cenabletx,
        Cachectl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cachectl::Cenabletx,
            cachectl::Cenabletx,
            Cachectl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Flush texture cache"]
    #[inline(always)]
    pub fn cflushtx(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cachectl::Cflushtx,
        cachectl::Cflushtx,
        Cachectl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cachectl::Cflushtx,
            cachectl::Cflushtx,
            Cachectl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cachectl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cachectl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cachectl {
    #[inline(always)]
    fn default() -> Cachectl {
        <crate::RegValueT<Cachectl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cachectl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cenablefx_SPEC;
    pub type Cenablefx = crate::EnumBitfieldStruct<u8, Cenablefx_SPEC>;
    impl Cenablefx {
        #[doc = "disable the framebuffer cache"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable the framebuffer cache"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cflushfx_SPEC;
    pub type Cflushfx = crate::EnumBitfieldStruct<u8, Cflushfx_SPEC>;
    impl Cflushfx {
        #[doc = "do not flush the framebuffer cache"]
        pub const _0: Self = Self::new(0);

        #[doc = "flush the framebuffer cache"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cenabletx_SPEC;
    pub type Cenabletx = crate::EnumBitfieldStruct<u8, Cenabletx_SPEC>;
    impl Cenabletx {
        #[doc = "disable the texture cache"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable the texture cache"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cflushtx_SPEC;
    pub type Cflushtx = crate::EnumBitfieldStruct<u8, Cflushtx_SPEC>;
    impl Cflushtx {
        #[doc = "do not flush the texture cache"]
        pub const _0: Self = Self::new(0);

        #[doc = "flush the texture cache"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dliststart_SPEC;
impl crate::sealed::RegSpec for Dliststart_SPEC {
    type DataType = u32;
}

#[doc = "Display List Start Address Register"]
pub type Dliststart = crate::RegValueT<Dliststart_SPEC>;

impl Dliststart {
    #[doc = "Display list start address"]
    #[inline(always)]
    pub fn dliststart(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Dliststart_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Dliststart_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dliststart {
    #[inline(always)]
    fn default() -> Dliststart {
        <crate::RegValueT<Dliststart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perfcount_SPEC;
impl crate::sealed::RegSpec for Perfcount_SPEC {
    type DataType = u32;
}

#[doc = "Performance Counter %s"]
pub type Perfcount = crate::RegValueT<Perfcount_SPEC>;

impl Perfcount {
    #[doc = "Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H."]
    #[inline(always)]
    pub fn perfcountk(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Perfcount_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Perfcount_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Perfcount {
    #[inline(always)]
    fn default() -> Perfcount {
        <crate::RegValueT<Perfcount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perftrigger_SPEC;
impl crate::sealed::RegSpec for Perftrigger_SPEC {
    type DataType = u32;
}

#[doc = "Performance Counters Control Register"]
pub type Perftrigger = crate::RegValueT<Perftrigger_SPEC>;

impl Perftrigger {
    #[doc = "Selects the internal event that will increment PERFCOUNT1 register."]
    #[inline(always)]
    pub fn perftrigger1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        perftrigger::Perftrigger1,
        perftrigger::Perftrigger1,
        Perftrigger_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            perftrigger::Perftrigger1,
            perftrigger::Perftrigger1,
            Perftrigger_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Selects the internal event that will increment PERFCOUNT2 register"]
    #[inline(always)]
    pub fn perftrigger2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        perftrigger::Perftrigger2,
        perftrigger::Perftrigger2,
        Perftrigger_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            perftrigger::Perftrigger2,
            perftrigger::Perftrigger2,
            Perftrigger_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Perftrigger {
    #[inline(always)]
    fn default() -> Perftrigger {
        <crate::RegValueT<Perftrigger_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod perftrigger {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perftrigger1_SPEC;
    pub type Perftrigger1 = crate::EnumBitfieldStruct<u8, Perftrigger1_SPEC>;
    impl Perftrigger1 {
        #[doc = "disable performance counter"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "2D Drawing Engine active cycles"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "framebuffer read access"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "framebuffer write access"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "texture read access"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "invisible pixels (enumerated but selected with alpha 0percent)"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "invisible pixels while internal FIFO is empty (lost cycles)"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "display list reader active cycles"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "framebuffer read hits"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "framebuffer read misses"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "framebuffer write hits"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "framebuffer write misses"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "texture read hits"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "texture read misses"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "every clock cycle (for use as timer)"]
        pub const _0_X_1_F: Self = Self::new(31);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perftrigger2_SPEC;
    pub type Perftrigger2 = crate::EnumBitfieldStruct<u8, Perftrigger2_SPEC>;
    impl Perftrigger2 {
        #[doc = "disable performance counter"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "2D Drawing Engine active cycles"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "framebuffer read access"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "framebuffer write access"]
        pub const _0_X_03: Self = Self::new(3);

        #[doc = "texture read access"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "invisible pixels (enumerated but selected with alpha 0percent)"]
        pub const _0_X_05: Self = Self::new(5);

        #[doc = "invisible pixels while internal FIFO is empty (lost cycles)"]
        pub const _0_X_06: Self = Self::new(6);

        #[doc = "display list reader active cycles"]
        pub const _0_X_07: Self = Self::new(7);

        #[doc = "framebuffer read hits"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "framebuffer read misses"]
        pub const _0_X_09: Self = Self::new(9);

        #[doc = "framebuffer write hits"]
        pub const _0_X_0_A: Self = Self::new(10);

        #[doc = "framebuffer write misses"]
        pub const _0_X_0_B: Self = Self::new(11);

        #[doc = "texture read hits"]
        pub const _0_X_0_C: Self = Self::new(12);

        #[doc = "texture read misses"]
        pub const _0_X_0_D: Self = Self::new(13);

        #[doc = "every clock cycle (for use as timer)"]
        pub const _0_X_1_F: Self = Self::new(31);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Texcladdr_SPEC;
impl crate::sealed::RegSpec for Texcladdr_SPEC {
    type DataType = u32;
}

#[doc = "CLUT Start Address Register"]
pub type Texcladdr = crate::RegValueT<Texcladdr_SPEC>;

impl Texcladdr {
    #[doc = "Texture CLUT start address for indexed texture format"]
    #[inline(always)]
    pub fn claddr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Texcladdr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Texcladdr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Texcladdr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Texcladdr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Texcladdr {
    #[inline(always)]
    fn default() -> Texcladdr {
        <crate::RegValueT<Texcladdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Texcldata_SPEC;
impl crate::sealed::RegSpec for Texcldata_SPEC {
    type DataType = u32;
}

#[doc = "CLUT Data Register"]
pub type Texcldata = crate::RegValueT<Texcldata_SPEC>;

impl Texcldata {
    #[doc = "Texture CLUT data for Indexed texture format"]
    #[inline(always)]
    pub fn cldata(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Texcldata_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Texcldata_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Texcldata {
    #[inline(always)]
    fn default() -> Texcldata {
        <crate::RegValueT<Texcldata_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Texcloffset_SPEC;
impl crate::sealed::RegSpec for Texcloffset_SPEC {
    type DataType = u32;
}

#[doc = "CLUT Offset Register"]
pub type Texcloffset = crate::RegValueT<Texcloffset_SPEC>;

impl Texcloffset {
    #[doc = "Texture CLUT offset for Indexed texture format. CLOFFSET\\[7:0\\] is or\'ed with the original index"]
    #[inline(always)]
    pub fn cloffset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Texcloffset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Texcloffset_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Texcloffset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Texcloffset_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Texcloffset {
    #[inline(always)]
    fn default() -> Texcloffset {
        <crate::RegValueT<Texcloffset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Colkey_SPEC;
impl crate::sealed::RegSpec for Colkey_SPEC {
    type DataType = u32;
}

#[doc = "Color Key Register"]
pub type Colkey = crate::RegValueT<Colkey_SPEC>;

impl Colkey {
    #[doc = "Blue channel of color key"]
    #[inline(always)]
    pub fn colkeyb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Colkey_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Colkey_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Green channel of color key"]
    #[inline(always)]
    pub fn colkeyg(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Colkey_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Colkey_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Red channel of color key"]
    #[inline(always)]
    pub fn colkeyr(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Colkey_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Colkey_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Colkey_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Colkey_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Colkey {
    #[inline(always)]
    fn default() -> Colkey {
        <crate::RegValueT<Colkey_SPEC> as RegisterValue<_>>::new(0)
    }
}
