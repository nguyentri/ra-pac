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
#[doc = r"Ethernet MAC"]
unsafe impl ::core::marker::Send for super::Rmac0Ns {}
unsafe impl ::core::marker::Sync for super::Rmac0Ns {}
impl super::Rmac0Ns {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "PHY Station Management Register"]
    #[inline(always)]
    pub const fn mpsm(&self) -> &'static crate::common::Reg<self::Mpsm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpsm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "PHY Interfaces Configuration Register"]
    #[inline(always)]
    pub const fn mpic(&self) -> &'static crate::common::Reg<self::Mpic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "PHY Interfaces Monitoring Register"]
    #[inline(always)]
    pub const fn mpim(&self) -> &'static crate::common::Reg<self::Mpim_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mpim_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "I/O Configuration Register"]
    #[inline(always)]
    pub const fn mioc(&self) -> &'static crate::common::Reg<self::Mioc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mioc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Transmission Frame Format Configuration Register"]
    #[inline(always)]
    pub const fn mtffc(&self) -> &'static crate::common::Reg<self::Mtffc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtffc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Transmission Pause or PFC Frame Configuration Register"]
    #[inline(always)]
    pub const fn mtpfc(&self) -> &'static crate::common::Reg<self::Mtpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Transmission Pause or PFC Frame Configuration Register 2"]
    #[inline(always)]
    pub const fn mtpfc2(
        &self,
    ) -> &'static crate::common::Reg<self::Mtpfc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtpfc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Transmission Pause or PFC Frame Configuration Register 3 for Priority Group t"]
    #[inline(always)]
    pub const fn mtpfc3t(
        &self,
    ) -> &'static crate::common::Reg<self::Mtpfc3T_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtpfc3T_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Reception General Configuration Register"]
    #[inline(always)]
    pub const fn mrgc(&self) -> &'static crate::common::Reg<self::Mrgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Reception MAC Address Configuration Register 0"]
    #[inline(always)]
    pub const fn mrmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Mrmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Reception MAC Address Configuration Register 1"]
    #[inline(always)]
    pub const fn mrmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Mrmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Reception Address Filter Configuration Register"]
    #[inline(always)]
    pub const fn mrafc(&self) -> &'static crate::common::Reg<self::Mrafc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrafc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Reception Storm Configuration for E-Frames Register"]
    #[inline(always)]
    pub const fn mrsce(&self) -> &'static crate::common::Reg<self::Mrsce_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrsce_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Reception Storm Configuration for P-Frames Register"]
    #[inline(always)]
    pub const fn mrscp(&self) -> &'static crate::common::Reg<self::Mrscp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrscp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Reception Storm Counter Configuration Register"]
    #[inline(always)]
    pub const fn mrscc(&self) -> &'static crate::common::Reg<self::Mrscc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrscc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Reception Frame Size Configuration for E-Frames Register"]
    #[inline(always)]
    pub const fn mrfsce(
        &self,
    ) -> &'static crate::common::Reg<self::Mrfsce_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrfsce_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "Reception Frame Size Configuration for P-Frames Register"]
    #[inline(always)]
    pub const fn mrfscp(
        &self,
    ) -> &'static crate::common::Reg<self::Mrfscp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrfscp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Timestamp Reception Configuration Register"]
    #[inline(always)]
    pub const fn mtrc(&self) -> &'static crate::common::Reg<self::Mtrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mtrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Reception Pause or PFC Frame Monitoring Register"]
    #[inline(always)]
    pub const fn mrpfm(&self) -> &'static crate::common::Reg<self::Mrpfm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrpfm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "PTP Filtering Register Configuration Register %s"]
    #[inline(always)]
    pub const fn mpfc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mpfc_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn mpfc0(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc1(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc2(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc3(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc4(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc5(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc6(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc7(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc8(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc9(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc10(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc11(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc12(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc13(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc14(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpfc15(&self) -> &'static crate::common::Reg<self::Mpfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }

    #[doc = "Link Verification Configuration Register"]
    #[inline(always)]
    pub const fn mlvc(&self) -> &'static crate::common::Reg<self::Mlvc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mlvc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "Energy Efficient Ethernet Configuration Register"]
    #[inline(always)]
    pub const fn meeec(&self) -> &'static crate::common::Reg<self::Meeec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Meeec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[doc = "Loopback Configuration Register"]
    #[inline(always)]
    pub const fn mlbc(&self) -> &'static crate::common::Reg<self::Mlbc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mlbc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
            )
        }
    }

    #[doc = "Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn meis(&self) -> &'static crate::common::Reg<self::Meis_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Meis_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn meie(&self) -> &'static crate::common::Reg<self::Meie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Meie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "Error Interrupt Disable Register"]
    #[inline(always)]
    pub const fn meid(&self) -> &'static crate::common::Reg<self::Meid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Meid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn mmis0(&self) -> &'static crate::common::Reg<self::Mmis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn mmie0(&self) -> &'static crate::common::Reg<self::Mmie0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmie0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Disable Register 0"]
    #[inline(always)]
    pub const fn mmid0(&self) -> &'static crate::common::Reg<self::Mmid0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmid0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn mmis1(&self) -> &'static crate::common::Reg<self::Mmis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn mmie1(&self) -> &'static crate::common::Reg<self::Mmie1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmie1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(548usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Disable Register 1"]
    #[inline(always)]
    pub const fn mmid1(&self) -> &'static crate::common::Reg<self::Mmid1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmid1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Status Register 2"]
    #[inline(always)]
    pub const fn mmis2(&self) -> &'static crate::common::Reg<self::Mmis2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmis2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(560usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Enable Register 2"]
    #[inline(always)]
    pub const fn mmie2(&self) -> &'static crate::common::Reg<self::Mmie2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmie2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(564usize),
            )
        }
    }

    #[doc = "Monitoring Interrupt Disable Register 2"]
    #[inline(always)]
    pub const fn mmid2(&self) -> &'static crate::common::Reg<self::Mmid2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmid2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(568usize),
            )
        }
    }

    #[doc = "Manual Pause Frame Transmit Counter Register"]
    #[inline(always)]
    pub const fn mmpftct(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpftct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mmpftct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "Automatic Pause Frame Transmit Counter Register"]
    #[inline(always)]
    pub const fn mapftct(
        &self,
    ) -> &'static crate::common::Reg<self::Mapftct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mapftct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "Pause Frame Receive Counter Register"]
    #[inline(always)]
    pub const fn mpfrct(&self) -> &'static crate::common::Reg<self::Mpfrct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mpfrct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(776usize),
            )
        }
    }

    #[doc = "False Carrier Indication Counter Register"]
    #[inline(always)]
    pub const fn mfcict(&self) -> &'static crate::common::Reg<self::Mfcict_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mfcict_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(780usize),
            )
        }
    }

    #[doc = "Energy Efficient Ethernet Counter Register"]
    #[inline(always)]
    pub const fn meeect(&self) -> &'static crate::common::Reg<self::Meeect_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Meeect_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }

    #[doc = "Manual PFC Frame Transmit Counter Register"]
    #[inline(always)]
    pub const fn mmpcftct(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpcftct_SPEC, crate::common::R>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x320usize))
        }
    }
    #[inline(always)]
    pub const fn mmpcftct0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpcftct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mmpcftct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x320usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpcftct1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpcftct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mmpcftct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x324usize),
            )
        }
    }

    #[doc = "Automatic PFC Frame Transmit Counter Register"]
    #[inline(always)]
    pub const fn mapcftct(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mapcftct_SPEC, crate::common::R>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x330usize))
        }
    }
    #[inline(always)]
    pub const fn mapcftct0(
        &self,
    ) -> &'static crate::common::Reg<self::Mapcftct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mapcftct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x330usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mapcftct1(
        &self,
    ) -> &'static crate::common::Reg<self::Mapcftct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mapcftct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x334usize),
            )
        }
    }

    #[doc = "PFC Frame Receive Counter Register"]
    #[inline(always)]
    pub const fn mpcfrct(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mpcfrct_SPEC, crate::common::R>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x340usize))
        }
    }
    #[inline(always)]
    pub const fn mpcfrct0(
        &self,
    ) -> &'static crate::common::Reg<self::Mpcfrct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mpcfrct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x340usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mpcfrct1(
        &self,
    ) -> &'static crate::common::Reg<self::Mpcfrct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mpcfrct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x344usize),
            )
        }
    }

    #[doc = "Receive Overflow Counter Register"]
    #[inline(always)]
    pub const fn mrovfc(&self) -> &'static crate::common::Reg<self::Mrovfc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrovfc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(864usize),
            )
        }
    }

    #[doc = "Received Good Frame Counter E-Frames Register"]
    #[inline(always)]
    pub const fn mrgfce(&self) -> &'static crate::common::Reg<self::Mrgfce_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrgfce_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1032usize),
            )
        }
    }

    #[doc = "Received Good Frame Counter P-Frames Register"]
    #[inline(always)]
    pub const fn mrgfcp(&self) -> &'static crate::common::Reg<self::Mrgfcp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrgfcp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1036usize),
            )
        }
    }

    #[doc = "Received Good Broadcast Frame Counter Register"]
    #[inline(always)]
    pub const fn mrbfc(&self) -> &'static crate::common::Reg<self::Mrbfc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrbfc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Received Good Multicast Frame Counter Register"]
    #[inline(always)]
    pub const fn mrmfc(&self) -> &'static crate::common::Reg<self::Mrmfc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrmfc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1044usize),
            )
        }
    }

    #[doc = "Received Good Unicast Frame Counter Register"]
    #[inline(always)]
    pub const fn mrufc(&self) -> &'static crate::common::Reg<self::Mrufc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrufc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[doc = "Received PHY Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrpefc(&self) -> &'static crate::common::Reg<self::Mrpefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrpefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1052usize),
            )
        }
    }

    #[doc = "Received Nibble Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrnefc(&self) -> &'static crate::common::Reg<self::Mrnefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrnefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1056usize),
            )
        }
    }

    #[doc = "Received FCS/mCRC Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrfmefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrfmefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrfmefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1060usize),
            )
        }
    }

    #[doc = "Received Final Fragment Missing Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrffmefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrffmefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrffmefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1064usize),
            )
        }
    }

    #[doc = "Received C-Fragment Count Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrcfcefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcfcefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrcfcefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1068usize),
            )
        }
    }

    #[doc = "Received Fragment Count Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrfcefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrfcefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrfcefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1072usize),
            )
        }
    }

    #[doc = "Received RMAC Filter Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrrcfefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrrcfefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrrcfefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1076usize),
            )
        }
    }

    #[doc = "Received Frame Count Register"]
    #[inline(always)]
    pub const fn mrfc(&self) -> &'static crate::common::Reg<self::Mrfc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrfc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1080usize),
            )
        }
    }

    #[doc = "Received Good Undersize Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrguefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrguefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrguefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1084usize),
            )
        }
    }

    #[doc = "Received Bad Undersize Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrbuefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrbuefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrbuefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1088usize),
            )
        }
    }

    #[doc = "Received Good Oversize Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrgoefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrgoefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrgoefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1092usize),
            )
        }
    }

    #[doc = "Received Bad Oversize Error Frame Count Register"]
    #[inline(always)]
    pub const fn mrboefc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrboefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrboefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1096usize),
            )
        }
    }

    #[doc = "Received Byte Counter E-Frames Upper Side Register"]
    #[inline(always)]
    pub const fn mrxbceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mrxbceu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrxbceu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1100usize),
            )
        }
    }

    #[doc = "Received Byte Counter E-Frames Lower Side Register"]
    #[inline(always)]
    pub const fn mrxbcel(
        &self,
    ) -> &'static crate::common::Reg<self::Mrxbcel_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrxbcel_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1104usize),
            )
        }
    }

    #[doc = "Received Byte Counter P-Frames Upper Side Register"]
    #[inline(always)]
    pub const fn mrxbcpu(
        &self,
    ) -> &'static crate::common::Reg<self::Mrxbcpu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrxbcpu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1108usize),
            )
        }
    }

    #[doc = "Received Byte Counter P-Frames Lower Side Register"]
    #[inline(always)]
    pub const fn mrxbcpl(
        &self,
    ) -> &'static crate::common::Reg<self::Mrxbcpl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrxbcpl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1112usize),
            )
        }
    }

    #[doc = "Transmitted Good Frame Counter E-Frames Register"]
    #[inline(always)]
    pub const fn mtgfce(&self) -> &'static crate::common::Reg<self::Mtgfce_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtgfce_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[doc = "Transmitted Good Frame Counter P-Frames Register"]
    #[inline(always)]
    pub const fn mtgfcp(&self) -> &'static crate::common::Reg<self::Mtgfcp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtgfcp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1292usize),
            )
        }
    }

    #[doc = "Transmitted Broadcast Frame Counter Register"]
    #[inline(always)]
    pub const fn mtbfc(&self) -> &'static crate::common::Reg<self::Mtbfc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtbfc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1296usize),
            )
        }
    }

    #[doc = "Transmitted Multicast Frame Counter Register"]
    #[inline(always)]
    pub const fn mtmfc(&self) -> &'static crate::common::Reg<self::Mtmfc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtmfc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[doc = "Transmitted Unicast Frame Counter Register"]
    #[inline(always)]
    pub const fn mtufc(&self) -> &'static crate::common::Reg<self::Mtufc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtufc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1304usize),
            )
        }
    }

    #[doc = "Transmitted Error Frame Counter Register"]
    #[inline(always)]
    pub const fn mtefc(&self) -> &'static crate::common::Reg<self::Mtefc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtefc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1308usize),
            )
        }
    }

    #[doc = "Transmitted Byte Counter E-Frames Upper Side Register"]
    #[inline(always)]
    pub const fn mtxbceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mtxbceu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtxbceu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1312usize),
            )
        }
    }

    #[doc = "Transmitted Byte Counter E-Frames Lower Side Register"]
    #[inline(always)]
    pub const fn mtxbcel(
        &self,
    ) -> &'static crate::common::Reg<self::Mtxbcel_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtxbcel_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1316usize),
            )
        }
    }

    #[doc = "Transmitted Byte Counter P-Frames Upper Side Register"]
    #[inline(always)]
    pub const fn mtxbcpu(
        &self,
    ) -> &'static crate::common::Reg<self::Mtxbcpu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtxbcpu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1320usize),
            )
        }
    }

    #[doc = "Transmitted Byte Counter P-Frames Lower Side Register"]
    #[inline(always)]
    pub const fn mtxbcpl(
        &self,
    ) -> &'static crate::common::Reg<self::Mtxbcpl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mtxbcpl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1324usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpsm_SPEC;
impl crate::sealed::RegSpec for Mpsm_SPEC {
    type DataType = u32;
}

#[doc = "PHY Station Management Register"]
pub type Mpsm = crate::RegValueT<Mpsm_SPEC>;

impl Mpsm {
    #[doc = "PHY Station Management Enable"]
    #[inline(always)]
    pub fn psme(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mpsm::Psme,
        mpsm::Psme,
        Mpsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mpsm::Psme,
            mpsm::Psme,
            Mpsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Management Frame Format"]
    #[inline(always)]
    pub fn mff(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mpsm::Mff,
        mpsm::Mff,
        Mpsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mpsm::Mff,
            mpsm::Mff,
            Mpsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Device Address"]
    #[inline(always)]
    pub fn pda(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Mpsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Mpsm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Register Address"]
    #[inline(always)]
    pub fn pra(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Mpsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Mpsm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Operation Code"]
    #[inline(always)]
    pub fn pop(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x3,
        1,
        0,
        mpsm::Pop,
        mpsm::Pop,
        Mpsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x3,
            1,
            0,
            mpsm::Pop,
            mpsm::Pop,
            Mpsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Register Data"]
    #[inline(always)]
    pub fn prd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Mpsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Mpsm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpsm {
    #[inline(always)]
    fn default() -> Mpsm {
        <crate::RegValueT<Mpsm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mpsm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psme_SPEC;
    pub type Psme = crate::EnumBitfieldStruct<u8, Psme_SPEC>;
    impl Psme {
        #[doc = "PHY register access is not on progress."]
        pub const _0: Self = Self::new(0);

        #[doc = "PHY register access is requested and on progress."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mff_SPEC;
    pub type Mff = crate::EnumBitfieldStruct<u8, Mff_SPEC>;
    impl Mff {
        #[doc = "normal management frame format defined in Clause 22 of IEEE802.3 Describe “MDIO” in this document."]
        pub const _0: Self = Self::new(0);

        #[doc = "extension management frame format defined in Clause 45 of IEEE802.3 Describe “eMDIO” in this document."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pop_SPEC;
    pub type Pop = crate::EnumBitfieldStruct<u8, Pop_SPEC>;
    impl Pop {
        #[doc = "Write Frame"]
        pub const _01: Self = Self::new(1);

        #[doc = "Read Frame"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved(0 0 or 1 1 should not be set when MPSM.MFF = 0)"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpic_SPEC;
impl crate::sealed::RegSpec for Mpic_SPEC {
    type DataType = u32;
}

#[doc = "PHY Interfaces Configuration Register"]
pub type Mpic = crate::RegValueT<Mpic_SPEC>;

impl Mpic {
    #[doc = "PHY Interface Select"]
    #[inline(always)]
    pub fn pis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        mpic::Pis,
        mpic::Pis,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            mpic::Pis,
            mpic::Pis,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Link Speed Configuration"]
    #[inline(always)]
    pub fn lsc(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        mpic::Lsc,
        mpic::Lsc,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            mpic::Lsc,
            mpic::Lsc,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Interrupt Polarity"]
    #[inline(always)]
    pub fn pip(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mpic::Pip,
        mpic::Pip,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mpic::Pip,
            mpic::Pip,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Interrupt Pin Plugged"]
    #[inline(always)]
    pub fn pipp(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mpic::Pipp,
        mpic::Pipp,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mpic::Pipp,
            mpic::Pipp,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Link Status Pin Plugged"]
    #[inline(always)]
    pub fn plspp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mpic::Plspp,
        mpic::Plspp,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mpic::Plspp,
            mpic::Plspp,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Station Management Clock Selection"]
    #[inline(always)]
    pub fn psmcs(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Mpic_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Mpic_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Station Management Disable Preamble"]
    #[inline(always)]
    pub fn psmdp(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mpic::Psmdp,
        mpic::Psmdp,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mpic::Psmdp,
            mpic::Psmdp,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Station Management Hold Time Adjustment"]
    #[inline(always)]
    pub fn psmht(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        mpic::Psmht,
        mpic::Psmht,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            mpic::Psmht,
            mpic::Psmht,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Station Management Capture Time Adjustment"]
    #[inline(always)]
    pub fn psmct(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        mpic::Psmct,
        mpic::Psmct,
        Mpic_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            mpic::Psmct,
            mpic::Psmct,
            Mpic_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mpic {
    #[inline(always)]
    fn default() -> Mpic {
        <crate::RegValueT<Mpic_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mpic {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pis_SPEC;
    pub type Pis = crate::EnumBitfieldStruct<u8, Pis_SPEC>;
    impl Pis {
        #[doc = "MII"]
        pub const _000: Self = Self::new(0);

        #[doc = "GMII"]
        pub const _010: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _100: Self = Self::new(4);

        #[doc = "Reserved"]
        pub const _101: Self = Self::new(5);

        #[doc = "Reserved"]
        pub const _110: Self = Self::new(6);

        #[doc = "Reserved"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsc_SPEC;
    pub type Lsc = crate::EnumBitfieldStruct<u8, Lsc_SPEC>;
    impl Lsc {
        #[doc = "10mbps"]
        pub const _000: Self = Self::new(0);

        #[doc = "100mbps"]
        pub const _001: Self = Self::new(1);

        #[doc = "1gbps"]
        pub const _010: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _011: Self = Self::new(3);

        #[doc = "Reserved"]
        pub const _110: Self = Self::new(6);

        #[doc = "Reserved"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pip_SPEC;
    pub type Pip = crate::EnumBitfieldStruct<u8, Pip_SPEC>;
    impl Pip {
        #[doc = "PHY interrupt is active low (default)."]
        pub const _0: Self = Self::new(0);

        #[doc = "PHY interrupt is active high."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipp_SPEC;
    pub type Pipp = crate::EnumBitfieldStruct<u8, Pipp_SPEC>;
    impl Pipp {
        #[doc = "Unplugged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Plugged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plspp_SPEC;
    pub type Plspp = crate::EnumBitfieldStruct<u8, Plspp_SPEC>;
    impl Plspp {
        #[doc = "Unplugged"]
        pub const _0: Self = Self::new(0);

        #[doc = "Plugged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psmdp_SPEC;
    pub type Psmdp = crate::EnumBitfieldStruct<u8, Psmdp_SPEC>;
    impl Psmdp {
        #[doc = "PHY Station Management Preamble enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "PHY Station Management Preamble disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psmht_SPEC;
    pub type Psmht = crate::EnumBitfieldStruct<u8, Psmht_SPEC>;
    impl Psmht {
        #[doc = "No adjusted hold time (i.e. change MDO at the rising edge of MDC)"]
        pub const _000: Self = Self::new(0);

        #[doc = "1 clk cycle extra hold time"]
        pub const _001: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psmct_SPEC;
    pub type Psmct = crate::EnumBitfieldStruct<u8, Psmct_SPEC>;
    impl Psmct {
        #[doc = "No adjusted capture time (i.e. capture MDI at the rising edge of MDC)"]
        pub const _000: Self = Self::new(0);

        #[doc = "capture before 1 clk cycle"]
        pub const _001: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpim_SPEC;
impl crate::sealed::RegSpec for Mpim_SPEC {
    type DataType = u32;
}

#[doc = "PHY Interfaces Monitoring Register"]
pub type Mpim = crate::RegValueT<Mpim_SPEC>;

impl Mpim {
    #[doc = "PHY Link Status Flag"]
    #[inline(always)]
    pub fn pls(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mpim::Pls, mpim::Pls, Mpim_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mpim::Pls,
            mpim::Pls,
            Mpim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "LPI Active"]
    #[inline(always)]
    pub fn lpia(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mpim::Lpia,
        mpim::Lpia,
        Mpim_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mpim::Lpia,
            mpim::Lpia,
            Mpim_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mpim {
    #[inline(always)]
    fn default() -> Mpim {
        <crate::RegValueT<Mpim_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mpim {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pls_SPEC;
    pub type Pls = crate::EnumBitfieldStruct<u8, Pls_SPEC>;
    impl Pls {
        #[doc = "PHY link signal is at the low level."]
        pub const _0: Self = Self::new(0);

        #[doc = "PHY link signal is at the high level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpia_SPEC;
    pub type Lpia = crate::EnumBitfieldStruct<u8, Lpia_SPEC>;
    impl Lpia {
        #[doc = "No LPI"]
        pub const _0: Self = Self::new(0);

        #[doc = "LPI active"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mioc_SPEC;
impl crate::sealed::RegSpec for Mioc_SPEC {
    type DataType = u32;
}

#[doc = "I/O Configuration Register"]
pub type Mioc = crate::RegValueT<Mioc_SPEC>;

impl Mioc {
    #[doc = "I/O Configuration 0"]
    #[inline(always)]
    pub fn mioc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mioc::Mioc0,
        mioc::Mioc0,
        Mioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mioc::Mioc0,
            mioc::Mioc0,
            Mioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mioc {
    #[inline(always)]
    fn default() -> Mioc {
        <crate::RegValueT<Mioc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mioc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mioc0_SPEC;
    pub type Mioc0 = crate::EnumBitfieldStruct<u8, Mioc0_SPEC>;
    impl Mioc0 {
        #[doc = "Normal mode. clk_phy_tx and clk_phy_rx are provided."]
        pub const _0: Self = Self::new(0);

        #[doc = "Emergency clock recovery mode. clk_phy_tx and clk_phy_rx are provided free-run clock (clk_common). (So transmission is not possible in this state.)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtffc_SPEC;
impl crate::sealed::RegSpec for Mtffc_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Frame Format Configuration Register"]
pub type Mtffc = crate::RegValueT<Mtffc_SPEC>;

impl Mtffc {
    #[doc = "Data Padding Disable"]
    #[inline(always)]
    pub fn dpad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mtffc::Dpad,
        mtffc::Dpad,
        Mtffc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mtffc::Dpad,
            mtffc::Dpad,
            Mtffc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Flow Control Mode"]
    #[inline(always)]
    pub fn fcm(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mtffc::Fcm,
        mtffc::Fcm,
        Mtffc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mtffc::Fcm,
            mtffc::Fcm,
            Mtffc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mtffc {
    #[inline(always)]
    fn default() -> Mtffc {
        <crate::RegValueT<Mtffc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mtffc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpad_SPEC;
    pub type Dpad = crate::EnumBitfieldStruct<u8, Dpad_SPEC>;
    impl Dpad {
        #[doc = "Padding is inserted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Padding is not inserted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fcm_SPEC;
    pub type Fcm = crate::EnumBitfieldStruct<u8, Fcm_SPEC>;
    impl Fcm {
        #[doc = "PAUSE"]
        pub const _0: Self = Self::new(0);

        #[doc = "PFC"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtpfc_SPEC;
impl crate::sealed::RegSpec for Mtpfc_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Pause or PFC Frame Configuration Register"]
pub type Mtpfc = crate::RegValueT<Mtpfc_SPEC>;

impl Mtpfc {
    #[doc = "Pause Time"]
    #[inline(always)]
    pub fn pt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mtpfc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mtpfc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pause Frame Retransmission Time"]
    #[inline(always)]
    pub fn pfrt(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Mtpfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Mtpfc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pause Frame Mode"]
    #[inline(always)]
    pub fn pfm(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mtpfc::Pfm,
        mtpfc::Pfm,
        Mtpfc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mtpfc::Pfm,
            mtpfc::Pfm,
            Mtpfc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pause or PFC Frame Retry Limit Value"]
    #[inline(always)]
    pub fn pfrlv(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Mtpfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Mtpfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtpfc {
    #[inline(always)]
    fn default() -> Mtpfc {
        <crate::RegValueT<Mtpfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mtpfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfm_SPEC;
    pub type Pfm = crate::EnumBitfieldStruct<u8, Pfm_SPEC>;
    impl Pfm {
        #[doc = "Automatic pause frame. Sending a pause frame will be triggered by a hardware pause request."]
        pub const _0: Self = Self::new(0);

        #[doc = "Manual pause frame. Sending a Pause frame will be triggered by MTPFC2.MPFR or MTPFC2.MPFCFR."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtpfc2_SPEC;
impl crate::sealed::RegSpec for Mtpfc2_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Pause or PFC Frame Configuration Register 2"]
pub type Mtpfc2 = crate::RegValueT<Mtpfc2_SPEC>;

impl Mtpfc2 {
    #[doc = "PFC Frame Transmission with TIME = 0"]
    #[inline(always)]
    pub fn pfcttz(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        mtpfc2::Pfcttz,
        mtpfc2::Pfcttz,
        Mtpfc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            mtpfc2::Pfcttz,
            mtpfc2::Pfcttz,
            Mtpfc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Manual PFC Frame Request"]
    #[inline(always)]
    pub fn mpfcfr(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Mtpfc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Mtpfc2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pause Frame Transmission with TIME = 0"]
    #[inline(always)]
    pub fn pfttz(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mtpfc2::Pfttz,
        mtpfc2::Pfttz,
        Mtpfc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mtpfc2::Pfttz,
            mtpfc2::Pfttz,
            Mtpfc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Manual Pause Frame Request"]
    #[inline(always)]
    pub fn mpfr(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Mtpfc2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mtpfc2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mtpfc2 {
    #[inline(always)]
    fn default() -> Mtpfc2 {
        <crate::RegValueT<Mtpfc2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mtpfc2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcttz_SPEC;
    pub type Pfcttz = crate::EnumBitfieldStruct<u8, Pfcttz_SPEC>;
    impl Pfcttz {
        #[doc = "The transmission of PFC frames with TIME value zero is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The transmission of PFC frames with TIME value zero is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfttz_SPEC;
    pub type Pfttz = crate::EnumBitfieldStruct<u8, Pfttz_SPEC>;
    impl Pfttz {
        #[doc = "The transmission of Pause frames with TIME value zero is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The transmission of Pause frames with TIME value zero is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtpfc3T_SPEC;
impl crate::sealed::RegSpec for Mtpfc3T_SPEC {
    type DataType = u32;
}

#[doc = "Transmission Pause or PFC Frame Configuration Register 3 for Priority Group t"]
pub type Mtpfc3T = crate::RegValueT<Mtpfc3T_SPEC>;

impl Mtpfc3T {
    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg0,
        mtpfc3t::Pfcpg0,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg0,
            mtpfc3t::Pfcpg0,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg1,
        mtpfc3t::Pfcpg1,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg1,
            mtpfc3t::Pfcpg1,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg2,
        mtpfc3t::Pfcpg2,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg2,
            mtpfc3t::Pfcpg2,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg3,
        mtpfc3t::Pfcpg3,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg3,
            mtpfc3t::Pfcpg3,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg4,
        mtpfc3t::Pfcpg4,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg4,
            mtpfc3t::Pfcpg4,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg5,
        mtpfc3t::Pfcpg5,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg5,
            mtpfc3t::Pfcpg5,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg6,
        mtpfc3t::Pfcpg6,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg6,
            mtpfc3t::Pfcpg6,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Priority Enable n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcpg7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mtpfc3t::Pfcpg7,
        mtpfc3t::Pfcpg7,
        Mtpfc3T_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mtpfc3t::Pfcpg7,
            mtpfc3t::Pfcpg7,
            Mtpfc3T_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mtpfc3T {
    #[inline(always)]
    fn default() -> Mtpfc3T {
        <crate::RegValueT<Mtpfc3T_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mtpfc3t {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg0_SPEC;
    pub type Pfcpg0 = crate::EnumBitfieldStruct<u8, Pfcpg0_SPEC>;
    impl Pfcpg0 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg1_SPEC;
    pub type Pfcpg1 = crate::EnumBitfieldStruct<u8, Pfcpg1_SPEC>;
    impl Pfcpg1 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg2_SPEC;
    pub type Pfcpg2 = crate::EnumBitfieldStruct<u8, Pfcpg2_SPEC>;
    impl Pfcpg2 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg3_SPEC;
    pub type Pfcpg3 = crate::EnumBitfieldStruct<u8, Pfcpg3_SPEC>;
    impl Pfcpg3 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg4_SPEC;
    pub type Pfcpg4 = crate::EnumBitfieldStruct<u8, Pfcpg4_SPEC>;
    impl Pfcpg4 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg5_SPEC;
    pub type Pfcpg5 = crate::EnumBitfieldStruct<u8, Pfcpg5_SPEC>;
    impl Pfcpg5 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg6_SPEC;
    pub type Pfcpg6 = crate::EnumBitfieldStruct<u8, Pfcpg6_SPEC>;
    impl Pfcpg6 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcpg7_SPEC;
    pub type Pfcpg7 = crate::EnumBitfieldStruct<u8, Pfcpg7_SPEC>;
    impl Pfcpg7 {
        #[doc = "priority x is not assigned to priority group t."]
        pub const _0: Self = Self::new(0);

        #[doc = "priority x is assigned to group t."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrgc_SPEC;
impl crate::sealed::RegSpec for Mrgc_SPEC {
    type DataType = u32;
}

#[doc = "Reception General Configuration Register"]
pub type Mrgc = crate::RegValueT<Mrgc_SPEC>;

impl Mrgc {
    #[doc = "Receive CRC Pass Through"]
    #[inline(always)]
    pub fn rcpt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrgc::Rcpt,
        mrgc::Rcpt,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrgc::Rcpt,
            mrgc::Rcpt,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pause Frame Reception Control"]
    #[inline(always)]
    pub fn pfrc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mrgc::Pfrc,
        mrgc::Pfrc,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mrgc::Pfrc,
            mrgc::Pfrc,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pause or PFC Frame Reception with Time = 0"]
    #[inline(always)]
    pub fn pfrtz(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mrgc::Pfrtz,
        mrgc::Pfrtz,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mrgc::Pfrtz,
            mrgc::Pfrtz,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn mpde(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mrgc::Mpde,
        mrgc::Mpde,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mrgc::Mpde,
            mrgc::Mpde,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reception Flow Control Forwarding Enable"]
    #[inline(always)]
    pub fn rfcfe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mrgc::Rfcfe,
        mrgc::Rfcfe,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mrgc::Rfcfe,
            mrgc::Rfcfe,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mrgc::Pfcrc0,
        mrgc::Pfcrc0,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mrgc::Pfcrc0,
            mrgc::Pfcrc0,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mrgc::Pfcrc1,
        mrgc::Pfcrc1,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mrgc::Pfcrc1,
            mrgc::Pfcrc1,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mrgc::Pfcrc2,
        mrgc::Pfcrc2,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mrgc::Pfcrc2,
            mrgc::Pfcrc2,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mrgc::Pfcrc3,
        mrgc::Pfcrc3,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mrgc::Pfcrc3,
            mrgc::Pfcrc3,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mrgc::Pfcrc4,
        mrgc::Pfcrc4,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mrgc::Pfcrc4,
            mrgc::Pfcrc4,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mrgc::Pfcrc5,
        mrgc::Pfcrc5,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mrgc::Pfcrc5,
            mrgc::Pfcrc5,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mrgc::Pfcrc6,
        mrgc::Pfcrc6,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mrgc::Pfcrc6,
            mrgc::Pfcrc6,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PFC Frame Reception Control n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfcrc7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mrgc::Pfcrc7,
        mrgc::Pfcrc7,
        Mrgc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mrgc::Pfcrc7,
            mrgc::Pfcrc7,
            Mrgc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrgc {
    #[inline(always)]
    fn default() -> Mrgc {
        <crate::RegValueT<Mrgc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrgc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcpt_SPEC;
    pub type Rcpt = crate::EnumBitfieldStruct<u8, Rcpt_SPEC>;
    impl Rcpt {
        #[doc = "Correct FCS is not passed to the MHD. Incorrect FCS is passed to the MHD."]
        pub const _0: Self = Self::new(0);

        #[doc = "Both Correct and Incorrect FCS are passed to the MHD."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfrc_SPEC;
    pub type Pfrc = crate::EnumBitfieldStruct<u8, Pfrc_SPEC>;
    impl Pfrc {
        #[doc = "Flow control for the receiving port is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving port is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfrtz_SPEC;
    pub type Pfrtz = crate::EnumBitfieldStruct<u8, Pfrtz_SPEC>;
    impl Pfrtz {
        #[doc = "Reception of Pause or PFC frames with the TIME value 0 is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reception of Pause or PFC frames with the TIME value 0 is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpde_SPEC;
    pub type Mpde = crate::EnumBitfieldStruct<u8, Mpde_SPEC>;
    impl Mpde {
        #[doc = "Magic Packet detection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Magic Packet detection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfcfe_SPEC;
    pub type Rfcfe = crate::EnumBitfieldStruct<u8, Rfcfe_SPEC>;
    impl Rfcfe {
        #[doc = "Flow Control Frame (PAUSE or PFC frame) is not forward to Rx MHD I/F."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow Control Frame (PAUSE or PFC frame) is forward to Rx MHD I/F."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc0_SPEC;
    pub type Pfcrc0 = crate::EnumBitfieldStruct<u8, Pfcrc0_SPEC>;
    impl Pfcrc0 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc1_SPEC;
    pub type Pfcrc1 = crate::EnumBitfieldStruct<u8, Pfcrc1_SPEC>;
    impl Pfcrc1 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc2_SPEC;
    pub type Pfcrc2 = crate::EnumBitfieldStruct<u8, Pfcrc2_SPEC>;
    impl Pfcrc2 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc3_SPEC;
    pub type Pfcrc3 = crate::EnumBitfieldStruct<u8, Pfcrc3_SPEC>;
    impl Pfcrc3 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc4_SPEC;
    pub type Pfcrc4 = crate::EnumBitfieldStruct<u8, Pfcrc4_SPEC>;
    impl Pfcrc4 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc5_SPEC;
    pub type Pfcrc5 = crate::EnumBitfieldStruct<u8, Pfcrc5_SPEC>;
    impl Pfcrc5 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc6_SPEC;
    pub type Pfcrc6 = crate::EnumBitfieldStruct<u8, Pfcrc6_SPEC>;
    impl Pfcrc6 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfcrc7_SPEC;
    pub type Pfcrc7 = crate::EnumBitfieldStruct<u8, Pfcrc7_SPEC>;
    impl Pfcrc7 {
        #[doc = "Flow control for the receiving priority level x (8) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flow control for the receiving priority level x (8) is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrmac0_SPEC;
impl crate::sealed::RegSpec for Mrmac0_SPEC {
    type DataType = u32;
}

#[doc = "Reception MAC Address Configuration Register 0"]
pub type Mrmac0 = crate::RegValueT<Mrmac0_SPEC>;

impl Mrmac0 {
    #[doc = "MAC Address Upper Part"]
    #[inline(always)]
    pub fn mau(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrmac0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrmac0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrmac0 {
    #[inline(always)]
    fn default() -> Mrmac0 {
        <crate::RegValueT<Mrmac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrmac1_SPEC;
impl crate::sealed::RegSpec for Mrmac1_SPEC {
    type DataType = u32;
}

#[doc = "Reception MAC Address Configuration Register 1"]
pub type Mrmac1 = crate::RegValueT<Mrmac1_SPEC>;

impl Mrmac1 {
    #[doc = "MAC Address Lower Part"]
    #[inline(always)]
    pub fn mal(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrmac1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrmac1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrmac1 {
    #[inline(always)]
    fn default() -> Mrmac1 {
        <crate::RegValueT<Mrmac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrafc_SPEC;
impl crate::sealed::RegSpec for Mrafc_SPEC {
    type DataType = u32;
}

#[doc = "Reception Address Filter Configuration Register"]
pub type Mrafc = crate::RegValueT<Mrafc_SPEC>;

impl Mrafc {
    #[doc = "Unicast Reception Enable E-Frames"]
    #[inline(always)]
    pub fn ucene(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrafc::Ucene,
        mrafc::Ucene,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrafc::Ucene,
            mrafc::Ucene,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Reception Enable E-Frames"]
    #[inline(always)]
    pub fn mcene(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mrafc::Mcene,
        mrafc::Mcene,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mrafc::Mcene,
            mrafc::Mcene,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Broadcast Reception Enable E-Frames"]
    #[inline(always)]
    pub fn bcene(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mrafc::Bcene,
        mrafc::Bcene,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mrafc::Bcene,
            mrafc::Bcene,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Storm Filter Reception Enable E-Frames"]
    #[inline(always)]
    pub fn mstene(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mrafc::Mstene,
        mrafc::Mstene,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mrafc::Mstene,
            mrafc::Mstene,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Broadcast Storm Filter Reception Enable E-Frames"]
    #[inline(always)]
    pub fn bstene(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mrafc::Bstene,
        mrafc::Bstene,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mrafc::Bstene,
            mrafc::Bstene,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Storm Autoclear E-Frames"]
    #[inline(always)]
    pub fn mcace(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mrafc::Mcace,
        mrafc::Mcace,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mrafc::Mcace,
            mrafc::Mcace,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Broadcast Storm Autoclear E-Frames"]
    #[inline(always)]
    pub fn bcace(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mrafc::Bcace,
        mrafc::Bcace,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mrafc::Bcace,
            mrafc::Bcace,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Null Destination Address Reception Enable E-Frames"]
    #[inline(always)]
    pub fn ndaree(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mrafc::Ndaree,
        mrafc::Ndaree,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mrafc::Ndaree,
            mrafc::Ndaree,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Same DA and SA Frames Reception Enable E-Frames"]
    #[inline(always)]
    pub fn sdsfree(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mrafc::Sdsfree,
        mrafc::Sdsfree,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mrafc::Sdsfree,
            mrafc::Sdsfree,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Null Source Address Reception Enable E-Frames"]
    #[inline(always)]
    pub fn nsaree(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mrafc::Nsaree,
        mrafc::Nsaree,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mrafc::Nsaree,
            mrafc::Nsaree,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Source Address Reception Enable E-Frames"]
    #[inline(always)]
    pub fn msaree(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mrafc::Msaree,
        mrafc::Msaree,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mrafc::Msaree,
            mrafc::Msaree,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Unicast Reception Enable P-Frames"]
    #[inline(always)]
    pub fn ucenp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mrafc::Ucenp,
        mrafc::Ucenp,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mrafc::Ucenp,
            mrafc::Ucenp,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Reception Enable P-Frames"]
    #[inline(always)]
    pub fn mcenp(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mrafc::Mcenp,
        mrafc::Mcenp,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mrafc::Mcenp,
            mrafc::Mcenp,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Broadcast Reception Enable P-Frames"]
    #[inline(always)]
    pub fn bcenp(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mrafc::Bcenp,
        mrafc::Bcenp,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mrafc::Bcenp,
            mrafc::Bcenp,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Storm Filter Reception Enable P-Frames"]
    #[inline(always)]
    pub fn mstenp(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mrafc::Mstenp,
        mrafc::Mstenp,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mrafc::Mstenp,
            mrafc::Mstenp,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Broadcast Storm Filter Reception Enable P-Frames"]
    #[inline(always)]
    pub fn bstenp(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mrafc::Bstenp,
        mrafc::Bstenp,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mrafc::Bstenp,
            mrafc::Bstenp,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Storm Autoclear P-Frames"]
    #[inline(always)]
    pub fn mcacp(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mrafc::Mcacp,
        mrafc::Mcacp,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mrafc::Mcacp,
            mrafc::Mcacp,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Broadcast Storm Autoclear P-Frames"]
    #[inline(always)]
    pub fn bcacp(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mrafc::Bcacp,
        mrafc::Bcacp,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mrafc::Bcacp,
            mrafc::Bcacp,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Null Destination Address Reception Enable P-Frames"]
    #[inline(always)]
    pub fn ndarep(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mrafc::Ndarep,
        mrafc::Ndarep,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mrafc::Ndarep,
            mrafc::Ndarep,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Same DA and SA Frames Reception Enable P-Frames"]
    #[inline(always)]
    pub fn sdsfrep(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mrafc::Sdsfrep,
        mrafc::Sdsfrep,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mrafc::Sdsfrep,
            mrafc::Sdsfrep,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Null Source Address Reception Enable P-Frames"]
    #[inline(always)]
    pub fn nsarep(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mrafc::Nsarep,
        mrafc::Nsarep,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mrafc::Nsarep,
            mrafc::Nsarep,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Source Address Reception Enable P-Frames"]
    #[inline(always)]
    pub fn msarep(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mrafc::Msarep,
        mrafc::Msarep,
        Mrafc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mrafc::Msarep,
            mrafc::Msarep,
            Mrafc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrafc {
    #[inline(always)]
    fn default() -> Mrafc {
        <crate::RegValueT<Mrafc_SPEC> as RegisterValue<_>>::new(458759)
    }
}
pub mod mrafc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ucene_SPEC;
    pub type Ucene = crate::EnumBitfieldStruct<u8, Ucene_SPEC>;
    impl Ucene {
        #[doc = "Unicast reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unicast reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcene_SPEC;
    pub type Mcene = crate::EnumBitfieldStruct<u8, Mcene_SPEC>;
    impl Mcene {
        #[doc = "Multicast reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multicast reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcene_SPEC;
    pub type Bcene = crate::EnumBitfieldStruct<u8, Bcene_SPEC>;
    impl Bcene {
        #[doc = "Broadcast reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Broadcast reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstene_SPEC;
    pub type Mstene = crate::EnumBitfieldStruct<u8, Mstene_SPEC>;
    impl Mstene {
        #[doc = "Multicast storm filter reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multicast storm filter reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bstene_SPEC;
    pub type Bstene = crate::EnumBitfieldStruct<u8, Bstene_SPEC>;
    impl Bstene {
        #[doc = "Broadcast storm filter reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Broadcast storm filter reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcace_SPEC;
    pub type Mcace = crate::EnumBitfieldStruct<u8, Mcace_SPEC>;
    impl Mcace {
        #[doc = "Multicast Storm Autoclear disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multicast Storm Autoclear enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcace_SPEC;
    pub type Bcace = crate::EnumBitfieldStruct<u8, Bcace_SPEC>;
    impl Bcace {
        #[doc = "Broadcast Storm Autoclear disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Broadcast Storm Autoclear enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ndaree_SPEC;
    pub type Ndaree = crate::EnumBitfieldStruct<u8, Ndaree_SPEC>;
    impl Ndaree {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdsfree_SPEC;
    pub type Sdsfree = crate::EnumBitfieldStruct<u8, Sdsfree_SPEC>;
    impl Sdsfree {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nsaree_SPEC;
    pub type Nsaree = crate::EnumBitfieldStruct<u8, Nsaree_SPEC>;
    impl Nsaree {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msaree_SPEC;
    pub type Msaree = crate::EnumBitfieldStruct<u8, Msaree_SPEC>;
    impl Msaree {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ucenp_SPEC;
    pub type Ucenp = crate::EnumBitfieldStruct<u8, Ucenp_SPEC>;
    impl Ucenp {
        #[doc = "Unicast reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Unicast reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcenp_SPEC;
    pub type Mcenp = crate::EnumBitfieldStruct<u8, Mcenp_SPEC>;
    impl Mcenp {
        #[doc = "Multicast reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multicast reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcenp_SPEC;
    pub type Bcenp = crate::EnumBitfieldStruct<u8, Bcenp_SPEC>;
    impl Bcenp {
        #[doc = "Broadcast reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Broadcast reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstenp_SPEC;
    pub type Mstenp = crate::EnumBitfieldStruct<u8, Mstenp_SPEC>;
    impl Mstenp {
        #[doc = "Multicast storm filter reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multicast storm filter reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bstenp_SPEC;
    pub type Bstenp = crate::EnumBitfieldStruct<u8, Bstenp_SPEC>;
    impl Bstenp {
        #[doc = "Broadcast storm filter reception disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Broadcast storm filter reception enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcacp_SPEC;
    pub type Mcacp = crate::EnumBitfieldStruct<u8, Mcacp_SPEC>;
    impl Mcacp {
        #[doc = "Multicast Storm Autoclear disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multicast Storm Autoclear enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcacp_SPEC;
    pub type Bcacp = crate::EnumBitfieldStruct<u8, Bcacp_SPEC>;
    impl Bcacp {
        #[doc = "Broadcast Storm Autoclear disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Broadcast Storm Autoclear enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ndarep_SPEC;
    pub type Ndarep = crate::EnumBitfieldStruct<u8, Ndarep_SPEC>;
    impl Ndarep {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdsfrep_SPEC;
    pub type Sdsfrep = crate::EnumBitfieldStruct<u8, Sdsfrep_SPEC>;
    impl Sdsfrep {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nsarep_SPEC;
    pub type Nsarep = crate::EnumBitfieldStruct<u8, Nsarep_SPEC>;
    impl Nsarep {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msarep_SPEC;
    pub type Msarep = crate::EnumBitfieldStruct<u8, Msarep_SPEC>;
    impl Msarep {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrsce_SPEC;
impl crate::sealed::RegSpec for Mrsce_SPEC {
    type DataType = u32;
}

#[doc = "Reception Storm Configuration for E-Frames Register"]
pub type Mrsce = crate::RegValueT<Mrsce_SPEC>;

impl Mrsce {
    #[doc = "Consecutive Multicast Frame Reception Count Setting for E-Frames"]
    #[inline(always)]
    pub fn cmfe(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrsce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrsce_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Consecutive Broadcast Frame Reception Count Setting for E-Frames"]
    #[inline(always)]
    pub fn cbfe(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Mrsce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Mrsce_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrsce {
    #[inline(always)]
    fn default() -> Mrsce {
        <crate::RegValueT<Mrsce_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrscp_SPEC;
impl crate::sealed::RegSpec for Mrscp_SPEC {
    type DataType = u32;
}

#[doc = "Reception Storm Configuration for P-Frames Register"]
pub type Mrscp = crate::RegValueT<Mrscp_SPEC>;

impl Mrscp {
    #[doc = "Consecutive Multicast Frame Reception Count Setting for P-Frames"]
    #[inline(always)]
    pub fn cmfp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrscp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrscp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Consecutive Broadcast Frame Reception Count Setting for P-Frames"]
    #[inline(always)]
    pub fn cbfp(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Mrscp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Mrscp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrscp {
    #[inline(always)]
    fn default() -> Mrscp {
        <crate::RegValueT<Mrscp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrscc_SPEC;
impl crate::sealed::RegSpec for Mrscc_SPEC {
    type DataType = u32;
}

#[doc = "Reception Storm Counter Configuration Register"]
pub type Mrscc = crate::RegValueT<Mrscc_SPEC>;

impl Mrscc {
    #[doc = "Multicast Storm Counter Clear E-Frames"]
    #[inline(always)]
    pub fn mscce(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mrscc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mrscc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Broadcast Storm Counter Clear E-Frames"]
    #[inline(always)]
    pub fn bscce(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mrscc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mrscc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Multicast Storm Counter Clear P-Frames"]
    #[inline(always)]
    pub fn msccp(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mrscc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mrscc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Broadcast Storm Counter Clear P-Frames"]
    #[inline(always)]
    pub fn bsccp(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mrscc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mrscc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mrscc {
    #[inline(always)]
    fn default() -> Mrscc {
        <crate::RegValueT<Mrscc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrfsce_SPEC;
impl crate::sealed::RegSpec for Mrfsce_SPEC {
    type DataType = u32;
}

#[doc = "Reception Frame Size Configuration for E-Frames Register"]
pub type Mrfsce = crate::RegValueT<Mrfsce_SPEC>;

impl Mrfsce {
    #[doc = "E-Frame Maximum Size"]
    #[inline(always)]
    pub fn emxs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrfsce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrfsce_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "E-Frame Minimum Size"]
    #[inline(always)]
    pub fn emns(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Mrfsce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Mrfsce_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrfsce {
    #[inline(always)]
    fn default() -> Mrfsce {
        <crate::RegValueT<Mrfsce_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrfscp_SPEC;
impl crate::sealed::RegSpec for Mrfscp_SPEC {
    type DataType = u32;
}

#[doc = "Reception Frame Size Configuration for P-Frames Register"]
pub type Mrfscp = crate::RegValueT<Mrfscp_SPEC>;

impl Mrfscp {
    #[doc = "P-Frame Maximum Size"]
    #[inline(always)]
    pub fn pmxs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrfscp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrfscp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "P-Frame Minimum Size"]
    #[inline(always)]
    pub fn pmns(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Mrfscp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Mrfscp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrfscp {
    #[inline(always)]
    fn default() -> Mrfscp {
        <crate::RegValueT<Mrfscp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtrc_SPEC;
impl crate::sealed::RegSpec for Mtrc_SPEC {
    type DataType = u32;
}

#[doc = "Timestamp Reception Configuration Register"]
pub type Mtrc = crate::RegValueT<Mtrc_SPEC>;

impl Mtrc {
    #[doc = "Timestamp Reception Hardware Filter Match Enable n (n = 0 to 1)"]
    #[inline(always)]
    pub fn trhfme0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mtrc::Trhfme0,
        mtrc::Trhfme0,
        Mtrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mtrc::Trhfme0,
            mtrc::Trhfme0,
            Mtrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Reception Hardware Filter Match Enable n (n = 0 to 1)"]
    #[inline(always)]
    pub fn trhfme1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mtrc::Trhfme1,
        mtrc::Trhfme1,
        Mtrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mtrc::Trhfme1,
            mtrc::Trhfme1,
            Mtrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Reception Default Disable E-Frame"]
    #[inline(always)]
    pub fn trdde(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mtrc::Trdde,
        mtrc::Trdde,
        Mtrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mtrc::Trdde,
            mtrc::Trdde,
            Mtrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Reception Default Disable P-Frame"]
    #[inline(always)]
    pub fn trddp(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mtrc::Trddp,
        mtrc::Trddp,
        Mtrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mtrc::Trddp,
            mtrc::Trddp,
            Mtrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Capture on TX Side E-Frame"]
    #[inline(always)]
    pub fn tctse(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mtrc::Tctse,
        mtrc::Tctse,
        Mtrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mtrc::Tctse,
            mtrc::Tctse,
            Mtrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timestamp Capture on TX Side P-Frame"]
    #[inline(always)]
    pub fn tctsp(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        mtrc::Tctsp,
        mtrc::Tctsp,
        Mtrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            mtrc::Tctsp,
            mtrc::Tctsp,
            Mtrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Default Timer Number"]
    #[inline(always)]
    pub fn dtn(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Mtrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Mtrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtrc {
    #[inline(always)]
    fn default() -> Mtrc {
        <crate::RegValueT<Mtrc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mtrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trhfme0_SPEC;
    pub type Trhfme0 = crate::EnumBitfieldStruct<u8, Trhfme0_SPEC>;
    impl Trhfme0 {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trhfme1_SPEC;
    pub type Trhfme1 = crate::EnumBitfieldStruct<u8, Trhfme1_SPEC>;
    impl Trhfme1 {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trdde_SPEC;
    pub type Trdde = crate::EnumBitfieldStruct<u8, Trdde_SPEC>;
    impl Trdde {
        #[doc = "enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "disable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trddp_SPEC;
    pub type Trddp = crate::EnumBitfieldStruct<u8, Trddp_SPEC>;
    impl Trddp {
        #[doc = "enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "disable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tctse_SPEC;
    pub type Tctse = crate::EnumBitfieldStruct<u8, Tctse_SPEC>;
    impl Tctse {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tctsp_SPEC;
    pub type Tctsp = crate::EnumBitfieldStruct<u8, Tctsp_SPEC>;
    impl Tctsp {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrpfm_SPEC;
impl crate::sealed::RegSpec for Mrpfm_SPEC {
    type DataType = u32;
}

#[doc = "Reception Pause or PFC Frame Monitoring Register"]
pub type Mrpfm = crate::RegValueT<Mrpfm_SPEC>;

impl Mrpfm {
    #[doc = "Pause Time Counting Active"]
    #[inline(always)]
    pub fn ptca(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrpfm::Ptca,
        mrpfm::Ptca,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrpfm::Ptca,
            mrpfm::Ptca,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mrpfm::Pfctca0,
        mrpfm::Pfctca0,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mrpfm::Pfctca0,
            mrpfm::Pfctca0,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mrpfm::Pfctca1,
        mrpfm::Pfctca1,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mrpfm::Pfctca1,
            mrpfm::Pfctca1,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mrpfm::Pfctca2,
        mrpfm::Pfctca2,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mrpfm::Pfctca2,
            mrpfm::Pfctca2,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mrpfm::Pfctca3,
        mrpfm::Pfctca3,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mrpfm::Pfctca3,
            mrpfm::Pfctca3,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mrpfm::Pfctca4,
        mrpfm::Pfctca4,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mrpfm::Pfctca4,
            mrpfm::Pfctca4,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mrpfm::Pfctca5,
        mrpfm::Pfctca5,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mrpfm::Pfctca5,
            mrpfm::Pfctca5,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mrpfm::Pfctca6,
        mrpfm::Pfctca6,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mrpfm::Pfctca6,
            mrpfm::Pfctca6,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PFC Time Counting Active n (n = 0 to 7)"]
    #[inline(always)]
    pub fn pfctca7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mrpfm::Pfctca7,
        mrpfm::Pfctca7,
        Mrpfm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mrpfm::Pfctca7,
            mrpfm::Pfctca7,
            Mrpfm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrpfm {
    #[inline(always)]
    fn default() -> Mrpfm {
        <crate::RegValueT<Mrpfm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrpfm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptca_SPEC;
    pub type Ptca = crate::EnumBitfieldStruct<u8, Ptca_SPEC>;
    impl Ptca {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca0_SPEC;
    pub type Pfctca0 = crate::EnumBitfieldStruct<u8, Pfctca0_SPEC>;
    impl Pfctca0 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca1_SPEC;
    pub type Pfctca1 = crate::EnumBitfieldStruct<u8, Pfctca1_SPEC>;
    impl Pfctca1 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca2_SPEC;
    pub type Pfctca2 = crate::EnumBitfieldStruct<u8, Pfctca2_SPEC>;
    impl Pfctca2 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca3_SPEC;
    pub type Pfctca3 = crate::EnumBitfieldStruct<u8, Pfctca3_SPEC>;
    impl Pfctca3 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca4_SPEC;
    pub type Pfctca4 = crate::EnumBitfieldStruct<u8, Pfctca4_SPEC>;
    impl Pfctca4 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca5_SPEC;
    pub type Pfctca5 = crate::EnumBitfieldStruct<u8, Pfctca5_SPEC>;
    impl Pfctca5 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca6_SPEC;
    pub type Pfctca6 = crate::EnumBitfieldStruct<u8, Pfctca6_SPEC>;
    impl Pfctca6 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfctca7_SPEC;
    pub type Pfctca7 = crate::EnumBitfieldStruct<u8, Pfctca7_SPEC>;
    impl Pfctca7 {
        #[doc = "No pause state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pause state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpfc_SPEC;
impl crate::sealed::RegSpec for Mpfc_SPEC {
    type DataType = u32;
}

#[doc = "PTP Filtering Register Configuration Register %s"]
pub type Mpfc = crate::RegValueT<Mpfc_SPEC>;

impl Mpfc {
    #[doc = "PTP Filtering Byte Number"]
    #[inline(always)]
    pub fn pfbn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mpfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mpfc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PTP Filtering Byte Value"]
    #[inline(always)]
    pub fn pfbv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mpfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mpfc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timer Enable for Filtering n (n = 0 to 1)"]
    #[inline(always)]
    pub fn tef1_to_tef0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Mpfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Mpfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpfc {
    #[inline(always)]
    fn default() -> Mpfc {
        <crate::RegValueT<Mpfc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mlvc_SPEC;
impl crate::sealed::RegSpec for Mlvc_SPEC {
    type DataType = u32;
}

#[doc = "Link Verification Configuration Register"]
pub type Mlvc = crate::RegValueT<Mlvc_SPEC>;

impl Mlvc {
    #[doc = "Link Verification Timer"]
    #[inline(always)]
    pub fn lvt(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Mlvc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Mlvc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Preemption Auto Response Enable"]
    #[inline(always)]
    pub fn pase(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mlvc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mlvc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Preemption Link Verification"]
    #[inline(always)]
    pub fn plv(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mlvc::Plv,
        mlvc::Plv,
        Mlvc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mlvc::Plv,
            mlvc::Plv,
            Mlvc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mlvc {
    #[inline(always)]
    fn default() -> Mlvc {
        <crate::RegValueT<Mlvc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mlvc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plv_SPEC;
    pub type Plv = crate::EnumBitfieldStruct<u8, Plv_SPEC>;
    impl Plv {
        #[doc = "Preemption Link Verification not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Preemption Link Verification requested"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meeec_SPEC;
impl crate::sealed::RegSpec for Meeec_SPEC {
    type DataType = u32;
}

#[doc = "Energy Efficient Ethernet Configuration Register"]
pub type Meeec = crate::RegValueT<Meeec_SPEC>;

impl Meeec {
    #[doc = "LPI Transmit Request"]
    #[inline(always)]
    pub fn lpitr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        meeec::Lpitr,
        meeec::Lpitr,
        Meeec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            meeec::Lpitr,
            meeec::Lpitr,
            Meeec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Meeec {
    #[inline(always)]
    fn default() -> Meeec {
        <crate::RegValueT<Meeec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod meeec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpitr_SPEC;
    pub type Lpitr = crate::EnumBitfieldStruct<u8, Lpitr_SPEC>;
    impl Lpitr {
        #[doc = "LPI mode is not requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "LPI mode is requested"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mlbc_SPEC;
impl crate::sealed::RegSpec for Mlbc_SPEC {
    type DataType = u32;
}

#[doc = "Loopback Configuration Register"]
pub type Mlbc = crate::RegValueT<Mlbc_SPEC>;

impl Mlbc {
    #[doc = "Loopback Mode Enable"]
    #[inline(always)]
    pub fn lbme(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mlbc::Lbme,
        mlbc::Lbme,
        Mlbc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mlbc::Lbme,
            mlbc::Lbme,
            Mlbc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mlbc {
    #[inline(always)]
    fn default() -> Mlbc {
        <crate::RegValueT<Mlbc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mlbc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lbme_SPEC;
    pub type Lbme = crate::EnumBitfieldStruct<u8, Lbme_SPEC>;
    impl Lbme {
        #[doc = "disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meis_SPEC;
impl crate::sealed::RegSpec for Meis_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Status Register"]
pub type Meis = crate::RegValueT<Meis_SPEC>;

impl Meis {
    #[doc = "Transmission Stream Lost Status Flag"]
    #[inline(always)]
    pub fn tsls(self) -> crate::common::RegisterFieldBool<0, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Pause or PFC Frame Reception Error Status Flag"]
    #[inline(always)]
    pub fn pres(self) -> crate::common::RegisterFieldBool<2, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Pause or PFC Frame Retransmit Retry Over Status Flag"]
    #[inline(always)]
    pub fn pfrros(self) -> crate::common::RegisterFieldBool<3, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "False Carrier Detection Status Flag"]
    #[inline(always)]
    pub fn fcds(self) -> crate::common::RegisterFieldBool<4, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX CRC Error Status Flag"]
    #[inline(always)]
    pub fn tces(self) -> crate::common::RegisterFieldBool<5, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Bad CRC Insertion Status Flag"]
    #[inline(always)]
    pub fn tbcis(self) -> crate::common::RegisterFieldBool<6, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Bad Fragment Error Status Flag"]
    #[inline(always)]
    pub fn bfes(self) -> crate::common::RegisterFieldBool<7, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Count Error Status Flag"]
    #[inline(always)]
    pub fn fces(self) -> crate::common::RegisterFieldBool<8, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "E-Frame Overflow Error Status Flag"]
    #[inline(always)]
    pub fn reoes(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        meis::Reoes,
        meis::Reoes,
        Meis_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            meis::Reoes,
            meis::Reoes,
            Meis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "P-Frame Overflow Error Status Flag"]
    #[inline(always)]
    pub fn rpoes(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        meis::Rpoes,
        meis::Rpoes,
        Meis_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            meis::Rpoes,
            meis::Rpoes,
            Meis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Captured Timestamp Lost Error Status Flag n (n = 0 to 1)"]
    #[inline(always)]
    pub fn ctles1_to_ctles0(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Meis_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Data Error Status Flag"]
    #[inline(always)]
    pub fn pdes(self) -> crate::common::RegisterFieldBool<20, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Nibble Alignment Error Status Flag"]
    #[inline(always)]
    pub fn pnaes(self) -> crate::common::RegisterFieldBool<21, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FCS/mCRC Error Status Flag"]
    #[inline(always)]
    pub fn fcmces(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Final Fragment Missing Error Status Flag"]
    #[inline(always)]
    pub fn ffmes(self) -> crate::common::RegisterFieldBool<23, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "C Fragment Count Error Status Flag"]
    #[inline(always)]
    pub fn cfces(self) -> crate::common::RegisterFieldBool<24, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fragment Count Error Status Flag"]
    #[inline(always)]
    pub fn frces(self) -> crate::common::RegisterFieldBool<25, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Reception Partially out of Operation Mode Status Flag"]
    #[inline(always)]
    pub fn rpooms(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Filtered Status Flag"]
    #[inline(always)]
    pub fn ffs(self) -> crate::common::RegisterFieldBool<27, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Undersize Error Status Flag"]
    #[inline(always)]
    pub fn fues(self) -> crate::common::RegisterFieldBool<28, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Oversize Error Status Flag"]
    #[inline(always)]
    pub fn foes(self) -> crate::common::RegisterFieldBool<29, 1, 0, Meis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Meis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Meis {
    #[inline(always)]
    fn default() -> Meis {
        <crate::RegValueT<Meis_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod meis {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reoes_SPEC;
    pub type Reoes = crate::EnumBitfieldStruct<u8, Reoes_SPEC>;
    impl Reoes {
        #[doc = "No overflow error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow error occurred during e-frame reception."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpoes_SPEC;
    pub type Rpoes = crate::EnumBitfieldStruct<u8, Rpoes_SPEC>;
    impl Rpoes {
        #[doc = "No overflow error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow error occurred during p-frame reception."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meie_SPEC;
impl crate::sealed::RegSpec for Meie_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Enable Register"]
pub type Meie = crate::RegValueT<Meie_SPEC>;

impl Meie {
    #[doc = "Transmission Stream Lost Enable"]
    #[inline(always)]
    pub fn tsle(self) -> crate::common::RegisterFieldBool<0, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Pause or PFC Frame Reception Error Enable"]
    #[inline(always)]
    pub fn pree(self) -> crate::common::RegisterFieldBool<2, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Pause or PFC Frame Retransmit Retry Over Enable"]
    #[inline(always)]
    pub fn pfrroe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "False Carrier Detection Enable"]
    #[inline(always)]
    pub fn fcde(self) -> crate::common::RegisterFieldBool<4, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Tx CRC Error Enable"]
    #[inline(always)]
    pub fn tcee(self) -> crate::common::RegisterFieldBool<5, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Bad CRC Insertion Enable"]
    #[inline(always)]
    pub fn tbcie(self) -> crate::common::RegisterFieldBool<6, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Bad Fragment Error Enable"]
    #[inline(always)]
    pub fn bfee(self) -> crate::common::RegisterFieldBool<7, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Count Error Enable"]
    #[inline(always)]
    pub fn fcee(self) -> crate::common::RegisterFieldBool<8, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "E-Frame Overflow Error Enable"]
    #[inline(always)]
    pub fn reoee(self) -> crate::common::RegisterFieldBool<9, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "P-Frame Overflow Error Enable"]
    #[inline(always)]
    pub fn rpoee(self) -> crate::common::RegisterFieldBool<10, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Captured Timestamp Lost Error Enable n (n = 0 to 1)"]
    #[inline(always)]
    pub fn ctlee1_to_ctlee0(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Meie_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Data Error Enable"]
    #[inline(always)]
    pub fn pdee(self) -> crate::common::RegisterFieldBool<20, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Nibble Alignment Error Enable"]
    #[inline(always)]
    pub fn pnaee(self) -> crate::common::RegisterFieldBool<21, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FCS/mCRC Error Enable"]
    #[inline(always)]
    pub fn fcmcee(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Final Fragment Missing Error Enable"]
    #[inline(always)]
    pub fn ffmee(self) -> crate::common::RegisterFieldBool<23, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "C Fragment Count Error Enable"]
    #[inline(always)]
    pub fn cfcee(self) -> crate::common::RegisterFieldBool<24, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fragment Count Error Enable"]
    #[inline(always)]
    pub fn frcee(self) -> crate::common::RegisterFieldBool<25, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Reception Partially out of Operation Mode Enable"]
    #[inline(always)]
    pub fn rpoome(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Filtered Enable"]
    #[inline(always)]
    pub fn ffe(self) -> crate::common::RegisterFieldBool<27, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Undersize Error Enable"]
    #[inline(always)]
    pub fn fuee(self) -> crate::common::RegisterFieldBool<28, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Oversize Error Enable"]
    #[inline(always)]
    pub fn foee(self) -> crate::common::RegisterFieldBool<29, 1, 0, Meie_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Meie_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Meie {
    #[inline(always)]
    fn default() -> Meie {
        <crate::RegValueT<Meie_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meid_SPEC;
impl crate::sealed::RegSpec for Meid_SPEC {
    type DataType = u32;
}

#[doc = "Error Interrupt Disable Register"]
pub type Meid = crate::RegValueT<Meid_SPEC>;

impl Meid {
    #[doc = "Transmission Stream Lost Disable"]
    #[inline(always)]
    pub fn tsld(self) -> crate::common::RegisterFieldBool<0, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Pause or PFC Frame Reception Error Disable"]
    #[inline(always)]
    pub fn pred(self) -> crate::common::RegisterFieldBool<2, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Pause or PFC Frame Frame Retransmit Retry Over Disable"]
    #[inline(always)]
    pub fn pfrrod(self) -> crate::common::RegisterFieldBool<3, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "False Carrier Detection Disable"]
    #[inline(always)]
    pub fn fcdd(self) -> crate::common::RegisterFieldBool<4, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Tx CRC Error Disable"]
    #[inline(always)]
    pub fn tced(self) -> crate::common::RegisterFieldBool<5, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Bad CRC Insertion Disable"]
    #[inline(always)]
    pub fn tbcid(self) -> crate::common::RegisterFieldBool<6, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Bad Fragment Error Disable"]
    #[inline(always)]
    pub fn bfed(self) -> crate::common::RegisterFieldBool<7, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Count Error Disable"]
    #[inline(always)]
    pub fn fced(self) -> crate::common::RegisterFieldBool<8, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "E-Frame Overflow Error Disable"]
    #[inline(always)]
    pub fn reoed(self) -> crate::common::RegisterFieldBool<9, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "P-Frame Overflow Error Disable"]
    #[inline(always)]
    pub fn rpoed(self) -> crate::common::RegisterFieldBool<10, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Captured Timestamp Lost Error Disable n (n = 0 to 1)"]
    #[inline(always)]
    pub fn ctled1_to_ctled0(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Meid_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Data Error Disable"]
    #[inline(always)]
    pub fn pded(self) -> crate::common::RegisterFieldBool<20, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Nibble Alignment Error Disable"]
    #[inline(always)]
    pub fn pnaed(self) -> crate::common::RegisterFieldBool<21, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FCS/mCRC Error Disable"]
    #[inline(always)]
    pub fn fcmced(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Final Fragment Missing Error Disable"]
    #[inline(always)]
    pub fn ffmed(self) -> crate::common::RegisterFieldBool<23, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "C Fragment Count Error Disable"]
    #[inline(always)]
    pub fn cfced(self) -> crate::common::RegisterFieldBool<24, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fragment Count Error Disable"]
    #[inline(always)]
    pub fn frced(self) -> crate::common::RegisterFieldBool<25, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Reception Partially out of Operation Mode Disable"]
    #[inline(always)]
    pub fn rpoomd(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Filtered Disable"]
    #[inline(always)]
    pub fn ffd(self) -> crate::common::RegisterFieldBool<27, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Undersize Error Disable"]
    #[inline(always)]
    pub fn fued(self) -> crate::common::RegisterFieldBool<28, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Oversize Error Disable"]
    #[inline(always)]
    pub fn foed(self) -> crate::common::RegisterFieldBool<29, 1, 0, Meid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Meid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Meid {
    #[inline(always)]
    fn default() -> Meid {
        <crate::RegValueT<Meid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmis0_SPEC;
impl crate::sealed::RegSpec for Mmis0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Status Register 0"]
pub type Mmis0 = crate::RegValueT<Mmis0_SPEC>;

impl Mmis0 {
    #[doc = "PHY Link Signal Change Status Flag"]
    #[inline(always)]
    pub fn plscs(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Interrupt Detection Status Flag"]
    #[inline(always)]
    pub fn pids(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Link Verification Succeed Status Flag"]
    #[inline(always)]
    pub fn lvss(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Link Verification Failed Status Flag"]
    #[inline(always)]
    pub fn lvfs(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Verify Frame Reception Status Flag"]
    #[inline(always)]
    pub fn vfrs(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mmis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mmis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmis0 {
    #[inline(always)]
    fn default() -> Mmis0 {
        <crate::RegValueT<Mmis0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmie0_SPEC;
impl crate::sealed::RegSpec for Mmie0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Enable Register 0"]
pub type Mmie0 = crate::RegValueT<Mmie0_SPEC>;

impl Mmie0 {
    #[doc = "PHY Link Signal Change Enable"]
    #[inline(always)]
    pub fn plsce(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Interrupt Detection Enable"]
    #[inline(always)]
    pub fn pide(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mmie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Link Verification Succeed Enable"]
    #[inline(always)]
    pub fn lvse(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mmie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Link Verification Failed Enable"]
    #[inline(always)]
    pub fn lvfe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mmie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mmie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Verify Frame Reception Enable"]
    #[inline(always)]
    pub fn vfre(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mmie0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mmie0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmie0 {
    #[inline(always)]
    fn default() -> Mmie0 {
        <crate::RegValueT<Mmie0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmid0_SPEC;
impl crate::sealed::RegSpec for Mmid0_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Disable Register 0"]
pub type Mmid0 = crate::RegValueT<Mmid0_SPEC>;

impl Mmid0 {
    #[doc = "PHY Link Signal Change Disable"]
    #[inline(always)]
    pub fn plscd(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Interrupt Detection Disable"]
    #[inline(always)]
    pub fn pidd(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Link Verification Succeed Disable"]
    #[inline(always)]
    pub fn lvsd(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Link Verification Failed Disable"]
    #[inline(always)]
    pub fn lvfd(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Verify Frame Reception Disable"]
    #[inline(always)]
    pub fn vfrd(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mmid0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mmid0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmid0 {
    #[inline(always)]
    fn default() -> Mmid0 {
        <crate::RegValueT<Mmid0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmis1_SPEC;
impl crate::sealed::RegSpec for Mmis1_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Status Register 1"]
pub type Mmis1 = crate::RegValueT<Mmis1_SPEC>;

impl Mmis1 {
    #[doc = "PHY Read Access Completed Status Flag"]
    #[inline(always)]
    pub fn pracs(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Write Access Completed Status Flag"]
    #[inline(always)]
    pub fn pwacs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mmis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Address Access Completed Status Flag"]
    #[inline(always)]
    pub fn paacs(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mmis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Post-Read Access Completed Status Flag"]
    #[inline(always)]
    pub fn ppracs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Mmis1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mmis1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmis1 {
    #[inline(always)]
    fn default() -> Mmis1 {
        <crate::RegValueT<Mmis1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmie1_SPEC;
impl crate::sealed::RegSpec for Mmie1_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Enable Register 1"]
pub type Mmie1 = crate::RegValueT<Mmie1_SPEC>;

impl Mmie1 {
    #[doc = "PHY Read Access Completed Enable"]
    #[inline(always)]
    pub fn prace(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Write Access Completed Enable"]
    #[inline(always)]
    pub fn pwace(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mmie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Runcat Access Completed Enable"]
    #[inline(always)]
    pub fn paace(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mmie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Post-Read Access Completed Enable"]
    #[inline(always)]
    pub fn pprace(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Mmie1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mmie1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmie1 {
    #[inline(always)]
    fn default() -> Mmie1 {
        <crate::RegValueT<Mmie1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmid1_SPEC;
impl crate::sealed::RegSpec for Mmid1_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Disable Register 1"]
pub type Mmid1 = crate::RegValueT<Mmid1_SPEC>;

impl Mmid1 {
    #[doc = "PHY Read Access Completed Disable"]
    #[inline(always)]
    pub fn pracd(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Write Access Completed Disable"]
    #[inline(always)]
    pub fn pwacd(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mmid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Address Access Completed Disable"]
    #[inline(always)]
    pub fn paacd(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mmid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Post-Read Access Completed Disable"]
    #[inline(always)]
    pub fn ppracd(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mmid1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mmid1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmid1 {
    #[inline(always)]
    fn default() -> Mmid1 {
        <crate::RegValueT<Mmid1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmis2_SPEC;
impl crate::sealed::RegSpec for Mmis2_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Status Register 2"]
pub type Mmis2 = crate::RegValueT<Mmis2_SPEC>;

impl Mmis2 {
    #[doc = "Magic Packet Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn mpdis(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmis2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmis2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "LPI Assertion Interrupt Status Flag"]
    #[inline(always)]
    pub fn lpiais(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Mmis2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmis2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "LPI De-Assertion Interrupt Status Flag"]
    #[inline(always)]
    pub fn lpidis(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Mmis2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmis2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmis2 {
    #[inline(always)]
    fn default() -> Mmis2 {
        <crate::RegValueT<Mmis2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmie2_SPEC;
impl crate::sealed::RegSpec for Mmie2_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Enable Register 2"]
pub type Mmie2 = crate::RegValueT<Mmie2_SPEC>;

impl Mmie2 {
    #[doc = "Magic Packet Detection Interrupt Enable"]
    #[inline(always)]
    pub fn mpdie(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmie2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmie2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "LPI Assertion Interrupt Enable"]
    #[inline(always)]
    pub fn lpiaie(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Mmie2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmie2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "LPI De-Assertion Interrupt Enable"]
    #[inline(always)]
    pub fn lpidie(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Mmie2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmie2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmie2 {
    #[inline(always)]
    fn default() -> Mmie2 {
        <crate::RegValueT<Mmie2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmid2_SPEC;
impl crate::sealed::RegSpec for Mmid2_SPEC {
    type DataType = u32;
}

#[doc = "Monitoring Interrupt Disable Register 2"]
pub type Mmid2 = crate::RegValueT<Mmid2_SPEC>;

impl Mmid2 {
    #[doc = "Magic Packet Detection Interrupt Disable"]
    #[inline(always)]
    pub fn mpdid(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mmid2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mmid2_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "LPI Assertion Interrupt Disable"]
    #[inline(always)]
    pub fn lpiaid(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mmid2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mmid2_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "LPI De-Assertion Interrupt Disable"]
    #[inline(always)]
    pub fn lpidid(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mmid2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mmid2_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mmid2 {
    #[inline(always)]
    fn default() -> Mmid2 {
        <crate::RegValueT<Mmid2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpftct_SPEC;
impl crate::sealed::RegSpec for Mmpftct_SPEC {
    type DataType = u32;
}

#[doc = "Manual Pause Frame Transmit Counter Register"]
pub type Mmpftct = crate::RegValueT<Mmpftct_SPEC>;

impl Mmpftct {
    #[doc = "Manual Pause frame Transmit Counter"]
    #[inline(always)]
    pub fn mpftc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mmpftct_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mmpftct_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpftct {
    #[inline(always)]
    fn default() -> Mmpftct {
        <crate::RegValueT<Mmpftct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mapftct_SPEC;
impl crate::sealed::RegSpec for Mapftct_SPEC {
    type DataType = u32;
}

#[doc = "Automatic Pause Frame Transmit Counter Register"]
pub type Mapftct = crate::RegValueT<Mapftct_SPEC>;

impl Mapftct {
    #[doc = "Automatic pause Frame Counter"]
    #[inline(always)]
    pub fn apftc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mapftct_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mapftct_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mapftct {
    #[inline(always)]
    fn default() -> Mapftct {
        <crate::RegValueT<Mapftct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpfrct_SPEC;
impl crate::sealed::RegSpec for Mpfrct_SPEC {
    type DataType = u32;
}

#[doc = "Pause Frame Receive Counter Register"]
pub type Mpfrct = crate::RegValueT<Mpfrct_SPEC>;

impl Mpfrct {
    #[doc = "Pause Frame Receive Counter"]
    #[inline(always)]
    pub fn pfrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mpfrct_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mpfrct_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpfrct {
    #[inline(always)]
    fn default() -> Mpfrct {
        <crate::RegValueT<Mpfrct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfcict_SPEC;
impl crate::sealed::RegSpec for Mfcict_SPEC {
    type DataType = u32;
}

#[doc = "False Carrier Indication Counter Register"]
pub type Mfcict = crate::RegValueT<Mfcict_SPEC>;

impl Mfcict {
    #[doc = "False Carrier Indication Counter"]
    #[inline(always)]
    pub fn fcic(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mfcict_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mfcict_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mfcict {
    #[inline(always)]
    fn default() -> Mfcict {
        <crate::RegValueT<Mfcict_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meeect_SPEC;
impl crate::sealed::RegSpec for Meeect_SPEC {
    type DataType = u32;
}

#[doc = "Energy Efficient Ethernet Counter Register"]
pub type Meeect = crate::RegValueT<Meeect_SPEC>;

impl Meeect {
    #[doc = "Energy Efficient Ethernet Receive Counter"]
    #[inline(always)]
    pub fn eeerc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Meeect_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Meeect_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Meeect {
    #[inline(always)]
    fn default() -> Meeect {
        <crate::RegValueT<Meeect_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpcftct_SPEC;
impl crate::sealed::RegSpec for Mmpcftct_SPEC {
    type DataType = u32;
}

#[doc = "Manual PFC Frame Transmit Counter Register"]
pub type Mmpcftct = crate::RegValueT<Mmpcftct_SPEC>;

impl Mmpcftct {
    #[doc = "Manual PFC frame Transmit Counter"]
    #[inline(always)]
    pub fn mpcfctc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mmpcftct_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mmpcftct_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpcftct {
    #[inline(always)]
    fn default() -> Mmpcftct {
        <crate::RegValueT<Mmpcftct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mapcftct_SPEC;
impl crate::sealed::RegSpec for Mapcftct_SPEC {
    type DataType = u32;
}

#[doc = "Automatic PFC Frame Transmit Counter Register"]
pub type Mapcftct = crate::RegValueT<Mapcftct_SPEC>;

impl Mapcftct {
    #[doc = "Automatic PFC Frame Counter"]
    #[inline(always)]
    pub fn apcfctc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mapcftct_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mapcftct_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mapcftct {
    #[inline(always)]
    fn default() -> Mapcftct {
        <crate::RegValueT<Mapcftct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpcfrct_SPEC;
impl crate::sealed::RegSpec for Mpcfrct_SPEC {
    type DataType = u32;
}

#[doc = "PFC Frame Receive Counter Register"]
pub type Mpcfrct = crate::RegValueT<Mpcfrct_SPEC>;

impl Mpcfrct {
    #[doc = "PFC Frame Receive Counter"]
    #[inline(always)]
    pub fn pcfcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mpcfrct_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mpcfrct_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpcfrct {
    #[inline(always)]
    fn default() -> Mpcfrct {
        <crate::RegValueT<Mpcfrct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrovfc_SPEC;
impl crate::sealed::RegSpec for Mrovfc_SPEC {
    type DataType = u32;
}

#[doc = "Receive Overflow Counter Register"]
pub type Mrovfc = crate::RegValueT<Mrovfc_SPEC>;

impl Mrovfc {
    #[doc = "Receive overflow counter"]
    #[inline(always)]
    pub fn rovfc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrovfc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrovfc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrovfc {
    #[inline(always)]
    fn default() -> Mrovfc {
        <crate::RegValueT<Mrovfc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrgfce_SPEC;
impl crate::sealed::RegSpec for Mrgfce_SPEC {
    type DataType = u32;
}

#[doc = "Received Good Frame Counter E-Frames Register"]
pub type Mrgfce = crate::RegValueT<Mrgfce_SPEC>;

impl Mrgfce {
    #[doc = "Received good frame number E-frames"]
    #[inline(always)]
    pub fn rgfne(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrgfce_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrgfce_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrgfce {
    #[inline(always)]
    fn default() -> Mrgfce {
        <crate::RegValueT<Mrgfce_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrgfcp_SPEC;
impl crate::sealed::RegSpec for Mrgfcp_SPEC {
    type DataType = u32;
}

#[doc = "Received Good Frame Counter P-Frames Register"]
pub type Mrgfcp = crate::RegValueT<Mrgfcp_SPEC>;

impl Mrgfcp {
    #[doc = "Received good frame number P-frames"]
    #[inline(always)]
    pub fn gfnp(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrgfcp_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrgfcp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrgfcp {
    #[inline(always)]
    fn default() -> Mrgfcp {
        <crate::RegValueT<Mrgfcp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrbfc_SPEC;
impl crate::sealed::RegSpec for Mrbfc_SPEC {
    type DataType = u32;
}

#[doc = "Received Good Broadcast Frame Counter Register"]
pub type Mrbfc = crate::RegValueT<Mrbfc_SPEC>;

impl Mrbfc {
    #[doc = "Received good broadcast frame number"]
    #[inline(always)]
    pub fn rbfn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrbfc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrbfc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrbfc {
    #[inline(always)]
    fn default() -> Mrbfc {
        <crate::RegValueT<Mrbfc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrmfc_SPEC;
impl crate::sealed::RegSpec for Mrmfc_SPEC {
    type DataType = u32;
}

#[doc = "Received Good Multicast Frame Counter Register"]
pub type Mrmfc = crate::RegValueT<Mrmfc_SPEC>;

impl Mrmfc {
    #[doc = "Received good multicast frame number"]
    #[inline(always)]
    pub fn rmfn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrmfc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrmfc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrmfc {
    #[inline(always)]
    fn default() -> Mrmfc {
        <crate::RegValueT<Mrmfc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrufc_SPEC;
impl crate::sealed::RegSpec for Mrufc_SPEC {
    type DataType = u32;
}

#[doc = "Received Good Unicast Frame Counter Register"]
pub type Mrufc = crate::RegValueT<Mrufc_SPEC>;

impl Mrufc {
    #[doc = "Received good unicast frame number"]
    #[inline(always)]
    pub fn rufn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrufc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrufc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrufc {
    #[inline(always)]
    fn default() -> Mrufc {
        <crate::RegValueT<Mrufc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrpefc_SPEC;
impl crate::sealed::RegSpec for Mrpefc_SPEC {
    type DataType = u32;
}

#[doc = "Received PHY Error Frame Count Register"]
pub type Mrpefc = crate::RegValueT<Mrpefc_SPEC>;

impl Mrpefc {
    #[doc = "Received PHY error frame number"]
    #[inline(always)]
    pub fn rpefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrpefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrpefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrpefc {
    #[inline(always)]
    fn default() -> Mrpefc {
        <crate::RegValueT<Mrpefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrnefc_SPEC;
impl crate::sealed::RegSpec for Mrnefc_SPEC {
    type DataType = u32;
}

#[doc = "Received Nibble Error Frame Count Register"]
pub type Mrnefc = crate::RegValueT<Mrnefc_SPEC>;

impl Mrnefc {
    #[doc = "Received nibble error frame number"]
    #[inline(always)]
    pub fn rnefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrnefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrnefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrnefc {
    #[inline(always)]
    fn default() -> Mrnefc {
        <crate::RegValueT<Mrnefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrfmefc_SPEC;
impl crate::sealed::RegSpec for Mrfmefc_SPEC {
    type DataType = u32;
}

#[doc = "Received FCS/mCRC Error Frame Count Register"]
pub type Mrfmefc = crate::RegValueT<Mrfmefc_SPEC>;

impl Mrfmefc {
    #[doc = "Received FCS/mCRC error frame number"]
    #[inline(always)]
    pub fn rfmefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrfmefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrfmefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrfmefc {
    #[inline(always)]
    fn default() -> Mrfmefc {
        <crate::RegValueT<Mrfmefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrffmefc_SPEC;
impl crate::sealed::RegSpec for Mrffmefc_SPEC {
    type DataType = u32;
}

#[doc = "Received Final Fragment Missing Error Frame Count Register"]
pub type Mrffmefc = crate::RegValueT<Mrffmefc_SPEC>;

impl Mrffmefc {
    #[doc = "Received final fragment missing error frame number"]
    #[inline(always)]
    pub fn rffmefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrffmefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrffmefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrffmefc {
    #[inline(always)]
    fn default() -> Mrffmefc {
        <crate::RegValueT<Mrffmefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcfcefc_SPEC;
impl crate::sealed::RegSpec for Mrcfcefc_SPEC {
    type DataType = u32;
}

#[doc = "Received C-Fragment Count Error Frame Count Register"]
pub type Mrcfcefc = crate::RegValueT<Mrcfcefc_SPEC>;

impl Mrcfcefc {
    #[doc = "Received C-fragment count error frame number"]
    #[inline(always)]
    pub fn rcfcefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrcfcefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrcfcefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcfcefc {
    #[inline(always)]
    fn default() -> Mrcfcefc {
        <crate::RegValueT<Mrcfcefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrfcefc_SPEC;
impl crate::sealed::RegSpec for Mrfcefc_SPEC {
    type DataType = u32;
}

#[doc = "Received Fragment Count Error Frame Count Register"]
pub type Mrfcefc = crate::RegValueT<Mrfcefc_SPEC>;

impl Mrfcefc {
    #[doc = "Received fragment count error frame number"]
    #[inline(always)]
    pub fn rfcefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrfcefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrfcefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrfcefc {
    #[inline(always)]
    fn default() -> Mrfcefc {
        <crate::RegValueT<Mrfcefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrrcfefc_SPEC;
impl crate::sealed::RegSpec for Mrrcfefc_SPEC {
    type DataType = u32;
}

#[doc = "Received RMAC Filter Error Frame Count Register"]
pub type Mrrcfefc = crate::RegValueT<Mrrcfefc_SPEC>;

impl Mrrcfefc {
    #[doc = "Received RMAC filter error frame number"]
    #[inline(always)]
    pub fn rrcfefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mrrcfefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mrrcfefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrrcfefc {
    #[inline(always)]
    fn default() -> Mrrcfefc {
        <crate::RegValueT<Mrrcfefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrfc_SPEC;
impl crate::sealed::RegSpec for Mrfc_SPEC {
    type DataType = u32;
}

#[doc = "Received Frame Count Register"]
pub type Mrfc = crate::RegValueT<Mrfc_SPEC>;

impl Mrfc {
    #[doc = "Received frame number"]
    #[inline(always)]
    pub fn rfn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrfc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrfc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrfc {
    #[inline(always)]
    fn default() -> Mrfc {
        <crate::RegValueT<Mrfc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrguefc_SPEC;
impl crate::sealed::RegSpec for Mrguefc_SPEC {
    type DataType = u32;
}

#[doc = "Received Good Undersize Error Frame Count Register"]
pub type Mrguefc = crate::RegValueT<Mrguefc_SPEC>;

impl Mrguefc {
    #[doc = "Received good undersize error frame number"]
    #[inline(always)]
    pub fn ruefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrguefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrguefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrguefc {
    #[inline(always)]
    fn default() -> Mrguefc {
        <crate::RegValueT<Mrguefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrbuefc_SPEC;
impl crate::sealed::RegSpec for Mrbuefc_SPEC {
    type DataType = u32;
}

#[doc = "Received Bad Undersize Error Frame Count Register"]
pub type Mrbuefc = crate::RegValueT<Mrbuefc_SPEC>;

impl Mrbuefc {
    #[doc = "Received bad undersize error frame number"]
    #[inline(always)]
    pub fn ruefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrbuefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrbuefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrbuefc {
    #[inline(always)]
    fn default() -> Mrbuefc {
        <crate::RegValueT<Mrbuefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrgoefc_SPEC;
impl crate::sealed::RegSpec for Mrgoefc_SPEC {
    type DataType = u32;
}

#[doc = "Received Good Oversize Error Frame Count Register"]
pub type Mrgoefc = crate::RegValueT<Mrgoefc_SPEC>;

impl Mrgoefc {
    #[doc = "Received good oversize error frame number"]
    #[inline(always)]
    pub fn rgoefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrgoefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrgoefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrgoefc {
    #[inline(always)]
    fn default() -> Mrgoefc {
        <crate::RegValueT<Mrgoefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrboefc_SPEC;
impl crate::sealed::RegSpec for Mrboefc_SPEC {
    type DataType = u32;
}

#[doc = "Received Bad Oversize Error Frame Count Register"]
pub type Mrboefc = crate::RegValueT<Mrboefc_SPEC>;

impl Mrboefc {
    #[doc = "Received bad oversize error frame number"]
    #[inline(always)]
    pub fn rboefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrboefc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrboefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrboefc {
    #[inline(always)]
    fn default() -> Mrboefc {
        <crate::RegValueT<Mrboefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrxbceu_SPEC;
impl crate::sealed::RegSpec for Mrxbceu_SPEC {
    type DataType = u32;
}

#[doc = "Received Byte Counter E-Frames Upper Side Register"]
pub type Mrxbceu = crate::RegValueT<Mrxbceu_SPEC>;

impl Mrxbceu {
    #[doc = "Received byte number E-frames upper side"]
    #[inline(always)]
    pub fn rbneu(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrxbceu_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrxbceu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrxbceu {
    #[inline(always)]
    fn default() -> Mrxbceu {
        <crate::RegValueT<Mrxbceu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrxbcel_SPEC;
impl crate::sealed::RegSpec for Mrxbcel_SPEC {
    type DataType = u32;
}

#[doc = "Received Byte Counter E-Frames Lower Side Register"]
pub type Mrxbcel = crate::RegValueT<Mrxbcel_SPEC>;

impl Mrxbcel {
    #[doc = "Received byte number E-frames lower side"]
    #[inline(always)]
    pub fn rbnel(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrxbcel_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrxbcel_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrxbcel {
    #[inline(always)]
    fn default() -> Mrxbcel {
        <crate::RegValueT<Mrxbcel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrxbcpu_SPEC;
impl crate::sealed::RegSpec for Mrxbcpu_SPEC {
    type DataType = u32;
}

#[doc = "Received Byte Counter P-Frames Upper Side Register"]
pub type Mrxbcpu = crate::RegValueT<Mrxbcpu_SPEC>;

impl Mrxbcpu {
    #[doc = "Received byte number P-frames upper side"]
    #[inline(always)]
    pub fn rbnpu(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrxbcpu_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrxbcpu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrxbcpu {
    #[inline(always)]
    fn default() -> Mrxbcpu {
        <crate::RegValueT<Mrxbcpu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrxbcpl_SPEC;
impl crate::sealed::RegSpec for Mrxbcpl_SPEC {
    type DataType = u32;
}

#[doc = "Received Byte Counter P-Frames Lower Side Register"]
pub type Mrxbcpl = crate::RegValueT<Mrxbcpl_SPEC>;

impl Mrxbcpl {
    #[doc = "Received byte number P-frames lower side"]
    #[inline(always)]
    pub fn rbnpl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mrxbcpl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mrxbcpl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrxbcpl {
    #[inline(always)]
    fn default() -> Mrxbcpl {
        <crate::RegValueT<Mrxbcpl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtgfce_SPEC;
impl crate::sealed::RegSpec for Mtgfce_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Good Frame Counter E-Frames Register"]
pub type Mtgfce = crate::RegValueT<Mtgfce_SPEC>;

impl Mtgfce {
    #[doc = "Transmitted good frame number E-frames"]
    #[inline(always)]
    pub fn tgfne(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtgfce_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtgfce_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtgfce {
    #[inline(always)]
    fn default() -> Mtgfce {
        <crate::RegValueT<Mtgfce_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtgfcp_SPEC;
impl crate::sealed::RegSpec for Mtgfcp_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Good Frame Counter P-Frames Register"]
pub type Mtgfcp = crate::RegValueT<Mtgfcp_SPEC>;

impl Mtgfcp {
    #[doc = "Transmitted good frame number P-frames"]
    #[inline(always)]
    pub fn tgfnp(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtgfcp_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtgfcp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtgfcp {
    #[inline(always)]
    fn default() -> Mtgfcp {
        <crate::RegValueT<Mtgfcp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtbfc_SPEC;
impl crate::sealed::RegSpec for Mtbfc_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Broadcast Frame Counter Register"]
pub type Mtbfc = crate::RegValueT<Mtbfc_SPEC>;

impl Mtbfc {
    #[doc = "Transmitted broadcast frame number"]
    #[inline(always)]
    pub fn tbfn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtbfc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtbfc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtbfc {
    #[inline(always)]
    fn default() -> Mtbfc {
        <crate::RegValueT<Mtbfc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtmfc_SPEC;
impl crate::sealed::RegSpec for Mtmfc_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Multicast Frame Counter Register"]
pub type Mtmfc = crate::RegValueT<Mtmfc_SPEC>;

impl Mtmfc {
    #[doc = "Transmitted multicast frame number"]
    #[inline(always)]
    pub fn tmfn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtmfc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtmfc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtmfc {
    #[inline(always)]
    fn default() -> Mtmfc {
        <crate::RegValueT<Mtmfc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtufc_SPEC;
impl crate::sealed::RegSpec for Mtufc_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Unicast Frame Counter Register"]
pub type Mtufc = crate::RegValueT<Mtufc_SPEC>;

impl Mtufc {
    #[doc = "Transmitted unicast frame number"]
    #[inline(always)]
    pub fn tufn(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtufc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtufc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtufc {
    #[inline(always)]
    fn default() -> Mtufc {
        <crate::RegValueT<Mtufc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtefc_SPEC;
impl crate::sealed::RegSpec for Mtefc_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Error Frame Counter Register"]
pub type Mtefc = crate::RegValueT<Mtefc_SPEC>;

impl Mtefc {
    #[doc = "Transmitted error frame number"]
    #[inline(always)]
    pub fn tefn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mtefc_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mtefc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtefc {
    #[inline(always)]
    fn default() -> Mtefc {
        <crate::RegValueT<Mtefc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtxbceu_SPEC;
impl crate::sealed::RegSpec for Mtxbceu_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Byte Counter E-Frames Upper Side Register"]
pub type Mtxbceu = crate::RegValueT<Mtxbceu_SPEC>;

impl Mtxbceu {
    #[doc = "Transmitted byte number E-frames upper side"]
    #[inline(always)]
    pub fn tbneu(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtxbceu_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtxbceu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtxbceu {
    #[inline(always)]
    fn default() -> Mtxbceu {
        <crate::RegValueT<Mtxbceu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtxbcel_SPEC;
impl crate::sealed::RegSpec for Mtxbcel_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Byte Counter E-Frames Lower Side Register"]
pub type Mtxbcel = crate::RegValueT<Mtxbcel_SPEC>;

impl Mtxbcel {
    #[doc = "Transmitted byte number E-frames lower side"]
    #[inline(always)]
    pub fn tbnel(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtxbcel_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtxbcel_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtxbcel {
    #[inline(always)]
    fn default() -> Mtxbcel {
        <crate::RegValueT<Mtxbcel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtxbcpu_SPEC;
impl crate::sealed::RegSpec for Mtxbcpu_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Byte Counter P-Frames Upper Side Register"]
pub type Mtxbcpu = crate::RegValueT<Mtxbcpu_SPEC>;

impl Mtxbcpu {
    #[doc = "Transmitted byte number P-frames upper side"]
    #[inline(always)]
    pub fn tbnpu(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtxbcpu_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtxbcpu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtxbcpu {
    #[inline(always)]
    fn default() -> Mtxbcpu {
        <crate::RegValueT<Mtxbcpu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtxbcpl_SPEC;
impl crate::sealed::RegSpec for Mtxbcpl_SPEC {
    type DataType = u32;
}

#[doc = "Transmitted Byte Counter P-Frames Lower Side Register"]
pub type Mtxbcpl = crate::RegValueT<Mtxbcpl_SPEC>;

impl Mtxbcpl {
    #[doc = "Transmitted byte number P-frames lower side"]
    #[inline(always)]
    pub fn tbnpl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mtxbcpl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mtxbcpl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mtxbcpl {
    #[inline(always)]
    fn default() -> Mtxbcpl {
        <crate::RegValueT<Mtxbcpl_SPEC> as RegisterValue<_>>::new(0)
    }
}
