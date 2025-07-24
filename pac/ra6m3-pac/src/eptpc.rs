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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:32 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PTP Module for the Ethernet Controller"]
unsafe impl ::core::marker::Send for super::Eptpc {}
unsafe impl ::core::marker::Sync for super::Eptpc {}
impl super::Eptpc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "MINT Interrupt Source Status Register"]
    #[inline(always)]
    pub const fn miesr(&self) -> &'static crate::common::Reg<self::Miesr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Miesr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "MINT Interrupt Request Permission Register"]
    #[inline(always)]
    pub const fn mieipr(
        &self,
    ) -> &'static crate::common::Reg<self::Mieipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mieipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "ELC Output/IPLS Interrupt Request Permission Register"]
    #[inline(always)]
    pub const fn elippr(
        &self,
    ) -> &'static crate::common::Reg<self::Elippr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elippr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "ELC Output/IPLS Interrupt Permission Automatic Clearing Register"]
    #[inline(always)]
    pub const fn elipacr(
        &self,
    ) -> &'static crate::common::Reg<self::Elipacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elipacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "STCA Status Register"]
    #[inline(always)]
    pub const fn stsr(&self) -> &'static crate::common::Reg<self::Stsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "STCA Status Notification Permission Register"]
    #[inline(always)]
    pub const fn stipr(&self) -> &'static crate::common::Reg<self::Stipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "STCA Clock Frequency Setting Register"]
    #[inline(always)]
    pub const fn stcfr(&self) -> &'static crate::common::Reg<self::Stcfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stcfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "STCA Operating Mode Register"]
    #[inline(always)]
    pub const fn stmr(&self) -> &'static crate::common::Reg<self::Stmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Sync Message Reception Timeout Register"]
    #[inline(always)]
    pub const fn syntor(
        &self,
    ) -> &'static crate::common::Reg<self::Syntor_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syntor_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "IPLS Interrupt Request Timer Select Register"]
    #[inline(always)]
    pub const fn iptselr(
        &self,
    ) -> &'static crate::common::Reg<self::Iptselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iptselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "MINT Interrupt Request Timer Select Register"]
    #[inline(always)]
    pub const fn mitselr(
        &self,
    ) -> &'static crate::common::Reg<self::Mitselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mitselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "ELC Output Timer Select Register"]
    #[inline(always)]
    pub const fn eltselr(
        &self,
    ) -> &'static crate::common::Reg<self::Eltselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eltselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Time Synchronization Channel Select Register"]
    #[inline(always)]
    pub const fn stchselr(
        &self,
    ) -> &'static crate::common::Reg<self::Stchselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stchselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Slave Time Synchronization Start Register"]
    #[inline(always)]
    pub const fn synstartr(
        &self,
    ) -> &'static crate::common::Reg<self::Synstartr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Synstartr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Local Time Counter Initial Value Load Directive Register"]
    #[inline(always)]
    pub const fn lcivldr(
        &self,
    ) -> &'static crate::common::Reg<self::Lcivldr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Lcivldr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Synchronization Loss Detection Threshold Registers"]
    #[inline(always)]
    pub const fn syntdaru(
        &self,
    ) -> &'static crate::common::Reg<self::Syntdaru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syntdaru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Synchronization Loss Detection Threshold Registers"]
    #[inline(always)]
    pub const fn syntdarl(
        &self,
    ) -> &'static crate::common::Reg<self::Syntdarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syntdarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Synchronization Detection Threshold Registers"]
    #[inline(always)]
    pub const fn syntdbru(
        &self,
    ) -> &'static crate::common::Reg<self::Syntdbru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syntdbru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Synchronization Detection Threshold Registers"]
    #[inline(always)]
    pub const fn syntdbrl(
        &self,
    ) -> &'static crate::common::Reg<self::Syntdbrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syntdbrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "Local Time Counter Initial Value Registers"]
    #[inline(always)]
    pub const fn lcivru(
        &self,
    ) -> &'static crate::common::Reg<self::Lcivru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcivru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[doc = "Local Time Counter Initial Value Registers"]
    #[inline(always)]
    pub const fn lcivrm(
        &self,
    ) -> &'static crate::common::Reg<self::Lcivrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcivrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[doc = "Local Time Counter Initial Value Registers"]
    #[inline(always)]
    pub const fn lcivrl(
        &self,
    ) -> &'static crate::common::Reg<self::Lcivrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcivrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "Worst 10 Acquisition Directive Register"]
    #[inline(always)]
    pub const fn getw10r(
        &self,
    ) -> &'static crate::common::Reg<self::Getw10R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Getw10R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "Positive Gradient Limit Registers"]
    #[inline(always)]
    pub const fn plimitru(
        &self,
    ) -> &'static crate::common::Reg<self::Plimitru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Plimitru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[doc = "Positive Gradient Limit Registers"]
    #[inline(always)]
    pub const fn plimitrm(
        &self,
    ) -> &'static crate::common::Reg<self::Plimitrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Plimitrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[doc = "Positive Gradient Limit Registers"]
    #[inline(always)]
    pub const fn plimitrl(
        &self,
    ) -> &'static crate::common::Reg<self::Plimitrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Plimitrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Negative Gradient Limit Registers"]
    #[inline(always)]
    pub const fn mlimitru(
        &self,
    ) -> &'static crate::common::Reg<self::Mlimitru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mlimitru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "Negative Gradient Limit Registers"]
    #[inline(always)]
    pub const fn mlimitrm(
        &self,
    ) -> &'static crate::common::Reg<self::Mlimitrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mlimitrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[doc = "Negative Gradient Limit Registers"]
    #[inline(always)]
    pub const fn mlimitrl(
        &self,
    ) -> &'static crate::common::Reg<self::Mlimitrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mlimitrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(316usize),
            )
        }
    }

    #[doc = "Statistical Information Retention Control Register"]
    #[inline(always)]
    pub const fn getinfor(
        &self,
    ) -> &'static crate::common::Reg<self::Getinfor_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Getinfor_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "Local Time Counters"]
    #[inline(always)]
    pub const fn lccvru(&self) -> &'static crate::common::Reg<self::Lccvru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Lccvru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(368usize),
            )
        }
    }

    #[doc = "Local Time Counters"]
    #[inline(always)]
    pub const fn lccvrm(&self) -> &'static crate::common::Reg<self::Lccvrm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Lccvrm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(372usize),
            )
        }
    }

    #[doc = "Local Time Counters"]
    #[inline(always)]
    pub const fn lccvrl(&self) -> &'static crate::common::Reg<self::Lccvrl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Lccvrl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(376usize),
            )
        }
    }

    #[doc = "Positive Gradient Worst 10 Value Registers"]
    #[inline(always)]
    pub const fn pw10vru(
        &self,
    ) -> &'static crate::common::Reg<self::Pw10Vru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pw10Vru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "Positive Gradient Worst 10 Value Registers"]
    #[inline(always)]
    pub const fn pw10vrm(
        &self,
    ) -> &'static crate::common::Reg<self::Pw10Vrm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pw10Vrm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Positive Gradient Worst 10 Value Registers"]
    #[inline(always)]
    pub const fn pw10vrl(
        &self,
    ) -> &'static crate::common::Reg<self::Pw10Vrl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pw10Vrl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "Negative Gradient Worst 10 Value Registers"]
    #[inline(always)]
    pub const fn mw10ru(&self) -> &'static crate::common::Reg<self::Mw10Ru_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mw10Ru_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(720usize),
            )
        }
    }

    #[doc = "Negative Gradient Worst 10 Value Registers"]
    #[inline(always)]
    pub const fn mw10rm(&self) -> &'static crate::common::Reg<self::Mw10Rm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mw10Rm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(724usize),
            )
        }
    }

    #[doc = "Negative Gradient Worst 10 Value Registers"]
    #[inline(always)]
    pub const fn mw10rl(&self) -> &'static crate::common::Reg<self::Mw10Rl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mw10Rl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(728usize),
            )
        }
    }

    #[doc = "Timer Start Time Setting Register %s"]
    #[inline(always)]
    pub const fn tmsttru(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tmsttru_SPEC, crate::common::RW>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
    #[inline(always)]
    pub const fn tmsttru0(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x300usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttru1(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x310usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttru2(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttru3(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttru4(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttru5(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x350usize),
            )
        }
    }

    #[doc = "Timer Start Time Setting Register %s"]
    #[inline(always)]
    pub const fn tmsttrl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tmsttrl_SPEC, crate::common::RW>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x304usize))
        }
    }
    #[inline(always)]
    pub const fn tmsttrl0(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x304usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttrl1(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x314usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttrl2(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttrl3(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttrl4(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmsttrl5(
        &self,
    ) -> &'static crate::common::Reg<self::Tmsttrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmsttrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x354usize),
            )
        }
    }

    #[doc = "Timer Cycle Setting Registers %s"]
    #[inline(always)]
    pub const fn tmcycr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tmcycr_SPEC, crate::common::RW>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x308usize))
        }
    }
    #[inline(always)]
    pub const fn tmcycr0(
        &self,
    ) -> &'static crate::common::Reg<self::Tmcycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmcycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x308usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmcycr1(
        &self,
    ) -> &'static crate::common::Reg<self::Tmcycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmcycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x318usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmcycr2(
        &self,
    ) -> &'static crate::common::Reg<self::Tmcycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmcycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x328usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmcycr3(
        &self,
    ) -> &'static crate::common::Reg<self::Tmcycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmcycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x338usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmcycr4(
        &self,
    ) -> &'static crate::common::Reg<self::Tmcycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmcycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x348usize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmcycr5(
        &self,
    ) -> &'static crate::common::Reg<self::Tmcycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmcycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x358usize),
            )
        }
    }

    #[doc = "Timer Pulse Width Setting Register %s"]
    #[inline(always)]
    pub const fn tmplsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Tmplsr_SPEC, crate::common::RW>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30cusize))
        }
    }
    #[inline(always)]
    pub const fn tmplsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Tmplsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmplsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmplsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Tmplsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmplsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x31cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmplsr2(
        &self,
    ) -> &'static crate::common::Reg<self::Tmplsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmplsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmplsr3(
        &self,
    ) -> &'static crate::common::Reg<self::Tmplsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmplsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x33cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmplsr4(
        &self,
    ) -> &'static crate::common::Reg<self::Tmplsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmplsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn tmplsr5(
        &self,
    ) -> &'static crate::common::Reg<self::Tmplsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmplsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x35cusize),
            )
        }
    }

    #[doc = "Timer Start Register"]
    #[inline(always)]
    pub const fn tmstartr(
        &self,
    ) -> &'static crate::common::Reg<self::Tmstartr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmstartr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(892usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miesr_SPEC;
impl crate::sealed::RegSpec for Miesr_SPEC {
    type DataType = u32;
}

#[doc = "MINT Interrupt Source Status Register"]
pub type Miesr = crate::RegValueT<Miesr_SPEC>;

impl Miesr {
    #[doc = "Pulse Output Timer 5 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        miesr::Cyc5,
        miesr::Cyc5,
        Miesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            miesr::Cyc5,
            miesr::Cyc5,
            Miesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        miesr::Cyc4,
        miesr::Cyc4,
        Miesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            miesr::Cyc4,
            miesr::Cyc4,
            Miesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        miesr::Cyc3,
        miesr::Cyc3,
        Miesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            miesr::Cyc3,
            miesr::Cyc3,
            Miesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        miesr::Cyc2,
        miesr::Cyc2,
        Miesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            miesr::Cyc2,
            miesr::Cyc2,
            Miesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        miesr::Cyc1,
        miesr::Cyc1,
        Miesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            miesr::Cyc1,
            miesr::Cyc1,
            Miesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 Rising Edge Detection Flag"]
    #[inline(always)]
    pub fn cyc0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        miesr::Cyc0,
        miesr::Cyc0,
        Miesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            miesr::Cyc0,
            miesr::Cyc0,
            Miesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNFP0 Status Flag"]
    #[inline(always)]
    pub fn sy0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        miesr::Sy0,
        miesr::Sy0,
        Miesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            miesr::Sy0,
            miesr::Sy0,
            Miesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "STCA Status Flag"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        miesr::St,
        miesr::St,
        Miesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            miesr::St,
            miesr::St,
            Miesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Miesr {
    #[inline(always)]
    fn default() -> Miesr {
        <crate::RegValueT<Miesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod miesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc5_SPEC;
    pub type Cyc5 = crate::EnumBitfieldStruct<u8, Cyc5_SPEC>;
    impl Cyc5 {
        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 5 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 5 is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc4_SPEC;
    pub type Cyc4 = crate::EnumBitfieldStruct<u8, Cyc4_SPEC>;
    impl Cyc4 {
        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 4 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 4 is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc3_SPEC;
    pub type Cyc3 = crate::EnumBitfieldStruct<u8, Cyc3_SPEC>;
    impl Cyc3 {
        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 3 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 3 is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc2_SPEC;
    pub type Cyc2 = crate::EnumBitfieldStruct<u8, Cyc2_SPEC>;
    impl Cyc2 {
        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 2 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 2 is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc1_SPEC;
    pub type Cyc1 = crate::EnumBitfieldStruct<u8, Cyc1_SPEC>;
    impl Cyc1 {
        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 1 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 1 is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc0_SPEC;
    pub type Cyc0 = crate::EnumBitfieldStruct<u8, Cyc0_SPEC>;
    impl Cyc0 {
        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 0 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A rising edge in the periodic pulse signal from pulse output timer 0 is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sy0_SPEC;
    pub type Sy0 = crate::EnumBitfieldStruct<u8, Sy0_SPEC>;
    impl Sy0 {
        #[doc = "No change in the state of the SYNFP0 module"]
        pub const _0: Self = Self::new(0);

        #[doc = "A change in the state of the SYNFP0 module"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "No change in the state of the STCA module"]
        pub const _0: Self = Self::new(0);

        #[doc = "A change in the state of the STCA module"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mieipr_SPEC;
impl crate::sealed::RegSpec for Mieipr_SPEC {
    type DataType = u32;
}

#[doc = "MINT Interrupt Request Permission Register"]
pub type Mieipr = crate::RegValueT<Mieipr_SPEC>;

impl Mieipr {
    #[doc = "Pulse Output Timer 5 Rising Edge Detection Interrupt Request Permission"]
    #[inline(always)]
    pub fn cyc5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mieipr::Cyc5,
        mieipr::Cyc5,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mieipr::Cyc5,
            mieipr::Cyc5,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 Rising Edge Detection Interrupt Request Permission"]
    #[inline(always)]
    pub fn cyc4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mieipr::Cyc4,
        mieipr::Cyc4,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mieipr::Cyc4,
            mieipr::Cyc4,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 Rising Edge Detection Interrupt Request Permission"]
    #[inline(always)]
    pub fn cyc3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mieipr::Cyc3,
        mieipr::Cyc3,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mieipr::Cyc3,
            mieipr::Cyc3,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 Rising Edge Detection Interrupt Request Permission"]
    #[inline(always)]
    pub fn cyc2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mieipr::Cyc2,
        mieipr::Cyc2,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mieipr::Cyc2,
            mieipr::Cyc2,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 Rising Edge Detection Interrupt Request Permission"]
    #[inline(always)]
    pub fn cyc1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mieipr::Cyc1,
        mieipr::Cyc1,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mieipr::Cyc1,
            mieipr::Cyc1,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 Rising Edge Detection Interrupt Request Permission"]
    #[inline(always)]
    pub fn cyc0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mieipr::Cyc0,
        mieipr::Cyc0,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mieipr::Cyc0,
            mieipr::Cyc0,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNFP0 Status Interrupt Request Permission"]
    #[inline(always)]
    pub fn sy0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mieipr::Sy0,
        mieipr::Sy0,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mieipr::Sy0,
            mieipr::Sy0,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STCA Status Interrupt Request Permission"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mieipr::St,
        mieipr::St,
        Mieipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mieipr::St,
            mieipr::St,
            Mieipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mieipr {
    #[inline(always)]
    fn default() -> Mieipr {
        <crate::RegValueT<Mieipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mieipr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc5_SPEC;
    pub type Cyc5 = crate::EnumBitfieldStruct<u8, Cyc5_SPEC>;
    impl Cyc5 {
        #[doc = "Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc4_SPEC;
    pub type Cyc4 = crate::EnumBitfieldStruct<u8, Cyc4_SPEC>;
    impl Cyc4 {
        #[doc = "Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc3_SPEC;
    pub type Cyc3 = crate::EnumBitfieldStruct<u8, Cyc3_SPEC>;
    impl Cyc3 {
        #[doc = "Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc2_SPEC;
    pub type Cyc2 = crate::EnumBitfieldStruct<u8, Cyc2_SPEC>;
    impl Cyc2 {
        #[doc = "Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc1_SPEC;
    pub type Cyc1 = crate::EnumBitfieldStruct<u8, Cyc1_SPEC>;
    impl Cyc1 {
        #[doc = "Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cyc0_SPEC;
    pub type Cyc0 = crate::EnumBitfieldStruct<u8, Cyc0_SPEC>;
    impl Cyc0 {
        #[doc = "Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sy0_SPEC;
    pub type Sy0 = crate::EnumBitfieldStruct<u8, Sy0_SPEC>;
    impl Sy0 {
        #[doc = "Prohibits the generation of MINT interrupt requests by the SYNFP0 status flag."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests by the SYNFP0 status flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "Prohibits the generation of MINT interrupt requests by the STCA status flag."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits the generation of MINT interrupt requests by the STCA status flag."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elippr_SPEC;
impl crate::sealed::RegSpec for Elippr_SPEC {
    type DataType = u32;
}

#[doc = "ELC Output/IPLS Interrupt Request Permission Register"]
pub type Elippr = crate::RegValueT<Elippr_SPEC>;

impl Elippr {
    #[doc = "Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission"]
    #[inline(always)]
    pub fn plsn(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        elippr::Plsn,
        elippr::Plsn,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            elippr::Plsn,
            elippr::Plsn,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission"]
    #[inline(always)]
    pub fn plsp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        elippr::Plsp,
        elippr::Plsp,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            elippr::Plsp,
            elippr::Plsp,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 5 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn5(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        elippr::Cycn5,
        elippr::Cycn5,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            elippr::Cycn5,
            elippr::Cycn5,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        elippr::Cycn4,
        elippr::Cycn4,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            elippr::Cycn4,
            elippr::Cycn4,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        elippr::Cycn3,
        elippr::Cycn3,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            elippr::Cycn3,
            elippr::Cycn3,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        elippr::Cycn2,
        elippr::Cycn2,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            elippr::Cycn2,
            elippr::Cycn2,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        elippr::Cycn1,
        elippr::Cycn1,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            elippr::Cycn1,
            elippr::Cycn1,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 Falling Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycn0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        elippr::Cycn0,
        elippr::Cycn0,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            elippr::Cycn0,
            elippr::Cycn0,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 5 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        elippr::Cycp5,
        elippr::Cycp5,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            elippr::Cycp5,
            elippr::Cycp5,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        elippr::Cycp4,
        elippr::Cycp4,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            elippr::Cycp4,
            elippr::Cycp4,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        elippr::Cycp3,
        elippr::Cycp3,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            elippr::Cycp3,
            elippr::Cycp3,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        elippr::Cycp2,
        elippr::Cycp2,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elippr::Cycp2,
            elippr::Cycp2,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 Rising Edge Detection Event Output Enable"]
    #[inline(always)]
    pub fn cycp1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        elippr::Cycp1,
        elippr::Cycp1,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            elippr::Cycp1,
            elippr::Cycp1,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 Rising Edge Detection Event  Output Enable"]
    #[inline(always)]
    pub fn cycp0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elippr::Cycp0,
        elippr::Cycp0,
        Elippr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elippr::Cycp0,
            elippr::Cycp0,
            Elippr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elippr {
    #[inline(always)]
    fn default() -> Elippr {
        <crate::RegValueT<Elippr_SPEC> as RegisterValue<_>>::new(16191)
    }
}
pub mod elippr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plsn_SPEC;
    pub type Plsn = crate::EnumBitfieldStruct<u8, Plsn_SPEC>;
    impl Plsn {
        #[doc = "Prohibits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plsp_SPEC;
    pub type Plsp = crate::EnumBitfieldStruct<u8, Plsp_SPEC>;
    impl Plsp {
        #[doc = "Prohibits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer."]
        pub const _0: Self = Self::new(0);

        #[doc = "Permits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn5_SPEC;
    pub type Cycn5 = crate::EnumBitfieldStruct<u8, Cycn5_SPEC>;
    impl Cycn5 {
        #[doc = "Falling edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn4_SPEC;
    pub type Cycn4 = crate::EnumBitfieldStruct<u8, Cycn4_SPEC>;
    impl Cycn4 {
        #[doc = "Falling edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn3_SPEC;
    pub type Cycn3 = crate::EnumBitfieldStruct<u8, Cycn3_SPEC>;
    impl Cycn3 {
        #[doc = "Falling edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn2_SPEC;
    pub type Cycn2 = crate::EnumBitfieldStruct<u8, Cycn2_SPEC>;
    impl Cycn2 {
        #[doc = "Falling edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn1_SPEC;
    pub type Cycn1 = crate::EnumBitfieldStruct<u8, Cycn1_SPEC>;
    impl Cycn1 {
        #[doc = "Falling edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn0_SPEC;
    pub type Cycn0 = crate::EnumBitfieldStruct<u8, Cycn0_SPEC>;
    impl Cycn0 {
        #[doc = "Falling edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Falling edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp5_SPEC;
    pub type Cycp5 = crate::EnumBitfieldStruct<u8, Cycp5_SPEC>;
    impl Cycp5 {
        #[doc = "Rising edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp4_SPEC;
    pub type Cycp4 = crate::EnumBitfieldStruct<u8, Cycp4_SPEC>;
    impl Cycp4 {
        #[doc = "Rising edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp3_SPEC;
    pub type Cycp3 = crate::EnumBitfieldStruct<u8, Cycp3_SPEC>;
    impl Cycp3 {
        #[doc = "Rising edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp2_SPEC;
    pub type Cycp2 = crate::EnumBitfieldStruct<u8, Cycp2_SPEC>;
    impl Cycp2 {
        #[doc = "Rising edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp1_SPEC;
    pub type Cycp1 = crate::EnumBitfieldStruct<u8, Cycp1_SPEC>;
    impl Cycp1 {
        #[doc = "Rising edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp0_SPEC;
    pub type Cycp0 = crate::EnumBitfieldStruct<u8, Cycp0_SPEC>;
    impl Cycp0 {
        #[doc = "Rising edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elipacr_SPEC;
impl crate::sealed::RegSpec for Elipacr_SPEC {
    type DataType = u32;
}

#[doc = "ELC Output/IPLS Interrupt Permission Automatic Clearing Register"]
pub type Elipacr = crate::RegValueT<Elipacr_SPEC>;

impl Elipacr {
    #[doc = "ELIPPR.PLSN Bit Automatic Clearing"]
    #[inline(always)]
    pub fn plsn(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        elipacr::Plsn,
        elipacr::Plsn,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            elipacr::Plsn,
            elipacr::Plsn,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.PLSP Bit Automatic Clearing"]
    #[inline(always)]
    pub fn plsp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        elipacr::Plsp,
        elipacr::Plsp,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            elipacr::Plsp,
            elipacr::Plsp,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCN5 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycn5(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        elipacr::Cycn5,
        elipacr::Cycn5,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            elipacr::Cycn5,
            elipacr::Cycn5,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCN4 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycn4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        elipacr::Cycn4,
        elipacr::Cycn4,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            elipacr::Cycn4,
            elipacr::Cycn4,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCN3 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycn3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        elipacr::Cycn3,
        elipacr::Cycn3,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            elipacr::Cycn3,
            elipacr::Cycn3,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCN2 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycn2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        elipacr::Cycn2,
        elipacr::Cycn2,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            elipacr::Cycn2,
            elipacr::Cycn2,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCN1 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycn1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        elipacr::Cycn1,
        elipacr::Cycn1,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            elipacr::Cycn1,
            elipacr::Cycn1,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCN0 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycn0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        elipacr::Cycn0,
        elipacr::Cycn0,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            elipacr::Cycn0,
            elipacr::Cycn0,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCP5 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycp5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        elipacr::Cycp5,
        elipacr::Cycp5,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            elipacr::Cycp5,
            elipacr::Cycp5,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCP4 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycp4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        elipacr::Cycp4,
        elipacr::Cycp4,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            elipacr::Cycp4,
            elipacr::Cycp4,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCP3 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycp3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        elipacr::Cycp3,
        elipacr::Cycp3,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            elipacr::Cycp3,
            elipacr::Cycp3,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCP2 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycp2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        elipacr::Cycp2,
        elipacr::Cycp2,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elipacr::Cycp2,
            elipacr::Cycp2,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCP1 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycp1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        elipacr::Cycp1,
        elipacr::Cycp1,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            elipacr::Cycp1,
            elipacr::Cycp1,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ELIPPR.CYCP0 Bit Automatic Clearing"]
    #[inline(always)]
    pub fn cycp0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        elipacr::Cycp0,
        elipacr::Cycp0,
        Elipacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            elipacr::Cycp0,
            elipacr::Cycp0,
            Elipacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Elipacr {
    #[inline(always)]
    fn default() -> Elipacr {
        <crate::RegValueT<Elipacr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elipacr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plsn_SPEC;
    pub type Plsn = crate::EnumBitfieldStruct<u8, Plsn_SPEC>;
    impl Plsn {
        #[doc = "Disables automatic clearing of the enable bit for IPLS interrupt requests in response to detection of rising edges of the pulse output timer."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for IPLS interrupt requests in response to detection of rising edges of the pulse output timer."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plsp_SPEC;
    pub type Plsp = crate::EnumBitfieldStruct<u8, Plsp_SPEC>;
    impl Plsp {
        #[doc = "Disables automatic clearing of the enable bit for IPLS interrupt requests in response to detection of rising edges of the pulse output timer."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for IPLS interrupt requests in response to detection of rising edges of the pulse output timer."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn5_SPEC;
    pub type Cycn5 = crate::EnumBitfieldStruct<u8, Cycn5_SPEC>;
    impl Cycn5 {
        #[doc = "Disables automatic clearing of the enable bit for the output of falling edges of pulse output timer 5."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of falling edges of pulse output timer 5."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn4_SPEC;
    pub type Cycn4 = crate::EnumBitfieldStruct<u8, Cycn4_SPEC>;
    impl Cycn4 {
        #[doc = "Disables automatic clearing of the enable bit for the output of falling edges of pulse output timer 4."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of falling edges of pulse output timer 4."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn3_SPEC;
    pub type Cycn3 = crate::EnumBitfieldStruct<u8, Cycn3_SPEC>;
    impl Cycn3 {
        #[doc = "Disables automatic clearing of the enable bit for the output of falling edges of pulse output timer 3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of falling edges of pulse output timer 3."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn2_SPEC;
    pub type Cycn2 = crate::EnumBitfieldStruct<u8, Cycn2_SPEC>;
    impl Cycn2 {
        #[doc = "Disables automatic clearing of the enable bit for the output of falling edges of pulse output timer 2."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of falling edges of pulse output timer 2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn1_SPEC;
    pub type Cycn1 = crate::EnumBitfieldStruct<u8, Cycn1_SPEC>;
    impl Cycn1 {
        #[doc = "Disables automatic clearing of the enable bit for the output of falling edges of pulse output timer 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of falling edges of pulse output timer 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycn0_SPEC;
    pub type Cycn0 = crate::EnumBitfieldStruct<u8, Cycn0_SPEC>;
    impl Cycn0 {
        #[doc = "Disables automatic clearing of the enable bit for the output of falling edges of pulse output timer 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of falling edges of pulse output timer 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp5_SPEC;
    pub type Cycp5 = crate::EnumBitfieldStruct<u8, Cycp5_SPEC>;
    impl Cycp5 {
        #[doc = "Disables automatic clearing of the enable bit for the output of rising edges of pulse output timer 5."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of rising edges of pulse output timer 5."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp4_SPEC;
    pub type Cycp4 = crate::EnumBitfieldStruct<u8, Cycp4_SPEC>;
    impl Cycp4 {
        #[doc = "Disables automatic clearing of the enable bit for the output of rising edges of pulse output timer 4."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of rising edges of pulse output timer 4."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp3_SPEC;
    pub type Cycp3 = crate::EnumBitfieldStruct<u8, Cycp3_SPEC>;
    impl Cycp3 {
        #[doc = "Disables automatic clearing of the enable bit for the output of rising edges of pulse output timer 3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of rising edges of pulse output timer 3."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp2_SPEC;
    pub type Cycp2 = crate::EnumBitfieldStruct<u8, Cycp2_SPEC>;
    impl Cycp2 {
        #[doc = "Disables automatic clearing of the enable bit for the output of rising edges of pulse output timer 2."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of rising edges of pulse output timer 2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp1_SPEC;
    pub type Cycp1 = crate::EnumBitfieldStruct<u8, Cycp1_SPEC>;
    impl Cycp1 {
        #[doc = "Disables automatic clearing of the enable bit for the output of rising edges of pulse output timer 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of rising edges of pulse output timer 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cycp0_SPEC;
    pub type Cycp0 = crate::EnumBitfieldStruct<u8, Cycp0_SPEC>;
    impl Cycp0 {
        #[doc = "Disables automatic clearing of the enable bit for the output of rising edges of pulse output timer 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables automatic clearing of the enable bit for the output of rising edges of pulse output timer 0."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stsr_SPEC;
impl crate::sealed::RegSpec for Stsr_SPEC {
    type DataType = u32;
}

#[doc = "STCA Status Register"]
pub type Stsr = crate::RegValueT<Stsr_SPEC>;

impl Stsr {
    #[doc = "Worst 10 Acquisition Completion Flag"]
    #[inline(always)]
    pub fn w10d(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        stsr::W10D,
        stsr::W10D,
        Stsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            stsr::W10D,
            stsr::W10D,
            Stsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sync Message Reception Timeout Detection Flag"]
    #[inline(always)]
    pub fn syntout(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        stsr::Syntout,
        stsr::Syntout,
        Stsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            stsr::Syntout,
            stsr::Syntout,
            Stsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Synchronization Loss Detection Flag"]
    #[inline(always)]
    pub fn syncout(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        stsr::Syncout,
        stsr::Syncout,
        Stsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            stsr::Syncout,
            stsr::Syncout,
            Stsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Synchronized State Detection Flag"]
    #[inline(always)]
    pub fn sync(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        stsr::Sync,
        stsr::Sync,
        Stsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            stsr::Sync,
            stsr::Sync,
            Stsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stsr {
    #[inline(always)]
    fn default() -> Stsr {
        <crate::RegValueT<Stsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct W10D_SPEC;
    pub type W10D = crate::EnumBitfieldStruct<u8, W10D_SPEC>;
    impl W10D {
        #[doc = "Ten worst values not acquired yet"]
        pub const _0: Self = Self::new(0);

        #[doc = "Ten worst values acquired"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syntout_SPEC;
    pub type Syntout = crate::EnumBitfieldStruct<u8, Syntout_SPEC>;
    impl Syntout {
        #[doc = "Sync message reception timeout not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sync message reception timeout detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncout_SPEC;
    pub type Syncout = crate::EnumBitfieldStruct<u8, Syncout_SPEC>;
    impl Syncout {
        #[doc = "Loss of synchronization not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loss of synchronization detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync_SPEC;
    pub type Sync = crate::EnumBitfieldStruct<u8, Sync_SPEC>;
    impl Sync {
        #[doc = "Synchronization not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Synchronization detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stipr_SPEC;
impl crate::sealed::RegSpec for Stipr_SPEC {
    type DataType = u32;
}

#[doc = "STCA Status Notification Permission Register"]
pub type Stipr = crate::RegValueT<Stipr_SPEC>;

impl Stipr {
    #[doc = "W10D Status Notification Enable"]
    #[inline(always)]
    pub fn w10d(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        stipr::W10D,
        stipr::W10D,
        Stipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            stipr::W10D,
            stipr::W10D,
            Stipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNTOUT Status Notification Enable"]
    #[inline(always)]
    pub fn syntout(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        stipr::Syntout,
        stipr::Syntout,
        Stipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            stipr::Syntout,
            stipr::Syntout,
            Stipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNCOUT Status Notification Enable"]
    #[inline(always)]
    pub fn syncout(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        stipr::Syncout,
        stipr::Syncout,
        Stipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            stipr::Syncout,
            stipr::Syncout,
            Stipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SYNC Status Notification Enable"]
    #[inline(always)]
    pub fn sync(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        stipr::Sync,
        stipr::Sync,
        Stipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            stipr::Sync,
            stipr::Sync,
            Stipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stipr {
    #[inline(always)]
    fn default() -> Stipr {
        <crate::RegValueT<Stipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stipr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct W10D_SPEC;
    pub type W10D = crate::EnumBitfieldStruct<u8, W10D_SPEC>;
    impl W10D {
        #[doc = "Disable notification of the STSR.W10D state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable notification of the STSR.W10D state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syntout_SPEC;
    pub type Syntout = crate::EnumBitfieldStruct<u8, Syntout_SPEC>;
    impl Syntout {
        #[doc = "Disable notification of the STSR.SYNTOUT state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable notification of the STSR.SYNTOUT state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncout_SPEC;
    pub type Syncout = crate::EnumBitfieldStruct<u8, Syncout_SPEC>;
    impl Syncout {
        #[doc = "Disable notification of the STSR.SYNCOUT state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable notification of the STSR.SYNCOUT state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sync_SPEC;
    pub type Sync = crate::EnumBitfieldStruct<u8, Sync_SPEC>;
    impl Sync {
        #[doc = "Disable notification of the STSR.SYNC state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable notification of the STSR.SYNC state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcfr_SPEC;
impl crate::sealed::RegSpec for Stcfr_SPEC {
    type DataType = u32;
}

#[doc = "STCA Clock Frequency Setting Register"]
pub type Stcfr = crate::RegValueT<Stcfr_SPEC>;

impl Stcfr {
    #[doc = "STCA Clock Frequency"]
    #[inline(always)]
    pub fn stcf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        stcfr::Stcf,
        stcfr::Stcf,
        Stcfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            stcfr::Stcf,
            stcfr::Stcf,
            Stcfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stcfr {
    #[inline(always)]
    fn default() -> Stcfr {
        <crate::RegValueT<Stcfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stcfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcf_SPEC;
    pub type Stcf = crate::EnumBitfieldStruct<u8, Stcf_SPEC>;
    impl Stcf {
        #[doc = "20MHz"]
        pub const _00: Self = Self::new(0);

        #[doc = "25MHz"]
        pub const _01: Self = Self::new(1);

        #[doc = "50MHz"]
        pub const _10: Self = Self::new(2);

        #[doc = "100 MHz"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmr_SPEC;
impl crate::sealed::RegSpec for Stmr_SPEC {
    type DataType = u32;
}

#[doc = "STCA Operating Mode Register"]
pub type Stmr = crate::RegValueT<Stmr_SPEC>;

impl Stmr {
    #[doc = "Alarm Detection Enable 1"]
    #[inline(always)]
    pub fn alen1(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        stmr::Alen1,
        stmr::Alen1,
        Stmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            stmr::Alen1,
            stmr::Alen1,
            Stmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alarm Detection Enable 0"]
    #[inline(always)]
    pub fn alen0(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        stmr::Alen0,
        stmr::Alen0,
        Stmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            stmr::Alen0,
            stmr::Alen0,
            Stmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Synchronization Loss Detection Threshold Setting"]
    #[inline(always)]
    pub fn dvth(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        stmr::Dvth,
        stmr::Dvth,
        Stmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            stmr::Dvth,
            stmr::Dvth,
            Stmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Synchronized State Detection Threshold Setting"]
    #[inline(always)]
    pub fn syth(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        stmr::Syth,
        stmr::Syth,
        Stmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            stmr::Syth,
            stmr::Syth,
            Stmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Worst 10 Acquisition Control Select"]
    #[inline(always)]
    pub fn w10s(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        stmr::W10S,
        stmr::W10S,
        Stmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            stmr::W10S,
            stmr::W10S,
            Stmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Time Synchronization Correction Mode"]
    #[inline(always)]
    pub fn cmod(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        stmr::Cmod,
        stmr::Cmod,
        Stmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            stmr::Cmod,
            stmr::Cmod,
            Stmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Worst 10 Acquisition Time"]
    #[inline(always)]
    pub fn wint(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        stmr::Wint,
        stmr::Wint,
        Stmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            stmr::Wint,
            stmr::Wint,
            Stmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stmr {
    #[inline(always)]
    fn default() -> Stmr {
        <crate::RegValueT<Stmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alen1_SPEC;
    pub type Alen1 = crate::EnumBitfieldStruct<u8, Alen1_SPEC>;
    impl Alen1 {
        #[doc = "The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "The STSR.SYNTOUT flag is not set to 1 on detection of the Sync message reception timeout interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alen0_SPEC;
    pub type Alen0 = crate::EnumBitfieldStruct<u8, Alen0_SPEC>;
    impl Alen0 {
        #[doc = "The STSR.SYNC or SYNCOUT flag is not set to 1 on detection of synchronization or loss of synchronization."]
        pub const _0: Self = Self::new(0);

        #[doc = "The STSR.SYNC or SYNCOUT flag is set to 1 on detection of synchronization or loss of synchronization."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvth_SPEC;
    pub type Dvth = crate::EnumBitfieldStruct<u8, Dvth_SPEC>;
    impl Dvth {
        #[doc = "None"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "(DVTH) time"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syth_SPEC;
    pub type Syth = crate::EnumBitfieldStruct<u8, Syth_SPEC>;
    impl Syth {
        #[doc = "None"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "(SYTH) time"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct W10S_SPEC;
    pub type W10S = crate::EnumBitfieldStruct<u8, W10S_SPEC>;
    impl W10S {
        #[doc = "Measurement is started by hardware and the value acquired in the PW10VR or MW10R register is used as the limit for filtering."]
        pub const _0: Self = Self::new(0);

        #[doc = "Measurement is started by the GETW10R.GW10 bit. Also, the value set in the PLIMITR or MLIMITR register is used as the limit for filtering."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmod_SPEC;
    pub type Cmod = crate::EnumBitfieldStruct<u8, Cmod_SPEC>;
    impl Cmod {
        #[doc = "Mode 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mode 2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wint_SPEC;
    pub type Wint = crate::EnumBitfieldStruct<u8, Wint_SPEC>;
    impl Wint {
        #[doc = "The worst 10 values are not acquired."]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Sync message reception: (WINT) time"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syntor_SPEC;
impl crate::sealed::RegSpec for Syntor_SPEC {
    type DataType = u32;
}

#[doc = "Sync Message Reception Timeout Register"]
pub type Syntor = crate::RegValueT<Syntor_SPEC>;

impl Syntor {
    #[doc = "A Sync message not being received within 1024 x n (ns), where n is the setting, leads to a timeout for reception of Sync messages, leading to the STSR.SYNTOUT flag being set to 1."]
    #[inline(always)]
    pub fn syntor(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Syntor_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Syntor_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Syntor {
    #[inline(always)]
    fn default() -> Syntor {
        <crate::RegValueT<Syntor_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptselr_SPEC;
impl crate::sealed::RegSpec for Iptselr_SPEC {
    type DataType = u32;
}

#[doc = "IPLS Interrupt Request Timer Select Register"]
pub type Iptselr = crate::RegValueT<Iptselr_SPEC>;

impl Iptselr {
    #[doc = "Pulse Output Timer 5 Select"]
    #[inline(always)]
    pub fn iptsel5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iptselr::Iptsel5,
        iptselr::Iptsel5,
        Iptselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iptselr::Iptsel5,
            iptselr::Iptsel5,
            Iptselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 Select"]
    #[inline(always)]
    pub fn iptsel4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        iptselr::Iptsel4,
        iptselr::Iptsel4,
        Iptselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            iptselr::Iptsel4,
            iptselr::Iptsel4,
            Iptselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 Select"]
    #[inline(always)]
    pub fn iptsel3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iptselr::Iptsel3,
        iptselr::Iptsel3,
        Iptselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iptselr::Iptsel3,
            iptselr::Iptsel3,
            Iptselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 Select"]
    #[inline(always)]
    pub fn iptsel2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iptselr::Iptsel2,
        iptselr::Iptsel2,
        Iptselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iptselr::Iptsel2,
            iptselr::Iptsel2,
            Iptselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 Select"]
    #[inline(always)]
    pub fn iptsel1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iptselr::Iptsel1,
        iptselr::Iptsel1,
        Iptselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iptselr::Iptsel1,
            iptselr::Iptsel1,
            Iptselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 Select"]
    #[inline(always)]
    pub fn iptsel0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iptselr::Iptsel0,
        iptselr::Iptsel0,
        Iptselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iptselr::Iptsel0,
            iptselr::Iptsel0,
            Iptselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iptselr {
    #[inline(always)]
    fn default() -> Iptselr {
        <crate::RegValueT<Iptselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iptselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iptsel5_SPEC;
    pub type Iptsel5 = crate::EnumBitfieldStruct<u8, Iptsel5_SPEC>;
    impl Iptsel5 {
        #[doc = "Pulse output timer 5 is not selected as a source of IPLS interrupt requests."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 5 is selected as a source of IPLS interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iptsel4_SPEC;
    pub type Iptsel4 = crate::EnumBitfieldStruct<u8, Iptsel4_SPEC>;
    impl Iptsel4 {
        #[doc = "Pulse output timer 4 is not selected as a source of IPLS interrupt requests."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 4 is selected as a source of IPLS interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iptsel3_SPEC;
    pub type Iptsel3 = crate::EnumBitfieldStruct<u8, Iptsel3_SPEC>;
    impl Iptsel3 {
        #[doc = "Pulse output timer 3 is not selected as a source of IPLS interrupt requests."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 3 is selected as a source of IPLS interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iptsel2_SPEC;
    pub type Iptsel2 = crate::EnumBitfieldStruct<u8, Iptsel2_SPEC>;
    impl Iptsel2 {
        #[doc = "Pulse output timer 2 is not selected as a source of IPLS interrupt requests."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 2 is selected as a source of IPLS interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iptsel1_SPEC;
    pub type Iptsel1 = crate::EnumBitfieldStruct<u8, Iptsel1_SPEC>;
    impl Iptsel1 {
        #[doc = "Pulse output timer 1 is not selected as a source of IPLS interrupt requests."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 1 is selected as a source of IPLS interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iptsel0_SPEC;
    pub type Iptsel0 = crate::EnumBitfieldStruct<u8, Iptsel0_SPEC>;
    impl Iptsel0 {
        #[doc = "Pulse output timer 0 is not selected as a source of IPLS interrupt requests."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 0 is selected as a source of IPLS interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mitselr_SPEC;
impl crate::sealed::RegSpec for Mitselr_SPEC {
    type DataType = u32;
}

#[doc = "MINT Interrupt Request Timer Select Register"]
pub type Mitselr = crate::RegValueT<Mitselr_SPEC>;

impl Mitselr {
    #[doc = "Pulse Output Timer 5 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mitselr::Minten5,
        mitselr::Minten5,
        Mitselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mitselr::Minten5,
            mitselr::Minten5,
            Mitselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mitselr::Minten4,
        mitselr::Minten4,
        Mitselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mitselr::Minten4,
            mitselr::Minten4,
            Mitselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mitselr::Minten3,
        mitselr::Minten3,
        Mitselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mitselr::Minten3,
            mitselr::Minten3,
            Mitselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mitselr::Minten2,
        mitselr::Minten2,
        Mitselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mitselr::Minten2,
            mitselr::Minten2,
            Mitselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mitselr::Minten1,
        mitselr::Minten1,
        Mitselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mitselr::Minten1,
            mitselr::Minten1,
            Mitselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 MINT Interrupt Output Enable"]
    #[inline(always)]
    pub fn minten0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mitselr::Minten0,
        mitselr::Minten0,
        Mitselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mitselr::Minten0,
            mitselr::Minten0,
            Mitselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mitselr {
    #[inline(always)]
    fn default() -> Mitselr {
        <crate::RegValueT<Mitselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mitselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Minten5_SPEC;
    pub type Minten5 = crate::EnumBitfieldStruct<u8, Minten5_SPEC>;
    impl Minten5 {
        #[doc = "Output of rising edges by pulse output timer 5 is not reflected by the MIESR.CYC5 flag as a MINT interrupt source."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of rising edges by pulse output timer 5 is reflected by the MIESR.CYC5 flag as a MINT interrupt source."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Minten4_SPEC;
    pub type Minten4 = crate::EnumBitfieldStruct<u8, Minten4_SPEC>;
    impl Minten4 {
        #[doc = "Output of rising edges by pulse output timer 4 is not reflected by the MIESR.CYC4 flag as a MINT interrupt source."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of rising edges by pulse output timer 4 is reflected by the MIESR.CYC4 flag as a MINT interrupt source."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Minten3_SPEC;
    pub type Minten3 = crate::EnumBitfieldStruct<u8, Minten3_SPEC>;
    impl Minten3 {
        #[doc = "Output of rising edges by pulse output timer 3 is not reflected by the MIESR.CYC3 flag as a MINT interrupt source."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of rising edges by pulse output timer 3 is reflected by the MIESR.CYC3 flag as a MINT interrupt source."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Minten2_SPEC;
    pub type Minten2 = crate::EnumBitfieldStruct<u8, Minten2_SPEC>;
    impl Minten2 {
        #[doc = "Output of rising edges by pulse output timer 2 is not reflected by the MIESR.CYC2 flag as a MINT interrupt source."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of rising edges by pulse output timer 2 is reflected by the MIESR.CYC2 flag as a MINT interrupt source."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Minten1_SPEC;
    pub type Minten1 = crate::EnumBitfieldStruct<u8, Minten1_SPEC>;
    impl Minten1 {
        #[doc = "Output of rising edges by pulse output timer 1 is not reflected by the MIESR.CYC1 flag as a MINT interrupt source."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of rising edges by pulse output timer 1 is reflected by the MIESR.CYC1 flag as a MINT interrupt source."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Minten0_SPEC;
    pub type Minten0 = crate::EnumBitfieldStruct<u8, Minten0_SPEC>;
    impl Minten0 {
        #[doc = "Output of rising edges by pulse output timer 0 is not reflected by the MIESR.CYC0 flag as a MINT interrupt source."]
        pub const _0: Self = Self::new(0);

        #[doc = "Output of rising edges by pulse output timer 0 is reflected by the MIESR.CYC0 flag as a MINT interrupt source."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eltselr_SPEC;
impl crate::sealed::RegSpec for Eltselr_SPEC {
    type DataType = u32;
}

#[doc = "ELC Output Timer Select Register"]
pub type Eltselr = crate::RegValueT<Eltselr_SPEC>;

impl Eltselr {
    #[doc = "Pulse Output Timer 5 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eltselr::Eltdis5,
        eltselr::Eltdis5,
        Eltselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eltselr::Eltdis5,
            eltselr::Eltdis5,
            Eltselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eltselr::Eltdis4,
        eltselr::Eltdis4,
        Eltselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eltselr::Eltdis4,
            eltselr::Eltdis4,
            Eltselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eltselr::Eltdis3,
        eltselr::Eltdis3,
        Eltselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eltselr::Eltdis3,
            eltselr::Eltdis3,
            Eltselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eltselr::Eltdis2,
        eltselr::Eltdis2,
        Eltselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eltselr::Eltdis2,
            eltselr::Eltdis2,
            Eltselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eltselr::Eltdis1,
        eltselr::Eltdis1,
        Eltselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eltselr::Eltdis1,
            eltselr::Eltdis1,
            Eltselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eltselr::Eltdis0,
        eltselr::Eltdis0,
        Eltselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eltselr::Eltdis0,
            eltselr::Eltdis0,
            Eltselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eltselr {
    #[inline(always)]
    fn default() -> Eltselr {
        <crate::RegValueT<Eltselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eltselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eltdis5_SPEC;
    pub type Eltdis5 = crate::EnumBitfieldStruct<u8, Eltdis5_SPEC>;
    impl Eltdis5 {
        #[doc = "Pulse output timer 5 is used for the generation of event signals for the ELC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 5 is not used for the generation of event signals for the ELC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eltdis4_SPEC;
    pub type Eltdis4 = crate::EnumBitfieldStruct<u8, Eltdis4_SPEC>;
    impl Eltdis4 {
        #[doc = "Pulse output timer 4 is used for the generation of event signals for the ELC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 4 is not used for the generation of event signals for the ELC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eltdis3_SPEC;
    pub type Eltdis3 = crate::EnumBitfieldStruct<u8, Eltdis3_SPEC>;
    impl Eltdis3 {
        #[doc = "Pulse output timer 3 is used for the generation of event signals for the ELC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 3 is not used for the generation of event signals for the ELC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eltdis2_SPEC;
    pub type Eltdis2 = crate::EnumBitfieldStruct<u8, Eltdis2_SPEC>;
    impl Eltdis2 {
        #[doc = "Pulse output timer 2 is used for the generation of event signals for the ELC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 2 is not used for the generation of event signals for the ELC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eltdis1_SPEC;
    pub type Eltdis1 = crate::EnumBitfieldStruct<u8, Eltdis1_SPEC>;
    impl Eltdis1 {
        #[doc = "Pulse output timer 1 is used for the generation of event signals for the ELC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 1 is not used for the generation of event signals for the ELC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eltdis0_SPEC;
    pub type Eltdis0 = crate::EnumBitfieldStruct<u8, Eltdis0_SPEC>;
    impl Eltdis0 {
        #[doc = "Pulse output timer 0 is used for the generation of event signals for the ELC."]
        pub const _0: Self = Self::new(0);

        #[doc = "Pulse output timer 0 is not used for the generation of event signals for the ELC."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stchselr_SPEC;
impl crate::sealed::RegSpec for Stchselr_SPEC {
    type DataType = u32;
}

#[doc = "Time Synchronization Channel Select Register"]
pub type Stchselr = crate::RegValueT<Stchselr_SPEC>;

impl Stchselr {
    #[doc = "Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1."]
    #[inline(always)]
    pub fn sysel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        stchselr::Sysel,
        stchselr::Sysel,
        Stchselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            stchselr::Sysel,
            stchselr::Sysel,
            Stchselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stchselr {
    #[inline(always)]
    fn default() -> Stchselr {
        <crate::RegValueT<Stchselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stchselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sysel_SPEC;
    pub type Sysel = crate::EnumBitfieldStruct<u8, Sysel_SPEC>;
    impl Sysel {
        #[doc = "Time information from synchronization from the SYNFP0 module is used."]
        pub const _0: Self = Self::new(0);

        #[doc = "Time information from synchronization from the SYNFP1 module is used."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Synstartr_SPEC;
impl crate::sealed::RegSpec for Synstartr_SPEC {
    type DataType = u32;
}

#[doc = "Slave Time Synchronization Start Register"]
pub type Synstartr = crate::RegValueT<Synstartr_SPEC>;

impl Synstartr {
    #[doc = "Slave Time Synchronization Control"]
    #[inline(always)]
    pub fn str(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        synstartr::Str,
        synstartr::Str,
        Synstartr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            synstartr::Str,
            synstartr::Str,
            Synstartr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Synstartr {
    #[inline(always)]
    fn default() -> Synstartr {
        <crate::RegValueT<Synstartr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod synstartr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Str_SPEC;
    pub type Str = crate::EnumBitfieldStruct<u8, Str_SPEC>;
    impl Str {
        #[doc = "Slave time synchronization is stopped."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave time synchronization is started."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcivldr_SPEC;
impl crate::sealed::RegSpec for Lcivldr_SPEC {
    type DataType = u32;
}

#[doc = "Local Time Counter Initial Value Load Directive Register"]
pub type Lcivldr = crate::RegValueT<Lcivldr_SPEC>;

impl Lcivldr {
    #[doc = "Local Time Counter Initial Value Load Directive"]
    #[inline(always)]
    pub fn load(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lcivldr::Load,
        lcivldr::Load,
        Lcivldr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lcivldr::Load,
            lcivldr::Load,
            Lcivldr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcivldr {
    #[inline(always)]
    fn default() -> Lcivldr {
        <crate::RegValueT<Lcivldr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcivldr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Load_SPEC;
    pub type Load = crate::EnumBitfieldStruct<u8, Load_SPEC>;
    impl Load {
        #[doc = "The initial value is not loaded into the local time counter."]
        pub const _0: Self = Self::new(0);

        #[doc = "The initial value is loaded into the local time counter."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syntdaru_SPEC;
impl crate::sealed::RegSpec for Syntdaru_SPEC {
    type DataType = u32;
}

#[doc = "Synchronization Loss Detection Threshold Registers"]
pub type Syntdaru = crate::RegValueT<Syntdaru_SPEC>;

impl Syntdaru {
    #[doc = "These bits hold the setting for the higher-order 32 bits of the threshold for detection of loss of synchronization."]
    #[inline(always)]
    pub fn syntdaru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Syntdaru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Syntdaru_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syntdaru {
    #[inline(always)]
    fn default() -> Syntdaru {
        <crate::RegValueT<Syntdaru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syntdarl_SPEC;
impl crate::sealed::RegSpec for Syntdarl_SPEC {
    type DataType = u32;
}

#[doc = "Synchronization Loss Detection Threshold Registers"]
pub type Syntdarl = crate::RegValueT<Syntdarl_SPEC>;

impl Syntdarl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the threshold for detection of loss of synchronization."]
    #[inline(always)]
    pub fn syntdarl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Syntdarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Syntdarl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syntdarl {
    #[inline(always)]
    fn default() -> Syntdarl {
        <crate::RegValueT<Syntdarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syntdbru_SPEC;
impl crate::sealed::RegSpec for Syntdbru_SPEC {
    type DataType = u32;
}

#[doc = "Synchronization Detection Threshold Registers"]
pub type Syntdbru = crate::RegValueT<Syntdbru_SPEC>;

impl Syntdbru {
    #[doc = "These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization."]
    #[inline(always)]
    pub fn syntdbru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Syntdbru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Syntdbru_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syntdbru {
    #[inline(always)]
    fn default() -> Syntdbru {
        <crate::RegValueT<Syntdbru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syntdbrl_SPEC;
impl crate::sealed::RegSpec for Syntdbrl_SPEC {
    type DataType = u32;
}

#[doc = "Synchronization Detection Threshold Registers"]
pub type Syntdbrl = crate::RegValueT<Syntdbrl_SPEC>;

impl Syntdbrl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization."]
    #[inline(always)]
    pub fn syntdbrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Syntdbrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Syntdbrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syntdbrl {
    #[inline(always)]
    fn default() -> Syntdbrl {
        <crate::RegValueT<Syntdbrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcivru_SPEC;
impl crate::sealed::RegSpec for Lcivru_SPEC {
    type DataType = u32;
}

#[doc = "Local Time Counter Initial Value Registers"]
pub type Lcivru = crate::RegValueT<Lcivru_SPEC>;

impl Lcivru {
    #[doc = "These bits hold the setting for the higher-order 16 bits of the integer portion of the initial value for the local timer counter."]
    #[inline(always)]
    pub fn lcivru(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Lcivru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Lcivru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lcivru {
    #[inline(always)]
    fn default() -> Lcivru {
        <crate::RegValueT<Lcivru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcivrm_SPEC;
impl crate::sealed::RegSpec for Lcivrm_SPEC {
    type DataType = u32;
}

#[doc = "Local Time Counter Initial Value Registers"]
pub type Lcivrm = crate::RegValueT<Lcivrm_SPEC>;

impl Lcivrm {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter."]
    #[inline(always)]
    pub fn lcivrm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lcivrm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lcivrm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lcivrm {
    #[inline(always)]
    fn default() -> Lcivrm {
        <crate::RegValueT<Lcivrm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcivrl_SPEC;
impl crate::sealed::RegSpec for Lcivrl_SPEC {
    type DataType = u32;
}

#[doc = "Local Time Counter Initial Value Registers"]
pub type Lcivrl = crate::RegValueT<Lcivrl_SPEC>;

impl Lcivrl {
    #[doc = "These bits hold the setting for the fractional portion of the initial value of the local timer counter in nanoseconds."]
    #[inline(always)]
    pub fn lcivrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lcivrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lcivrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lcivrl {
    #[inline(always)]
    fn default() -> Lcivrl {
        <crate::RegValueT<Lcivrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Getw10R_SPEC;
impl crate::sealed::RegSpec for Getw10R_SPEC {
    type DataType = u32;
}

#[doc = "Worst 10 Acquisition Directive Register"]
pub type Getw10R = crate::RegValueT<Getw10R_SPEC>;

impl Getw10R {
    #[doc = "Worst 10 Acquisition Directive"]
    #[inline(always)]
    pub fn gw10(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        getw10r::Gw10,
        getw10r::Gw10,
        Getw10R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            getw10r::Gw10,
            getw10r::Gw10,
            Getw10R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Getw10R {
    #[inline(always)]
    fn default() -> Getw10R {
        <crate::RegValueT<Getw10R_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod getw10r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gw10_SPEC;
    pub type Gw10 = crate::EnumBitfieldStruct<u8, Gw10_SPEC>;
    impl Gw10 {
        #[doc = "The worst-10 values are not acquired."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts acquisition of the worst-10 values."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plimitru_SPEC;
impl crate::sealed::RegSpec for Plimitru_SPEC {
    type DataType = u32;
}

#[doc = "Positive Gradient Limit Registers"]
pub type Plimitru = crate::RegValueT<Plimitru_SPEC>;

impl Plimitru {
    #[doc = "These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient."]
    #[inline(always)]
    pub fn plimitru(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, u32, Plimitru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            Plimitru_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Plimitru {
    #[inline(always)]
    fn default() -> Plimitru {
        <crate::RegValueT<Plimitru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plimitrm_SPEC;
impl crate::sealed::RegSpec for Plimitrm_SPEC {
    type DataType = u32;
}

#[doc = "Positive Gradient Limit Registers"]
pub type Plimitrm = crate::RegValueT<Plimitrm_SPEC>;

impl Plimitrm {
    #[doc = "These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient."]
    #[inline(always)]
    pub fn plimitrm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Plimitrm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Plimitrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Plimitrm {
    #[inline(always)]
    fn default() -> Plimitrm {
        <crate::RegValueT<Plimitrm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plimitrl_SPEC;
impl crate::sealed::RegSpec for Plimitrl_SPEC {
    type DataType = u32;
}

#[doc = "Positive Gradient Limit Registers"]
pub type Plimitrl = crate::RegValueT<Plimitrl_SPEC>;

impl Plimitrl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient."]
    #[inline(always)]
    pub fn plimitrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Plimitrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Plimitrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Plimitrl {
    #[inline(always)]
    fn default() -> Plimitrl {
        <crate::RegValueT<Plimitrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mlimitru_SPEC;
impl crate::sealed::RegSpec for Mlimitru_SPEC {
    type DataType = u32;
}

#[doc = "Negative Gradient Limit Registers"]
pub type Mlimitru = crate::RegValueT<Mlimitru_SPEC>;

impl Mlimitru {
    #[doc = "These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient."]
    #[inline(always)]
    pub fn mlimitru(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, u32, Mlimitru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7fffffff,
            1,
            0,
            u32,
            u32,
            Mlimitru_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mlimitru {
    #[inline(always)]
    fn default() -> Mlimitru {
        <crate::RegValueT<Mlimitru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mlimitrm_SPEC;
impl crate::sealed::RegSpec for Mlimitrm_SPEC {
    type DataType = u32;
}

#[doc = "Negative Gradient Limit Registers"]
pub type Mlimitrm = crate::RegValueT<Mlimitrm_SPEC>;

impl Mlimitrm {
    #[doc = "These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient."]
    #[inline(always)]
    pub fn mlimitrm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mlimitrm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Mlimitrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mlimitrm {
    #[inline(always)]
    fn default() -> Mlimitrm {
        <crate::RegValueT<Mlimitrm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mlimitrl_SPEC;
impl crate::sealed::RegSpec for Mlimitrl_SPEC {
    type DataType = u32;
}

#[doc = "Negative Gradient Limit Registers"]
pub type Mlimitrl = crate::RegValueT<Mlimitrl_SPEC>;

impl Mlimitrl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient."]
    #[inline(always)]
    pub fn mlimitrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mlimitrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Mlimitrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mlimitrl {
    #[inline(always)]
    fn default() -> Mlimitrl {
        <crate::RegValueT<Mlimitrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Getinfor_SPEC;
impl crate::sealed::RegSpec for Getinfor_SPEC {
    type DataType = u32;
}

#[doc = "Statistical Information Retention Control Register"]
pub type Getinfor = crate::RegValueT<Getinfor_SPEC>;

impl Getinfor {
    #[doc = "Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed."]
    #[inline(always)]
    pub fn info(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        getinfor::Info,
        getinfor::Info,
        Getinfor_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            getinfor::Info,
            getinfor::Info,
            Getinfor_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Getinfor {
    #[inline(always)]
    fn default() -> Getinfor {
        <crate::RegValueT<Getinfor_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod getinfor {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Info_SPEC;
    pub type Info = crate::EnumBitfieldStruct<u8, Info_SPEC>;
    impl Info {
        #[doc = "Has no effects.(write) / Information retention is completed.(read)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Information is retained.(write) / Processing for information retention is in progress.(read)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lccvru_SPEC;
impl crate::sealed::RegSpec for Lccvru_SPEC {
    type DataType = u32;
}

#[doc = "Local Time Counters"]
pub type Lccvru = crate::RegValueT<Lccvru_SPEC>;

impl Lccvru {
    #[doc = "These bits are for reading the higher-order 16 bits of the integer portion of the local timer counter\'s value."]
    #[inline(always)]
    pub fn lccvru(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Lccvru_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Lccvru_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Lccvru {
    #[inline(always)]
    fn default() -> Lccvru {
        <crate::RegValueT<Lccvru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lccvrm_SPEC;
impl crate::sealed::RegSpec for Lccvrm_SPEC {
    type DataType = u32;
}

#[doc = "Local Time Counters"]
pub type Lccvrm = crate::RegValueT<Lccvrm_SPEC>;

impl Lccvrm {
    #[doc = "These bits are for reading the lower-order 32 bits of the integer portion of the local timer counter\'s value."]
    #[inline(always)]
    pub fn lccvrm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lccvrm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lccvrm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Lccvrm {
    #[inline(always)]
    fn default() -> Lccvrm {
        <crate::RegValueT<Lccvrm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lccvrl_SPEC;
impl crate::sealed::RegSpec for Lccvrl_SPEC {
    type DataType = u32;
}

#[doc = "Local Time Counters"]
pub type Lccvrl = crate::RegValueT<Lccvrl_SPEC>;

impl Lccvrl {
    #[doc = "These bits are for reading the fractional portion of the local timer counter\'s value (in nanoseconds)."]
    #[inline(always)]
    pub fn lccvrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lccvrl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lccvrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Lccvrl {
    #[inline(always)]
    fn default() -> Lccvrl {
        <crate::RegValueT<Lccvrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pw10Vru_SPEC;
impl crate::sealed::RegSpec for Pw10Vru_SPEC {
    type DataType = u32;
}

#[doc = "Positive Gradient Worst 10 Value Registers"]
pub type Pw10Vru = crate::RegValueT<Pw10Vru_SPEC>;

impl Pw10Vru {
    #[doc = "These bits are for reading the higher-order 32 bits of the positive gradient value."]
    #[inline(always)]
    pub fn pw10vru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Pw10Vru_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Pw10Vru_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pw10Vru {
    #[inline(always)]
    fn default() -> Pw10Vru {
        <crate::RegValueT<Pw10Vru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pw10Vrm_SPEC;
impl crate::sealed::RegSpec for Pw10Vrm_SPEC {
    type DataType = u32;
}

#[doc = "Positive Gradient Worst 10 Value Registers"]
pub type Pw10Vrm = crate::RegValueT<Pw10Vrm_SPEC>;

impl Pw10Vrm {
    #[doc = "These bits are for reading the middle-order 32 bits of the positive gradient value."]
    #[inline(always)]
    pub fn pw10vrm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Pw10Vrm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Pw10Vrm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pw10Vrm {
    #[inline(always)]
    fn default() -> Pw10Vrm {
        <crate::RegValueT<Pw10Vrm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pw10Vrl_SPEC;
impl crate::sealed::RegSpec for Pw10Vrl_SPEC {
    type DataType = u32;
}

#[doc = "Positive Gradient Worst 10 Value Registers"]
pub type Pw10Vrl = crate::RegValueT<Pw10Vrl_SPEC>;

impl Pw10Vrl {
    #[doc = "These bits are for reading the lower-order 32 bits of the positive gradient value."]
    #[inline(always)]
    pub fn pw10vrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Pw10Vrl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Pw10Vrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pw10Vrl {
    #[inline(always)]
    fn default() -> Pw10Vrl {
        <crate::RegValueT<Pw10Vrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mw10Ru_SPEC;
impl crate::sealed::RegSpec for Mw10Ru_SPEC {
    type DataType = u32;
}

#[doc = "Negative Gradient Worst 10 Value Registers"]
pub type Mw10Ru = crate::RegValueT<Mw10Ru_SPEC>;

impl Mw10Ru {
    #[doc = "These bits are for reading the higher-order 32 bits of the negative gradient value."]
    #[inline(always)]
    pub fn mw10ru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mw10Ru_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mw10Ru_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mw10Ru {
    #[inline(always)]
    fn default() -> Mw10Ru {
        <crate::RegValueT<Mw10Ru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mw10Rm_SPEC;
impl crate::sealed::RegSpec for Mw10Rm_SPEC {
    type DataType = u32;
}

#[doc = "Negative Gradient Worst 10 Value Registers"]
pub type Mw10Rm = crate::RegValueT<Mw10Rm_SPEC>;

impl Mw10Rm {
    #[doc = "These bits are for reading the middle-order 32 bits of the negative gradient value."]
    #[inline(always)]
    pub fn mw10rm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mw10Rm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mw10Rm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mw10Rm {
    #[inline(always)]
    fn default() -> Mw10Rm {
        <crate::RegValueT<Mw10Rm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mw10Rl_SPEC;
impl crate::sealed::RegSpec for Mw10Rl_SPEC {
    type DataType = u32;
}

#[doc = "Negative Gradient Worst 10 Value Registers"]
pub type Mw10Rl = crate::RegValueT<Mw10Rl_SPEC>;

impl Mw10Rl {
    #[doc = "These bits are for reading the lower-order 32 bits of the negative gradient value."]
    #[inline(always)]
    pub fn mw10rl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mw10Rl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mw10Rl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mw10Rl {
    #[inline(always)]
    fn default() -> Mw10Rl {
        <crate::RegValueT<Mw10Rl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmsttru_SPEC;
impl crate::sealed::RegSpec for Tmsttru_SPEC {
    type DataType = u32;
}

#[doc = "Timer Start Time Setting Register %s"]
pub type Tmsttru = crate::RegValueT<Tmsttru_SPEC>;

impl Tmsttru {
    #[doc = "These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds."]
    #[inline(always)]
    pub fn tmsttru(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tmsttru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tmsttru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tmsttru {
    #[inline(always)]
    fn default() -> Tmsttru {
        <crate::RegValueT<Tmsttru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmsttrl_SPEC;
impl crate::sealed::RegSpec for Tmsttrl_SPEC {
    type DataType = u32;
}

#[doc = "Timer Start Time Setting Register %s"]
pub type Tmsttrl = crate::RegValueT<Tmsttrl_SPEC>;

impl Tmsttrl {
    #[doc = "These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds."]
    #[inline(always)]
    pub fn tmsttrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tmsttrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tmsttrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tmsttrl {
    #[inline(always)]
    fn default() -> Tmsttrl {
        <crate::RegValueT<Tmsttrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmcycr_SPEC;
impl crate::sealed::RegSpec for Tmcycr_SPEC {
    type DataType = u32;
}

#[doc = "Timer Cycle Setting Registers %s"]
pub type Tmcycr = crate::RegValueT<Tmcycr_SPEC>;

impl Tmcycr {
    #[doc = "These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock."]
    #[inline(always)]
    pub fn tmcycr(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffffff, 1, 0, u32, u32, Tmcycr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fffffff,1,0,u32,u32,Tmcycr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tmcycr {
    #[inline(always)]
    fn default() -> Tmcycr {
        <crate::RegValueT<Tmcycr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmplsr_SPEC;
impl crate::sealed::RegSpec for Tmplsr_SPEC {
    type DataType = u32;
}

#[doc = "Timer Pulse Width Setting Register %s"]
pub type Tmplsr = crate::RegValueT<Tmplsr_SPEC>;

impl Tmplsr {
    #[doc = "These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock."]
    #[inline(always)]
    pub fn tmplsr(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Tmplsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Tmplsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tmplsr {
    #[inline(always)]
    fn default() -> Tmplsr {
        <crate::RegValueT<Tmplsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmstartr_SPEC;
impl crate::sealed::RegSpec for Tmstartr_SPEC {
    type DataType = u32;
}

#[doc = "Timer Start Register"]
pub type Tmstartr = crate::RegValueT<Tmstartr_SPEC>;

impl Tmstartr {
    #[doc = "Pulse Output Timer 5 Start"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        tmstartr::En5,
        tmstartr::En5,
        Tmstartr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tmstartr::En5,
            tmstartr::En5,
            Tmstartr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 4 Start"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tmstartr::En4,
        tmstartr::En4,
        Tmstartr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tmstartr::En4,
            tmstartr::En4,
            Tmstartr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 3 Start"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        tmstartr::En3,
        tmstartr::En3,
        Tmstartr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tmstartr::En3,
            tmstartr::En3,
            Tmstartr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 2 Start"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tmstartr::En2,
        tmstartr::En2,
        Tmstartr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tmstartr::En2,
            tmstartr::En2,
            Tmstartr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 1 Start"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tmstartr::En1,
        tmstartr::En1,
        Tmstartr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tmstartr::En1,
            tmstartr::En1,
            Tmstartr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pulse Output Timer 0 Start"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tmstartr::En0,
        tmstartr::En0,
        Tmstartr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tmstartr::En0,
            tmstartr::En0,
            Tmstartr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmstartr {
    #[inline(always)]
    fn default() -> Tmstartr {
        <crate::RegValueT<Tmstartr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmstartr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "Stops pulse output timer 5."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts pulse output timer 5."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "Stops pulse output timer 4."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts pulse output timer 4."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "Stops pulse output timer 3."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts pulse output timer 3."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "Stops pulse output timer 2."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts pulse output timer 2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "Stops pulse output timer 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts pulse output timer 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En0_SPEC;
    pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
    impl En0 {
        #[doc = "Stops pulse output timer 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Starts pulse output timer 0."]
        pub const _1: Self = Self::new(1);
    }
}
