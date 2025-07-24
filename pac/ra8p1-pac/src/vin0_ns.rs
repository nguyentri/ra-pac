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
#[doc = r"VIN0_NS Register area"]
unsafe impl ::core::marker::Send for super::Vin0Ns {}
unsafe impl ::core::marker::Sync for super::Vin0Ns {}
impl super::Vin0Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Main Control Register"]
    #[inline(always)]
    pub const fn mc(&self) -> &'static crate::common::Reg<self::Mc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Module Status Register"]
    #[inline(always)]
    pub const fn ms(&self) -> &'static crate::common::Reg<self::Ms_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ms_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Frame Capture Register"]
    #[inline(always)]
    pub const fn fc(&self) -> &'static crate::common::Reg<self::Fc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Start Line Pre-Clip Register"]
    #[inline(always)]
    pub const fn slprc(&self) -> &'static crate::common::Reg<self::Slprc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Slprc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "End Line Pre-Clip Register"]
    #[inline(always)]
    pub const fn elprc(&self) -> &'static crate::common::Reg<self::Elprc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elprc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Start Pixel Pre-Clip Register"]
    #[inline(always)]
    pub const fn spprc(&self) -> &'static crate::common::Reg<self::Spprc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spprc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "End Pixel Pre-Clip Register"]
    #[inline(always)]
    pub const fn epprc(&self) -> &'static crate::common::Reg<self::Epprc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Epprc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "CSI2 Interface Mode Register"]
    #[inline(always)]
    pub const fn csi_ifmd(
        &self,
    ) -> &'static crate::common::Reg<self::CsiIfmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CsiIfmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Field detection control Register"]
    #[inline(always)]
    pub const fn csifld(
        &self,
    ) -> &'static crate::common::Reg<self::Csifld_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csifld_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Image Stride Register"]
    #[inline(always)]
    pub const fn is(&self) -> &'static crate::common::Reg<self::Is_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Is_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Memory Base 1 Register"]
    #[inline(always)]
    pub const fn mb1(&self) -> &'static crate::common::Reg<self::Mb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Memory Base 2 Register"]
    #[inline(always)]
    pub const fn mb2(&self) -> &'static crate::common::Reg<self::Mb2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mb2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Memory Base 3 Register"]
    #[inline(always)]
    pub const fn mb3(&self) -> &'static crate::common::Reg<self::Mb3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mb3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Line Count Register"]
    #[inline(always)]
    pub const fn lc(&self) -> &'static crate::common::Reg<self::Lc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Lc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &'static crate::common::Reg<self::Ie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn ints(&self) -> &'static crate::common::Reg<self::Ints_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ints_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Scanline Interrupt Register"]
    #[inline(always)]
    pub const fn si(&self) -> &'static crate::common::Reg<self::Si_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Si_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "AXI transfer stop control register"]
    #[inline(always)]
    pub const fn mtcstop(
        &self,
    ) -> &'static crate::common::Reg<self::Mtcstop_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtcstop_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Data Mode Register"]
    #[inline(always)]
    pub const fn dmr(&self) -> &'static crate::common::Reg<self::Dmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "UV Address Offset Register"]
    #[inline(always)]
    pub const fn uvaof(&self) -> &'static crate::common::Reg<self::Uvaof_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uvaof_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Scaling Control Registers"]
    #[inline(always)]
    pub const fn uds_ctrl(
        &self,
    ) -> &'static crate::common::Reg<self::UdsCtrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UdsCtrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Scaling Factor Registers"]
    #[inline(always)]
    pub const fn uds_scale(
        &self,
    ) -> &'static crate::common::Reg<self::UdsScale_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UdsScale_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Passband Registers"]
    #[inline(always)]
    pub const fn uds_pass_bwidth(
        &self,
    ) -> &'static crate::common::Reg<self::UdsPassBwidth_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UdsPassBwidth_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "UDS Output Size Clipping Registers"]
    #[inline(always)]
    pub const fn uds_clip_size(
        &self,
    ) -> &'static crate::common::Reg<self::UdsClipSize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UdsClipSize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Lookup Table Pointer Register"]
    #[inline(always)]
    pub const fn lutp(&self) -> &'static crate::common::Reg<self::Lutp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lutp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Lookup Table Data Register"]
    #[inline(always)]
    pub const fn lutd(&self) -> &'static crate::common::Reg<self::Lutd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lutd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "RGB to Y Calculation Setting Register 1"]
    #[inline(always)]
    pub const fn yccr1(&self) -> &'static crate::common::Reg<self::Yccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Yccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[doc = "RGB to Y Calculation Setting Register 2"]
    #[inline(always)]
    pub const fn yccr2(&self) -> &'static crate::common::Reg<self::Yccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Yccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(556usize),
            )
        }
    }

    #[doc = "RGB to Y Calculation Setting Register 3"]
    #[inline(always)]
    pub const fn yccr3(&self) -> &'static crate::common::Reg<self::Yccr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Yccr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(560usize),
            )
        }
    }

    #[doc = "RGB to Cb Calculation Setting Register 1"]
    #[inline(always)]
    pub const fn cbccr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cbccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cbccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(564usize),
            )
        }
    }

    #[doc = "RGB to Cb Calculation Setting Register 2"]
    #[inline(always)]
    pub const fn cbccr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cbccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cbccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(568usize),
            )
        }
    }

    #[doc = "RGB to Cb Calculation Setting Register 3"]
    #[inline(always)]
    pub const fn cbccr3(
        &self,
    ) -> &'static crate::common::Reg<self::Cbccr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cbccr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(572usize),
            )
        }
    }

    #[doc = "RGB to Cr Calculation Setting Register 1"]
    #[inline(always)]
    pub const fn crccr1(
        &self,
    ) -> &'static crate::common::Reg<self::Crccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(576usize),
            )
        }
    }

    #[doc = "RGB to Cr Calculation Setting Register 2"]
    #[inline(always)]
    pub const fn crccr2(
        &self,
    ) -> &'static crate::common::Reg<self::Crccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(580usize),
            )
        }
    }

    #[doc = "RGB to Cr Calculation Setting Register 3"]
    #[inline(always)]
    pub const fn crccr3(
        &self,
    ) -> &'static crate::common::Reg<self::Crccr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crccr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(584usize),
            )
        }
    }

    #[doc = "YC to RGB Calculation Setting Extension Register 1"]
    #[inline(always)]
    pub const fn csce1(&self) -> &'static crate::common::Reg<self::Csce1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csce1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "YC to RGB Calculation Setting Extension Register 2"]
    #[inline(always)]
    pub const fn csce2(&self) -> &'static crate::common::Reg<self::Csce2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csce2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "YC to RGB Calculation Setting Extension Register 3"]
    #[inline(always)]
    pub const fn csce3(&self) -> &'static crate::common::Reg<self::Csce3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csce3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(776usize),
            )
        }
    }

    #[doc = "YC to RGB Calculation Setting Extension Register 4"]
    #[inline(always)]
    pub const fn csce4(&self) -> &'static crate::common::Reg<self::Csce4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csce4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(780usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mc_SPEC;
impl crate::sealed::RegSpec for Mc_SPEC {
    type DataType = u32;
}

#[doc = "Main Control Register"]
pub type Mc = crate::RegValueT<Mc_SPEC>;

impl Mc {
    #[doc = "Module Enable"]
    #[inline(always)]
    pub fn me(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mc::Me, mc::Me, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mc::Me,mc::Me,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Color Space Conversion Bypass Mode"]
    #[inline(always)]
    pub fn bps(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mc::Bps, mc::Bps, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mc::Bps,mc::Bps,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interlace Mode"]
    #[inline(always)]
    pub fn im(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, mc::Im, mc::Im, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,mc::Im,mc::Im,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Endian Type"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mc::En, mc::En, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,mc::En,mc::En,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dithering Mode Control"]
    #[inline(always)]
    pub fn dc(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, mc::Dc, mc::Dc, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,mc::Dc,mc::Dc,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Input Interface Format"]
    #[inline(always)]
    pub fn inf(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, mc::Inf, mc::Inf, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,mc::Inf,mc::Inf,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Lookup Table Enable"]
    #[inline(always)]
    pub fn lute(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, mc::Lute, mc::Lute, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,mc::Lute,mc::Lute,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Initialization control at STartup"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, mc::St, mc::St, Mc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<22,0x1,1,0,mc::St,mc::St,Mc_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Dithering mode Control 2"]
    #[inline(always)]
    pub fn dc2(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mc::Dc2, mc::Dc2, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,mc::Dc2,mc::Dc2,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "YUV444 conversion"]
    #[inline(always)]
    pub fn yuv444(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mc::Yuv444,
        mc::Yuv444,
        Mc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mc::Yuv444,
            mc::Yuv444,
            Mc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is used to enable or disable scaling by the UDS."]
    #[inline(always)]
    pub fn scle(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, mc::Scle, mc::Scle, Mc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,mc::Scle,mc::Scle,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pixel Data Clipping"]
    #[inline(always)]
    pub fn clp(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Mc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Mc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mc {
    #[inline(always)]
    fn default() -> Mc {
        <crate::RegValueT<Mc_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod mc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Me_SPEC;
    pub type Me = crate::EnumBitfieldStruct<u8, Me_SPEC>;
    impl Me {
        #[doc = "The module operation is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "The module operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bps_SPEC;
    pub type Bps = crate::EnumBitfieldStruct<u8, Bps_SPEC>;
    impl Bps {
        #[doc = "The input YCbCr data is converted into the RGB color space and RGB data is converted into the YCbCr color space."]
        pub const _0: Self = Self::new(0);

        #[doc = "Color space conversion is not performed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Im_SPEC;
    pub type Im = crate::EnumBitfieldStruct<u8, Im_SPEC>;
    impl Im {
        #[doc = "Odd-field (field 1) capture mode Handles only odd fields as frames and stores them in external memory."]
        pub const _00: Self = Self::new(0);

        #[doc = "Odd-/even-field capture mode Handles odd and even fields as separate frames and stores them in external memory. This mode is available only in continuous frame capture mode. For progressive input and continuous frame capture mode, set 01b."]
        pub const _01: Self = Self::new(1);

        #[doc = "Even-field (field 2) capture mode Handles only even fields as frames and stores them in external memory."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Image data is packed and allocated in little endian."]
        pub const _0: Self = Self::new(0);

        #[doc = "Image data is packed and allocated in big endian."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dc_SPEC;
    pub type Dc = crate::EnumBitfieldStruct<u8, Dc_SPEC>;
    impl Dc {
        #[doc = "Dithering with cumulative addition"]
        pub const _00: Self = Self::new(0);

        #[doc = "Ordered dithering 1"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "Ordered dithering 2"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inf_SPEC;
    pub type Inf = crate::EnumBitfieldStruct<u8, Inf_SPEC>;
    impl Inf {
        #[doc = "Setting prohibited"]
        pub const _000: Self = Self::new(0);

        #[doc = "8-bit YCbCr-422"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _010: Self = Self::new(2);

        #[doc = "10-bit YCbCr-422"]
        pub const _011: Self = Self::new(3);

        #[doc = "8-bit user defined data (RAW8)"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "24-bit RGB-888"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lute_SPEC;
    pub type Lute = crate::EnumBitfieldStruct<u8, Lute_SPEC>;
    impl Lute {
        #[doc = "LUT is not used."]
        pub const _0: Self = Self::new(0);

        #[doc = "LUT is used."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "Invalid."]
        pub const _0: Self = Self::new(0);

        #[doc = "Perform the initialization procedure. It is prohibited to use it except for the initialization and resumption procedure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dc2_SPEC;
    pub type Dc2 = crate::EnumBitfieldStruct<u8, Dc2_SPEC>;
    impl Dc2 {
        #[doc = "Enables dithering in the frame direction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disables dithering in the frame direction"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Yuv444_SPEC;
    pub type Yuv444 = crate::EnumBitfieldStruct<u8, Yuv444_SPEC>;
    impl Yuv444 {
        #[doc = "The data of CbCr to be interpolated holds the data one pixel before."]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates CbCr data to be interpolated. This register is invalid when the input format is RGB. If the input format is RAW, set 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scle_SPEC;
    pub type Scle = crate::EnumBitfieldStruct<u8, Scle_SPEC>;
    impl Scle {
        #[doc = "Disables scaling by the UDS."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables scaling by the UDS."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ms_SPEC;
impl crate::sealed::RegSpec for Ms_SPEC {
    type DataType = u32;
}

#[doc = "Module Status Register"]
pub type Ms = crate::RegValueT<Ms_SPEC>;

impl Ms {
    #[doc = "Video Capture Active Status"]
    #[inline(always)]
    pub fn ca(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ms::Ca, ms::Ca, Ms_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,ms::Ca,ms::Ca,Ms_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Active Video Status"]
    #[inline(always)]
    pub fn av(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ms::Av, ms::Av, Ms_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,ms::Av,ms::Av,Ms_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Field Status"]
    #[inline(always)]
    pub fn fs(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ms::Fs, ms::Fs, Ms_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,ms::Fs,ms::Fs,Ms_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Frame Buffer Status"]
    #[inline(always)]
    pub fn fbs(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, ms::Fbs, ms::Fbs, Ms_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x3,1,0,ms::Fbs,ms::Fbs,Ms_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "External frame Memory capture Active status"]
    #[inline(always)]
    pub fn ma(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ms::Ma, ms::Ma, Ms_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,ms::Ma,ms::Ma,Ms_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "External Frame Memory buffer Status"]
    #[inline(always)]
    pub fn fms(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, ms::Fms, ms::Fms, Ms_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x3,1,0,ms::Fms,ms::Fms,Ms_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ms {
    #[inline(always)]
    fn default() -> Ms {
        <crate::RegValueT<Ms_SPEC> as RegisterValue<_>>::new(1572892)
    }
}
pub mod ms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ca_SPEC;
    pub type Ca = crate::EnumBitfieldStruct<u8, Ca_SPEC>;
    impl Ca {
        #[doc = "Video capture is not operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "Video capture is operating."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Av_SPEC;
    pub type Av = crate::EnumBitfieldStruct<u8, Av_SPEC>;
    impl Av {
        #[doc = "The current field is not in the active video area."]
        pub const _0: Self = Self::new(0);

        #[doc = "The current field is in the active video area."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fs_SPEC;
    pub type Fs = crate::EnumBitfieldStruct<u8, Fs_SPEC>;
    impl Fs {
        #[doc = "The current field is an odd field (field 1)."]
        pub const _0: Self = Self::new(0);

        #[doc = "The current field is an even field (field 2)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fbs_SPEC;
    pub type Fbs = crate::EnumBitfieldStruct<u8, Fbs_SPEC>;
    impl Fbs {
        #[doc = "The latest valid frame buffer has the base address defined by the memory base 1 register."]
        pub const _00: Self = Self::new(0);

        #[doc = "The latest valid frame buffer has the base address defined by the memory base 2 register."]
        pub const _01: Self = Self::new(1);

        #[doc = "The latest valid frame buffer has the base address defined by the memory base 3 register."]
        pub const _10: Self = Self::new(2);

        #[doc = "There is no valid frame buffer."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ma_SPEC;
    pub type Ma = crate::EnumBitfieldStruct<u8, Ma_SPEC>;
    impl Ma {
        #[doc = "Capture to external memory is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Capture to external memory is in operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fms_SPEC;
    pub type Fms = crate::EnumBitfieldStruct<u8, Fms_SPEC>;
    impl Fms {
        #[doc = "The latest valid frame buffer has the base address defined by the memory base 1 register."]
        pub const _00: Self = Self::new(0);

        #[doc = "The latest valid frame buffer has the base address defined by the memory base 2 register."]
        pub const _01: Self = Self::new(1);

        #[doc = "The latest valid frame buffer has the base address defined by the memory base 3 register."]
        pub const _10: Self = Self::new(2);

        #[doc = "There is no valid frame buffer."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc_SPEC;
impl crate::sealed::RegSpec for Fc_SPEC {
    type DataType = u32;
}

#[doc = "Frame Capture Register"]
pub type Fc = crate::RegValueT<Fc_SPEC>;

impl Fc {
    #[doc = "Continuous Frame Capture Mode"]
    #[inline(always)]
    pub fn cc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, fc::Cc, fc::Cc, Fc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,fc::Cc,fc::Cc,Fc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fc {
    #[inline(always)]
    fn default() -> Fc {
        <crate::RegValueT<Fc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cc_SPEC;
    pub type Cc = crate::EnumBitfieldStruct<u8, Cc_SPEC>;
    impl Cc {
        #[doc = "The continuous frame capture mode is not set."]
        pub const _0: Self = Self::new(0);

        #[doc = "The continuous frame capture mode is set."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slprc_SPEC;
impl crate::sealed::RegSpec for Slprc_SPEC {
    type DataType = u32;
}

#[doc = "Start Line Pre-Clip Register"]
pub type Slprc = crate::RegValueT<Slprc_SPEC>;

impl Slprc {
    #[doc = "Start Line PRe-Clip"]
    #[inline(always)]
    pub fn slprc(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Slprc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Slprc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Slprc {
    #[inline(always)]
    fn default() -> Slprc {
        <crate::RegValueT<Slprc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elprc_SPEC;
impl crate::sealed::RegSpec for Elprc_SPEC {
    type DataType = u32;
}

#[doc = "End Line Pre-Clip Register"]
pub type Elprc = crate::RegValueT<Elprc_SPEC>;

impl Elprc {
    #[doc = "End Line PRe-Clip"]
    #[inline(always)]
    pub fn elprc(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Elprc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Elprc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elprc {
    #[inline(always)]
    fn default() -> Elprc {
        <crate::RegValueT<Elprc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spprc_SPEC;
impl crate::sealed::RegSpec for Spprc_SPEC {
    type DataType = u32;
}

#[doc = "Start Pixel Pre-Clip Register"]
pub type Spprc = crate::RegValueT<Spprc_SPEC>;

impl Spprc {
    #[doc = "Start Pixel Pre-Clip"]
    #[inline(always)]
    pub fn spprc(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Spprc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Spprc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spprc {
    #[inline(always)]
    fn default() -> Spprc {
        <crate::RegValueT<Spprc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epprc_SPEC;
impl crate::sealed::RegSpec for Epprc_SPEC {
    type DataType = u32;
}

#[doc = "End Pixel Pre-Clip Register"]
pub type Epprc = crate::RegValueT<Epprc_SPEC>;

impl Epprc {
    #[doc = "End Pixel PRe-Clip"]
    #[inline(always)]
    pub fn epprc(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Epprc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Epprc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Epprc {
    #[inline(always)]
    fn default() -> Epprc {
        <crate::RegValueT<Epprc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiIfmd_SPEC;
impl crate::sealed::RegSpec for CsiIfmd_SPEC {
    type DataType = u32;
}

#[doc = "CSI2 Interface Mode Register"]
pub type CsiIfmd = crate::RegValueT<CsiIfmd_SPEC>;

impl CsiIfmd {
    #[doc = "Virtual Channel SELect"]
    #[inline(always)]
    pub fn vc_sel(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, CsiIfmd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,CsiIfmd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Type select"]
    #[inline(always)]
    pub fn dt(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        csi_ifmd::Dt,
        csi_ifmd::Dt,
        CsiIfmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            csi_ifmd::Dt,
            csi_ifmd::Dt,
            CsiIfmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Extension Select"]
    #[inline(always)]
    pub fn des0(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        csi_ifmd::Des0,
        csi_ifmd::Des0,
        CsiIfmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            csi_ifmd::Des0,
            csi_ifmd::Des0,
            CsiIfmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CsiIfmd {
    #[inline(always)]
    fn default() -> CsiIfmd {
        <crate::RegValueT<CsiIfmd_SPEC> as RegisterValue<_>>::new(33562112)
    }
}
pub mod csi_ifmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dt_SPEC;
    pub type Dt = crate::EnumBitfieldStruct<u8, Dt_SPEC>;
    impl Dt {
        #[doc = "YUV422 8-bit"]
        pub const _0_X_1_E: Self = Self::new(30);

        #[doc = "YUV422 10-bit"]
        pub const _0_X_1_F: Self = Self::new(31);

        #[doc = "RGB888"]
        pub const _0_X_24: Self = Self::new(36);

        #[doc = "RAW 8-bit"]
        pub const _0_X_2_A: Self = Self::new(42);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Des0_SPEC;
    pub type Des0 = crate::EnumBitfieldStruct<u8, Des0_SPEC>;
    impl Des0 {
        #[doc = "Empty bits in the input data are repeatedly expanded from the highest-order bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Empty bits will be padded with zeros."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csifld_SPEC;
impl crate::sealed::RegSpec for Csifld_SPEC {
    type DataType = u32;
}

#[doc = "Field detection control Register"]
pub type Csifld = crate::RegValueT<Csifld_SPEC>;

impl Csifld {
    #[doc = "FieLD detect ENable"]
    #[inline(always)]
    pub fn fld_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        csifld::FldEn,
        csifld::FldEn,
        Csifld_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            csifld::FldEn,
            csifld::FldEn,
            Csifld_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "even FieLD DETect SELect"]
    #[inline(always)]
    pub fn fld_sel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        csifld::FldSel,
        csifld::FldSel,
        Csifld_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            csifld::FldSel,
            csifld::FldSel,
            Csifld_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "even FieLD NUMber setting"]
    #[inline(always)]
    pub fn fld_num(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Csifld_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Csifld_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Csifld {
    #[inline(always)]
    fn default() -> Csifld {
        <crate::RegValueT<Csifld_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod csifld {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct FldEn_SPEC;
    pub type FldEn = crate::EnumBitfieldStruct<u8, FldEn_SPEC>;
    impl FldEn {
        #[doc = "Even field detection control disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Even field detection control enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct FldSel_SPEC;
    pub type FldSel = crate::EnumBitfieldStruct<u8, FldSel_SPEC>;
    impl FldSel {
        #[doc = "Setting is prohibited."]
        pub const _00: Self = Self::new(0);

        #[doc = "When FLD_NUM matches the field number \\[0\\] bit, the field is detected as an even field."]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting is prohibited."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting is prohibited."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Is_SPEC;
impl crate::sealed::RegSpec for Is_SPEC {
    type DataType = u32;
}

#[doc = "Image Stride Register"]
pub type Is = crate::RegValueT<Is_SPEC>;

impl Is {
    #[doc = "Image Stride (Setting unit: pixel)"]
    #[inline(always)]
    pub fn is(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Is_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Is_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Is {
    #[inline(always)]
    fn default() -> Is {
        <crate::RegValueT<Is_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb1_SPEC;
impl crate::sealed::RegSpec for Mb1_SPEC {
    type DataType = u32;
}

#[doc = "Memory Base 1 Register"]
pub type Mb1 = crate::RegValueT<Mb1_SPEC>;

impl Mb1 {
    #[doc = "Memory Base Address 1"]
    #[inline(always)]
    pub fn mb1(
        self,
    ) -> crate::common::RegisterField<7, 0x1ffffff, 1, 0, u32, u32, Mb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1ffffff,1,0,u32,u32,Mb1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mb1 {
    #[inline(always)]
    fn default() -> Mb1 {
        <crate::RegValueT<Mb1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb2_SPEC;
impl crate::sealed::RegSpec for Mb2_SPEC {
    type DataType = u32;
}

#[doc = "Memory Base 2 Register"]
pub type Mb2 = crate::RegValueT<Mb2_SPEC>;

impl Mb2 {
    #[doc = "Memory Base Address 2"]
    #[inline(always)]
    pub fn mb2(
        self,
    ) -> crate::common::RegisterField<7, 0x1ffffff, 1, 0, u32, u32, Mb2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1ffffff,1,0,u32,u32,Mb2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mb2 {
    #[inline(always)]
    fn default() -> Mb2 {
        <crate::RegValueT<Mb2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mb3_SPEC;
impl crate::sealed::RegSpec for Mb3_SPEC {
    type DataType = u32;
}

#[doc = "Memory Base 3 Register"]
pub type Mb3 = crate::RegValueT<Mb3_SPEC>;

impl Mb3 {
    #[doc = "Memory Base Address 3"]
    #[inline(always)]
    pub fn mb3(
        self,
    ) -> crate::common::RegisterField<7, 0x1ffffff, 1, 0, u32, u32, Mb3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1ffffff,1,0,u32,u32,Mb3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mb3 {
    #[inline(always)]
    fn default() -> Mb3 {
        <crate::RegValueT<Mb3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lc_SPEC;
impl crate::sealed::RegSpec for Lc_SPEC {
    type DataType = u32;
}

#[doc = "Line Count Register"]
pub type Lc = crate::RegValueT<Lc_SPEC>;

impl Lc {
    #[doc = "Line Count"]
    #[inline(always)]
    pub fn lc(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Lc_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Lc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Lc {
    #[inline(always)]
    fn default() -> Lc {
        <crate::RegValueT<Lc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie_SPEC;
impl crate::sealed::RegSpec for Ie_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Enable Register"]
pub type Ie = crate::RegValueT<Ie_SPEC>;

impl Ie {
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn foe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ie::Foe, ie::Foe, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ie::Foe,ie::Foe,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "End of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn efe(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ie::Efe, ie::Efe, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ie::Efe,ie::Efe,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Scanline Interrupt Enable"]
    #[inline(always)]
    pub fn sie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ie::Sie, ie::Sie, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ie::Sie,ie::Sie,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Field Interrupt Enable"]
    #[inline(always)]
    pub fn fie(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ie::Fie, ie::Fie, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ie::Fie,ie::Fie,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Frame Memory write completion interrupt Enable"]
    #[inline(always)]
    pub fn fme(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ie::Fme, ie::Fme, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ie::Fme,ie::Fme,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PRCLIPH Error interrupt Enable"]
    #[inline(always)]
    pub fn prcliphee(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ie::Prcliphee,
        ie::Prcliphee,
        Ie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ie::Prcliphee,
            ie::Prcliphee,
            Ie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PRCLIPV Error interrupt Enable"]
    #[inline(always)]
    pub fn prclipvee(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ie::Prclipvee,
        ie::Prclipvee,
        Ie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ie::Prclipvee,
            ie::Prclipvee,
            Ie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Response Overflow interrupt Enable"]
    #[inline(always)]
    pub fn roe(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ie::Roe, ie::Roe, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ie::Roe,ie::Roe,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Axi Resp Error interrupt Enable"]
    #[inline(always)]
    pub fn aree(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ie::Aree, ie::Aree, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ie::Aree,ie::Aree,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VSYNC Deasserting Detect Interrupt Enable"]
    #[inline(always)]
    pub fn vre(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ie::Vre, ie::Vre, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ie::Vre,ie::Vre,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vsync asserting detect interrupt Enable"]
    #[inline(always)]
    pub fn vfe(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ie::Vfe, ie::Vfe, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ie::Vfe,ie::Vfe,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Field Interrupt Enable 2"]
    #[inline(always)]
    pub fn fie2(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ie::Fie2, ie::Fie2, Ie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ie::Fie2,ie::Fie2,Ie_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ie {
    #[inline(always)]
    fn default() -> Ie {
        <crate::RegValueT<Ie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Foe_SPEC;
    pub type Foe = crate::EnumBitfieldStruct<u8, Foe_SPEC>;
    impl Foe {
        #[doc = "FIFO overflow interrupts are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO overflow interrupts are enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Efe_SPEC;
    pub type Efe = crate::EnumBitfieldStruct<u8, Efe_SPEC>;
    impl Efe {
        #[doc = "End of frame interrupts are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "End of frame interrupts are enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sie_SPEC;
    pub type Sie = crate::EnumBitfieldStruct<u8, Sie_SPEC>;
    impl Sie {
        #[doc = "Scanline interrupts are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Scanline interrupts are enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fie_SPEC;
    pub type Fie = crate::EnumBitfieldStruct<u8, Fie_SPEC>;
    impl Fie {
        #[doc = "Field-switching interrupts are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Field-switching interrupts are enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fme_SPEC;
    pub type Fme = crate::EnumBitfieldStruct<u8, Fme_SPEC>;
    impl Fme {
        #[doc = "Memory write completion interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory write completion interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prcliphee_SPEC;
    pub type Prcliphee = crate::EnumBitfieldStruct<u8, Prcliphee_SPEC>;
    impl Prcliphee {
        #[doc = "PRCLIPH error detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "PRCLIPH error detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prclipvee_SPEC;
    pub type Prclipvee = crate::EnumBitfieldStruct<u8, Prclipvee_SPEC>;
    impl Prclipvee {
        #[doc = "PRCLIPV error detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "PRCLIPV error detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Roe_SPEC;
    pub type Roe = crate::EnumBitfieldStruct<u8, Roe_SPEC>;
    impl Roe {
        #[doc = "Response overflow detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Response overflow detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aree_SPEC;
    pub type Aree = crate::EnumBitfieldStruct<u8, Aree_SPEC>;
    impl Aree {
        #[doc = "AXI Resp error detection interrupt disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "AXI Resp error detection interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vre_SPEC;
    pub type Vre = crate::EnumBitfieldStruct<u8, Vre_SPEC>;
    impl Vre {
        #[doc = "VSYNC deasserting detect interrupts are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "VSYNC deasserting detect interrupts are enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vfe_SPEC;
    pub type Vfe = crate::EnumBitfieldStruct<u8, Vfe_SPEC>;
    impl Vfe {
        #[doc = "VSYNC asserting detect interrupts are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "VSYNC asserting detect interrupts are enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fie2_SPEC;
    pub type Fie2 = crate::EnumBitfieldStruct<u8, Fie2_SPEC>;
    impl Fie2 {
        #[doc = "Field interrupts are disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Field interrupts are enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints_SPEC;
impl crate::sealed::RegSpec for Ints_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Status Register"]
pub type Ints = crate::RegValueT<Ints_SPEC>;

impl Ints {
    #[doc = "FIFO Overflow Interrupt Status"]
    #[inline(always)]
    pub fn fos(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End of Frame Interrupt Status"]
    #[inline(always)]
    pub fn efs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Scanline Interrupt Status"]
    #[inline(always)]
    pub fn sis(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Field Interrupt Status"]
    #[inline(always)]
    pub fn fis(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Memory write completion interrupt Status"]
    #[inline(always)]
    pub fn fms(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PRCLIPH Error interrupt Status"]
    #[inline(always)]
    pub fn prcliphes(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PRCLIPV Error interrupt Status"]
    #[inline(always)]
    pub fn prclipves(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Response Overflow interrupt Status"]
    #[inline(always)]
    pub fn ros(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Axi Resp Error interrupt Status"]
    #[inline(always)]
    pub fn ares(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VSYNC Deasserting Detect Interrupt Status"]
    #[inline(always)]
    pub fn vrs(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VSYNC Asserting Detect Interrupt Status"]
    #[inline(always)]
    pub fn vfs(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Field Interrupt Status 2"]
    #[inline(always)]
    pub fn fis2(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ints_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ints_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ints {
    #[inline(always)]
    fn default() -> Ints {
        <crate::RegValueT<Ints_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Si_SPEC;
impl crate::sealed::RegSpec for Si_SPEC {
    type DataType = u32;
}

#[doc = "Scanline Interrupt Register"]
pub type Si = crate::RegValueT<Si_SPEC>;

impl Si {
    #[doc = "Scanline Interrupt Setting"]
    #[inline(always)]
    pub fn si(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Si_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Si_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Si {
    #[inline(always)]
    fn default() -> Si {
        <crate::RegValueT<Si_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtcstop_SPEC;
impl crate::sealed::RegSpec for Mtcstop_SPEC {
    type DataType = u32;
}

#[doc = "AXI transfer stop control register"]
pub type Mtcstop = crate::RegValueT<Mtcstop_SPEC>;

impl Mtcstop {
    #[doc = "axi forced STOP REQuest"]
    #[inline(always)]
    pub fn stopreq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Mtcstop_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mtcstop_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "for axi forced STOP request, ACKnowledgement"]
    #[inline(always)]
    pub fn stopack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Mtcstop_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mtcstop_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OUTSTANDing current number"]
    #[inline(always)]
    pub fn outstand(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Mtcstop_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Mtcstop_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtcstop {
    #[inline(always)]
    fn default() -> Mtcstop {
        <crate::RegValueT<Mtcstop_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmr_SPEC;
impl crate::sealed::RegSpec for Dmr_SPEC {
    type DataType = u32;
}

#[doc = "Data Mode Register"]
pub type Dmr = crate::RegValueT<Dmr_SPEC>;

impl Dmr {
    #[doc = "Data Conversion Mode"]
    #[inline(always)]
    pub fn dtmd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, dmr::Dtmd, dmr::Dtmd, Dmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dmr::Dtmd,
            dmr::Dtmd,
            Dmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alpha Bit"]
    #[inline(always)]
    pub fn abit(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, dmr::Abit, dmr::Abit, Dmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmr::Abit,
            dmr::Abit,
            Dmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output Data Byte Swap Mode"]
    #[inline(always)]
    pub fn bpsm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, dmr::Bpsm, dmr::Bpsm, Dmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmr::Bpsm,
            dmr::Bpsm,
            Dmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Extension RGB Conversion Mode"]
    #[inline(always)]
    pub fn exrgb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dmr::Exrgb,
        dmr::Exrgb,
        Dmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dmr::Exrgb,
            dmr::Exrgb,
            Dmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "YC Data Through Mode"]
    #[inline(always)]
    pub fn yc_thr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        dmr::YcThr,
        dmr::YcThr,
        Dmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            dmr::YcThr,
            dmr::YcThr,
            Dmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "YC Data Transfer Mode"]
    #[inline(always)]
    pub fn ymode(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        dmr::Ymode,
        dmr::Ymode,
        Dmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            dmr::Ymode,
            dmr::Ymode,
            Dmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alpha 8"]
    #[inline(always)]
    pub fn a8bit(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Dmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Dmr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmr {
    #[inline(always)]
    fn default() -> Dmr {
        <crate::RegValueT<Dmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtmd_SPEC;
    pub type Dtmd = crate::EnumBitfieldStruct<u8, Dtmd_SPEC>;
    impl Dtmd {
        #[doc = "Data is not converted."]
        pub const _00: Self = Self::new(0);

        #[doc = "RGB is converted to ARGB before output."]
        pub const _01: Self = Self::new(1);

        #[doc = "YC is separated before output."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abit_SPEC;
    pub type Abit = crate::EnumBitfieldStruct<u8, Abit_SPEC>;
    impl Abit {
        #[doc = "The alpha value is set to 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "The alpha value is set to 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpsm_SPEC;
    pub type Bpsm = crate::EnumBitfieldStruct<u8, Bpsm_SPEC>;
    impl Bpsm {
        #[doc = "Bytes are not swapped in output data."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bytes are swapped in output data."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exrgb_SPEC;
    pub type Exrgb = crate::EnumBitfieldStruct<u8, Exrgb_SPEC>;
    impl Exrgb {
        #[doc = "RGB data extension processing is not performed."]
        pub const _0: Self = Self::new(0);

        #[doc = "Data is extended to 32-bit RGB conversion when DTMD\\[1:0\\] is set to 00 or 01 as the data conversion mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct YcThr_SPEC;
    pub type YcThr = crate::EnumBitfieldStruct<u8, YcThr_SPEC>;
    impl YcThr {
        #[doc = "Y and CbCr data are transferred to memory in accordance with the setting in the YMODE\\[2:0\\] bits."]
        pub const _0: Self = Self::new(0);

        #[doc = "Y and CbCr data are transferred to memory as 10-bit data in accordance with the input format."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ymode_SPEC;
    pub type Ymode = crate::EnumBitfieldStruct<u8, Ymode_SPEC>;
    impl Ymode {
        #[doc = "Both Y and CbCr data are transferred to memory."]
        pub const _000: Self = Self::new(0);

        #[doc = "Only Y data is transferred to memory as 8-bit data."]
        pub const _001: Self = Self::new(1);

        #[doc = "10-bit Y data and 8-bit CbCr data are transferred to memory"]
        pub const _010: Self = Self::new(2);

        #[doc = "Only Y data is transferred to memory as 10-bit data"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);

        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uvaof_SPEC;
impl crate::sealed::RegSpec for Uvaof_SPEC {
    type DataType = u32;
}

#[doc = "UV Address Offset Register"]
pub type Uvaof = crate::RegValueT<Uvaof_SPEC>;

impl Uvaof {
    #[doc = "UV Data Address Offset"]
    #[inline(always)]
    pub fn uvaof(
        self,
    ) -> crate::common::RegisterField<7, 0x1ffffff, 1, 0, u32, u32, Uvaof_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1ffffff,1,0,u32,u32,Uvaof_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uvaof {
    #[inline(always)]
    fn default() -> Uvaof {
        <crate::RegValueT<Uvaof_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsCtrl_SPEC;
impl crate::sealed::RegSpec for UdsCtrl_SPEC {
    type DataType = u32;
}

#[doc = "Scaling Control Registers"]
pub type UdsCtrl = crate::RegValueT<UdsCtrl_SPEC>;

impl UdsCtrl {
    #[doc = "B/Cb Interpolation Method When Bilinear/Nearest Neighbor Interpolation is Selected"]
    #[inline(always)]
    pub fn ne_bcb(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        uds_ctrl::NeBcb,
        uds_ctrl::NeBcb,
        UdsCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            uds_ctrl::NeBcb,
            uds_ctrl::NeBcb,
            UdsCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "G/Y Interpolation Method When Bilinear/Nearest Neighbor Interpolation is Selected"]
    #[inline(always)]
    pub fn ne_gy(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        uds_ctrl::NeGy,
        uds_ctrl::NeGy,
        UdsCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            uds_ctrl::NeGy,
            uds_ctrl::NeGy,
            UdsCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "R/Cr Interpolation Method When Bilinear/Nearest Neighbor Interpolation is Selected"]
    #[inline(always)]
    pub fn ne_rcr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        uds_ctrl::NeRcr,
        uds_ctrl::NeRcr,
        UdsCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            uds_ctrl::NeRcr,
            uds_ctrl::NeRcr,
            UdsCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pixel Component Interpolation Method at Scale-Up/Down"]
    #[inline(always)]
    pub fn bc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        uds_ctrl::Bc,
        uds_ctrl::Bc,
        UdsCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            uds_ctrl::Bc,
            uds_ctrl::Bc,
            UdsCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BiLinear or nearest neighbor interpolation characteristic ADVanced mode"]
    #[inline(always)]
    pub fn bladv(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, UdsCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, UdsCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Advanced MoDe: Pixel Count at Scale-Up"]
    #[inline(always)]
    pub fn amd(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        uds_ctrl::Amd,
        uds_ctrl::Amd,
        UdsCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            uds_ctrl::Amd,
            uds_ctrl::Amd,
            UdsCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UdsCtrl {
    #[inline(always)]
    fn default() -> UdsCtrl {
        <crate::RegValueT<UdsCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod uds_ctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct NeBcb_SPEC;
    pub type NeBcb = crate::EnumBitfieldStruct<u8, NeBcb_SPEC>;
    impl NeBcb {
        #[doc = "Bilinear method"]
        pub const _0: Self = Self::new(0);

        #[doc = "Nearest neighbor method"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct NeGy_SPEC;
    pub type NeGy = crate::EnumBitfieldStruct<u8, NeGy_SPEC>;
    impl NeGy {
        #[doc = "Bilinear method"]
        pub const _0: Self = Self::new(0);

        #[doc = "Nearest neighbor method"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct NeRcr_SPEC;
    pub type NeRcr = crate::EnumBitfieldStruct<u8, NeRcr_SPEC>;
    impl NeRcr {
        #[doc = "Bilinear method"]
        pub const _0: Self = Self::new(0);

        #[doc = "Nearest neighbor method"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bc_SPEC;
    pub type Bc = crate::EnumBitfieldStruct<u8, Bc_SPEC>;
    impl Bc {
        #[doc = "Bilinear or nearest neighbor interpolation method is used"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interpolation method equivalent to 4 to 17 taps in accordance with the scaling factor is used (multi-tap mode)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amd_SPEC;
    pub type Amd = crate::EnumBitfieldStruct<u8, Amd_SPEC>;
    impl Amd {
        #[doc = "Pixel count after scale-up is 1 + < (n - 1) × scale-up factor>"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pixel count after scale-up is <n × scale-up factor>"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsScale_SPEC;
impl crate::sealed::RegSpec for UdsScale_SPEC {
    type DataType = u32;
}

#[doc = "Scaling Factor Registers"]
pub type UdsScale = crate::RegValueT<UdsScale_SPEC>;

impl UdsScale {
    #[doc = "Multiplier (Fractional Part) of Vertical Scaling Factor"]
    #[inline(always)]
    pub fn vfrac(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, UdsScale_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,UdsScale_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multiplier (Integral Part) of Vertical Scaling Factor"]
    #[inline(always)]
    pub fn vmant(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, UdsScale_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,UdsScale_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multiplier (Fractional Part) of Horizontal Scaling Factor"]
    #[inline(always)]
    pub fn hfrac(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, UdsScale_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,UdsScale_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multiplier (Integral Part) of Horizontal Scaling Factor"]
    #[inline(always)]
    pub fn hmant(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, UdsScale_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,UdsScale_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UdsScale {
    #[inline(always)]
    fn default() -> UdsScale {
        <crate::RegValueT<UdsScale_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsPassBwidth_SPEC;
impl crate::sealed::RegSpec for UdsPassBwidth_SPEC {
    type DataType = u32;
}

#[doc = "Passband Registers"]
pub type UdsPassBwidth = crate::RegValueT<UdsPassBwidth_SPEC>;

impl UdsPassBwidth {
    #[doc = "Vertical Signal Passband at Image Scale-Up/Down"]
    #[inline(always)]
    pub fn bwidth_v(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, UdsPassBwidth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,UdsPassBwidth_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Horizontal Signal Passband at Image Scale-Up/Down"]
    #[inline(always)]
    pub fn bwidth_h(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, UdsPassBwidth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,UdsPassBwidth_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UdsPassBwidth {
    #[inline(always)]
    fn default() -> UdsPassBwidth {
        <crate::RegValueT<UdsPassBwidth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsClipSize_SPEC;
impl crate::sealed::RegSpec for UdsClipSize_SPEC {
    type DataType = u32;
}

#[doc = "UDS Output Size Clipping Registers"]
pub type UdsClipSize = crate::RegValueT<UdsClipSize_SPEC>;

impl UdsClipSize {
    #[doc = "Clipping Size of Vertical Pixel Count after Scale-Up/-Down"]
    #[inline(always)]
    pub fn cl_vsize(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, UdsClipSize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,UdsClipSize_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clipping Size of Horizontal Pixel Count after Scale-Up/-Down"]
    #[inline(always)]
    pub fn cl_hsize(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, UdsClipSize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,UdsClipSize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UdsClipSize {
    #[inline(always)]
    fn default() -> UdsClipSize {
        <crate::RegValueT<UdsClipSize_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutp_SPEC;
impl crate::sealed::RegSpec for Lutp_SPEC {
    type DataType = u32;
}

#[doc = "Lookup Table Pointer Register"]
pub type Lutp = crate::RegValueT<Lutp_SPEC>;

impl Lutp {
    #[doc = "Lookup Table Cr Pointer"]
    #[inline(always)]
    pub fn ltcrpr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Lutp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Lutp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Lookup Table Cb Pointer"]
    #[inline(always)]
    pub fn ltcbpr(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, Lutp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,Lutp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Lookup Table Y Pointer"]
    #[inline(always)]
    pub fn ltypr(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, Lutp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,Lutp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lutp {
    #[inline(always)]
    fn default() -> Lutp {
        <crate::RegValueT<Lutp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutd_SPEC;
impl crate::sealed::RegSpec for Lutd_SPEC {
    type DataType = u32;
}

#[doc = "Lookup Table Data Register"]
pub type Lutd = crate::RegValueT<Lutd_SPEC>;

impl Lutd {
    #[doc = "Lookup Table Cr Data"]
    #[inline(always)]
    pub fn ltcrdt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Lutd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Lutd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Lookup Table Cb Data"]
    #[inline(always)]
    pub fn ltcbdt(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Lutd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Lutd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Lookup Table Y Data"]
    #[inline(always)]
    pub fn ltydt(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Lutd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Lutd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lutd {
    #[inline(always)]
    fn default() -> Lutd {
        <crate::RegValueT<Lutd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Yccr1_SPEC;
impl crate::sealed::RegSpec for Yccr1_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Y Calculation Setting Register 1"]
pub type Yccr1 = crate::RegValueT<Yccr1_SPEC>;

impl Yccr1 {
    #[doc = "R Multiplication Coefficient for Y Calculation"]
    #[inline(always)]
    pub fn yclrp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Yccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Yccr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Yccr1 {
    #[inline(always)]
    fn default() -> Yccr1 {
        <crate::RegValueT<Yccr1_SPEC> as RegisterValue<_>>::new(263)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Yccr2_SPEC;
impl crate::sealed::RegSpec for Yccr2_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Y Calculation Setting Register 2"]
pub type Yccr2 = crate::RegValueT<Yccr2_SPEC>;

impl Yccr2 {
    #[doc = "G Multiplication Coefficient for Y Calculation"]
    #[inline(always)]
    pub fn yclgp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Yccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Yccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B Multiplication Coefficient for Y Calculation"]
    #[inline(always)]
    pub fn yclbp(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Yccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Yccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Yccr2 {
    #[inline(always)]
    fn default() -> Yccr2 {
        <crate::RegValueT<Yccr2_SPEC> as RegisterValue<_>>::new(6554116)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Yccr3_SPEC;
impl crate::sealed::RegSpec for Yccr3_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Y Calculation Setting Register 3"]
pub type Yccr3 = crate::RegValueT<Yccr3_SPEC>;

impl Yccr3 {
    #[doc = "Y Calculation Data Normalized Additional Value"]
    #[inline(always)]
    pub fn yclap(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Yccr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Yccr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Y Calculation Shift Down Result Round-Off Enable"]
    #[inline(always)]
    pub fn yclhen(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        yccr3::Yclhen,
        yccr3::Yclhen,
        Yccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            yccr3::Yclhen,
            yccr3::Yclhen,
            Yccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Y Calculation Shift Down Volume"]
    #[inline(always)]
    pub fn yclsft(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Yccr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Yccr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Yccr3 {
    #[inline(always)]
    fn default() -> Yccr3 {
        <crate::RegValueT<Yccr3_SPEC> as RegisterValue<_>>::new(2315321600)
    }
}
pub mod yccr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Yclhen_SPEC;
    pub type Yclhen = crate::EnumBitfieldStruct<u8, Yclhen_SPEC>;
    impl Yclhen {
        #[doc = "Round down to down shift process"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-off to down shift process is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbccr1_SPEC;
impl crate::sealed::RegSpec for Cbccr1_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Cb Calculation Setting Register 1"]
pub type Cbccr1 = crate::RegValueT<Cbccr1_SPEC>;

impl Cbccr1 {
    #[doc = "R Multiplication Coefficient for Cb Calculation"]
    #[inline(always)]
    pub fn cbclrp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cbccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cbccr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cbccr1 {
    #[inline(always)]
    fn default() -> Cbccr1 {
        <crate::RegValueT<Cbccr1_SPEC> as RegisterValue<_>>::new(8040)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbccr2_SPEC;
impl crate::sealed::RegSpec for Cbccr2_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Cb Calculation Setting Register 2"]
pub type Cbccr2 = crate::RegValueT<Cbccr2_SPEC>;

impl Cbccr2 {
    #[doc = "G Multiplication Coefficient for Cb Calculation"]
    #[inline(always)]
    pub fn cbclgp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cbccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cbccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B Multiplication Coefficient for Cb Calculation"]
    #[inline(always)]
    pub fn cbclbp(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Cbccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Cbccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cbccr2 {
    #[inline(always)]
    fn default() -> Cbccr2 {
        <crate::RegValueT<Cbccr2_SPEC> as RegisterValue<_>>::new(29499094)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbccr3_SPEC;
impl crate::sealed::RegSpec for Cbccr3_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Cb Calculation Setting Register 3"]
pub type Cbccr3 = crate::RegValueT<Cbccr3_SPEC>;

impl Cbccr3 {
    #[doc = "Cb Calculation Data Normalized Additional Value"]
    #[inline(always)]
    pub fn cbclap(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Cbccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Cbccr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cb Calculation Shift Down Result Round-Off Enable"]
    #[inline(always)]
    pub fn cbclhen(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        cbccr3::Cbclhen,
        cbccr3::Cbclhen,
        Cbccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            cbccr3::Cbclhen,
            cbccr3::Cbclhen,
            Cbccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cb Calculation Shift Down Volume"]
    #[inline(always)]
    pub fn cbclsft(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Cbccr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Cbccr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cbccr3 {
    #[inline(always)]
    fn default() -> Cbccr3 {
        <crate::RegValueT<Cbccr3_SPEC> as RegisterValue<_>>::new(2315323392)
    }
}
pub mod cbccr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cbclhen_SPEC;
    pub type Cbclhen = crate::EnumBitfieldStruct<u8, Cbclhen_SPEC>;
    impl Cbclhen {
        #[doc = "Round down to down shift process"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-off to down shift process is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr1_SPEC;
impl crate::sealed::RegSpec for Crccr1_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Cr Calculation Setting Register 1"]
pub type Crccr1 = crate::RegValueT<Crccr1_SPEC>;

impl Crccr1 {
    #[doc = "R Multiplication Coefficient for Cr Calculation"]
    #[inline(always)]
    pub fn crclrp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Crccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Crccr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crccr1 {
    #[inline(always)]
    fn default() -> Crccr1 {
        <crate::RegValueT<Crccr1_SPEC> as RegisterValue<_>>::new(450)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr2_SPEC;
impl crate::sealed::RegSpec for Crccr2_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Cr Calculation Setting Register 2"]
pub type Crccr2 = crate::RegValueT<Crccr2_SPEC>;

impl Crccr2 {
    #[doc = "G Multiplication Coefficient for Cr Calculation"]
    #[inline(always)]
    pub fn crclgp(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Crccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Crccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "B Multiplication Coefficient for Cr Calculation"]
    #[inline(always)]
    pub fn crclbp(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, Crccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,Crccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crccr2 {
    #[inline(always)]
    fn default() -> Crccr2 {
        <crate::RegValueT<Crccr2_SPEC> as RegisterValue<_>>::new(532094599)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr3_SPEC;
impl crate::sealed::RegSpec for Crccr3_SPEC {
    type DataType = u32;
}

#[doc = "RGB to Cr Calculation Setting Register 3"]
pub type Crccr3 = crate::RegValueT<Crccr3_SPEC>;

impl Crccr3 {
    #[doc = "Cr Calculation Data Normalized Additional Value"]
    #[inline(always)]
    pub fn crclap(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Crccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Crccr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cr Calculation Shift Down Result Round-Off Enable"]
    #[inline(always)]
    pub fn crclhen(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        crccr3::Crclhen,
        crccr3::Crclhen,
        Crccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            crccr3::Crclhen,
            crccr3::Crclhen,
            Crccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Cr Calculation Shift Down Volume"]
    #[inline(always)]
    pub fn crclsft(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Crccr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Crccr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crccr3 {
    #[inline(always)]
    fn default() -> Crccr3 {
        <crate::RegValueT<Crccr3_SPEC> as RegisterValue<_>>::new(2315323392)
    }
}
pub mod crccr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crclhen_SPEC;
    pub type Crclhen = crate::EnumBitfieldStruct<u8, Crclhen_SPEC>;
    impl Crclhen {
        #[doc = "Round down to down shift process"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-off to down shift process is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csce1_SPEC;
impl crate::sealed::RegSpec for Csce1_SPEC {
    type DataType = u32;
}

#[doc = "YC to RGB Calculation Setting Extension Register 1"]
pub type Csce1 = crate::RegValueT<Csce1_SPEC>;

impl Csce1 {
    #[doc = "Y Multiplication Coefficient 2 for RGB Calculation"]
    #[inline(always)]
    pub fn ymul2(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Csce1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,Csce1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ROUND off enable"]
    #[inline(always)]
    pub fn round(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        csce1::Round,
        csce1::Round,
        Csce1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            csce1::Round,
            csce1::Round,
            Csce1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Csce1 {
    #[inline(always)]
    fn default() -> Csce1 {
        <crate::RegValueT<Csce1_SPEC> as RegisterValue<_>>::new(4767)
    }
}
pub mod csce1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Round_SPEC;
    pub type Round = crate::EnumBitfieldStruct<u8, Round_SPEC>;
    impl Round {
        #[doc = "Round down"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round off"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csce2_SPEC;
impl crate::sealed::RegSpec for Csce2_SPEC {
    type DataType = u32;
}

#[doc = "YC to RGB Calculation Setting Extension Register 2"]
pub type Csce2 = crate::RegValueT<Csce2_SPEC>;

impl Csce2 {
    #[doc = "CbCr Subtraction Coefficient 2 for RGB Calculation"]
    #[inline(always)]
    pub fn csub2(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Csce2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Csce2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Y Subtraction Coefficient 2 for RGB Calculation"]
    #[inline(always)]
    pub fn ysub2(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Csce2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Csce2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Csce2 {
    #[inline(always)]
    fn default() -> Csce2 {
        <crate::RegValueT<Csce2_SPEC> as RegisterValue<_>>::new(16779264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csce3_SPEC;
impl crate::sealed::RegSpec for Csce3_SPEC {
    type DataType = u32;
}

#[doc = "YC to RGB Calculation Setting Extension Register 3"]
pub type Csce3 = crate::RegValueT<Csce3_SPEC>;

impl Csce3 {
    #[doc = "Cr Multiplication Coefficient 2 for G Calculation"]
    #[inline(always)]
    pub fn gcrmul2(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Csce3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,Csce3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cr Multiplication Coefficient 2 for R Calculation"]
    #[inline(always)]
    pub fn rcrmul2(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, Csce3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,Csce3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Csce3 {
    #[inline(always)]
    fn default() -> Csce3 {
        <crate::RegValueT<Csce3_SPEC> as RegisterValue<_>>::new(428412162)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csce4_SPEC;
impl crate::sealed::RegSpec for Csce4_SPEC {
    type DataType = u32;
}

#[doc = "YC to RGB Calculation Setting Extension Register 4"]
pub type Csce4 = crate::RegValueT<Csce4_SPEC>;

impl Csce4 {
    #[doc = "Cb Multiplication Coefficient 2 for B Calculation"]
    #[inline(always)]
    pub fn bcbmul2(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Csce4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,Csce4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cb Multiplication Coefficient 2 for G Calculation"]
    #[inline(always)]
    pub fn gcbmul2(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, Csce4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,Csce4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Csce4 {
    #[inline(always)]
    fn default() -> Csce4 {
        <crate::RegValueT<Csce4_SPEC> as RegisterValue<_>>::new(105193541)
    }
}
