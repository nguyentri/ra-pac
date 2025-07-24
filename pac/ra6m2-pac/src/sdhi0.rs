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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:16 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SD Host Interface 0"]
unsafe impl ::core::marker::Send for super::Sdhi0 {}
unsafe impl ::core::marker::Sync for super::Sdhi0 {}
impl super::Sdhi0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Command Type Register"]
    #[inline(always)]
    pub const fn sd_cmd(&self) -> &'static crate::common::Reg<self::SdCmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdCmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "SD Command Argument Register"]
    #[inline(always)]
    pub const fn sd_arg(&self) -> &'static crate::common::Reg<self::SdArg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdArg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "SD Command Argument Register 1"]
    #[inline(always)]
    pub const fn sd_arg1(
        &self,
    ) -> &'static crate::common::Reg<self::SdArg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdArg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Data Stop Register"]
    #[inline(always)]
    pub const fn sd_stop(
        &self,
    ) -> &'static crate::common::Reg<self::SdStop_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdStop_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Block Count Register"]
    #[inline(always)]
    pub const fn sd_seccnt(
        &self,
    ) -> &'static crate::common::Reg<self::SdSeccnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdSeccnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "SD Card Response Register 10"]
    #[inline(always)]
    pub const fn sd_rsp10(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp10_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp10_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "SD Card Response Register 1"]
    #[inline(always)]
    pub const fn sd_rsp1(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "SD Card Response Register 32"]
    #[inline(always)]
    pub const fn sd_rsp32(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp32_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp32_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "SD Card Response Register 3"]
    #[inline(always)]
    pub const fn sd_rsp3(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "SD Card Response Register 54"]
    #[inline(always)]
    pub const fn sd_rsp54(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp54_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp54_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "SD Card Response Register 5"]
    #[inline(always)]
    pub const fn sd_rsp5(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp5_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp5_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "SD Card Response Register 76"]
    #[inline(always)]
    pub const fn sd_rsp76(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp76_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp76_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "SD Card Response Register 7"]
    #[inline(always)]
    pub const fn sd_rsp7(
        &self,
    ) -> &'static crate::common::Reg<self::SdRsp7_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdRsp7_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "SD Card Interrupt Flag Register 1"]
    #[inline(always)]
    pub const fn sd_info1(
        &self,
    ) -> &'static crate::common::Reg<self::SdInfo1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdInfo1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "SD Card Interrupt Flag Register 2"]
    #[inline(always)]
    pub const fn sd_info2(
        &self,
    ) -> &'static crate::common::Reg<self::SdInfo2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdInfo2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "SD_INFO1 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn sd_info1_mask(
        &self,
    ) -> &'static crate::common::Reg<self::SdInfo1Mask_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdInfo1Mask_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "SD_INFO2 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn sd_info2_mask(
        &self,
    ) -> &'static crate::common::Reg<self::SdInfo2Mask_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdInfo2Mask_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "SD Clock Control Register"]
    #[inline(always)]
    pub const fn sd_clk_ctrl(
        &self,
    ) -> &'static crate::common::Reg<self::SdClkCtrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdClkCtrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Transfer Data Length Register"]
    #[inline(always)]
    pub const fn sd_size(
        &self,
    ) -> &'static crate::common::Reg<self::SdSize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdSize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "SD Card Access Control Option Register"]
    #[inline(always)]
    pub const fn sd_option(
        &self,
    ) -> &'static crate::common::Reg<self::SdOption_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdOption_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "SD Error Status Register 1"]
    #[inline(always)]
    pub const fn sd_err_sts1(
        &self,
    ) -> &'static crate::common::Reg<self::SdErrSts1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdErrSts1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "SD Error Status Register 2"]
    #[inline(always)]
    pub const fn sd_err_sts2(
        &self,
    ) -> &'static crate::common::Reg<self::SdErrSts2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SdErrSts2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "SD Buffer Register"]
    #[inline(always)]
    pub const fn sd_buf0(
        &self,
    ) -> &'static crate::common::Reg<self::SdBuf0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdBuf0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "SDIO Mode Control Register"]
    #[inline(always)]
    pub const fn sdio_mode(
        &self,
    ) -> &'static crate::common::Reg<self::SdioMode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdioMode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "SDIO Interrupt Flag Register 1"]
    #[inline(always)]
    pub const fn sdio_info1(
        &self,
    ) -> &'static crate::common::Reg<self::SdioInfo1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdioInfo1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "SDIO_INFO1 Interrupt Mask Register"]
    #[inline(always)]
    pub const fn sdio_info1_mask(
        &self,
    ) -> &'static crate::common::Reg<self::SdioInfo1Mask_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdioInfo1Mask_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "DMA Mode Enable Register"]
    #[inline(always)]
    pub const fn sd_dmaen(
        &self,
    ) -> &'static crate::common::Reg<self::SdDmaen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdDmaen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(432usize),
            )
        }
    }

    #[doc = "Software Reset Register"]
    #[inline(always)]
    pub const fn soft_rst(
        &self,
    ) -> &'static crate::common::Reg<self::SoftRst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SoftRst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(448usize),
            )
        }
    }

    #[doc = "SD Interface Mode Setting Register"]
    #[inline(always)]
    pub const fn sdif_mode(
        &self,
    ) -> &'static crate::common::Reg<self::SdifMode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdifMode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(460usize),
            )
        }
    }

    #[doc = "Swap Control Register"]
    #[inline(always)]
    pub const fn ext_swap(
        &self,
    ) -> &'static crate::common::Reg<self::ExtSwap_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ExtSwap_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(480usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdCmd_SPEC;
impl crate::sealed::RegSpec for SdCmd_SPEC {
    type DataType = u32;
}

#[doc = "Command Type Register"]
pub type SdCmd = crate::RegValueT<SdCmd_SPEC>;

impl SdCmd {
    #[doc = "Multiple Block Transfer Mode (enabled at multiple block transfer)"]
    #[inline(always)]
    pub fn cmd12at(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        sd_cmd::Cmd12At,
        sd_cmd::Cmd12At,
        SdCmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            sd_cmd::Cmd12At,
            sd_cmd::Cmd12At,
            SdCmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Single/Multiple Block Transfer (enabled when the command with data is handled)"]
    #[inline(always)]
    pub fn trstp(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        sd_cmd::Trstp,
        sd_cmd::Trstp,
        SdCmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            sd_cmd::Trstp,
            sd_cmd::Trstp,
            SdCmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write/Read Mode (enabled when the command with data is handled)"]
    #[inline(always)]
    pub fn cmdrw(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sd_cmd::Cmdrw,
        sd_cmd::Cmdrw,
        SdCmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sd_cmd::Cmdrw,
            sd_cmd::Cmdrw,
            SdCmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Mode (Command Type)"]
    #[inline(always)]
    pub fn cmdtp(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        sd_cmd::Cmdtp,
        sd_cmd::Cmdtp,
        SdCmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            sd_cmd::Cmdtp,
            sd_cmd::Cmdtp,
            SdCmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mode/Response TypeNOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type."]
    #[inline(always)]
    pub fn rsptp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        sd_cmd::Rsptp,
        sd_cmd::Rsptp,
        SdCmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            sd_cmd::Rsptp,
            sd_cmd::Rsptp,
            SdCmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Type Select"]
    #[inline(always)]
    pub fn acmd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        sd_cmd::Acmd,
        sd_cmd::Acmd,
        SdCmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            sd_cmd::Acmd,
            sd_cmd::Acmd,
            SdCmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command IndexThese bits specify Command Format\\[45:40\\] (command index).\\[Examples\\]CMD6: SD_CMD\\[7:0\\] = 8\'b00_000110CMD18: SD_CMD\\[7:0\\] = 8\'b00_010010ACMD13: SD_CMD\\[7:0\\] = 8\'b01_001101"]
    #[inline(always)]
    pub fn cmdidx(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, SdCmd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,SdCmd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdCmd {
    #[inline(always)]
    fn default() -> SdCmd {
        <crate::RegValueT<SdCmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sd_cmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmd12At_SPEC;
    pub type Cmd12At = crate::EnumBitfieldStruct<u8, Cmd12At_SPEC>;
    impl Cmd12At {
        #[doc = "CMD12 is automatically issued at multiple block transfer."]
        pub const _00: Self = Self::new(0);

        #[doc = "CMD12 is not automatically issued at multiple block transfer."]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trstp_SPEC;
    pub type Trstp = crate::EnumBitfieldStruct<u8, Trstp_SPEC>;
    impl Trstp {
        #[doc = "Single block transfer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multiple block transfer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdrw_SPEC;
    pub type Cmdrw = crate::EnumBitfieldStruct<u8, Cmdrw_SPEC>;
    impl Cmdrw {
        #[doc = "Write (SD/MMC host interface -> SD card/MMC)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read (SD/MMC host interface <- SD card/MMC)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdtp_SPEC;
    pub type Cmdtp = crate::EnumBitfieldStruct<u8, Cmdtp_SPEC>;
    impl Cmdtp {
        #[doc = "Command does not include data transfer (bc, bcr, or ac)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Command includes data transfer (adtc)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsptp_SPEC;
    pub type Rsptp = crate::EnumBitfieldStruct<u8, Rsptp_SPEC>;
    impl Rsptp {
        #[doc = "Normal mode The response type and the transfer mode are selected by SD_CMD\\[7:0\\], and the SD_CMD\\[15:11\\] setting is disabled."]
        pub const _000: Self = Self::new(0);

        #[doc = "Expansion mode and no response"]
        pub const _011: Self = Self::new(3);

        #[doc = "Expansion mode and R1, R5, R6, or R7 response"]
        pub const _100: Self = Self::new(4);

        #[doc = "Expansion mode and R1b response"]
        pub const _101: Self = Self::new(5);

        #[doc = "Expansion mode and R2 response"]
        pub const _110: Self = Self::new(6);

        #[doc = "Expansion mode and R3 or R4 response"]
        pub const _111: Self = Self::new(7);

        #[doc = "Settings prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acmd_SPEC;
    pub type Acmd = crate::EnumBitfieldStruct<u8, Acmd_SPEC>;
    impl Acmd {
        #[doc = "CMD"]
        pub const _00: Self = Self::new(0);

        #[doc = "ACMD"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdArg_SPEC;
impl crate::sealed::RegSpec for SdArg_SPEC {
    type DataType = u32;
}

#[doc = "SD Command Argument Register"]
pub type SdArg = crate::RegValueT<SdArg_SPEC>;

impl SdArg {
    #[doc = "Argument RegisterSet command format\\[39:8\\] (argument)"]
    #[inline(always)]
    pub fn sd_arg(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SdArg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SdArg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdArg {
    #[inline(always)]
    fn default() -> SdArg {
        <crate::RegValueT<SdArg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdArg1_SPEC;
impl crate::sealed::RegSpec for SdArg1_SPEC {
    type DataType = u32;
}

#[doc = "SD Command Argument Register 1"]
pub type SdArg1 = crate::RegValueT<SdArg1_SPEC>;

impl SdArg1 {
    #[doc = "Argument Register 1Set command format\\[39:24\\] (argument)"]
    #[inline(always)]
    pub fn sd_arg1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SdArg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SdArg1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdArg1 {
    #[inline(always)]
    fn default() -> SdArg1 {
        <crate::RegValueT<SdArg1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdStop_SPEC;
impl crate::sealed::RegSpec for SdStop_SPEC {
    type DataType = u32;
}

#[doc = "Data Stop Register"]
pub type SdStop = crate::RegValueT<SdStop_SPEC>;

impl SdStop {
    #[doc = "Block Count EnableSet SEC to 1 at multiple block transfer.When SD_CMD is set as follows to start the command sequence while SEC is set to 1, CMD12 is automatically issued to stop multi-block transfer with the number of blocks which is set to SD_SECCNT.1. CMD18 or CMD25 in normal mode (SD_CMD\\[10:8\\] = 000)2. SD_CMD\\[15:13\\] = 001 in extended mode (CMD12 is automatically issued, multiple block transfer)When the command sequence is halted because of a communications error or timeout, CMD12 is not automatically issued.NOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1."]
    #[inline(always)]
    pub fn sec(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_stop::Sec,
        sd_stop::Sec,
        SdStop_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_stop::Sec,
            sd_stop::Sec,
            SdStop_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop- When STP is set to 1 during multiple block transfer, CMD12 is issued to halt the transfer through the SD host interface.However, if a command sequence is halted because of a communications error or timeout, CMD12 is not issued. Although continued buffer access is possible even after STP has been set to 1, the buffer access error bit (ERR5 or ERR4) in SD_INFO2 will be set accordingly.- When STP has been set to 1 during transfer for single block write, the access end flag is set when SD_BUF becomes empty, and CMD12 is not issued. If SD_BUF does contain data, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP has been set to 1 during transfer for single block read, the access end flag is set immediately after setting of the STP bit and CMD12 is not issued.- When STP is set to 1 during reception of the busy state after an R1b response, the access end flag is set on completion of reception of the busy state without CMD12 having been issued.- When STP is set to 1 after a command sequence has been completed, CMD12 is not issued and the access end flag is not set.- Set STP to 1 after the response end flag has been set.- Set STP to 0 after the response end flag has been set."]
    #[inline(always)]
    pub fn stp(self) -> crate::common::RegisterFieldBool<0, 1, 0, SdStop_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SdStop_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for SdStop {
    #[inline(always)]
    fn default() -> SdStop {
        <crate::RegValueT<SdStop_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sd_stop {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sec_SPEC;
    pub type Sec = crate::EnumBitfieldStruct<u8, Sec_SPEC>;
    impl Sec {
        #[doc = "Disables SD_SECCNT setting value."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables SD_SECCNT setting value."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdSeccnt_SPEC;
impl crate::sealed::RegSpec for SdSeccnt_SPEC {
    type DataType = u32;
}

#[doc = "Block Count Register"]
pub type SdSeccnt = crate::RegValueT<SdSeccnt_SPEC>;

impl SdSeccnt {
    #[doc = "Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1."]
    #[inline(always)]
    pub fn sd_seccnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SdSeccnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SdSeccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdSeccnt {
    #[inline(always)]
    fn default() -> SdSeccnt {
        <crate::RegValueT<SdSeccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp10_SPEC;
impl crate::sealed::RegSpec for SdRsp10_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 10"]
pub type SdRsp10 = crate::RegValueT<SdRsp10_SPEC>;

impl SdRsp10 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp10(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SdRsp10_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SdRsp10_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp10 {
    #[inline(always)]
    fn default() -> SdRsp10 {
        <crate::RegValueT<SdRsp10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp1_SPEC;
impl crate::sealed::RegSpec for SdRsp1_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 1"]
pub type SdRsp1 = crate::RegValueT<SdRsp1_SPEC>;

impl SdRsp1 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SdRsp1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SdRsp1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp1 {
    #[inline(always)]
    fn default() -> SdRsp1 {
        <crate::RegValueT<SdRsp1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp32_SPEC;
impl crate::sealed::RegSpec for SdRsp32_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 32"]
pub type SdRsp32 = crate::RegValueT<SdRsp32_SPEC>;

impl SdRsp32 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp32(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SdRsp32_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SdRsp32_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp32 {
    #[inline(always)]
    fn default() -> SdRsp32 {
        <crate::RegValueT<SdRsp32_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp3_SPEC;
impl crate::sealed::RegSpec for SdRsp3_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 3"]
pub type SdRsp3 = crate::RegValueT<SdRsp3_SPEC>;

impl SdRsp3 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp3(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SdRsp3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SdRsp3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp3 {
    #[inline(always)]
    fn default() -> SdRsp3 {
        <crate::RegValueT<SdRsp3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp54_SPEC;
impl crate::sealed::RegSpec for SdRsp54_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 54"]
pub type SdRsp54 = crate::RegValueT<SdRsp54_SPEC>;

impl SdRsp54 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp54(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SdRsp54_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SdRsp54_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp54 {
    #[inline(always)]
    fn default() -> SdRsp54 {
        <crate::RegValueT<SdRsp54_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp5_SPEC;
impl crate::sealed::RegSpec for SdRsp5_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 5"]
pub type SdRsp5 = crate::RegValueT<SdRsp5_SPEC>;

impl SdRsp5 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp5(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SdRsp5_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SdRsp5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp5 {
    #[inline(always)]
    fn default() -> SdRsp5 {
        <crate::RegValueT<SdRsp5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp76_SPEC;
impl crate::sealed::RegSpec for SdRsp76_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 76"]
pub type SdRsp76 = crate::RegValueT<SdRsp76_SPEC>;

impl SdRsp76 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp76(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, SdRsp76_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,SdRsp76_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp76 {
    #[inline(always)]
    fn default() -> SdRsp76 {
        <crate::RegValueT<SdRsp76_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRsp7_SPEC;
impl crate::sealed::RegSpec for SdRsp7_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Response Register 7"]
pub type SdRsp7 = crate::RegValueT<SdRsp7_SPEC>;

impl SdRsp7 {
    #[doc = "Store the response from the SD card/MMC"]
    #[inline(always)]
    pub fn sd_rsp7(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SdRsp7_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,SdRsp7_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdRsp7 {
    #[inline(always)]
    fn default() -> SdRsp7 {
        <crate::RegValueT<SdRsp7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdInfo1_SPEC;
impl crate::sealed::RegSpec for SdInfo1_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Interrupt Flag Register 1"]
pub type SdInfo1 = crate::RegValueT<SdInfo1_SPEC>;

impl SdInfo1 {
    #[doc = "Inticates the SDnDAT3 State"]
    #[inline(always)]
    pub fn sdd3mon(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sd_info1::Sdd3Mon,
        sd_info1::Sdd3Mon,
        SdInfo1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sd_info1::Sdd3Mon,
            sd_info1::Sdd3Mon,
            SdInfo1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SDnDAT3 Card Insertion"]
    #[inline(always)]
    pub fn sdd3in(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sd_info1::Sdd3In,
        sd_info1::Sdd3In,
        SdInfo1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sd_info1::Sdd3In,
            sd_info1::Sdd3In,
            SdInfo1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDnDAT3 Card Removal"]
    #[inline(always)]
    pub fn sdd3rm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_info1::Sdd3Rm,
        sd_info1::Sdd3Rm,
        SdInfo1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_info1::Sdd3Rm,
            sd_info1::Sdd3Rm,
            SdInfo1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Indicates the SDnWP state"]
    #[inline(always)]
    pub fn sdwpmon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sd_info1::Sdwpmon,
        sd_info1::Sdwpmon,
        SdInfo1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sd_info1::Sdwpmon,
            sd_info1::Sdwpmon,
            SdInfo1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Indicates the SDnCD state"]
    #[inline(always)]
    pub fn sdcdmon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sd_info1::Sdcdmon,
        sd_info1::Sdcdmon,
        SdInfo1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sd_info1::Sdcdmon,
            sd_info1::Sdcdmon,
            SdInfo1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SDnCD Card Insertion"]
    #[inline(always)]
    pub fn sdcdin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sd_info1::Sdcdin,
        sd_info1::Sdcdin,
        SdInfo1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sd_info1::Sdcdin,
            sd_info1::Sdcdin,
            SdInfo1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDnCD  Card Removal"]
    #[inline(always)]
    pub fn sdcdrm(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sd_info1::Sdcdrm,
        sd_info1::Sdcdrm,
        SdInfo1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sd_info1::Sdcdrm,
            sd_info1::Sdcdrm,
            SdInfo1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Access End"]
    #[inline(always)]
    pub fn acend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sd_info1::Acend,
        sd_info1::Acend,
        SdInfo1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sd_info1::Acend,
            sd_info1::Acend,
            SdInfo1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Response End Detection"]
    #[inline(always)]
    pub fn rspend(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sd_info1::Rspend,
        sd_info1::Rspend,
        SdInfo1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sd_info1::Rspend,
            sd_info1::Rspend,
            SdInfo1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdInfo1 {
    #[inline(always)]
    fn default() -> SdInfo1 {
        <crate::RegValueT<SdInfo1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sd_info1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdd3Mon_SPEC;
    pub type Sdd3Mon = crate::EnumBitfieldStruct<u8, Sdd3Mon_SPEC>;
    impl Sdd3Mon {
        #[doc = "SDnDAT3 is set to 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "SDnDAT3 is set to 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdd3In_SPEC;
    pub type Sdd3In = crate::EnumBitfieldStruct<u8, Sdd3In_SPEC>;
    impl Sdd3In {
        #[doc = "SD card insertion not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "SD card insertion detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdd3Rm_SPEC;
    pub type Sdd3Rm = crate::EnumBitfieldStruct<u8, Sdd3Rm_SPEC>;
    impl Sdd3Rm {
        #[doc = "SD card removal not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "SD card removal detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdwpmon_SPEC;
    pub type Sdwpmon = crate::EnumBitfieldStruct<u8, Sdwpmon_SPEC>;
    impl Sdwpmon {
        #[doc = "SDnWP is set to 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "SDnWP is set to 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdcdmon_SPEC;
    pub type Sdcdmon = crate::EnumBitfieldStruct<u8, Sdcdmon_SPEC>;
    impl Sdcdmon {
        #[doc = "Indicates that Mcycle has elapsed with SDnCD held 1.(Mcycle is set by bits 3 to 0 in SD_OPTION.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that Mcycle has elapsed with SDnCD held 0. (Mcycle is set by bits 3 to 0 in SD_OPTION.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdcdin_SPEC;
    pub type Sdcdin = crate::EnumBitfieldStruct<u8, Sdcdin_SPEC>;
    impl Sdcdin {
        #[doc = "Card insertion not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Card insertion detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdcdrm_SPEC;
    pub type Sdcdrm = crate::EnumBitfieldStruct<u8, Sdcdrm_SPEC>;
    impl Sdcdrm {
        #[doc = "Card removal not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Card removal detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acend_SPEC;
    pub type Acend = crate::EnumBitfieldStruct<u8, Acend_SPEC>;
    impl Acend {
        #[doc = "Access end is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Access end is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspend_SPEC;
    pub type Rspend = crate::EnumBitfieldStruct<u8, Rspend_SPEC>;
    impl Rspend {
        #[doc = "Response end is not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Response end is detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdInfo2_SPEC;
impl crate::sealed::RegSpec for SdInfo2_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Interrupt Flag Register 2"]
pub type SdInfo2 = crate::RegValueT<SdInfo2_SPEC>;

impl SdInfo2 {
    #[doc = "Illegal Access Error"]
    #[inline(always)]
    pub fn ila(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sd_info2::Ila,
        sd_info2::Ila,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sd_info2::Ila,
            sd_info2::Ila,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Type Register Busy"]
    #[inline(always)]
    pub fn cbsy(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sd_info2::Cbsy,
        sd_info2::Cbsy,
        SdInfo2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sd_info2::Cbsy,
            sd_info2::Cbsy,
            SdInfo2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "When a command sequence is started by writing to SD_CMD, the CBSY bit is set to 1 and, at the same time, the SCLKDIVEN bit is set to 0. The SCLKDIVEN bit is set to 1 after 8 cycles of SDCLK have elapsed after setting of the CBSY bit to 0 due to completion of the command sequence."]
    #[inline(always)]
    pub fn sd_clk_ctrlen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        sd_info2::SdClkCtrlen,
        sd_info2::SdClkCtrlen,
        SdInfo2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            sd_info2::SdClkCtrlen,
            sd_info2::SdClkCtrlen,
            SdInfo2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SD_BUF Write Enable"]
    #[inline(always)]
    pub fn bwe(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sd_info2::Bwe,
        sd_info2::Bwe,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sd_info2::Bwe,
            sd_info2::Bwe,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD_BUF Read Enable"]
    #[inline(always)]
    pub fn bre(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_info2::Bre,
        sd_info2::Bre,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_info2::Bre,
            sd_info2::Bre,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDDAT0Indicates the SDDAT0 state of the port specified by SD_PORTSEL."]
    #[inline(always)]
    pub fn sdd0mon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sd_info2::Sdd0Mon,
        sd_info2::Sdd0Mon,
        SdInfo2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sd_info2::Sdd0Mon,
            sd_info2::Sdd0Mon,
            SdInfo2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Timeout"]
    #[inline(always)]
    pub fn rspto(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sd_info2::Rspto,
        sd_info2::Rspto,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sd_info2::Rspto,
            sd_info2::Rspto,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD_BUF Illegal Read Access"]
    #[inline(always)]
    pub fn ilr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sd_info2::Ilr,
        sd_info2::Ilr,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sd_info2::Ilr,
            sd_info2::Ilr,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD_BUF Illegal Write Access"]
    #[inline(always)]
    pub fn ilw(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sd_info2::Ilw,
        sd_info2::Ilw,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sd_info2::Ilw,
            sd_info2::Ilw,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Timeout"]
    #[inline(always)]
    pub fn dto(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sd_info2::Dto,
        sd_info2::Dto,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sd_info2::Dto,
            sd_info2::Dto,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "END Error"]
    #[inline(always)]
    pub fn ende(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sd_info2::Ende,
        sd_info2::Ende,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sd_info2::Ende,
            sd_info2::Ende,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn crce(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sd_info2::Crce,
        sd_info2::Crce,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sd_info2::Crce,
            sd_info2::Crce,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Error"]
    #[inline(always)]
    pub fn cmde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sd_info2::Cmde,
        sd_info2::Cmde,
        SdInfo2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sd_info2::Cmde,
            sd_info2::Cmde,
            SdInfo2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdInfo2 {
    #[inline(always)]
    fn default() -> SdInfo2 {
        <crate::RegValueT<SdInfo2_SPEC> as RegisterValue<_>>::new(8192)
    }
}
pub mod sd_info2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ila_SPEC;
    pub type Ila = crate::EnumBitfieldStruct<u8, Ila_SPEC>;
    impl Ila {
        #[doc = "Illegal access error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Illegal access error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cbsy_SPEC;
    pub type Cbsy = crate::EnumBitfieldStruct<u8, Cbsy_SPEC>;
    impl Cbsy {
        #[doc = "A command sequence is being executed."]
        pub const _0: Self = Self::new(0);

        #[doc = "A command sequence has been completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SdClkCtrlen_SPEC;
    pub type SdClkCtrlen = crate::EnumBitfieldStruct<u8, SdClkCtrlen_SPEC>;
    impl SdClkCtrlen {
        #[doc = "The SD/MMC bus (CMD, DAT) is busy. Writing to the SCLKEN and DIV bits in SD_CLK_CTRL is not possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SD/MMC bus (CMD, DAT) is not busy."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bwe_SPEC;
    pub type Bwe = crate::EnumBitfieldStruct<u8, Bwe_SPEC>;
    impl Bwe {
        #[doc = "Data can be written in SD_BUF0."]
        pub const _1: Self = Self::new(1);

        #[doc = "Data cannot be written in SD_BUF0."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bre_SPEC;
    pub type Bre = crate::EnumBitfieldStruct<u8, Bre_SPEC>;
    impl Bre {
        #[doc = "Data can be read from SD_BUF0."]
        pub const _1: Self = Self::new(1);

        #[doc = "Data cannot be read from SD_BUF0."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdd0Mon_SPEC;
    pub type Sdd0Mon = crate::EnumBitfieldStruct<u8, Sdd0Mon_SPEC>;
    impl Sdd0Mon {
        #[doc = "SDDAT0 is set to 1."]
        pub const _1: Self = Self::new(1);

        #[doc = "SDDAT0 is set to 0."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspto_SPEC;
    pub type Rspto = crate::EnumBitfieldStruct<u8, Rspto_SPEC>;
    impl Rspto {
        #[doc = "Response timeout not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Response timeout detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilr_SPEC;
    pub type Ilr = crate::EnumBitfieldStruct<u8, Ilr_SPEC>;
    impl Ilr {
        #[doc = "Illegal read access to the SD_BUF register not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Illegal read access to the SD_BUF register detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilw_SPEC;
    pub type Ilw = crate::EnumBitfieldStruct<u8, Ilw_SPEC>;
    impl Ilw {
        #[doc = "Illegal write access to the SD_BUF register not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Illegal write access to the SD_BUF register detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dto_SPEC;
    pub type Dto = crate::EnumBitfieldStruct<u8, Dto_SPEC>;
    impl Dto {
        #[doc = "Data timeout not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data timeout detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ende_SPEC;
    pub type Ende = crate::EnumBitfieldStruct<u8, Ende_SPEC>;
    impl Ende {
        #[doc = "End bit error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "End bit error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crce_SPEC;
    pub type Crce = crate::EnumBitfieldStruct<u8, Crce_SPEC>;
    impl Crce {
        #[doc = "CRC error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "CRC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmde_SPEC;
    pub type Cmde = crate::EnumBitfieldStruct<u8, Cmde_SPEC>;
    impl Cmde {
        #[doc = "Command error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Command error detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdInfo1Mask_SPEC;
impl crate::sealed::RegSpec for SdInfo1Mask_SPEC {
    type DataType = u32;
}

#[doc = "SD_INFO1 Interrupt Mask Register"]
pub type SdInfo1Mask = crate::RegValueT<SdInfo1Mask_SPEC>;

impl SdInfo1Mask {
    #[doc = "SDnDAT3 Card Insertion Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdd3inm(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sd_info1_mask::Sdd3Inm,
        sd_info1_mask::Sdd3Inm,
        SdInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sd_info1_mask::Sdd3Inm,
            sd_info1_mask::Sdd3Inm,
            SdInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDnDAT3 Card Removal Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdd3rmm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_info1_mask::Sdd3Rmm,
        sd_info1_mask::Sdd3Rmm,
        SdInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_info1_mask::Sdd3Rmm,
            sd_info1_mask::Sdd3Rmm,
            SdInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDnCD card Insertion Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdcdinm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sd_info1_mask::Sdcdinm,
        sd_info1_mask::Sdcdinm,
        SdInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sd_info1_mask::Sdcdinm,
            sd_info1_mask::Sdcdinm,
            SdInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDnCD card Removal Interrupt Request Mask"]
    #[inline(always)]
    pub fn sdcdrmm(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sd_info1_mask::Sdcdrmm,
        sd_info1_mask::Sdcdrmm,
        SdInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sd_info1_mask::Sdcdrmm,
            sd_info1_mask::Sdcdrmm,
            SdInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Access End Interrupt Request Mask"]
    #[inline(always)]
    pub fn acendm(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sd_info1_mask::Acendm,
        sd_info1_mask::Acendm,
        SdInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sd_info1_mask::Acendm,
            sd_info1_mask::Acendm,
            SdInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Response End Interrupt Request Mask"]
    #[inline(always)]
    pub fn rspendm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sd_info1_mask::Rspendm,
        sd_info1_mask::Rspendm,
        SdInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sd_info1_mask::Rspendm,
            sd_info1_mask::Rspendm,
            SdInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdInfo1Mask {
    #[inline(always)]
    fn default() -> SdInfo1Mask {
        <crate::RegValueT<SdInfo1Mask_SPEC> as RegisterValue<_>>::new(797)
    }
}
pub mod sd_info1_mask {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdd3Inm_SPEC;
    pub type Sdd3Inm = crate::EnumBitfieldStruct<u8, Sdd3Inm_SPEC>;
    impl Sdd3Inm {
        #[doc = "SD card insertion interrupt request by the SDnDAT3  is not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "SD card insertion interrupt request by the SDnDAT3 is masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdd3Rmm_SPEC;
    pub type Sdd3Rmm = crate::EnumBitfieldStruct<u8, Sdd3Rmm_SPEC>;
    impl Sdd3Rmm {
        #[doc = "SD card removal interrupt request by the SDnDAT3 is not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "SD card removal interrupt request by the SDnDAT3 is masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdcdinm_SPEC;
    pub type Sdcdinm = crate::EnumBitfieldStruct<u8, Sdcdinm_SPEC>;
    impl Sdcdinm {
        #[doc = "Card insertion interrupt request by the SDnCD is not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Card insertion interrupt request by the SDnCD is masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdcdrmm_SPEC;
    pub type Sdcdrmm = crate::EnumBitfieldStruct<u8, Sdcdrmm_SPEC>;
    impl Sdcdrmm {
        #[doc = "Card removal interrupt request by the by the SDnCD is not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Card removal interrupt request by the by the SDnCD is masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acendm_SPEC;
    pub type Acendm = crate::EnumBitfieldStruct<u8, Acendm_SPEC>;
    impl Acendm {
        #[doc = "Access end interrupt request is not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Access end interrupt request is masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspendm_SPEC;
    pub type Rspendm = crate::EnumBitfieldStruct<u8, Rspendm_SPEC>;
    impl Rspendm {
        #[doc = "Response end interrupt request is not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Response end interrupt request is masked"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdInfo2Mask_SPEC;
impl crate::sealed::RegSpec for SdInfo2Mask_SPEC {
    type DataType = u32;
}

#[doc = "SD_INFO2 Interrupt Mask Register"]
pub type SdInfo2Mask = crate::RegValueT<SdInfo2Mask_SPEC>;

impl SdInfo2Mask {
    #[doc = "Illegal Access Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn ilam(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sd_info2_mask::Ilam,
        sd_info2_mask::Ilam,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sd_info2_mask::Ilam,
            sd_info2_mask::Ilam,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BWE Interrupt Request Mask"]
    #[inline(always)]
    pub fn bwem(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sd_info2_mask::Bwem,
        sd_info2_mask::Bwem,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sd_info2_mask::Bwem,
            sd_info2_mask::Bwem,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRE Interrupt Request Mask"]
    #[inline(always)]
    pub fn brem(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_info2_mask::Brem,
        sd_info2_mask::Brem,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_info2_mask::Brem,
            sd_info2_mask::Brem,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Response Timeout Interrupt Request Mask"]
    #[inline(always)]
    pub fn rsptom(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sd_info2_mask::Rsptom,
        sd_info2_mask::Rsptom,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sd_info2_mask::Rsptom,
            sd_info2_mask::Rsptom,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD_BUF Register Illegal Read Interrupt Request Mask"]
    #[inline(always)]
    pub fn ilrm(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sd_info2_mask::Ilrm,
        sd_info2_mask::Ilrm,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sd_info2_mask::Ilrm,
            sd_info2_mask::Ilrm,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD_BUF Register Illegal Write Interrupt Request Mask"]
    #[inline(always)]
    pub fn ilwm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sd_info2_mask::Ilwm,
        sd_info2_mask::Ilwm,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sd_info2_mask::Ilwm,
            sd_info2_mask::Ilwm,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Timeout Interrupt Request Mask"]
    #[inline(always)]
    pub fn dtom(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sd_info2_mask::Dtom,
        sd_info2_mask::Dtom,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sd_info2_mask::Dtom,
            sd_info2_mask::Dtom,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "End Bit Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn endem(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sd_info2_mask::Endem,
        sd_info2_mask::Endem,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sd_info2_mask::Endem,
            sd_info2_mask::Endem,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CRC Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn crcem(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sd_info2_mask::Crcem,
        sd_info2_mask::Crcem,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sd_info2_mask::Crcem,
            sd_info2_mask::Crcem,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Error Interrupt Request Mask"]
    #[inline(always)]
    pub fn cmdem(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sd_info2_mask::Cmdem,
        sd_info2_mask::Cmdem,
        SdInfo2Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sd_info2_mask::Cmdem,
            sd_info2_mask::Cmdem,
            SdInfo2Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdInfo2Mask {
    #[inline(always)]
    fn default() -> SdInfo2Mask {
        <crate::RegValueT<SdInfo2Mask_SPEC> as RegisterValue<_>>::new(35711)
    }
}
pub mod sd_info2_mask {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilam_SPEC;
    pub type Ilam = crate::EnumBitfieldStruct<u8, Ilam_SPEC>;
    impl Ilam {
        #[doc = "Illegal access error interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Illegal access error interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bwem_SPEC;
    pub type Bwem = crate::EnumBitfieldStruct<u8, Bwem_SPEC>;
    impl Bwem {
        #[doc = "Write enable interrupt request for the SD_BUF register not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enable interrupt request for the SD_BUF register masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brem_SPEC;
    pub type Brem = crate::EnumBitfieldStruct<u8, Brem_SPEC>;
    impl Brem {
        #[doc = "Read enable interrupt request for the SD buffer not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read enable interrupt request for the SD buffer masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsptom_SPEC;
    pub type Rsptom = crate::EnumBitfieldStruct<u8, Rsptom_SPEC>;
    impl Rsptom {
        #[doc = "Response timeout interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Response timeout interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilrm_SPEC;
    pub type Ilrm = crate::EnumBitfieldStruct<u8, Ilrm_SPEC>;
    impl Ilrm {
        #[doc = "Illegal read detection interrupt request for the SD_BUF register not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Illegal read detection interrupt request for the SD_BUF register masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilwm_SPEC;
    pub type Ilwm = crate::EnumBitfieldStruct<u8, Ilwm_SPEC>;
    impl Ilwm {
        #[doc = "Illegal write detection interrupt request for the SD_BUF register not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Illegal write detection interrupt request for the SD_BUF register masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtom_SPEC;
    pub type Dtom = crate::EnumBitfieldStruct<u8, Dtom_SPEC>;
    impl Dtom {
        #[doc = "Data timeout interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data timeout interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Endem_SPEC;
    pub type Endem = crate::EnumBitfieldStruct<u8, Endem_SPEC>;
    impl Endem {
        #[doc = "End bit detection error interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "End bit detection error interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcem_SPEC;
    pub type Crcem = crate::EnumBitfieldStruct<u8, Crcem_SPEC>;
    impl Crcem {
        #[doc = "CRC error interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "CRC error interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdem_SPEC;
    pub type Cmdem = crate::EnumBitfieldStruct<u8, Cmdem_SPEC>;
    impl Cmdem {
        #[doc = "Command error interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "Command error interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdClkCtrl_SPEC;
impl crate::sealed::RegSpec for SdClkCtrl_SPEC {
    type DataType = u32;
}

#[doc = "SD Clock Control Register"]
pub type SdClkCtrl = crate::RegValueT<SdClkCtrl_SPEC>;

impl SdClkCtrl {
    #[doc = "SD/MMC Clock Output Automatic Control Enable"]
    #[inline(always)]
    pub fn clkctrlen(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sd_clk_ctrl::Clkctrlen,
        sd_clk_ctrl::Clkctrlen,
        SdClkCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sd_clk_ctrl::Clkctrlen,
            sd_clk_ctrl::Clkctrlen,
            SdClkCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD/MMC Clock Output Control Enable"]
    #[inline(always)]
    pub fn clken(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_clk_ctrl::Clken,
        sd_clk_ctrl::Clken,
        SdClkCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_clk_ctrl::Clken,
            sd_clk_ctrl::Clken,
            SdClkCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDHI Clock Frequency Select"]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        sd_clk_ctrl::Clksel,
        sd_clk_ctrl::Clksel,
        SdClkCtrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            sd_clk_ctrl::Clksel,
            sd_clk_ctrl::Clksel,
            SdClkCtrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdClkCtrl {
    #[inline(always)]
    fn default() -> SdClkCtrl {
        <crate::RegValueT<SdClkCtrl_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod sd_clk_ctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clkctrlen_SPEC;
    pub type Clkctrlen = crate::EnumBitfieldStruct<u8, Clkctrlen_SPEC>;
    impl Clkctrlen {
        #[doc = "Automatic control for SD/MMC Clock output is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatic control for SD/MMC Clock output is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clken_SPEC;
    pub type Clken = crate::EnumBitfieldStruct<u8, Clken_SPEC>;
    impl Clken {
        #[doc = "SD/MMC Clock output is disabled. The SDCLK signal is fixed 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "SD/MMC Clock output is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "PCLKA divided by 2"]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "PCLKA divided by 4"]
        pub const _0_X_01: Self = Self::new(1);

        #[doc = "PCLKA divided by 8"]
        pub const _0_X_02: Self = Self::new(2);

        #[doc = "PCLKA divided by 16"]
        pub const _0_X_04: Self = Self::new(4);

        #[doc = "PCLKA divided by 32"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "PCLKA divided by 64"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "PCLKA divided by 128"]
        pub const _0_X_20: Self = Self::new(32);

        #[doc = "PCLKA divided by 256"]
        pub const _0_X_40: Self = Self::new(64);

        #[doc = "PCLKA divided by 512"]
        pub const _0_X_80: Self = Self::new(128);

        #[doc = "Settings prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdSize_SPEC;
impl crate::sealed::RegSpec for SdSize_SPEC {
    type DataType = u32;
}

#[doc = "Transfer Data Length Register"]
pub type SdSize = crate::RegValueT<SdSize_SPEC>;

impl SdSize {
    #[doc = "Transfer Data SizeThese bits specify a size between 1 and 512 bytes for the transfer of single blocks.In cases of multiple block transfer with automatic issuing of CMD12 (CMD18 and CMD25), the only specifiable transfer data size is 512 bytes. Furthermore, in cases of multiple block transfer without automatic issuing of CMD12, as well as 512 bytes, 32, 64, 128, and 256 bytes are specifiable. However, in the reading of 32, 64, 128, and 256 bytes for the transfer of multiple blocks, this is restricted to multiple block transfer by CMD53.Additionally, if a command accompanies data transfer, do not set these bits to 0."]
    #[inline(always)]
    pub fn len(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, SdSize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,SdSize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdSize {
    #[inline(always)]
    fn default() -> SdSize {
        <crate::RegValueT<SdSize_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdOption_SPEC;
impl crate::sealed::RegSpec for SdOption_SPEC {
    type DataType = u32;
}

#[doc = "SD Card Access Control Option Register"]
pub type SdOption = crate::RegValueT<SdOption_SPEC>;

impl SdOption {
    #[doc = "Bus WidthNOTE: The initial value is applied at a reset and when the SOFT_RST.SDRST flag is 0."]
    #[inline(always)]
    pub fn width(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sd_option::Width,
        sd_option::Width,
        SdOption_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sd_option::Width,
            sd_option::Width,
            SdOption_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Widthsee b15, WIDTH bit"]
    #[inline(always)]
    pub fn width8(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SdOption_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,SdOption_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timeout MASKWhen timeout occurs in case of inactivating timeout, software reset should be executed to terminate command sequence."]
    #[inline(always)]
    pub fn toutmask(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_option::Toutmask,
        sd_option::Toutmask,
        SdOption_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_option::Toutmask,
            sd_option::Toutmask,
            SdOption_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout Counter"]
    #[inline(always)]
    pub fn top(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        sd_option::Top,
        sd_option::Top,
        SdOption_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            sd_option::Top,
            sd_option::Top,
            SdOption_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Card Detect Time Counter"]
    #[inline(always)]
    pub fn ctop(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        sd_option::Ctop,
        sd_option::Ctop,
        SdOption_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            sd_option::Ctop,
            sd_option::Ctop,
            SdOption_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdOption {
    #[inline(always)]
    fn default() -> SdOption {
        <crate::RegValueT<SdOption_SPEC> as RegisterValue<_>>::new(16622)
    }
}
pub mod sd_option {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Width_SPEC;
    pub type Width = crate::EnumBitfieldStruct<u8, Width_SPEC>;
    impl Width {
        #[doc = "4-bit width (WIDTH8=0) / 8-bit width (WIDTH8=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "1-bit width (WIDTH8=0 or 1 )"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toutmask_SPEC;
    pub type Toutmask = crate::EnumBitfieldStruct<u8, Toutmask_SPEC>;
    impl Toutmask {
        #[doc = "Activate Timeout"]
        pub const _0: Self = Self::new(0);

        #[doc = "Inactivate Timeout(RSPTO bit and DTO bit of SD_INFO2 and SD_ERR_STS2 won\'t be set)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Top_SPEC;
    pub type Top = crate::EnumBitfieldStruct<u8, Top_SPEC>;
    impl Top {
        #[doc = "Setting prohibited"]
        pub const _1111: Self = Self::new(15);

        #[doc = "SDHI clock x 2^(TOP+13)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctop_SPEC;
    pub type Ctop = crate::EnumBitfieldStruct<u8, Ctop_SPEC>;
    impl Ctop {
        #[doc = "Setting prohibited"]
        pub const _1111: Self = Self::new(15);

        #[doc = "IMCLK x 2^(CTOP+10)"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdErrSts1_SPEC;
impl crate::sealed::RegSpec for SdErrSts1_SPEC {
    type DataType = u32;
}

#[doc = "SD Error Status Register 1"]
pub type SdErrSts1 = crate::RegValueT<SdErrSts1_SPEC>;

impl SdErrSts1 {
    #[doc = "CRC Status TokenStore the CRC status token value (normal value is 010b)"]
    #[inline(always)]
    pub fn crctk(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, SdErrSts1_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,SdErrSts1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "CRC Status Token Error"]
    #[inline(always)]
    pub fn crctke(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        sd_err_sts1::Crctke,
        sd_err_sts1::Crctke,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            sd_err_sts1::Crctke,
            sd_err_sts1::Crctke,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Read Data CRC Error"]
    #[inline(always)]
    pub fn rdcrce(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sd_err_sts1::Rdcrce,
        sd_err_sts1::Rdcrce,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sd_err_sts1::Rdcrce,
            sd_err_sts1::Rdcrce,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response CRC Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPCRCE0."]
    #[inline(always)]
    pub fn rspcrce1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sd_err_sts1::Rspcrce1,
        sd_err_sts1::Rspcrce1,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sd_err_sts1::Rspcrce1,
            sd_err_sts1::Rspcrce1,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response CRC Error 0NOTE: other than a response to a command issued within a command sequence"]
    #[inline(always)]
    pub fn rspcrce0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sd_err_sts1::Rspcrce0,
        sd_err_sts1::Rspcrce0,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sd_err_sts1::Rspcrce0,
            sd_err_sts1::Rspcrce0,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CRC Status Token Length Error"]
    #[inline(always)]
    pub fn crclene(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sd_err_sts1::Crclene,
        sd_err_sts1::Crclene,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sd_err_sts1::Crclene,
            sd_err_sts1::Crclene,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Read Data Length Error"]
    #[inline(always)]
    pub fn rdlene(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sd_err_sts1::Rdlene,
        sd_err_sts1::Rdlene,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sd_err_sts1::Rdlene,
            sd_err_sts1::Rdlene,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Length Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPLENE0."]
    #[inline(always)]
    pub fn rsplene1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sd_err_sts1::Rsplene1,
        sd_err_sts1::Rsplene1,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sd_err_sts1::Rsplene1,
            sd_err_sts1::Rsplene1,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Length Error 0NOTE: other than a response to a command issued within a command sequence"]
    #[inline(always)]
    pub fn rsplene0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sd_err_sts1::Rsplene0,
        sd_err_sts1::Rsplene0,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sd_err_sts1::Rsplene0,
            sd_err_sts1::Rsplene0,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Command Error 1NOTE: In cases where CMD12 is issued by setting a command index in SD_CMD, this is Indicated in CMDE0."]
    #[inline(always)]
    pub fn cmde1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sd_err_sts1::Cmde1,
        sd_err_sts1::Cmde1,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sd_err_sts1::Cmde1,
            sd_err_sts1::Cmde1,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Command Error 0NOTE: other than a response to a command issued within a command sequence"]
    #[inline(always)]
    pub fn cmde0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sd_err_sts1::Cmde0,
        sd_err_sts1::Cmde0,
        SdErrSts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sd_err_sts1::Cmde0,
            sd_err_sts1::Cmde0,
            SdErrSts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdErrSts1 {
    #[inline(always)]
    fn default() -> SdErrSts1 {
        <crate::RegValueT<SdErrSts1_SPEC> as RegisterValue<_>>::new(8192)
    }
}
pub mod sd_err_sts1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crctke_SPEC;
    pub type Crctke = crate::EnumBitfieldStruct<u8, Crctke_SPEC>;
    impl Crctke {
        #[doc = "An error has not occured in the CRC status."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occured in the CRC status."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdcrce_SPEC;
    pub type Rdcrce = crate::EnumBitfieldStruct<u8, Rdcrce_SPEC>;
    impl Rdcrce {
        #[doc = "CRC error has detected in read data"]
        pub const _0: Self = Self::new(0);

        #[doc = "CRC error has not detected in read data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspcrce1_SPEC;
    pub type Rspcrce1 = crate::EnumBitfieldStruct<u8, Rspcrce1_SPEC>;
    impl Rspcrce1 {
        #[doc = "CRC error has not occured."]
        pub const _0: Self = Self::new(0);

        #[doc = "CRC error has occured in the response to a command issued within a command sequence."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspcrce0_SPEC;
    pub type Rspcrce0 = crate::EnumBitfieldStruct<u8, Rspcrce0_SPEC>;
    impl Rspcrce0 {
        #[doc = "A CRC error has not occur in a response"]
        pub const _0: Self = Self::new(0);

        #[doc = "A CRC error has occured in a response"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crclene_SPEC;
    pub type Crclene = crate::EnumBitfieldStruct<u8, Crclene_SPEC>;
    impl Crclene {
        #[doc = "An error has not occured in the CRC status length."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occured in the CRC status length (and the end bit has not been detected)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdlene_SPEC;
    pub type Rdlene = crate::EnumBitfieldStruct<u8, Rdlene_SPEC>;
    impl Rdlene {
        #[doc = "An error has occurred not in the read data length."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occured in the read data length (and the end bit has not been detected among the valid bits)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsplene1_SPEC;
    pub type Rsplene1 = crate::EnumBitfieldStruct<u8, Rsplene1_SPEC>;
    impl Rsplene1 {
        #[doc = "An error has not occurred in the response length to a command issued within a command sequence."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occured in the response length to a command issued within a command sequence."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsplene0_SPEC;
    pub type Rsplene0 = crate::EnumBitfieldStruct<u8, Rsplene0_SPEC>;
    impl Rsplene0 {
        #[doc = "An error has not occured in the response length"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occured in the response length"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmde1_SPEC;
    pub type Cmde1 = crate::EnumBitfieldStruct<u8, Cmde1_SPEC>;
    impl Cmde1 {
        #[doc = "An error has not occurs in the command index of the response to a command issued within a command sequence."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occured in the command index of the response to a command issued within a command sequence."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmde0_SPEC;
    pub type Cmde0 = crate::EnumBitfieldStruct<u8, Cmde0_SPEC>;
    impl Cmde0 {
        #[doc = "An error has not occured in the command index of a response."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occured in the command index of a response."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdErrSts2_SPEC;
impl crate::sealed::RegSpec for SdErrSts2_SPEC {
    type DataType = u32;
}

#[doc = "SD Error Status Register 2"]
pub type SdErrSts2 = crate::RegValueT<SdErrSts2_SPEC>;

impl SdErrSts2 {
    #[doc = "CRC Status Token Busy Timeout"]
    #[inline(always)]
    pub fn crcbsyto(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sd_err_sts2::Crcbsyto,
        sd_err_sts2::Crcbsyto,
        SdErrSts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sd_err_sts2::Crcbsyto,
            sd_err_sts2::Crcbsyto,
            SdErrSts2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CRC Status Token Timeout"]
    #[inline(always)]
    pub fn crcto(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sd_err_sts2::Crcto,
        sd_err_sts2::Crcto,
        SdErrSts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sd_err_sts2::Crcto,
            sd_err_sts2::Crcto,
            SdErrSts2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Read Data Timeout"]
    #[inline(always)]
    pub fn rdto(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sd_err_sts2::Rdto,
        sd_err_sts2::Rdto,
        SdErrSts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sd_err_sts2::Rdto,
            sd_err_sts2::Rdto,
            SdErrSts2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Busy Timeout 1"]
    #[inline(always)]
    pub fn bsyto1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sd_err_sts2::Bsyto1,
        sd_err_sts2::Bsyto1,
        SdErrSts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sd_err_sts2::Bsyto1,
            sd_err_sts2::Bsyto1,
            SdErrSts2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Busy Timeout 0"]
    #[inline(always)]
    pub fn bsyto0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sd_err_sts2::Bsyto0,
        sd_err_sts2::Bsyto0,
        SdErrSts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sd_err_sts2::Bsyto0,
            sd_err_sts2::Bsyto0,
            SdErrSts2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Timeout 1"]
    #[inline(always)]
    pub fn rspto1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sd_err_sts2::Rspto1,
        sd_err_sts2::Rspto1,
        SdErrSts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sd_err_sts2::Rspto1,
            sd_err_sts2::Rspto1,
            SdErrSts2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Response Timeout 0"]
    #[inline(always)]
    pub fn rspto0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sd_err_sts2::Rspto0,
        sd_err_sts2::Rspto0,
        SdErrSts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sd_err_sts2::Rspto0,
            sd_err_sts2::Rspto0,
            SdErrSts2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdErrSts2 {
    #[inline(always)]
    fn default() -> SdErrSts2 {
        <crate::RegValueT<SdErrSts2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sd_err_sts2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcbsyto_SPEC;
    pub type Crcbsyto = crate::EnumBitfieldStruct<u8, Crcbsyto_SPEC>;
    impl Crcbsyto {
        #[doc = "Not timeout"]
        pub const _0: Self = Self::new(0);

        #[doc = "The busy state continues for longer than N-cycle after the CRC status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcto_SPEC;
    pub type Crcto = crate::EnumBitfieldStruct<u8, Crcto_SPEC>;
    impl Crcto {
        #[doc = "Not timeout"]
        pub const _0: Self = Self::new(0);

        #[doc = "The CRC status is not received though a longer time than N-cycle has elapsed after data writing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdto_SPEC;
    pub type Rdto = crate::EnumBitfieldStruct<u8, Rdto_SPEC>;
    impl Rdto {
        #[doc = "Not timeout"]
        pub const _0: Self = Self::new(0);

        #[doc = "The read data is not received though a longer time than N-cycle has elapsed after read command. / The read data for the next block are not received though a longer time than N-cycle has elapsed after the reception of read data. / The read data for the next block are not received though a longer time than N-cycle has elapsed after release of the read wait state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsyto1_SPEC;
    pub type Bsyto1 = crate::EnumBitfieldStruct<u8, Bsyto1_SPEC>;
    impl Bsyto1 {
        #[doc = "Not timeout."]
        pub const _0: Self = Self::new(0);

        #[doc = "The busy state for longer than N-cycle continues after CMD12 has been issued within a command sequence. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in BSYTO0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsyto0_SPEC;
    pub type Bsyto0 = crate::EnumBitfieldStruct<u8, Bsyto0_SPEC>;
    impl Bsyto0 {
        #[doc = "Not timeout."]
        pub const _0: Self = Self::new(0);

        #[doc = "The busy state for longer than N-cycle continues after R1b response."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspto1_SPEC;
    pub type Rspto1 = crate::EnumBitfieldStruct<u8, Rspto1_SPEC>;
    impl Rspto1 {
        #[doc = "Not timeout."]
        pub const _0: Self = Self::new(0);

        #[doc = "The response to a command issued within a command sequence*2 is not received though a longer time than 640 cycles of SD/MMC clock has elapsed. In cases where CMD12 is issued by setting a command index in SD_CMD, this is indicated in RSPTO0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspto0_SPEC;
    pub type Rspto0 = crate::EnumBitfieldStruct<u8, Rspto0_SPEC>;
    impl Rspto0 {
        #[doc = "Not timeout."]
        pub const _0: Self = Self::new(0);

        #[doc = "The response (other than a response to a command issued within a command sequence) is not received though a longer time than 640 cycles of SD/MMC clock has elapsed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdBuf0_SPEC;
impl crate::sealed::RegSpec for SdBuf0_SPEC {
    type DataType = u32;
}

#[doc = "SD Buffer Register"]
pub type SdBuf0 = crate::RegValueT<SdBuf0_SPEC>;

impl SdBuf0 {
    #[doc = "SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data."]
    #[inline(always)]
    pub fn sd_buf(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SdBuf0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SdBuf0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdBuf0 {
    #[inline(always)]
    fn default() -> SdBuf0 {
        <crate::RegValueT<SdBuf0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdioMode_SPEC;
impl crate::sealed::RegSpec for SdioMode_SPEC {
    type DataType = u32;
}

#[doc = "SDIO Mode Control Register"]
pub type SdioMode = crate::RegValueT<SdioMode_SPEC>;

impl SdioMode {
    #[doc = "SDIO None AbortNOTE: See manual"]
    #[inline(always)]
    pub fn c52pub(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SdioMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, SdioMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SDIO AbortNOTE: See manual"]
    #[inline(always)]
    pub fn ioabt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SdioMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, SdioMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read Wait Request"]
    #[inline(always)]
    pub fn rwreq(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sdio_mode::Rwreq,
        sdio_mode::Rwreq,
        SdioMode_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sdio_mode::Rwreq,
            sdio_mode::Rwreq,
            SdioMode_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDIO Mode"]
    #[inline(always)]
    pub fn inten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdio_mode::Inten,
        sdio_mode::Inten,
        SdioMode_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdio_mode::Inten,
            sdio_mode::Inten,
            SdioMode_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdioMode {
    #[inline(always)]
    fn default() -> SdioMode {
        <crate::RegValueT<SdioMode_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdio_mode {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwreq_SPEC;
    pub type Rwreq = crate::EnumBitfieldStruct<u8, Rwreq_SPEC>;
    impl Rwreq {
        #[doc = "Allow SD/MMC to exit read wait state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request for SD/MMC to enter read wait state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inten_SPEC;
    pub type Inten = crate::EnumBitfieldStruct<u8, Inten_SPEC>;
    impl Inten {
        #[doc = "Enables the SD host interface to receive SDIO interrupt from the SDIO card"]
        pub const _1: Self = Self::new(1);

        #[doc = "Disables the SD host interface to receive SDIO interrupt from the SDIO card"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdioInfo1_SPEC;
impl crate::sealed::RegSpec for SdioInfo1_SPEC {
    type DataType = u32;
}

#[doc = "SDIO Interrupt Flag Register 1"]
pub type SdioInfo1 = crate::RegValueT<SdioInfo1_SPEC>;

impl SdioInfo1 {
    #[doc = "EXWT Status FlagNOTE: See manual"]
    #[inline(always)]
    pub fn exwt(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SdioInfo1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,SdioInfo1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "EXPUB52 Status FlagNOTE: See manual"]
    #[inline(always)]
    pub fn expub52(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SdioInfo1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,SdioInfo1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SDIO Interrupt Status"]
    #[inline(always)]
    pub fn ioirq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdio_info1::Ioirq,
        sdio_info1::Ioirq,
        SdioInfo1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdio_info1::Ioirq,
            sdio_info1::Ioirq,
            SdioInfo1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdioInfo1 {
    #[inline(always)]
    fn default() -> SdioInfo1 {
        <crate::RegValueT<SdioInfo1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdio_info1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioirq_SPEC;
    pub type Ioirq = crate::EnumBitfieldStruct<u8, Ioirq_SPEC>;
    impl Ioirq {
        #[doc = "SDIO interrupt not accepted"]
        pub const _0: Self = Self::new(0);

        #[doc = "SDIO interrupt accepted"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdioInfo1Mask_SPEC;
impl crate::sealed::RegSpec for SdioInfo1Mask_SPEC {
    type DataType = u32;
}

#[doc = "SDIO_INFO1 Interrupt Mask Register"]
pub type SdioInfo1Mask = crate::RegValueT<SdioInfo1Mask_SPEC>;

impl SdioInfo1Mask {
    #[doc = "EXWT Interrupt Request Mask Control"]
    #[inline(always)]
    pub fn exwtm(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        sdio_info1_mask::Exwtm,
        sdio_info1_mask::Exwtm,
        SdioInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            sdio_info1_mask::Exwtm,
            sdio_info1_mask::Exwtm,
            SdioInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EXPUB52 Interrupt Request Mask Control"]
    #[inline(always)]
    pub fn expub52m(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sdio_info1_mask::Expub52M,
        sdio_info1_mask::Expub52M,
        SdioInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sdio_info1_mask::Expub52M,
            sdio_info1_mask::Expub52M,
            SdioInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IOIRQ Interrupt Mask Control"]
    #[inline(always)]
    pub fn ioirqm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdio_info1_mask::Ioirqm,
        sdio_info1_mask::Ioirqm,
        SdioInfo1Mask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdio_info1_mask::Ioirqm,
            sdio_info1_mask::Ioirqm,
            SdioInfo1Mask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdioInfo1Mask {
    #[inline(always)]
    fn default() -> SdioInfo1Mask {
        <crate::RegValueT<SdioInfo1Mask_SPEC> as RegisterValue<_>>::new(49159)
    }
}
pub mod sdio_info1_mask {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exwtm_SPEC;
    pub type Exwtm = crate::EnumBitfieldStruct<u8, Exwtm_SPEC>;
    impl Exwtm {
        #[doc = "EXWT interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXWT interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Expub52M_SPEC;
    pub type Expub52M = crate::EnumBitfieldStruct<u8, Expub52M_SPEC>;
    impl Expub52M {
        #[doc = "EXPUB52 interrupt request not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "EXPUB52 interrupt request masked"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ioirqm_SPEC;
    pub type Ioirqm = crate::EnumBitfieldStruct<u8, Ioirqm_SPEC>;
    impl Ioirqm {
        #[doc = "IOIRQ interrupt not masked"]
        pub const _0: Self = Self::new(0);

        #[doc = "IOIRQ interrupt masked"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdDmaen_SPEC;
impl crate::sealed::RegSpec for SdDmaen_SPEC {
    type DataType = u32;
}

#[doc = "DMA Mode Enable Register"]
pub type SdDmaen = crate::RegValueT<SdDmaen_SPEC>;

impl SdDmaen {
    #[doc = "SD_BUF Read/Write DMA Transfer"]
    #[inline(always)]
    pub fn dmaen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sd_dmaen::Dmaen,
        sd_dmaen::Dmaen,
        SdDmaen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sd_dmaen::Dmaen,
            sd_dmaen::Dmaen,
            SdDmaen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdDmaen {
    #[inline(always)]
    fn default() -> SdDmaen {
        <crate::RegValueT<SdDmaen_SPEC> as RegisterValue<_>>::new(4112)
    }
}
pub mod sd_dmaen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmaen_SPEC;
    pub type Dmaen = crate::EnumBitfieldStruct<u8, Dmaen_SPEC>;
    impl Dmaen {
        #[doc = "The SD_BUF read/write DMA transfer is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SD_BUF read/write DMA transfer is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SoftRst_SPEC;
impl crate::sealed::RegSpec for SoftRst_SPEC {
    type DataType = u32;
}

#[doc = "Software Reset Register"]
pub type SoftRst = crate::RegValueT<SoftRst_SPEC>;

impl SoftRst {
    #[doc = "Software Reset of SD I/F Unit"]
    #[inline(always)]
    pub fn sdrst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        soft_rst::Sdrst,
        soft_rst::Sdrst,
        SoftRst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            soft_rst::Sdrst,
            soft_rst::Sdrst,
            SoftRst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SoftRst {
    #[inline(always)]
    fn default() -> SoftRst {
        <crate::RegValueT<SoftRst_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod soft_rst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdrst_SPEC;
    pub type Sdrst = crate::EnumBitfieldStruct<u8, Sdrst_SPEC>;
    impl Sdrst {
        #[doc = "Reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset released"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdifMode_SPEC;
impl crate::sealed::RegSpec for SdifMode_SPEC {
    type DataType = u32;
}

#[doc = "SD Interface Mode Setting Register"]
pub type SdifMode = crate::RegValueT<SdifMode_SPEC>;

impl SdifMode {
    #[doc = "CRC Check Mask (for MMC test commands)"]
    #[inline(always)]
    pub fn nochkcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sdif_mode::Nochkcr,
        sdif_mode::Nochkcr,
        SdifMode_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sdif_mode::Nochkcr,
            sdif_mode::Nochkcr,
            SdifMode_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdifMode {
    #[inline(always)]
    fn default() -> SdifMode {
        <crate::RegValueT<SdifMode_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdif_mode {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nochkcr_SPEC;
    pub type Nochkcr = crate::EnumBitfieldStruct<u8, Nochkcr_SPEC>;
    impl Nochkcr {
        #[doc = "CRC check is valid"]
        pub const _0: Self = Self::new(0);

        #[doc = "CRC check is invalid(CRC16 value is ignored when read and CRC Status value is ignored when write)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtSwap_SPEC;
impl crate::sealed::RegSpec for ExtSwap_SPEC {
    type DataType = u32;
}

#[doc = "Swap Control Register"]
pub type ExtSwap = crate::RegValueT<ExtSwap_SPEC>;

impl ExtSwap {
    #[doc = "SD_BUF0 Swap Read"]
    #[inline(always)]
    pub fn brswp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ext_swap::Brswp,
        ext_swap::Brswp,
        ExtSwap_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ext_swap::Brswp,
            ext_swap::Brswp,
            ExtSwap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD_BUF0 Swap Write"]
    #[inline(always)]
    pub fn bwswp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ext_swap::Bwswp,
        ext_swap::Bwswp,
        ExtSwap_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ext_swap::Bwswp,
            ext_swap::Bwswp,
            ExtSwap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ExtSwap {
    #[inline(always)]
    fn default() -> ExtSwap {
        <crate::RegValueT<ExtSwap_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ext_swap {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brswp_SPEC;
    pub type Brswp = crate::EnumBitfieldStruct<u8, Brswp_SPEC>;
    impl Brswp {
        #[doc = "The current data are read without swapping."]
        pub const _0: Self = Self::new(0);

        #[doc = "Swapping of the positions of the higher- and lower-order bytes of data for reading proceeds."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bwswp_SPEC;
    pub type Bwswp = crate::EnumBitfieldStruct<u8, Bwswp_SPEC>;
    impl Bwswp {
        #[doc = "The current data are written without swapping."]
        pub const _0: Self = Self::new(0);

        #[doc = "Swapping of the positions of the higher- and lower-order bytes of data for writing proceeds."]
        pub const _1: Self = Self::new(1);
    }
}
