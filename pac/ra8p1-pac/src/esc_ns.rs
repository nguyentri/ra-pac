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
#[doc = r"EtherCAT Slave Controller"]
unsafe impl ::core::marker::Send for super::EscNs {}
unsafe impl ::core::marker::Sync for super::EscNs {}
impl super::EscNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Type Register"]
    #[inline(always)]
    pub const fn r#type(&self) -> &'static crate::common::Reg<self::Type_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Type_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Revision Register"]
    #[inline(always)]
    pub const fn revision(
        &self,
    ) -> &'static crate::common::Reg<self::Revision_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Revision_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "Build Register"]
    #[inline(always)]
    pub const fn build(&self) -> &'static crate::common::Reg<self::Build_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Build_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "FMMU Supported Register"]
    #[inline(always)]
    pub const fn fmmu_num(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuNum_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuNum_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "SyncManager Supported Register"]
    #[inline(always)]
    pub const fn sync_manager(
        &self,
    ) -> &'static crate::common::Reg<self::SyncManager_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SyncManager_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "RAM Size Register"]
    #[inline(always)]
    pub const fn ram_size(
        &self,
    ) -> &'static crate::common::Reg<self::RamSize_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::RamSize_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Port Descriptor Register"]
    #[inline(always)]
    pub const fn port_desc(
        &self,
    ) -> &'static crate::common::Reg<self::PortDesc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::PortDesc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "ESC Features Supported Register"]
    #[inline(always)]
    pub const fn feature(
        &self,
    ) -> &'static crate::common::Reg<self::Feature_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Feature_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Configured Station Address Register"]
    #[inline(always)]
    pub const fn station_adr(
        &self,
    ) -> &'static crate::common::Reg<self::StationAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::StationAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Configured Station Alias Register"]
    #[inline(always)]
    pub const fn station_alias(
        &self,
    ) -> &'static crate::common::Reg<self::StationAlias_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::StationAlias_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Write Register Enable Register"]
    #[inline(always)]
    pub const fn wr_reg_enable(
        &self,
    ) -> &'static crate::common::Reg<self::WrRegEnable_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WrRegEnable_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Write Register Protection Register"]
    #[inline(always)]
    pub const fn wr_reg_protect(
        &self,
    ) -> &'static crate::common::Reg<self::WrRegProtect_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WrRegProtect_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(33usize),
            )
        }
    }

    #[doc = "ESC Write Enable Register"]
    #[inline(always)]
    pub const fn esc_wr_enable(
        &self,
    ) -> &'static crate::common::Reg<self::EscWrEnable_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EscWrEnable_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "ESC Write Protection Register"]
    #[inline(always)]
    pub const fn esc_wr_protect(
        &self,
    ) -> &'static crate::common::Reg<self::EscWrProtect_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EscWrProtect_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
            )
        }
    }

    #[doc = "ESC Reset ECAT Register for read"]
    #[inline(always)]
    pub const fn esc_reset_ecat_r(
        &self,
    ) -> &'static crate::common::Reg<self::EscResetEcatR_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EscResetEcatR_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "ESC Reset ECAT Register for write"]
    #[inline(always)]
    pub const fn esc_reset_ecat_w(
        &self,
    ) -> &'static crate::common::Reg<self::EscResetEcatW_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EscResetEcatW_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "ESC Reset PDI Register for read"]
    #[inline(always)]
    pub const fn esc_reset_pdi_r(
        &self,
    ) -> &'static crate::common::Reg<self::EscResetPdiR_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EscResetPdiR_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[doc = "ESC Reset PDI Register for write"]
    #[inline(always)]
    pub const fn esc_reset_pdi_w(
        &self,
    ) -> &'static crate::common::Reg<self::EscResetPdiW_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EscResetPdiW_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[doc = "ESC DL Control Register"]
    #[inline(always)]
    pub const fn esc_dl_control(
        &self,
    ) -> &'static crate::common::Reg<self::EscDlControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EscDlControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Physical Read/Write Offset Register"]
    #[inline(always)]
    pub const fn physical_rw_offset(
        &self,
    ) -> &'static crate::common::Reg<self::PhysicalRwOffset_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::PhysicalRwOffset_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "ESC DL Status Register"]
    #[inline(always)]
    pub const fn esc_dl_status(
        &self,
    ) -> &'static crate::common::Reg<self::EscDlStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EscDlStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "AL Control Register"]
    #[inline(always)]
    pub const fn al_control(
        &self,
    ) -> &'static crate::common::Reg<self::AlControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::AlControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "AL Status Register"]
    #[inline(always)]
    pub const fn al_status(
        &self,
    ) -> &'static crate::common::Reg<self::AlStatus_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AlStatus_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "AL Status Code Register"]
    #[inline(always)]
    pub const fn al_status_code(
        &self,
    ) -> &'static crate::common::Reg<self::AlStatusCode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AlStatusCode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "RUN LED Override Register"]
    #[inline(always)]
    pub const fn run_led_override(
        &self,
    ) -> &'static crate::common::Reg<self::RunLedOverride_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RunLedOverride_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[doc = "ERR LED Override Register"]
    #[inline(always)]
    pub const fn err_led_override(
        &self,
    ) -> &'static crate::common::Reg<self::ErrLedOverride_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ErrLedOverride_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(313usize),
            )
        }
    }

    #[doc = "PDI Control Register"]
    #[inline(always)]
    pub const fn pdi_control(
        &self,
    ) -> &'static crate::common::Reg<self::PdiControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::PdiControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "ESC Configuration Register"]
    #[inline(always)]
    pub const fn esc_config(
        &self,
    ) -> &'static crate::common::Reg<self::EscConfig_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EscConfig_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(321usize),
            )
        }
    }

    #[doc = "PDI Configuration Register"]
    #[inline(always)]
    pub const fn pdi_config(
        &self,
    ) -> &'static crate::common::Reg<self::PdiConfig_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::PdiConfig_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(336usize),
            )
        }
    }

    #[doc = "SYNC/LATCH PDI Configuration Register"]
    #[inline(always)]
    pub const fn sync_latch_config(
        &self,
    ) -> &'static crate::common::Reg<self::SyncLatchConfig_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SyncLatchConfig_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(337usize),
            )
        }
    }

    #[doc = "Extended PDI Configuration Register"]
    #[inline(always)]
    pub const fn ext_pdi_config(
        &self,
    ) -> &'static crate::common::Reg<self::ExtPdiConfig_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::ExtPdiConfig_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(338usize),
            )
        }
    }

    #[doc = "ECAT Event Mask Register"]
    #[inline(always)]
    pub const fn ecat_event_mask(
        &self,
    ) -> &'static crate::common::Reg<self::EcatEventMask_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EcatEventMask_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "AL Event Mask Register"]
    #[inline(always)]
    pub const fn al_event_mask(
        &self,
    ) -> &'static crate::common::Reg<self::AlEventMask_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AlEventMask_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "ECAT Event Request Register"]
    #[inline(always)]
    pub const fn ecat_event_req(
        &self,
    ) -> &'static crate::common::Reg<self::EcatEventReq_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EcatEventReq_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "AL Event Request Register"]
    #[inline(always)]
    pub const fn al_event_req(
        &self,
    ) -> &'static crate::common::Reg<self::AlEventReq_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::AlEventReq_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[doc = "RX Error Counter %s Register (n = 0 to 1)"]
    #[inline(always)]
    pub const fn rx_err_count(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::RxErrCount_SPEC, crate::common::R>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
    #[inline(always)]
    pub const fn rx_err_count0(
        &self,
    ) -> &'static crate::common::Reg<self::RxErrCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::RxErrCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn rx_err_count1(
        &self,
    ) -> &'static crate::common::Reg<self::RxErrCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::RxErrCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x302usize),
            )
        }
    }

    #[doc = "Forwarded RX Error Counter %s Register (n = 0 to 1)"]
    #[inline(always)]
    pub const fn fwd_rx_err_count(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FwdRxErrCount_SPEC, crate::common::R>,
        2,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x308usize))
        }
    }
    #[inline(always)]
    pub const fn fwd_rx_err_count0(
        &self,
    ) -> &'static crate::common::Reg<self::FwdRxErrCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FwdRxErrCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fwd_rx_err_count1(
        &self,
    ) -> &'static crate::common::Reg<self::FwdRxErrCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FwdRxErrCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x309usize),
            )
        }
    }

    #[doc = "ECAT Processing Unit Error Counter Register"]
    #[inline(always)]
    pub const fn ecat_proc_err_count(
        &self,
    ) -> &'static crate::common::Reg<self::EcatProcErrCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EcatProcErrCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(780usize),
            )
        }
    }

    #[doc = "PDI Error Counter Register"]
    #[inline(always)]
    pub const fn pdi_err_count(
        &self,
    ) -> &'static crate::common::Reg<self::PdiErrCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::PdiErrCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(781usize),
            )
        }
    }

    #[doc = "Lost Link Counter %s Register (n = 0 to 1)"]
    #[inline(always)]
    pub const fn lost_link_count(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::LostLinkCount_SPEC, crate::common::R>,
        2,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x310usize))
        }
    }
    #[inline(always)]
    pub const fn lost_link_count0(
        &self,
    ) -> &'static crate::common::Reg<self::LostLinkCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::LostLinkCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn lost_link_count1(
        &self,
    ) -> &'static crate::common::Reg<self::LostLinkCount_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::LostLinkCount_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x311usize),
            )
        }
    }

    #[doc = "Watchdog Divider Register"]
    #[inline(always)]
    pub const fn wd_divide(
        &self,
    ) -> &'static crate::common::Reg<self::WdDivide_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WdDivide_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "Watchdog Time PDI Register"]
    #[inline(always)]
    pub const fn wdt_pdi(
        &self,
    ) -> &'static crate::common::Reg<self::WdtPdi_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WdtPdi_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Watchdog Time Process Data Register"]
    #[inline(always)]
    pub const fn wdt_data(
        &self,
    ) -> &'static crate::common::Reg<self::WdtData_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WdtData_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1056usize),
            )
        }
    }

    #[doc = "Watchdog Status Process Data Register"]
    #[inline(always)]
    pub const fn wds_data(
        &self,
    ) -> &'static crate::common::Reg<self::WdsData_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WdsData_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1088usize),
            )
        }
    }

    #[doc = "Watchdog Counter Process Data Register"]
    #[inline(always)]
    pub const fn wdc_data(
        &self,
    ) -> &'static crate::common::Reg<self::WdcData_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WdcData_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1090usize),
            )
        }
    }

    #[doc = "Watchdog Counter PDI Register"]
    #[inline(always)]
    pub const fn wdc_pdi(
        &self,
    ) -> &'static crate::common::Reg<self::WdcPdi_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::WdcPdi_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1091usize),
            )
        }
    }

    #[doc = "EEPROM Configuration Register"]
    #[inline(always)]
    pub const fn eep_conf(
        &self,
    ) -> &'static crate::common::Reg<self::EepConf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::EepConf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[doc = "EEPROM PDI Access State Register"]
    #[inline(always)]
    pub const fn eep_state(
        &self,
    ) -> &'static crate::common::Reg<self::EepState_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EepState_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1281usize),
            )
        }
    }

    #[doc = "EEPROM Control/Status Register"]
    #[inline(always)]
    pub const fn eep_cont_stat(
        &self,
    ) -> &'static crate::common::Reg<self::EepContStat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EepContStat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1282usize),
            )
        }
    }

    #[doc = "EEPROM Address Register"]
    #[inline(always)]
    pub const fn eep_adr(
        &self,
    ) -> &'static crate::common::Reg<self::EepAdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EepAdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[doc = "EEPROM Data Register"]
    #[inline(always)]
    pub const fn eep_data(
        &self,
    ) -> &'static crate::common::Reg<self::EepData_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EepData_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[doc = "MII Management Control/Status Register"]
    #[inline(always)]
    pub const fn mii_cont_stat(
        &self,
    ) -> &'static crate::common::Reg<self::MiiContStat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MiiContStat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1296usize),
            )
        }
    }

    #[doc = "PHY Address Register"]
    #[inline(always)]
    pub const fn phy_adr(
        &self,
    ) -> &'static crate::common::Reg<self::PhyAdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PhyAdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1298usize),
            )
        }
    }

    #[doc = "PHY Register Address Register"]
    #[inline(always)]
    pub const fn phy_reg_adr(
        &self,
    ) -> &'static crate::common::Reg<self::PhyRegAdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PhyRegAdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1299usize),
            )
        }
    }

    #[doc = "PHY Data Register"]
    #[inline(always)]
    pub const fn phy_data(
        &self,
    ) -> &'static crate::common::Reg<self::PhyData_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PhyData_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[doc = "MII Management ECAT Access State Register"]
    #[inline(always)]
    pub const fn mii_ecat_acs_stat(
        &self,
    ) -> &'static crate::common::Reg<self::MiiEcatAcsStat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::MiiEcatAcsStat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1302usize),
            )
        }
    }

    #[doc = "MII Management PDI Access State Register"]
    #[inline(always)]
    pub const fn mii_pdi_acs_stat(
        &self,
    ) -> &'static crate::common::Reg<self::MiiPdiAcsStat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MiiPdiAcsStat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1303usize),
            )
        }
    }

    #[doc = "FMMU Logical Start Address %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_l_start_adr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600usize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x620usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x630usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x640usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x650usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x660usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_l_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x670usize),
            )
        }
    }

    #[doc = "FMMU Length %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_len(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuLen_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x604usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x614usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x624usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x634usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x644usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x654usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x664usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_len(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x674usize),
            )
        }
    }

    #[doc = "FMMU Logical Start Bit %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_l_start_bit(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x606usize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x606usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x616usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x626usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x636usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x646usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x656usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x666usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_l_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x676usize),
            )
        }
    }

    #[doc = "FMMU Logical Stop Bit %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_l_stop_bit(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x607usize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x607usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x617usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x627usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x637usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x647usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x657usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x667usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_l_stop_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuLStopBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuLStopBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x677usize),
            )
        }
    }

    #[doc = "FMMU Physical Start Address %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_p_start_adr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x618usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x628usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x638usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x648usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x658usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x668usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x678usize),
            )
        }
    }

    #[doc = "FMMU Physical Start Bit %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_p_start_bit(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60ausize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x60ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x62ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x63ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x64ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x65ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x66ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_p_start_bit(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuPStartBit_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuPStartBit_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x67ausize),
            )
        }
    }

    #[doc = "FMMU Type %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_type(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuType_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60busize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x60busize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61busize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x62busize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x63busize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x64busize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x65busize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x66busize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_type(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuType_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuType_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x67busize),
            )
        }
    }

    #[doc = "FMMU Activate %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn fmmu_act(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::FmmuAct_SPEC, crate::common::R>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60cusize))
        }
    }
    #[inline(always)]
    pub const fn fmmu0_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x60cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu1_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x61cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu2_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x62cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu3_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x63cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu4_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x64cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu5_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x65cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu6_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x66cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmmu7_act(
        &self,
    ) -> &'static crate::common::Reg<self::FmmuAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::FmmuAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x67cusize),
            )
        }
    }

    #[doc = "SyncManager Physical Start Address %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn sm_p_start_adr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x800usize))
        }
    }
    #[inline(always)]
    pub const fn sm0_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm1_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x808usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm2_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x810usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm3_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x818usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm4_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x820usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm5_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x828usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm6_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x830usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm7_p_start_adr(
        &self,
    ) -> &'static crate::common::Reg<self::SmPStartAdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmPStartAdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x838usize),
            )
        }
    }

    #[doc = "SyncManager Length %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn sm_len(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::SmLen_SPEC, crate::common::R>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x802usize))
        }
    }
    #[inline(always)]
    pub const fn sm0_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x802usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm1_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm2_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x812usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm3_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x81ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm4_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x822usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm5_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x82ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm6_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x832usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm7_len(&self) -> &'static crate::common::Reg<self::SmLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x83ausize),
            )
        }
    }

    #[doc = "SyncManager Control %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn sm_control(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::SmControl_SPEC, crate::common::R>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x804usize))
        }
    }
    #[inline(always)]
    pub const fn sm0_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm1_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm2_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm3_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x81cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm4_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm5_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x82cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm6_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm7_control(
        &self,
    ) -> &'static crate::common::Reg<self::SmControl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmControl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x83cusize),
            )
        }
    }

    #[doc = "SyncManager Status %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn sm_status(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::SmStatus_SPEC, crate::common::R>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x805usize))
        }
    }
    #[inline(always)]
    pub const fn sm0_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x805usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm1_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm2_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x815usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm3_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x81dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm4_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x825usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm5_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x82dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm6_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x835usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm7_status(
        &self,
    ) -> &'static crate::common::Reg<self::SmStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x83dusize),
            )
        }
    }

    #[doc = "SyncManager Activate %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn sm_act(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::SmAct_SPEC, crate::common::R>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x806usize))
        }
    }
    #[inline(always)]
    pub const fn sm0_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x806usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm1_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm2_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x816usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm3_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x81eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm4_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x826usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm5_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x82eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm6_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x836usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm7_act(&self) -> &'static crate::common::Reg<self::SmAct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SmAct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x83eusize),
            )
        }
    }

    #[doc = "SyncManager PDI Control %s Register (n = 0 to 7)"]
    #[inline(always)]
    pub const fn sm_pdi_cont(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x807usize))
        }
    }
    #[inline(always)]
    pub const fn sm0_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x807usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm1_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm2_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x817usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm3_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x81fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm4_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x827usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm5_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x82fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm6_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x837usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sm7_pdi_cont(
        &self,
    ) -> &'static crate::common::Reg<self::SmPdiCont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmPdiCont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x83fusize),
            )
        }
    }

    #[doc = "Receive Time Port 0 Register"]
    #[inline(always)]
    pub const fn dc_rcv_time_port0(
        &self,
    ) -> &'static crate::common::Reg<self::DcRcvTimePort0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcRcvTimePort0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2304usize),
            )
        }
    }

    #[doc = "Receive Time Port 1 Register"]
    #[inline(always)]
    pub const fn dc_rcv_time_port1(
        &self,
    ) -> &'static crate::common::Reg<self::DcRcvTimePort1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcRcvTimePort1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2308usize),
            )
        }
    }

    #[doc = "Receive Time Port 2 Register"]
    #[inline(always)]
    pub const fn dc_rcv_time_port2(
        &self,
    ) -> &'static crate::common::Reg<self::DcRcvTimePort2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcRcvTimePort2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2312usize),
            )
        }
    }

    #[doc = "System Time Register L"]
    #[inline(always)]
    pub const fn dc_sys_time_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcSysTimeL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSysTimeL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2320usize),
            )
        }
    }

    #[doc = "System Time Register H"]
    #[inline(always)]
    pub const fn dc_sys_time_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcSysTimeH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSysTimeH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2324usize),
            )
        }
    }

    #[doc = "Receive Time ECAT Processing Unit Register L"]
    #[inline(always)]
    pub const fn dc_rcv_time_unit_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcRcvTimeUnitL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcRcvTimeUnitL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2328usize),
            )
        }
    }

    #[doc = "Receive Time ECAT Processing Unit Register H"]
    #[inline(always)]
    pub const fn dc_rcv_time_unit_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcRcvTimeUnitH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcRcvTimeUnitH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2332usize),
            )
        }
    }

    #[doc = "System Time Offset Register L"]
    #[inline(always)]
    pub const fn dc_sys_time_offset_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcSysTimeOffsetL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSysTimeOffsetL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2336usize),
            )
        }
    }

    #[doc = "System Time Offset Register H"]
    #[inline(always)]
    pub const fn dc_sys_time_offset_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcSysTimeOffsetH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSysTimeOffsetH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2340usize),
            )
        }
    }

    #[doc = "System Time Delay Register"]
    #[inline(always)]
    pub const fn dc_sys_time_delay(
        &self,
    ) -> &'static crate::common::Reg<self::DcSysTimeDelay_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSysTimeDelay_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2344usize),
            )
        }
    }

    #[doc = "System Time Difference Register"]
    #[inline(always)]
    pub const fn dc_sys_time_diff(
        &self,
    ) -> &'static crate::common::Reg<self::DcSysTimeDiff_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSysTimeDiff_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2348usize),
            )
        }
    }

    #[doc = "Speed Counter Start Register"]
    #[inline(always)]
    pub const fn dc_speed_count_start(
        &self,
    ) -> &'static crate::common::Reg<self::DcSpeedCountStart_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSpeedCountStart_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2352usize),
            )
        }
    }

    #[doc = "Speed Counter Difference Register"]
    #[inline(always)]
    pub const fn dc_speed_count_diff(
        &self,
    ) -> &'static crate::common::Reg<self::DcSpeedCountDiff_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSpeedCountDiff_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2354usize),
            )
        }
    }

    #[doc = "System Time Difference Filter Depth Register"]
    #[inline(always)]
    pub const fn dc_sys_time_diff_fil_depth(
        &self,
    ) -> &'static crate::common::Reg<self::DcSysTimeDiffFilDepth_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSysTimeDiffFilDepth_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2356usize),
            )
        }
    }

    #[doc = "Speed Counter Filter Depth Register"]
    #[inline(always)]
    pub const fn dc_speed_count_fil_depth(
        &self,
    ) -> &'static crate::common::Reg<self::DcSpeedCountFilDepth_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSpeedCountFilDepth_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2357usize),
            )
        }
    }

    #[doc = "Cyclic Unit Control Register"]
    #[inline(always)]
    pub const fn dc_cyc_cont(
        &self,
    ) -> &'static crate::common::Reg<self::DcCycCont_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcCycCont_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2432usize),
            )
        }
    }

    #[doc = "Activation Register"]
    #[inline(always)]
    pub const fn dc_act(&self) -> &'static crate::common::Reg<self::DcAct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcAct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2433usize),
            )
        }
    }

    #[doc = "SYNC Signal Pulse Length Register"]
    #[inline(always)]
    pub const fn dc_pulse_len(
        &self,
    ) -> &'static crate::common::Reg<self::DcPulseLen_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcPulseLen_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2434usize),
            )
        }
    }

    #[doc = "Activation Status Register"]
    #[inline(always)]
    pub const fn dc_act_stat(
        &self,
    ) -> &'static crate::common::Reg<self::DcActStat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcActStat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2436usize),
            )
        }
    }

    #[doc = "SYNC0 Status Register"]
    #[inline(always)]
    pub const fn dc_sync0_stat(
        &self,
    ) -> &'static crate::common::Reg<self::DcSync0Stat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSync0Stat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2446usize),
            )
        }
    }

    #[doc = "SYNC1 Status Register"]
    #[inline(always)]
    pub const fn dc_sync1_stat(
        &self,
    ) -> &'static crate::common::Reg<self::DcSync1Stat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcSync1Stat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2447usize),
            )
        }
    }

    #[doc = "Start Time Cyclic Operation/Next SYNC0 Pulse Register L"]
    #[inline(always)]
    pub const fn dc_cyc_start_time_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcCycStartTimeL_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcCycStartTimeL_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2448usize),
            )
        }
    }

    #[doc = "Start Time Cyclic Operation/Next SYNC0 Pulse Register H"]
    #[inline(always)]
    pub const fn dc_cyc_start_time_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcCycStartTimeH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcCycStartTimeH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2452usize),
            )
        }
    }

    #[doc = "Next SYNC1 Pulse Register L"]
    #[inline(always)]
    pub const fn dc_next_sync1_pulse_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcNextSync1PulseL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcNextSync1PulseL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2456usize),
            )
        }
    }

    #[doc = "Next SYNC1 Pulse Register H"]
    #[inline(always)]
    pub const fn dc_next_sync1_pulse_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcNextSync1PulseH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcNextSync1PulseH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2460usize),
            )
        }
    }

    #[doc = "SYNC0 Cycle Time Register"]
    #[inline(always)]
    pub const fn dc_sync0_cyc_time(
        &self,
    ) -> &'static crate::common::Reg<self::DcSync0CycTime_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcSync0CycTime_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2464usize),
            )
        }
    }

    #[doc = "SYNC1 Cycle Time Register"]
    #[inline(always)]
    pub const fn dc_sync1_cyc_time(
        &self,
    ) -> &'static crate::common::Reg<self::DcSync1CycTime_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcSync1CycTime_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2468usize),
            )
        }
    }

    #[doc = "Latch 0 Control Register"]
    #[inline(always)]
    pub const fn dc_latch0_cont(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch0Cont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcLatch0Cont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2472usize),
            )
        }
    }

    #[doc = "Latch 1 Control Register"]
    #[inline(always)]
    pub const fn dc_latch1_cont(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch1Cont_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcLatch1Cont_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2473usize),
            )
        }
    }

    #[doc = "Latch 0 Status Register"]
    #[inline(always)]
    pub const fn dc_latch0_stat(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch0Stat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch0Stat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2478usize),
            )
        }
    }

    #[doc = "Latch 1 Status Register"]
    #[inline(always)]
    pub const fn dc_latch1_stat(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch1Stat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch1Stat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2479usize),
            )
        }
    }

    #[doc = "Latch 0 Time Positive Edge Register L"]
    #[inline(always)]
    pub const fn dc_latch0_time_pos_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch0TimePosL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch0TimePosL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2480usize),
            )
        }
    }

    #[doc = "Latch 0 Time Positive Edge Register H"]
    #[inline(always)]
    pub const fn dc_latch0_time_pos_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch0TimePosH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch0TimePosH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2484usize),
            )
        }
    }

    #[doc = "Latch 0 Time Negative Edge Register L"]
    #[inline(always)]
    pub const fn dc_latch0_time_neg_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch0TimeNegL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch0TimeNegL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2488usize),
            )
        }
    }

    #[doc = "Latch 0 Time Negative Edge Register H"]
    #[inline(always)]
    pub const fn dc_latch0_time_neg_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch0TimeNegH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch0TimeNegH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2492usize),
            )
        }
    }

    #[doc = "Latch 1 Time Positive Edge Register L"]
    #[inline(always)]
    pub const fn dc_latch1_time_pos_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch1TimePosL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch1TimePosL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2496usize),
            )
        }
    }

    #[doc = "Latch 1 Time Positive Edge Register H"]
    #[inline(always)]
    pub const fn dc_latch1_time_pos_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch1TimePosH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch1TimePosH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2500usize),
            )
        }
    }

    #[doc = "Latch 1 Time Negative Edge Register L"]
    #[inline(always)]
    pub const fn dc_latch1_time_neg_l(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch1TimeNegL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch1TimeNegL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2504usize),
            )
        }
    }

    #[doc = "Latch 1 Time Negative Edge Register H"]
    #[inline(always)]
    pub const fn dc_latch1_time_neg_h(
        &self,
    ) -> &'static crate::common::Reg<self::DcLatch1TimeNegH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcLatch1TimeNegH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2508usize),
            )
        }
    }

    #[doc = "Buffer Change Event Time Register"]
    #[inline(always)]
    pub const fn dc_ecat_cng_ev_time(
        &self,
    ) -> &'static crate::common::Reg<self::DcEcatCngEvTime_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcEcatCngEvTime_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2544usize),
            )
        }
    }

    #[doc = "PDI Buffer Start Event Time Register"]
    #[inline(always)]
    pub const fn dc_pdi_start_ev_time(
        &self,
    ) -> &'static crate::common::Reg<self::DcPdiStartEvTime_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcPdiStartEvTime_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2552usize),
            )
        }
    }

    #[doc = "PDI Buffer Change Event Time Register"]
    #[inline(always)]
    pub const fn dc_pdi_cng_ev_time(
        &self,
    ) -> &'static crate::common::Reg<self::DcPdiCngEvTime_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::DcPdiCngEvTime_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2556usize),
            )
        }
    }

    #[doc = "Product ID Register L"]
    #[inline(always)]
    pub const fn product_id_l(
        &self,
    ) -> &'static crate::common::Reg<self::ProductIdL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::ProductIdL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3584usize),
            )
        }
    }

    #[doc = "Product ID Register H"]
    #[inline(always)]
    pub const fn product_id_h(
        &self,
    ) -> &'static crate::common::Reg<self::ProductIdH_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::ProductIdH_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3588usize),
            )
        }
    }

    #[doc = "Vendor ID Register L"]
    #[inline(always)]
    pub const fn vendor_id_l(
        &self,
    ) -> &'static crate::common::Reg<self::VendorIdL_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::VendorIdL_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3592usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type_SPEC;
impl crate::sealed::RegSpec for Type_SPEC {
    type DataType = u8;
}

#[doc = "Type Register"]
pub type Type = crate::RegValueT<Type_SPEC>;

impl Type {
    #[doc = "Type of the EtherCAT slave controller"]
    #[inline(always)]
    pub fn r#type(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Type_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Type_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Type {
    #[inline(always)]
    fn default() -> Type {
        <crate::RegValueT<Type_SPEC> as RegisterValue<_>>::new(160)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Revision_SPEC;
impl crate::sealed::RegSpec for Revision_SPEC {
    type DataType = u8;
}

#[doc = "Revision Register"]
pub type Revision = crate::RegValueT<Revision_SPEC>;

impl Revision {
    #[doc = "Revision of the EtherCAT slave controller"]
    #[inline(always)]
    pub fn rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Revision_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Revision_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Revision {
    #[inline(always)]
    fn default() -> Revision {
        <crate::RegValueT<Revision_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Build_SPEC;
impl crate::sealed::RegSpec for Build_SPEC {
    type DataType = u8;
}

#[doc = "Build Register"]
pub type Build = crate::RegValueT<Build_SPEC>;

impl Build {
    #[doc = "Build number of the EtherCAT slave controller"]
    #[inline(always)]
    pub fn build(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Build_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Build_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Build {
    #[inline(always)]
    fn default() -> Build {
        <crate::RegValueT<Build_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuNum_SPEC;
impl crate::sealed::RegSpec for FmmuNum_SPEC {
    type DataType = u8;
}

#[doc = "FMMU Supported Register"]
pub type FmmuNum = crate::RegValueT<FmmuNum_SPEC>;

impl FmmuNum {
    #[doc = "Number of FMMU channels supported in the EtherCAT slave controller"]
    #[inline(always)]
    pub fn numfmmu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, FmmuNum_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,FmmuNum_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FmmuNum {
    #[inline(always)]
    fn default() -> FmmuNum {
        <crate::RegValueT<FmmuNum_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncManager_SPEC;
impl crate::sealed::RegSpec for SyncManager_SPEC {
    type DataType = u8;
}

#[doc = "SyncManager Supported Register"]
pub type SyncManager = crate::RegValueT<SyncManager_SPEC>;

impl SyncManager {
    #[doc = "Number of SyncManager channels supported in the EtherCAT slave controller"]
    #[inline(always)]
    pub fn numsync(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SyncManager_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,SyncManager_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SyncManager {
    #[inline(always)]
    fn default() -> SyncManager {
        <crate::RegValueT<SyncManager_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamSize_SPEC;
impl crate::sealed::RegSpec for RamSize_SPEC {
    type DataType = u8;
}

#[doc = "RAM Size Register"]
pub type RamSize = crate::RegValueT<RamSize_SPEC>;

impl RamSize {
    #[doc = "Process data RAM size supported in the EtherCAT slave controller (unit: KB)"]
    #[inline(always)]
    pub fn ramsize(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RamSize_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,RamSize_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RamSize {
    #[inline(always)]
    fn default() -> RamSize {
        <crate::RegValueT<RamSize_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortDesc_SPEC;
impl crate::sealed::RegSpec for PortDesc_SPEC {
    type DataType = u8;
}

#[doc = "Port Descriptor Register"]
pub type PortDesc = crate::RegValueT<PortDesc_SPEC>;

impl PortDesc {
    #[doc = "Port 0 configuration"]
    #[inline(always)]
    pub fn p0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        port_desc::P0,
        port_desc::P0,
        PortDesc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            port_desc::P0,
            port_desc::P0,
            PortDesc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 1 configuration"]
    #[inline(always)]
    pub fn p1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        port_desc::P1,
        port_desc::P1,
        PortDesc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            port_desc::P1,
            port_desc::P1,
            PortDesc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 2 configuration"]
    #[inline(always)]
    pub fn p2(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        port_desc::P2,
        port_desc::P2,
        PortDesc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            port_desc::P2,
            port_desc::P2,
            PortDesc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 3 configuration"]
    #[inline(always)]
    pub fn p3(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        port_desc::P3,
        port_desc::P3,
        PortDesc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            port_desc::P3,
            port_desc::P3,
            PortDesc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PortDesc {
    #[inline(always)]
    fn default() -> PortDesc {
        <crate::RegValueT<PortDesc_SPEC> as RegisterValue<_>>::new(63)
    }
}
pub mod port_desc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P0_SPEC;
    pub type P0 = crate::EnumBitfieldStruct<u8, P0_SPEC>;
    impl P0 {
        #[doc = "Not implemented"]
        pub const _00: Self = Self::new(0);

        #[doc = "Not configured (SII EEPROM)"]
        pub const _01: Self = Self::new(1);

        #[doc = "EBUS"]
        pub const _10: Self = Self::new(2);

        #[doc = "MII"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P1_SPEC;
    pub type P1 = crate::EnumBitfieldStruct<u8, P1_SPEC>;
    impl P1 {
        #[doc = "Not implemented"]
        pub const _00: Self = Self::new(0);

        #[doc = "Not configured (SII EEPROM)"]
        pub const _01: Self = Self::new(1);

        #[doc = "EBUS"]
        pub const _10: Self = Self::new(2);

        #[doc = "MII"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P2_SPEC;
    pub type P2 = crate::EnumBitfieldStruct<u8, P2_SPEC>;
    impl P2 {
        #[doc = "Not implemented"]
        pub const _00: Self = Self::new(0);

        #[doc = "Not configured (SII EEPROM)"]
        pub const _01: Self = Self::new(1);

        #[doc = "EBUS"]
        pub const _10: Self = Self::new(2);

        #[doc = "MII"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct P3_SPEC;
    pub type P3 = crate::EnumBitfieldStruct<u8, P3_SPEC>;
    impl P3 {
        #[doc = "Not implemented"]
        pub const _00: Self = Self::new(0);

        #[doc = "Not configured (SII EEPROM)"]
        pub const _01: Self = Self::new(1);

        #[doc = "EBUS"]
        pub const _10: Self = Self::new(2);

        #[doc = "MII"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feature_SPEC;
impl crate::sealed::RegSpec for Feature_SPEC {
    type DataType = u16;
}

#[doc = "ESC Features Supported Register"]
pub type Feature = crate::RegValueT<Feature_SPEC>;

impl Feature {
    #[doc = "FMMU Operation"]
    #[inline(always)]
    pub fn fmmu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        feature::Fmmu,
        feature::Fmmu,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            feature::Fmmu,
            feature::Fmmu,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Distributed Clock"]
    #[inline(always)]
    pub fn dc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        feature::Dc,
        feature::Dc,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            feature::Dc,
            feature::Dc,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Distributed Clock Width"]
    #[inline(always)]
    pub fn dcwid(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        feature::Dcwid,
        feature::Dcwid,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            feature::Dcwid,
            feature::Dcwid,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Enhanced Link Detection in MII"]
    #[inline(always)]
    pub fn linkdecmii(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        feature::Linkdecmii,
        feature::Linkdecmii,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            feature::Linkdecmii,
            feature::Linkdecmii,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Separate handling of FCS errors"]
    #[inline(always)]
    pub fn fcs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        feature::Fcs,
        feature::Fcs,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            feature::Fcs,
            feature::Fcs,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Enhanced DC SYNC activation"]
    #[inline(always)]
    pub fn dcsync(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        feature::Dcsync,
        feature::Dcsync,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            feature::Dcsync,
            feature::Dcsync,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EtherCAT LRW command support"]
    #[inline(always)]
    pub fn lrw(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        feature::Lrw,
        feature::Lrw,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            feature::Lrw,
            feature::Lrw,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EtherCAT read/write command support (BRW, APRW, FPRW)"]
    #[inline(always)]
    pub fn rwsupp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        feature::Rwsupp,
        feature::Rwsupp,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            feature::Rwsupp,
            feature::Rwsupp,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Fixed FMMU/SyncManager configuration"]
    #[inline(always)]
    pub fn fsconfig(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        feature::Fsconfig,
        feature::Fsconfig,
        Feature_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            feature::Fsconfig,
            feature::Fsconfig,
            Feature_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Feature {
    #[inline(always)]
    fn default() -> Feature {
        <crate::RegValueT<Feature_SPEC> as RegisterValue<_>>::new(460)
    }
}
pub mod feature {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmmu_SPEC;
    pub type Fmmu = crate::EnumBitfieldStruct<u8, Fmmu_SPEC>;
    impl Fmmu {
        #[doc = "Bit oriented"]
        pub const _0: Self = Self::new(0);

        #[doc = "Byte oriented"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dc_SPEC;
    pub type Dc = crate::EnumBitfieldStruct<u8, Dc_SPEC>;
    impl Dc {
        #[doc = "Not available"]
        pub const _0: Self = Self::new(0);

        #[doc = "Available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcwid_SPEC;
    pub type Dcwid = crate::EnumBitfieldStruct<u8, Dcwid_SPEC>;
    impl Dcwid {
        #[doc = "32 bits"]
        pub const _0: Self = Self::new(0);

        #[doc = "64 bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Linkdecmii_SPEC;
    pub type Linkdecmii = crate::EnumBitfieldStruct<u8, Linkdecmii_SPEC>;
    impl Linkdecmii {
        #[doc = "Not available"]
        pub const _0: Self = Self::new(0);

        #[doc = "Available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fcs_SPEC;
    pub type Fcs = crate::EnumBitfieldStruct<u8, Fcs_SPEC>;
    impl Fcs {
        #[doc = "Not supported"]
        pub const _0: Self = Self::new(0);

        #[doc = "Supported. Frames with wrong FCS and additional nibble will be counted separately in forwarded RX error counter."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcsync_SPEC;
    pub type Dcsync = crate::EnumBitfieldStruct<u8, Dcsync_SPEC>;
    impl Dcsync {
        #[doc = "Not available"]
        pub const _0: Self = Self::new(0);

        #[doc = "Available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrw_SPEC;
    pub type Lrw = crate::EnumBitfieldStruct<u8, Lrw_SPEC>;
    impl Lrw {
        #[doc = "Supported"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not supported"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwsupp_SPEC;
    pub type Rwsupp = crate::EnumBitfieldStruct<u8, Rwsupp_SPEC>;
    impl Rwsupp {
        #[doc = "Supported"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not supported"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsconfig_SPEC;
    pub type Fsconfig = crate::EnumBitfieldStruct<u8, Fsconfig_SPEC>;
    impl Fsconfig {
        #[doc = "Variable configuration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Fixed configuration"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StationAdr_SPEC;
impl crate::sealed::RegSpec for StationAdr_SPEC {
    type DataType = u16;
}

#[doc = "Configured Station Address Register"]
pub type StationAdr = crate::RegValueT<StationAdr_SPEC>;

impl StationAdr {
    #[doc = "Node Addressing Address Indication"]
    #[inline(always)]
    pub fn nodaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, StationAdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,StationAdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for StationAdr {
    #[inline(always)]
    fn default() -> StationAdr {
        <crate::RegValueT<StationAdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StationAlias_SPEC;
impl crate::sealed::RegSpec for StationAlias_SPEC {
    type DataType = u16;
}

#[doc = "Configured Station Alias Register"]
pub type StationAlias = crate::RegValueT<StationAlias_SPEC>;

impl StationAlias {
    #[doc = "Alias Address Indication"]
    #[inline(always)]
    pub fn nodaliaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, StationAlias_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            StationAlias_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for StationAlias {
    #[inline(always)]
    fn default() -> StationAlias {
        <crate::RegValueT<StationAlias_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrRegEnable_SPEC;
impl crate::sealed::RegSpec for WrRegEnable_SPEC {
    type DataType = u8;
}

#[doc = "Write Register Enable Register"]
pub type WrRegEnable = crate::RegValueT<WrRegEnable_SPEC>;

impl WrRegEnable {
    #[doc = "Register Write Protection Unlock"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, WrRegEnable_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,WrRegEnable_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WrRegEnable {
    #[inline(always)]
    fn default() -> WrRegEnable {
        <crate::RegValueT<WrRegEnable_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrRegProtect_SPEC;
impl crate::sealed::RegSpec for WrRegProtect_SPEC {
    type DataType = u8;
}

#[doc = "Write Register Protection Register"]
pub type WrRegProtect = crate::RegValueT<WrRegProtect_SPEC>;

impl WrRegProtect {
    #[doc = "Register Write Protection Specification"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wr_reg_protect::Protect,
        wr_reg_protect::Protect,
        WrRegProtect_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wr_reg_protect::Protect,
            wr_reg_protect::Protect,
            WrRegProtect_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WrRegProtect {
    #[inline(always)]
    fn default() -> WrRegProtect {
        <crate::RegValueT<WrRegProtect_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wr_reg_protect {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Protection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Protection enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscWrEnable_SPEC;
impl crate::sealed::RegSpec for EscWrEnable_SPEC {
    type DataType = u8;
}

#[doc = "ESC Write Enable Register"]
pub type EscWrEnable = crate::RegValueT<EscWrEnable_SPEC>;

impl EscWrEnable {
    #[doc = "Register/Memory Write Protection Unlock"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EscWrEnable_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,EscWrEnable_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EscWrEnable {
    #[inline(always)]
    fn default() -> EscWrEnable {
        <crate::RegValueT<EscWrEnable_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscWrProtect_SPEC;
impl crate::sealed::RegSpec for EscWrProtect_SPEC {
    type DataType = u8;
}

#[doc = "ESC Write Protection Register"]
pub type EscWrProtect = crate::RegValueT<EscWrProtect_SPEC>;

impl EscWrProtect {
    #[doc = "Register/Memory Write Protection Specification"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        esc_wr_protect::Protect,
        esc_wr_protect::Protect,
        EscWrProtect_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            esc_wr_protect::Protect,
            esc_wr_protect::Protect,
            EscWrProtect_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EscWrProtect {
    #[inline(always)]
    fn default() -> EscWrProtect {
        <crate::RegValueT<EscWrProtect_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esc_wr_protect {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Protection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Protection enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscResetEcatR_SPEC;
impl crate::sealed::RegSpec for EscResetEcatR_SPEC {
    type DataType = u8;
}

#[doc = "ESC Reset ECAT Register for read"]
pub type EscResetEcatR = crate::RegValueT<EscResetEcatR_SPEC>;

impl EscResetEcatR {
    #[doc = "Reset Progress Status"]
    #[inline(always)]
    pub fn reset_ecat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        esc_reset_ecat_r::ResetEcat,
        esc_reset_ecat_r::ResetEcat,
        EscResetEcatR_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            esc_reset_ecat_r::ResetEcat,
            esc_reset_ecat_r::ResetEcat,
            EscResetEcatR_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EscResetEcatR {
    #[inline(always)]
    fn default() -> EscResetEcatR {
        <crate::RegValueT<EscResetEcatR_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esc_reset_ecat_r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ResetEcat_SPEC;
    pub type ResetEcat = crate::EnumBitfieldStruct<u8, ResetEcat_SPEC>;
    impl ResetEcat {
        #[doc = "After writing 0x52"]
        pub const _01: Self = Self::new(1);

        #[doc = "After writing 0x45 (if 0x52 was written before)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Others"]
        pub const _00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscResetEcatW_SPEC;
impl crate::sealed::RegSpec for EscResetEcatW_SPEC {
    type DataType = u8;
}

#[doc = "ESC Reset ECAT Register for write"]
pub type EscResetEcatW = crate::RegValueT<EscResetEcatW_SPEC>;

impl EscResetEcatW {
    #[doc = "Software Reset Setting"]
    #[inline(always)]
    pub fn reset_ecat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, EscResetEcatW_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,EscResetEcatW_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EscResetEcatW {
    #[inline(always)]
    fn default() -> EscResetEcatW {
        <crate::RegValueT<EscResetEcatW_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscResetPdiR_SPEC;
impl crate::sealed::RegSpec for EscResetPdiR_SPEC {
    type DataType = u8;
}

#[doc = "ESC Reset PDI Register for read"]
pub type EscResetPdiR = crate::RegValueT<EscResetPdiR_SPEC>;

impl EscResetPdiR {
    #[doc = "Reset Progress Status"]
    #[inline(always)]
    pub fn reset_pdi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        esc_reset_pdi_r::ResetPdi,
        esc_reset_pdi_r::ResetPdi,
        EscResetPdiR_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            esc_reset_pdi_r::ResetPdi,
            esc_reset_pdi_r::ResetPdi,
            EscResetPdiR_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EscResetPdiR {
    #[inline(always)]
    fn default() -> EscResetPdiR {
        <crate::RegValueT<EscResetPdiR_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esc_reset_pdi_r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ResetPdi_SPEC;
    pub type ResetPdi = crate::EnumBitfieldStruct<u8, ResetPdi_SPEC>;
    impl ResetPdi {
        #[doc = "After writing 0x52"]
        pub const _01: Self = Self::new(1);

        #[doc = "After writing 0x45 (if 0x52 was written before)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Others"]
        pub const _00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscResetPdiW_SPEC;
impl crate::sealed::RegSpec for EscResetPdiW_SPEC {
    type DataType = u8;
}

#[doc = "ESC Reset PDI Register for write"]
pub type EscResetPdiW = crate::RegValueT<EscResetPdiW_SPEC>;

impl EscResetPdiW {
    #[doc = "Software Reset Setting"]
    #[inline(always)]
    pub fn reset_pdi(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, EscResetPdiW_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,EscResetPdiW_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EscResetPdiW {
    #[inline(always)]
    fn default() -> EscResetPdiW {
        <crate::RegValueT<EscResetPdiW_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscDlControl_SPEC;
impl crate::sealed::RegSpec for EscDlControl_SPEC {
    type DataType = u32;
}

#[doc = "ESC DL Control Register"]
pub type EscDlControl = crate::RegValueT<EscDlControl_SPEC>;

impl EscDlControl {
    #[doc = "Forwarding Rule"]
    #[inline(always)]
    pub fn fwdrule(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        esc_dl_control::Fwdrule,
        esc_dl_control::Fwdrule,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            esc_dl_control::Fwdrule,
            esc_dl_control::Fwdrule,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Temporary Use of Bits 15 to 8 Settings"]
    #[inline(always)]
    pub fn tempuse(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        esc_dl_control::Tempuse,
        esc_dl_control::Tempuse,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            esc_dl_control::Tempuse,
            esc_dl_control::Tempuse,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 0 Configuration"]
    #[inline(always)]
    pub fn lp0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        esc_dl_control::Lp0,
        esc_dl_control::Lp0,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            esc_dl_control::Lp0,
            esc_dl_control::Lp0,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 1 Configuration"]
    #[inline(always)]
    pub fn lp1(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        esc_dl_control::Lp1,
        esc_dl_control::Lp1,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            esc_dl_control::Lp1,
            esc_dl_control::Lp1,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 2 Configuration"]
    #[inline(always)]
    pub fn lp2(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        esc_dl_control::Lp2,
        esc_dl_control::Lp2,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            esc_dl_control::Lp2,
            esc_dl_control::Lp2,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 3 Configuration"]
    #[inline(always)]
    pub fn lp3(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        esc_dl_control::Lp3,
        esc_dl_control::Lp3,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            esc_dl_control::Lp3,
            esc_dl_control::Lp3,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RX FIFO Size"]
    #[inline(always)]
    pub fn rxfifo(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        esc_dl_control::Rxfifo,
        esc_dl_control::Rxfifo,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            esc_dl_control::Rxfifo,
            esc_dl_control::Rxfifo,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Station Alias Status"]
    #[inline(always)]
    pub fn staalias(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        esc_dl_control::Staalias,
        esc_dl_control::Staalias,
        EscDlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            esc_dl_control::Staalias,
            esc_dl_control::Staalias,
            EscDlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EscDlControl {
    #[inline(always)]
    fn default() -> EscDlControl {
        <crate::RegValueT<EscDlControl_SPEC> as RegisterValue<_>>::new(507905)
    }
}
pub mod esc_dl_control {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fwdrule_SPEC;
    pub type Fwdrule = crate::EnumBitfieldStruct<u8, Fwdrule_SPEC>;
    impl Fwdrule {
        #[doc = "EtherCAT frames are processed. Non-EtherCAT frames are forwarded without processing."]
        pub const _0: Self = Self::new(0);

        #[doc = "EtherCAT frames are processed. Non-EtherCAT frames are destroyed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tempuse_SPEC;
    pub type Tempuse = crate::EnumBitfieldStruct<u8, Tempuse_SPEC>;
    impl Tempuse {
        #[doc = "Permanent use"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use for about 1 second, then revert to previous settings"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp0_SPEC;
    pub type Lp0 = crate::EnumBitfieldStruct<u8, Lp0_SPEC>;
    impl Lp0 {
        #[doc = "Auto"]
        pub const _00: Self = Self::new(0);

        #[doc = "Auto close"]
        pub const _01: Self = Self::new(1);

        #[doc = "Open"]
        pub const _10: Self = Self::new(2);

        #[doc = "Closed"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp1_SPEC;
    pub type Lp1 = crate::EnumBitfieldStruct<u8, Lp1_SPEC>;
    impl Lp1 {
        #[doc = "Auto"]
        pub const _00: Self = Self::new(0);

        #[doc = "Auto close"]
        pub const _01: Self = Self::new(1);

        #[doc = "Open"]
        pub const _10: Self = Self::new(2);

        #[doc = "Closed"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp2_SPEC;
    pub type Lp2 = crate::EnumBitfieldStruct<u8, Lp2_SPEC>;
    impl Lp2 {
        #[doc = "Auto"]
        pub const _00: Self = Self::new(0);

        #[doc = "Auto close"]
        pub const _01: Self = Self::new(1);

        #[doc = "Open"]
        pub const _10: Self = Self::new(2);

        #[doc = "Closed"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp3_SPEC;
    pub type Lp3 = crate::EnumBitfieldStruct<u8, Lp3_SPEC>;
    impl Lp3 {
        #[doc = "Auto"]
        pub const _00: Self = Self::new(0);

        #[doc = "Auto close"]
        pub const _01: Self = Self::new(1);

        #[doc = "Open"]
        pub const _10: Self = Self::new(2);

        #[doc = "Closed"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxfifo_SPEC;
    pub type Rxfifo = crate::EnumBitfieldStruct<u8, Rxfifo_SPEC>;
    impl Rxfifo {
        #[doc = "Shortened by 40 ns"]
        pub const _00_X: Self = Self::new(0);

        #[doc = "Shortened by 40 ns"]
        pub const _01_X: Self = Self::new(2);

        #[doc = "No change"]
        pub const _110: Self = Self::new(6);

        #[doc = "Default"]
        pub const _111: Self = Self::new(7);

        #[doc = "No change"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Staalias_SPEC;
    pub type Staalias = crate::EnumBitfieldStruct<u8, Staalias_SPEC>;
    impl Staalias {
        #[doc = "Ignore station alias"]
        pub const _0: Self = Self::new(0);

        #[doc = "Alias can be used for all configured address command types such as FPRD, FPWR."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhysicalRwOffset_SPEC;
impl crate::sealed::RegSpec for PhysicalRwOffset_SPEC {
    type DataType = u16;
}

#[doc = "Physical Read/Write Offset Register"]
pub type PhysicalRwOffset = crate::RegValueT<PhysicalRwOffset_SPEC>;

impl PhysicalRwOffset {
    #[doc = "Offset between Read and Write Addresses"]
    #[inline(always)]
    pub fn rwoffset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        PhysicalRwOffset_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            PhysicalRwOffset_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PhysicalRwOffset {
    #[inline(always)]
    fn default() -> PhysicalRwOffset {
        <crate::RegValueT<PhysicalRwOffset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscDlStatus_SPEC;
impl crate::sealed::RegSpec for EscDlStatus_SPEC {
    type DataType = u16;
}

#[doc = "ESC DL Status Register"]
pub type EscDlStatus = crate::RegValueT<EscDlStatus_SPEC>;

impl EscDlStatus {
    #[doc = "PDI/EEPROM Load State Indication"]
    #[inline(always)]
    pub fn pdiope(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        esc_dl_status::Pdiope,
        esc_dl_status::Pdiope,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            esc_dl_status::Pdiope,
            esc_dl_status::Pdiope,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PDI Watchdog Timer Status"]
    #[inline(always)]
    pub fn pdiwdst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        esc_dl_status::Pdiwdst,
        esc_dl_status::Pdiwdst,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            esc_dl_status::Pdiwdst,
            esc_dl_status::Pdiwdst,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Enhanced Link Detection Indication"]
    #[inline(always)]
    pub fn enhlinkd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        esc_dl_status::Enhlinkd,
        esc_dl_status::Enhlinkd,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            esc_dl_status::Enhlinkd,
            esc_dl_status::Enhlinkd,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 0 Link State Indication"]
    #[inline(always)]
    pub fn phyp0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        esc_dl_status::Phyp0,
        esc_dl_status::Phyp0,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            esc_dl_status::Phyp0,
            esc_dl_status::Phyp0,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 1 Link State Indication"]
    #[inline(always)]
    pub fn phyp1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        esc_dl_status::Phyp1,
        esc_dl_status::Phyp1,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            esc_dl_status::Phyp1,
            esc_dl_status::Phyp1,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 2 Link State Indication"]
    #[inline(always)]
    pub fn phyp2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        esc_dl_status::Phyp2,
        esc_dl_status::Phyp2,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            esc_dl_status::Phyp2,
            esc_dl_status::Phyp2,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 3 Link State Indication"]
    #[inline(always)]
    pub fn phyp3(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        esc_dl_status::Phyp3,
        esc_dl_status::Phyp3,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            esc_dl_status::Phyp3,
            esc_dl_status::Phyp3,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 0 State Indication"]
    #[inline(always)]
    pub fn lp0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        esc_dl_status::Lp0,
        esc_dl_status::Lp0,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            esc_dl_status::Lp0,
            esc_dl_status::Lp0,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 0 Communication State Indication"]
    #[inline(always)]
    pub fn comp0(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        esc_dl_status::Comp0,
        esc_dl_status::Comp0,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            esc_dl_status::Comp0,
            esc_dl_status::Comp0,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 1 State Indication"]
    #[inline(always)]
    pub fn lp1(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        esc_dl_status::Lp1,
        esc_dl_status::Lp1,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            esc_dl_status::Lp1,
            esc_dl_status::Lp1,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 1 Communication State Indication"]
    #[inline(always)]
    pub fn comp1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        esc_dl_status::Comp1,
        esc_dl_status::Comp1,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            esc_dl_status::Comp1,
            esc_dl_status::Comp1,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 2 State Indication"]
    #[inline(always)]
    pub fn lp2(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        esc_dl_status::Lp2,
        esc_dl_status::Lp2,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            esc_dl_status::Lp2,
            esc_dl_status::Lp2,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 2 Communication State Indication"]
    #[inline(always)]
    pub fn comp2(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        esc_dl_status::Comp2,
        esc_dl_status::Comp2,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            esc_dl_status::Comp2,
            esc_dl_status::Comp2,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Loop Port 3 State Indication"]
    #[inline(always)]
    pub fn lp3(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        esc_dl_status::Lp3,
        esc_dl_status::Lp3,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            esc_dl_status::Lp3,
            esc_dl_status::Lp3,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 3 Communication State Indication"]
    #[inline(always)]
    pub fn comp3(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        esc_dl_status::Comp3,
        esc_dl_status::Comp3,
        EscDlStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            esc_dl_status::Comp3,
            esc_dl_status::Comp3,
            EscDlStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EscDlStatus {
    #[inline(always)]
    fn default() -> EscDlStatus {
        <crate::RegValueT<EscDlStatus_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod esc_dl_status {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdiope_SPEC;
    pub type Pdiope = crate::EnumBitfieldStruct<u8, Pdiope_SPEC>;
    impl Pdiope {
        #[doc = "EEPROM not loaded, the PDI not operational (process data RAM is not accessible)"]
        pub const _0: Self = Self::new(0);

        #[doc = "EEPROM loaded correctly, the PDI operational (process data RAM is accessible)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdiwdst_SPEC;
    pub type Pdiwdst = crate::EnumBitfieldStruct<u8, Pdiwdst_SPEC>;
    impl Pdiwdst {
        #[doc = "Timeout of the watchdog timer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Watchdog timer reloaded"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enhlinkd_SPEC;
    pub type Enhlinkd = crate::EnumBitfieldStruct<u8, Enhlinkd_SPEC>;
    impl Enhlinkd {
        #[doc = "Deactivated for all ports"]
        pub const _0: Self = Self::new(0);

        #[doc = "Activated for at least one port"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phyp0_SPEC;
    pub type Phyp0 = crate::EnumBitfieldStruct<u8, Phyp0_SPEC>;
    impl Phyp0 {
        #[doc = "No link"]
        pub const _0: Self = Self::new(0);

        #[doc = "Link detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phyp1_SPEC;
    pub type Phyp1 = crate::EnumBitfieldStruct<u8, Phyp1_SPEC>;
    impl Phyp1 {
        #[doc = "No link"]
        pub const _0: Self = Self::new(0);

        #[doc = "Link detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phyp2_SPEC;
    pub type Phyp2 = crate::EnumBitfieldStruct<u8, Phyp2_SPEC>;
    impl Phyp2 {
        #[doc = "No link"]
        pub const _0: Self = Self::new(0);

        #[doc = "Link detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phyp3_SPEC;
    pub type Phyp3 = crate::EnumBitfieldStruct<u8, Phyp3_SPEC>;
    impl Phyp3 {
        #[doc = "No link"]
        pub const _0: Self = Self::new(0);

        #[doc = "Link detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp0_SPEC;
    pub type Lp0 = crate::EnumBitfieldStruct<u8, Lp0_SPEC>;
    impl Lp0 {
        #[doc = "Open"]
        pub const _0: Self = Self::new(0);

        #[doc = "Closed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Comp0_SPEC;
    pub type Comp0 = crate::EnumBitfieldStruct<u8, Comp0_SPEC>;
    impl Comp0 {
        #[doc = "No stable communication"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication established"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp1_SPEC;
    pub type Lp1 = crate::EnumBitfieldStruct<u8, Lp1_SPEC>;
    impl Lp1 {
        #[doc = "Open"]
        pub const _0: Self = Self::new(0);

        #[doc = "Closed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Comp1_SPEC;
    pub type Comp1 = crate::EnumBitfieldStruct<u8, Comp1_SPEC>;
    impl Comp1 {
        #[doc = "No stable communication"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication established"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp2_SPEC;
    pub type Lp2 = crate::EnumBitfieldStruct<u8, Lp2_SPEC>;
    impl Lp2 {
        #[doc = "Open"]
        pub const _0: Self = Self::new(0);

        #[doc = "Closed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Comp2_SPEC;
    pub type Comp2 = crate::EnumBitfieldStruct<u8, Comp2_SPEC>;
    impl Comp2 {
        #[doc = "No stable communication"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication established"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lp3_SPEC;
    pub type Lp3 = crate::EnumBitfieldStruct<u8, Lp3_SPEC>;
    impl Lp3 {
        #[doc = "Open"]
        pub const _0: Self = Self::new(0);

        #[doc = "Closed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Comp3_SPEC;
    pub type Comp3 = crate::EnumBitfieldStruct<u8, Comp3_SPEC>;
    impl Comp3 {
        #[doc = "No stable communication"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication established"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlControl_SPEC;
impl crate::sealed::RegSpec for AlControl_SPEC {
    type DataType = u16;
}

#[doc = "AL Control Register"]
pub type AlControl = crate::RegValueT<AlControl_SPEC>;

impl AlControl {
    #[doc = "Change the state transition of the device state machine."]
    #[inline(always)]
    pub fn inistate(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        al_control::Inistate,
        al_control::Inistate,
        AlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            al_control::Inistate,
            al_control::Inistate,
            AlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Error Indication Acknowledge (Response)"]
    #[inline(always)]
    pub fn errindack(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        al_control::Errindack,
        al_control::Errindack,
        AlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            al_control::Errindack,
            al_control::Errindack,
            AlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device ID Request"]
    #[inline(always)]
    pub fn deviceid(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        al_control::Deviceid,
        al_control::Deviceid,
        AlControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            al_control::Deviceid,
            al_control::Deviceid,
            AlControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AlControl {
    #[inline(always)]
    fn default() -> AlControl {
        <crate::RegValueT<AlControl_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod al_control {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inistate_SPEC;
    pub type Inistate = crate::EnumBitfieldStruct<u8, Inistate_SPEC>;
    impl Inistate {
        #[doc = "Initial state request"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Pre-operational state request"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Bootstrap state request"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Safe-operational state request"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Operational state request"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errindack_SPEC;
    pub type Errindack = crate::EnumBitfieldStruct<u8, Errindack_SPEC>;
    impl Errindack {
        #[doc = "Error Indication in AL status register is not acknowledged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error Indication in AL status register is acknowledged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deviceid_SPEC;
    pub type Deviceid = crate::EnumBitfieldStruct<u8, Deviceid_SPEC>;
    impl Deviceid {
        #[doc = "No request is present"]
        pub const _0: Self = Self::new(0);

        #[doc = "A request is present"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlStatus_SPEC;
impl crate::sealed::RegSpec for AlStatus_SPEC {
    type DataType = u16;
}

#[doc = "AL Status Register"]
pub type AlStatus = crate::RegValueT<AlStatus_SPEC>;

impl AlStatus {
    #[doc = "State Machine State Indication"]
    #[inline(always)]
    pub fn actstate(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        al_status::Actstate,
        al_status::Actstate,
        AlStatus_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            al_status::Actstate,
            al_status::Actstate,
            AlStatus_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Error State Indication"]
    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        al_status::Err,
        al_status::Err,
        AlStatus_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            al_status::Err,
            al_status::Err,
            AlStatus_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device ID Load State Indication"]
    #[inline(always)]
    pub fn deviceid(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        al_status::Deviceid,
        al_status::Deviceid,
        AlStatus_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            al_status::Deviceid,
            al_status::Deviceid,
            AlStatus_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AlStatus {
    #[inline(always)]
    fn default() -> AlStatus {
        <crate::RegValueT<AlStatus_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod al_status {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actstate_SPEC;
    pub type Actstate = crate::EnumBitfieldStruct<u8, Actstate_SPEC>;
    impl Actstate {
        #[doc = "Initial state"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Pre-operational state"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Request bootstrap state"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Safe-operational state"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Operational state"]
        pub const _0_X_8: Self = Self::new(8);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err_SPEC;
    pub type Err = crate::EnumBitfieldStruct<u8, Err_SPEC>;
    impl Err {
        #[doc = "The device is in the state as requested or flag was cleared by command"]
        pub const _0: Self = Self::new(0);

        #[doc = "The device has not entered the requested state or the state was changed as a result of local action"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deviceid_SPEC;
    pub type Deviceid = crate::EnumBitfieldStruct<u8, Deviceid_SPEC>;
    impl Deviceid {
        #[doc = "Loading failed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loading was successful"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlStatusCode_SPEC;
impl crate::sealed::RegSpec for AlStatusCode_SPEC {
    type DataType = u16;
}

#[doc = "AL Status Code Register"]
pub type AlStatusCode = crate::RegValueT<AlStatusCode_SPEC>;

impl AlStatusCode {
    #[doc = "AL status code"]
    #[inline(always)]
    pub fn statuscode(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, AlStatusCode_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AlStatusCode_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AlStatusCode {
    #[inline(always)]
    fn default() -> AlStatusCode {
        <crate::RegValueT<AlStatusCode_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RunLedOverride_SPEC;
impl crate::sealed::RegSpec for RunLedOverride_SPEC {
    type DataType = u8;
}

#[doc = "RUN LED Override Register"]
pub type RunLedOverride = crate::RegValueT<RunLedOverride_SPEC>;

impl RunLedOverride {
    #[doc = "LED Code Indication (FSM state: Bits \\[3:0\\] of the AL Status register, AL_STATUS)"]
    #[inline(always)]
    pub fn ledcode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        run_led_override::Ledcode,
        run_led_override::Ledcode,
        RunLedOverride_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            run_led_override::Ledcode,
            run_led_override::Ledcode,
            RunLedOverride_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Override Setting"]
    #[inline(always)]
    pub fn overrideen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        run_led_override::Overrideen,
        run_led_override::Overrideen,
        RunLedOverride_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            run_led_override::Overrideen,
            run_led_override::Overrideen,
            RunLedOverride_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RunLedOverride {
    #[inline(always)]
    fn default() -> RunLedOverride {
        <crate::RegValueT<RunLedOverride_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod run_led_override {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ledcode_SPEC;
    pub type Ledcode = crate::EnumBitfieldStruct<u8, Ledcode_SPEC>;
    impl Ledcode {
        #[doc = "Off (FSM: 1-Init)"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Blinking (FSM: 2-PreOp)"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "Flickering (FSM: 3-Bootstrap)"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "On (FSM: 8-Op)"]
        pub const _0_X_F: Self = Self::new(15);

        #[doc = "Flash 1x - 12x (FSM: 4-SafeOp 1x)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Overrideen_SPEC;
    pub type Overrideen = crate::EnumBitfieldStruct<u8, Overrideen_SPEC>;
    impl Overrideen {
        #[doc = "Override disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Override enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrLedOverride_SPEC;
impl crate::sealed::RegSpec for ErrLedOverride_SPEC {
    type DataType = u8;
}

#[doc = "ERR LED Override Register"]
pub type ErrLedOverride = crate::RegValueT<ErrLedOverride_SPEC>;

impl ErrLedOverride {
    #[doc = "LED Code Indication"]
    #[inline(always)]
    pub fn ledcode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        err_led_override::Ledcode,
        err_led_override::Ledcode,
        ErrLedOverride_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            err_led_override::Ledcode,
            err_led_override::Ledcode,
            ErrLedOverride_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Override Setting"]
    #[inline(always)]
    pub fn overrideen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        err_led_override::Overrideen,
        err_led_override::Overrideen,
        ErrLedOverride_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            err_led_override::Overrideen,
            err_led_override::Overrideen,
            ErrLedOverride_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ErrLedOverride {
    #[inline(always)]
    fn default() -> ErrLedOverride {
        <crate::RegValueT<ErrLedOverride_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod err_led_override {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ledcode_SPEC;
    pub type Ledcode = crate::EnumBitfieldStruct<u8, Ledcode_SPEC>;
    impl Ledcode {
        #[doc = "Off"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Blinking"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "Flickering"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "On"]
        pub const _0_X_F: Self = Self::new(15);

        #[doc = "Flash 1x to 12x"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Overrideen_SPEC;
    pub type Overrideen = crate::EnumBitfieldStruct<u8, Overrideen_SPEC>;
    impl Overrideen {
        #[doc = "Override disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Override enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiControl_SPEC;
impl crate::sealed::RegSpec for PdiControl_SPEC {
    type DataType = u8;
}

#[doc = "PDI Control Register"]
pub type PdiControl = crate::RegValueT<PdiControl_SPEC>;

impl PdiControl {
    #[doc = "Process Data Interface. In this LSI, the following value is indicated."]
    #[inline(always)]
    pub fn pdi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        pdi_control::Pdi,
        pdi_control::Pdi,
        PdiControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            pdi_control::Pdi,
            pdi_control::Pdi,
            PdiControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PdiControl {
    #[inline(always)]
    fn default() -> PdiControl {
        <crate::RegValueT<PdiControl_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod pdi_control {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdi_SPEC;
    pub type Pdi = crate::EnumBitfieldStruct<u8, Pdi_SPEC>;
    impl Pdi {
        #[doc = "On-chip bus"]
        pub const _0_X_80: Self = Self::new(128);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscConfig_SPEC;
impl crate::sealed::RegSpec for EscConfig_SPEC {
    type DataType = u8;
}

#[doc = "ESC Configuration Register"]
pub type EscConfig = crate::RegValueT<EscConfig_SPEC>;

impl EscConfig {
    #[doc = "Device emulation (control of AL status)"]
    #[inline(always)]
    pub fn devemu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        esc_config::Devemu,
        esc_config::Devemu,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            esc_config::Devemu,
            esc_config::Devemu,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sets enhanced link detection for all ports"]
    #[inline(always)]
    pub fn enlallp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        esc_config::Enlallp,
        esc_config::Enlallp,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            esc_config::Enlallp,
            esc_config::Enlallp,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sets the SYNC output unit for distributed clocks (fixed to 1 in this LSI)"]
    #[inline(always)]
    pub fn dcsync(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        esc_config::Dcsync,
        esc_config::Dcsync,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            esc_config::Dcsync,
            esc_config::Dcsync,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sets the latch input unit for distributed clocks"]
    #[inline(always)]
    pub fn dclatch(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        esc_config::Dclatch,
        esc_config::Dclatch,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            esc_config::Dclatch,
            esc_config::Dclatch,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 0 Enhanced Link Detection Setting"]
    #[inline(always)]
    pub fn enlp0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        esc_config::Enlp0,
        esc_config::Enlp0,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            esc_config::Enlp0,
            esc_config::Enlp0,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 1 Enhanced Link Detection Setting"]
    #[inline(always)]
    pub fn enlp1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        esc_config::Enlp1,
        esc_config::Enlp1,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            esc_config::Enlp1,
            esc_config::Enlp1,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 2 Enhanced Link Detection Setting"]
    #[inline(always)]
    pub fn enlp2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        esc_config::Enlp2,
        esc_config::Enlp2,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            esc_config::Enlp2,
            esc_config::Enlp2,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Port 3 Enhanced Link Detection Setting"]
    #[inline(always)]
    pub fn enlp3(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        esc_config::Enlp3,
        esc_config::Enlp3,
        EscConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            esc_config::Enlp3,
            esc_config::Enlp3,
            EscConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EscConfig {
    #[inline(always)]
    fn default() -> EscConfig {
        <crate::RegValueT<EscConfig_SPEC> as RegisterValue<_>>::new(12)
    }
}
pub mod esc_config {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Devemu_SPEC;
    pub type Devemu = crate::EnumBitfieldStruct<u8, Devemu_SPEC>;
    impl Devemu {
        #[doc = "The AL status register must be set by the PDI"]
        pub const _0: Self = Self::new(0);

        #[doc = "The AL status register is set to a value written to the AL control register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enlallp_SPEC;
    pub type Enlallp = crate::EnumBitfieldStruct<u8, Enlallp_SPEC>;
    impl Enlallp {
        #[doc = "Disabled (if bits 15 to 12 of address 0 in the EEPROM = 0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled at all ports"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcsync_SPEC;
    pub type Dcsync = crate::EnumBitfieldStruct<u8, Dcsync_SPEC>;
    impl Dcsync {
        #[doc = "Disabled (power saving)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dclatch_SPEC;
    pub type Dclatch = crate::EnumBitfieldStruct<u8, Dclatch_SPEC>;
    impl Dclatch {
        #[doc = "Disabled (power saving)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enlp0_SPEC;
    pub type Enlp0 = crate::EnumBitfieldStruct<u8, Enlp0_SPEC>;
    impl Enlp0 {
        #[doc = "Disabled (if bit 9 of address 0 in the EEPROM = 0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enlp1_SPEC;
    pub type Enlp1 = crate::EnumBitfieldStruct<u8, Enlp1_SPEC>;
    impl Enlp1 {
        #[doc = "Disabled (if bit 9 of address 0 in the EEPROM = 0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enlp2_SPEC;
    pub type Enlp2 = crate::EnumBitfieldStruct<u8, Enlp2_SPEC>;
    impl Enlp2 {
        #[doc = "Disabled (if bit 9 of address 0 in the EEPROM = 0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enlp3_SPEC;
    pub type Enlp3 = crate::EnumBitfieldStruct<u8, Enlp3_SPEC>;
    impl Enlp3 {
        #[doc = "Disabled (if bit 9 of address 0 in the EEPROM = 0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiConfig_SPEC;
impl crate::sealed::RegSpec for PdiConfig_SPEC {
    type DataType = u8;
}

#[doc = "PDI Configuration Register"]
pub type PdiConfig = crate::RegValueT<PdiConfig_SPEC>;

impl PdiConfig {
    #[doc = "On-Chip Bus Clock Indication"]
    #[inline(always)]
    pub fn onchipbusclk(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, PdiConfig_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,PdiConfig_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "On-Chip Bus Type Indication"]
    #[inline(always)]
    pub fn onchipbus(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, PdiConfig_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,PdiConfig_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PdiConfig {
    #[inline(always)]
    fn default() -> PdiConfig {
        <crate::RegValueT<PdiConfig_SPEC> as RegisterValue<_>>::new(68)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyncLatchConfig_SPEC;
impl crate::sealed::RegSpec for SyncLatchConfig_SPEC {
    type DataType = u8;
}

#[doc = "SYNC/LATCH PDI Configuration Register"]
pub type SyncLatchConfig = crate::RegValueT<SyncLatchConfig_SPEC>;

impl SyncLatchConfig {
    #[doc = "SYNC0 Output Driver and Polarity Indication"]
    #[inline(always)]
    pub fn sync0out(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SyncLatchConfig_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SyncLatchConfig_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SYNC0/LATCH0 Indication"]
    #[inline(always)]
    pub fn synclat0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sync_latch_config::Synclat0,
        sync_latch_config::Synclat0,
        SyncLatchConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sync_latch_config::Synclat0,
            sync_latch_config::Synclat0,
            SyncLatchConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SYNC0 State Mapping Indication"]
    #[inline(always)]
    pub fn sync0map(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sync_latch_config::Sync0Map,
        sync_latch_config::Sync0Map,
        SyncLatchConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sync_latch_config::Sync0Map,
            sync_latch_config::Sync0Map,
            SyncLatchConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SYNC1 Output Driver and Polarity Indication"]
    #[inline(always)]
    pub fn sync1out(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, SyncLatchConfig_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,SyncLatchConfig_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SYNC1/LATCH1 Indication"]
    #[inline(always)]
    pub fn synclat1(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sync_latch_config::Synclat1,
        sync_latch_config::Synclat1,
        SyncLatchConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sync_latch_config::Synclat1,
            sync_latch_config::Synclat1,
            SyncLatchConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SYNC1 State Mapping Indication"]
    #[inline(always)]
    pub fn sync1map(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sync_latch_config::Sync1Map,
        sync_latch_config::Sync1Map,
        SyncLatchConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sync_latch_config::Sync1Map,
            sync_latch_config::Sync1Map,
            SyncLatchConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SyncLatchConfig {
    #[inline(always)]
    fn default() -> SyncLatchConfig {
        <crate::RegValueT<SyncLatchConfig_SPEC> as RegisterValue<_>>::new(238)
    }
}
pub mod sync_latch_config {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synclat0_SPEC;
    pub type Synclat0 = crate::EnumBitfieldStruct<u8, Synclat0_SPEC>;
    impl Synclat0 {
        #[doc = "LATCH0 input"]
        pub const _0: Self = Self::new(0);

        #[doc = "SYNC0 output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync0Map_SPEC;
    pub type Sync0Map = crate::EnumBitfieldStruct<u8, Sync0Map_SPEC>;
    impl Sync0Map {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synclat1_SPEC;
    pub type Synclat1 = crate::EnumBitfieldStruct<u8, Synclat1_SPEC>;
    impl Synclat1 {
        #[doc = "LATCH1 input"]
        pub const _0: Self = Self::new(0);

        #[doc = "SYNC1 output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync1Map_SPEC;
    pub type Sync1Map = crate::EnumBitfieldStruct<u8, Sync1Map_SPEC>;
    impl Sync1Map {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtPdiConfig_SPEC;
impl crate::sealed::RegSpec for ExtPdiConfig_SPEC {
    type DataType = u16;
}

#[doc = "Extended PDI Configuration Register"]
pub type ExtPdiConfig = crate::RegValueT<ExtPdiConfig_SPEC>;

impl ExtPdiConfig {
    #[doc = "PDI Data Bus Width Indication"]
    #[inline(always)]
    pub fn databuswid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ext_pdi_config::Databuswid,
        ext_pdi_config::Databuswid,
        ExtPdiConfig_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ext_pdi_config::Databuswid,
            ext_pdi_config::Databuswid,
            ExtPdiConfig_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ExtPdiConfig {
    #[inline(always)]
    fn default() -> ExtPdiConfig {
        <crate::RegValueT<ExtPdiConfig_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ext_pdi_config {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Databuswid_SPEC;
    pub type Databuswid = crate::EnumBitfieldStruct<u8, Databuswid_SPEC>;
    impl Databuswid {
        #[doc = "4 bytes"]
        pub const _00: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _01: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEventMask_SPEC;
impl crate::sealed::RegSpec for EcatEventMask_SPEC {
    type DataType = u16;
}

#[doc = "ECAT Event Mask Register"]
pub type EcatEventMask = crate::RegValueT<EcatEventMask_SPEC>;

impl EcatEventMask {
    #[doc = "Event Request Mask Setting"]
    #[inline(always)]
    pub fn ecatevmask(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        ecat_event_mask::Ecatevmask,
        ecat_event_mask::Ecatevmask,
        EcatEventMask_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            ecat_event_mask::Ecatevmask,
            ecat_event_mask::Ecatevmask,
            EcatEventMask_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EcatEventMask {
    #[inline(always)]
    fn default() -> EcatEventMask {
        <crate::RegValueT<EcatEventMask_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecat_event_mask {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecatevmask_SPEC;
    pub type Ecatevmask = crate::EnumBitfieldStruct<u8, Ecatevmask_SPEC>;
    impl Ecatevmask {
        #[doc = "The corresponding bit of the ECAT Event Request register (ECAT_EVENT_REQ at 0x0210) is not mapped"]
        pub const _0: Self = Self::new(0);

        #[doc = "The corresponding bit of the ECAT Event Request register is mapped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlEventMask_SPEC;
impl crate::sealed::RegSpec for AlEventMask_SPEC {
    type DataType = u32;
}

#[doc = "AL Event Mask Register"]
pub type AlEventMask = crate::RegValueT<AlEventMask_SPEC>;

impl AlEventMask {
    #[doc = "Event Request Mask Setting"]
    #[inline(always)]
    pub fn alevmask(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        al_event_mask::Alevmask,
        al_event_mask::Alevmask,
        AlEventMask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            al_event_mask::Alevmask,
            al_event_mask::Alevmask,
            AlEventMask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AlEventMask {
    #[inline(always)]
    fn default() -> AlEventMask {
        <crate::RegValueT<AlEventMask_SPEC> as RegisterValue<_>>::new(16776975)
    }
}
pub mod al_event_mask {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alevmask_SPEC;
    pub type Alevmask = crate::EnumBitfieldStruct<u8, Alevmask_SPEC>;
    impl Alevmask {
        #[doc = "The corresponding bit of the AL Event Request register (AL_EVENT_REQ at 0x0220) is not mapped"]
        pub const _0: Self = Self::new(0);

        #[doc = "The corresponding bit of the AL Event Request register is mapped"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatEventReq_SPEC;
impl crate::sealed::RegSpec for EcatEventReq_SPEC {
    type DataType = u16;
}

#[doc = "ECAT Event Request Register"]
pub type EcatEventReq = crate::RegValueT<EcatEventReq_SPEC>;

impl EcatEventReq {
    #[doc = "DC Latch Event State Indication"]
    #[inline(always)]
    pub fn dclatch(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecat_event_req::Dclatch,
        ecat_event_req::Dclatch,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecat_event_req::Dclatch,
            ecat_event_req::Dclatch,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DL Status Event State Indication"]
    #[inline(always)]
    pub fn dlsta(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecat_event_req::Dlsta,
        ecat_event_req::Dlsta,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecat_event_req::Dlsta,
            ecat_event_req::Dlsta,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "AL Status Event State Indication"]
    #[inline(always)]
    pub fn alsta(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ecat_event_req::Alsta,
        ecat_event_req::Alsta,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ecat_event_req::Alsta,
            ecat_event_req::Alsta,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 0 Status Indication"]
    #[inline(always)]
    pub fn smsta0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ecat_event_req::Smsta0,
        ecat_event_req::Smsta0,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ecat_event_req::Smsta0,
            ecat_event_req::Smsta0,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 1 Status Indication"]
    #[inline(always)]
    pub fn smsta1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ecat_event_req::Smsta1,
        ecat_event_req::Smsta1,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ecat_event_req::Smsta1,
            ecat_event_req::Smsta1,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 2 Status Indication"]
    #[inline(always)]
    pub fn smsta2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ecat_event_req::Smsta2,
        ecat_event_req::Smsta2,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ecat_event_req::Smsta2,
            ecat_event_req::Smsta2,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 3 Status Indication"]
    #[inline(always)]
    pub fn smsta3(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ecat_event_req::Smsta3,
        ecat_event_req::Smsta3,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ecat_event_req::Smsta3,
            ecat_event_req::Smsta3,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 4 Status Indication"]
    #[inline(always)]
    pub fn smsta4(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ecat_event_req::Smsta4,
        ecat_event_req::Smsta4,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ecat_event_req::Smsta4,
            ecat_event_req::Smsta4,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 5 Status Indication"]
    #[inline(always)]
    pub fn smsta5(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ecat_event_req::Smsta5,
        ecat_event_req::Smsta5,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ecat_event_req::Smsta5,
            ecat_event_req::Smsta5,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 6 Status Indication"]
    #[inline(always)]
    pub fn smsta6(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ecat_event_req::Smsta6,
        ecat_event_req::Smsta6,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ecat_event_req::Smsta6,
            ecat_event_req::Smsta6,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mirror value of SyncManager 7 Status Indication"]
    #[inline(always)]
    pub fn smsta7(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ecat_event_req::Smsta7,
        ecat_event_req::Smsta7,
        EcatEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ecat_event_req::Smsta7,
            ecat_event_req::Smsta7,
            EcatEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EcatEventReq {
    #[inline(always)]
    fn default() -> EcatEventReq {
        <crate::RegValueT<EcatEventReq_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecat_event_req {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dclatch_SPEC;
    pub type Dclatch = crate::EnumBitfieldStruct<u8, Dclatch_SPEC>;
    impl Dclatch {
        #[doc = "No change on DC latch inputs"]
        pub const _0: Self = Self::new(0);

        #[doc = "At least one change on DC latch inputs"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlsta_SPEC;
    pub type Dlsta = crate::EnumBitfieldStruct<u8, Dlsta_SPEC>;
    impl Dlsta {
        #[doc = "No change in DL status"]
        pub const _0: Self = Self::new(0);

        #[doc = "DL status change"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alsta_SPEC;
    pub type Alsta = crate::EnumBitfieldStruct<u8, Alsta_SPEC>;
    impl Alsta {
        #[doc = "No change in AL status"]
        pub const _0: Self = Self::new(0);

        #[doc = "AL status change"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta0_SPEC;
    pub type Smsta0 = crate::EnumBitfieldStruct<u8, Smsta0_SPEC>;
    impl Smsta0 {
        #[doc = "No Sync channel 0 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 0 event pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta1_SPEC;
    pub type Smsta1 = crate::EnumBitfieldStruct<u8, Smsta1_SPEC>;
    impl Smsta1 {
        #[doc = "No Sync channel 1 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 1 event pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta2_SPEC;
    pub type Smsta2 = crate::EnumBitfieldStruct<u8, Smsta2_SPEC>;
    impl Smsta2 {
        #[doc = "No Sync channel 2 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 2 event pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta3_SPEC;
    pub type Smsta3 = crate::EnumBitfieldStruct<u8, Smsta3_SPEC>;
    impl Smsta3 {
        #[doc = "No Sync channel 3 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 3 event pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta4_SPEC;
    pub type Smsta4 = crate::EnumBitfieldStruct<u8, Smsta4_SPEC>;
    impl Smsta4 {
        #[doc = "No Sync channel 4 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 4 event pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta5_SPEC;
    pub type Smsta5 = crate::EnumBitfieldStruct<u8, Smsta5_SPEC>;
    impl Smsta5 {
        #[doc = "No Sync channel 5 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 5 event pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta6_SPEC;
    pub type Smsta6 = crate::EnumBitfieldStruct<u8, Smsta6_SPEC>;
    impl Smsta6 {
        #[doc = "No Sync channel 6 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 6 event pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smsta7_SPEC;
    pub type Smsta7 = crate::EnumBitfieldStruct<u8, Smsta7_SPEC>;
    impl Smsta7 {
        #[doc = "No Sync channel 7 event"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync channel 7 event pending"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlEventReq_SPEC;
impl crate::sealed::RegSpec for AlEventReq_SPEC {
    type DataType = u32;
}

#[doc = "AL Event Request Register"]
pub type AlEventReq = crate::RegValueT<AlEventReq_SPEC>;

impl AlEventReq {
    #[doc = "AL Control Event State Indication"]
    #[inline(always)]
    pub fn alctrl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        al_event_req::Alctrl,
        al_event_req::Alctrl,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            al_event_req::Alctrl,
            al_event_req::Alctrl,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DC Latch Event State Indication"]
    #[inline(always)]
    pub fn dclatch(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        al_event_req::Dclatch,
        al_event_req::Dclatch,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            al_event_req::Dclatch,
            al_event_req::Dclatch,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DC SYNC0 State Indication"]
    #[inline(always)]
    pub fn dcsync0sta(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AlEventReq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AlEventReq_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DC SYNC1 State Indication"]
    #[inline(always)]
    pub fn dcsync1sta(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AlEventReq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AlEventReq_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SyncManager Activation Indication"]
    #[inline(always)]
    pub fn syncact(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        al_event_req::Syncact,
        al_event_req::Syncact,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            al_event_req::Syncact,
            al_event_req::Syncact,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Watchdog Process Data Indication"]
    #[inline(always)]
    pub fn wdpd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        al_event_req::Wdpd,
        al_event_req::Wdpd,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            al_event_req::Wdpd,
            al_event_req::Wdpd,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 0 interrupt (bit 0 or 1 of the SyncManager status register (0x0805))"]
    #[inline(always)]
    pub fn smint0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        al_event_req::Smint0,
        al_event_req::Smint0,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            al_event_req::Smint0,
            al_event_req::Smint0,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 1 interrupt (bit 0 or 1 of the SyncManager status register (0x080D))"]
    #[inline(always)]
    pub fn smint1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        al_event_req::Smint1,
        al_event_req::Smint1,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            al_event_req::Smint1,
            al_event_req::Smint1,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 2 interrupt (bit 0 or 1 of the SyncManager status register (0x0815))"]
    #[inline(always)]
    pub fn smint2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        al_event_req::Smint2,
        al_event_req::Smint2,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            al_event_req::Smint2,
            al_event_req::Smint2,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 3 interrupt (bit 0 or 1 of the SyncManager status register (0x081D))"]
    #[inline(always)]
    pub fn smint3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        al_event_req::Smint3,
        al_event_req::Smint3,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            al_event_req::Smint3,
            al_event_req::Smint3,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 4 interrupt (bit 0 or 1 of the SyncManager status register (0x0825))"]
    #[inline(always)]
    pub fn smint4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        al_event_req::Smint4,
        al_event_req::Smint4,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            al_event_req::Smint4,
            al_event_req::Smint4,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 5 interrupt (bit 0 or 1 of the SyncManager status register (0x082D))"]
    #[inline(always)]
    pub fn smint5(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        al_event_req::Smint5,
        al_event_req::Smint5,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            al_event_req::Smint5,
            al_event_req::Smint5,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 6 interrupt (bit 0 or 1 of the SyncManager status register (0x0835))"]
    #[inline(always)]
    pub fn smint6(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        al_event_req::Smint6,
        al_event_req::Smint6,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            al_event_req::Smint6,
            al_event_req::Smint6,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SyncManager 7 interrupt (bit 0 or 1 of the SyncManager status register (0x083D))"]
    #[inline(always)]
    pub fn smint7(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        al_event_req::Smint7,
        al_event_req::Smint7,
        AlEventReq_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            al_event_req::Smint7,
            al_event_req::Smint7,
            AlEventReq_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AlEventReq {
    #[inline(always)]
    fn default() -> AlEventReq {
        <crate::RegValueT<AlEventReq_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod al_event_req {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alctrl_SPEC;
    pub type Alctrl = crate::EnumBitfieldStruct<u8, Alctrl_SPEC>;
    impl Alctrl {
        #[doc = "No change in the AL control register"]
        pub const _0: Self = Self::new(0);

        #[doc = "The AL control register has been written"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dclatch_SPEC;
    pub type Dclatch = crate::EnumBitfieldStruct<u8, Dclatch_SPEC>;
    impl Dclatch {
        #[doc = "No change on DC latch inputs"]
        pub const _0: Self = Self::new(0);

        #[doc = "At least one change on DC latch inputs"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncact_SPEC;
    pub type Syncact = crate::EnumBitfieldStruct<u8, Syncact_SPEC>;
    impl Syncact {
        #[doc = "No change in any SyncManager"]
        pub const _0: Self = Self::new(0);

        #[doc = "At least one SyncManager changed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdpd_SPEC;
    pub type Wdpd = crate::EnumBitfieldStruct<u8, Wdpd_SPEC>;
    impl Wdpd {
        #[doc = "Valid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timeout"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint0_SPEC;
    pub type Smint0 = crate::EnumBitfieldStruct<u8, Smint0_SPEC>;
    impl Smint0 {
        #[doc = "No SyncManager 0 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 0 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint1_SPEC;
    pub type Smint1 = crate::EnumBitfieldStruct<u8, Smint1_SPEC>;
    impl Smint1 {
        #[doc = "No SyncManager 1 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 1 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint2_SPEC;
    pub type Smint2 = crate::EnumBitfieldStruct<u8, Smint2_SPEC>;
    impl Smint2 {
        #[doc = "No SyncManager 2 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 2 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint3_SPEC;
    pub type Smint3 = crate::EnumBitfieldStruct<u8, Smint3_SPEC>;
    impl Smint3 {
        #[doc = "No SyncManager 3 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 3 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint4_SPEC;
    pub type Smint4 = crate::EnumBitfieldStruct<u8, Smint4_SPEC>;
    impl Smint4 {
        #[doc = "No SyncManager 4 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 4 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint5_SPEC;
    pub type Smint5 = crate::EnumBitfieldStruct<u8, Smint5_SPEC>;
    impl Smint5 {
        #[doc = "No SyncManager 5 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 5 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint6_SPEC;
    pub type Smint6 = crate::EnumBitfieldStruct<u8, Smint6_SPEC>;
    impl Smint6 {
        #[doc = "No SyncManager 6 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 6 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smint7_SPEC;
    pub type Smint7 = crate::EnumBitfieldStruct<u8, Smint7_SPEC>;
    impl Smint7 {
        #[doc = "No SyncManager 7 interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "SyncManager 7 interrupt pending"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxErrCount_SPEC;
impl crate::sealed::RegSpec for RxErrCount_SPEC {
    type DataType = u16;
}

#[doc = "RX Error Counter %s Register (n = 0 to 1)"]
pub type RxErrCount = crate::RegValueT<RxErrCount_SPEC>;

impl RxErrCount {
    #[doc = "Invalid Frame Counter Value Indication"]
    #[inline(always)]
    pub fn invfrmcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RxErrCount_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,RxErrCount_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "RX Frame Error Counter Value Indication"]
    #[inline(always)]
    pub fn rxerrcnt(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RxErrCount_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,RxErrCount_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RxErrCount {
    #[inline(always)]
    fn default() -> RxErrCount {
        <crate::RegValueT<RxErrCount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FwdRxErrCount_SPEC;
impl crate::sealed::RegSpec for FwdRxErrCount_SPEC {
    type DataType = u8;
}

#[doc = "Forwarded RX Error Counter %s Register (n = 0 to 1)"]
pub type FwdRxErrCount = crate::RegValueT<FwdRxErrCount_SPEC>;

impl FwdRxErrCount {
    #[doc = "Forwarded Error Counter Value Indication"]
    #[inline(always)]
    pub fn fwderrcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, FwdRxErrCount_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,FwdRxErrCount_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FwdRxErrCount {
    #[inline(always)]
    fn default() -> FwdRxErrCount {
        <crate::RegValueT<FwdRxErrCount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatProcErrCount_SPEC;
impl crate::sealed::RegSpec for EcatProcErrCount_SPEC {
    type DataType = u8;
}

#[doc = "ECAT Processing Unit Error Counter Register"]
pub type EcatProcErrCount = crate::RegValueT<EcatProcErrCount_SPEC>;

impl EcatProcErrCount {
    #[doc = "Processing Unit Error Counter Value Indication"]
    #[inline(always)]
    pub fn epuerrcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, EcatProcErrCount_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,EcatProcErrCount_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EcatProcErrCount {
    #[inline(always)]
    fn default() -> EcatProcErrCount {
        <crate::RegValueT<EcatProcErrCount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdiErrCount_SPEC;
impl crate::sealed::RegSpec for PdiErrCount_SPEC {
    type DataType = u8;
}

#[doc = "PDI Error Counter Register"]
pub type PdiErrCount = crate::RegValueT<PdiErrCount_SPEC>;

impl PdiErrCount {
    #[doc = "PDI Error Counter Value Indication"]
    #[inline(always)]
    pub fn pdierrcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, PdiErrCount_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,PdiErrCount_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PdiErrCount {
    #[inline(always)]
    fn default() -> PdiErrCount {
        <crate::RegValueT<PdiErrCount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LostLinkCount_SPEC;
impl crate::sealed::RegSpec for LostLinkCount_SPEC {
    type DataType = u8;
}

#[doc = "Lost Link Counter %s Register (n = 0 to 1)"]
pub type LostLinkCount = crate::RegValueT<LostLinkCount_SPEC>;

impl LostLinkCount {
    #[doc = "Lost Link Counter Value Indication"]
    #[inline(always)]
    pub fn lostlinkcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, LostLinkCount_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,LostLinkCount_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LostLinkCount {
    #[inline(always)]
    fn default() -> LostLinkCount {
        <crate::RegValueT<LostLinkCount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdDivide_SPEC;
impl crate::sealed::RegSpec for WdDivide_SPEC {
    type DataType = u16;
}

#[doc = "Watchdog Divider Register"]
pub type WdDivide = crate::RegValueT<WdDivide_SPEC>;

impl WdDivide {
    #[doc = "Watchdog Clock Frequency Divisor Setting"]
    #[inline(always)]
    pub fn wddiv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, WdDivide_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,WdDivide_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WdDivide {
    #[inline(always)]
    fn default() -> WdDivide {
        <crate::RegValueT<WdDivide_SPEC> as RegisterValue<_>>::new(2498)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtPdi_SPEC;
impl crate::sealed::RegSpec for WdtPdi_SPEC {
    type DataType = u16;
}

#[doc = "Watchdog Time PDI Register"]
pub type WdtPdi = crate::RegValueT<WdtPdi_SPEC>;

impl WdtPdi {
    #[doc = "Watchdog Overflow Time Setting"]
    #[inline(always)]
    pub fn wdtimpdi(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, WdtPdi_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,WdtPdi_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WdtPdi {
    #[inline(always)]
    fn default() -> WdtPdi {
        <crate::RegValueT<WdtPdi_SPEC> as RegisterValue<_>>::new(1000)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtData_SPEC;
impl crate::sealed::RegSpec for WdtData_SPEC {
    type DataType = u16;
}

#[doc = "Watchdog Time Process Data Register"]
pub type WdtData = crate::RegValueT<WdtData_SPEC>;

impl WdtData {
    #[doc = "Watchdog Overflow Time Setting"]
    #[inline(always)]
    pub fn wdtimpd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, WdtData_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,WdtData_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WdtData {
    #[inline(always)]
    fn default() -> WdtData {
        <crate::RegValueT<WdtData_SPEC> as RegisterValue<_>>::new(1000)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdsData_SPEC;
impl crate::sealed::RegSpec for WdsData_SPEC {
    type DataType = u16;
}

#[doc = "Watchdog Status Process Data Register"]
pub type WdsData = crate::RegValueT<WdsData_SPEC>;

impl WdsData {
    #[doc = "Watchdog State Indication"]
    #[inline(always)]
    pub fn wdstapd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wds_data::Wdstapd,
        wds_data::Wdstapd,
        WdsData_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wds_data::Wdstapd,
            wds_data::Wdstapd,
            WdsData_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WdsData {
    #[inline(always)]
    fn default() -> WdsData {
        <crate::RegValueT<WdsData_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wds_data {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdstapd_SPEC;
    pub type Wdstapd = crate::EnumBitfieldStruct<u8, Wdstapd_SPEC>;
    impl Wdstapd {
        #[doc = "The timeout period of the process data watchdog timer elapses"]
        pub const _0: Self = Self::new(0);

        #[doc = "The process data watchdog timer is active or disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdcData_SPEC;
impl crate::sealed::RegSpec for WdcData_SPEC {
    type DataType = u8;
}

#[doc = "Watchdog Counter Process Data Register"]
pub type WdcData = crate::RegValueT<WdcData_SPEC>;

impl WdcData {
    #[doc = "Watchdog Counter Value Indication"]
    #[inline(always)]
    pub fn wdcntpd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WdcData_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WdcData_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WdcData {
    #[inline(always)]
    fn default() -> WdcData {
        <crate::RegValueT<WdcData_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdcPdi_SPEC;
impl crate::sealed::RegSpec for WdcPdi_SPEC {
    type DataType = u8;
}

#[doc = "Watchdog Counter PDI Register"]
pub type WdcPdi = crate::RegValueT<WdcPdi_SPEC>;

impl WdcPdi {
    #[doc = "Watchdog Counter Value Indication"]
    #[inline(always)]
    pub fn wdcntpdi(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WdcPdi_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WdcPdi_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WdcPdi {
    #[inline(always)]
    fn default() -> WdcPdi {
        <crate::RegValueT<WdcPdi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepConf_SPEC;
impl crate::sealed::RegSpec for EepConf_SPEC {
    type DataType = u8;
}

#[doc = "EEPROM Configuration Register"]
pub type EepConf = crate::RegValueT<EepConf_SPEC>;

impl EepConf {
    #[doc = "PDI EEPROM Control"]
    #[inline(always)]
    pub fn ctrlpdi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eep_conf::Ctrlpdi,
        eep_conf::Ctrlpdi,
        EepConf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eep_conf::Ctrlpdi,
            eep_conf::Ctrlpdi,
            EepConf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EEPROM Access Right Change"]
    #[inline(always)]
    pub fn forceecat(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eep_conf::Forceecat,
        eep_conf::Forceecat,
        EepConf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eep_conf::Forceecat,
            eep_conf::Forceecat,
            EepConf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EepConf {
    #[inline(always)]
    fn default() -> EepConf {
        <crate::RegValueT<EepConf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eep_conf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrlpdi_SPEC;
    pub type Ctrlpdi = crate::EnumBitfieldStruct<u8, Ctrlpdi_SPEC>;
    impl Ctrlpdi {
        #[doc = "The PDI has no EEPROM control"]
        pub const _0: Self = Self::new(0);

        #[doc = "The PDI has EEPROM control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Forceecat_SPEC;
    pub type Forceecat = crate::EnumBitfieldStruct<u8, Forceecat_SPEC>;
    impl Forceecat {
        #[doc = "No change"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset bit 0 of the EEPROM PDI Access State register (EEP_STATE at 0x0501) to 0. That is, prohibit access to the EEPROM by the PDI."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepState_SPEC;
impl crate::sealed::RegSpec for EepState_SPEC {
    type DataType = u8;
}

#[doc = "EEPROM PDI Access State Register"]
pub type EepState = crate::RegValueT<EepState_SPEC>;

impl EepState {
    #[doc = "EEPROM Access Right Setting"]
    #[inline(always)]
    pub fn pdiaccess(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eep_state::Pdiaccess,
        eep_state::Pdiaccess,
        EepState_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eep_state::Pdiaccess,
            eep_state::Pdiaccess,
            EepState_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EepState {
    #[inline(always)]
    fn default() -> EepState {
        <crate::RegValueT<EepState_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eep_state {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdiaccess_SPEC;
    pub type Pdiaccess = crate::EnumBitfieldStruct<u8, Pdiaccess_SPEC>;
    impl Pdiaccess {
        #[doc = "Prohibits the PDI from access to the EEPROM"]
        pub const _0: Self = Self::new(0);

        #[doc = "The PDI has access to the EEPROM"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepContStat_SPEC;
impl crate::sealed::RegSpec for EepContStat_SPEC {
    type DataType = u16;
}

#[doc = "EEPROM Control/Status Register"]
pub type EepContStat = crate::RegValueT<EepContStat_SPEC>;

impl EepContStat {
    #[doc = "ECAT Write Enable"]
    #[inline(always)]
    pub fn ecatwren(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eep_cont_stat::Ecatwren,
        eep_cont_stat::Ecatwren,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eep_cont_stat::Ecatwren,
            eep_cont_stat::Ecatwren,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EEPROM Read Byte Indication"]
    #[inline(always)]
    pub fn readbyte(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eep_cont_stat::Readbyte,
        eep_cont_stat::Readbyte,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eep_cont_stat::Readbyte,
            eep_cont_stat::Readbyte,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EEPROM Algorithm Indication"]
    #[inline(always)]
    pub fn promsize(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eep_cont_stat::Promsize,
        eep_cont_stat::Promsize,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eep_cont_stat::Promsize,
            eep_cont_stat::Promsize,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Command"]
    #[inline(always)]
    pub fn command(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        eep_cont_stat::Command,
        eep_cont_stat::Command,
        EepContStat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            eep_cont_stat::Command,
            eep_cont_stat::Command,
            EepContStat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Checksum Error Indication"]
    #[inline(always)]
    pub fn cksumerr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        eep_cont_stat::Cksumerr,
        eep_cont_stat::Cksumerr,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            eep_cont_stat::Cksumerr,
            eep_cont_stat::Cksumerr,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EEPROM Loading Status Indication"]
    #[inline(always)]
    pub fn loadsta(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        eep_cont_stat::Loadsta,
        eep_cont_stat::Loadsta,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            eep_cont_stat::Loadsta,
            eep_cont_stat::Loadsta,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Acknowledge/Command Error Indication"]
    #[inline(always)]
    pub fn ackcmderr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        eep_cont_stat::Ackcmderr,
        eep_cont_stat::Ackcmderr,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            eep_cont_stat::Ackcmderr,
            eep_cont_stat::Ackcmderr,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Write Enable Error Indication"]
    #[inline(always)]
    pub fn wrenerr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        eep_cont_stat::Wrenerr,
        eep_cont_stat::Wrenerr,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            eep_cont_stat::Wrenerr,
            eep_cont_stat::Wrenerr,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "EEPROM Interface State Indication"]
    #[inline(always)]
    pub fn busy(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        eep_cont_stat::Busy,
        eep_cont_stat::Busy,
        EepContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            eep_cont_stat::Busy,
            eep_cont_stat::Busy,
            EepContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EepContStat {
    #[inline(always)]
    fn default() -> EepContStat {
        <crate::RegValueT<EepContStat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eep_cont_stat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecatwren_SPEC;
    pub type Ecatwren = crate::EnumBitfieldStruct<u8, Ecatwren_SPEC>;
    impl Ecatwren {
        #[doc = "Write requests are disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write requests are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Readbyte_SPEC;
    pub type Readbyte = crate::EnumBitfieldStruct<u8, Readbyte_SPEC>;
    impl Readbyte {
        #[doc = "4 bytes"]
        pub const _0: Self = Self::new(0);

        #[doc = "8 bytes"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Promsize_SPEC;
    pub type Promsize = crate::EnumBitfieldStruct<u8, Promsize_SPEC>;
    impl Promsize {
        #[doc = "1 address byte (1-Kbit to 16-Kbit EEPROMs)"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 address bytes (32-Kbit to 4-Mbit EEPROMs)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Command_SPEC;
    pub type Command = crate::EnumBitfieldStruct<u8, Command_SPEC>;
    impl Command {
        #[doc = "No command/EEPROM idle (clear error bits)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Read"]
        pub const _001: Self = Self::new(1);

        #[doc = "Write"]
        pub const _010: Self = Self::new(2);

        #[doc = "Reload"]
        pub const _100: Self = Self::new(4);

        #[doc = "Reserved/invalid commands (must not be issued)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksumerr_SPEC;
    pub type Cksumerr = crate::EnumBitfieldStruct<u8, Cksumerr_SPEC>;
    impl Cksumerr {
        #[doc = "No error in the checksum"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error in the checksum"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Loadsta_SPEC;
    pub type Loadsta = crate::EnumBitfieldStruct<u8, Loadsta_SPEC>;
    impl Loadsta {
        #[doc = "EEPROM has been loaded and device information has no problem"]
        pub const _0: Self = Self::new(0);

        #[doc = "EEPROM has not been loaded and device information is not available (EEPROM loading in progress or finished with a failure)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackcmderr_SPEC;
    pub type Ackcmderr = crate::EnumBitfieldStruct<u8, Ackcmderr_SPEC>;
    impl Ackcmderr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Missing EEPROM acknowledge or invalid command"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrenerr_SPEC;
    pub type Wrenerr = crate::EnumBitfieldStruct<u8, Wrenerr_SPEC>;
    impl Wrenerr {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write command without write enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busy_SPEC;
    pub type Busy = crate::EnumBitfieldStruct<u8, Busy_SPEC>;
    impl Busy {
        #[doc = "The EEPROM interface is idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "The EEPROM interface is busy"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepAdr_SPEC;
impl crate::sealed::RegSpec for EepAdr_SPEC {
    type DataType = u32;
}

#[doc = "EEPROM Address Register"]
pub type EepAdr = crate::RegValueT<EepAdr_SPEC>;

impl EepAdr {
    #[doc = "EEPROM Address Setting"]
    #[inline(always)]
    pub fn address(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, EepAdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,EepAdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EepAdr {
    #[inline(always)]
    fn default() -> EepAdr {
        <crate::RegValueT<EepAdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EepData_SPEC;
impl crate::sealed::RegSpec for EepData_SPEC {
    type DataType = u32;
}

#[doc = "EEPROM Data Register"]
pub type EepData = crate::RegValueT<EepData_SPEC>;

impl EepData {
    #[doc = "Data to be written to the EEPROM or data read from the EEPROM (lower 2 bytes)"]
    #[inline(always)]
    pub fn lodata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, EepData_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,EepData_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data read from the EEPROM (upper 2 bytes)"]
    #[inline(always)]
    pub fn hidata(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, EepData_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,EepData_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EepData {
    #[inline(always)]
    fn default() -> EepData {
        <crate::RegValueT<EepData_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiContStat_SPEC;
impl crate::sealed::RegSpec for MiiContStat_SPEC {
    type DataType = u16;
}

#[doc = "MII Management Control/Status Register"]
pub type MiiContStat = crate::RegValueT<MiiContStat_SPEC>;

impl MiiContStat {
    #[doc = "Write Enable"]
    #[inline(always)]
    pub fn wren(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mii_cont_stat::Wren,
        mii_cont_stat::Wren,
        MiiContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mii_cont_stat::Wren,
            mii_cont_stat::Wren,
            MiiContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PDI Control Indication"]
    #[inline(always)]
    pub fn pdictrl(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mii_cont_stat::Pdictrl,
        mii_cont_stat::Pdictrl,
        MiiContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mii_cont_stat::Pdictrl,
            mii_cont_stat::Pdictrl,
            MiiContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MI Link Detection"]
    #[inline(always)]
    pub fn milink(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mii_cont_stat::Milink,
        mii_cont_stat::Milink,
        MiiContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mii_cont_stat::Milink,
            mii_cont_stat::Milink,
            MiiContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PHY Address Offset Indication"]
    #[inline(always)]
    pub fn phyoffset(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, MiiContStat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,MiiContStat_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command"]
    #[inline(always)]
    pub fn command(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        mii_cont_stat::Command,
        mii_cont_stat::Command,
        MiiContStat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            mii_cont_stat::Command,
            mii_cont_stat::Command,
            MiiContStat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read Error Indication"]
    #[inline(always)]
    pub fn readerr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mii_cont_stat::Readerr,
        mii_cont_stat::Readerr,
        MiiContStat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mii_cont_stat::Readerr,
            mii_cont_stat::Readerr,
            MiiContStat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Error Indication"]
    #[inline(always)]
    pub fn cmderr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mii_cont_stat::Cmderr,
        mii_cont_stat::Cmderr,
        MiiContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mii_cont_stat::Cmderr,
            mii_cont_stat::Cmderr,
            MiiContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "MII Management State Indication"]
    #[inline(always)]
    pub fn busy(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mii_cont_stat::Busy,
        mii_cont_stat::Busy,
        MiiContStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mii_cont_stat::Busy,
            mii_cont_stat::Busy,
            MiiContStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for MiiContStat {
    #[inline(always)]
    fn default() -> MiiContStat {
        <crate::RegValueT<MiiContStat_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod mii_cont_stat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wren_SPEC;
    pub type Wren = crate::EnumBitfieldStruct<u8, Wren_SPEC>;
    impl Wren {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdictrl_SPEC;
    pub type Pdictrl = crate::EnumBitfieldStruct<u8, Pdictrl_SPEC>;
    impl Pdictrl {
        #[doc = "Only ECAT control"]
        pub const _0: Self = Self::new(0);

        #[doc = "PDI control possible"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Milink_SPEC;
    pub type Milink = crate::EnumBitfieldStruct<u8, Milink_SPEC>;
    impl Milink {
        #[doc = "Not available"]
        pub const _0: Self = Self::new(0);

        #[doc = "Available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Command_SPEC;
    pub type Command = crate::EnumBitfieldStruct<u8, Command_SPEC>;
    impl Command {
        #[doc = "No command/MI idle (clear error bits)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Read"]
        pub const _01: Self = Self::new(1);

        #[doc = "Write"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Readerr_SPEC;
    pub type Readerr = crate::EnumBitfieldStruct<u8, Readerr_SPEC>;
    impl Readerr {
        #[doc = "No read error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read error occurred (PHY or register not available)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmderr_SPEC;
    pub type Cmderr = crate::EnumBitfieldStruct<u8, Cmderr_SPEC>;
    impl Cmderr {
        #[doc = "Last command was successful"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invalid command or write command without write enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busy_SPEC;
    pub type Busy = crate::EnumBitfieldStruct<u8, Busy_SPEC>;
    impl Busy {
        #[doc = "MII management interface is idle"]
        pub const _0: Self = Self::new(0);

        #[doc = "MII management interface is busy"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyAdr_SPEC;
impl crate::sealed::RegSpec for PhyAdr_SPEC {
    type DataType = u8;
}

#[doc = "PHY Address Register"]
pub type PhyAdr = crate::RegValueT<PhyAdr_SPEC>;

impl PhyAdr {
    #[doc = "PHY Address Setting"]
    #[inline(always)]
    pub fn phyaddr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, PhyAdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,PhyAdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PhyAdr {
    #[inline(always)]
    fn default() -> PhyAdr {
        <crate::RegValueT<PhyAdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyRegAdr_SPEC;
impl crate::sealed::RegSpec for PhyRegAdr_SPEC {
    type DataType = u8;
}

#[doc = "PHY Register Address Register"]
pub type PhyRegAdr = crate::RegValueT<PhyRegAdr_SPEC>;

impl PhyRegAdr {
    #[doc = "Address of PHY register"]
    #[inline(always)]
    pub fn phyregaddr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, PhyRegAdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,PhyRegAdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PhyRegAdr {
    #[inline(always)]
    fn default() -> PhyRegAdr {
        <crate::RegValueT<PhyRegAdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyData_SPEC;
impl crate::sealed::RegSpec for PhyData_SPEC {
    type DataType = u16;
}

#[doc = "PHY Data Register"]
pub type PhyData = crate::RegValueT<PhyData_SPEC>;

impl PhyData {
    #[doc = "PHY Register Data Indication/Setting"]
    #[inline(always)]
    pub fn phyregdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, PhyData_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,PhyData_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PhyData {
    #[inline(always)]
    fn default() -> PhyData {
        <crate::RegValueT<PhyData_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiEcatAcsStat_SPEC;
impl crate::sealed::RegSpec for MiiEcatAcsStat_SPEC {
    type DataType = u8;
}

#[doc = "MII Management ECAT Access State Register"]
pub type MiiEcatAcsStat = crate::RegValueT<MiiEcatAcsStat_SPEC>;

impl MiiEcatAcsStat {
    #[doc = "MII Management Interface Access Right Setting"]
    #[inline(always)]
    pub fn acsmii(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mii_ecat_acs_stat::Acsmii,
        mii_ecat_acs_stat::Acsmii,
        MiiEcatAcsStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mii_ecat_acs_stat::Acsmii,
            mii_ecat_acs_stat::Acsmii,
            MiiEcatAcsStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for MiiEcatAcsStat {
    #[inline(always)]
    fn default() -> MiiEcatAcsStat {
        <crate::RegValueT<MiiEcatAcsStat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mii_ecat_acs_stat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acsmii_SPEC;
    pub type Acsmii = crate::EnumBitfieldStruct<u8, Acsmii_SPEC>;
    impl Acsmii {
        #[doc = "Enables access to the MII management interface by the PDI."]
        pub const _0: Self = Self::new(0);

        #[doc = "Exclusive access to the MII management interface by the ECAT"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiiPdiAcsStat_SPEC;
impl crate::sealed::RegSpec for MiiPdiAcsStat_SPEC {
    type DataType = u8;
}

#[doc = "MII Management PDI Access State Register"]
pub type MiiPdiAcsStat = crate::RegValueT<MiiPdiAcsStat_SPEC>;

impl MiiPdiAcsStat {
    #[doc = "Right of access to the MII management interface"]
    #[inline(always)]
    pub fn acsmii(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mii_pdi_acs_stat::Acsmii,
        mii_pdi_acs_stat::Acsmii,
        MiiPdiAcsStat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mii_pdi_acs_stat::Acsmii,
            mii_pdi_acs_stat::Acsmii,
            MiiPdiAcsStat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Forced change of access by the PDI (forced change of bit 0)"]
    #[inline(always)]
    pub fn forpdi(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mii_pdi_acs_stat::Forpdi,
        mii_pdi_acs_stat::Forpdi,
        MiiPdiAcsStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mii_pdi_acs_stat::Forpdi,
            mii_pdi_acs_stat::Forpdi,
            MiiPdiAcsStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for MiiPdiAcsStat {
    #[inline(always)]
    fn default() -> MiiPdiAcsStat {
        <crate::RegValueT<MiiPdiAcsStat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mii_pdi_acs_stat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acsmii_SPEC;
    pub type Acsmii = crate::EnumBitfieldStruct<u8, Acsmii_SPEC>;
    impl Acsmii {
        #[doc = "Access to the MII management interface by the ECAT"]
        pub const _0: Self = Self::new(0);

        #[doc = "Access to the MII management interface by the PDI"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Forpdi_SPEC;
    pub type Forpdi = crate::EnumBitfieldStruct<u8, Forpdi_SPEC>;
    impl Forpdi {
        #[doc = "The value of bit 0 of this register is not changed"]
        pub const _0: Self = Self::new(0);

        #[doc = "The value of bit 0 of this register is reset to 0 (the right of access is changed to the ECAT)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLStartAdr_SPEC;
impl crate::sealed::RegSpec for FmmuLStartAdr_SPEC {
    type DataType = u32;
}

#[doc = "FMMU Logical Start Address %s Register (n = 0 to 7)"]
pub type FmmuLStartAdr = crate::RegValueT<FmmuLStartAdr_SPEC>;

impl FmmuLStartAdr {
    #[doc = "Logical Start Address Setting"]
    #[inline(always)]
    pub fn lstaadr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FmmuLStartAdr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FmmuLStartAdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FmmuLStartAdr {
    #[inline(always)]
    fn default() -> FmmuLStartAdr {
        <crate::RegValueT<FmmuLStartAdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLen_SPEC;
impl crate::sealed::RegSpec for FmmuLen_SPEC {
    type DataType = u16;
}

#[doc = "FMMU Length %s Register (n = 0 to 7)"]
pub type FmmuLen = crate::RegValueT<FmmuLen_SPEC>;

impl FmmuLen {
    #[doc = "Area Size Specification"]
    #[inline(always)]
    pub fn fmmulen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, FmmuLen_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,FmmuLen_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FmmuLen {
    #[inline(always)]
    fn default() -> FmmuLen {
        <crate::RegValueT<FmmuLen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLStartBit_SPEC;
impl crate::sealed::RegSpec for FmmuLStartBit_SPEC {
    type DataType = u8;
}

#[doc = "FMMU Logical Start Bit %s Register (n = 0 to 7)"]
pub type FmmuLStartBit = crate::RegValueT<FmmuLStartBit_SPEC>;

impl FmmuLStartBit {
    #[doc = "Start Bit Setting"]
    #[inline(always)]
    pub fn lstabit(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, FmmuLStartBit_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,FmmuLStartBit_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FmmuLStartBit {
    #[inline(always)]
    fn default() -> FmmuLStartBit {
        <crate::RegValueT<FmmuLStartBit_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuLStopBit_SPEC;
impl crate::sealed::RegSpec for FmmuLStopBit_SPEC {
    type DataType = u8;
}

#[doc = "FMMU Logical Stop Bit %s Register (n = 0 to 7)"]
pub type FmmuLStopBit = crate::RegValueT<FmmuLStopBit_SPEC>;

impl FmmuLStopBit {
    #[doc = "Last Bit Setting"]
    #[inline(always)]
    pub fn lstpbit(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, FmmuLStopBit_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,FmmuLStopBit_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FmmuLStopBit {
    #[inline(always)]
    fn default() -> FmmuLStopBit {
        <crate::RegValueT<FmmuLStopBit_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuPStartAdr_SPEC;
impl crate::sealed::RegSpec for FmmuPStartAdr_SPEC {
    type DataType = u16;
}

#[doc = "FMMU Physical Start Address %s Register (n = 0 to 7)"]
pub type FmmuPStartAdr = crate::RegValueT<FmmuPStartAdr_SPEC>;

impl FmmuPStartAdr {
    #[doc = "Physical Start Address Setting"]
    #[inline(always)]
    pub fn phystaadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, FmmuPStartAdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FmmuPStartAdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FmmuPStartAdr {
    #[inline(always)]
    fn default() -> FmmuPStartAdr {
        <crate::RegValueT<FmmuPStartAdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuPStartBit_SPEC;
impl crate::sealed::RegSpec for FmmuPStartBit_SPEC {
    type DataType = u8;
}

#[doc = "FMMU Physical Start Bit %s Register (n = 0 to 7)"]
pub type FmmuPStartBit = crate::RegValueT<FmmuPStartBit_SPEC>;

impl FmmuPStartBit {
    #[doc = "Physical Start Bit Setting"]
    #[inline(always)]
    pub fn phystabit(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, FmmuPStartBit_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,FmmuPStartBit_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FmmuPStartBit {
    #[inline(always)]
    fn default() -> FmmuPStartBit {
        <crate::RegValueT<FmmuPStartBit_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuType_SPEC;
impl crate::sealed::RegSpec for FmmuType_SPEC {
    type DataType = u8;
}

#[doc = "FMMU Type %s Register (n = 0 to 7)"]
pub type FmmuType = crate::RegValueT<FmmuType_SPEC>;

impl FmmuType {
    #[doc = "Read Access Mapping Setting"]
    #[inline(always)]
    pub fn read(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fmmu_type::Read,
        fmmu_type::Read,
        FmmuType_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fmmu_type::Read,
            fmmu_type::Read,
            FmmuType_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Write Access Mapping Setting"]
    #[inline(always)]
    pub fn write(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fmmu_type::Write,
        fmmu_type::Write,
        FmmuType_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fmmu_type::Write,
            fmmu_type::Write,
            FmmuType_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FmmuType {
    #[inline(always)]
    fn default() -> FmmuType {
        <crate::RegValueT<FmmuType_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fmmu_type {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Read_SPEC;
    pub type Read = crate::EnumBitfieldStruct<u8, Read_SPEC>;
    impl Read {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Write_SPEC;
    pub type Write = crate::EnumBitfieldStruct<u8, Write_SPEC>;
    impl Write {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmmuAct_SPEC;
impl crate::sealed::RegSpec for FmmuAct_SPEC {
    type DataType = u8;
}

#[doc = "FMMU Activate %s Register (n = 0 to 7)"]
pub type FmmuAct = crate::RegValueT<FmmuAct_SPEC>;

impl FmmuAct {
    #[doc = "FMMU Enable/Disable"]
    #[inline(always)]
    pub fn activate(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fmmu_act::Activate,
        fmmu_act::Activate,
        FmmuAct_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fmmu_act::Activate,
            fmmu_act::Activate,
            FmmuAct_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FmmuAct {
    #[inline(always)]
    fn default() -> FmmuAct {
        <crate::RegValueT<FmmuAct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fmmu_act {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Activate_SPEC;
    pub type Activate = crate::EnumBitfieldStruct<u8, Activate_SPEC>;
    impl Activate {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmPStartAdr_SPEC;
impl crate::sealed::RegSpec for SmPStartAdr_SPEC {
    type DataType = u16;
}

#[doc = "SyncManager Physical Start Address %s Register (n = 0 to 7)"]
pub type SmPStartAdr = crate::RegValueT<SmPStartAdr_SPEC>;

impl SmPStartAdr {
    #[doc = "Physical Start Address Setting"]
    #[inline(always)]
    pub fn smstaaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SmPStartAdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SmPStartAdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SmPStartAdr {
    #[inline(always)]
    fn default() -> SmPStartAdr {
        <crate::RegValueT<SmPStartAdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmLen_SPEC;
impl crate::sealed::RegSpec for SmLen_SPEC {
    type DataType = u16;
}

#[doc = "SyncManager Length %s Register (n = 0 to 7)"]
pub type SmLen = crate::RegValueT<SmLen_SPEC>;

impl SmLen {
    #[doc = "Area Size Setting"]
    #[inline(always)]
    pub fn smlen(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SmLen_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SmLen_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SmLen {
    #[inline(always)]
    fn default() -> SmLen {
        <crate::RegValueT<SmLen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmControl_SPEC;
impl crate::sealed::RegSpec for SmControl_SPEC {
    type DataType = u8;
}

#[doc = "SyncManager Control %s Register (n = 0 to 7)"]
pub type SmControl = crate::RegValueT<SmControl_SPEC>;

impl SmControl {
    #[doc = "Operating Mode Setting"]
    #[inline(always)]
    pub fn opemode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sm_control::Opemode,
        sm_control::Opemode,
        SmControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sm_control::Opemode,
            sm_control::Opemode,
            SmControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Direction Setting"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        sm_control::Dir,
        sm_control::Dir,
        SmControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            sm_control::Dir,
            sm_control::Dir,
            SmControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECAT Event Interrupt Setting"]
    #[inline(always)]
    pub fn irqecat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sm_control::Irqecat,
        sm_control::Irqecat,
        SmControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sm_control::Irqecat,
            sm_control::Irqecat,
            SmControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "AL Event Interrupt Setting"]
    #[inline(always)]
    pub fn irqpdi(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sm_control::Irqpdi,
        sm_control::Irqpdi,
        SmControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sm_control::Irqpdi,
            sm_control::Irqpdi,
            SmControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Watchdog Trigger Setting"]
    #[inline(always)]
    pub fn wdtrgen(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sm_control::Wdtrgen,
        sm_control::Wdtrgen,
        SmControl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sm_control::Wdtrgen,
            sm_control::Wdtrgen,
            SmControl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SmControl {
    #[inline(always)]
    fn default() -> SmControl {
        <crate::RegValueT<SmControl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sm_control {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opemode_SPEC;
    pub type Opemode = crate::EnumBitfieldStruct<u8, Opemode_SPEC>;
    impl Opemode {
        #[doc = "Buffer mode (3-buffer mode)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Mailbox mode (single buffer mode)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Read (ECAT: read access; PDI: write access)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Write (ECAT: write access; PDI: read access)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Reserved"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqecat_SPEC;
    pub type Irqecat = crate::EnumBitfieldStruct<u8, Irqecat_SPEC>;
    impl Irqecat {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqpdi_SPEC;
    pub type Irqpdi = crate::EnumBitfieldStruct<u8, Irqpdi_SPEC>;
    impl Irqpdi {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtrgen_SPEC;
    pub type Wdtrgen = crate::EnumBitfieldStruct<u8, Wdtrgen_SPEC>;
    impl Wdtrgen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmStatus_SPEC;
impl crate::sealed::RegSpec for SmStatus_SPEC {
    type DataType = u8;
}

#[doc = "SyncManager Status %s Register (n = 0 to 7)"]
pub type SmStatus = crate::RegValueT<SmStatus_SPEC>;

impl SmStatus {
    #[doc = "Write Complete Interrupt State Indication"]
    #[inline(always)]
    pub fn intwr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sm_status::Intwr,
        sm_status::Intwr,
        SmStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sm_status::Intwr,
            sm_status::Intwr,
            SmStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Read Complete Interrupt State Indication"]
    #[inline(always)]
    pub fn intrd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sm_status::Intrd,
        sm_status::Intrd,
        SmStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sm_status::Intrd,
            sm_status::Intrd,
            SmStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Mailbox Status Indication"]
    #[inline(always)]
    pub fn mailbox(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sm_status::Mailbox,
        sm_status::Mailbox,
        SmStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sm_status::Mailbox,
            sm_status::Mailbox,
            SmStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Status Indication"]
    #[inline(always)]
    pub fn buffered(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        sm_status::Buffered,
        sm_status::Buffered,
        SmStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            sm_status::Buffered,
            sm_status::Buffered,
            SmStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Read State Indication"]
    #[inline(always)]
    pub fn rdbuf(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SmStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, SmStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Write State Indication"]
    #[inline(always)]
    pub fn wrbuf(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SmStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, SmStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for SmStatus {
    #[inline(always)]
    fn default() -> SmStatus {
        <crate::RegValueT<SmStatus_SPEC> as RegisterValue<_>>::new(48)
    }
}
pub mod sm_status {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intwr_SPEC;
    pub type Intwr = crate::EnumBitfieldStruct<u8, Intwr_SPEC>;
    impl Intwr {
        #[doc = "Indicates that the buffer was successfully written"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the first byte of the buffer was read (interrupt cleared)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intrd_SPEC;
    pub type Intrd = crate::EnumBitfieldStruct<u8, Intrd_SPEC>;
    impl Intrd {
        #[doc = "Indicates that the buffer was successfully read"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates that the first byte of the buffer was written (interrupt cleared)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mailbox_SPEC;
    pub type Mailbox = crate::EnumBitfieldStruct<u8, Mailbox_SPEC>;
    impl Mailbox {
        #[doc = "Mailbox empty"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mailbox full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buffered_SPEC;
    pub type Buffered = crate::EnumBitfieldStruct<u8, Buffered_SPEC>;
    impl Buffered {
        #[doc = "1st buffer"]
        pub const _00: Self = Self::new(0);

        #[doc = "2nd buffer"]
        pub const _01: Self = Self::new(1);

        #[doc = "3rd buffer"]
        pub const _10: Self = Self::new(2);

        #[doc = "No buffer written"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmAct_SPEC;
impl crate::sealed::RegSpec for SmAct_SPEC {
    type DataType = u8;
}

#[doc = "SyncManager Activate %s Register (n = 0 to 7)"]
pub type SmAct = crate::RegValueT<SmAct_SPEC>;

impl SmAct {
    #[doc = "SyncManager Enable/Disable"]
    #[inline(always)]
    pub fn smen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sm_act::Smen,
        sm_act::Smen,
        SmAct_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sm_act::Smen,
            sm_act::Smen,
            SmAct_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Repeat Request"]
    #[inline(always)]
    pub fn repeatreq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SmAct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SmAct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ECAT Latch Event Specification"]
    #[inline(always)]
    pub fn latchecat(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sm_act::Latchecat,
        sm_act::Latchecat,
        SmAct_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sm_act::Latchecat,
            sm_act::Latchecat,
            SmAct_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PDI Latch Event Specification"]
    #[inline(always)]
    pub fn latchpdi(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sm_act::Latchpdi,
        sm_act::Latchpdi,
        SmAct_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sm_act::Latchpdi,
            sm_act::Latchpdi,
            SmAct_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SmAct {
    #[inline(always)]
    fn default() -> SmAct {
        <crate::RegValueT<SmAct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sm_act {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smen_SPEC;
    pub type Smen = crate::EnumBitfieldStruct<u8, Smen_SPEC>;
    impl Smen {
        #[doc = "Disabled. Memory is accessed without SyncManager control."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled. SyncManager is active and controls memory area set in configuration."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Latchecat_SPEC;
    pub type Latchecat = crate::EnumBitfieldStruct<u8, Latchecat_SPEC>;
    impl Latchecat {
        #[doc = "No latch events"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates latch events if the EtherCAT master switches the buffers"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Latchpdi_SPEC;
    pub type Latchpdi = crate::EnumBitfieldStruct<u8, Latchpdi_SPEC>;
    impl Latchpdi {
        #[doc = "No latch events"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates latch events if the PDI switches the buffers or accesses the buffer start address"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmPdiCont_SPEC;
impl crate::sealed::RegSpec for SmPdiCont_SPEC {
    type DataType = u8;
}

#[doc = "SyncManager PDI Control %s Register (n = 0 to 7)"]
pub type SmPdiCont = crate::RegValueT<SmPdiCont_SPEC>;

impl SmPdiCont {
    #[doc = "SyncManager Operation Indication/Setting"]
    #[inline(always)]
    pub fn deactive(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sm_pdi_cont::Deactive,
        sm_pdi_cont::Deactive,
        SmPdiCont_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sm_pdi_cont::Deactive,
            sm_pdi_cont::Deactive,
            SmPdiCont_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Repeat Acknowledge"]
    #[inline(always)]
    pub fn repeatack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SmPdiCont_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SmPdiCont_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SmPdiCont {
    #[inline(always)]
    fn default() -> SmPdiCont {
        <crate::RegValueT<SmPdiCont_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sm_pdi_cont {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deactive_SPEC;
    pub type Deactive = crate::EnumBitfieldStruct<u8, Deactive_SPEC>;
    impl Deactive {
        #[doc = "Read: Normal operation. SyncManager is activated. Write: Activates SyncManager."]
        pub const _0: Self = Self::new(0);

        #[doc = "Read: SyncManager is deactivated and reset. SyncManager locks access to memory area. Write: Deactivates SyncManager."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcRcvTimePort0_SPEC;
impl crate::sealed::RegSpec for DcRcvTimePort0_SPEC {
    type DataType = u32;
}

#[doc = "Receive Time Port 0 Register"]
pub type DcRcvTimePort0 = crate::RegValueT<DcRcvTimePort0_SPEC>;

impl DcRcvTimePort0 {
    #[doc = "Receive Time Indication/Latch"]
    #[inline(always)]
    pub fn rcvtime0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcRcvTimePort0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcRcvTimePort0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcRcvTimePort0 {
    #[inline(always)]
    fn default() -> DcRcvTimePort0 {
        <crate::RegValueT<DcRcvTimePort0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcRcvTimePort1_SPEC;
impl crate::sealed::RegSpec for DcRcvTimePort1_SPEC {
    type DataType = u32;
}

#[doc = "Receive Time Port 1 Register"]
pub type DcRcvTimePort1 = crate::RegValueT<DcRcvTimePort1_SPEC>;

impl DcRcvTimePort1 {
    #[doc = "Receive Time Indication"]
    #[inline(always)]
    pub fn rcvtime1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcRcvTimePort1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcRcvTimePort1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcRcvTimePort1 {
    #[inline(always)]
    fn default() -> DcRcvTimePort1 {
        <crate::RegValueT<DcRcvTimePort1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcRcvTimePort2_SPEC;
impl crate::sealed::RegSpec for DcRcvTimePort2_SPEC {
    type DataType = u32;
}

#[doc = "Receive Time Port 2 Register"]
pub type DcRcvTimePort2 = crate::RegValueT<DcRcvTimePort2_SPEC>;

impl DcRcvTimePort2 {
    #[doc = "Receive Time Indication"]
    #[inline(always)]
    pub fn rcvtime2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcRcvTimePort2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcRcvTimePort2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcRcvTimePort2 {
    #[inline(always)]
    fn default() -> DcRcvTimePort2 {
        <crate::RegValueT<DcRcvTimePort2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSysTimeL_SPEC;
impl crate::sealed::RegSpec for DcSysTimeL_SPEC {
    type DataType = u32;
}

#[doc = "System Time Register L"]
pub type DcSysTimeL = crate::RegValueT<DcSysTimeL_SPEC>;

impl DcSysTimeL {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcSysTimeL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcSysTimeL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSysTimeL {
    #[inline(always)]
    fn default() -> DcSysTimeL {
        <crate::RegValueT<DcSysTimeL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSysTimeH_SPEC;
impl crate::sealed::RegSpec for DcSysTimeH_SPEC {
    type DataType = u32;
}

#[doc = "System Time Register H"]
pub type DcSysTimeH = crate::RegValueT<DcSysTimeH_SPEC>;

impl DcSysTimeH {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcSysTimeH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcSysTimeH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSysTimeH {
    #[inline(always)]
    fn default() -> DcSysTimeH {
        <crate::RegValueT<DcSysTimeH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcRcvTimeUnitL_SPEC;
impl crate::sealed::RegSpec for DcRcvTimeUnitL_SPEC {
    type DataType = u32;
}

#[doc = "Receive Time ECAT Processing Unit Register L"]
pub type DcRcvTimeUnitL = crate::RegValueT<DcRcvTimeUnitL_SPEC>;

impl DcRcvTimeUnitL {
    #[doc = "Receive Time Indication"]
    #[inline(always)]
    pub fn rcvtimeepu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcRcvTimeUnitL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcRcvTimeUnitL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcRcvTimeUnitL {
    #[inline(always)]
    fn default() -> DcRcvTimeUnitL {
        <crate::RegValueT<DcRcvTimeUnitL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcRcvTimeUnitH_SPEC;
impl crate::sealed::RegSpec for DcRcvTimeUnitH_SPEC {
    type DataType = u32;
}

#[doc = "Receive Time ECAT Processing Unit Register H"]
pub type DcRcvTimeUnitH = crate::RegValueT<DcRcvTimeUnitH_SPEC>;

impl DcRcvTimeUnitH {
    #[doc = "Receive Time Indication"]
    #[inline(always)]
    pub fn rcvtimeepu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcRcvTimeUnitH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcRcvTimeUnitH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcRcvTimeUnitH {
    #[inline(always)]
    fn default() -> DcRcvTimeUnitH {
        <crate::RegValueT<DcRcvTimeUnitH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSysTimeOffsetL_SPEC;
impl crate::sealed::RegSpec for DcSysTimeOffsetL_SPEC {
    type DataType = u32;
}

#[doc = "System Time Offset Register L"]
pub type DcSysTimeOffsetL = crate::RegValueT<DcSysTimeOffsetL_SPEC>;

impl DcSysTimeOffsetL {
    #[doc = "System Time and Local Time Difference Indication"]
    #[inline(always)]
    pub fn systimofst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcSysTimeOffsetL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcSysTimeOffsetL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSysTimeOffsetL {
    #[inline(always)]
    fn default() -> DcSysTimeOffsetL {
        <crate::RegValueT<DcSysTimeOffsetL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSysTimeOffsetH_SPEC;
impl crate::sealed::RegSpec for DcSysTimeOffsetH_SPEC {
    type DataType = u32;
}

#[doc = "System Time Offset Register H"]
pub type DcSysTimeOffsetH = crate::RegValueT<DcSysTimeOffsetH_SPEC>;

impl DcSysTimeOffsetH {
    #[doc = "System Time and Local Time Difference Indication"]
    #[inline(always)]
    pub fn systimofst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcSysTimeOffsetH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcSysTimeOffsetH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSysTimeOffsetH {
    #[inline(always)]
    fn default() -> DcSysTimeOffsetH {
        <crate::RegValueT<DcSysTimeOffsetH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSysTimeDelay_SPEC;
impl crate::sealed::RegSpec for DcSysTimeDelay_SPEC {
    type DataType = u32;
}

#[doc = "System Time Delay Register"]
pub type DcSysTimeDelay = crate::RegValueT<DcSysTimeDelay_SPEC>;

impl DcSysTimeDelay {
    #[doc = "Propagation Delay Indication"]
    #[inline(always)]
    pub fn systimdly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcSysTimeDelay_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcSysTimeDelay_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSysTimeDelay {
    #[inline(always)]
    fn default() -> DcSysTimeDelay {
        <crate::RegValueT<DcSysTimeDelay_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSysTimeDiff_SPEC;
impl crate::sealed::RegSpec for DcSysTimeDiff_SPEC {
    type DataType = u32;
}

#[doc = "System Time Difference Register"]
pub type DcSysTimeDiff = crate::RegValueT<DcSysTimeDiff_SPEC>;

impl DcSysTimeDiff {
    #[doc = "System Time Mean Difference Indication"]
    #[inline(always)]
    pub fn diff(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffffff,
        1,
        0,
        u32,
        u32,
        DcSysTimeDiff_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            DcSysTimeDiff_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "System Time Greater/Less Indication"]
    #[inline(always)]
    pub fn lcp(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        dc_sys_time_diff::Lcp,
        dc_sys_time_diff::Lcp,
        DcSysTimeDiff_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            dc_sys_time_diff::Lcp,
            dc_sys_time_diff::Lcp,
            DcSysTimeDiff_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSysTimeDiff {
    #[inline(always)]
    fn default() -> DcSysTimeDiff {
        <crate::RegValueT<DcSysTimeDiff_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_sys_time_diff {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcp_SPEC;
    pub type Lcp = crate::EnumBitfieldStruct<u8, Lcp_SPEC>;
    impl Lcp {
        #[doc = "Local copy of the system time greater than or equal to the received system time"]
        pub const _0: Self = Self::new(0);

        #[doc = "Local copy of the system time less than the received system time"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSpeedCountStart_SPEC;
impl crate::sealed::RegSpec for DcSpeedCountStart_SPEC {
    type DataType = u16;
}

#[doc = "Speed Counter Start Register"]
pub type DcSpeedCountStart = crate::RegValueT<DcSpeedCountStart_SPEC>;

impl DcSpeedCountStart {
    #[doc = "Drift Correction Bandwidth Setting"]
    #[inline(always)]
    pub fn spdcntstrt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        DcSpeedCountStart_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            DcSpeedCountStart_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSpeedCountStart {
    #[inline(always)]
    fn default() -> DcSpeedCountStart {
        <crate::RegValueT<DcSpeedCountStart_SPEC> as RegisterValue<_>>::new(4096)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSpeedCountDiff_SPEC;
impl crate::sealed::RegSpec for DcSpeedCountDiff_SPEC {
    type DataType = u16;
}

#[doc = "Speed Counter Difference Register"]
pub type DcSpeedCountDiff = crate::RegValueT<DcSpeedCountDiff_SPEC>;

impl DcSpeedCountDiff {
    #[doc = "Clock Period Deviation Indication"]
    #[inline(always)]
    pub fn spdcntdiff(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        DcSpeedCountDiff_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            DcSpeedCountDiff_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSpeedCountDiff {
    #[inline(always)]
    fn default() -> DcSpeedCountDiff {
        <crate::RegValueT<DcSpeedCountDiff_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSysTimeDiffFilDepth_SPEC;
impl crate::sealed::RegSpec for DcSysTimeDiffFilDepth_SPEC {
    type DataType = u8;
}

#[doc = "System Time Difference Filter Depth Register"]
pub type DcSysTimeDiffFilDepth = crate::RegValueT<DcSysTimeDiffFilDepth_SPEC>;

impl DcSysTimeDiffFilDepth {
    #[doc = "Filter Depth Setting"]
    #[inline(always)]
    pub fn systimdep(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        DcSysTimeDiffFilDepth_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            DcSysTimeDiffFilDepth_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSysTimeDiffFilDepth {
    #[inline(always)]
    fn default() -> DcSysTimeDiffFilDepth {
        <crate::RegValueT<DcSysTimeDiffFilDepth_SPEC> as RegisterValue<_>>::new(4)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSpeedCountFilDepth_SPEC;
impl crate::sealed::RegSpec for DcSpeedCountFilDepth_SPEC {
    type DataType = u8;
}

#[doc = "Speed Counter Filter Depth Register"]
pub type DcSpeedCountFilDepth = crate::RegValueT<DcSpeedCountFilDepth_SPEC>;

impl DcSpeedCountFilDepth {
    #[doc = "Filter Depth Setting"]
    #[inline(always)]
    pub fn clkperdep(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        DcSpeedCountFilDepth_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            DcSpeedCountFilDepth_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSpeedCountFilDepth {
    #[inline(always)]
    fn default() -> DcSpeedCountFilDepth {
        <crate::RegValueT<DcSpeedCountFilDepth_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcCycCont_SPEC;
impl crate::sealed::RegSpec for DcCycCont_SPEC {
    type DataType = u8;
}

#[doc = "Cyclic Unit Control Register"]
pub type DcCycCont = crate::RegValueT<DcCycCont_SPEC>;

impl DcCycCont {
    #[doc = "SYNC Output Unit Control Setting"]
    #[inline(always)]
    pub fn syncout(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dc_cyc_cont::Syncout,
        dc_cyc_cont::Syncout,
        DcCycCont_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dc_cyc_cont::Syncout,
            dc_cyc_cont::Syncout,
            DcCycCont_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Latch Input Unit 0 Control Setting"]
    #[inline(always)]
    pub fn latch0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dc_cyc_cont::Latch0,
        dc_cyc_cont::Latch0,
        DcCycCont_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dc_cyc_cont::Latch0,
            dc_cyc_cont::Latch0,
            DcCycCont_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Latch Input Unit 1 Control Setting"]
    #[inline(always)]
    pub fn latch1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dc_cyc_cont::Latch1,
        dc_cyc_cont::Latch1,
        DcCycCont_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dc_cyc_cont::Latch1,
            dc_cyc_cont::Latch1,
            DcCycCont_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcCycCont {
    #[inline(always)]
    fn default() -> DcCycCont {
        <crate::RegValueT<DcCycCont_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_cyc_cont {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncout_SPEC;
    pub type Syncout = crate::EnumBitfieldStruct<u8, Syncout_SPEC>;
    impl Syncout {
        #[doc = "ECAT control"]
        pub const _0: Self = Self::new(0);

        #[doc = "PDI control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Latch0_SPEC;
    pub type Latch0 = crate::EnumBitfieldStruct<u8, Latch0_SPEC>;
    impl Latch0 {
        #[doc = "ECAT control"]
        pub const _0: Self = Self::new(0);

        #[doc = "PDI control"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Latch1_SPEC;
    pub type Latch1 = crate::EnumBitfieldStruct<u8, Latch1_SPEC>;
    impl Latch1 {
        #[doc = "ECAT control"]
        pub const _0: Self = Self::new(0);

        #[doc = "PDI control"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcAct_SPEC;
impl crate::sealed::RegSpec for DcAct_SPEC {
    type DataType = u8;
}

#[doc = "Activation Register"]
pub type DcAct = crate::RegValueT<DcAct_SPEC>;

impl DcAct {
    #[doc = "Sync Output Unit Activation"]
    #[inline(always)]
    pub fn syncact(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dc_act::Syncact,
        dc_act::Syncact,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dc_act::Syncact,
            dc_act::Syncact,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNC0 Output Setting"]
    #[inline(always)]
    pub fn sync0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dc_act::Sync0,
        dc_act::Sync0,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dc_act::Sync0,
            dc_act::Sync0,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNC1 Output Setting"]
    #[inline(always)]
    pub fn sync1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dc_act::Sync1,
        dc_act::Sync1,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dc_act::Sync1,
            dc_act::Sync1,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNC Output Unit Activation"]
    #[inline(always)]
    pub fn autoact(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dc_act::Autoact,
        dc_act::Autoact,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dc_act::Autoact,
            dc_act::Autoact,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Time Cyclic Operation Extension"]
    #[inline(always)]
    pub fn extstarttime(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dc_act::Extstarttime,
        dc_act::Extstarttime,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dc_act::Extstarttime,
            dc_act::Extstarttime,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Time Plausibility"]
    #[inline(always)]
    pub fn starttime(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dc_act::Starttime,
        dc_act::Starttime,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dc_act::Starttime,
            dc_act::Starttime,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Near Future Range Setting"]
    #[inline(always)]
    pub fn nearfuture(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dc_act::Nearfuture,
        dc_act::Nearfuture,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dc_act::Nearfuture,
            dc_act::Nearfuture,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Debug Pulse Setting"]
    #[inline(always)]
    pub fn dbgpulse(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dc_act::Dbgpulse,
        dc_act::Dbgpulse,
        DcAct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dc_act::Dbgpulse,
            dc_act::Dbgpulse,
            DcAct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcAct {
    #[inline(always)]
    fn default() -> DcAct {
        <crate::RegValueT<DcAct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_act {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncact_SPEC;
    pub type Syncact = crate::EnumBitfieldStruct<u8, Syncact_SPEC>;
    impl Syncact {
        #[doc = "Deactivated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Activated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync0_SPEC;
    pub type Sync0 = crate::EnumBitfieldStruct<u8, Sync0_SPEC>;
    impl Sync0 {
        #[doc = "Deactivated"]
        pub const _0: Self = Self::new(0);

        #[doc = "SYNC0 pulse output is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync1_SPEC;
    pub type Sync1 = crate::EnumBitfieldStruct<u8, Sync1_SPEC>;
    impl Sync1 {
        #[doc = "Deactivated"]
        pub const _0: Self = Self::new(0);

        #[doc = "SYNC1 pulse output is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Autoact_SPEC;
    pub type Autoact = crate::EnumBitfieldStruct<u8, Autoact_SPEC>;
    impl Autoact {
        #[doc = "Deactivated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Activated. Bit 0 is automatically set to 1 in this register after the start time is written."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extstarttime_SPEC;
    pub type Extstarttime = crate::EnumBitfieldStruct<u8, Extstarttime_SPEC>;
    impl Extstarttime {
        #[doc = "No extension"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extends the start time written with 32 bits to 64 bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Starttime_SPEC;
    pub type Starttime = crate::EnumBitfieldStruct<u8, Starttime_SPEC>;
    impl Starttime {
        #[doc = "Disabled. Sync signal is generated if the start time is reached."]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync signal is generated immediately if the start time is outside the range of the near future."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nearfuture_SPEC;
    pub type Nearfuture = crate::EnumBitfieldStruct<u8, Nearfuture_SPEC>;
    impl Nearfuture {
        #[doc = "Up to 263 ns from now (1/2 of the DC width)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Up to 231 ns from now (about 2.1 sec.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgpulse_SPEC;
    pub type Dbgpulse = crate::EnumBitfieldStruct<u8, Dbgpulse_SPEC>;
    impl Dbgpulse {
        #[doc = "Deactivated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Immediately generates a single debug ping on the SYNC0 and SYNC1 pins in accord with the setting of bits 2 and 1 of this register."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcPulseLen_SPEC;
impl crate::sealed::RegSpec for DcPulseLen_SPEC {
    type DataType = u16;
}

#[doc = "SYNC Signal Pulse Length Register"]
pub type DcPulseLen = crate::RegValueT<DcPulseLen_SPEC>;

impl DcPulseLen {
    #[doc = "SYNC Signal Pulse Length Indication"]
    #[inline(always)]
    pub fn pulselen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        dc_pulse_len::Pulselen,
        dc_pulse_len::Pulselen,
        DcPulseLen_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            dc_pulse_len::Pulselen,
            dc_pulse_len::Pulselen,
            DcPulseLen_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcPulseLen {
    #[inline(always)]
    fn default() -> DcPulseLen {
        <crate::RegValueT<DcPulseLen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_pulse_len {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pulselen_SPEC;
    pub type Pulselen = crate::EnumBitfieldStruct<u8, Pulselen_SPEC>;
    impl Pulselen {
        #[doc = "Acknowledge mode. In this mode, SYNC signal is cleared by reading the SYNC0 or SYNC1 status register (DC_SYNC0/1_STAT at 0x098E, 0x098F)."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcActStat_SPEC;
impl crate::sealed::RegSpec for DcActStat_SPEC {
    type DataType = u8;
}

#[doc = "Activation Status Register"]
pub type DcActStat = crate::RegValueT<DcActStat_SPEC>;

impl DcActStat {
    #[doc = "SYNC0 Status Indication"]
    #[inline(always)]
    pub fn sync0act(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dc_act_stat::Sync0Act,
        dc_act_stat::Sync0Act,
        DcActStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dc_act_stat::Sync0Act,
            dc_act_stat::Sync0Act,
            DcActStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SYNC1 Status Indication"]
    #[inline(always)]
    pub fn sync1act(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dc_act_stat::Sync1Act,
        dc_act_stat::Sync1Act,
        DcActStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dc_act_stat::Sync1Act,
            dc_act_stat::Sync1Act,
            DcActStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Plausibility Result Indication"]
    #[inline(always)]
    pub fn starttime(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dc_act_stat::Starttime,
        dc_act_stat::Starttime,
        DcActStat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dc_act_stat::Starttime,
            dc_act_stat::Starttime,
            DcActStat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcActStat {
    #[inline(always)]
    fn default() -> DcActStat {
        <crate::RegValueT<DcActStat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_act_stat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync0Act_SPEC;
    pub type Sync0Act = crate::EnumBitfieldStruct<u8, Sync0Act_SPEC>;
    impl Sync0Act {
        #[doc = "First SYNC0 pulse is not pending"]
        pub const _0: Self = Self::new(0);

        #[doc = "First SYNC0 pulse is pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync1Act_SPEC;
    pub type Sync1Act = crate::EnumBitfieldStruct<u8, Sync1Act_SPEC>;
    impl Sync1Act {
        #[doc = "First SYNC1 pulse is not pending"]
        pub const _0: Self = Self::new(0);

        #[doc = "First SYNC1 pulse is pending"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Starttime_SPEC;
    pub type Starttime = crate::EnumBitfieldStruct<u8, Starttime_SPEC>;
    impl Starttime {
        #[doc = "The start time is within the near future"]
        pub const _0: Self = Self::new(0);

        #[doc = "The start time is out of the near future"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSync0Stat_SPEC;
impl crate::sealed::RegSpec for DcSync0Stat_SPEC {
    type DataType = u8;
}

#[doc = "SYNC0 Status Register"]
pub type DcSync0Stat = crate::RegValueT<DcSync0Stat_SPEC>;

impl DcSync0Stat {
    #[doc = "SYNC0 State Indication"]
    #[inline(always)]
    pub fn sync0sta(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcSync0Stat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,DcSync0Stat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcSync0Stat {
    #[inline(always)]
    fn default() -> DcSync0Stat {
        <crate::RegValueT<DcSync0Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSync1Stat_SPEC;
impl crate::sealed::RegSpec for DcSync1Stat_SPEC {
    type DataType = u8;
}

#[doc = "SYNC1 Status Register"]
pub type DcSync1Stat = crate::RegValueT<DcSync1Stat_SPEC>;

impl DcSync1Stat {
    #[doc = "SYNC1 State Indication"]
    #[inline(always)]
    pub fn sync1sta(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcSync1Stat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,DcSync1Stat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcSync1Stat {
    #[inline(always)]
    fn default() -> DcSync1Stat {
        <crate::RegValueT<DcSync1Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcCycStartTimeL_SPEC;
impl crate::sealed::RegSpec for DcCycStartTimeL_SPEC {
    type DataType = u32;
}

#[doc = "Start Time Cyclic Operation/Next SYNC0 Pulse Register L"]
pub type DcCycStartTimeL = crate::RegValueT<DcCycStartTimeL_SPEC>;

impl DcCycStartTimeL {
    #[doc = "Start Time Setting/System Time Indication"]
    #[inline(always)]
    pub fn statim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcCycStartTimeL_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcCycStartTimeL_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcCycStartTimeL {
    #[inline(always)]
    fn default() -> DcCycStartTimeL {
        <crate::RegValueT<DcCycStartTimeL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcCycStartTimeH_SPEC;
impl crate::sealed::RegSpec for DcCycStartTimeH_SPEC {
    type DataType = u32;
}

#[doc = "Start Time Cyclic Operation/Next SYNC0 Pulse Register H"]
pub type DcCycStartTimeH = crate::RegValueT<DcCycStartTimeH_SPEC>;

impl DcCycStartTimeH {
    #[doc = "Start Time Setting/System Time Indication"]
    #[inline(always)]
    pub fn statim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcCycStartTimeH_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcCycStartTimeH_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcCycStartTimeH {
    #[inline(always)]
    fn default() -> DcCycStartTimeH {
        <crate::RegValueT<DcCycStartTimeH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcNextSync1PulseL_SPEC;
impl crate::sealed::RegSpec for DcNextSync1PulseL_SPEC {
    type DataType = u32;
}

#[doc = "Next SYNC1 Pulse Register L"]
pub type DcNextSync1PulseL = crate::RegValueT<DcNextSync1PulseL_SPEC>;

impl DcNextSync1PulseL {
    #[doc = "SYNC1 Pulse System Time Indication"]
    #[inline(always)]
    pub fn sync1pulse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcNextSync1PulseL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcNextSync1PulseL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcNextSync1PulseL {
    #[inline(always)]
    fn default() -> DcNextSync1PulseL {
        <crate::RegValueT<DcNextSync1PulseL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcNextSync1PulseH_SPEC;
impl crate::sealed::RegSpec for DcNextSync1PulseH_SPEC {
    type DataType = u32;
}

#[doc = "Next SYNC1 Pulse Register H"]
pub type DcNextSync1PulseH = crate::RegValueT<DcNextSync1PulseH_SPEC>;

impl DcNextSync1PulseH {
    #[doc = "SYNC1 Pulse System Time Indication"]
    #[inline(always)]
    pub fn sync1pulse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcNextSync1PulseH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcNextSync1PulseH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcNextSync1PulseH {
    #[inline(always)]
    fn default() -> DcNextSync1PulseH {
        <crate::RegValueT<DcNextSync1PulseH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSync0CycTime_SPEC;
impl crate::sealed::RegSpec for DcSync0CycTime_SPEC {
    type DataType = u32;
}

#[doc = "SYNC0 Cycle Time Register"]
pub type DcSync0CycTime = crate::RegValueT<DcSync0CycTime_SPEC>;

impl DcSync0CycTime {
    #[doc = "Time Between Consecutive SYNC0 Pulses"]
    #[inline(always)]
    pub fn sync0cyc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcSync0CycTime_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcSync0CycTime_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSync0CycTime {
    #[inline(always)]
    fn default() -> DcSync0CycTime {
        <crate::RegValueT<DcSync0CycTime_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcSync1CycTime_SPEC;
impl crate::sealed::RegSpec for DcSync1CycTime_SPEC {
    type DataType = u32;
}

#[doc = "SYNC1 Cycle Time Register"]
pub type DcSync1CycTime = crate::RegValueT<DcSync1CycTime_SPEC>;

impl DcSync1CycTime {
    #[doc = "Time between SYNC1 and SYNC0 Pulses"]
    #[inline(always)]
    pub fn sync1cyc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcSync1CycTime_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcSync1CycTime_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcSync1CycTime {
    #[inline(always)]
    fn default() -> DcSync1CycTime {
        <crate::RegValueT<DcSync1CycTime_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch0Cont_SPEC;
impl crate::sealed::RegSpec for DcLatch0Cont_SPEC {
    type DataType = u8;
}

#[doc = "Latch 0 Control Register"]
pub type DcLatch0Cont = crate::RegValueT<DcLatch0Cont_SPEC>;

impl DcLatch0Cont {
    #[doc = "Latch 0 Positive Edge Function Setting"]
    #[inline(always)]
    pub fn posedge(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dc_latch0_cont::Posedge,
        dc_latch0_cont::Posedge,
        DcLatch0Cont_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dc_latch0_cont::Posedge,
            dc_latch0_cont::Posedge,
            DcLatch0Cont_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Latch 0 Negative Edge Function Setting"]
    #[inline(always)]
    pub fn negedge(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dc_latch0_cont::Negedge,
        dc_latch0_cont::Negedge,
        DcLatch0Cont_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dc_latch0_cont::Negedge,
            dc_latch0_cont::Negedge,
            DcLatch0Cont_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch0Cont {
    #[inline(always)]
    fn default() -> DcLatch0Cont {
        <crate::RegValueT<DcLatch0Cont_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_latch0_cont {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posedge_SPEC;
    pub type Posedge = crate::EnumBitfieldStruct<u8, Posedge_SPEC>;
    impl Posedge {
        #[doc = "Continuous latch active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single event (only first event active)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Negedge_SPEC;
    pub type Negedge = crate::EnumBitfieldStruct<u8, Negedge_SPEC>;
    impl Negedge {
        #[doc = "Continuous latch active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single event (only first event active)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch1Cont_SPEC;
impl crate::sealed::RegSpec for DcLatch1Cont_SPEC {
    type DataType = u8;
}

#[doc = "Latch 1 Control Register"]
pub type DcLatch1Cont = crate::RegValueT<DcLatch1Cont_SPEC>;

impl DcLatch1Cont {
    #[doc = "Latch 1 Positive Edge Function Setting"]
    #[inline(always)]
    pub fn posedge(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dc_latch1_cont::Posedge,
        dc_latch1_cont::Posedge,
        DcLatch1Cont_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dc_latch1_cont::Posedge,
            dc_latch1_cont::Posedge,
            DcLatch1Cont_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Latch 1 Negative Edge Function Setting"]
    #[inline(always)]
    pub fn negedge(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dc_latch1_cont::Negedge,
        dc_latch1_cont::Negedge,
        DcLatch1Cont_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dc_latch1_cont::Negedge,
            dc_latch1_cont::Negedge,
            DcLatch1Cont_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch1Cont {
    #[inline(always)]
    fn default() -> DcLatch1Cont {
        <crate::RegValueT<DcLatch1Cont_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_latch1_cont {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posedge_SPEC;
    pub type Posedge = crate::EnumBitfieldStruct<u8, Posedge_SPEC>;
    impl Posedge {
        #[doc = "Continuous latch active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single event (only first event active)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Negedge_SPEC;
    pub type Negedge = crate::EnumBitfieldStruct<u8, Negedge_SPEC>;
    impl Negedge {
        #[doc = "Continuous Latch active"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single event (only first event active)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch0Stat_SPEC;
impl crate::sealed::RegSpec for DcLatch0Stat_SPEC {
    type DataType = u8;
}

#[doc = "Latch 0 Status Register"]
pub type DcLatch0Stat = crate::RegValueT<DcLatch0Stat_SPEC>;

impl DcLatch0Stat {
    #[doc = "Latch 0 Positive Edge Event Indication"]
    #[inline(always)]
    pub fn eventpos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dc_latch0_stat::Eventpos,
        dc_latch0_stat::Eventpos,
        DcLatch0Stat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dc_latch0_stat::Eventpos,
            dc_latch0_stat::Eventpos,
            DcLatch0Stat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Latch 0 Negative Edge Event Indication"]
    #[inline(always)]
    pub fn eventneg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dc_latch0_stat::Eventneg,
        dc_latch0_stat::Eventneg,
        DcLatch0Stat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dc_latch0_stat::Eventneg,
            dc_latch0_stat::Eventneg,
            DcLatch0Stat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Latch 0 Input Pin State Indication"]
    #[inline(always)]
    pub fn pinstate(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcLatch0Stat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DcLatch0Stat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcLatch0Stat {
    #[inline(always)]
    fn default() -> DcLatch0Stat {
        <crate::RegValueT<DcLatch0Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_latch0_stat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eventpos_SPEC;
    pub type Eventpos = crate::EnumBitfieldStruct<u8, Eventpos_SPEC>;
    impl Eventpos {
        #[doc = "Rising edge not detected or continuous mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge detected and mode is single-event"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eventneg_SPEC;
    pub type Eventneg = crate::EnumBitfieldStruct<u8, Eventneg_SPEC>;
    impl Eventneg {
        #[doc = "Falling edge not detected or continuous mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edge detected and mode is single-event"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch1Stat_SPEC;
impl crate::sealed::RegSpec for DcLatch1Stat_SPEC {
    type DataType = u8;
}

#[doc = "Latch 1 Status Register"]
pub type DcLatch1Stat = crate::RegValueT<DcLatch1Stat_SPEC>;

impl DcLatch1Stat {
    #[doc = "Latch 1 Positive Edge Event Indication"]
    #[inline(always)]
    pub fn eventpos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dc_latch1_stat::Eventpos,
        dc_latch1_stat::Eventpos,
        DcLatch1Stat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dc_latch1_stat::Eventpos,
            dc_latch1_stat::Eventpos,
            DcLatch1Stat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Latch 1 Negative Edge Event Indication"]
    #[inline(always)]
    pub fn eventneg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dc_latch1_stat::Eventneg,
        dc_latch1_stat::Eventneg,
        DcLatch1Stat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dc_latch1_stat::Eventneg,
            dc_latch1_stat::Eventneg,
            DcLatch1Stat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Latch 1 Input Pin State Indication"]
    #[inline(always)]
    pub fn pinstate(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcLatch1Stat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DcLatch1Stat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcLatch1Stat {
    #[inline(always)]
    fn default() -> DcLatch1Stat {
        <crate::RegValueT<DcLatch1Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dc_latch1_stat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eventpos_SPEC;
    pub type Eventpos = crate::EnumBitfieldStruct<u8, Eventpos_SPEC>;
    impl Eventpos {
        #[doc = "Rising edge not detected or continuous mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge detected and mode is single-event"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eventneg_SPEC;
    pub type Eventneg = crate::EnumBitfieldStruct<u8, Eventneg_SPEC>;
    impl Eventneg {
        #[doc = "Falling edge not detected or continuous mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edge detected and mode is single-event"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch0TimePosL_SPEC;
impl crate::sealed::RegSpec for DcLatch0TimePosL_SPEC {
    type DataType = u32;
}

#[doc = "Latch 0 Time Positive Edge Register L"]
pub type DcLatch0TimePosL = crate::RegValueT<DcLatch0TimePosL_SPEC>;

impl DcLatch0TimePosL {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch0TimePosL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch0TimePosL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch0TimePosL {
    #[inline(always)]
    fn default() -> DcLatch0TimePosL {
        <crate::RegValueT<DcLatch0TimePosL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch0TimePosH_SPEC;
impl crate::sealed::RegSpec for DcLatch0TimePosH_SPEC {
    type DataType = u32;
}

#[doc = "Latch 0 Time Positive Edge Register H"]
pub type DcLatch0TimePosH = crate::RegValueT<DcLatch0TimePosH_SPEC>;

impl DcLatch0TimePosH {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch0TimePosH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch0TimePosH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch0TimePosH {
    #[inline(always)]
    fn default() -> DcLatch0TimePosH {
        <crate::RegValueT<DcLatch0TimePosH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch0TimeNegL_SPEC;
impl crate::sealed::RegSpec for DcLatch0TimeNegL_SPEC {
    type DataType = u32;
}

#[doc = "Latch 0 Time Negative Edge Register L"]
pub type DcLatch0TimeNegL = crate::RegValueT<DcLatch0TimeNegL_SPEC>;

impl DcLatch0TimeNegL {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch0TimeNegL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch0TimeNegL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch0TimeNegL {
    #[inline(always)]
    fn default() -> DcLatch0TimeNegL {
        <crate::RegValueT<DcLatch0TimeNegL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch0TimeNegH_SPEC;
impl crate::sealed::RegSpec for DcLatch0TimeNegH_SPEC {
    type DataType = u32;
}

#[doc = "Latch 0 Time Negative Edge Register H"]
pub type DcLatch0TimeNegH = crate::RegValueT<DcLatch0TimeNegH_SPEC>;

impl DcLatch0TimeNegH {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch0TimeNegH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch0TimeNegH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch0TimeNegH {
    #[inline(always)]
    fn default() -> DcLatch0TimeNegH {
        <crate::RegValueT<DcLatch0TimeNegH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch1TimePosL_SPEC;
impl crate::sealed::RegSpec for DcLatch1TimePosL_SPEC {
    type DataType = u32;
}

#[doc = "Latch 1 Time Positive Edge Register L"]
pub type DcLatch1TimePosL = crate::RegValueT<DcLatch1TimePosL_SPEC>;

impl DcLatch1TimePosL {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch1TimePosL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch1TimePosL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch1TimePosL {
    #[inline(always)]
    fn default() -> DcLatch1TimePosL {
        <crate::RegValueT<DcLatch1TimePosL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch1TimePosH_SPEC;
impl crate::sealed::RegSpec for DcLatch1TimePosH_SPEC {
    type DataType = u32;
}

#[doc = "Latch 1 Time Positive Edge Register H"]
pub type DcLatch1TimePosH = crate::RegValueT<DcLatch1TimePosH_SPEC>;

impl DcLatch1TimePosH {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch1TimePosH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch1TimePosH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch1TimePosH {
    #[inline(always)]
    fn default() -> DcLatch1TimePosH {
        <crate::RegValueT<DcLatch1TimePosH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch1TimeNegL_SPEC;
impl crate::sealed::RegSpec for DcLatch1TimeNegL_SPEC {
    type DataType = u32;
}

#[doc = "Latch 1 Time Negative Edge Register L"]
pub type DcLatch1TimeNegL = crate::RegValueT<DcLatch1TimeNegL_SPEC>;

impl DcLatch1TimeNegL {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch1TimeNegL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch1TimeNegL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch1TimeNegL {
    #[inline(always)]
    fn default() -> DcLatch1TimeNegL {
        <crate::RegValueT<DcLatch1TimeNegL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcLatch1TimeNegH_SPEC;
impl crate::sealed::RegSpec for DcLatch1TimeNegH_SPEC {
    type DataType = u32;
}

#[doc = "Latch 1 Time Negative Edge Register H"]
pub type DcLatch1TimeNegH = crate::RegValueT<DcLatch1TimeNegH_SPEC>;

impl DcLatch1TimeNegH {
    #[doc = "System Time Indication"]
    #[inline(always)]
    pub fn systime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcLatch1TimeNegH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcLatch1TimeNegH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcLatch1TimeNegH {
    #[inline(always)]
    fn default() -> DcLatch1TimeNegH {
        <crate::RegValueT<DcLatch1TimeNegH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcEcatCngEvTime_SPEC;
impl crate::sealed::RegSpec for DcEcatCngEvTime_SPEC {
    type DataType = u32;
}

#[doc = "Buffer Change Event Time Register"]
pub type DcEcatCngEvTime = crate::RegValueT<DcEcatCngEvTime_SPEC>;

impl DcEcatCngEvTime {
    #[doc = "Local Time Indication"]
    #[inline(always)]
    pub fn ecatchange(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcEcatCngEvTime_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcEcatCngEvTime_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcEcatCngEvTime {
    #[inline(always)]
    fn default() -> DcEcatCngEvTime {
        <crate::RegValueT<DcEcatCngEvTime_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcPdiStartEvTime_SPEC;
impl crate::sealed::RegSpec for DcPdiStartEvTime_SPEC {
    type DataType = u32;
}

#[doc = "PDI Buffer Start Event Time Register"]
pub type DcPdiStartEvTime = crate::RegValueT<DcPdiStartEvTime_SPEC>;

impl DcPdiStartEvTime {
    #[doc = "Local Time Indication"]
    #[inline(always)]
    pub fn pdistart(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcPdiStartEvTime_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcPdiStartEvTime_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcPdiStartEvTime {
    #[inline(always)]
    fn default() -> DcPdiStartEvTime {
        <crate::RegValueT<DcPdiStartEvTime_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcPdiCngEvTime_SPEC;
impl crate::sealed::RegSpec for DcPdiCngEvTime_SPEC {
    type DataType = u32;
}

#[doc = "PDI Buffer Change Event Time Register"]
pub type DcPdiCngEvTime = crate::RegValueT<DcPdiCngEvTime_SPEC>;

impl DcPdiCngEvTime {
    #[doc = "Local Time Indication"]
    #[inline(always)]
    pub fn pdichange(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DcPdiCngEvTime_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DcPdiCngEvTime_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DcPdiCngEvTime {
    #[inline(always)]
    fn default() -> DcPdiCngEvTime {
        <crate::RegValueT<DcPdiCngEvTime_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProductIdL_SPEC;
impl crate::sealed::RegSpec for ProductIdL_SPEC {
    type DataType = u32;
}

#[doc = "Product ID Register L"]
pub type ProductIdL = crate::RegValueT<ProductIdL_SPEC>;

impl ProductIdL {
    #[doc = "Product ID Indication"]
    #[inline(always)]
    pub fn proid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ProductIdL_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ProductIdL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ProductIdL {
    #[inline(always)]
    fn default() -> ProductIdL {
        <crate::RegValueT<ProductIdL_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProductIdH_SPEC;
impl crate::sealed::RegSpec for ProductIdH_SPEC {
    type DataType = u32;
}

#[doc = "Product ID Register H"]
pub type ProductIdH = crate::RegValueT<ProductIdH_SPEC>;

impl ProductIdH {
    #[doc = "Product ID Indication"]
    #[inline(always)]
    pub fn proid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ProductIdH_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ProductIdH_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ProductIdH {
    #[inline(always)]
    fn default() -> ProductIdH {
        <crate::RegValueT<ProductIdH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendorIdL_SPEC;
impl crate::sealed::RegSpec for VendorIdL_SPEC {
    type DataType = u32;
}

#[doc = "Vendor ID Register L"]
pub type VendorIdL = crate::RegValueT<VendorIdL_SPEC>;

impl VendorIdL {
    #[doc = "Vendor ID Indication"]
    #[inline(always)]
    pub fn vendorid(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, VendorIdL_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            VendorIdL_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for VendorIdL {
    #[inline(always)]
    fn default() -> VendorIdL {
        <crate::RegValueT<VendorIdL_SPEC> as RegisterValue<_>>::new(0)
    }
}
