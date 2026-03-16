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
// Generated from SVD 1.00.01, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:40:06 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Octa Serial Peripheral Interface 0"]
unsafe impl ::core::marker::Send for super::Ospi0B {}
unsafe impl ::core::marker::Sync for super::Ospi0B {}
impl super::Ospi0B {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "OSPI Wrapper Configuration Register"]
    #[inline(always)]
    pub const fn wrapcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Wrapcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wrapcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "OSPI Common Configuration Register"]
    #[inline(always)]
    pub const fn comcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Comcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Comcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "OSPI Bridge Map Configuration Register chn (n = 0, 1)"]
    #[inline(always)]
    pub const fn bmcfgch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bmcfgch_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8usize))
        }
    }
    #[inline(always)]
    pub const fn bmcfgch0(
        &self,
    ) -> &'static crate::common::Reg<self::Bmcfgch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bmcfgch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bmcfgch1(
        &self,
    ) -> &'static crate::common::Reg<self::Bmcfgch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bmcfgch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }

    #[doc = "OSPI Command Map Configuration Register 0 CSn"]
    #[inline(always)]
    pub const fn cmcfg0cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cmcfg0Cs_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10usize))
        }
    }
    #[inline(always)]
    pub const fn cmcfg0cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Cmcfg0Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmcfg0Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cmcfg0cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Cmcfg0Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmcfg0Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }

    #[doc = "OSPI Command Map Configuration Register 1 CSn"]
    #[inline(always)]
    pub const fn cmcfg1cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cmcfg1Cs_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x14usize))
        }
    }
    #[inline(always)]
    pub const fn cmcfg1cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Cmcfg1Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmcfg1Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cmcfg1cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Cmcfg1Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmcfg1Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }

    #[doc = "OSPI Command Map Configuration Register 2 CSn"]
    #[inline(always)]
    pub const fn cmcfg2cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cmcfg2Cs_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18usize))
        }
    }
    #[inline(always)]
    pub const fn cmcfg2cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Cmcfg2Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmcfg2Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cmcfg2cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Cmcfg2Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmcfg2Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }

    #[doc = "OSPI Link I/O Configuration Register CSn"]
    #[inline(always)]
    pub const fn liocfgcs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Liocfgcs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x50usize))
        }
    }
    #[inline(always)]
    pub const fn liocfgcs0(
        &self,
    ) -> &'static crate::common::Reg<self::Liocfgcs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Liocfgcs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn liocfgcs1(
        &self,
    ) -> &'static crate::common::Reg<self::Liocfgcs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Liocfgcs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }

    #[doc = "OSPI Bridge Map Control Register 0"]
    #[inline(always)]
    pub const fn bmctl0(
        &self,
    ) -> &'static crate::common::Reg<self::Bmctl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bmctl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "OSPI Bridge Map Control Register 1"]
    #[inline(always)]
    pub const fn bmctl1(&self) -> &'static crate::common::Reg<self::Bmctl1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Bmctl1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "OSPI Command Map Control register chn (n = 0, 1)"]
    #[inline(always)]
    pub const fn cmctlch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cmctlch_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x68usize))
        }
    }
    #[inline(always)]
    pub const fn cmctlch0(
        &self,
    ) -> &'static crate::common::Reg<self::Cmctlch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmctlch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cmctlch1(
        &self,
    ) -> &'static crate::common::Reg<self::Cmctlch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmctlch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6cusize),
            )
        }
    }

    #[doc = "OSPI Command Manual Control Register 0"]
    #[inline(always)]
    pub const fn cdctl0(
        &self,
    ) -> &'static crate::common::Reg<self::Cdctl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdctl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "OSPI Command Manual Control Register 1"]
    #[inline(always)]
    pub const fn cdctl1(
        &self,
    ) -> &'static crate::common::Reg<self::Cdctl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdctl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "OSPI Command Manual Control Register 2"]
    #[inline(always)]
    pub const fn cdctl2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdctl2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdctl2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "OSPI Command Manual Type Buf %s"]
    #[inline(always)]
    pub const fn cdtbuf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cdtbuf_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn cdtbuf0(
        &self,
    ) -> &'static crate::common::Reg<self::Cdtbuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdtbuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdtbuf1(
        &self,
    ) -> &'static crate::common::Reg<self::Cdtbuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdtbuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdtbuf2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdtbuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdtbuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdtbuf3(
        &self,
    ) -> &'static crate::common::Reg<self::Cdtbuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdtbuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb0usize),
            )
        }
    }

    #[doc = "OSPI Command Manual Address Buf %s"]
    #[inline(always)]
    pub const fn cdabuf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cdabuf_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x84usize))
        }
    }
    #[inline(always)]
    pub const fn cdabuf0(
        &self,
    ) -> &'static crate::common::Reg<self::Cdabuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdabuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdabuf1(
        &self,
    ) -> &'static crate::common::Reg<self::Cdabuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdabuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdabuf2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdabuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdabuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdabuf3(
        &self,
    ) -> &'static crate::common::Reg<self::Cdabuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdabuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb4usize),
            )
        }
    }

    #[doc = "OSPI Command Manual Data 0 Buf %s"]
    #[inline(always)]
    pub const fn cdd0buf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cdd0Buf_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }
    #[inline(always)]
    pub const fn cdd0buf0(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd0Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd0Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdd0buf1(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd0Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd0Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdd0buf2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd0Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd0Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdd0buf3(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd0Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd0Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xb8usize),
            )
        }
    }

    #[doc = "OSPI Command Manual Data 1 Buf %s"]
    #[inline(always)]
    pub const fn cdd1buf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cdd1Buf_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8cusize))
        }
    }
    #[inline(always)]
    pub const fn cdd1buf0(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd1Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd1Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdd1buf1(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd1Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd1Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdd1buf2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd1Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd1Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xacusize),
            )
        }
    }
    #[inline(always)]
    pub const fn cdd1buf3(
        &self,
    ) -> &'static crate::common::Reg<self::Cdd1Buf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdd1Buf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbcusize),
            )
        }
    }

    #[doc = "OSPI Link Pattern Control Register 0"]
    #[inline(always)]
    pub const fn lpctl0(
        &self,
    ) -> &'static crate::common::Reg<self::Lpctl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lpctl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "OSPI Link Pattern Control Register 1"]
    #[inline(always)]
    pub const fn lpctl1(
        &self,
    ) -> &'static crate::common::Reg<self::Lpctl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lpctl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "OSPI Link I/O Control Register"]
    #[inline(always)]
    pub const fn lioctl(
        &self,
    ) -> &'static crate::common::Reg<self::Lioctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lioctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 0 CSn"]
    #[inline(always)]
    pub const fn ccctl0cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl0Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x130usize))
        }
    }
    #[inline(always)]
    pub const fn ccctl0cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl0Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl0Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl0cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl0Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl0Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x150usize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 1 CSn"]
    #[inline(always)]
    pub const fn ccctl1cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl1Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x134usize))
        }
    }
    #[inline(always)]
    pub const fn ccctl1cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl1Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl1Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl1cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl1Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl1Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x154usize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 2 CSn"]
    #[inline(always)]
    pub const fn ccctl2cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl2Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x138usize))
        }
    }
    #[inline(always)]
    pub const fn ccctl2cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl2Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl2Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl2cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl2Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl2Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x158usize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 3 CSn"]
    #[inline(always)]
    pub const fn ccctl3cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl3Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13cusize))
        }
    }
    #[inline(always)]
    pub const fn ccctl3cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl3Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl3Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x13cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl3cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl3Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl3Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x15cusize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 4 CSn"]
    #[inline(always)]
    pub const fn ccctl4cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl4Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }
    #[inline(always)]
    pub const fn ccctl4cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl4Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl4Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl4cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl4Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl4Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 5 CSn"]
    #[inline(always)]
    pub const fn ccctl5cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl5Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x144usize))
        }
    }
    #[inline(always)]
    pub const fn ccctl5cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl5Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl5Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl5cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl5Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl5Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 6 CSn"]
    #[inline(always)]
    pub const fn ccctl6cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl6Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x148usize))
        }
    }
    #[inline(always)]
    pub const fn ccctl6cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl6Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl6Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl6cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl6Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl6Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }

    #[doc = "OSPI Command Calibration Control Register 7 CSn"]
    #[inline(always)]
    pub const fn ccctl7cs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ccctl7Cs_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x14cusize))
        }
    }
    #[inline(always)]
    pub const fn ccctl7cs0(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl7Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl7Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn ccctl7cs1(
        &self,
    ) -> &'static crate::common::Reg<self::Ccctl7Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccctl7Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x16cusize),
            )
        }
    }

    #[doc = "OSPI Common Status Register"]
    #[inline(always)]
    pub const fn comstt(&self) -> &'static crate::common::Reg<self::Comstt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Comstt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[doc = "OSPI Calibration Status Register CSn"]
    #[inline(always)]
    pub const fn casttcs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Casttcs_SPEC, crate::common::R>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x188usize))
        }
    }
    #[inline(always)]
    pub const fn casttcs0(
        &self,
    ) -> &'static crate::common::Reg<self::Casttcs_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Casttcs_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn casttcs1(
        &self,
    ) -> &'static crate::common::Reg<self::Casttcs_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Casttcs_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x18cusize),
            )
        }
    }

    #[doc = "OSPI Interrupt Status Register"]
    #[inline(always)]
    pub const fn ints(&self) -> &'static crate::common::Reg<self::Ints_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ints_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[doc = "OSPI Interrupt Clear Register"]
    #[inline(always)]
    pub const fn intc(&self) -> &'static crate::common::Reg<self::Intc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Intc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(404usize),
            )
        }
    }

    #[doc = "OSPI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inte(&self) -> &'static crate::common::Reg<self::Inte_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inte_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(408usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrapcfg_SPEC;
impl crate::sealed::RegSpec for Wrapcfg_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Wrapper Configuration Register"]
pub type Wrapcfg = crate::RegValueT<Wrapcfg_SPEC>;

impl Wrapcfg {
    #[doc = "OM_DQS shift for slave0"]
    #[inline(always)]
    pub fn dssftcs0(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Wrapcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Wrapcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "OM_DQS shift for slave1"]
    #[inline(always)]
    pub fn dssftcs1(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Wrapcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Wrapcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Wrapcfg {
    #[inline(always)]
    fn default() -> Wrapcfg {
        <crate::RegValueT<Wrapcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comcfg_SPEC;
impl crate::sealed::RegSpec for Comcfg_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Common Configuration Register"]
pub type Comcfg = crate::RegValueT<Comcfg_SPEC>;

impl Comcfg {
    #[doc = "Channel arbitration mode"]
    #[inline(always)]
    pub fn arbmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        comcfg::Arbmd,
        comcfg::Arbmd,
        Comcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            comcfg::Arbmd,
            comcfg::Arbmd,
            Comcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output Enable Asserting extension"]
    #[inline(always)]
    pub fn oeastex(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        comcfg::Oeastex,
        comcfg::Oeastex,
        Comcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            comcfg::Oeastex,
            comcfg::Oeastex,
            Comcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output Enable Negating extension"]
    #[inline(always)]
    pub fn oenegex(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        comcfg::Oenegex,
        comcfg::Oenegex,
        Comcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            comcfg::Oenegex,
            comcfg::Oenegex,
            Comcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Comcfg {
    #[inline(always)]
    fn default() -> Comcfg {
        <crate::RegValueT<Comcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod comcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmd_SPEC;
    pub type Arbmd = crate::EnumBitfieldStruct<u8, Arbmd_SPEC>;
    impl Arbmd {
        #[doc = "Round-Robbin (ch0-ch1-ch0-ch1…)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Always ch0 win"]
        pub const _01: Self = Self::new(1);

        #[doc = "Always ch1 win"]
        pub const _10: Self = Self::new(2);

        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oeastex_SPEC;
    pub type Oeastex = crate::EnumBitfieldStruct<u8, Oeastex_SPEC>;
    impl Oeastex {
        #[doc = "No extend 1 cycle Output enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend 1 cycle Output enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oenegex_SPEC;
    pub type Oenegex = crate::EnumBitfieldStruct<u8, Oenegex_SPEC>;
    impl Oenegex {
        #[doc = "No extend 1 cycle Output enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend 1 cycle Output enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmcfgch_SPEC;
impl crate::sealed::RegSpec for Bmcfgch_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Bridge Map Configuration Register chn (n = 0, 1)"]
pub type Bmcfgch = crate::RegValueT<Bmcfgch_SPEC>;

impl Bmcfgch {
    #[doc = "System bus Write Response mode"]
    #[inline(always)]
    pub fn wrmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bmcfgch::Wrmd,
        bmcfgch::Wrmd,
        Bmcfgch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bmcfgch::Wrmd,
            bmcfgch::Wrmd,
            Bmcfgch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Memory Write Combination mode"]
    #[inline(always)]
    pub fn mwrcomb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        bmcfgch::Mwrcomb,
        bmcfgch::Mwrcomb,
        Bmcfgch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            bmcfgch::Mwrcomb,
            bmcfgch::Mwrcomb,
            Bmcfgch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Memory Write Size"]
    #[inline(always)]
    pub fn mwrsize(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Bmcfgch_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Bmcfgch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Prefetch enable"]
    #[inline(always)]
    pub fn preen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bmcfgch::Preen,
        bmcfgch::Preen,
        Bmcfgch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bmcfgch::Preen,
            bmcfgch::Preen,
            Bmcfgch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Combination timer"]
    #[inline(always)]
    pub fn cmbtim(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Bmcfgch_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Bmcfgch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bmcfgch {
    #[inline(always)]
    fn default() -> Bmcfgch {
        <crate::RegValueT<Bmcfgch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bmcfgch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrmd_SPEC;
    pub type Wrmd = crate::EnumBitfieldStruct<u8, Wrmd_SPEC>;
    impl Wrmd {
        #[doc = "Return response after storing to Internal Write Buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Return response after issuing write transaction to xSPI bus"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mwrcomb_SPEC;
    pub type Mwrcomb = crate::EnumBitfieldStruct<u8, Mwrcomb_SPEC>;
    impl Mwrcomb {
        #[doc = "Disable combination mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable combination mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Preen_SPEC;
    pub type Preen = crate::EnumBitfieldStruct<u8, Preen_SPEC>;
    impl Preen {
        #[doc = "Disable prefetch function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable prefetch function"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcfg0Cs_SPEC;
impl crate::sealed::RegSpec for Cmcfg0Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Map Configuration Register 0 CSn"]
pub type Cmcfg0Cs = crate::RegValueT<Cmcfg0Cs_SPEC>;

impl Cmcfg0Cs {
    #[doc = "Frame format"]
    #[inline(always)]
    pub fn ffmt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cmcfg0cs::Ffmt,
        cmcfg0cs::Ffmt,
        Cmcfg0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cmcfg0cs::Ffmt,
            cmcfg0cs::Ffmt,
            Cmcfg0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address size"]
    #[inline(always)]
    pub fn addsize(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        cmcfg0cs::Addsize,
        cmcfg0cs::Addsize,
        Cmcfg0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            cmcfg0cs::Addsize,
            cmcfg0cs::Addsize,
            Cmcfg0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Wrapping burst mode"]
    #[inline(always)]
    pub fn wpbstmd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cmcfg0cs::Wpbstmd,
        cmcfg0cs::Wpbstmd,
        Cmcfg0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cmcfg0cs::Wpbstmd,
            cmcfg0cs::Wpbstmd,
            Cmcfg0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Array address mode"]
    #[inline(always)]
    pub fn aryamd(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cmcfg0cs::Aryamd,
        cmcfg0cs::Aryamd,
        Cmcfg0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cmcfg0cs::Aryamd,
            cmcfg0cs::Aryamd,
            Cmcfg0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Replace Enable"]
    #[inline(always)]
    pub fn addrpen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cmcfg0cs::Addrpen,
        cmcfg0cs::Addrpen,
        Cmcfg0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cmcfg0cs::Addrpen,
            cmcfg0cs::Addrpen,
            Cmcfg0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Replace Code"]
    #[inline(always)]
    pub fn addrpcd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cmcfg0Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cmcfg0Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmcfg0Cs {
    #[inline(always)]
    fn default() -> Cmcfg0Cs {
        <crate::RegValueT<Cmcfg0Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmcfg0cs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ffmt_SPEC;
    pub type Ffmt = crate::EnumBitfieldStruct<u8, Ffmt_SPEC>;
    impl Ffmt {
        #[doc = "Normal format: Command 1 byte, Address ADDSIZE, Data up to system bus transaction."]
        pub const _00: Self = Self::new(0);

        #[doc = "8D-8D-8D profile 1.0 format: Command 2 bytes, Address ADDSIZE, Data up to system bus transaction"]
        pub const _01: Self = Self::new(1);

        #[doc = "8D-8D-8D profile 2.0 Command Modifier format: Command & Modifier 6 bytes, Data up to system bus transaction"]
        pub const _10: Self = Self::new(2);

        #[doc = "8D-8D-8D profile 2.0 Commands with Extended Command Modifier format: Command & Modifier 6 bytes, Data up to system bus transaction"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addsize_SPEC;
    pub type Addsize = crate::EnumBitfieldStruct<u8, Addsize_SPEC>;
    impl Addsize {
        #[doc = "1 byte (256-byte address space)"]
        pub const _00: Self = Self::new(0);

        #[doc = "2 bytes (64 KB address space)"]
        pub const _01: Self = Self::new(1);

        #[doc = "3 bytes (16 MB address space)"]
        pub const _10: Self = Self::new(2);

        #[doc = "4 bytes (4 GB address space)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpbstmd_SPEC;
    pub type Wpbstmd = crate::EnumBitfieldStruct<u8, Wpbstmd_SPEC>;
    impl Wpbstmd {
        #[doc = "Separate xSPI transfer at the wrapping address boundary"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not separate xSPI transfer at the wrapping address boundary"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aryamd_SPEC;
    pub type Aryamd = crate::EnumBitfieldStruct<u8, Aryamd_SPEC>;
    impl Aryamd {
        #[doc = "Normal address mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Array address mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addrpen_SPEC;
    pub type Addrpen = crate::EnumBitfieldStruct<u8, Addrpen_SPEC>;
    impl Addrpen {
        #[doc = "No replacement (xSPI frame address field is same as System bus address)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Replacement"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcfg1Cs_SPEC;
impl crate::sealed::RegSpec for Cmcfg1Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Map Configuration Register 1 CSn"]
pub type Cmcfg1Cs = crate::RegValueT<Cmcfg1Cs_SPEC>;

impl Cmcfg1Cs {
    #[doc = "Read command"]
    #[inline(always)]
    pub fn rdcmd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cmcfg1Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cmcfg1Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Read latency cycle"]
    #[inline(always)]
    pub fn rdlate(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Cmcfg1Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Cmcfg1Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmcfg1Cs {
    #[inline(always)]
    fn default() -> Cmcfg1Cs {
        <crate::RegValueT<Cmcfg1Cs_SPEC> as RegisterValue<_>>::new(524288)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcfg2Cs_SPEC;
impl crate::sealed::RegSpec for Cmcfg2Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Map Configuration Register 2 CSn"]
pub type Cmcfg2Cs = crate::RegValueT<Cmcfg2Cs_SPEC>;

impl Cmcfg2Cs {
    #[doc = "Write command"]
    #[inline(always)]
    pub fn wrcmd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cmcfg2Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cmcfg2Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write latency cycle"]
    #[inline(always)]
    pub fn wrlate(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Cmcfg2Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Cmcfg2Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmcfg2Cs {
    #[inline(always)]
    fn default() -> Cmcfg2Cs {
        <crate::RegValueT<Cmcfg2Cs_SPEC> as RegisterValue<_>>::new(524288)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Liocfgcs_SPEC;
impl crate::sealed::RegSpec for Liocfgcs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Link I/O Configuration Register CSn"]
pub type Liocfgcs = crate::RegValueT<Liocfgcs_SPEC>;

impl Liocfgcs {
    #[doc = "Protocol mode"]
    #[inline(always)]
    pub fn prtmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        liocfgcs::Prtmd,
        liocfgcs::Prtmd,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            liocfgcs::Prtmd,
            liocfgcs::Prtmd,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Latency mode"]
    #[inline(always)]
    pub fn latemd(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Liocfgcs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Liocfgcs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write mask mode"]
    #[inline(always)]
    pub fn wrmskmd(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        liocfgcs::Wrmskmd,
        liocfgcs::Wrmskmd,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            liocfgcs::Wrmskmd,
            liocfgcs::Wrmskmd,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CS minimum idle term"]
    #[inline(always)]
    pub fn csmin(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Liocfgcs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Liocfgcs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CS asserting extension"]
    #[inline(always)]
    pub fn csastex(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        liocfgcs::Csastex,
        liocfgcs::Csastex,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            liocfgcs::Csastex,
            liocfgcs::Csastex,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CS negating extension"]
    #[inline(always)]
    pub fn csnegex(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        liocfgcs::Csnegex,
        liocfgcs::Csnegex,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            liocfgcs::Csnegex,
            liocfgcs::Csnegex,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDR driving timing"]
    #[inline(always)]
    pub fn sdrdrv(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        liocfgcs::Sdrdrv,
        liocfgcs::Sdrdrv,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            liocfgcs::Sdrdrv,
            liocfgcs::Sdrdrv,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDR Sampling mode"]
    #[inline(always)]
    pub fn sdrsmpmd(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        liocfgcs::Sdrsmpmd,
        liocfgcs::Sdrsmpmd,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            liocfgcs::Sdrsmpmd,
            liocfgcs::Sdrsmpmd,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDR Sampling window shift"]
    #[inline(always)]
    pub fn sdrsmpsft(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Liocfgcs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Liocfgcs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DDR sampling window extend"]
    #[inline(always)]
    pub fn ddrsmpex(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Liocfgcs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Liocfgcs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Liocfgcs {
    #[inline(always)]
    fn default() -> Liocfgcs {
        <crate::RegValueT<Liocfgcs_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod liocfgcs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prtmd_SPEC;
    pub type Prtmd = crate::EnumBitfieldStruct<u16, Prtmd_SPEC>;
    impl Prtmd {
        #[doc = "1S-1S-1S"]
        pub const _0_X_000: Self = Self::new(0);

        #[doc = "4S-4D-4D"]
        pub const _0_X_3_B_2: Self = Self::new(946);

        #[doc = "8D-8D-8D"]
        pub const _0_X_3_FF: Self = Self::new(1023);

        #[doc = "1S-2S-2S"]
        pub const _0_X_048: Self = Self::new(72);

        #[doc = "2S-2S-2S"]
        pub const _0_X_049: Self = Self::new(73);

        #[doc = "1S-4S-4S"]
        pub const _0_X_090: Self = Self::new(144);

        #[doc = "4S-4S-4S"]
        pub const _0_X_092: Self = Self::new(146);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrmskmd_SPEC;
    pub type Wrmskmd = crate::EnumBitfieldStruct<u8, Wrmskmd_SPEC>;
    impl Wrmskmd {
        #[doc = "Write mask disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write mask enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csastex_SPEC;
    pub type Csastex = crate::EnumBitfieldStruct<u8, Csastex_SPEC>;
    impl Csastex {
        #[doc = "No extension"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend 1 cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csnegex_SPEC;
    pub type Csnegex = crate::EnumBitfieldStruct<u8, Csnegex_SPEC>;
    impl Csnegex {
        #[doc = "No extension"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend 1 cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdrdrv_SPEC;
    pub type Sdrdrv = crate::EnumBitfieldStruct<u8, Sdrdrv_SPEC>;
    impl Sdrdrv {
        #[doc = "Drive at 1/2 cycle before CK rising-edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Drive at CK rising-edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdrsmpmd_SPEC;
    pub type Sdrsmpmd = crate::EnumBitfieldStruct<u8, Sdrsmpmd_SPEC>;
    impl Sdrsmpmd {
        #[doc = "Samples data input at falling-edge"]
        pub const _0: Self = Self::new(0);

        #[doc = "Samples data input at rising-edge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmctl0_SPEC;
impl crate::sealed::RegSpec for Bmctl0_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Bridge Map Control Register 0"]
pub type Bmctl0 = crate::RegValueT<Bmctl0_SPEC>;

impl Bmctl0 {
    #[doc = "System bus ch0 to slave0 memory area access enable"]
    #[inline(always)]
    pub fn ch0cs0acc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        bmctl0::Ch0Cs0Acc,
        bmctl0::Ch0Cs0Acc,
        Bmctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            bmctl0::Ch0Cs0Acc,
            bmctl0::Ch0Cs0Acc,
            Bmctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System bus ch0 to slave1 memory area access enable"]
    #[inline(always)]
    pub fn ch0cs1acc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        bmctl0::Ch0Cs1Acc,
        bmctl0::Ch0Cs1Acc,
        Bmctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            bmctl0::Ch0Cs1Acc,
            bmctl0::Ch0Cs1Acc,
            Bmctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System bus ch1 to slave0 memory area access enable"]
    #[inline(always)]
    pub fn ch1cs0acc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        bmctl0::Ch1Cs0Acc,
        bmctl0::Ch1Cs0Acc,
        Bmctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            bmctl0::Ch1Cs0Acc,
            bmctl0::Ch1Cs0Acc,
            Bmctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System bus ch1 to slave1 memory area access enable"]
    #[inline(always)]
    pub fn ch1cs1acc(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        bmctl0::Ch1Cs1Acc,
        bmctl0::Ch1Cs1Acc,
        Bmctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            bmctl0::Ch1Cs1Acc,
            bmctl0::Ch1Cs1Acc,
            Bmctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bmctl0 {
    #[inline(always)]
    fn default() -> Bmctl0 {
        <crate::RegValueT<Bmctl0_SPEC> as RegisterValue<_>>::new(255)
    }
}
pub mod bmctl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch0Cs0Acc_SPEC;
    pub type Ch0Cs0Acc = crate::EnumBitfieldStruct<u8, Ch0Cs0Acc_SPEC>;
    impl Ch0Cs0Acc {
        #[doc = "Read/Write disable"]
        pub const _00: Self = Self::new(0);

        #[doc = "Read enable, Write disable"]
        pub const _01: Self = Self::new(1);

        #[doc = "Read disable, Write enable"]
        pub const _10: Self = Self::new(2);

        #[doc = "Read/Write enable"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch0Cs1Acc_SPEC;
    pub type Ch0Cs1Acc = crate::EnumBitfieldStruct<u8, Ch0Cs1Acc_SPEC>;
    impl Ch0Cs1Acc {
        #[doc = "Read/Write disable"]
        pub const _00: Self = Self::new(0);

        #[doc = "Read enable, Write disable"]
        pub const _01: Self = Self::new(1);

        #[doc = "Read disable, Write enable"]
        pub const _10: Self = Self::new(2);

        #[doc = "Read/Write enable"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch1Cs0Acc_SPEC;
    pub type Ch1Cs0Acc = crate::EnumBitfieldStruct<u8, Ch1Cs0Acc_SPEC>;
    impl Ch1Cs0Acc {
        #[doc = "Read/Write disable"]
        pub const _00: Self = Self::new(0);

        #[doc = "Read enable, Write disable"]
        pub const _01: Self = Self::new(1);

        #[doc = "Read disable, Write enable"]
        pub const _10: Self = Self::new(2);

        #[doc = "Read/Write enable"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch1Cs1Acc_SPEC;
    pub type Ch1Cs1Acc = crate::EnumBitfieldStruct<u8, Ch1Cs1Acc_SPEC>;
    impl Ch1Cs1Acc {
        #[doc = "Read/Write disable"]
        pub const _00: Self = Self::new(0);

        #[doc = "Read enable, Write disable"]
        pub const _01: Self = Self::new(1);

        #[doc = "Read disable, Write enable"]
        pub const _10: Self = Self::new(2);

        #[doc = "Read/Write enable"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmctl1_SPEC;
impl crate::sealed::RegSpec for Bmctl1_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Bridge Map Control Register 1"]
pub type Bmctl1 = crate::RegValueT<Bmctl1_SPEC>;

impl Bmctl1 {
    #[doc = "Memory Write Data Push for ch0"]
    #[inline(always)]
    pub fn mwrpushch0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bmctl1::Mwrpushch0,
        bmctl1::Mwrpushch0,
        Bmctl1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bmctl1::Mwrpushch0,
            bmctl1::Mwrpushch0,
            Bmctl1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Memory Write Data Push for ch1"]
    #[inline(always)]
    pub fn mwrpushch1(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Bmctl1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Bmctl1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Prefetch Buffer clear for ch0"]
    #[inline(always)]
    pub fn pbufclrch0(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        bmctl1::Pbufclrch0,
        bmctl1::Pbufclrch0,
        Bmctl1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            bmctl1::Pbufclrch0,
            bmctl1::Pbufclrch0,
            Bmctl1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Prefetch Buffer clear for ch1"]
    #[inline(always)]
    pub fn pbufclrch1(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Bmctl1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Bmctl1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Bmctl1 {
    #[inline(always)]
    fn default() -> Bmctl1 {
        <crate::RegValueT<Bmctl1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bmctl1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mwrpushch0_SPEC;
    pub type Mwrpushch0 = crate::EnumBitfieldStruct<u8, Mwrpushch0_SPEC>;
    impl Mwrpushch0 {
        #[doc = "No command"]
        pub const _0: Self = Self::new(0);

        #[doc = "Push request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pbufclrch0_SPEC;
    pub type Pbufclrch0 = crate::EnumBitfieldStruct<u8, Pbufclrch0_SPEC>;
    impl Pbufclrch0 {
        #[doc = "No command"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmctlch_SPEC;
impl crate::sealed::RegSpec for Cmctlch_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Map Control register chn (n = 0, 1)"]
pub type Cmctlch = crate::RegValueT<Cmctlch_SPEC>;

impl Cmctlch {
    #[doc = "XiP mode enter code"]
    #[inline(always)]
    pub fn xipencode(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cmctlch_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cmctlch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "XiP mode exit code"]
    #[inline(always)]
    pub fn xipexcode(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cmctlch_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cmctlch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "XiP mode enable"]
    #[inline(always)]
    pub fn xipen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cmctlch::Xipen,
        cmctlch::Xipen,
        Cmctlch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cmctlch::Xipen,
            cmctlch::Xipen,
            Cmctlch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmctlch {
    #[inline(always)]
    fn default() -> Cmctlch {
        <crate::RegValueT<Cmctlch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmctlch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Xipen_SPEC;
    pub type Xipen = crate::EnumBitfieldStruct<u8, Xipen_SPEC>;
    impl Xipen {
        #[doc = "Disable XiP mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable XiP mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdctl0_SPEC;
impl crate::sealed::RegSpec for Cdctl0_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Manual Control Register 0"]
pub type Cdctl0 = crate::RegValueT<Cdctl0_SPEC>;

impl Cdctl0 {
    #[doc = "Transaction request"]
    #[inline(always)]
    pub fn trreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cdctl0::Trreq,
        cdctl0::Trreq,
        Cdctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cdctl0::Trreq,
            cdctl0::Trreq,
            Cdctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Periodic mode"]
    #[inline(always)]
    pub fn permd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cdctl0::Permd,
        cdctl0::Permd,
        Cdctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cdctl0::Permd,
            cdctl0::Permd,
            Cdctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Chip select"]
    #[inline(always)]
    pub fn cssel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cdctl0::Cssel,
        cdctl0::Cssel,
        Cdctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cdctl0::Cssel,
            cdctl0::Cssel,
            Cdctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transaction number"]
    #[inline(always)]
    pub fn trnum(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        cdctl0::Trnum,
        cdctl0::Trnum,
        Cdctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            cdctl0::Trnum,
            cdctl0::Trnum,
            Cdctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Periodic transaction interval"]
    #[inline(always)]
    pub fn peritv(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Cdctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Cdctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Periodic transaction repeat"]
    #[inline(always)]
    pub fn perrep(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cdctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cdctl0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdctl0 {
    #[inline(always)]
    fn default() -> Cdctl0 {
        <crate::RegValueT<Cdctl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cdctl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trreq_SPEC;
    pub type Trreq = crate::EnumBitfieldStruct<u8, Trreq_SPEC>;
    impl Trreq {
        #[doc = "No transaction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request transaction"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Permd_SPEC;
    pub type Permd = crate::EnumBitfieldStruct<u8, Permd_SPEC>;
    impl Permd {
        #[doc = "Direct manual-command mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Periodic manual-command mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cssel_SPEC;
    pub type Cssel = crate::EnumBitfieldStruct<u8, Cssel_SPEC>;
    impl Cssel {
        #[doc = "CS0"]
        pub const _0: Self = Self::new(0);

        #[doc = "CS1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trnum_SPEC;
    pub type Trnum = crate::EnumBitfieldStruct<u8, Trnum_SPEC>;
    impl Trnum {
        #[doc = "Issue 1 command (using command buffer 0)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Issue 2 commands (using command buffer 0-1)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Issue 3 commands (using command buffer 0-2)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Issue 4 commands (using command buffer 0-3)"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdctl1_SPEC;
impl crate::sealed::RegSpec for Cdctl1_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Manual Control Register 1"]
pub type Cdctl1 = crate::RegValueT<Cdctl1_SPEC>;

impl Cdctl1 {
    #[doc = "Periodic transaction expected value"]
    #[inline(always)]
    pub fn perexp(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdctl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdctl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdctl1 {
    #[inline(always)]
    fn default() -> Cdctl1 {
        <crate::RegValueT<Cdctl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdctl2_SPEC;
impl crate::sealed::RegSpec for Cdctl2_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Manual Control Register 2"]
pub type Cdctl2 = crate::RegValueT<Cdctl2_SPEC>;

impl Cdctl2 {
    #[doc = "Periodic transaction masked value"]
    #[inline(always)]
    pub fn permsk(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdctl2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdctl2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdctl2 {
    #[inline(always)]
    fn default() -> Cdctl2 {
        <crate::RegValueT<Cdctl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdtbuf_SPEC;
impl crate::sealed::RegSpec for Cdtbuf_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Manual Type Buf %s"]
pub type Cdtbuf = crate::RegValueT<Cdtbuf_SPEC>;

impl Cdtbuf {
    #[doc = "Command Size"]
    #[inline(always)]
    pub fn cmdsize(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cdtbuf::Cmdsize,
        cdtbuf::Cmdsize,
        Cdtbuf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cdtbuf::Cmdsize,
            cdtbuf::Cmdsize,
            Cdtbuf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address size"]
    #[inline(always)]
    pub fn addsize(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        cdtbuf::Addsize,
        cdtbuf::Addsize,
        Cdtbuf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            cdtbuf::Addsize,
            cdtbuf::Addsize,
            Cdtbuf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write/Read Data Size"]
    #[inline(always)]
    pub fn datasize(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, u8, Cdtbuf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xf,1,0,u8,u8,Cdtbuf_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Latency cycle"]
    #[inline(always)]
    pub fn late(
        self,
    ) -> crate::common::RegisterField<9, 0x1f, 1, 0, u8, u8, Cdtbuf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1f,1,0,u8,u8,Cdtbuf_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transaction Type"]
    #[inline(always)]
    pub fn trtype(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cdtbuf::Trtype,
        cdtbuf::Trtype,
        Cdtbuf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cdtbuf::Trtype,
            cdtbuf::Trtype,
            Cdtbuf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command (1-2 bytes)"]
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cdtbuf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cdtbuf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdtbuf {
    #[inline(always)]
    fn default() -> Cdtbuf {
        <crate::RegValueT<Cdtbuf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cdtbuf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdsize_SPEC;
    pub type Cmdsize = crate::EnumBitfieldStruct<u8, Cmdsize_SPEC>;
    impl Cmdsize {
        #[doc = "0 bytes (No command phase)"]
        pub const _00: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _01: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addsize_SPEC;
    pub type Addsize = crate::EnumBitfieldStruct<u8, Addsize_SPEC>;
    impl Addsize {
        #[doc = "0 bytes (No address phase)"]
        pub const _000: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _001: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _010: Self = Self::new(2);

        #[doc = "3 bytes"]
        pub const _011: Self = Self::new(3);

        #[doc = "4 bytes"]
        pub const _100: Self = Self::new(4);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trtype_SPEC;
    pub type Trtype = crate::EnumBitfieldStruct<u8, Trtype_SPEC>;
    impl Trtype {
        #[doc = "Read transaction (Readout data from slave device)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not read transaction"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdabuf_SPEC;
impl crate::sealed::RegSpec for Cdabuf_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Manual Address Buf %s"]
pub type Cdabuf = crate::RegValueT<Cdabuf_SPEC>;

impl Cdabuf {
    #[doc = "Address"]
    #[inline(always)]
    pub fn add(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdabuf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdabuf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdabuf {
    #[inline(always)]
    fn default() -> Cdabuf {
        <crate::RegValueT<Cdabuf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdd0Buf_SPEC;
impl crate::sealed::RegSpec for Cdd0Buf_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Manual Data 0 Buf %s"]
pub type Cdd0Buf = crate::RegValueT<Cdd0Buf_SPEC>;

impl Cdd0Buf {
    #[doc = "Write/Read Data"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdd0Buf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdd0Buf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdd0Buf {
    #[inline(always)]
    fn default() -> Cdd0Buf {
        <crate::RegValueT<Cdd0Buf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdd1Buf_SPEC;
impl crate::sealed::RegSpec for Cdd1Buf_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Manual Data 1 Buf %s"]
pub type Cdd1Buf = crate::RegValueT<Cdd1Buf_SPEC>;

impl Cdd1Buf {
    #[doc = "Write/Read Data"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdd1Buf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdd1Buf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdd1Buf {
    #[inline(always)]
    fn default() -> Cdd1Buf {
        <crate::RegValueT<Cdd1Buf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpctl0_SPEC;
impl crate::sealed::RegSpec for Lpctl0_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Link Pattern Control Register 0"]
pub type Lpctl0 = crate::RegValueT<Lpctl0_SPEC>;

impl Lpctl0 {
    #[doc = "Pattern request"]
    #[inline(always)]
    pub fn patreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lpctl0::Patreq,
        lpctl0::Patreq,
        Lpctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lpctl0::Patreq,
            lpctl0::Patreq,
            Lpctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Chip select"]
    #[inline(always)]
    pub fn cssel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        lpctl0::Cssel,
        lpctl0::Cssel,
        Lpctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            lpctl0::Cssel,
            lpctl0::Cssel,
            Lpctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "XiP Disable pattern pin"]
    #[inline(always)]
    pub fn xdpin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        lpctl0::Xdpin,
        lpctl0::Xdpin,
        Lpctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            lpctl0::Xdpin,
            lpctl0::Xdpin,
            Lpctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "XiP Disable pattern 1st phase length"]
    #[inline(always)]
    pub fn xd1len(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Lpctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Lpctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "XiP Disable pattern 1st phase value"]
    #[inline(always)]
    pub fn xd1val(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        lpctl0::Xd1Val,
        lpctl0::Xd1Val,
        Lpctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            lpctl0::Xd1Val,
            lpctl0::Xd1Val,
            Lpctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "XiP Disable pattern 2nd phase length"]
    #[inline(always)]
    pub fn xd2len(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Lpctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Lpctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "XiP Disable pattern 2nd phase value"]
    #[inline(always)]
    pub fn xd2val(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        lpctl0::Xd2Val,
        lpctl0::Xd2Val,
        Lpctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            lpctl0::Xd2Val,
            lpctl0::Xd2Val,
            Lpctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lpctl0 {
    #[inline(always)]
    fn default() -> Lpctl0 {
        <crate::RegValueT<Lpctl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lpctl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Patreq_SPEC;
    pub type Patreq = crate::EnumBitfieldStruct<u8, Patreq_SPEC>;
    impl Patreq {
        #[doc = "No request XiP Disable pattern"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request XiP Disable pattern"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cssel_SPEC;
    pub type Cssel = crate::EnumBitfieldStruct<u8, Cssel_SPEC>;
    impl Cssel {
        #[doc = "slave0 (CS0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "slave1 (CS1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Xdpin_SPEC;
    pub type Xdpin = crate::EnumBitfieldStruct<u8, Xdpin_SPEC>;
    impl Xdpin {
        #[doc = "1 pin"]
        pub const _00: Self = Self::new(0);

        #[doc = "2 pins"]
        pub const _01: Self = Self::new(1);

        #[doc = "4 pins"]
        pub const _10: Self = Self::new(2);

        #[doc = "8 pins"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Xd1Val_SPEC;
    pub type Xd1Val = crate::EnumBitfieldStruct<u8, Xd1Val_SPEC>;
    impl Xd1Val {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "High drive"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Xd2Val_SPEC;
    pub type Xd2Val = crate::EnumBitfieldStruct<u8, Xd2Val_SPEC>;
    impl Xd2Val {
        #[doc = "Low drive"]
        pub const _0: Self = Self::new(0);

        #[doc = "High drive"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpctl1_SPEC;
impl crate::sealed::RegSpec for Lpctl1_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Link Pattern Control Register 1"]
pub type Lpctl1 = crate::RegValueT<Lpctl1_SPEC>;

impl Lpctl1 {
    #[doc = "Pattern request"]
    #[inline(always)]
    pub fn patreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lpctl1::Patreq,
        lpctl1::Patreq,
        Lpctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lpctl1::Patreq,
            lpctl1::Patreq,
            Lpctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Chip select"]
    #[inline(always)]
    pub fn cssel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        lpctl1::Cssel,
        lpctl1::Cssel,
        Lpctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            lpctl1::Cssel,
            lpctl1::Cssel,
            Lpctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reset pattern repeat"]
    #[inline(always)]
    pub fn rstrep(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        lpctl1::Rstrep,
        lpctl1::Rstrep,
        Lpctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            lpctl1::Rstrep,
            lpctl1::Rstrep,
            Lpctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reset pattern width"]
    #[inline(always)]
    pub fn rstwid(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Lpctl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Lpctl1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reset pattern data output setup time"]
    #[inline(always)]
    pub fn rstsu(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Lpctl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Lpctl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lpctl1 {
    #[inline(always)]
    fn default() -> Lpctl1 {
        <crate::RegValueT<Lpctl1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lpctl1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Patreq_SPEC;
    pub type Patreq = crate::EnumBitfieldStruct<u8, Patreq_SPEC>;
    impl Patreq {
        #[doc = "No request"]
        pub const _00: Self = Self::new(0);

        #[doc = "Request Reset pattern"]
        pub const _01: Self = Self::new(1);

        #[doc = "Request CS only pattern"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cssel_SPEC;
    pub type Cssel = crate::EnumBitfieldStruct<u8, Cssel_SPEC>;
    impl Cssel {
        #[doc = "slave0 (CS0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "slave1 (CS1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstrep_SPEC;
    pub type Rstrep = crate::EnumBitfieldStruct<u8, Rstrep_SPEC>;
    impl Rstrep {
        #[doc = "4 times (Specified on Reset Signaling Protocol)"]
        pub const _00: Self = Self::new(0);

        #[doc = "5 times"]
        pub const _01: Self = Self::new(1);

        #[doc = "6 times"]
        pub const _10: Self = Self::new(2);

        #[doc = "7 times"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lioctl_SPEC;
impl crate::sealed::RegSpec for Lioctl_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Link I/O Control Register"]
pub type Lioctl = crate::RegValueT<Lioctl_SPEC>;

impl Lioctl {
    #[doc = "WP drive for slave1"]
    #[inline(always)]
    pub fn wpcs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lioctl::Wpcs1,
        lioctl::Wpcs1,
        Lioctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lioctl::Wpcs1,
            lioctl::Wpcs1,
            Lioctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reset drive"]
    #[inline(always)]
    pub fn rstcs0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        lioctl::Rstcs0,
        lioctl::Rstcs0,
        Lioctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            lioctl::Rstcs0,
            lioctl::Rstcs0,
            Lioctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lioctl {
    #[inline(always)]
    fn default() -> Lioctl {
        <crate::RegValueT<Lioctl_SPEC> as RegisterValue<_>>::new(196611)
    }
}
pub mod lioctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcs1_SPEC;
    pub type Wpcs1 = crate::EnumBitfieldStruct<u8, Wpcs1_SPEC>;
    impl Wpcs1 {
        #[doc = "Drive Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "Drive High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstcs0_SPEC;
    pub type Rstcs0 = crate::EnumBitfieldStruct<u8, Rstcs0_SPEC>;
    impl Rstcs0 {
        #[doc = "Drive Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "Drive High level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl0Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl0Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 0 CSn"]
pub type Ccctl0Cs = crate::RegValueT<Ccctl0Cs_SPEC>;

impl Ccctl0Cs {
    #[doc = "Automatic Calibration Enable"]
    #[inline(always)]
    pub fn caen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccctl0cs::Caen,
        ccctl0cs::Caen,
        Ccctl0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccctl0cs::Caen,
            ccctl0cs::Caen,
            Ccctl0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration no write mode"]
    #[inline(always)]
    pub fn canowr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccctl0cs::Canowr,
        ccctl0cs::Canowr,
        Ccctl0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccctl0cs::Canowr,
            ccctl0cs::Canowr,
            Ccctl0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration interval"]
    #[inline(always)]
    pub fn caitv(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Ccctl0Cs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Ccctl0Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Calibration OM_DQS shift start value"]
    #[inline(always)]
    pub fn casftsta(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ccctl0Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ccctl0Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Calibration OM_DQS shift end value"]
    #[inline(always)]
    pub fn casftend(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Ccctl0Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Ccctl0Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccctl0Cs {
    #[inline(always)]
    fn default() -> Ccctl0Cs {
        <crate::RegValueT<Ccctl0Cs_SPEC> as RegisterValue<_>>::new(520093696)
    }
}
pub mod ccctl0cs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Caen_SPEC;
    pub type Caen = crate::EnumBitfieldStruct<u8, Caen_SPEC>;
    impl Caen {
        #[doc = "Disable automatic calibration"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable automatic calibration"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Canowr_SPEC;
    pub type Canowr = crate::EnumBitfieldStruct<u8, Canowr_SPEC>;
    impl Canowr {
        #[doc = "Calibration sequence with write command"]
        pub const _0: Self = Self::new(0);

        #[doc = "Calibration sequence without write command"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl1Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl1Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 1 CSn"]
pub type Ccctl1Cs = crate::RegValueT<Ccctl1Cs_SPEC>;

impl Ccctl1Cs {
    #[doc = "Command Size"]
    #[inline(always)]
    pub fn cacmdsize(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ccctl1cs::Cacmdsize,
        ccctl1cs::Cacmdsize,
        Ccctl1Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ccctl1cs::Cacmdsize,
            ccctl1cs::Cacmdsize,
            Ccctl1Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address size"]
    #[inline(always)]
    pub fn caaddsize(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        ccctl1cs::Caaddsize,
        ccctl1cs::Caaddsize,
        Ccctl1Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            ccctl1cs::Caaddsize,
            ccctl1cs::Caaddsize,
            Ccctl1Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write/Read Data Size"]
    #[inline(always)]
    pub fn cadatasize(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, u8, Ccctl1Cs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xf,1,0,u8,u8,Ccctl1Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Latency cycle"]
    #[inline(always)]
    pub fn cawrlate(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ccctl1Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ccctl1Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Read Latency cycle"]
    #[inline(always)]
    pub fn cardlate(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Ccctl1Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Ccctl1Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccctl1Cs {
    #[inline(always)]
    fn default() -> Ccctl1Cs {
        <crate::RegValueT<Ccctl1Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccctl1cs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cacmdsize_SPEC;
    pub type Cacmdsize = crate::EnumBitfieldStruct<u8, Cacmdsize_SPEC>;
    impl Cacmdsize {
        #[doc = "0 bytes (No command phase)"]
        pub const _00: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _01: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Caaddsize_SPEC;
    pub type Caaddsize = crate::EnumBitfieldStruct<u8, Caaddsize_SPEC>;
    impl Caaddsize {
        #[doc = "0 bytes (No address phase)"]
        pub const _000: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _001: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _010: Self = Self::new(2);

        #[doc = "3 bytes"]
        pub const _011: Self = Self::new(3);

        #[doc = "4 bytes"]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl2Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl2Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 2 CSn"]
pub type Ccctl2Cs = crate::RegValueT<Ccctl2Cs_SPEC>;

impl Ccctl2Cs {
    #[doc = "Calibration pattern write command"]
    #[inline(always)]
    pub fn cawrcmd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ccctl2Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ccctl2Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Calibration pattern read command"]
    #[inline(always)]
    pub fn cardcmd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ccctl2Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ccctl2Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccctl2Cs {
    #[inline(always)]
    fn default() -> Ccctl2Cs {
        <crate::RegValueT<Ccctl2Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl3Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl3Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 3 CSn"]
pub type Ccctl3Cs = crate::RegValueT<Ccctl3Cs_SPEC>;

impl Ccctl3Cs {
    #[doc = "Calibration pattern address"]
    #[inline(always)]
    pub fn caadd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ccctl3Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ccctl3Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccctl3Cs {
    #[inline(always)]
    fn default() -> Ccctl3Cs {
        <crate::RegValueT<Ccctl3Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl4Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl4Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 4 CSn"]
pub type Ccctl4Cs = crate::RegValueT<Ccctl4Cs_SPEC>;

impl Ccctl4Cs {
    #[doc = "Calibration pattern data"]
    #[inline(always)]
    pub fn cadata(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ccctl4Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ccctl4Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccctl4Cs {
    #[inline(always)]
    fn default() -> Ccctl4Cs {
        <crate::RegValueT<Ccctl4Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl5Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl5Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 5 CSn"]
pub type Ccctl5Cs = crate::RegValueT<Ccctl5Cs_SPEC>;

impl Ccctl5Cs {
    #[doc = "Calibration pattern data"]
    #[inline(always)]
    pub fn cadata(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ccctl5Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ccctl5Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccctl5Cs {
    #[inline(always)]
    fn default() -> Ccctl5Cs {
        <crate::RegValueT<Ccctl5Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl6Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl6Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 6 CSn"]
pub type Ccctl6Cs = crate::RegValueT<Ccctl6Cs_SPEC>;

impl Ccctl6Cs {
    #[doc = "Calibration pattern data"]
    #[inline(always)]
    pub fn cadata(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ccctl6Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ccctl6Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccctl6Cs {
    #[inline(always)]
    fn default() -> Ccctl6Cs {
        <crate::RegValueT<Ccctl6Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl7Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl7Cs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Command Calibration Control Register 7 CSn"]
pub type Ccctl7Cs = crate::RegValueT<Ccctl7Cs_SPEC>;

impl Ccctl7Cs {
    #[doc = "Calibration pattern data"]
    #[inline(always)]
    pub fn cadata(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ccctl7Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ccctl7Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccctl7Cs {
    #[inline(always)]
    fn default() -> Ccctl7Cs {
        <crate::RegValueT<Ccctl7Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comstt_SPEC;
impl crate::sealed::RegSpec for Comstt_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Common Status Register"]
pub type Comstt = crate::RegValueT<Comstt_SPEC>;

impl Comstt {
    #[doc = "Memory access ongoing from ch0"]
    #[inline(always)]
    pub fn memaccch0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        comstt::Memaccch0,
        comstt::Memaccch0,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            comstt::Memaccch0,
            comstt::Memaccch0,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Memory access ongoing from ch1"]
    #[inline(always)]
    pub fn memaccch1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Comstt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Comstt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Prefetch Buffer Not Empty for ch0"]
    #[inline(always)]
    pub fn pbufnech0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        comstt::Pbufnech0,
        comstt::Pbufnech0,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            comstt::Pbufnech0,
            comstt::Pbufnech0,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Prefetch Buffer Not Empty for ch1"]
    #[inline(always)]
    pub fn pbufnech1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Comstt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Comstt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Write Buffer Not Empty for ch0"]
    #[inline(always)]
    pub fn wrbufnech0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        comstt::Wrbufnech0,
        comstt::Wrbufnech0,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            comstt::Wrbufnech0,
            comstt::Wrbufnech0,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Write Buffer Not Empty for ch1"]
    #[inline(always)]
    pub fn wrbufnech1(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Comstt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Comstt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ECS monitor for slave1"]
    #[inline(always)]
    pub fn ecscs1(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        comstt::Ecscs1,
        comstt::Ecscs1,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            comstt::Ecscs1,
            comstt::Ecscs1,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "INT monitor for slave1"]
    #[inline(always)]
    pub fn intcs1(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        comstt::Intcs1,
        comstt::Intcs1,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            comstt::Intcs1,
            comstt::Intcs1,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RSTO monitor for slave1"]
    #[inline(always)]
    pub fn rstocs1(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        comstt::Rstocs1,
        comstt::Rstocs1,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            comstt::Rstocs1,
            comstt::Rstocs1,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Comstt {
    #[inline(always)]
    fn default() -> Comstt {
        <crate::RegValueT<Comstt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod comstt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Memaccch0_SPEC;
    pub type Memaccch0 = crate::EnumBitfieldStruct<u8, Memaccch0_SPEC>;
    impl Memaccch0 {
        #[doc = "System bus bridge ch0 is not accessing to memory."]
        pub const _0: Self = Self::new(0);

        #[doc = "System bus bridge ch0 is accessing to memory."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pbufnech0_SPEC;
    pub type Pbufnech0 = crate::EnumBitfieldStruct<u8, Pbufnech0_SPEC>;
    impl Pbufnech0 {
        #[doc = "Empty"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrbufnech0_SPEC;
    pub type Wrbufnech0 = crate::EnumBitfieldStruct<u8, Wrbufnech0_SPEC>;
    impl Wrbufnech0 {
        #[doc = "Empty"]
        pub const _0: Self = Self::new(0);

        #[doc = "Not empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecscs1_SPEC;
    pub type Ecscs1 = crate::EnumBitfieldStruct<u8, Ecscs1_SPEC>;
    impl Ecscs1 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intcs1_SPEC;
    pub type Intcs1 = crate::EnumBitfieldStruct<u8, Intcs1_SPEC>;
    impl Intcs1 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstocs1_SPEC;
    pub type Rstocs1 = crate::EnumBitfieldStruct<u8, Rstocs1_SPEC>;
    impl Rstocs1 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Casttcs_SPEC;
impl crate::sealed::RegSpec for Casttcs_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Calibration Status Register CSn"]
pub type Casttcs = crate::RegValueT<Casttcs_SPEC>;

impl Casttcs {
    #[doc = "Calibration Success"]
    #[inline(always)]
    pub fn casuc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Casttcs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Casttcs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Casttcs {
    #[inline(always)]
    fn default() -> Casttcs {
        <crate::RegValueT<Casttcs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints_SPEC;
impl crate::sealed::RegSpec for Ints_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Interrupt Status Register"]
pub type Ints = crate::RegValueT<Ints_SPEC>;

impl Ints {
    #[doc = "Command Completed"]
    #[inline(always)]
    pub fn cmdcmp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ints::Cmdcmp,
        ints::Cmdcmp,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ints::Cmdcmp,
            ints::Cmdcmp,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Pattern Completed"]
    #[inline(always)]
    pub fn patcmp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ints::Patcmp,
        ints::Patcmp,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ints::Patcmp,
            ints::Patcmp,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Periodic transaction timeout"]
    #[inline(always)]
    pub fn perto(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ints::Perto,
        ints::Perto,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ints::Perto,
            ints::Perto,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "OM_DQS timeout for slave0"]
    #[inline(always)]
    pub fn dstocs0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ints::Dstocs0,
        ints::Dstocs0,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ints::Dstocs0,
            ints::Dstocs0,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "OM_DQS timeout for slave1"]
    #[inline(always)]
    pub fn dstocs1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ints::Dstocs1,
        ints::Dstocs1,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ints::Dstocs1,
            ints::Dstocs1,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECC error detection for slave1"]
    #[inline(always)]
    pub fn ecscs1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ints::Ecscs1,
        ints::Ecscs1,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ints::Ecscs1,
            ints::Ecscs1,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt detection for slave1"]
    #[inline(always)]
    pub fn intcs1(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ints::Intcs1,
        ints::Intcs1,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ints::Intcs1,
            ints::Intcs1,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "System bus error for ch0"]
    #[inline(always)]
    pub fn buserrch0(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "System bus error for ch1"]
    #[inline(always)]
    pub fn buserrch1(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Calibration failed for slave0"]
    #[inline(always)]
    pub fn cafailcs0(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ints::Cafailcs0,
        ints::Cafailcs0,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ints::Cafailcs0,
            ints::Cafailcs0,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Calibration failed for slave1"]
    #[inline(always)]
    pub fn cafailcs1(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ints::Cafailcs1,
        ints::Cafailcs1,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ints::Cafailcs1,
            ints::Cafailcs1,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Calibration success for slave0"]
    #[inline(always)]
    pub fn casuccs0(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ints::Casuccs0,
        ints::Casuccs0,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ints::Casuccs0,
            ints::Casuccs0,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Calibration success for slave1"]
    #[inline(always)]
    pub fn casuccs1(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ints::Casuccs1,
        ints::Casuccs1,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ints::Casuccs1,
            ints::Casuccs1,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ints {
    #[inline(always)]
    fn default() -> Ints {
        <crate::RegValueT<Ints_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ints {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdcmp_SPEC;
    pub type Cmdcmp = crate::EnumBitfieldStruct<u8, Cmdcmp_SPEC>;
    impl Cmdcmp {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Patcmp_SPEC;
    pub type Patcmp = crate::EnumBitfieldStruct<u8, Patcmp_SPEC>;
    impl Patcmp {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perto_SPEC;
    pub type Perto = crate::EnumBitfieldStruct<u8, Perto_SPEC>;
    impl Perto {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dstocs0_SPEC;
    pub type Dstocs0 = crate::EnumBitfieldStruct<u8, Dstocs0_SPEC>;
    impl Dstocs0 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dstocs1_SPEC;
    pub type Dstocs1 = crate::EnumBitfieldStruct<u8, Dstocs1_SPEC>;
    impl Dstocs1 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecscs1_SPEC;
    pub type Ecscs1 = crate::EnumBitfieldStruct<u8, Ecscs1_SPEC>;
    impl Ecscs1 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intcs1_SPEC;
    pub type Intcs1 = crate::EnumBitfieldStruct<u8, Intcs1_SPEC>;
    impl Intcs1 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cafailcs0_SPEC;
    pub type Cafailcs0 = crate::EnumBitfieldStruct<u8, Cafailcs0_SPEC>;
    impl Cafailcs0 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cafailcs1_SPEC;
    pub type Cafailcs1 = crate::EnumBitfieldStruct<u8, Cafailcs1_SPEC>;
    impl Cafailcs1 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Casuccs0_SPEC;
    pub type Casuccs0 = crate::EnumBitfieldStruct<u8, Casuccs0_SPEC>;
    impl Casuccs0 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Casuccs1_SPEC;
    pub type Casuccs1 = crate::EnumBitfieldStruct<u8, Casuccs1_SPEC>;
    impl Casuccs1 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intc_SPEC;
impl crate::sealed::RegSpec for Intc_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Interrupt Clear Register"]
pub type Intc = crate::RegValueT<Intc_SPEC>;

impl Intc {
    #[doc = "Command Completed interrupt clear"]
    #[inline(always)]
    pub fn cmdcmpc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intc::Cmdcmpc,
        intc::Cmdcmpc,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intc::Cmdcmpc,
            intc::Cmdcmpc,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Pattern Completed interrupt clear"]
    #[inline(always)]
    pub fn patcmpc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        intc::Patcmpc,
        intc::Patcmpc,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            intc::Patcmpc,
            intc::Patcmpc,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Periodic transaction timeout interrupt clear"]
    #[inline(always)]
    pub fn pertoc(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        intc::Pertoc,
        intc::Pertoc,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            intc::Pertoc,
            intc::Pertoc,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "OM_DQS timeout for slave0 interrupt clear"]
    #[inline(always)]
    pub fn dstocs0c(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        intc::Dstocs0C,
        intc::Dstocs0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            intc::Dstocs0C,
            intc::Dstocs0C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "OM_DQS timeout for slave1 interrupt clear"]
    #[inline(always)]
    pub fn dstocs1c(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        intc::Dstocs1C,
        intc::Dstocs1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            intc::Dstocs1C,
            intc::Dstocs1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "ECC error detection for slave1 interrupt clear"]
    #[inline(always)]
    pub fn ecscs1c(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        intc::Ecscs1C,
        intc::Ecscs1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            intc::Ecscs1C,
            intc::Ecscs1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt detection for slave1 interrupt clear"]
    #[inline(always)]
    pub fn intcs1c(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        intc::Intcs1C,
        intc::Intcs1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            intc::Intcs1C,
            intc::Intcs1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "System bus error for ch0 interrupt clear"]
    #[inline(always)]
    pub fn buserrch0c(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        intc::Buserrch0C,
        intc::Buserrch0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            intc::Buserrch0C,
            intc::Buserrch0C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "System bus error for ch1 interrupt clear"]
    #[inline(always)]
    pub fn buserrch1c(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        intc::Buserrch1C,
        intc::Buserrch1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            intc::Buserrch1C,
            intc::Buserrch1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Calibration failed for slave0 interrupt clear"]
    #[inline(always)]
    pub fn cafailcs0c(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        intc::Cafailcs0C,
        intc::Cafailcs0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            intc::Cafailcs0C,
            intc::Cafailcs0C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Calibration failed for slave1 interrupt clear"]
    #[inline(always)]
    pub fn cafailcs1c(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        intc::Cafailcs1C,
        intc::Cafailcs1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            intc::Cafailcs1C,
            intc::Cafailcs1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Calibration success for slave0 interrupt clear"]
    #[inline(always)]
    pub fn casuccs0c(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        intc::Casuccs0C,
        intc::Casuccs0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            intc::Casuccs0C,
            intc::Casuccs0C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Calibration success for slave1 interrupt clear"]
    #[inline(always)]
    pub fn casuccs1c(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        intc::Casuccs1C,
        intc::Casuccs1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            intc::Casuccs1C,
            intc::Casuccs1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intc {
    #[inline(always)]
    fn default() -> Intc {
        <crate::RegValueT<Intc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdcmpc_SPEC;
    pub type Cmdcmpc = crate::EnumBitfieldStruct<u8, Cmdcmpc_SPEC>;
    impl Cmdcmpc {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Patcmpc_SPEC;
    pub type Patcmpc = crate::EnumBitfieldStruct<u8, Patcmpc_SPEC>;
    impl Patcmpc {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pertoc_SPEC;
    pub type Pertoc = crate::EnumBitfieldStruct<u8, Pertoc_SPEC>;
    impl Pertoc {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dstocs0C_SPEC;
    pub type Dstocs0C = crate::EnumBitfieldStruct<u8, Dstocs0C_SPEC>;
    impl Dstocs0C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dstocs1C_SPEC;
    pub type Dstocs1C = crate::EnumBitfieldStruct<u8, Dstocs1C_SPEC>;
    impl Dstocs1C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecscs1C_SPEC;
    pub type Ecscs1C = crate::EnumBitfieldStruct<u8, Ecscs1C_SPEC>;
    impl Ecscs1C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intcs1C_SPEC;
    pub type Intcs1C = crate::EnumBitfieldStruct<u8, Intcs1C_SPEC>;
    impl Intcs1C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buserrch0C_SPEC;
    pub type Buserrch0C = crate::EnumBitfieldStruct<u8, Buserrch0C_SPEC>;
    impl Buserrch0C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buserrch1C_SPEC;
    pub type Buserrch1C = crate::EnumBitfieldStruct<u8, Buserrch1C_SPEC>;
    impl Buserrch1C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cafailcs0C_SPEC;
    pub type Cafailcs0C = crate::EnumBitfieldStruct<u8, Cafailcs0C_SPEC>;
    impl Cafailcs0C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cafailcs1C_SPEC;
    pub type Cafailcs1C = crate::EnumBitfieldStruct<u8, Cafailcs1C_SPEC>;
    impl Cafailcs1C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Casuccs0C_SPEC;
    pub type Casuccs0C = crate::EnumBitfieldStruct<u8, Casuccs0C_SPEC>;
    impl Casuccs0C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Casuccs1C_SPEC;
    pub type Casuccs1C = crate::EnumBitfieldStruct<u8, Casuccs1C_SPEC>;
    impl Casuccs1C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte_SPEC;
impl crate::sealed::RegSpec for Inte_SPEC {
    type DataType = u32;
}

#[doc = "OSPI Interrupt Enable Register"]
pub type Inte = crate::RegValueT<Inte_SPEC>;

impl Inte {
    #[doc = "Command Completed interrupt enable"]
    #[inline(always)]
    pub fn cmdcmpe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        inte::Cmdcmpe,
        inte::Cmdcmpe,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            inte::Cmdcmpe,
            inte::Cmdcmpe,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pattern Completed interrupt enable"]
    #[inline(always)]
    pub fn patcmpe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        inte::Patcmpe,
        inte::Patcmpe,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            inte::Patcmpe,
            inte::Patcmpe,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Periodic transaction timeout interrupt enable"]
    #[inline(always)]
    pub fn pertoe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        inte::Pertoe,
        inte::Pertoe,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            inte::Pertoe,
            inte::Pertoe,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "OM_DQS timeout for slave0 interrupt enable"]
    #[inline(always)]
    pub fn dstocs0e(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        inte::Dstocs0E,
        inte::Dstocs0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            inte::Dstocs0E,
            inte::Dstocs0E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "OM_DQS timeout for slave1 interrupt enable"]
    #[inline(always)]
    pub fn dstocs1e(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        inte::Dstocs1E,
        inte::Dstocs1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            inte::Dstocs1E,
            inte::Dstocs1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC error detection for slave1 interrupt enable"]
    #[inline(always)]
    pub fn ecscs1e(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        inte::Ecscs1E,
        inte::Ecscs1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            inte::Ecscs1E,
            inte::Ecscs1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt detection for slave1 interrupt enable"]
    #[inline(always)]
    pub fn intcs1e(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        inte::Intcs1E,
        inte::Intcs1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            inte::Intcs1E,
            inte::Intcs1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System bus error for ch0 interrupt enable"]
    #[inline(always)]
    pub fn buserrch0e(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        inte::Buserrch0E,
        inte::Buserrch0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            inte::Buserrch0E,
            inte::Buserrch0E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System bus error for ch1 interrupt enable"]
    #[inline(always)]
    pub fn buserrch1e(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        inte::Buserrch1E,
        inte::Buserrch1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            inte::Buserrch1E,
            inte::Buserrch1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration failed for slave0 interrupt enable"]
    #[inline(always)]
    pub fn cafailcs0e(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        inte::Cafailcs0E,
        inte::Cafailcs0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            inte::Cafailcs0E,
            inte::Cafailcs0E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration failed for slave1 interrupt enable"]
    #[inline(always)]
    pub fn cafailcs1e(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        inte::Cafailcs1E,
        inte::Cafailcs1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            inte::Cafailcs1E,
            inte::Cafailcs1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration success for slave0 interrupt enable"]
    #[inline(always)]
    pub fn casuccs0e(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        inte::Casuccs0E,
        inte::Casuccs0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            inte::Casuccs0E,
            inte::Casuccs0E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration success for slave1 interrupt enable"]
    #[inline(always)]
    pub fn casuccs1e(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        inte::Casuccs1E,
        inte::Casuccs1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            inte::Casuccs1E,
            inte::Casuccs1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Inte {
    #[inline(always)]
    fn default() -> Inte {
        <crate::RegValueT<Inte_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod inte {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdcmpe_SPEC;
    pub type Cmdcmpe = crate::EnumBitfieldStruct<u8, Cmdcmpe_SPEC>;
    impl Cmdcmpe {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Patcmpe_SPEC;
    pub type Patcmpe = crate::EnumBitfieldStruct<u8, Patcmpe_SPEC>;
    impl Patcmpe {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pertoe_SPEC;
    pub type Pertoe = crate::EnumBitfieldStruct<u8, Pertoe_SPEC>;
    impl Pertoe {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dstocs0E_SPEC;
    pub type Dstocs0E = crate::EnumBitfieldStruct<u8, Dstocs0E_SPEC>;
    impl Dstocs0E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dstocs1E_SPEC;
    pub type Dstocs1E = crate::EnumBitfieldStruct<u8, Dstocs1E_SPEC>;
    impl Dstocs1E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecscs1E_SPEC;
    pub type Ecscs1E = crate::EnumBitfieldStruct<u8, Ecscs1E_SPEC>;
    impl Ecscs1E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intcs1E_SPEC;
    pub type Intcs1E = crate::EnumBitfieldStruct<u8, Intcs1E_SPEC>;
    impl Intcs1E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buserrch0E_SPEC;
    pub type Buserrch0E = crate::EnumBitfieldStruct<u8, Buserrch0E_SPEC>;
    impl Buserrch0E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buserrch1E_SPEC;
    pub type Buserrch1E = crate::EnumBitfieldStruct<u8, Buserrch1E_SPEC>;
    impl Buserrch1E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cafailcs0E_SPEC;
    pub type Cafailcs0E = crate::EnumBitfieldStruct<u8, Cafailcs0E_SPEC>;
    impl Cafailcs0E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cafailcs1E_SPEC;
    pub type Cafailcs1E = crate::EnumBitfieldStruct<u8, Cafailcs1E_SPEC>;
    impl Cafailcs1E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Casuccs0E_SPEC;
    pub type Casuccs0E = crate::EnumBitfieldStruct<u8, Casuccs0E_SPEC>;
    impl Casuccs0E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Casuccs1E_SPEC;
    pub type Casuccs1E = crate::EnumBitfieldStruct<u8, Casuccs1E_SPEC>;
    impl Casuccs1E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
