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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:12:21 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PTP Module 0 for the Ethernet Controller"]
unsafe impl ::core::marker::Send for super::Eptpc0 {}
unsafe impl ::core::marker::Sync for super::Eptpc0 {}
impl super::Eptpc0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "SYNFP Status Register"]
    #[inline(always)]
    pub const fn sysr(&self) -> &'static crate::common::Reg<self::Sysr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sysr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "SYNFP Status Notification Permission Register"]
    #[inline(always)]
    pub const fn syipr(&self) -> &'static crate::common::Reg<self::Syipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "SYNFP MAC Address Registers"]
    #[inline(always)]
    pub const fn symacru(
        &self,
    ) -> &'static crate::common::Reg<self::Symacru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Symacru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "SYNFP MAC Address Registers"]
    #[inline(always)]
    pub const fn symacrl(
        &self,
    ) -> &'static crate::common::Reg<self::Symacrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Symacrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "SYNFP LLC-CTL Value Register"]
    #[inline(always)]
    pub const fn syllcctlr(
        &self,
    ) -> &'static crate::common::Reg<self::Syllcctlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syllcctlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "SYNFP Local IP Address Register"]
    #[inline(always)]
    pub const fn syipaddrr(
        &self,
    ) -> &'static crate::common::Reg<self::Syipaddrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syipaddrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "SYNFP Specification Version Setting Register"]
    #[inline(always)]
    pub const fn syspvrr(
        &self,
    ) -> &'static crate::common::Reg<self::Syspvrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syspvrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "SYNFP Domain Number Setting Register"]
    #[inline(always)]
    pub const fn sydomr(
        &self,
    ) -> &'static crate::common::Reg<self::Sydomr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sydomr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Announce Message Flag Field Setting Register"]
    #[inline(always)]
    pub const fn anfr(&self) -> &'static crate::common::Reg<self::Anfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Anfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Sync Message Flag Field Setting Register"]
    #[inline(always)]
    pub const fn synfr(&self) -> &'static crate::common::Reg<self::Synfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Synfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Delay_Req Message Flag Field Setting Register"]
    #[inline(always)]
    pub const fn dyrqfr(
        &self,
    ) -> &'static crate::common::Reg<self::Dyrqfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dyrqfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Delay_Resp Message Flag Field Setting Register"]
    #[inline(always)]
    pub const fn dyrpfr(
        &self,
    ) -> &'static crate::common::Reg<self::Dyrpfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dyrpfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "SYNFP Local Clock ID Registers"]
    #[inline(always)]
    pub const fn sycidru(
        &self,
    ) -> &'static crate::common::Reg<self::Sycidru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sycidru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "SYNFP Local Clock ID Registers"]
    #[inline(always)]
    pub const fn sycidrl(
        &self,
    ) -> &'static crate::common::Reg<self::Sycidrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sycidrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "SYNFP Local Port Number Register"]
    #[inline(always)]
    pub const fn sypnumr(
        &self,
    ) -> &'static crate::common::Reg<self::Sypnumr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sypnumr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "SYNFP Register Value Load Directive Register"]
    #[inline(always)]
    pub const fn syrvldr(
        &self,
    ) -> &'static crate::common::Reg<self::Syrvldr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Syrvldr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "SYNFP Reception Filter Register 1"]
    #[inline(always)]
    pub const fn syrfl1r(
        &self,
    ) -> &'static crate::common::Reg<self::Syrfl1R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syrfl1R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "SYNFP Reception Filter Register 2"]
    #[inline(always)]
    pub const fn syrfl2r(
        &self,
    ) -> &'static crate::common::Reg<self::Syrfl2R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syrfl2R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "SYNFP Transmission Enable Register"]
    #[inline(always)]
    pub const fn sytrenr(
        &self,
    ) -> &'static crate::common::Reg<self::Sytrenr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sytrenr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Master Clock ID Registers"]
    #[inline(always)]
    pub const fn mtcidu(
        &self,
    ) -> &'static crate::common::Reg<self::Mtcidu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtcidu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Master Clock ID Registers"]
    #[inline(always)]
    pub const fn mtcidl(
        &self,
    ) -> &'static crate::common::Reg<self::Mtcidl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtcidl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Master clock port number register"]
    #[inline(always)]
    pub const fn mtpid(&self) -> &'static crate::common::Reg<self::Mtpid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtpid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "SYNFP Transmission Interval Setting Register"]
    #[inline(always)]
    pub const fn sytlir(
        &self,
    ) -> &'static crate::common::Reg<self::Sytlir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sytlir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "SYNFP Received logMessageInterval Value Indication Register"]
    #[inline(always)]
    pub const fn syrlir(&self) -> &'static crate::common::Reg<self::Syrlir_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Syrlir_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "offsetFromMaster Value Registers"]
    #[inline(always)]
    pub const fn ofmru(&self) -> &'static crate::common::Reg<self::Ofmru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ofmru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "offsetFromMaster Value Registers"]
    #[inline(always)]
    pub const fn ofmrl(&self) -> &'static crate::common::Reg<self::Ofmrl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ofmrl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "meanPathDelay Value Registers"]
    #[inline(always)]
    pub const fn mpdru(&self) -> &'static crate::common::Reg<self::Mpdru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mpdru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "meanPathDelay Value Registers"]
    #[inline(always)]
    pub const fn mpdrl(&self) -> &'static crate::common::Reg<self::Mpdrl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mpdrl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "grandmasterPriority Field Setting Register"]
    #[inline(always)]
    pub const fn gmpr(&self) -> &'static crate::common::Reg<self::Gmpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gmpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "grandmasterClockQuality Field Setting Register"]
    #[inline(always)]
    pub const fn gmcqr(&self) -> &'static crate::common::Reg<self::Gmcqr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gmcqr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "grandmasterIdentity Field Setting Registers"]
    #[inline(always)]
    pub const fn gmidru(
        &self,
    ) -> &'static crate::common::Reg<self::Gmidru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gmidru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[doc = "grandmasterIdentity Field Setting Registers"]
    #[inline(always)]
    pub const fn gmidrl(
        &self,
    ) -> &'static crate::common::Reg<self::Gmidrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gmidrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[doc = "currentUtcOffset/timeSource Field Setting Register"]
    #[inline(always)]
    pub const fn cuotsr(
        &self,
    ) -> &'static crate::common::Reg<self::Cuotsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cuotsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "stepsRemoved Field Setting Register"]
    #[inline(always)]
    pub const fn srr(&self) -> &'static crate::common::Reg<self::Srr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "PTP-primary Message Destination MAC Address Setting Registers"]
    #[inline(always)]
    pub const fn ppmacru(
        &self,
    ) -> &'static crate::common::Reg<self::Ppmacru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppmacru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "PTP-primary Message Destination MAC Address Setting Registers"]
    #[inline(always)]
    pub const fn ppmacrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ppmacrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppmacrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "PTP-pdelay Message MAC Address Setting Registers"]
    #[inline(always)]
    pub const fn pdmacru(
        &self,
    ) -> &'static crate::common::Reg<self::Pdmacru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdmacru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "PTP-pdelay Message MAC Address Setting Registers"]
    #[inline(always)]
    pub const fn pdmacrl(
        &self,
    ) -> &'static crate::common::Reg<self::Pdmacrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdmacrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "PTP Message EtherType Setting Register"]
    #[inline(always)]
    pub const fn petyper(
        &self,
    ) -> &'static crate::common::Reg<self::Petyper_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Petyper_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "PTP-primary Message Destination IP Address Setting Register"]
    #[inline(always)]
    pub const fn ppipr(&self) -> &'static crate::common::Reg<self::Ppipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "PTP-pdelay Message Destination IP Address Setting Register"]
    #[inline(always)]
    pub const fn pdipr(&self) -> &'static crate::common::Reg<self::Pdipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "PTP Event Message TOS Setting Register"]
    #[inline(always)]
    pub const fn petosr(
        &self,
    ) -> &'static crate::common::Reg<self::Petosr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Petosr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[doc = "PTP general Message TOS Setting Register"]
    #[inline(always)]
    pub const fn pgtosr(
        &self,
    ) -> &'static crate::common::Reg<self::Pgtosr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgtosr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[doc = "PTP-primary Message TTL Setting Register"]
    #[inline(always)]
    pub const fn ppttlr(
        &self,
    ) -> &'static crate::common::Reg<self::Ppttlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppttlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "PTP-pdelay Message TTL Setting Register"]
    #[inline(always)]
    pub const fn pdttlr(
        &self,
    ) -> &'static crate::common::Reg<self::Pdttlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pdttlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "PTP Event Message UDP Destination Port Number Setting Register"]
    #[inline(always)]
    pub const fn peudpr(
        &self,
    ) -> &'static crate::common::Reg<self::Peudpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Peudpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[doc = "PTP general Message UDP Destination Port Number Setting Register"]
    #[inline(always)]
    pub const fn pgudpr(
        &self,
    ) -> &'static crate::common::Reg<self::Pgudpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pgudpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(316usize),
            )
        }
    }

    #[doc = "Frame Reception Filter Setting Register"]
    #[inline(always)]
    pub const fn ffltr(&self) -> &'static crate::common::Reg<self::Ffltr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ffltr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "Frame Reception Filter MAC Address %s Setting Registers"]
    #[inline(always)]
    pub const fn fmacru(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fmacru_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x160usize))
        }
    }
    #[inline(always)]
    pub const fn fmac0ru(
        &self,
    ) -> &'static crate::common::Reg<self::Fmacru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fmacru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmac1ru(
        &self,
    ) -> &'static crate::common::Reg<self::Fmacru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fmacru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }

    #[doc = "Frame Reception Filter MAC Address %s Setting Registers"]
    #[inline(always)]
    pub const fn fmacrl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fmacrl_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x164usize))
        }
    }
    #[inline(always)]
    pub const fn fmac0rl(
        &self,
    ) -> &'static crate::common::Reg<self::Fmacrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fmacrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }
    #[inline(always)]
    pub const fn fmac1rl(
        &self,
    ) -> &'static crate::common::Reg<self::Fmacrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fmacrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }

    #[doc = "Asymmetric Delay Setting Registers"]
    #[inline(always)]
    pub const fn dasymru(
        &self,
    ) -> &'static crate::common::Reg<self::Dasymru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dasymru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(448usize),
            )
        }
    }

    #[doc = "Asymmetric Delay Setting Registers"]
    #[inline(always)]
    pub const fn dasymrl(
        &self,
    ) -> &'static crate::common::Reg<self::Dasymrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dasymrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(452usize),
            )
        }
    }

    #[doc = "Timestamp Latency Setting Register"]
    #[inline(always)]
    pub const fn tslatr(
        &self,
    ) -> &'static crate::common::Reg<self::Tslatr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tslatr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(456usize),
            )
        }
    }

    #[doc = "SYNFP Operation Setting Register"]
    #[inline(always)]
    pub const fn syconfr(
        &self,
    ) -> &'static crate::common::Reg<self::Syconfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syconfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(460usize),
            )
        }
    }

    #[doc = "SYNFP Frame Format Setting Register"]
    #[inline(always)]
    pub const fn syformr(
        &self,
    ) -> &'static crate::common::Reg<self::Syformr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syformr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(464usize),
            )
        }
    }

    #[doc = "Response Message Reception Timeout Register"]
    #[inline(always)]
    pub const fn rstoutr(
        &self,
    ) -> &'static crate::common::Reg<self::Rstoutr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstoutr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(468usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysr_SPEC;
impl crate::sealed::RegSpec for Sysr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Status Register"]
pub type Sysr = crate::RegValueT<Sysr_SPEC>;

impl Sysr {
    #[doc = "Generation Stop Completion Detection Flag"]
    #[inline(always)]
    pub fn gendn(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        sysr::Gendn,
        sysr::Gendn,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            sysr::Gendn,
            sysr::Gendn,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Response Stop Completion Detection Flag"]
    #[inline(always)]
    pub fn resdn(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        sysr::Resdn,
        sysr::Resdn,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            sysr::Resdn,
            sysr::Resdn,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control Information Abnormality Detection Flag"]
    #[inline(always)]
    pub fn infabt(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sysr::Infabt,
        sysr::Infabt,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sysr::Infabt,
            sysr::Infabt,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Loop Reception Detection Flag"]
    #[inline(always)]
    pub fn reclp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sysr::Reclp,
        sysr::Reclp,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sysr::Reclp,
            sysr::Reclp,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Delay_Req Reception FIFO Overflow Detection Flag"]
    #[inline(always)]
    pub fn drqovr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sysr::Drqovr,
        sysr::Drqovr,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sysr::Drqovr,
            sysr::Drqovr,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive logMessageInterval Value Out-of-Range Flag"]
    #[inline(always)]
    pub fn intdev(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sysr::Intdev,
        sysr::Intdev,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sysr::Intdev,
            sysr::Intdev,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Delay_Resp/Pdelay_Resp Reception Timeout Detection Flag"]
    #[inline(always)]
    pub fn drpto(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sysr::Drpto,
        sysr::Drpto,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sysr::Drpto,
            sysr::Drpto,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "meanPathDelay Value Update Flag"]
    #[inline(always)]
    pub fn mpdud(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sysr::Mpdud,
        sysr::Mpdud,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sysr::Mpdud,
            sysr::Mpdud,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive logMessageInterval Value Change Detection Flag"]
    #[inline(always)]
    pub fn intchg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sysr::Intchg,
        sysr::Intchg,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sysr::Intchg,
            sysr::Intchg,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "offsetFromMaster Value Update Flag"]
    #[inline(always)]
    pub fn ofmud(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sysr::Ofmud,
        sysr::Ofmud,
        Sysr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sysr::Ofmud,
            sysr::Ofmud,
            Sysr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sysr {
    #[inline(always)]
    fn default() -> Sysr {
        <crate::RegValueT<Sysr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sysr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gendn_SPEC;
    pub type Gendn = crate::EnumBitfieldStruct<u8, Gendn_SPEC>;
    impl Gendn {
        #[doc = "Stopping generation has not been completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping generation has been completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Resdn_SPEC;
    pub type Resdn = crate::EnumBitfieldStruct<u8, Resdn_SPEC>;
    impl Resdn {
        #[doc = "Stopping responses has not been completed."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stopping responses has been completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Infabt_SPEC;
    pub type Infabt = crate::EnumBitfieldStruct<u8, Infabt_SPEC>;
    impl Infabt {
        #[doc = "No abnormality in control information"]
        pub const _0: Self = Self::new(0);

        #[doc = "Abnormality in control information"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reclp_SPEC;
    pub type Reclp = crate::EnumBitfieldStruct<u8, Reclp_SPEC>;
    impl Reclp {
        #[doc = "A received message has not returned through a loop."]
        pub const _0: Self = Self::new(0);

        #[doc = "A received message has returned through a loop."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drqovr_SPEC;
    pub type Drqovr = crate::EnumBitfieldStruct<u8, Drqovr_SPEC>;
    impl Drqovr {
        #[doc = "The received Delay_Req has not caused the reception FIFO to overflow."]
        pub const _0: Self = Self::new(0);

        #[doc = "The received Delay_Req has caused the reception FIFO to overflow."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intdev_SPEC;
    pub type Intdev = crate::EnumBitfieldStruct<u8, Intdev_SPEC>;
    impl Intdev {
        #[doc = "The received logMessageInterval value is within the range."]
        pub const _0: Self = Self::new(0);

        #[doc = "The received logMessageInterval value is out of the range."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drpto_SPEC;
    pub type Drpto = crate::EnumBitfieldStruct<u8, Drpto_SPEC>;
    impl Drpto {
        #[doc = "A Delay_Resp/Pdelay_Resp timeout has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A Delay_Resp/Pdelay_Resp timeout has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpdud_SPEC;
    pub type Mpdud = crate::EnumBitfieldStruct<u8, Mpdud_SPEC>;
    impl Mpdud {
        #[doc = "The meanPathDelay value has not been updated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The meanPathDelay value has been updated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intchg_SPEC;
    pub type Intchg = crate::EnumBitfieldStruct<u8, Intchg_SPEC>;
    impl Intchg {
        #[doc = "No change in the received logMessageInterval value."]
        pub const _0: Self = Self::new(0);

        #[doc = "A change in the received logMessageInterval value."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ofmud_SPEC;
    pub type Ofmud = crate::EnumBitfieldStruct<u8, Ofmud_SPEC>;
    impl Ofmud {
        #[doc = "The offsetFromMaster value has not been updated."]
        pub const _0: Self = Self::new(0);

        #[doc = "The offsetFromMaster value has been updated."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syipr_SPEC;
impl crate::sealed::RegSpec for Syipr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Status Notification Permission Register"]
pub type Syipr = crate::RegValueT<Syipr_SPEC>;

impl Syipr {
    #[doc = "SYSR.GENDN Status Notification Permission"]
    #[inline(always)]
    pub fn gendn(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        syipr::Gendn,
        syipr::Gendn,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            syipr::Gendn,
            syipr::Gendn,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.RESDN Status Notification Permission"]
    #[inline(always)]
    pub fn resdn(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        syipr::Resdn,
        syipr::Resdn,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            syipr::Resdn,
            syipr::Resdn,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.INFABT Status Notification Permission"]
    #[inline(always)]
    pub fn infabt(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        syipr::Infabt,
        syipr::Infabt,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            syipr::Infabt,
            syipr::Infabt,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.RECLP Status Notification Permission"]
    #[inline(always)]
    pub fn reclp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        syipr::Reclp,
        syipr::Reclp,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            syipr::Reclp,
            syipr::Reclp,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.DRQOVR Status Notification Permission"]
    #[inline(always)]
    pub fn drqovr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syipr::Drqovr,
        syipr::Drqovr,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syipr::Drqovr,
            syipr::Drqovr,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.INTDEV Status Notification Permission"]
    #[inline(always)]
    pub fn intdev(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        syipr::Intdev,
        syipr::Intdev,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            syipr::Intdev,
            syipr::Intdev,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.DRPTO Status Notification Permission"]
    #[inline(always)]
    pub fn drpto(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        syipr::Drpto,
        syipr::Drpto,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            syipr::Drpto,
            syipr::Drpto,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.MPDUD Status Notification Permission"]
    #[inline(always)]
    pub fn mpdud(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syipr::Mpdud,
        syipr::Mpdud,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syipr::Mpdud,
            syipr::Mpdud,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.INTCHG Status Notification Permission"]
    #[inline(always)]
    pub fn intchg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syipr::Intchg,
        syipr::Intchg,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syipr::Intchg,
            syipr::Intchg,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYSR.OFMUD Status Notification Permission"]
    #[inline(always)]
    pub fn ofmud(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syipr::Ofmud,
        syipr::Ofmud,
        Syipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syipr::Ofmud,
            syipr::Ofmud,
            Syipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syipr {
    #[inline(always)]
    fn default() -> Syipr {
        <crate::RegValueT<Syipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syipr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gendn_SPEC;
    pub type Gendn = crate::EnumBitfieldStruct<u8, Gendn_SPEC>;
    impl Gendn {
        #[doc = "Prohibits notification of the state of SYSR.GENDN."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.GENDN."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Resdn_SPEC;
    pub type Resdn = crate::EnumBitfieldStruct<u8, Resdn_SPEC>;
    impl Resdn {
        #[doc = "Prohibits notification of the state of SYSR.RESDN."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.RESDN."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Infabt_SPEC;
    pub type Infabt = crate::EnumBitfieldStruct<u8, Infabt_SPEC>;
    impl Infabt {
        #[doc = "Prohibits notification of the state of SYSR.INFABT."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.INFABT."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reclp_SPEC;
    pub type Reclp = crate::EnumBitfieldStruct<u8, Reclp_SPEC>;
    impl Reclp {
        #[doc = "Prohibits notification of the state of SYSR.RECLP."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.RECLP."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drqovr_SPEC;
    pub type Drqovr = crate::EnumBitfieldStruct<u8, Drqovr_SPEC>;
    impl Drqovr {
        #[doc = "Prohibits notification of the state of SYSR.DRQOVR."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.DRQOVR."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intdev_SPEC;
    pub type Intdev = crate::EnumBitfieldStruct<u8, Intdev_SPEC>;
    impl Intdev {
        #[doc = "Prohibits notification of the state of SYSR.INTDEV."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.INTDEV."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drpto_SPEC;
    pub type Drpto = crate::EnumBitfieldStruct<u8, Drpto_SPEC>;
    impl Drpto {
        #[doc = "Prohibits notification of the state of SYSR.DRPTO."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.DRPTO."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpdud_SPEC;
    pub type Mpdud = crate::EnumBitfieldStruct<u8, Mpdud_SPEC>;
    impl Mpdud {
        #[doc = "Prohibits notification of the state of SYSR.MPDUD."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.MPDUD."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intchg_SPEC;
    pub type Intchg = crate::EnumBitfieldStruct<u8, Intchg_SPEC>;
    impl Intchg {
        #[doc = "Prohibits notification of the state of SYSR.INTCHG."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.INTCHG."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ofmud_SPEC;
    pub type Ofmud = crate::EnumBitfieldStruct<u8, Ofmud_SPEC>;
    impl Ofmud {
        #[doc = "Prohibits notification of the state of SYSR.OFMUD."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits notification of the state of SYSR.OFMUD."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Symacru_SPEC;
impl crate::sealed::RegSpec for Symacru_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP MAC Address Registers"]
pub type Symacru = crate::RegValueT<Symacru_SPEC>;

impl Symacru {
    #[doc = "These bits hold the setting for the higher-order 24 bits of the local MAC address."]
    #[inline(always)]
    pub fn symacru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Symacru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Symacru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Symacru {
    #[inline(always)]
    fn default() -> Symacru {
        <crate::RegValueT<Symacru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Symacrl_SPEC;
impl crate::sealed::RegSpec for Symacrl_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP MAC Address Registers"]
pub type Symacrl = crate::RegValueT<Symacrl_SPEC>;

impl Symacrl {
    #[doc = "These bits hold the setting for the lower-order 24 bits of the local MAC address."]
    #[inline(always)]
    pub fn symacrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Symacrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Symacrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Symacrl {
    #[inline(always)]
    fn default() -> Symacrl {
        <crate::RegValueT<Symacrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syllcctlr_SPEC;
impl crate::sealed::RegSpec for Syllcctlr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP LLC-CTL Value Register"]
pub type Syllcctlr = crate::RegValueT<Syllcctlr_SPEC>;

impl Syllcctlr {
    #[doc = "LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames."]
    #[inline(always)]
    pub fn ctl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Syllcctlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Syllcctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Syllcctlr {
    #[inline(always)]
    fn default() -> Syllcctlr {
        <crate::RegValueT<Syllcctlr_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syipaddrr_SPEC;
impl crate::sealed::RegSpec for Syipaddrr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Local IP Address Register"]
pub type Syipaddrr = crate::RegValueT<Syipaddrr_SPEC>;

impl Syipaddrr {
    #[doc = "These bits hold the setting for the local IP address."]
    #[inline(always)]
    pub fn syipaddrr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Syipaddrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Syipaddrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syipaddrr {
    #[inline(always)]
    fn default() -> Syipaddrr {
        <crate::RegValueT<Syipaddrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspvrr_SPEC;
impl crate::sealed::RegSpec for Syspvrr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Specification Version Setting Register"]
pub type Syspvrr = crate::RegValueT<Syspvrr_SPEC>;

impl Syspvrr {
    #[doc = "transportSpecific Field ValueThese bits are used to set the transportSpecific field value of the PTP v2 header.When a message is received, this value is compared with the transportSpecific field of the received frame.In generating messages, the value is used for the transportSpecific field of the frame for transmission.Set these bits to 0000b (IEEE 1588)."]
    #[inline(always)]
    pub fn trsp(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Syspvrr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Syspvrr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "versionPTP Field ValueThese bits are used to set the versionPTP field value of the PTP v2 header.When a message is received, this value is compared with the versionPTP field of the received frame.In generating messages, the value is used for the versionPTP field of the frame for transmission.Set these bits to 0010b (PTP v2)."]
    #[inline(always)]
    pub fn ver(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Syspvrr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Syspvrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Syspvrr {
    #[inline(always)]
    fn default() -> Syspvrr {
        <crate::RegValueT<Syspvrr_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sydomr_SPEC;
impl crate::sealed::RegSpec for Sydomr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Domain Number Setting Register"]
pub type Sydomr = crate::RegValueT<Sydomr_SPEC>;

impl Sydomr {
    #[doc = "domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission."]
    #[inline(always)]
    pub fn dnum(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sydomr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sydomr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sydomr {
    #[inline(always)]
    fn default() -> Sydomr {
        <crate::RegValueT<Sydomr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anfr_SPEC;
impl crate::sealed::RegSpec for Anfr_SPEC {
    type DataType = u32;
}

#[doc = "Announce Message Flag Field Setting Register"]
pub type Anfr = crate::RegValueT<Anfr_SPEC>;

impl Anfr {
    #[doc = "PTP profile Specific 2"]
    #[inline(always)]
    pub fn flag14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        anfr::Flag14,
        anfr::Flag14,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            anfr::Flag14,
            anfr::Flag14,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP profile Specific 1"]
    #[inline(always)]
    pub fn flag13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        anfr::Flag13,
        anfr::Flag13,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            anfr::Flag13,
            anfr::Flag13,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "unicastFlag"]
    #[inline(always)]
    pub fn flag10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        anfr::Flag10,
        anfr::Flag10,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            anfr::Flag10,
            anfr::Flag10,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "alternateMasterFlag"]
    #[inline(always)]
    pub fn flag8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        anfr::Flag8,
        anfr::Flag8,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            anfr::Flag8,
            anfr::Flag8,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        anfr::Flag5,
        anfr::Flag5,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            anfr::Flag5,
            anfr::Flag5,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        anfr::Flag4,
        anfr::Flag4,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            anfr::Flag4,
            anfr::Flag4,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        anfr::Flag3,
        anfr::Flag3,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            anfr::Flag3,
            anfr::Flag3,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        anfr::Flag2,
        anfr::Flag2,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            anfr::Flag2,
            anfr::Flag2,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        anfr::Flag1,
        anfr::Flag1,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            anfr::Flag1,
            anfr::Flag1,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS."]
    #[inline(always)]
    pub fn flag0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        anfr::Flag0,
        anfr::Flag0,
        Anfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            anfr::Flag0,
            anfr::Flag0,
            Anfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Anfr {
    #[inline(always)]
    fn default() -> Anfr {
        <crate::RegValueT<Anfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod anfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag14_SPEC;
    pub type Flag14 = crate::EnumBitfieldStruct<u8, Flag14_SPEC>;
    impl Flag14 {
        #[doc = "PTP profile Specific 2 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 2 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag13_SPEC;
    pub type Flag13 = crate::EnumBitfieldStruct<u8, Flag13_SPEC>;
    impl Flag13 {
        #[doc = "PTP profile Specific 1 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 1 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag10_SPEC;
    pub type Flag10 = crate::EnumBitfieldStruct<u8, Flag10_SPEC>;
    impl Flag10 {
        #[doc = "unicastFlag is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "unicastFlag is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag8_SPEC;
    pub type Flag8 = crate::EnumBitfieldStruct<u8, Flag8_SPEC>;
    impl Flag8 {
        #[doc = "alternateMasterFlag is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "alternateMasterFlag is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag5_SPEC;
    pub type Flag5 = crate::EnumBitfieldStruct<u8, Flag5_SPEC>;
    impl Flag5 {
        #[doc = "frequencyTraceable is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "frequencyTraceable is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag4_SPEC;
    pub type Flag4 = crate::EnumBitfieldStruct<u8, Flag4_SPEC>;
    impl Flag4 {
        #[doc = "timeTraceable is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "timeTraceable is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag3_SPEC;
    pub type Flag3 = crate::EnumBitfieldStruct<u8, Flag3_SPEC>;
    impl Flag3 {
        #[doc = "ptpTimescale is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "ptpTimescale is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag2_SPEC;
    pub type Flag2 = crate::EnumBitfieldStruct<u8, Flag2_SPEC>;
    impl Flag2 {
        #[doc = "currentUtcOffsetValid is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "currentUtcOffsetValid is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag1_SPEC;
    pub type Flag1 = crate::EnumBitfieldStruct<u8, Flag1_SPEC>;
    impl Flag1 {
        #[doc = "leap59 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "leap59 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag0_SPEC;
    pub type Flag0 = crate::EnumBitfieldStruct<u8, Flag0_SPEC>;
    impl Flag0 {
        #[doc = "leap61 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "leap61 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Synfr_SPEC;
impl crate::sealed::RegSpec for Synfr_SPEC {
    type DataType = u32;
}

#[doc = "Sync Message Flag Field Setting Register"]
pub type Synfr = crate::RegValueT<Synfr_SPEC>;

impl Synfr {
    #[doc = "PTP profile Specific 2"]
    #[inline(always)]
    pub fn flag14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        synfr::Flag14,
        synfr::Flag14,
        Synfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            synfr::Flag14,
            synfr::Flag14,
            Synfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP profile Specific 1"]
    #[inline(always)]
    pub fn flag13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        synfr::Flag13,
        synfr::Flag13,
        Synfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            synfr::Flag13,
            synfr::Flag13,
            Synfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "unicastFlag"]
    #[inline(always)]
    pub fn flag10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        synfr::Flag10,
        synfr::Flag10,
        Synfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            synfr::Flag10,
            synfr::Flag10,
            Synfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "twoStepFlag"]
    #[inline(always)]
    pub fn flag9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        synfr::Flag9,
        synfr::Flag9,
        Synfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            synfr::Flag9,
            synfr::Flag9,
            Synfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "alternateMasterFlag"]
    #[inline(always)]
    pub fn flag8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        synfr::Flag8,
        synfr::Flag8,
        Synfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            synfr::Flag8,
            synfr::Flag8,
            Synfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Synfr {
    #[inline(always)]
    fn default() -> Synfr {
        <crate::RegValueT<Synfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod synfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag14_SPEC;
    pub type Flag14 = crate::EnumBitfieldStruct<u8, Flag14_SPEC>;
    impl Flag14 {
        #[doc = "PTP profile Specific 2 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 2 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag13_SPEC;
    pub type Flag13 = crate::EnumBitfieldStruct<u8, Flag13_SPEC>;
    impl Flag13 {
        #[doc = "PTP profile Specific 1 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 1 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag10_SPEC;
    pub type Flag10 = crate::EnumBitfieldStruct<u8, Flag10_SPEC>;
    impl Flag10 {
        #[doc = "unicastFlag is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "unicastFlag is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag9_SPEC;
    pub type Flag9 = crate::EnumBitfieldStruct<u8, Flag9_SPEC>;
    impl Flag9 {
        #[doc = "Set this bit to 0 (FALSE)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag8_SPEC;
    pub type Flag8 = crate::EnumBitfieldStruct<u8, Flag8_SPEC>;
    impl Flag8 {
        #[doc = "alternateMasterFlag is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "alternateMasterFlag is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dyrqfr_SPEC;
impl crate::sealed::RegSpec for Dyrqfr_SPEC {
    type DataType = u32;
}

#[doc = "Delay_Req Message Flag Field Setting Register"]
pub type Dyrqfr = crate::RegValueT<Dyrqfr_SPEC>;

impl Dyrqfr {
    #[doc = "PTP profile Specific 2"]
    #[inline(always)]
    pub fn flag14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dyrqfr::Flag14,
        dyrqfr::Flag14,
        Dyrqfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dyrqfr::Flag14,
            dyrqfr::Flag14,
            Dyrqfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP profile Specific 1"]
    #[inline(always)]
    pub fn flag13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dyrqfr::Flag13,
        dyrqfr::Flag13,
        Dyrqfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dyrqfr::Flag13,
            dyrqfr::Flag13,
            Dyrqfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "unicastFlag"]
    #[inline(always)]
    pub fn flag10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        dyrqfr::Flag10,
        dyrqfr::Flag10,
        Dyrqfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            dyrqfr::Flag10,
            dyrqfr::Flag10,
            Dyrqfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dyrqfr {
    #[inline(always)]
    fn default() -> Dyrqfr {
        <crate::RegValueT<Dyrqfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dyrqfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag14_SPEC;
    pub type Flag14 = crate::EnumBitfieldStruct<u8, Flag14_SPEC>;
    impl Flag14 {
        #[doc = "PTP profile Specific 2 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 2 is set to TRULE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag13_SPEC;
    pub type Flag13 = crate::EnumBitfieldStruct<u8, Flag13_SPEC>;
    impl Flag13 {
        #[doc = "PTP profile Specific 1 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 1 is set to TRULE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag10_SPEC;
    pub type Flag10 = crate::EnumBitfieldStruct<u8, Flag10_SPEC>;
    impl Flag10 {
        #[doc = "unicastFlag is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "unicastFlag is set to TRULE."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dyrpfr_SPEC;
impl crate::sealed::RegSpec for Dyrpfr_SPEC {
    type DataType = u32;
}

#[doc = "Delay_Resp Message Flag Field Setting Register"]
pub type Dyrpfr = crate::RegValueT<Dyrpfr_SPEC>;

impl Dyrpfr {
    #[doc = "PTP profile Specific 2"]
    #[inline(always)]
    pub fn flag14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dyrpfr::Flag14,
        dyrpfr::Flag14,
        Dyrpfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dyrpfr::Flag14,
            dyrpfr::Flag14,
            Dyrpfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP profile Specific 1"]
    #[inline(always)]
    pub fn flag13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dyrpfr::Flag13,
        dyrpfr::Flag13,
        Dyrpfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dyrpfr::Flag13,
            dyrpfr::Flag13,
            Dyrpfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "unicastFlag"]
    #[inline(always)]
    pub fn flag10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        dyrpfr::Flag10,
        dyrpfr::Flag10,
        Dyrpfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            dyrpfr::Flag10,
            dyrpfr::Flag10,
            Dyrpfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "woStepFlag"]
    #[inline(always)]
    pub fn flag9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dyrpfr::Flag9,
        dyrpfr::Flag9,
        Dyrpfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dyrpfr::Flag9,
            dyrpfr::Flag9,
            Dyrpfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "alternateMasterFlag"]
    #[inline(always)]
    pub fn flag8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dyrpfr::Flag8,
        dyrpfr::Flag8,
        Dyrpfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dyrpfr::Flag8,
            dyrpfr::Flag8,
            Dyrpfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dyrpfr {
    #[inline(always)]
    fn default() -> Dyrpfr {
        <crate::RegValueT<Dyrpfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dyrpfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag14_SPEC;
    pub type Flag14 = crate::EnumBitfieldStruct<u8, Flag14_SPEC>;
    impl Flag14 {
        #[doc = "PTP profile Specific 2 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 2 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag13_SPEC;
    pub type Flag13 = crate::EnumBitfieldStruct<u8, Flag13_SPEC>;
    impl Flag13 {
        #[doc = "PTP profile Specific 1 is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP profile Specific 1 is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag10_SPEC;
    pub type Flag10 = crate::EnumBitfieldStruct<u8, Flag10_SPEC>;
    impl Flag10 {
        #[doc = "unicastFlag is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "unicastFlag is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag9_SPEC;
    pub type Flag9 = crate::EnumBitfieldStruct<u8, Flag9_SPEC>;
    impl Flag9 {
        #[doc = "Set this bit to 0 (FALSE)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flag8_SPEC;
    pub type Flag8 = crate::EnumBitfieldStruct<u8, Flag8_SPEC>;
    impl Flag8 {
        #[doc = "alternateMasterFlag is set to FALSE."]
        pub const _0: Self = Self::new(0);

        #[doc = "alternateMasterFlag is set to TRUE."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sycidru_SPEC;
impl crate::sealed::RegSpec for Sycidru_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Local Clock ID Registers"]
pub type Sycidru = crate::RegValueT<Sycidru_SPEC>;

impl Sycidru {
    #[doc = "These bits hold the setting for the higher-order 32 bits of the clock-ID of your port."]
    #[inline(always)]
    pub fn sycidru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sycidru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Sycidru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sycidru {
    #[inline(always)]
    fn default() -> Sycidru {
        <crate::RegValueT<Sycidru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sycidrl_SPEC;
impl crate::sealed::RegSpec for Sycidrl_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Local Clock ID Registers"]
pub type Sycidrl = crate::RegValueT<Sycidrl_SPEC>;

impl Sycidrl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the clock-ID of your port."]
    #[inline(always)]
    pub fn sycidrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sycidrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Sycidrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sycidrl {
    #[inline(always)]
    fn default() -> Sycidrl {
        <crate::RegValueT<Sycidrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sypnumr_SPEC;
impl crate::sealed::RegSpec for Sypnumr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Local Port Number Register"]
pub type Sypnumr = crate::RegValueT<Sypnumr_SPEC>;

impl Sypnumr {
    #[doc = "Local Port Number SettingThese bits hold the setting for the port number of the local port."]
    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Sypnumr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Sypnumr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sypnumr {
    #[inline(always)]
    fn default() -> Sypnumr {
        <crate::RegValueT<Sypnumr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syrvldr_SPEC;
impl crate::sealed::RegSpec for Syrvldr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Register Value Load Directive Register"]
pub type Syrvldr = crate::RegValueT<Syrvldr_SPEC>;

impl Syrvldr {
    #[doc = "Announce Message Generation Information Update"]
    #[inline(always)]
    pub fn anup(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syrvldr::Anup,
        syrvldr::Anup,
        Syrvldr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syrvldr::Anup,
            syrvldr::Anup,
            Syrvldr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "State Update"]
    #[inline(always)]
    pub fn stup(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syrvldr::Stup,
        syrvldr::Stup,
        Syrvldr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syrvldr::Stup,
            syrvldr::Stup,
            Syrvldr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "BMC Update"]
    #[inline(always)]
    pub fn bmup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syrvldr::Bmup,
        syrvldr::Bmup,
        Syrvldr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syrvldr::Bmup,
            syrvldr::Bmup,
            Syrvldr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syrvldr {
    #[inline(always)]
    fn default() -> Syrvldr {
        <crate::RegValueT<Syrvldr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syrvldr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Anup_SPEC;
    pub type Anup = crate::EnumBitfieldStruct<u8, Anup_SPEC>;
    impl Anup {
        #[doc = "no effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting this bit to 1 leads to simultaneous reflection in the Announce message generation block of the values of the registers required for the generation of Announce messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stup_SPEC;
    pub type Stup = crate::EnumBitfieldStruct<u8, Stup_SPEC>;
    impl Stup {
        #[doc = "no effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers related to the reception and transmission of PTP messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bmup_SPEC;
    pub type Bmup = crate::EnumBitfieldStruct<u8, Bmup_SPEC>;
    impl Bmup {
        #[doc = "no effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers holding the MasterClock identifying information."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syrfl1R_SPEC;
impl crate::sealed::RegSpec for Syrfl1R_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Reception Filter Register 1"]
pub type Syrfl1R = crate::RegValueT<Syrfl1R_SPEC>;

impl Syrfl1R {
    #[doc = "Pdelay_Resp_Follow_Up Message Processing"]
    #[inline(always)]
    pub fn pdfup2(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        syrfl1r::Pdfup2,
        syrfl1r::Pdfup2,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            syrfl1r::Pdfup2,
            syrfl1r::Pdfup2,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pdelay_Resp_Follow_Up Message Processing"]
    #[inline(always)]
    pub fn pdfup0(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        syrfl1r::Pdfup0,
        syrfl1r::Pdfup0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            syrfl1r::Pdfup0,
            syrfl1r::Pdfup0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pdelay_Resp Message Processing"]
    #[inline(always)]
    pub fn pdrp2(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        syrfl1r::Pdrp2,
        syrfl1r::Pdrp2,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            syrfl1r::Pdrp2,
            syrfl1r::Pdrp2,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pdelay_Resp Message Processing"]
    #[inline(always)]
    pub fn pdrp0(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        syrfl1r::Pdrp0,
        syrfl1r::Pdrp0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            syrfl1r::Pdrp0,
            syrfl1r::Pdrp0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pdelay_Req Message Processing"]
    #[inline(always)]
    pub fn pdrq2(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        syrfl1r::Pdrq2,
        syrfl1r::Pdrq2,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            syrfl1r::Pdrq2,
            syrfl1r::Pdrq2,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pdelay_Req Message Processing"]
    #[inline(always)]
    pub fn pdrq0(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        syrfl1r::Pdrq0,
        syrfl1r::Pdrq0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            syrfl1r::Pdrq0,
            syrfl1r::Pdrq0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Delay_Resp Message Processing"]
    #[inline(always)]
    pub fn drp2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        syrfl1r::Drp2,
        syrfl1r::Drp2,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            syrfl1r::Drp2,
            syrfl1r::Drp2,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Delay_Resp Message Processing"]
    #[inline(always)]
    pub fn drp0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        syrfl1r::Drp0,
        syrfl1r::Drp0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            syrfl1r::Drp0,
            syrfl1r::Drp0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Delay_Req Message Processing"]
    #[inline(always)]
    pub fn drq2(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        syrfl1r::Drq2,
        syrfl1r::Drq2,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            syrfl1r::Drq2,
            syrfl1r::Drq2,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Delay_Req Message Processing"]
    #[inline(always)]
    pub fn drq0(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        syrfl1r::Drq0,
        syrfl1r::Drq0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            syrfl1r::Drq0,
            syrfl1r::Drq0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Follow_Up Message Processing"]
    #[inline(always)]
    pub fn fup2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        syrfl1r::Fup2,
        syrfl1r::Fup2,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            syrfl1r::Fup2,
            syrfl1r::Fup2,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Follow_Up Message Processing"]
    #[inline(always)]
    pub fn fup0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        syrfl1r::Fup0,
        syrfl1r::Fup0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            syrfl1r::Fup0,
            syrfl1r::Fup0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Message Processing"]
    #[inline(always)]
    pub fn sync2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syrfl1r::Sync2,
        syrfl1r::Sync2,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syrfl1r::Sync2,
            syrfl1r::Sync2,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Message Processing"]
    #[inline(always)]
    pub fn sync0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        syrfl1r::Sync0,
        syrfl1r::Sync0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            syrfl1r::Sync0,
            syrfl1r::Sync0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Announce Message Processing"]
    #[inline(always)]
    pub fn ance0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syrfl1r::Ance0,
        syrfl1r::Ance0,
        Syrfl1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syrfl1r::Ance0,
            syrfl1r::Ance0,
            Syrfl1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syrfl1R {
    #[inline(always)]
    fn default() -> Syrfl1R {
        <crate::RegValueT<Syrfl1R_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syrfl1r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdfup2_SPEC;
    pub type Pdfup2 = crate::EnumBitfieldStruct<u8, Pdfup2_SPEC>;
    impl Pdfup2 {
        #[doc = "The SYNFP does not process messages."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SYNFP does not process messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdfup0_SPEC;
    pub type Pdfup0 = crate::EnumBitfieldStruct<u8, Pdfup0_SPEC>;
    impl Pdfup0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdrp2_SPEC;
    pub type Pdrp2 = crate::EnumBitfieldStruct<u8, Pdrp2_SPEC>;
    impl Pdrp2 {
        #[doc = "The SYNFP does not process messages."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SYNFP processes messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdrp0_SPEC;
    pub type Pdrp0 = crate::EnumBitfieldStruct<u8, Pdrp0_SPEC>;
    impl Pdrp0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdrq2_SPEC;
    pub type Pdrq2 = crate::EnumBitfieldStruct<u8, Pdrq2_SPEC>;
    impl Pdrq2 {
        #[doc = "The SYNFP does not process messages."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SYNFP processes messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdrq0_SPEC;
    pub type Pdrq0 = crate::EnumBitfieldStruct<u8, Pdrq0_SPEC>;
    impl Pdrq0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drp2_SPEC;
    pub type Drp2 = crate::EnumBitfieldStruct<u8, Drp2_SPEC>;
    impl Drp2 {
        #[doc = "The SYNFP does not process messages."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SYNFP processes messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drp0_SPEC;
    pub type Drp0 = crate::EnumBitfieldStruct<u8, Drp0_SPEC>;
    impl Drp0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drq2_SPEC;
    pub type Drq2 = crate::EnumBitfieldStruct<u8, Drq2_SPEC>;
    impl Drq2 {
        #[doc = "The SYNFP does not process messages."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SYNFP processes messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drq0_SPEC;
    pub type Drq0 = crate::EnumBitfieldStruct<u8, Drq0_SPEC>;
    impl Drq0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fup2_SPEC;
    pub type Fup2 = crate::EnumBitfieldStruct<u8, Fup2_SPEC>;
    impl Fup2 {
        #[doc = "The SYNFP does not process messages."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SYNFP processes messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fup0_SPEC;
    pub type Fup0 = crate::EnumBitfieldStruct<u8, Fup0_SPEC>;
    impl Fup0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync2_SPEC;
    pub type Sync2 = crate::EnumBitfieldStruct<u8, Sync2_SPEC>;
    impl Sync2 {
        #[doc = "The SYNFP does not process messages."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SYNFP processes messages."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync0_SPEC;
    pub type Sync0 = crate::EnumBitfieldStruct<u8, Sync0_SPEC>;
    impl Sync0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ance0_SPEC;
    pub type Ance0 = crate::EnumBitfieldStruct<u8, Ance0_SPEC>;
    impl Ance0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syrfl2R_SPEC;
impl crate::sealed::RegSpec for Syrfl2R_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Reception Filter Register 2"]
pub type Syrfl2R = crate::RegValueT<Syrfl2R_SPEC>;

impl Syrfl2R {
    #[doc = "Illegal Message Processing Setting"]
    #[inline(always)]
    pub fn ill0(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        syrfl2r::Ill0,
        syrfl2r::Ill0,
        Syrfl2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            syrfl2r::Ill0,
            syrfl2r::Ill0,
            Syrfl2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Signaling Message Processing Setting"]
    #[inline(always)]
    pub fn sig0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        syrfl2r::Sig0,
        syrfl2r::Sig0,
        Syrfl2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            syrfl2r::Sig0,
            syrfl2r::Sig0,
            Syrfl2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Management Message Processing Setting"]
    #[inline(always)]
    pub fn man0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syrfl2r::Man0,
        syrfl2r::Man0,
        Syrfl2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syrfl2r::Man0,
            syrfl2r::Man0,
            Syrfl2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syrfl2R {
    #[inline(always)]
    fn default() -> Syrfl2R {
        <crate::RegValueT<Syrfl2R_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syrfl2r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ill0_SPEC;
    pub type Ill0 = crate::EnumBitfieldStruct<u8, Ill0_SPEC>;
    impl Ill0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sig0_SPEC;
    pub type Sig0 = crate::EnumBitfieldStruct<u8, Sig0_SPEC>;
    impl Sig0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Man0_SPEC;
    pub type Man0 = crate::EnumBitfieldStruct<u8, Man0_SPEC>;
    impl Man0 {
        #[doc = "Messages are not transferred to the PTPEDMAC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Messages are transferred to the PTPEDMAC."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sytrenr_SPEC;
impl crate::sealed::RegSpec for Sytrenr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Transmission Enable Register"]
pub type Sytrenr = crate::RegValueT<Sytrenr_SPEC>;

impl Sytrenr {
    #[doc = "Pdelay_Req Message Transmission Enable"]
    #[inline(always)]
    pub fn pdrq(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sytrenr::Pdrq,
        sytrenr::Pdrq,
        Sytrenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sytrenr::Pdrq,
            sytrenr::Pdrq,
            Sytrenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Delay_Req Message Transmission Enable"]
    #[inline(always)]
    pub fn drq(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sytrenr::Drq,
        sytrenr::Drq,
        Sytrenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sytrenr::Drq,
            sytrenr::Drq,
            Sytrenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Message Transmission Enable"]
    #[inline(always)]
    pub fn sync(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sytrenr::Sync,
        sytrenr::Sync,
        Sytrenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sytrenr::Sync,
            sytrenr::Sync,
            Sytrenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Announce Message Transmission Enable"]
    #[inline(always)]
    pub fn ance(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sytrenr::Ance,
        sytrenr::Ance,
        Sytrenr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sytrenr::Ance,
            sytrenr::Ance,
            Sytrenr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sytrenr {
    #[inline(always)]
    fn default() -> Sytrenr {
        <crate::RegValueT<Sytrenr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sytrenr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdrq_SPEC;
    pub type Pdrq = crate::EnumBitfieldStruct<u8, Pdrq_SPEC>;
    impl Pdrq {
        #[doc = "Pdelay_Req messages are not transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pdelay_Req messages are transmitted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drq_SPEC;
    pub type Drq = crate::EnumBitfieldStruct<u8, Drq_SPEC>;
    impl Drq {
        #[doc = "Delay_Req messages are not transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Delay_Req messages are transmitted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync_SPEC;
    pub type Sync = crate::EnumBitfieldStruct<u8, Sync_SPEC>;
    impl Sync {
        #[doc = "Sync messages are not transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync messages are transmitted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ance_SPEC;
    pub type Ance = crate::EnumBitfieldStruct<u8, Ance_SPEC>;
    impl Ance {
        #[doc = "Announce messages are not transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Announce messages are transmitted."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtcidu_SPEC;
impl crate::sealed::RegSpec for Mtcidu_SPEC {
    type DataType = u32;
}

#[doc = "Master Clock ID Registers"]
pub type Mtcidu = crate::RegValueT<Mtcidu_SPEC>;

impl Mtcidu {
    #[doc = "These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock."]
    #[inline(always)]
    pub fn mtcidu(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtcidu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtcidu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtcidu {
    #[inline(always)]
    fn default() -> Mtcidu {
        <crate::RegValueT<Mtcidu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtcidl_SPEC;
impl crate::sealed::RegSpec for Mtcidl_SPEC {
    type DataType = u32;
}

#[doc = "Master Clock ID Registers"]
pub type Mtcidl = crate::RegValueT<Mtcidl_SPEC>;

impl Mtcidl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock."]
    #[inline(always)]
    pub fn mtcidl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtcidl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtcidl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtcidl {
    #[inline(always)]
    fn default() -> Mtcidl {
        <crate::RegValueT<Mtcidl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtpid_SPEC;
impl crate::sealed::RegSpec for Mtpid_SPEC {
    type DataType = u32;
}

#[doc = "Master clock port number register"]
pub type Mtpid = crate::RegValueT<Mtpid_SPEC>;

impl Mtpid {
    #[doc = "Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock."]
    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mtpid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mtpid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtpid {
    #[inline(always)]
    fn default() -> Mtpid {
        <crate::RegValueT<Mtpid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sytlir_SPEC;
impl crate::sealed::RegSpec for Sytlir_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Transmission Interval Setting Register"]
pub type Sytlir = crate::RegValueT<Sytlir_SPEC>;

impl Sytlir {
    #[doc = "Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages."]
    #[inline(always)]
    pub fn dreq(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Sytlir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Sytlir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages."]
    #[inline(always)]
    pub fn sync(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Sytlir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Sytlir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages."]
    #[inline(always)]
    pub fn ance(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sytlir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sytlir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sytlir {
    #[inline(always)]
    fn default() -> Sytlir {
        <crate::RegValueT<Sytlir_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syrlir_SPEC;
impl crate::sealed::RegSpec for Syrlir_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Received logMessageInterval Value Indication Register"]
pub type Syrlir = crate::RegValueT<Syrlir_SPEC>;

impl Syrlir {
    #[doc = "Delay_Resp Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Delay_Resp message."]
    #[inline(always)]
    pub fn dresp(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Syrlir_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Syrlir_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Sync Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Sync message."]
    #[inline(always)]
    pub fn sync(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Syrlir_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Syrlir_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Announce Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Announce message."]
    #[inline(always)]
    pub fn ance(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Syrlir_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Syrlir_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Syrlir {
    #[inline(always)]
    fn default() -> Syrlir {
        <crate::RegValueT<Syrlir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofmru_SPEC;
impl crate::sealed::RegSpec for Ofmru_SPEC {
    type DataType = u32;
}

#[doc = "offsetFromMaster Value Registers"]
pub type Ofmru = crate::RegValueT<Ofmru_SPEC>;

impl Ofmru {
    #[doc = "These bits indicate the higher-order 32 bits of the calculated offsetFromMaster value."]
    #[inline(always)]
    pub fn ofmru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ofmru_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ofmru_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ofmru {
    #[inline(always)]
    fn default() -> Ofmru {
        <crate::RegValueT<Ofmru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofmrl_SPEC;
impl crate::sealed::RegSpec for Ofmrl_SPEC {
    type DataType = u32;
}

#[doc = "offsetFromMaster Value Registers"]
pub type Ofmrl = crate::RegValueT<Ofmrl_SPEC>;

impl Ofmrl {
    #[doc = "These bits indicate the lower-order 32 bits of the calculated offsetFromMaster value."]
    #[inline(always)]
    pub fn ofmrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ofmrl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ofmrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ofmrl {
    #[inline(always)]
    fn default() -> Ofmrl {
        <crate::RegValueT<Ofmrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpdru_SPEC;
impl crate::sealed::RegSpec for Mpdru_SPEC {
    type DataType = u32;
}

#[doc = "meanPathDelay Value Registers"]
pub type Mpdru = crate::RegValueT<Mpdru_SPEC>;

impl Mpdru {
    #[doc = "These bits indicate the higher-order 32 bits of the calculated meanPathDelay value."]
    #[inline(always)]
    pub fn mpdru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mpdru_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mpdru_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpdru {
    #[inline(always)]
    fn default() -> Mpdru {
        <crate::RegValueT<Mpdru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpdrl_SPEC;
impl crate::sealed::RegSpec for Mpdrl_SPEC {
    type DataType = u32;
}

#[doc = "meanPathDelay Value Registers"]
pub type Mpdrl = crate::RegValueT<Mpdrl_SPEC>;

impl Mpdrl {
    #[doc = "These bits indicate the lower-order 32 bits of the calculated meanPathDelay value."]
    #[inline(always)]
    pub fn mpdrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mpdrl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mpdrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpdrl {
    #[inline(always)]
    fn default() -> Mpdrl {
        <crate::RegValueT<Mpdrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gmpr_SPEC;
impl crate::sealed::RegSpec for Gmpr_SPEC {
    type DataType = u32;
}

#[doc = "grandmasterPriority Field Setting Register"]
pub type Gmpr = crate::RegValueT<Gmpr_SPEC>;

impl Gmpr {
    #[doc = "grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages."]
    #[inline(always)]
    pub fn gmpr1(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gmpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Gmpr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages."]
    #[inline(always)]
    pub fn gmpr2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Gmpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Gmpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gmpr {
    #[inline(always)]
    fn default() -> Gmpr {
        <crate::RegValueT<Gmpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gmcqr_SPEC;
impl crate::sealed::RegSpec for Gmcqr_SPEC {
    type DataType = u32;
}

#[doc = "grandmasterClockQuality Field Setting Register"]
pub type Gmcqr = crate::RegValueT<Gmcqr_SPEC>;

impl Gmcqr {
    #[doc = "These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance"]
    #[inline(always)]
    pub fn gmcqr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gmcqr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gmcqr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gmcqr {
    #[inline(always)]
    fn default() -> Gmcqr {
        <crate::RegValueT<Gmcqr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gmidru_SPEC;
impl crate::sealed::RegSpec for Gmidru_SPEC {
    type DataType = u32;
}

#[doc = "grandmasterIdentity Field Setting Registers"]
pub type Gmidru = crate::RegValueT<Gmidru_SPEC>;

impl Gmidru {
    #[doc = "These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
    #[inline(always)]
    pub fn gmidru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gmidru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gmidru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gmidru {
    #[inline(always)]
    fn default() -> Gmidru {
        <crate::RegValueT<Gmidru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gmidrl_SPEC;
impl crate::sealed::RegSpec for Gmidrl_SPEC {
    type DataType = u32;
}

#[doc = "grandmasterIdentity Field Setting Registers"]
pub type Gmidrl = crate::RegValueT<Gmidrl_SPEC>;

impl Gmidrl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the value of the grandmasterIdentity fields of Announce messages."]
    #[inline(always)]
    pub fn gmidrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Gmidrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Gmidrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gmidrl {
    #[inline(always)]
    fn default() -> Gmidrl {
        <crate::RegValueT<Gmidrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cuotsr_SPEC;
impl crate::sealed::RegSpec for Cuotsr_SPEC {
    type DataType = u32;
}

#[doc = "currentUtcOffset/timeSource Field Setting Register"]
pub type Cuotsr = crate::RegValueT<Cuotsr_SPEC>;

impl Cuotsr {
    #[doc = "currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages."]
    #[inline(always)]
    pub fn cuto(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cuotsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cuotsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages."]
    #[inline(always)]
    pub fn tsrc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cuotsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cuotsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cuotsr {
    #[inline(always)]
    fn default() -> Cuotsr {
        <crate::RegValueT<Cuotsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srr_SPEC;
impl crate::sealed::RegSpec for Srr_SPEC {
    type DataType = u32;
}

#[doc = "stepsRemoved Field Setting Register"]
pub type Srr = crate::RegValueT<Srr_SPEC>;

impl Srr {
    #[doc = "stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages."]
    #[inline(always)]
    pub fn srmv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Srr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Srr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srr {
    #[inline(always)]
    fn default() -> Srr {
        <crate::RegValueT<Srr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppmacru_SPEC;
impl crate::sealed::RegSpec for Ppmacru_SPEC {
    type DataType = u32;
}

#[doc = "PTP-primary Message Destination MAC Address Setting Registers"]
pub type Ppmacru = crate::RegValueT<Ppmacru_SPEC>;

impl Ppmacru {
    #[doc = "These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages."]
    #[inline(always)]
    pub fn ppmacru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Ppmacru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Ppmacru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ppmacru {
    #[inline(always)]
    fn default() -> Ppmacru {
        <crate::RegValueT<Ppmacru_SPEC> as RegisterValue<_>>::new(72473)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppmacrl_SPEC;
impl crate::sealed::RegSpec for Ppmacrl_SPEC {
    type DataType = u32;
}

#[doc = "PTP-primary Message Destination MAC Address Setting Registers"]
pub type Ppmacrl = crate::RegValueT<Ppmacrl_SPEC>;

impl Ppmacrl {
    #[doc = "These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages."]
    #[inline(always)]
    pub fn ppmacrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Ppmacrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Ppmacrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ppmacrl {
    #[inline(always)]
    fn default() -> Ppmacrl {
        <crate::RegValueT<Ppmacrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmacru_SPEC;
impl crate::sealed::RegSpec for Pdmacru_SPEC {
    type DataType = u32;
}

#[doc = "PTP-pdelay Message MAC Address Setting Registers"]
pub type Pdmacru = crate::RegValueT<Pdmacru_SPEC>;

impl Pdmacru {
    #[doc = "These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages."]
    #[inline(always)]
    pub fn pdmacru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Pdmacru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Pdmacru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdmacru {
    #[inline(always)]
    fn default() -> Pdmacru {
        <crate::RegValueT<Pdmacru_SPEC> as RegisterValue<_>>::new(98498)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmacrl_SPEC;
impl crate::sealed::RegSpec for Pdmacrl_SPEC {
    type DataType = u32;
}

#[doc = "PTP-pdelay Message MAC Address Setting Registers"]
pub type Pdmacrl = crate::RegValueT<Pdmacrl_SPEC>;

impl Pdmacrl {
    #[doc = "These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages."]
    #[inline(always)]
    pub fn pdmacrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Pdmacrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Pdmacrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdmacrl {
    #[inline(always)]
    fn default() -> Pdmacrl {
        <crate::RegValueT<Pdmacrl_SPEC> as RegisterValue<_>>::new(14)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Petyper_SPEC;
impl crate::sealed::RegSpec for Petyper_SPEC {
    type DataType = u32;
}

#[doc = "PTP Message EtherType Setting Register"]
pub type Petyper = crate::RegValueT<Petyper_SPEC>;

impl Petyper {
    #[doc = "PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format."]
    #[inline(always)]
    pub fn r#type(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Petyper_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Petyper_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Petyper {
    #[inline(always)]
    fn default() -> Petyper {
        <crate::RegValueT<Petyper_SPEC> as RegisterValue<_>>::new(35063)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppipr_SPEC;
impl crate::sealed::RegSpec for Ppipr_SPEC {
    type DataType = u32;
}

#[doc = "PTP-primary Message Destination IP Address Setting Register"]
pub type Ppipr = crate::RegValueT<Ppipr_SPEC>;

impl Ppipr {
    #[doc = "These bits hold the setting for the destination IP address for PTPprimary messages."]
    #[inline(always)]
    pub fn ppipr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ppipr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ppipr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ppipr {
    #[inline(always)]
    fn default() -> Ppipr {
        <crate::RegValueT<Ppipr_SPEC> as RegisterValue<_>>::new(3758096769)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdipr_SPEC;
impl crate::sealed::RegSpec for Pdipr_SPEC {
    type DataType = u32;
}

#[doc = "PTP-pdelay Message Destination IP Address Setting Register"]
pub type Pdipr = crate::RegValueT<Pdipr_SPEC>;

impl Pdipr {
    #[doc = "These bits hold the setting for the destination IP address for PTPpdelay messages."]
    #[inline(always)]
    pub fn pdipr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Pdipr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Pdipr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdipr {
    #[inline(always)]
    fn default() -> Pdipr {
        <crate::RegValueT<Pdipr_SPEC> as RegisterValue<_>>::new(3758096491)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Petosr_SPEC;
impl crate::sealed::RegSpec for Petosr_SPEC {
    type DataType = u32;
}

#[doc = "PTP Event Message TOS Setting Register"]
pub type Petosr = crate::RegValueT<Petosr_SPEC>;

impl Petosr {
    #[doc = "PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages."]
    #[inline(always)]
    pub fn evto(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Petosr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Petosr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Petosr {
    #[inline(always)]
    fn default() -> Petosr {
        <crate::RegValueT<Petosr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgtosr_SPEC;
impl crate::sealed::RegSpec for Pgtosr_SPEC {
    type DataType = u32;
}

#[doc = "PTP general Message TOS Setting Register"]
pub type Pgtosr = crate::RegValueT<Pgtosr_SPEC>;

impl Pgtosr {
    #[doc = "PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages."]
    #[inline(always)]
    pub fn geto(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Pgtosr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Pgtosr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pgtosr {
    #[inline(always)]
    fn default() -> Pgtosr {
        <crate::RegValueT<Pgtosr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppttlr_SPEC;
impl crate::sealed::RegSpec for Ppttlr_SPEC {
    type DataType = u32;
}

#[doc = "PTP-primary Message TTL Setting Register"]
pub type Ppttlr = crate::RegValueT<Ppttlr_SPEC>;

impl Ppttlr {
    #[doc = "PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages."]
    #[inline(always)]
    pub fn prtl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ppttlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ppttlr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ppttlr {
    #[inline(always)]
    fn default() -> Ppttlr {
        <crate::RegValueT<Ppttlr_SPEC> as RegisterValue<_>>::new(128)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdttlr_SPEC;
impl crate::sealed::RegSpec for Pdttlr_SPEC {
    type DataType = u32;
}

#[doc = "PTP-pdelay Message TTL Setting Register"]
pub type Pdttlr = crate::RegValueT<Pdttlr_SPEC>;

impl Pdttlr {
    #[doc = "PTP-pdelay Message TTL Field ValueThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-pdelay messages."]
    #[inline(always)]
    pub fn pdtl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Pdttlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Pdttlr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pdttlr {
    #[inline(always)]
    fn default() -> Pdttlr {
        <crate::RegValueT<Pdttlr_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peudpr_SPEC;
impl crate::sealed::RegSpec for Peudpr_SPEC {
    type DataType = u32;
}

#[doc = "PTP Event Message UDP Destination Port Number Setting Register"]
pub type Peudpr = crate::RegValueT<Peudpr_SPEC>;

impl Peudpr {
    #[doc = "PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages."]
    #[inline(always)]
    pub fn evupt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Peudpr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Peudpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Peudpr {
    #[inline(always)]
    fn default() -> Peudpr {
        <crate::RegValueT<Peudpr_SPEC> as RegisterValue<_>>::new(319)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgudpr_SPEC;
impl crate::sealed::RegSpec for Pgudpr_SPEC {
    type DataType = u32;
}

#[doc = "PTP general Message UDP Destination Port Number Setting Register"]
pub type Pgudpr = crate::RegValueT<Pgudpr_SPEC>;

impl Pgudpr {
    #[doc = "PTP general Message Destination Port NumberThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP general messages."]
    #[inline(always)]
    pub fn geupt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Pgudpr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pgudpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pgudpr {
    #[inline(always)]
    fn default() -> Pgudpr {
        <crate::RegValueT<Pgudpr_SPEC> as RegisterValue<_>>::new(320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffltr_SPEC;
impl crate::sealed::RegSpec for Ffltr_SPEC {
    type DataType = u32;
}

#[doc = "Frame Reception Filter Setting Register"]
pub type Ffltr = crate::RegValueT<Ffltr_SPEC>;

impl Ffltr {
    #[doc = "Extended Promiscuous ModeSetting"]
    #[inline(always)]
    pub fn extprm(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ffltr::Extprm,
        ffltr::Extprm,
        Ffltr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ffltr::Extprm,
            ffltr::Extprm,
            Ffltr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0."]
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ffltr::Enb,
        ffltr::Enb,
        Ffltr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ffltr::Enb,
            ffltr::Enb,
            Ffltr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1."]
    #[inline(always)]
    pub fn prt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ffltr::Prt,
        ffltr::Prt,
        Ffltr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ffltr::Prt,
            ffltr::Prt,
            Ffltr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1."]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ffltr::Sel,
        ffltr::Sel,
        Ffltr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ffltr::Sel,
            ffltr::Sel,
            Ffltr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ffltr {
    #[inline(always)]
    fn default() -> Ffltr {
        <crate::RegValueT<Ffltr_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod ffltr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extprm_SPEC;
    pub type Extprm = crate::EnumBitfieldStruct<u8, Extprm_SPEC>;
    impl Extprm {
        #[doc = "Normal operation (unicast frames addressed to the EPTPC are received, filtering of PTP frames is applied, multicast filtering is applied, and all broadcast frames are received)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Extended promiscuous mode (all frames are received)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        #[doc = "Filtering is disabled (all multicast frames are received)."]
        pub const _0: Self = Self::new(0);

        #[doc = "See PRT and SEL bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prt_SPEC;
    pub type Prt = crate::EnumBitfieldStruct<u8, Prt_SPEC>;
    impl Prt {
        #[doc = "Do not receive multicast frames."]
        pub const _0: Self = Self::new(0);

        #[doc = "See SEL bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Only receive multicast frames matching the MAC address setting in FMAC0R(U/L)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Only receive multicast frames matching the MAC address setting in FMAC1R(U/L)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmacru_SPEC;
impl crate::sealed::RegSpec for Fmacru_SPEC {
    type DataType = u32;
}

#[doc = "Frame Reception Filter MAC Address %s Setting Registers"]
pub type Fmacru = crate::RegValueT<Fmacru_SPEC>;

impl Fmacru {
    #[doc = "These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames."]
    #[inline(always)]
    pub fn fmacru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Fmacru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Fmacru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fmacru {
    #[inline(always)]
    fn default() -> Fmacru {
        <crate::RegValueT<Fmacru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmacrl_SPEC;
impl crate::sealed::RegSpec for Fmacrl_SPEC {
    type DataType = u32;
}

#[doc = "Frame Reception Filter MAC Address %s Setting Registers"]
pub type Fmacrl = crate::RegValueT<Fmacrl_SPEC>;

impl Fmacrl {
    #[doc = "These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames."]
    #[inline(always)]
    pub fn fmacrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Fmacrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Fmacrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fmacrl {
    #[inline(always)]
    fn default() -> Fmacrl {
        <crate::RegValueT<Fmacrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dasymru_SPEC;
impl crate::sealed::RegSpec for Dasymru_SPEC {
    type DataType = u32;
}

#[doc = "Asymmetric Delay Setting Registers"]
pub type Dasymru = crate::RegValueT<Dasymru_SPEC>;

impl Dasymru {
    #[doc = "These bits hold the setting for the higher-order 16 bits of the asymmetric delay value."]
    #[inline(always)]
    pub fn dasymru(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dasymru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dasymru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dasymru {
    #[inline(always)]
    fn default() -> Dasymru {
        <crate::RegValueT<Dasymru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dasymrl_SPEC;
impl crate::sealed::RegSpec for Dasymrl_SPEC {
    type DataType = u32;
}

#[doc = "Asymmetric Delay Setting Registers"]
pub type Dasymrl = crate::RegValueT<Dasymrl_SPEC>;

impl Dasymrl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the asymmetric delay value."]
    #[inline(always)]
    pub fn dasymrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dasymrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dasymrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dasymrl {
    #[inline(always)]
    fn default() -> Dasymrl {
        <crate::RegValueT<Dasymrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tslatr_SPEC;
impl crate::sealed::RegSpec for Tslatr_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Latency Setting Register"]
pub type Tslatr = crate::RegValueT<Tslatr_SPEC>;

impl Tslatr {
    #[doc = "Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports."]
    #[inline(always)]
    pub fn ingp(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Tslatr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Tslatr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports."]
    #[inline(always)]
    pub fn egp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tslatr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tslatr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tslatr {
    #[inline(always)]
    fn default() -> Tslatr {
        <crate::RegValueT<Tslatr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syconfr_SPEC;
impl crate::sealed::RegSpec for Syconfr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Operation Setting Register"]
pub type Syconfr = crate::RegValueT<Syconfr_SPEC>;

impl Syconfr {
    #[doc = "Receive Message domainNumber Filter Disable"]
    #[inline(always)]
    pub fn fildis(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        syconfr::Fildis,
        syconfr::Fildis,
        Syconfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            syconfr::Fildis,
            syconfr::Fildis,
            Syconfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Message Transmission Bandwidth Securing Disable"]
    #[inline(always)]
    pub fn sbdis(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        syconfr::Sbdis,
        syconfr::Sbdis,
        Syconfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            syconfr::Sbdis,
            syconfr::Sbdis,
            Syconfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles)."]
    #[inline(always)]
    pub fn tcyc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Syconfr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Syconfr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Syconfr {
    #[inline(always)]
    fn default() -> Syconfr {
        <crate::RegValueT<Syconfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syconfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fildis_SPEC;
    pub type Fildis = crate::EnumBitfieldStruct<u8, Fildis_SPEC>;
    impl Fildis {
        #[doc = "Filtering conditions for the reception of PTP messages include comparison with the domainNumber field."]
        pub const _0: Self = Self::new(0);

        #[doc = "Filtering conditions for the reception of PTP messages do not include comparison with the domainNumber field."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdis_SPEC;
    pub type Sbdis = crate::EnumBitfieldStruct<u8, Sbdis_SPEC>;
    impl Sbdis {
        #[doc = "Securing of the bandwidth for the transmission of SYNC messages is enabled (transfer by the EDMAC is given lower priority)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Securing of the bandwidth for the transmission of SYNC messages is disabled (transfer by the EDMAC is given higher priority)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syformr_SPEC;
impl crate::sealed::RegSpec for Syformr_SPEC {
    type DataType = u32;
}

#[doc = "SYNFP Frame Format Setting Register"]
pub type Syformr = crate::RegValueT<Syformr_SPEC>;

impl Syformr {
    #[doc = "Ethernet Frame Format Setting"]
    #[inline(always)]
    pub fn form1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syformr::Form1,
        syformr::Form1,
        Syformr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syformr::Form1,
            syformr::Form1,
            Syformr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Ethernet/UDP Encapsulation"]
    #[inline(always)]
    pub fn form0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syformr::Form0,
        syformr::Form0,
        Syformr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syformr::Form0,
            syformr::Form0,
            Syformr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syformr {
    #[inline(always)]
    fn default() -> Syformr {
        <crate::RegValueT<Syformr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syformr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Form1_SPEC;
    pub type Form1 = crate::EnumBitfieldStruct<u8, Form1_SPEC>;
    impl Form1 {
        #[doc = "Set this bit to 0 (Ethernet II frame format)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Form0_SPEC;
    pub type Form0 = crate::EnumBitfieldStruct<u8, Form0_SPEC>;
    impl Form0 {
        #[doc = "PTP directly over Ethernet"]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP over UDP/IPv4"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstoutr_SPEC;
impl crate::sealed::RegSpec for Rstoutr_SPEC {
    type DataType = u32;
}

#[doc = "Response Message Reception Timeout Register"]
pub type Rstoutr = crate::RegValueT<Rstoutr_SPEC>;

impl Rstoutr {
    #[doc = "Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout."]
    #[inline(always)]
    pub fn rstoutr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rstoutr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rstoutr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rstoutr {
    #[inline(always)]
    fn default() -> Rstoutr {
        <crate::RegValueT<Rstoutr_SPEC> as RegisterValue<_>>::new(0)
    }
}
