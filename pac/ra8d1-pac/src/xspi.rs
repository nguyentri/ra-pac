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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:38:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"eXpanded SPI"]
unsafe impl ::core::marker::Send for super::XSpi {}
unsafe impl ::core::marker::Sync for super::XSpi {}
impl super::XSpi {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "xSPI Wrapper Configuration register"]
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

    #[doc = "xSPI Common Configuration register"]
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

    #[doc = "xSPI Bridge Map Configuration register ch%s"]
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

    #[doc = "xSPI Command Map Configuration register 0 cs%s"]
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

    #[doc = "xSPI Command Map Configuration register 1 cs%s"]
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

    #[doc = "xSPI Command Map Configuration register 2 cs%s"]
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

    #[doc = "xSPI AXI ID Bridge Map Config %s ch0"]
    #[inline(always)]
    pub const fn aibmcfgch0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Aibmcfgch0_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30usize))
        }
    }
    #[inline(always)]
    pub const fn aibmcfg0ch0(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aibmcfg1ch0(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aibmcfg2ch0(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aibmcfg3ch0(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x3cusize),
            )
        }
    }

    #[doc = "xSPI AXI ID Bridge Map Config %s ch1"]
    #[inline(always)]
    pub const fn aibmcfgch1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Aibmcfgch1_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }
    #[inline(always)]
    pub const fn aibmcfg0ch1(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aibmcfg1ch1(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aibmcfg2ch1(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aibmcfg3ch1(
        &self,
    ) -> &'static crate::common::Reg<self::Aibmcfgch1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aibmcfgch1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }

    #[doc = "xSPI Link I/O Configuration register cs%s"]
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

    #[doc = "xSPI AXI Bridge Map Config"]
    #[inline(always)]
    pub const fn abmcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Abmcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Abmcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "xSPI Bridge Map Control register 0"]
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

    #[doc = "xSPI Bridge Map Control register 1"]
    #[inline(always)]
    pub const fn bmctl1(&self) -> &'static crate::common::Reg<self::Bmctl1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Bmctl1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "xSPI Command Map Control register ch%s"]
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

    #[doc = "xSPI Command Manual Control register 0"]
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

    #[doc = "xSPI Command Manual Control register 1"]
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

    #[doc = "xSPI Command Manual Control register 2"]
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

    #[doc = "xSPI Command Manual Type buf%s"]
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

    #[doc = "xSPI Command Manual Address buf%s"]
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

    #[doc = "xSPI Command Manual Data 0 buf%s"]
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

    #[doc = "xSPI Command Manual Data 1 buf%s"]
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

    #[doc = "xSPI Link Pattern Control register 0"]
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

    #[doc = "xSPI Link Pattern Control register 1"]
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

    #[doc = "xSPI Link I/O Control register"]
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

    #[doc = "xSPI Command Calibration Control register 0 cs%s"]
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

    #[doc = "xSPI Command Calibration Control register 1 cs%s"]
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

    #[doc = "xSPI Command Calibration Control register 2 cs%s"]
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

    #[doc = "xSPI Command Calibration Control register 3 cs%s"]
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

    #[doc = "xSPI Command Calibration Control register 4 cs%s"]
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

    #[doc = "xSPI Command Calibration Control register 5 cs%s"]
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

    #[doc = "xSPI Command Calibration Control register 6 cs%s"]
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

    #[doc = "xSPI Command Calibration Control register 7 cs%s"]
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

    #[doc = "xSPI Version register"]
    #[inline(always)]
    pub const fn verstt(&self) -> &'static crate::common::Reg<self::Verstt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Verstt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "xSPI Common Status register"]
    #[inline(always)]
    pub const fn comstt(&self) -> &'static crate::common::Reg<self::Comstt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Comstt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[doc = "xSPI Calibration Status register cs%s"]
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

    #[doc = "xSPI Interrupt Status register"]
    #[inline(always)]
    pub const fn ints(&self) -> &'static crate::common::Reg<self::Ints_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ints_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[doc = "xSPI Interrupt Clear register"]
    #[inline(always)]
    pub const fn intc(&self) -> &'static crate::common::Reg<self::Intc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Intc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(404usize),
            )
        }
    }

    #[doc = "xSPI Interrupt Enable register"]
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

#[doc = "xSPI Wrapper Configuration register"]
pub type Wrapcfg = crate::RegValueT<Wrapcfg_SPEC>;

impl Wrapcfg {
    #[doc = "CK shift for slave0"]
    #[inline(always)]
    pub fn cksftcs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        wrapcfg::Cksftcs0,
        wrapcfg::Cksftcs0,
        Wrapcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            wrapcfg::Cksftcs0,
            wrapcfg::Cksftcs0,
            Wrapcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DS shift for slave0"]
    #[inline(always)]
    pub fn dssftcs0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1f,
        1,
        0,
        wrapcfg::Dssftcs0,
        wrapcfg::Dssftcs0,
        Wrapcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1f,
            1,
            0,
            wrapcfg::Dssftcs0,
            wrapcfg::Dssftcs0,
            Wrapcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CK shift for slave1"]
    #[inline(always)]
    pub fn cksftcs1(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Wrapcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Wrapcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DS shift for slave1"]
    #[inline(always)]
    pub fn dssftcs1(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Wrapcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Wrapcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Wrapcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Wrapcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Wrapcfg {
    #[inline(always)]
    fn default() -> Wrapcfg {
        <crate::RegValueT<Wrapcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wrapcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksftcs0_SPEC;
    pub type Cksftcs0 = crate::EnumBitfieldStruct<u8, Cksftcs0_SPEC>;
    impl Cksftcs0 {
        #[doc = "No shift"]
        pub const _0: Self = Self::new(0);

        #[doc = "Add a delay of 1 cell"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dssftcs0_SPEC;
    pub type Dssftcs0 = crate::EnumBitfieldStruct<u8, Dssftcs0_SPEC>;
    impl Dssftcs0 {
        #[doc = "No shift"]
        pub const _0: Self = Self::new(0);

        #[doc = "Add a delay of 1 cell"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comcfg_SPEC;
impl crate::sealed::RegSpec for Comcfg_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Common Configuration register"]
pub type Comcfg = crate::RegValueT<Comcfg_SPEC>;

impl Comcfg {
    #[doc = "Channel arbitration mode"]
    #[inline(always)]
    pub fn arbmd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Comcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Comcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ECS/INT Output Enable"]
    #[inline(always)]
    pub fn ecsintouten(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Comcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Comcfg_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3fff, 1, 0, u16, u16, Comcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3fff,1,0,u16,u16,Comcfg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Oeastex_SPEC;
    pub type Oeastex = crate::EnumBitfieldStruct<u8, Oeastex_SPEC>;
    impl Oeastex {
        #[doc = "No extend 1cycle Output enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend 1cycle Output enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oenegex_SPEC;
    pub type Oenegex = crate::EnumBitfieldStruct<u8, Oenegex_SPEC>;
    impl Oenegex {
        #[doc = "No extend 1cycle Output enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extend 1cycle Output enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmcfgch_SPEC;
impl crate::sealed::RegSpec for Bmcfgch_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Bridge Map Configuration register ch%s"]
pub type Bmcfgch = crate::RegValueT<Bmcfgch_SPEC>;

impl Bmcfgch {
    #[doc = "AHB Write Response mode"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7f, 1, 0, u8, u8, Bmcfgch_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x7f,1,0,u8,u8,Bmcfgch_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Combination timer"]
    #[inline(always)]
    pub fn cmbtim(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        bmcfgch::Cmbtim,
        bmcfgch::Cmbtim,
        Bmcfgch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            bmcfgch::Cmbtim,
            bmcfgch::Cmbtim,
            Bmcfgch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmbtim_SPEC;
    pub type Cmbtim = crate::EnumBitfieldStruct<u8, Cmbtim_SPEC>;
    impl Cmbtim {
        #[doc = "means disabling the combination timer. When the timer is"]
        pub const _00_H: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcfg0Cs_SPEC;
impl crate::sealed::RegSpec for Cmcfg0Cs_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Command Map Configuration register 0 cs%s"]
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

    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3ff, 1, 0, u16, u16, Cmcfg0Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3ff,1,0,u16,u16,Cmcfg0Cs_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Normal format: Command 1byte, Address ADDSIZE, Data up to AHB transaction."]
        pub const _0: Self = Self::new(0);

        #[doc = "8D-8D-8D profile 1.0 format: Command 2byte, Address ADDSIZE, Data up to AHB transaction"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addsize_SPEC;
    pub type Addsize = crate::EnumBitfieldStruct<u8, Addsize_SPEC>;
    impl Addsize {
        #[doc = "1byte (256Byte address space)"]
        pub const _0: Self = Self::new(0);

        #[doc = "2byte (64KByte address space)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpbstmd_SPEC;
    pub type Wpbstmd = crate::EnumBitfieldStruct<u8, Wpbstmd_SPEC>;
    impl Wpbstmd {
        #[doc = "separate xSPI transfer at the wrapping address boundary"]
        pub const _0: Self = Self::new(0);

        #[doc = "not separate xSPI transfer at the wrapping address boundary"]
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
        #[doc = "No replacement (xSPI frame address field is same as AHB address)"]
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

#[doc = "xSPI Command Map Configuration register 1 cs%s"]
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
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        cmcfg1cs::Rdlate,
        cmcfg1cs::Rdlate,
        Cmcfg1Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            cmcfg1cs::Rdlate,
            cmcfg1cs::Rdlate,
            Cmcfg1Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<21, 0x7ff, 1, 0, u16, u16, Cmcfg1Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x7ff,1,0,u16,u16,Cmcfg1Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmcfg1Cs {
    #[inline(always)]
    fn default() -> Cmcfg1Cs {
        <crate::RegValueT<Cmcfg1Cs_SPEC> as RegisterValue<_>>::new(524288)
    }
}
pub mod cmcfg1cs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdlate_SPEC;
    pub type Rdlate = crate::EnumBitfieldStruct<u8, Rdlate_SPEC>;
    impl Rdlate {
        #[doc = "No latency"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcfg2Cs_SPEC;
impl crate::sealed::RegSpec for Cmcfg2Cs_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Command Map Configuration register 2 cs%s"]
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
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        cmcfg2cs::Wrlate,
        cmcfg2cs::Wrlate,
        Cmcfg2Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            cmcfg2cs::Wrlate,
            cmcfg2cs::Wrlate,
            Cmcfg2Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000000. The write value should be 00000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<21, 0x7ff, 1, 0, u16, u16, Cmcfg2Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x7ff,1,0,u16,u16,Cmcfg2Cs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmcfg2Cs {
    #[inline(always)]
    fn default() -> Cmcfg2Cs {
        <crate::RegValueT<Cmcfg2Cs_SPEC> as RegisterValue<_>>::new(524288)
    }
}
pub mod cmcfg2cs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrlate_SPEC;
    pub type Wrlate = crate::EnumBitfieldStruct<u8, Wrlate_SPEC>;
    impl Wrlate {
        #[doc = "No latency"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aibmcfgch0_SPEC;
impl crate::sealed::RegSpec for Aibmcfgch0_SPEC {
    type DataType = u32;
}

#[doc = "xSPI AXI ID Bridge Map Config %s ch0"]
pub type Aibmcfgch0 = crate::RegValueT<Aibmcfgch0_SPEC>;

impl Aibmcfgch0 {
    #[doc = "AXI ID to Chx Map Value"]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Aibmcfgch0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Aibmcfgch0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI ID to Chx Map Mask"]
    #[inline(always)]
    pub fn mask(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Aibmcfgch0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Aibmcfgch0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Aibmcfgch0 {
    #[inline(always)]
    fn default() -> Aibmcfgch0 {
        <crate::RegValueT<Aibmcfgch0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aibmcfgch1_SPEC;
impl crate::sealed::RegSpec for Aibmcfgch1_SPEC {
    type DataType = u32;
}

#[doc = "xSPI AXI ID Bridge Map Config %s ch1"]
pub type Aibmcfgch1 = crate::RegValueT<Aibmcfgch1_SPEC>;

impl Aibmcfgch1 {
    #[doc = "AXI ID to Chx Map Value"]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Aibmcfgch1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Aibmcfgch1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI ID to Chx Map Mask"]
    #[inline(always)]
    pub fn mask(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Aibmcfgch1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Aibmcfgch1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Aibmcfgch1 {
    #[inline(always)]
    fn default() -> Aibmcfgch1 {
        <crate::RegValueT<Aibmcfgch1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Liocfgcs_SPEC;
impl crate::sealed::RegSpec for Liocfgcs_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Link I/O Configuration register cs%s"]
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
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        liocfgcs::Latemd,
        liocfgcs::Latemd,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            liocfgcs::Latemd,
            liocfgcs::Latemd,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Liocfgcs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Liocfgcs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CS minimum idle term"]
    #[inline(always)]
    pub fn csmin(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        liocfgcs::Csmin,
        liocfgcs::Csmin,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            liocfgcs::Csmin,
            liocfgcs::Csmin,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        liocfgcs::Sdrsmpsft,
        liocfgcs::Sdrsmpsft,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            liocfgcs::Sdrsmpsft,
            liocfgcs::Sdrsmpsft,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DDR sampling window extend"]
    #[inline(always)]
    pub fn ddrsmpex(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        liocfgcs::Ddrsmpex,
        liocfgcs::Ddrsmpex,
        Liocfgcs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            liocfgcs::Ddrsmpex,
            liocfgcs::Ddrsmpex,
            Liocfgcs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _0_000_000_000_B: Self = Self::new(0);

        #[doc = "4S-4D-4D"]
        pub const _1_110_110_010_B: Self = Self::new(946);

        #[doc = "8D-8D-8D"]
        pub const _1_111_111_111_B: Self = Self::new(1023);

        #[doc = "8D-8D-8S"]
        pub const _1_011_111_111_B: Self = Self::new(767);

        #[doc = "1S-2S-2S"]
        pub const _0_001_001_000_B: Self = Self::new(72);

        #[doc = "2S-2S-2S"]
        pub const _0_001_001_001_B: Self = Self::new(73);

        #[doc = "1S-4S-4S"]
        pub const _0_010_010_000_B: Self = Self::new(144);

        #[doc = "4S-4S-4S"]
        pub const _0_010_010_010_B: Self = Self::new(146);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Latemd_SPEC;
    pub type Latemd = crate::EnumBitfieldStruct<u8, Latemd_SPEC>;
    impl Latemd {
        #[doc = "Configurable latency"]
        pub const _0: Self = Self::new(0);

        #[doc = "Variable latency"]
        pub const _1: Self = Self::new(1);
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
    pub struct Csmin_SPEC;
    pub type Csmin = crate::EnumBitfieldStruct<u8, Csmin_SPEC>;
    impl Csmin {
        #[doc = "1 cycle"]
        pub const _0: Self = Self::new(0);
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdrsmpsft_SPEC;
    pub type Sdrsmpsft = crate::EnumBitfieldStruct<u8, Sdrsmpsft_SPEC>;
    impl Sdrsmpsft {
        #[doc = "Sample w/o delay"]
        pub const _0: Self = Self::new(0);

        #[doc = "Sample at 1 cycle delay"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddrsmpex_SPEC;
    pub type Ddrsmpex = crate::EnumBitfieldStruct<u8, Ddrsmpex_SPEC>;
    impl Ddrsmpex {
        #[doc = "Expand no cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "Expand 1 cycle"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abmcfg_SPEC;
impl crate::sealed::RegSpec for Abmcfg_SPEC {
    type DataType = u32;
}

#[doc = "xSPI AXI Bridge Map Config"]
pub type Abmcfg = crate::RegValueT<Abmcfg_SPEC>;

impl Abmcfg {
    #[doc = "AXI Transfer Ordering Mode"]
    #[inline(always)]
    pub fn odrmd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Abmcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Abmcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, u16, Abmcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16,u16,Abmcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI ID to Bridge Channel Select"]
    #[inline(always)]
    pub fn chsel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Abmcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Abmcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Abmcfg {
    #[inline(always)]
    fn default() -> Abmcfg {
        <crate::RegValueT<Abmcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmctl0_SPEC;
impl crate::sealed::RegSpec for Bmctl0_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Bridge Map Control register 0"]
pub type Bmctl0 = crate::RegValueT<Bmctl0_SPEC>;

impl Bmctl0 {
    #[doc = "System bus ch0 to slave0 memory area access enable"]
    #[inline(always)]
    pub fn ch0cs0acc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Bmctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Bmctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "System bus ch0 to slave1 memory area access enable"]
    #[inline(always)]
    pub fn ch0cs1acc(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Bmctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Bmctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "System bus ch1 to slave0 memory area access enable"]
    #[inline(always)]
    pub fn ch1cs0acc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Bmctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Bmctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "System bus ch1 to slave1 memory area access enable"]
    #[inline(always)]
    pub fn ch1cs1acc(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Bmctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Bmctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Bmctl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Bmctl0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bmctl0 {
    #[inline(always)]
    fn default() -> Bmctl0 {
        <crate::RegValueT<Bmctl0_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmctl1_SPEC;
impl crate::sealed::RegSpec for Bmctl1_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Bridge Map Control register 1"]
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

    #[doc = "The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Bmctl1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Bmctl1_SPEC,crate::common::W>::from_register(self,0)
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

#[doc = "xSPI Command Map Control register ch%s"]
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

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, Cmctlch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,Cmctlch_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "xSPI Command Manual Control register 0"]
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
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        cdctl0::Peritv,
        cdctl0::Peritv,
        Cdctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            cdctl0::Peritv,
            cdctl0::Peritv,
            Cdctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Periodic transaction repeat"]
    #[inline(always)]
    pub fn perrep(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cdctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cdctl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cdctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cdctl0_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Issue 1 command (using command buffer0)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Issue 2 commands (using command buffer0-1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Peritv_SPEC;
    pub type Peritv = crate::EnumBitfieldStruct<u8, Peritv_SPEC>;
    impl Peritv {
        #[doc = "2(=2^1) cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "4(=2^2) cycles"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdctl1_SPEC;
impl crate::sealed::RegSpec for Cdctl1_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Command Manual Control register 1"]
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

#[doc = "xSPI Command Manual Control register 2"]
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

#[doc = "xSPI Command Manual Type buf%s"]
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
    ) -> crate::common::RegisterField<
        5,
        0xf,
        1,
        0,
        cdtbuf::Datasize,
        cdtbuf::Datasize,
        Cdtbuf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0xf,
            1,
            0,
            cdtbuf::Datasize,
            cdtbuf::Datasize,
            Cdtbuf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Latency cycle"]
    #[inline(always)]
    pub fn late(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1f,
        1,
        0,
        cdtbuf::Late,
        cdtbuf::Late,
        Cdtbuf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1f,
            1,
            0,
            cdtbuf::Late,
            cdtbuf::Late,
            Cdtbuf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Cdtbuf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cdtbuf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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

    #[doc = "Command (1-2byte)"]
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
        #[doc = "0byte (No command phase)"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Addsize_SPEC;
    pub type Addsize = crate::EnumBitfieldStruct<u8, Addsize_SPEC>;
    impl Addsize {
        #[doc = "0byte (No address phase)"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datasize_SPEC;
    pub type Datasize = crate::EnumBitfieldStruct<u8, Datasize_SPEC>;
    impl Datasize {
        #[doc = "0 byte (No data phase)"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Late_SPEC;
    pub type Late = crate::EnumBitfieldStruct<u8, Late_SPEC>;
    impl Late {
        #[doc = "No latency"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trtype_SPEC;
    pub type Trtype = crate::EnumBitfieldStruct<u8, Trtype_SPEC>;
    impl Trtype {
        #[doc = "Read transaction (Readout from memory device)"]
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

#[doc = "xSPI Command Manual Address buf%s"]
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

#[doc = "xSPI Command Manual Data 0 buf%s"]
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

#[doc = "xSPI Command Manual Data 1 buf%s"]
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

#[doc = "xSPI Link Pattern Control register 0"]
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
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        lpctl0::Xd1Len,
        lpctl0::Xd1Len,
        Lpctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            lpctl0::Xd1Len,
            lpctl0::Xd1Len,
            Lpctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    ) -> crate::common::RegisterField<
        24,
        0x1f,
        1,
        0,
        lpctl0::Xd2Len,
        lpctl0::Xd2Len,
        Lpctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1f,
            1,
            0,
            lpctl0::Xd2Len,
            lpctl0::Xd2Len,
            Lpctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, u8, Lpctl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8,u8,Lpctl0_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "1pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "2pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Xd1Len_SPEC;
    pub type Xd1Len = crate::EnumBitfieldStruct<u8, Xd1Len_SPEC>;
    impl Xd1Len {
        #[doc = "0 cycle"]
        pub const _0: Self = Self::new(0);
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
    pub struct Xd2Len_SPEC;
    pub type Xd2Len = crate::EnumBitfieldStruct<u8, Xd2Len_SPEC>;
    impl Xd2Len {
        #[doc = "0 cycle"]
        pub const _0: Self = Self::new(0);
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

#[doc = "xSPI Link Pattern Control register 1"]
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
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        lpctl1::Rstwid,
        lpctl1::Rstwid,
        Lpctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            lpctl1::Rstwid,
            lpctl1::Rstwid,
            Lpctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reset pattern data output setup time"]
    #[inline(always)]
    pub fn rstsu(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        lpctl1::Rstsu,
        lpctl1::Rstsu,
        Lpctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            lpctl1::Rstsu,
            lpctl1::Rstsu,
            Lpctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Lpctl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Lpctl1_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        #[doc = "5 times"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstwid_SPEC;
    pub type Rstwid = crate::EnumBitfieldStruct<u8, Rstwid_SPEC>;
    impl Rstwid {
        #[doc = "2(=2^1) cycles"]
        pub const _0: Self = Self::new(0);

        #[doc = "4(=2^2) cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstsu_SPEC;
    pub type Rstsu = crate::EnumBitfieldStruct<u8, Rstsu_SPEC>;
    impl Rstsu {
        #[doc = "1 cycle"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lioctl_SPEC;
impl crate::sealed::RegSpec for Lioctl_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Link I/O Control register"]
pub type Lioctl = crate::RegValueT<Lioctl_SPEC>;

impl Lioctl {
    #[doc = "WP drive for slave0"]
    #[inline(always)]
    pub fn wpcs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lioctl::Wpcs0,
        lioctl::Wpcs0,
        Lioctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lioctl::Wpcs0,
            lioctl::Wpcs0,
            Lioctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "WP drive for slave1"]
    #[inline(always)]
    pub fn wpcs1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Lioctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Lioctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Reset drive for slave0"]
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

    #[doc = "Reset drive for slave1"]
    #[inline(always)]
    pub fn rstcs1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Lioctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Lioctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3fff, 1, 0, u16, u16, Lioctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3fff,1,0,u16,u16,Lioctl_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Wpcs0_SPEC;
    pub type Wpcs0 = crate::EnumBitfieldStruct<u8, Wpcs0_SPEC>;
    impl Wpcs0 {
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

#[doc = "xSPI Command Calibration Control register 0 cs%s"]
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
    ) -> crate::common::RegisterField<
        8,
        0x1f,
        1,
        0,
        ccctl0cs::Caitv,
        ccctl0cs::Caitv,
        Ccctl0Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1f,
            1,
            0,
            ccctl0cs::Caitv,
            ccctl0cs::Caitv,
            Ccctl0Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Calibration DS shift start value"]
    #[inline(always)]
    pub fn casftsta(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ccctl0Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ccctl0Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Calibration DS shift end value"]
    #[inline(always)]
    pub fn casftend(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Ccctl0Cs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Ccctl0Cs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Ccctl0Cs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Ccctl0Cs_SPEC,crate::common::RW>::from_register(self,0)
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Caitv_SPEC;
    pub type Caitv = crate::EnumBitfieldStruct<u8, Caitv_SPEC>;
    impl Caitv {
        #[doc = "2(=2^1) cycle wait"]
        pub const _0: Self = Self::new(0);

        #[doc = "4(=2^1) cycle wait"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl1Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl1Cs_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Command Calibration Control register 1 cs%s"]
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
    ) -> crate::common::RegisterField<
        5,
        0xf,
        1,
        0,
        ccctl1cs::Cadatasize,
        ccctl1cs::Cadatasize,
        Ccctl1Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0xf,
            1,
            0,
            ccctl1cs::Cadatasize,
            ccctl1cs::Cadatasize,
            Ccctl1Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Latency cycle"]
    #[inline(always)]
    pub fn cawrlate(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        ccctl1cs::Cawrlate,
        ccctl1cs::Cawrlate,
        Ccctl1Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            ccctl1cs::Cawrlate,
            ccctl1cs::Cawrlate,
            Ccctl1Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read Latency cycle"]
    #[inline(always)]
    pub fn cardlate(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1f,
        1,
        0,
        ccctl1cs::Cardlate,
        ccctl1cs::Cardlate,
        Ccctl1Cs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1f,
            1,
            0,
            ccctl1cs::Cardlate,
            ccctl1cs::Cardlate,
            Ccctl1Cs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Ccctl1Cs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Ccctl1Cs_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "0byte (No command phase)"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Caaddsize_SPEC;
    pub type Caaddsize = crate::EnumBitfieldStruct<u8, Caaddsize_SPEC>;
    impl Caaddsize {
        #[doc = "0byte (No address phase)"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cadatasize_SPEC;
    pub type Cadatasize = crate::EnumBitfieldStruct<u8, Cadatasize_SPEC>;
    impl Cadatasize {
        #[doc = "1 byte"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cawrlate_SPEC;
    pub type Cawrlate = crate::EnumBitfieldStruct<u8, Cawrlate_SPEC>;
    impl Cawrlate {
        #[doc = "No latency"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cardlate_SPEC;
    pub type Cardlate = crate::EnumBitfieldStruct<u8, Cardlate_SPEC>;
    impl Cardlate {
        #[doc = "No latency"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctl2Cs_SPEC;
impl crate::sealed::RegSpec for Ccctl2Cs_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Command Calibration Control register 2 cs%s"]
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

#[doc = "xSPI Command Calibration Control register 3 cs%s"]
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

#[doc = "xSPI Command Calibration Control register 4 cs%s"]
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

#[doc = "xSPI Command Calibration Control register 5 cs%s"]
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

#[doc = "xSPI Command Calibration Control register 6 cs%s"]
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

#[doc = "xSPI Command Calibration Control register 7 cs%s"]
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
pub struct Verstt_SPEC;
impl crate::sealed::RegSpec for Verstt_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Version register"]
pub type Verstt = crate::RegValueT<Verstt_SPEC>;

impl Verstt {
    #[doc = "Version"]
    #[inline(always)]
    pub fn ver(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Verstt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Verstt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Verstt {
    #[inline(always)]
    fn default() -> Verstt {
        <crate::RegValueT<Verstt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comstt_SPEC;
impl crate::sealed::RegSpec for Comstt_SPEC {
    type DataType = u32;
}

#[doc = "xSPI Common Status register"]
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

    #[doc = "ECS monitor for slave0"]
    #[inline(always)]
    pub fn ecscs0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        comstt::Ecscs0,
        comstt::Ecscs0,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            comstt::Ecscs0,
            comstt::Ecscs0,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "INT monitor for slave0"]
    #[inline(always)]
    pub fn intcs0(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        comstt::Intcs0,
        comstt::Intcs0,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            comstt::Intcs0,
            comstt::Intcs0,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RSTO monitor for slave0"]
    #[inline(always)]
    pub fn rstocs0(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        comstt::Rstocs0,
        comstt::Rstocs0,
        Comstt_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            comstt::Rstocs0,
            comstt::Rstocs0,
            Comstt_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECS monitor for slave1"]
    #[inline(always)]
    pub fn ecscs1(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Comstt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Comstt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "INT monitor for slave1"]
    #[inline(always)]
    pub fn intcs1(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Comstt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Comstt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RSTO monitor for slave1"]
    #[inline(always)]
    pub fn rstocs1(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Comstt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Comstt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<23, 0x1ff, 1, 0, u16, u16, Comstt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<23,0x1ff,1,0,u16,u16,Comstt_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "AHB ch0 is not accessing to memory."]
        pub const _0: Self = Self::new(0);

        #[doc = "AHB ch0 is accessing to memory."]
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
    pub struct Ecscs0_SPEC;
    pub type Ecscs0 = crate::EnumBitfieldStruct<u8, Ecscs0_SPEC>;
    impl Ecscs0 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intcs0_SPEC;
    pub type Intcs0 = crate::EnumBitfieldStruct<u8, Intcs0_SPEC>;
    impl Intcs0 {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstocs0_SPEC;
    pub type Rstocs0 = crate::EnumBitfieldStruct<u8, Rstocs0_SPEC>;
    impl Rstocs0 {
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

#[doc = "xSPI Calibration Status register cs%s"]
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

#[doc = "xSPI Interrupt Status register"]
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

    #[doc = "Initial Sequence Completed"]
    #[inline(always)]
    pub fn inicmp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ints::Inicmp,
        ints::Inicmp,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ints::Inicmp,
            ints::Inicmp,
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

    #[doc = "DS timeout for slave0"]
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

    #[doc = "DS timeout for slave1"]
    #[inline(always)]
    pub fn dstocs1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ECC error detection for slave0"]
    #[inline(always)]
    pub fn ecscs0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ints::Ecscs0,
        ints::Ecscs0,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ints::Ecscs0,
            ints::Ecscs0,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "ECC error detection for slave1"]
    #[inline(always)]
    pub fn ecscs1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt detection for slave0"]
    #[inline(always)]
    pub fn intcs0(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ints::Intcs0,
        ints::Intcs0,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ints::Intcs0,
            ints::Intcs0,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt detection for slave1"]
    #[inline(always)]
    pub fn intcs1(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Bridge Buffer overflow for CH0"]
    #[inline(always)]
    pub fn brgofch0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ints::Brgofch0,
        ints::Brgofch0,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ints::Brgofch0,
            ints::Brgofch0,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer overflow for CH1"]
    #[inline(always)]
    pub fn brgofch1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Bridge Buffer underflow for CH0"]
    #[inline(always)]
    pub fn brgufch0(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ints::Brgufch0,
        ints::Brgufch0,
        Ints_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ints::Brgufch0,
            ints::Brgufch0,
            Ints_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer underflow for CH1"]
    #[inline(always)]
    pub fn brgufch1(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AHB bus error for CH0"]
    #[inline(always)]
    pub fn buserrch0(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AHB bus error for CH1"]
    #[inline(always)]
    pub fn buserrch1(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, u8, Ints_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3f,1,0,u8,u8,Ints_SPEC,crate::common::R>::from_register(self,0)
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
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ints_SPEC, crate::common::R>::from_register(
            self, 0,
        )
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
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ints_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ints_SPEC, crate::common::R>::from_register(
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
    pub struct Inicmp_SPEC;
    pub type Inicmp = crate::EnumBitfieldStruct<u8, Inicmp_SPEC>;
    impl Inicmp {
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
    pub struct Ecscs0_SPEC;
    pub type Ecscs0 = crate::EnumBitfieldStruct<u8, Ecscs0_SPEC>;
    impl Ecscs0 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intcs0_SPEC;
    pub type Intcs0 = crate::EnumBitfieldStruct<u8, Intcs0_SPEC>;
    impl Intcs0 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgofch0_SPEC;
    pub type Brgofch0 = crate::EnumBitfieldStruct<u8, Brgofch0_SPEC>;
    impl Brgofch0 {
        #[doc = "No detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgufch0_SPEC;
    pub type Brgufch0 = crate::EnumBitfieldStruct<u8, Brgufch0_SPEC>;
    impl Brgufch0 {
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
    pub struct Casuccs0_SPEC;
    pub type Casuccs0 = crate::EnumBitfieldStruct<u8, Casuccs0_SPEC>;
    impl Casuccs0 {
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

#[doc = "xSPI Interrupt Clear register"]
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

    #[doc = "Initial Sequence Completed interrupt clear"]
    #[inline(always)]
    pub fn inicmpc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        intc::Inicmpc,
        intc::Inicmpc,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            intc::Inicmpc,
            intc::Inicmpc,
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

    #[doc = "DS timeout for slave0 interrupt clear"]
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

    #[doc = "DS timeout for slave1 interrupt clear"]
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

    #[doc = "ECC error detection for slave0 interrupt clear"]
    #[inline(always)]
    pub fn ecscs0c(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        intc::Ecscs0C,
        intc::Ecscs0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            intc::Ecscs0C,
            intc::Ecscs0C,
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

    #[doc = "Interrupt detection for slave0 interrupt clear"]
    #[inline(always)]
    pub fn intcs0c(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        intc::Intcs0C,
        intc::Intcs0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            intc::Intcs0C,
            intc::Intcs0C,
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

    #[doc = "Bridge Buffer overflow for CH0 interrupt clear"]
    #[inline(always)]
    pub fn brgofch0c(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        intc::Brgofch0C,
        intc::Brgofch0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            intc::Brgofch0C,
            intc::Brgofch0C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer overflow for CH1 interrupt clear"]
    #[inline(always)]
    pub fn brgofch1c(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        intc::Brgofch1C,
        intc::Brgofch1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            intc::Brgofch1C,
            intc::Brgofch1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer underflow for CH0 interrupt clear"]
    #[inline(always)]
    pub fn brgufch0c(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        intc::Brgufch0C,
        intc::Brgufch0C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            intc::Brgufch0C,
            intc::Brgufch0C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer underflow for CH1 interrupt clear"]
    #[inline(always)]
    pub fn brgufch1c(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        intc::Brgufch1C,
        intc::Brgufch1C,
        Intc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            intc::Brgufch1C,
            intc::Brgufch1C,
            Intc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "AHB bus error for CH0 interrupt clear"]
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

    #[doc = "AHB bus error for CH1 interrupt clear"]
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

    #[doc = "The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, u8, Intc_SPEC, crate::common::W> {
        crate::common::RegisterField::<22,0x3f,1,0,u8,u8,Intc_SPEC,crate::common::W>::from_register(self,0)
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
    pub struct Inicmpc_SPEC;
    pub type Inicmpc = crate::EnumBitfieldStruct<u8, Inicmpc_SPEC>;
    impl Inicmpc {
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
    pub struct Ecscs0C_SPEC;
    pub type Ecscs0C = crate::EnumBitfieldStruct<u8, Ecscs0C_SPEC>;
    impl Ecscs0C {
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
    pub struct Intcs0C_SPEC;
    pub type Intcs0C = crate::EnumBitfieldStruct<u8, Intcs0C_SPEC>;
    impl Intcs0C {
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
    pub struct Brgofch0C_SPEC;
    pub type Brgofch0C = crate::EnumBitfieldStruct<u8, Brgofch0C_SPEC>;
    impl Brgofch0C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgofch1C_SPEC;
    pub type Brgofch1C = crate::EnumBitfieldStruct<u8, Brgofch1C_SPEC>;
    impl Brgofch1C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgufch0C_SPEC;
    pub type Brgufch0C = crate::EnumBitfieldStruct<u8, Brgufch0C_SPEC>;
    impl Brgufch0C {
        #[doc = "No change interrupt status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear interrupt status"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgufch1C_SPEC;
    pub type Brgufch1C = crate::EnumBitfieldStruct<u8, Brgufch1C_SPEC>;
    impl Brgufch1C {
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

#[doc = "xSPI Interrupt Enable register"]
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

    #[doc = "Initial Sequence Completed interrupt enable"]
    #[inline(always)]
    pub fn inicmpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        inte::Inicmpe,
        inte::Inicmpe,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            inte::Inicmpe,
            inte::Inicmpe,
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

    #[doc = "DS timeout for slave0 interrupt enable"]
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

    #[doc = "DS timeout for slave1 interrupt enable"]
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

    #[doc = "ECC error detection for slave0 interrupt enable"]
    #[inline(always)]
    pub fn ecscs0e(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        inte::Ecscs0E,
        inte::Ecscs0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            inte::Ecscs0E,
            inte::Ecscs0E,
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

    #[doc = "Interrupt detection for slave0 interrupt enable"]
    #[inline(always)]
    pub fn intcs0e(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        inte::Intcs0E,
        inte::Intcs0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            inte::Intcs0E,
            inte::Intcs0E,
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

    #[doc = "Bridge Buffer overflow for CH0 interrupt enable"]
    #[inline(always)]
    pub fn brgofch0e(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        inte::Brgofch0E,
        inte::Brgofch0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            inte::Brgofch0E,
            inte::Brgofch0E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer overflow for CH1 interrupt enable"]
    #[inline(always)]
    pub fn brgofch1e(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        inte::Brgofch1E,
        inte::Brgofch1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            inte::Brgofch1E,
            inte::Brgofch1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer underflow for CH0 interrupt enable"]
    #[inline(always)]
    pub fn brgufch0e(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        inte::Brgufch0E,
        inte::Brgufch0E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            inte::Brgufch0E,
            inte::Brgufch0E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bridge Buffer underflow for CH1 interrupt enable"]
    #[inline(always)]
    pub fn brgufch1e(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        inte::Brgufch1E,
        inte::Brgufch1E,
        Inte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            inte::Brgufch1E,
            inte::Brgufch1E,
            Inte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AHB bus error for CH0 interrupt enable"]
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

    #[doc = "AHB bus error for CH1 interrupt enable"]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, u8, Inte_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3f,1,0,u8,u8,Inte_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Inicmpe_SPEC;
    pub type Inicmpe = crate::EnumBitfieldStruct<u8, Inicmpe_SPEC>;
    impl Inicmpe {
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
    pub struct Ecscs0E_SPEC;
    pub type Ecscs0E = crate::EnumBitfieldStruct<u8, Ecscs0E_SPEC>;
    impl Ecscs0E {
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
    pub struct Intcs0E_SPEC;
    pub type Intcs0E = crate::EnumBitfieldStruct<u8, Intcs0E_SPEC>;
    impl Intcs0E {
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
    pub struct Brgofch0E_SPEC;
    pub type Brgofch0E = crate::EnumBitfieldStruct<u8, Brgofch0E_SPEC>;
    impl Brgofch0E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgofch1E_SPEC;
    pub type Brgofch1E = crate::EnumBitfieldStruct<u8, Brgofch1E_SPEC>;
    impl Brgofch1E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgufch0E_SPEC;
    pub type Brgufch0E = crate::EnumBitfieldStruct<u8, Brgufch0E_SPEC>;
    impl Brgufch0E {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brgufch1E_SPEC;
    pub type Brgufch1E = crate::EnumBitfieldStruct<u8, Brgufch1E_SPEC>;
    impl Brgufch1E {
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
