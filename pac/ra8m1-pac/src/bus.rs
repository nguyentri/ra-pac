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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"BUS Control"]
unsafe impl ::core::marker::Send for super::Bus {}
unsafe impl ::core::marker::Sync for super::Bus {}
impl super::Bus {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CS%s Mode Register"]
    #[inline(always)]
    pub const fn csmod(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Csmod_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2usize))
        }
    }
    #[inline(always)]
    pub const fn cs0mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs1mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs2mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x22usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs3mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x32usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs4mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x42usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs5mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x52usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs6mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x62usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs7mod(&self) -> &'static crate::common::Reg<self::Csmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x72usize),
            )
        }
    }

    #[doc = "CS%s Wait Control Register 1"]
    #[inline(always)]
    pub const fn cswcr1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4usize))
        }
    }
    #[inline(always)]
    pub const fn cs0wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs1wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs2wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs3wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x34usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs4wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x44usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs5wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x54usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs6wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x64usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs7wcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }

    #[doc = "CS%s Wait Control Register 2"]
    #[inline(always)]
    pub const fn cswcr2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8usize))
        }
    }
    #[inline(always)]
    pub const fn cs0wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs1wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs2wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs3wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs4wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs5wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs6wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs7wcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cswcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }

    #[doc = "CS0 Control Register"]
    #[inline(always)]
    pub const fn cs0cr(&self) -> &'static crate::common::Reg<self::Cs0Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs0Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2050usize),
            )
        }
    }

    #[doc = "CS%s Recovery Cycle Register"]
    #[inline(always)]
    pub const fn csrec(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Csrec_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80ausize))
        }
    }
    #[inline(always)]
    pub const fn cs0rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs1rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x81ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs2rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x82ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs3rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x83ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs4rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs5rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x85ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs6rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x86ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs7rec(&self) -> &'static crate::common::Reg<self::Csrec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x87ausize),
            )
        }
    }

    #[doc = "CS%s Control Register"]
    #[inline(always)]
    pub const fn cscr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cscr_SPEC, crate::common::RW>,
        7,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x812usize))
        }
    }
    #[inline(always)]
    pub const fn cs1cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x812usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs2cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x822usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs3cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x832usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs4cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x842usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs5cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x852usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs6cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x862usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cs7cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x872usize),
            )
        }
    }

    #[doc = "CS Recovery Cycle Insertion Enable Register"]
    #[inline(always)]
    pub const fn csrecen(
        &self,
    ) -> &'static crate::common::Reg<self::Csrecen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csrecen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2176usize),
            )
        }
    }

    #[doc = "SDC Control Register"]
    #[inline(always)]
    pub const fn sdccr(&self) -> &'static crate::common::Reg<self::Sdccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[doc = "SDC Mode Register"]
    #[inline(always)]
    pub const fn sdcmod(
        &self,
    ) -> &'static crate::common::Reg<self::Sdcmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdcmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3073usize),
            )
        }
    }

    #[doc = "SDRAM Access Mode Register"]
    #[inline(always)]
    pub const fn sdamod(
        &self,
    ) -> &'static crate::common::Reg<self::Sdamod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdamod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3074usize),
            )
        }
    }

    #[doc = "SDRAM Self-Refresh Control Register"]
    #[inline(always)]
    pub const fn sdself(
        &self,
    ) -> &'static crate::common::Reg<self::Sdself_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdself_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3088usize),
            )
        }
    }

    #[doc = "SDRAM Refresh Control Register"]
    #[inline(always)]
    pub const fn sdrfcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdrfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdrfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3092usize),
            )
        }
    }

    #[doc = "SDRAM Auto-Refresh Control Register"]
    #[inline(always)]
    pub const fn sdrfen(
        &self,
    ) -> &'static crate::common::Reg<self::Sdrfen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdrfen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3094usize),
            )
        }
    }

    #[doc = "SDRAM Initialization Sequence Control Register"]
    #[inline(always)]
    pub const fn sdicr(&self) -> &'static crate::common::Reg<self::Sdicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3104usize),
            )
        }
    }

    #[doc = "SDRAM Initialization Register"]
    #[inline(always)]
    pub const fn sdir(&self) -> &'static crate::common::Reg<self::Sdir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3108usize),
            )
        }
    }

    #[doc = "SDRAM Address Register"]
    #[inline(always)]
    pub const fn sdadr(&self) -> &'static crate::common::Reg<self::Sdadr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3136usize),
            )
        }
    }

    #[doc = "SDRAM Timing Register"]
    #[inline(always)]
    pub const fn sdtr(&self) -> &'static crate::common::Reg<self::Sdtr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdtr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3140usize),
            )
        }
    }

    #[doc = "SDRAM Mode Register"]
    #[inline(always)]
    pub const fn sdmod(&self) -> &'static crate::common::Reg<self::Sdmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3144usize),
            )
        }
    }

    #[doc = "SDRAM Status Register"]
    #[inline(always)]
    pub const fn sdsr(&self) -> &'static crate::common::Reg<self::Sdsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sdsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3152usize),
            )
        }
    }

    #[doc = "BUS_Operation_After_Detection Register"]
    #[inline(always)]
    pub const fn busoad(
        &self,
    ) -> &'static crate::common::Reg<self::Busoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

    #[doc = "BUS_Operation_After_Detection Protect Register"]
    #[inline(always)]
    pub const fn busoadpt(
        &self,
    ) -> &'static crate::common::Reg<self::Busoadpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busoadpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4100usize),
            )
        }
    }

    #[doc = "Bus Master Arbitration Control Register"]
    #[inline(always)]
    pub const fn busmabt(
        &self,
    ) -> &'static crate::common::Reg<self::Busmabt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmabt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 1 FHBI"]
    #[inline(always)]
    pub const fn bussabt1fhbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt1Fhbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt1Fhbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4608usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0 FLBI"]
    #[inline(always)]
    pub const fn bussabt0flbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Flbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Flbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4624usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 1 S0BI"]
    #[inline(always)]
    pub const fn bussabt1s0bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt1S0Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt1S0Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4632usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 1 S1BI"]
    #[inline(always)]
    pub const fn bussabt1s1bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt1S1Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt1S1Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4640usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0 STBYSBI"]
    #[inline(always)]
    pub const fn bussabt0stbysbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Stbysbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Stbysbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4680usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 2 ECBI"]
    #[inline(always)]
    pub const fn bussabt2ecbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt2Ecbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt2Ecbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4688usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 2 EOBI"]
    #[inline(always)]
    pub const fn bussabt2eobi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt2Eobi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt2Eobi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4696usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0 PBBI"]
    #[inline(always)]
    pub const fn bussabt0pbbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Pbbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Pbbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4704usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0 PABI"]
    #[inline(always)]
    pub const fn bussabt0pabi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Pabi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Pabi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4712usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0 PIBI"]
    #[inline(always)]
    pub const fn bussabt0pibi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Pibi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Pibi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4720usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0 PSBI"]
    #[inline(always)]
    pub const fn bussabt0psbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Psbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Psbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4728usize),
            )
        }
    }

    #[doc = "Bus Divider Bypass Register"]
    #[inline(always)]
    pub const fn busdivbyp(
        &self,
    ) -> &'static crate::common::Reg<self::Busdivbyp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busdivbyp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4864usize),
            )
        }
    }

    #[doc = "QoS Priority Setting After Reach Threshold"]
    #[inline(always)]
    pub const fn qospri(
        &self,
    ) -> &'static crate::common::Reg<self::Qospri_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qospri_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5120usize),
            )
        }
    }

    #[doc = "QoS Cycle Setting Register %s"]
    #[inline(always)]
    pub const fn qoscyc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Qoscyc_SPEC, crate::common::RW>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1410usize))
        }
    }
    #[inline(always)]
    pub const fn qoscyc0(
        &self,
    ) -> &'static crate::common::Reg<self::Qoscyc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qoscyc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1410usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qoscyc1(
        &self,
    ) -> &'static crate::common::Reg<self::Qoscyc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qoscyc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1418usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qoscyc2(
        &self,
    ) -> &'static crate::common::Reg<self::Qoscyc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qoscyc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1420usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qoscyc3(
        &self,
    ) -> &'static crate::common::Reg<self::Qoscyc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qoscyc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1428usize),
            )
        }
    }

    #[doc = "Bus Master priority setting register"]
    #[inline(always)]
    pub const fn busmpri(
        &self,
    ) -> &'static crate::common::Reg<self::Busmpri_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmpri_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5168usize),
            )
        }
    }

    #[doc = "QoS Cycle Setting Register 4"]
    #[inline(always)]
    pub const fn qoscyc4(
        &self,
    ) -> &'static crate::common::Reg<self::Qoscyc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qoscyc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5176usize),
            )
        }
    }

    #[doc = "QoS Threshold Setting Register %s"]
    #[inline(always)]
    pub const fn qosthd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Qosthd_SPEC, crate::common::RW>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1500usize))
        }
    }
    #[inline(always)]
    pub const fn qosthd0(
        &self,
    ) -> &'static crate::common::Reg<self::Qosthd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosthd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1500usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qosthd1(
        &self,
    ) -> &'static crate::common::Reg<self::Qosthd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosthd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1508usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qosthd2(
        &self,
    ) -> &'static crate::common::Reg<self::Qosthd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosthd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1510usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qosthd3(
        &self,
    ) -> &'static crate::common::Reg<self::Qosthd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosthd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1518usize),
            )
        }
    }

    #[doc = "QoS Threshold Setting Register 4"]
    #[inline(always)]
    pub const fn qosthd4(
        &self,
    ) -> &'static crate::common::Reg<self::Qosthd4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosthd4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5416usize),
            )
        }
    }

    #[doc = "QoS Data Transferred Amount Monitor Register %s"]
    #[inline(always)]
    pub const fn qosdmon(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Qosdmon_SPEC, crate::common::RW>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1600usize))
        }
    }
    #[inline(always)]
    pub const fn qosdmon0(
        &self,
    ) -> &'static crate::common::Reg<self::Qosdmon_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosdmon_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1600usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qosdmon1(
        &self,
    ) -> &'static crate::common::Reg<self::Qosdmon_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosdmon_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1608usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qosdmon2(
        &self,
    ) -> &'static crate::common::Reg<self::Qosdmon_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosdmon_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1610usize),
            )
        }
    }
    #[inline(always)]
    pub const fn qosdmon3(
        &self,
    ) -> &'static crate::common::Reg<self::Qosdmon_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosdmon_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1618usize),
            )
        }
    }

    #[doc = "QoS Data Transferred Amount Monitor Register 4"]
    #[inline(always)]
    pub const fn qosdmon4(
        &self,
    ) -> &'static crate::common::Reg<self::Qosdmon4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qosdmon4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5672usize),
            )
        }
    }

    #[doc = "Bus Error Address Register %s"]
    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1830usize))
        }
    }
    #[inline(always)]
    pub const fn bus4erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1830usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus5erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1840usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus6erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1850usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus7erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1860usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus8erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1870usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus9erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1880usize),
            )
        }
    }

    #[doc = "Bus Error RW Register %s"]
    #[inline(always)]
    pub const fn buserrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrrw_SPEC, crate::common::R>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1834usize))
        }
    }
    #[inline(always)]
    pub const fn bus4errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus5errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus6errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus7errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus8errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1874usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus9errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1884usize),
            )
        }
    }

    #[doc = "Bus TZF Error Address Register %s"]
    #[inline(always)]
    pub const fn btzferradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferradd_SPEC, crate::common::R>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1930usize))
        }
    }
    #[inline(always)]
    pub const fn btzf4erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1930usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf5erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1940usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf6erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1950usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf7erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1960usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf8erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1970usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf9erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1980usize),
            )
        }
    }

    #[doc = "Bus ZTF Error RW Register %s"]
    #[inline(always)]
    pub const fn btzferrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferrrw_SPEC, crate::common::R>,
        6,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1934usize))
        }
    }
    #[inline(always)]
    pub const fn btzf4errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1934usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf5errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1944usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf6errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1954usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf7errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1964usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf8errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1974usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf9errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1984usize),
            )
        }
    }

    #[doc = "Bus Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        12,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a00usize))
        }
    }
    #[inline(always)]
    pub const fn bus1errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a00usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus2errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus3errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus4errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a30usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus5errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a40usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus6errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a50usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus7errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a60usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus8errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus9errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus10errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus11errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1aa0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus12errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ab0usize),
            )
        }
    }

    #[doc = "Bus Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrclr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW>,
        12,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a08usize))
        }
    }
    #[inline(always)]
    pub const fn bus1errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a08usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus2errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus3errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus4errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a38usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus5errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a48usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus6errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a58usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus7errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a68usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus8errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus9errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus10errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus11errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1aa8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus12errclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1ab8usize),
            )
        }
    }

    #[doc = "Master Bufferable Write Error Status Register"]
    #[inline(always)]
    pub const fn mbwerrstat(
        &self,
    ) -> &'static crate::common::Reg<self::Mbwerrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mbwerrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6912usize),
            )
        }
    }

    #[doc = "Master Bufferable Write Error Clear Register"]
    #[inline(always)]
    pub const fn mbwerrclr(
        &self,
    ) -> &'static crate::common::Reg<self::Mbwerrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mbwerrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6920usize),
            )
        }
    }

    #[doc = "Slave Bufferable Write Error Status Register"]
    #[inline(always)]
    pub const fn sbwerrstat(
        &self,
    ) -> &'static crate::common::Reg<self::Sbwerrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sbwerrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6944usize),
            )
        }
    }

    #[doc = "Slave Bufferable Write Error Clear Register"]
    #[inline(always)]
    pub const fn sbwerrclr(
        &self,
    ) -> &'static crate::common::Reg<self::Sbwerrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sbwerrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6952usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csmod_SPEC;
impl crate::sealed::RegSpec for Csmod_SPEC {
    type DataType = u16;
}

#[doc = "CS%s Mode Register"]
pub type Csmod = crate::RegValueT<Csmod_SPEC>;

impl Csmod {
    #[doc = "Write Access Mode Select"]
    #[inline(always)]
    pub fn wrmod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        csmod::Wrmod,
        csmod::Wrmod,
        Csmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            csmod::Wrmod,
            csmod::Wrmod,
            Csmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Wait Enable"]
    #[inline(always)]
    pub fn ewenb(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        csmod::Ewenb,
        csmod::Ewenb,
        Csmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            csmod::Ewenb,
            csmod::Ewenb,
            Csmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Page Read Access Enable"]
    #[inline(always)]
    pub fn prenb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        csmod::Prenb,
        csmod::Prenb,
        Csmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            csmod::Prenb,
            csmod::Prenb,
            Csmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Page Write Access Enable"]
    #[inline(always)]
    pub fn pwenb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        csmod::Pwenb,
        csmod::Pwenb,
        Csmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            csmod::Pwenb,
            csmod::Pwenb,
            Csmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, Csmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,Csmod_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Page Read Access Mode Select"]
    #[inline(always)]
    pub fn prmod(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        csmod::Prmod,
        csmod::Prmod,
        Csmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            csmod::Prmod,
            csmod::Prmod,
            Csmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Csmod {
    #[inline(always)]
    fn default() -> Csmod {
        <crate::RegValueT<Csmod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod csmod {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrmod_SPEC;
    pub type Wrmod = crate::EnumBitfieldStruct<u8, Wrmod_SPEC>;
    impl Wrmod {
        #[doc = "Byte strobe mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single write strobe mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewenb_SPEC;
    pub type Ewenb = crate::EnumBitfieldStruct<u8, Ewenb_SPEC>;
    impl Ewenb {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prenb_SPEC;
    pub type Prenb = crate::EnumBitfieldStruct<u8, Prenb_SPEC>;
    impl Prenb {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwenb_SPEC;
    pub type Pwenb = crate::EnumBitfieldStruct<u8, Pwenb_SPEC>;
    impl Pwenb {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prmod_SPEC;
    pub type Prmod = crate::EnumBitfieldStruct<u8, Prmod_SPEC>;
    impl Prmod {
        #[doc = "Normal access compatible mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "External data read continuous assertion mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cswcr1_SPEC;
impl crate::sealed::RegSpec for Cswcr1_SPEC {
    type DataType = u32;
}

#[doc = "CS%s Wait Control Register 1"]
pub type Cswcr1 = crate::RegValueT<Cswcr1_SPEC>;

impl Cswcr1 {
    #[doc = "Page Write Cycle Wait SelectNOTE: The CSPWWAIT value is valid only when the PWENB bit in CSnMOD is set to 1."]
    #[inline(always)]
    pub fn cspwwait(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cswcr1::Cspwwait,
        cswcr1::Cspwwait,
        Cswcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cswcr1::Cspwwait,
            cswcr1::Cspwwait,
            Cswcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Page Read Cycle Wait SelectNOTE: The CSPRWAIT value is valid only when the PRENB bit in CSnMOD is set to 1."]
    #[inline(always)]
    pub fn csprwait(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        cswcr1::Csprwait,
        cswcr1::Csprwait,
        Cswcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            cswcr1::Csprwait,
            cswcr1::Csprwait,
            Cswcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Normal Write Cycle Wait Select"]
    #[inline(always)]
    pub fn cswwait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        cswcr1::Cswwait,
        cswcr1::Cswwait,
        Cswcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            cswcr1::Cswwait,
            cswcr1::Cswwait,
            Cswcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Normal Read Cycle Wait Select"]
    #[inline(always)]
    pub fn csrwait(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1f,
        1,
        0,
        cswcr1::Csrwait,
        cswcr1::Csrwait,
        Cswcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1f,
            1,
            0,
            cswcr1::Csrwait,
            cswcr1::Csrwait,
            Cswcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Cswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Cswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cswcr1 {
    #[inline(always)]
    fn default() -> Cswcr1 {
        <crate::RegValueT<Cswcr1_SPEC> as RegisterValue<_>>::new(117901063)
    }
}
pub mod cswcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cspwwait_SPEC;
    pub type Cspwwait = crate::EnumBitfieldStruct<u8, Cspwwait_SPEC>;
    impl Cspwwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csprwait_SPEC;
    pub type Csprwait = crate::EnumBitfieldStruct<u8, Csprwait_SPEC>;
    impl Csprwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cswwait_SPEC;
    pub type Cswwait = crate::EnumBitfieldStruct<u8, Cswwait_SPEC>;
    impl Cswwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csrwait_SPEC;
    pub type Csrwait = crate::EnumBitfieldStruct<u8, Csrwait_SPEC>;
    impl Csrwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cswcr2_SPEC;
impl crate::sealed::RegSpec for Cswcr2_SPEC {
    type DataType = u32;
}

#[doc = "CS%s Wait Control Register 2"]
pub type Cswcr2 = crate::RegValueT<Cswcr2_SPEC>;

impl Cswcr2 {
    #[doc = "Read Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn csroff(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cswcr2::Csroff,
        cswcr2::Csroff,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cswcr2::Csroff,
            cswcr2::Csroff,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn cswoff(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        cswcr2::Cswoff,
        cswcr2::Cswoff,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            cswcr2::Cswoff,
            cswcr2::Cswoff,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Data Output Extension Cycle Select"]
    #[inline(always)]
    pub fn wdoff(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        cswcr2::Wdoff,
        cswcr2::Wdoff,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            cswcr2::Wdoff,
            cswcr2::Wdoff,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Cycle Wait Select"]
    #[inline(always)]
    pub fn r#await(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        cswcr2::Await,
        cswcr2::Await,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            cswcr2::Await,
            cswcr2::Await,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RD Assert Wait Select"]
    #[inline(always)]
    pub fn rdon(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        cswcr2::Rdon,
        cswcr2::Rdon,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            cswcr2::Rdon,
            cswcr2::Rdon,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "WR Assert Wait Select"]
    #[inline(always)]
    pub fn wron(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
        cswcr2::Wron,
        cswcr2::Wron,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x7,
            1,
            0,
            cswcr2::Wron,
            cswcr2::Wron,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Data Output Wait Select"]
    #[inline(always)]
    pub fn wdon(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        cswcr2::Wdon,
        cswcr2::Wdon,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            cswcr2::Wdon,
            cswcr2::Wdon,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CS Assert Wait Select"]
    #[inline(always)]
    pub fn cson(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        cswcr2::Cson,
        cswcr2::Cson,
        Cswcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            cswcr2::Cson,
            cswcr2::Cson,
            Cswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cswcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cswcr2 {
    #[inline(always)]
    fn default() -> Cswcr2 {
        <crate::RegValueT<Cswcr2_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod cswcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csroff_SPEC;
    pub type Csroff = crate::EnumBitfieldStruct<u8, Csroff_SPEC>;
    impl Csroff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cswoff_SPEC;
    pub type Cswoff = crate::EnumBitfieldStruct<u8, Cswoff_SPEC>;
    impl Cswoff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdoff_SPEC;
    pub type Wdoff = crate::EnumBitfieldStruct<u8, Wdoff_SPEC>;
    impl Wdoff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Await_SPEC;
    pub type Await = crate::EnumBitfieldStruct<u8, Await_SPEC>;
    impl Await {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdon_SPEC;
    pub type Rdon = crate::EnumBitfieldStruct<u8, Rdon_SPEC>;
    impl Rdon {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wron_SPEC;
    pub type Wron = crate::EnumBitfieldStruct<u8, Wron_SPEC>;
    impl Wron {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdon_SPEC;
    pub type Wdon = crate::EnumBitfieldStruct<u8, Wdon_SPEC>;
    impl Wdon {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cson_SPEC;
    pub type Cson = crate::EnumBitfieldStruct<u8, Cson_SPEC>;
    impl Cson {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs0Cr_SPEC;
impl crate::sealed::RegSpec for Cs0Cr_SPEC {
    type DataType = u16;
}

#[doc = "CS0 Control Register"]
pub type Cs0Cr = crate::RegValueT<Cs0Cr_SPEC>;

impl Cs0Cr {
    #[doc = "Operation Enable"]
    #[inline(always)]
    pub fn exenb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cs0cr::Exenb,
        cs0cr::Exenb,
        Cs0Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cs0cr::Exenb,
            cs0cr::Exenb,
            Cs0Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Bus Width Select"]
    #[inline(always)]
    pub fn bsize(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        cs0cr::Bsize,
        cs0cr::Bsize,
        Cs0Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            cs0cr::Bsize,
            cs0cr::Bsize,
            Cs0Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Endian Mode"]
    #[inline(always)]
    pub fn emode(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cs0cr::Emode,
        cs0cr::Emode,
        Cs0Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cs0cr::Emode,
            cs0cr::Emode,
            Cs0Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address/Data Multiplexed I/O Interface Select"]
    #[inline(always)]
    pub fn mpxen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cs0cr::Mpxen,
        cs0cr::Mpxen,
        Cs0Cr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cs0cr::Mpxen,
            cs0cr::Mpxen,
            Cs0Cr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, Cs0Cr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,Cs0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cs0Cr {
    #[inline(always)]
    fn default() -> Cs0Cr {
        <crate::RegValueT<Cs0Cr_SPEC> as RegisterValue<_>>::new(33)
    }
}
pub mod cs0cr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disable operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        #[doc = "16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "32-bit bus space"]
        pub const _01: Self = Self::new(1);

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        #[doc = "Little endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big endian"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpxen_SPEC;
    pub type Mpxen = crate::EnumBitfieldStruct<u8, Mpxen_SPEC>;
    impl Mpxen {
        #[doc = "Separate bus interface is selected for area n"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csrec_SPEC;
impl crate::sealed::RegSpec for Csrec_SPEC {
    type DataType = u16;
}

#[doc = "CS%s Recovery Cycle Register"]
pub type Csrec = crate::RegValueT<Csrec_SPEC>;

impl Csrec {
    #[doc = "Read Recovery"]
    #[inline(always)]
    pub fn rrcv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        csrec::Rrcv,
        csrec::Rrcv,
        Csrec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            csrec::Rrcv,
            csrec::Rrcv,
            Csrec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Recovery"]
    #[inline(always)]
    pub fn wrcv(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        csrec::Wrcv,
        csrec::Wrcv,
        Csrec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            csrec::Wrcv,
            csrec::Wrcv,
            Csrec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Csrec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Csrec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Csrec {
    #[inline(always)]
    fn default() -> Csrec {
        <crate::RegValueT<Csrec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod csrec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrcv_SPEC;
    pub type Rrcv = crate::EnumBitfieldStruct<u8, Rrcv_SPEC>;
    impl Rrcv {
        #[doc = "No recovery cycle is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrcv_SPEC;
    pub type Wrcv = crate::EnumBitfieldStruct<u8, Wrcv_SPEC>;
    impl Wrcv {
        #[doc = "No recovery cycle is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cscr_SPEC;
impl crate::sealed::RegSpec for Cscr_SPEC {
    type DataType = u16;
}

#[doc = "CS%s Control Register"]
pub type Cscr = crate::RegValueT<Cscr_SPEC>;

impl Cscr {
    #[doc = "Operation Enable"]
    #[inline(always)]
    pub fn exenb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cscr::Exenb,
        cscr::Exenb,
        Cscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cscr::Exenb,
            cscr::Exenb,
            Cscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "External Bus Width Select"]
    #[inline(always)]
    pub fn bsize(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        cscr::Bsize,
        cscr::Bsize,
        Cscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            cscr::Bsize,
            cscr::Bsize,
            Cscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Endian Mode"]
    #[inline(always)]
    pub fn emode(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cscr::Emode,
        cscr::Emode,
        Cscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cscr::Emode,
            cscr::Emode,
            Cscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address/Data Multiplexed I/O Interface Select"]
    #[inline(always)]
    pub fn mpxen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cscr::Mpxen,
        cscr::Mpxen,
        Cscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cscr::Mpxen,
            cscr::Mpxen,
            Cscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, Cscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,Cscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cscr {
    #[inline(always)]
    fn default() -> Cscr {
        <crate::RegValueT<Cscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disable operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        #[doc = "16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "32-bit bus space"]
        pub const _01: Self = Self::new(1);

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        #[doc = "Little endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big endian"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpxen_SPEC;
    pub type Mpxen = crate::EnumBitfieldStruct<u8, Mpxen_SPEC>;
    impl Mpxen {
        #[doc = "Separate bus interface is selected for area n"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csrecen_SPEC;
impl crate::sealed::RegSpec for Csrecen_SPEC {
    type DataType = u16;
}

#[doc = "CS Recovery Cycle Insertion Enable Register"]
pub type Csrecen = crate::RegValueT<Csrecen_SPEC>;

impl Csrecen {
    #[doc = "Separate Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    pub fn rcven0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        csrecen::Rcven0,
        csrecen::Rcven0,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            csrecen::Rcven0,
            csrecen::Rcven0,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    pub fn rcven1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        csrecen::Rcven1,
        csrecen::Rcven1,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            csrecen::Rcven1,
            csrecen::Rcven1,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    pub fn rcven2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        csrecen::Rcven2,
        csrecen::Rcven2,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            csrecen::Rcven2,
            csrecen::Rcven2,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    pub fn rcven3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        csrecen::Rcven3,
        csrecen::Rcven3,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            csrecen::Rcven3,
            csrecen::Rcven3,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    pub fn rcven4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        csrecen::Rcven4,
        csrecen::Rcven4,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            csrecen::Rcven4,
            csrecen::Rcven4,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    pub fn rcven5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        csrecen::Rcven5,
        csrecen::Rcven5,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            csrecen::Rcven5,
            csrecen::Rcven5,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    pub fn rcven6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        csrecen::Rcven6,
        csrecen::Rcven6,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            csrecen::Rcven6,
            csrecen::Rcven6,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    pub fn rcven7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        csrecen::Rcven7,
        csrecen::Rcven7,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            csrecen::Rcven7,
            csrecen::Rcven7,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    pub fn rcvenm0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        csrecen::Rcvenm0,
        csrecen::Rcvenm0,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            csrecen::Rcvenm0,
            csrecen::Rcvenm0,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    pub fn rcvenm1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        csrecen::Rcvenm1,
        csrecen::Rcvenm1,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            csrecen::Rcvenm1,
            csrecen::Rcvenm1,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    pub fn rcvenm2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        csrecen::Rcvenm2,
        csrecen::Rcvenm2,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            csrecen::Rcvenm2,
            csrecen::Rcvenm2,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    pub fn rcvenm3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        csrecen::Rcvenm3,
        csrecen::Rcvenm3,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            csrecen::Rcvenm3,
            csrecen::Rcvenm3,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    pub fn rcvenm4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        csrecen::Rcvenm4,
        csrecen::Rcvenm4,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            csrecen::Rcvenm4,
            csrecen::Rcvenm4,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    pub fn rcvenm5(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        csrecen::Rcvenm5,
        csrecen::Rcvenm5,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            csrecen::Rcvenm5,
            csrecen::Rcvenm5,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    pub fn rcvenm6(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        csrecen::Rcvenm6,
        csrecen::Rcvenm6,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            csrecen::Rcvenm6,
            csrecen::Rcvenm6,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    pub fn rcvenm7(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        csrecen::Rcvenm7,
        csrecen::Rcvenm7,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            csrecen::Rcvenm7,
            csrecen::Rcvenm7,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Csrecen {
    #[inline(always)]
    fn default() -> Csrecen {
        <crate::RegValueT<Csrecen_SPEC> as RegisterValue<_>>::new(15934)
    }
}
pub mod csrecen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven0_SPEC;
    pub type Rcven0 = crate::EnumBitfieldStruct<u8, Rcven0_SPEC>;
    impl Rcven0 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven1_SPEC;
    pub type Rcven1 = crate::EnumBitfieldStruct<u8, Rcven1_SPEC>;
    impl Rcven1 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven2_SPEC;
    pub type Rcven2 = crate::EnumBitfieldStruct<u8, Rcven2_SPEC>;
    impl Rcven2 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven3_SPEC;
    pub type Rcven3 = crate::EnumBitfieldStruct<u8, Rcven3_SPEC>;
    impl Rcven3 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven4_SPEC;
    pub type Rcven4 = crate::EnumBitfieldStruct<u8, Rcven4_SPEC>;
    impl Rcven4 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven5_SPEC;
    pub type Rcven5 = crate::EnumBitfieldStruct<u8, Rcven5_SPEC>;
    impl Rcven5 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven6_SPEC;
    pub type Rcven6 = crate::EnumBitfieldStruct<u8, Rcven6_SPEC>;
    impl Rcven6 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven7_SPEC;
    pub type Rcven7 = crate::EnumBitfieldStruct<u8, Rcven7_SPEC>;
    impl Rcven7 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm0_SPEC;
    pub type Rcvenm0 = crate::EnumBitfieldStruct<u8, Rcvenm0_SPEC>;
    impl Rcvenm0 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm1_SPEC;
    pub type Rcvenm1 = crate::EnumBitfieldStruct<u8, Rcvenm1_SPEC>;
    impl Rcvenm1 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm2_SPEC;
    pub type Rcvenm2 = crate::EnumBitfieldStruct<u8, Rcvenm2_SPEC>;
    impl Rcvenm2 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm3_SPEC;
    pub type Rcvenm3 = crate::EnumBitfieldStruct<u8, Rcvenm3_SPEC>;
    impl Rcvenm3 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm4_SPEC;
    pub type Rcvenm4 = crate::EnumBitfieldStruct<u8, Rcvenm4_SPEC>;
    impl Rcvenm4 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm5_SPEC;
    pub type Rcvenm5 = crate::EnumBitfieldStruct<u8, Rcvenm5_SPEC>;
    impl Rcvenm5 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm6_SPEC;
    pub type Rcvenm6 = crate::EnumBitfieldStruct<u8, Rcvenm6_SPEC>;
    impl Rcvenm6 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm7_SPEC;
    pub type Rcvenm7 = crate::EnumBitfieldStruct<u8, Rcvenm7_SPEC>;
    impl Rcvenm7 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdccr_SPEC;
impl crate::sealed::RegSpec for Sdccr_SPEC {
    type DataType = u8;
}

#[doc = "SDC Control Register"]
pub type Sdccr = crate::RegValueT<Sdccr_SPEC>;

impl Sdccr {
    #[doc = "Operation Enable"]
    #[inline(always)]
    pub fn exenb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdccr::Exenb,
        sdccr::Exenb,
        Sdccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdccr::Exenb,
            sdccr::Exenb,
            Sdccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDRAM Bus Width Select"]
    #[inline(always)]
    pub fn bsize(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        sdccr::Bsize,
        sdccr::Bsize,
        Sdccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            sdccr::Bsize,
            sdccr::Bsize,
            Sdccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Sdccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Sdccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdccr {
    #[inline(always)]
    fn default() -> Sdccr {
        <crate::RegValueT<Sdccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        #[doc = "16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "32-bit bus space"]
        pub const _01: Self = Self::new(1);

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdcmod_SPEC;
impl crate::sealed::RegSpec for Sdcmod_SPEC {
    type DataType = u8;
}

#[doc = "SDC Mode Register"]
pub type Sdcmod = crate::RegValueT<Sdcmod_SPEC>;

impl Sdcmod {
    #[doc = "Endian Mode"]
    #[inline(always)]
    pub fn emode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdcmod::Emode,
        sdcmod::Emode,
        Sdcmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdcmod::Emode,
            sdcmod::Emode,
            Sdcmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sdcmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sdcmod_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdcmod {
    #[inline(always)]
    fn default() -> Sdcmod {
        <crate::RegValueT<Sdcmod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdcmod {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        #[doc = "Endian order of SDRAM address space is the same as the endian order of the operating mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Endian order of SDRAM address space is not the endian order of the operating mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdamod_SPEC;
impl crate::sealed::RegSpec for Sdamod_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Access Mode Register"]
pub type Sdamod = crate::RegValueT<Sdamod_SPEC>;

impl Sdamod {
    #[doc = "Continuous Access Enable"]
    #[inline(always)]
    pub fn be(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdamod::Be,
        sdamod::Be,
        Sdamod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdamod::Be,
            sdamod::Be,
            Sdamod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sdamod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sdamod_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdamod {
    #[inline(always)]
    fn default() -> Sdamod {
        <crate::RegValueT<Sdamod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdamod {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Be_SPEC;
    pub type Be = crate::EnumBitfieldStruct<u8, Be_SPEC>;
    impl Be {
        #[doc = "Continuous access is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Continuous access is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdself_SPEC;
impl crate::sealed::RegSpec for Sdself_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Self-Refresh Control Register"]
pub type Sdself = crate::RegValueT<Sdself_SPEC>;

impl Sdself {
    #[doc = "SDRAM Self-Refresh Enable"]
    #[inline(always)]
    pub fn sfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdself::Sfen,
        sdself::Sfen,
        Sdself_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdself::Sfen,
            sdself::Sfen,
            Sdself_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sdself_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sdself_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdself {
    #[inline(always)]
    fn default() -> Sdself {
        <crate::RegValueT<Sdself_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdself {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfen_SPEC;
    pub type Sfen = crate::EnumBitfieldStruct<u8, Sfen_SPEC>;
    impl Sfen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdrfcr_SPEC;
impl crate::sealed::RegSpec for Sdrfcr_SPEC {
    type DataType = u16;
}

#[doc = "SDRAM Refresh Control Register"]
pub type Sdrfcr = crate::RegValueT<Sdrfcr_SPEC>;

impl Sdrfcr {
    #[doc = "Auto-Refresh Request Interval Setting"]
    #[inline(always)]
    pub fn rfc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        sdrfcr::Rfc,
        sdrfcr::Rfc,
        Sdrfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            sdrfcr::Rfc,
            sdrfcr::Rfc,
            Sdrfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )"]
    #[inline(always)]
    pub fn refw(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Sdrfcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Sdrfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdrfcr {
    #[inline(always)]
    fn default() -> Sdrfcr {
        <crate::RegValueT<Sdrfcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sdrfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfc_SPEC;
    pub type Rfc = crate::EnumBitfieldStruct<u8, Rfc_SPEC>;
    impl Rfc {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdrfen_SPEC;
impl crate::sealed::RegSpec for Sdrfen_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Auto-Refresh Control Register"]
pub type Sdrfen = crate::RegValueT<Sdrfen_SPEC>;

impl Sdrfen {
    #[doc = "Auto-Refresh Operation Enable"]
    #[inline(always)]
    pub fn rfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdrfen::Rfen,
        sdrfen::Rfen,
        Sdrfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdrfen::Rfen,
            sdrfen::Rfen,
            Sdrfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sdrfen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sdrfen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdrfen {
    #[inline(always)]
    fn default() -> Sdrfen {
        <crate::RegValueT<Sdrfen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdrfen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfen_SPEC;
    pub type Rfen = crate::EnumBitfieldStruct<u8, Rfen_SPEC>;
    impl Rfen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdicr_SPEC;
impl crate::sealed::RegSpec for Sdicr_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Initialization Sequence Control Register"]
pub type Sdicr = crate::RegValueT<Sdicr_SPEC>;

impl Sdicr {
    #[doc = "Initialization Sequence Start"]
    #[inline(always)]
    pub fn inirq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdicr::Inirq,
        sdicr::Inirq,
        Sdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdicr::Inirq,
            sdicr::Inirq,
            Sdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sdicr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sdicr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdicr {
    #[inline(always)]
    fn default() -> Sdicr {
        <crate::RegValueT<Sdicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inirq_SPEC;
    pub type Inirq = crate::EnumBitfieldStruct<u8, Inirq_SPEC>;
    impl Inirq {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initialization sequence starts"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdir_SPEC;
impl crate::sealed::RegSpec for Sdir_SPEC {
    type DataType = u16;
}

#[doc = "SDRAM Initialization Register"]
pub type Sdir = crate::RegValueT<Sdir_SPEC>;

impl Sdir {
    #[doc = "Initialization Auto-Refresh Interval ( PRF+3 cycles )"]
    #[inline(always)]
    pub fn arfi(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Sdir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Sdir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Initialization Auto-Refresh Count"]
    #[inline(always)]
    pub fn arfc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        sdir::Arfc,
        sdir::Arfc,
        Sdir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            sdir::Arfc,
            sdir::Arfc,
            Sdir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Initialization Precharge Cycle Count   ( PRF+3 cycles )"]
    #[inline(always)]
    pub fn prc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Sdir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Sdir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, Sdir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,Sdir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdir {
    #[inline(always)]
    fn default() -> Sdir {
        <crate::RegValueT<Sdir_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod sdir {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arfc_SPEC;
    pub type Arfc = crate::EnumBitfieldStruct<u8, Arfc_SPEC>;
    impl Arfc {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadr_SPEC;
impl crate::sealed::RegSpec for Sdadr_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Address Register"]
pub type Sdadr = crate::RegValueT<Sdadr_SPEC>;

impl Sdadr {
    #[doc = "Address Multiplex Select"]
    #[inline(always)]
    pub fn mxc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sdadr::Mxc,
        sdadr::Mxc,
        Sdadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sdadr::Mxc,
            sdadr::Mxc,
            Sdadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Sdadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Sdadr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadr {
    #[inline(always)]
    fn default() -> Sdadr {
        <crate::RegValueT<Sdadr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mxc_SPEC;
    pub type Mxc = crate::EnumBitfieldStruct<u8, Mxc_SPEC>;
    impl Mxc {
        #[doc = "8-bit shift"]
        pub const _00: Self = Self::new(0);

        #[doc = "9-bit shift"]
        pub const _01: Self = Self::new(1);

        #[doc = "10-bit shift"]
        pub const _10: Self = Self::new(2);

        #[doc = "11-bit shift"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdtr_SPEC;
impl crate::sealed::RegSpec for Sdtr_SPEC {
    type DataType = u32;
}

#[doc = "SDRAM Timing Register"]
pub type Sdtr = crate::RegValueT<Sdtr_SPEC>;

impl Sdtr {
    #[doc = "SDRAMC Column Latency"]
    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, sdtr::Cl, sdtr::Cl, Sdtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,sdtr::Cl,sdtr::Cl,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Recovery Interval"]
    #[inline(always)]
    pub fn wr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sdtr::Wr, sdtr::Wr, Sdtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,sdtr::Wr,sdtr::Wr,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Row Precharge Interval ( RP+1  cycles )"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, Sdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Row Column Latency ( RCD+1  cycles )"]
    #[inline(always)]
    pub fn rcd(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Sdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Row Active Interval"]
    #[inline(always)]
    pub fn rai(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        sdtr::Rai,
        sdtr::Rai,
        Sdtr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            sdtr::Rai,
            sdtr::Rai,
            Sdtr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<19, 0x1fff, 1, 0, u16, u16, Sdtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1fff,1,0,u16,u16,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdtr {
    #[inline(always)]
    fn default() -> Sdtr {
        <crate::RegValueT<Sdtr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod sdtr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cl_SPEC;
    pub type Cl = crate::EnumBitfieldStruct<u8, Cl_SPEC>;
    impl Cl {
        #[doc = "1 cycle"]
        pub const _001: Self = Self::new(1);

        #[doc = "2 cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "3 cycles"]
        pub const _011: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wr_SPEC;
    pub type Wr = crate::EnumBitfieldStruct<u8, Wr_SPEC>;
    impl Wr {
        #[doc = "1 cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rai_SPEC;
    pub type Rai = crate::EnumBitfieldStruct<u8, Rai_SPEC>;
    impl Rai {
        #[doc = "1 cycle"]
        pub const _000: Self = Self::new(0);

        #[doc = "2 cycles"]
        pub const _001: Self = Self::new(1);

        #[doc = "3 cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "4 cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "5 cycles"]
        pub const _100: Self = Self::new(4);

        #[doc = "6 cycles"]
        pub const _101: Self = Self::new(5);

        #[doc = "7 cycles"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmod_SPEC;
impl crate::sealed::RegSpec for Sdmod_SPEC {
    type DataType = u16;
}

#[doc = "SDRAM Mode Register"]
pub type Sdmod = crate::RegValueT<Sdmod_SPEC>;

impl Sdmod {
    #[doc = "Mode Register SettingWriting to these bits: Mode register set command is issued."]
    #[inline(always)]
    pub fn mr(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Sdmod_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Sdmod_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Sdmod_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sdmod_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Sdmod {
    #[inline(always)]
    fn default() -> Sdmod {
        <crate::RegValueT<Sdmod_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdsr_SPEC;
impl crate::sealed::RegSpec for Sdsr_SPEC {
    type DataType = u8;
}

#[doc = "SDRAM Status Register"]
pub type Sdsr = crate::RegValueT<Sdsr_SPEC>;

impl Sdsr {
    #[doc = "Mode Register Setting Status"]
    #[inline(always)]
    pub fn mrsst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdsr::Mrsst,
        sdsr::Mrsst,
        Sdsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdsr::Mrsst,
            sdsr::Mrsst,
            Sdsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Initialization Status"]
    #[inline(always)]
    pub fn inist(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sdsr::Inist,
        sdsr::Inist,
        Sdsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sdsr::Inist,
            sdsr::Inist,
            Sdsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Self-Refresh Transition/Recovery Status"]
    #[inline(always)]
    pub fn srfst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdsr::Srfst,
        sdsr::Srfst,
        Sdsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdsr::Srfst,
            sdsr::Srfst,
            Sdsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Sdsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Sdsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdsr {
    #[inline(always)]
    fn default() -> Sdsr {
        <crate::RegValueT<Sdsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrsst_SPEC;
    pub type Mrsst = crate::EnumBitfieldStruct<u8, Mrsst_SPEC>;
    impl Mrsst {
        #[doc = "Mode register setting not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mode register setting in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inist_SPEC;
    pub type Inist = crate::EnumBitfieldStruct<u8, Inist_SPEC>;
    impl Inist {
        #[doc = "Initialization sequence not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initialization sequence in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srfst_SPEC;
    pub type Srfst = crate::EnumBitfieldStruct<u8, Srfst_SPEC>;
    impl Srfst {
        #[doc = "Transition/recovery not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transition/recovery in progress"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busoad_SPEC;
impl crate::sealed::RegSpec for Busoad_SPEC {
    type DataType = u16;
}

#[doc = "BUS_Operation_After_Detection Register"]
pub type Busoad = crate::RegValueT<Busoad_SPEC>;

impl Busoad {
    #[doc = "Illegal address access error operation after detection"]
    #[inline(always)]
    pub fn ilerroad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busoad::Ilerroad,
        busoad::Ilerroad,
        Busoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busoad::Ilerroad,
            busoad::Ilerroad,
            Busoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave bus error operation after detection"]
    #[inline(always)]
    pub fn slerroad(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        busoad::Slerroad,
        busoad::Slerroad,
        Busoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            busoad::Slerroad,
            busoad::Slerroad,
            Busoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bufferable write error operation after detection"]
    #[inline(always)]
    pub fn bwerroad(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        busoad::Bwerroad,
        busoad::Bwerroad,
        Busoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            busoad::Bwerroad,
            busoad::Bwerroad,
            Busoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1fff, 1, 0, u16, u16, Busoad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1fff,1,0,u16,u16,Busoad_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busoad {
    #[inline(always)]
    fn default() -> Busoad {
        <crate::RegValueT<Busoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerroad_SPEC;
    pub type Ilerroad = crate::EnumBitfieldStruct<u8, Ilerroad_SPEC>;
    impl Ilerroad {
        #[doc = "Only error response is returned"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerroad_SPEC;
    pub type Slerroad = crate::EnumBitfieldStruct<u8, Slerroad_SPEC>;
    impl Slerroad {
        #[doc = "Only error response is returned"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bwerroad_SPEC;
    pub type Bwerroad = crate::EnumBitfieldStruct<u8, Bwerroad_SPEC>;
    impl Bwerroad {
        #[doc = "NMI"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busoadpt_SPEC;
impl crate::sealed::RegSpec for Busoadpt_SPEC {
    type DataType = u16;
}

#[doc = "BUS_Operation_After_Detection Protect Register"]
pub type Busoadpt = crate::RegValueT<Busoadpt_SPEC>;

impl Busoadpt {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busoadpt::Protect,
        busoadpt::Protect,
        Busoadpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busoadpt::Protect,
            busoadpt::Protect,
            Busoadpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Busoadpt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Busoadpt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Key code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Busoadpt_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Busoadpt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Busoadpt {
    #[inline(always)]
    fn default() -> Busoadpt {
        <crate::RegValueT<Busoadpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busoadpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "BUSOAD register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "BUSOAD register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmabt_SPEC;
impl crate::sealed::RegSpec for Busmabt_SPEC {
    type DataType = u32;
}

#[doc = "Bus Master Arbitration Control Register"]
pub type Busmabt = crate::RegValueT<Busmabt_SPEC>;

impl Busmabt {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busmabt::Arbs,
        busmabt::Arbs,
        Busmabt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busmabt::Arbs,
            busmabt::Arbs,
            Busmabt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Busmabt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Busmabt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busmabt {
    #[inline(always)]
    fn default() -> Busmabt {
        <crate::RegValueT<Busmabt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmabt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt1Fhbi_SPEC;
impl crate::sealed::RegSpec for Bussabt1Fhbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 1 FHBI"]
pub type Bussabt1Fhbi = crate::RegValueT<Bussabt1Fhbi_SPEC>;

impl Bussabt1Fhbi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        bussabt1fhbi::Arbs,
        bussabt1fhbi::Arbs,
        Bussabt1Fhbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            bussabt1fhbi::Arbs,
            bussabt1fhbi::Arbs,
            Bussabt1Fhbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt1Fhbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt1Fhbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt1Fhbi {
    #[inline(always)]
    fn default() -> Bussabt1Fhbi {
        <crate::RegValueT<Bussabt1Fhbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt1fhbi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Combination of round-robin and fixed priority"]
        pub const _10: Self = Self::new(2);

        #[doc = "Round-robin"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Flbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Flbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0 FLBI"]
pub type Bussabt0Flbi = crate::RegValueT<Bussabt0Flbi_SPEC>;

impl Bussabt0Flbi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0flbi::Arbs,
        bussabt0flbi::Arbs,
        Bussabt0Flbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0flbi::Arbs,
            bussabt0flbi::Arbs,
            Bussabt0Flbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt0Flbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt0Flbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Flbi {
    #[inline(always)]
    fn default() -> Bussabt0Flbi {
        <crate::RegValueT<Bussabt0Flbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0flbi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt1S0Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt1S0Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 1 S0BI"]
pub type Bussabt1S0Bi = crate::RegValueT<Bussabt1S0Bi_SPEC>;

impl Bussabt1S0Bi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        bussabt1s0bi::Arbs,
        bussabt1s0bi::Arbs,
        Bussabt1S0Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            bussabt1s0bi::Arbs,
            bussabt1s0bi::Arbs,
            Bussabt1S0Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt1S0Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt1S0Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt1S0Bi {
    #[inline(always)]
    fn default() -> Bussabt1S0Bi {
        <crate::RegValueT<Bussabt1S0Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt1s0bi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Combination of round-robin and fixed priority"]
        pub const _10: Self = Self::new(2);

        #[doc = "Round-robin"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt1S1Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt1S1Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 1 S1BI"]
pub type Bussabt1S1Bi = crate::RegValueT<Bussabt1S1Bi_SPEC>;

impl Bussabt1S1Bi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        bussabt1s1bi::Arbs,
        bussabt1s1bi::Arbs,
        Bussabt1S1Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            bussabt1s1bi::Arbs,
            bussabt1s1bi::Arbs,
            Bussabt1S1Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt1S1Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt1S1Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt1S1Bi {
    #[inline(always)]
    fn default() -> Bussabt1S1Bi {
        <crate::RegValueT<Bussabt1S1Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt1s1bi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Combination of round-robin and fixed priority"]
        pub const _10: Self = Self::new(2);

        #[doc = "Round-robin"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Stbysbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Stbysbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0 STBYSBI"]
pub type Bussabt0Stbysbi = crate::RegValueT<Bussabt0Stbysbi_SPEC>;

impl Bussabt0Stbysbi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0stbysbi::Arbs,
        bussabt0stbysbi::Arbs,
        Bussabt0Stbysbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0stbysbi::Arbs,
            bussabt0stbysbi::Arbs,
            Bussabt0Stbysbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt0Stbysbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt0Stbysbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Stbysbi {
    #[inline(always)]
    fn default() -> Bussabt0Stbysbi {
        <crate::RegValueT<Bussabt0Stbysbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0stbysbi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt2Ecbi_SPEC;
impl crate::sealed::RegSpec for Bussabt2Ecbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 2 ECBI"]
pub type Bussabt2Ecbi = crate::RegValueT<Bussabt2Ecbi_SPEC>;

impl Bussabt2Ecbi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt2ecbi::Arbs0,
        bussabt2ecbi::Arbs0,
        Bussabt2Ecbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt2ecbi::Arbs0,
            bussabt2ecbi::Arbs0,
            Bussabt2Ecbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bussabt2ecbi::Arbs1,
        bussabt2ecbi::Arbs1,
        Bussabt2Ecbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bussabt2ecbi::Arbs1,
            bussabt2ecbi::Arbs1,
            Bussabt2Ecbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bussabt2ecbi::Arbs2,
        bussabt2ecbi::Arbs2,
        Bussabt2Ecbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bussabt2ecbi::Arbs2,
            bussabt2ecbi::Arbs2,
            Bussabt2Ecbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt2Ecbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt2Ecbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt2Ecbi {
    #[inline(always)]
    fn default() -> Bussabt2Ecbi {
        <crate::RegValueT<Bussabt2Ecbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt2ecbi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs0_SPEC;
    pub type Arbs0 = crate::EnumBitfieldStruct<u8, Arbs0_SPEC>;
    impl Arbs0 {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs1_SPEC;
    pub type Arbs1 = crate::EnumBitfieldStruct<u8, Arbs1_SPEC>;
    impl Arbs1 {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs2_SPEC;
    pub type Arbs2 = crate::EnumBitfieldStruct<u8, Arbs2_SPEC>;
    impl Arbs2 {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt2Eobi_SPEC;
impl crate::sealed::RegSpec for Bussabt2Eobi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 2 EOBI"]
pub type Bussabt2Eobi = crate::RegValueT<Bussabt2Eobi_SPEC>;

impl Bussabt2Eobi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt2eobi::Arbs0,
        bussabt2eobi::Arbs0,
        Bussabt2Eobi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt2eobi::Arbs0,
            bussabt2eobi::Arbs0,
            Bussabt2Eobi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bussabt2eobi::Arbs1,
        bussabt2eobi::Arbs1,
        Bussabt2Eobi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bussabt2eobi::Arbs1,
            bussabt2eobi::Arbs1,
            Bussabt2Eobi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bussabt2eobi::Arbs2,
        bussabt2eobi::Arbs2,
        Bussabt2Eobi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bussabt2eobi::Arbs2,
            bussabt2eobi::Arbs2,
            Bussabt2Eobi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt2Eobi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt2Eobi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt2Eobi {
    #[inline(always)]
    fn default() -> Bussabt2Eobi {
        <crate::RegValueT<Bussabt2Eobi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt2eobi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs0_SPEC;
    pub type Arbs0 = crate::EnumBitfieldStruct<u8, Arbs0_SPEC>;
    impl Arbs0 {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs1_SPEC;
    pub type Arbs1 = crate::EnumBitfieldStruct<u8, Arbs1_SPEC>;
    impl Arbs1 {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs2_SPEC;
    pub type Arbs2 = crate::EnumBitfieldStruct<u8, Arbs2_SPEC>;
    impl Arbs2 {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Pbbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pbbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0 PBBI"]
pub type Bussabt0Pbbi = crate::RegValueT<Bussabt0Pbbi_SPEC>;

impl Bussabt0Pbbi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0pbbi::Arbs,
        bussabt0pbbi::Arbs,
        Bussabt0Pbbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0pbbi::Arbs,
            bussabt0pbbi::Arbs,
            Bussabt0Pbbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt0Pbbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt0Pbbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Pbbi {
    #[inline(always)]
    fn default() -> Bussabt0Pbbi {
        <crate::RegValueT<Bussabt0Pbbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0pbbi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Pabi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pabi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0 PABI"]
pub type Bussabt0Pabi = crate::RegValueT<Bussabt0Pabi_SPEC>;

impl Bussabt0Pabi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0pabi::Arbs,
        bussabt0pabi::Arbs,
        Bussabt0Pabi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0pabi::Arbs,
            bussabt0pabi::Arbs,
            Bussabt0Pabi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt0Pabi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt0Pabi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Pabi {
    #[inline(always)]
    fn default() -> Bussabt0Pabi {
        <crate::RegValueT<Bussabt0Pabi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0pabi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Pibi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pibi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0 PIBI"]
pub type Bussabt0Pibi = crate::RegValueT<Bussabt0Pibi_SPEC>;

impl Bussabt0Pibi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0pibi::Arbs,
        bussabt0pibi::Arbs,
        Bussabt0Pibi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0pibi::Arbs,
            bussabt0pibi::Arbs,
            Bussabt0Pibi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt0Pibi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt0Pibi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Pibi {
    #[inline(always)]
    fn default() -> Bussabt0Pibi {
        <crate::RegValueT<Bussabt0Pibi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0pibi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Psbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Psbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0 PSBI"]
pub type Bussabt0Psbi = crate::RegValueT<Bussabt0Psbi_SPEC>;

impl Bussabt0Psbi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0psbi::Arbs,
        bussabt0psbi::Arbs,
        Bussabt0Psbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0psbi::Arbs,
            bussabt0psbi::Arbs,
            Bussabt0Psbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Bussabt0Psbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Bussabt0Psbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Psbi {
    #[inline(always)]
    fn default() -> Bussabt0Psbi {
        <crate::RegValueT<Bussabt0Psbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0psbi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "Fixed priority"]
        pub const _0: Self = Self::new(0);

        #[doc = "Round-robin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busdivbyp_SPEC;
impl crate::sealed::RegSpec for Busdivbyp_SPEC {
    type DataType = u32;
}

#[doc = "Bus Divider Bypass Register"]
pub type Busdivbyp = crate::RegValueT<Busdivbyp_SPEC>;

impl Busdivbyp {
    #[doc = "Divider for EDMACBI bypass enable"]
    #[inline(always)]
    pub fn edmabpe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busdivbyp::Edmabpe,
        busdivbyp::Edmabpe,
        Busdivbyp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busdivbyp::Edmabpe,
            busdivbyp::Edmabpe,
            Busdivbyp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Divider for GLCDC0BI bypass enable"]
    #[inline(always)]
    pub fn lcdc0bpe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        busdivbyp::Lcdc0Bpe,
        busdivbyp::Lcdc0Bpe,
        Busdivbyp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            busdivbyp::Lcdc0Bpe,
            busdivbyp::Lcdc0Bpe,
            Busdivbyp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Divider for GLCDC1BI bypass enable"]
    #[inline(always)]
    pub fn lcdc1bpe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        busdivbyp::Lcdc1Bpe,
        busdivbyp::Lcdc1Bpe,
        Busdivbyp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            busdivbyp::Lcdc1Bpe,
            busdivbyp::Lcdc1Bpe,
            Busdivbyp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Divider for GDSSBI bypass enable"]
    #[inline(always)]
    pub fn gdssbpe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        busdivbyp::Gdssbpe,
        busdivbyp::Gdssbpe,
        Busdivbyp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            busdivbyp::Gdssbpe,
            busdivbyp::Gdssbpe,
            Busdivbyp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Divider for CPUSAHBI bypass enable"]
    #[inline(always)]
    pub fn cpu0sbpe(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        busdivbyp::Cpu0Sbpe,
        busdivbyp::Cpu0Sbpe,
        Busdivbyp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            busdivbyp::Cpu0Sbpe,
            busdivbyp::Cpu0Sbpe,
            Busdivbyp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, Busdivbyp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,Busdivbyp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busdivbyp {
    #[inline(always)]
    fn default() -> Busdivbyp {
        <crate::RegValueT<Busdivbyp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busdivbyp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edmabpe_SPEC;
    pub type Edmabpe = crate::EnumBitfieldStruct<u8, Edmabpe_SPEC>;
    impl Edmabpe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdc0Bpe_SPEC;
    pub type Lcdc0Bpe = crate::EnumBitfieldStruct<u8, Lcdc0Bpe_SPEC>;
    impl Lcdc0Bpe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdc1Bpe_SPEC;
    pub type Lcdc1Bpe = crate::EnumBitfieldStruct<u8, Lcdc1Bpe_SPEC>;
    impl Lcdc1Bpe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gdssbpe_SPEC;
    pub type Gdssbpe = crate::EnumBitfieldStruct<u8, Gdssbpe_SPEC>;
    impl Gdssbpe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpu0Sbpe_SPEC;
    pub type Cpu0Sbpe = crate::EnumBitfieldStruct<u8, Cpu0Sbpe_SPEC>;
    impl Cpu0Sbpe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qospri_SPEC;
impl crate::sealed::RegSpec for Qospri_SPEC {
    type DataType = u32;
}

#[doc = "QoS Priority Setting After Reach Threshold"]
pub type Qospri = crate::RegValueT<Qospri_SPEC>;

impl Qospri {
    #[inline(always)]
    pub fn qplcd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        qospri::Qplcd0,
        qospri::Qplcd0,
        Qospri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            qospri::Qplcd0,
            qospri::Qplcd0,
            Qospri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qplcd1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        qospri::Qplcd1,
        qospri::Qplcd1,
        Qospri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            qospri::Qplcd1,
            qospri::Qplcd1,
            Qospri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qpdrw0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        qospri::Qpdrw0,
        qospri::Qpdrw0,
        Qospri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            qospri::Qpdrw0,
            qospri::Qpdrw0,
            Qospri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qpdrw1(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        qospri::Qpdrw1,
        qospri::Qpdrw1,
        Qospri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            qospri::Qpdrw1,
            qospri::Qpdrw1,
            Qospri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qpceu(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        qospri::Qpceu,
        qospri::Qpceu,
        Qospri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            qospri::Qpceu,
            qospri::Qpceu,
            Qospri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Qospri_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Qospri_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qospri {
    #[inline(always)]
    fn default() -> Qospri {
        <crate::RegValueT<Qospri_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qospri {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qplcd0_SPEC;
    pub type Qplcd0 = crate::EnumBitfieldStruct<u8, Qplcd0_SPEC>;
    impl Qplcd0 {
        #[doc = "Priority LOW"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority MIDDLE"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qplcd1_SPEC;
    pub type Qplcd1 = crate::EnumBitfieldStruct<u8, Qplcd1_SPEC>;
    impl Qplcd1 {
        #[doc = "Priority LOW"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority MIDDLE"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qpdrw0_SPEC;
    pub type Qpdrw0 = crate::EnumBitfieldStruct<u8, Qpdrw0_SPEC>;
    impl Qpdrw0 {
        #[doc = "Priority LOW"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority MIDDLE"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qpdrw1_SPEC;
    pub type Qpdrw1 = crate::EnumBitfieldStruct<u8, Qpdrw1_SPEC>;
    impl Qpdrw1 {
        #[doc = "Priority LOW"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority MIDDLE"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qpceu_SPEC;
    pub type Qpceu = crate::EnumBitfieldStruct<u8, Qpceu_SPEC>;
    impl Qpceu {
        #[doc = "Priority LOW"]
        pub const _0: Self = Self::new(0);

        #[doc = "Priority MIDDLE"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qoscyc_SPEC;
impl crate::sealed::RegSpec for Qoscyc_SPEC {
    type DataType = u32;
}

#[doc = "QoS Cycle Setting Register %s"]
pub type Qoscyc = crate::RegValueT<Qoscyc_SPEC>;

impl Qoscyc {
    #[doc = "QoS Unit Cycle Setting"]
    #[inline(always)]
    pub fn qoscyc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        qoscyc::Qoscyc,
        qoscyc::Qoscyc,
        Qoscyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            qoscyc::Qoscyc,
            qoscyc::Qoscyc,
            Qoscyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Qoscyc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Qoscyc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qoscyc {
    #[inline(always)]
    fn default() -> Qoscyc {
        <crate::RegValueT<Qoscyc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qoscyc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qoscyc_SPEC;
    pub type Qoscyc = crate::EnumBitfieldStruct<u16, Qoscyc_SPEC>;
    impl Qoscyc {
        #[doc = "QoS circuit is disabled."]
        pub const _0000000000000000: Self = Self::new(0);

        #[doc = "QoS unit cycle is 16"]
        pub const _0000000000000001: Self = Self::new(1);

        #[doc = "QoS unit cycle is 32"]
        pub const _0000000000000010: Self = Self::new(2);

        #[doc = "QoS unit cycle is 48"]
        pub const _0000000000000011: Self = Self::new(3);

        #[doc = "QoS unit cycle is 1,048,560"]
        pub const _1111111111111111: Self = Self::new(65535);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmpri_SPEC;
impl crate::sealed::RegSpec for Busmpri_SPEC {
    type DataType = u32;
}

#[doc = "Bus Master priority setting register"]
pub type Busmpri = crate::RegValueT<Busmpri_SPEC>;

impl Busmpri {
    #[doc = "Priority Select"]
    #[inline(always)]
    pub fn pri(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        busmpri::Pri,
        busmpri::Pri,
        Busmpri_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            busmpri::Pri,
            busmpri::Pri,
            Busmpri_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Busmpri_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Busmpri_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busmpri {
    #[inline(always)]
    fn default() -> Busmpri {
        <crate::RegValueT<Busmpri_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmpri {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pri_SPEC;
    pub type Pri = crate::EnumBitfieldStruct<u8, Pri_SPEC>;
    impl Pri {
        #[doc = "Priority of MIPI is MIDDLE"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Priority of MIPI is LOW"]
        pub const _1111: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qoscyc4_SPEC;
impl crate::sealed::RegSpec for Qoscyc4_SPEC {
    type DataType = u32;
}

#[doc = "QoS Cycle Setting Register 4"]
pub type Qoscyc4 = crate::RegValueT<Qoscyc4_SPEC>;

impl Qoscyc4 {
    #[doc = "QoS Unit Cycle Setting"]
    #[inline(always)]
    pub fn qoscyc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        qoscyc4::Qoscyc,
        qoscyc4::Qoscyc,
        Qoscyc4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            qoscyc4::Qoscyc,
            qoscyc4::Qoscyc,
            Qoscyc4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Qoscyc4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Qoscyc4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qoscyc4 {
    #[inline(always)]
    fn default() -> Qoscyc4 {
        <crate::RegValueT<Qoscyc4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qoscyc4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qoscyc_SPEC;
    pub type Qoscyc = crate::EnumBitfieldStruct<u16, Qoscyc_SPEC>;
    impl Qoscyc {
        #[doc = "QoS circuit is disabled."]
        pub const _0000000000000000: Self = Self::new(0);

        #[doc = "QoS unit cycle is 16"]
        pub const _0000000000000001: Self = Self::new(1);

        #[doc = "QoS unit cycle is 32"]
        pub const _0000000000000010: Self = Self::new(2);

        #[doc = "QoS unit cycle is 48"]
        pub const _0000000000000011: Self = Self::new(3);

        #[doc = "QoS unit cycle is 1,048,560"]
        pub const _1111111111111111: Self = Self::new(65535);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qosthd_SPEC;
impl crate::sealed::RegSpec for Qosthd_SPEC {
    type DataType = u32;
}

#[doc = "QoS Threshold Setting Register %s"]
pub type Qosthd = crate::RegValueT<Qosthd_SPEC>;

impl Qosthd {
    #[doc = "QoS Threshold Setting"]
    #[inline(always)]
    pub fn qosthd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        qosthd::Qosthd,
        qosthd::Qosthd,
        Qosthd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            qosthd::Qosthd,
            qosthd::Qosthd,
            Qosthd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Qosthd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Qosthd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qosthd {
    #[inline(always)]
    fn default() -> Qosthd {
        <crate::RegValueT<Qosthd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qosthd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qosthd_SPEC;
    pub type Qosthd = crate::EnumBitfieldStruct<u16, Qosthd_SPEC>;
    impl Qosthd {
        #[doc = "Threshold is 0 byte"]
        pub const _0000000000000000: Self = Self::new(0);

        #[doc = "Threshold is 1 byte"]
        pub const _0000000000000001: Self = Self::new(1);

        #[doc = "Threshold is 2 bytes"]
        pub const _0000000000000010: Self = Self::new(2);

        #[doc = "Threshold is 3 bytes"]
        pub const _0000000000000011: Self = Self::new(3);

        #[doc = "Threshold is 65,535 bytes"]
        pub const _1111111111111111: Self = Self::new(65535);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qosthd4_SPEC;
impl crate::sealed::RegSpec for Qosthd4_SPEC {
    type DataType = u32;
}

#[doc = "QoS Threshold Setting Register 4"]
pub type Qosthd4 = crate::RegValueT<Qosthd4_SPEC>;

impl Qosthd4 {
    #[doc = "QoS Threshold Setting"]
    #[inline(always)]
    pub fn qosthd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        qosthd4::Qosthd,
        qosthd4::Qosthd,
        Qosthd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            qosthd4::Qosthd,
            qosthd4::Qosthd,
            Qosthd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Qosthd4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Qosthd4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qosthd4 {
    #[inline(always)]
    fn default() -> Qosthd4 {
        <crate::RegValueT<Qosthd4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qosthd4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qosthd_SPEC;
    pub type Qosthd = crate::EnumBitfieldStruct<u16, Qosthd_SPEC>;
    impl Qosthd {
        #[doc = "Threshold is 0 byte"]
        pub const _0000000000000000: Self = Self::new(0);

        #[doc = "Threshold is 1 byte"]
        pub const _0000000000000001: Self = Self::new(1);

        #[doc = "Threshold is 2 bytes"]
        pub const _0000000000000010: Self = Self::new(2);

        #[doc = "Threshold is 3 bytes"]
        pub const _0000000000000011: Self = Self::new(3);

        #[doc = "Threshold is 65,535 bytes"]
        pub const _1111111111111111: Self = Self::new(65535);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qosdmon_SPEC;
impl crate::sealed::RegSpec for Qosdmon_SPEC {
    type DataType = u32;
}

#[doc = "QoS Data Transferred Amount Monitor Register %s"]
pub type Qosdmon = crate::RegValueT<Qosdmon_SPEC>;

impl Qosdmon {
    #[doc = "QoS data transferred amount within a unit cycle"]
    #[inline(always)]
    pub fn qosdmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        qosdmon::Qosdmon,
        qosdmon::Qosdmon,
        Qosdmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            qosdmon::Qosdmon,
            qosdmon::Qosdmon,
            Qosdmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Qosdmon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Qosdmon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qosdmon {
    #[inline(always)]
    fn default() -> Qosdmon {
        <crate::RegValueT<Qosdmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qosdmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qosdmon_SPEC;
    pub type Qosdmon = crate::EnumBitfieldStruct<u16, Qosdmon_SPEC>;
    impl Qosdmon {
        #[doc = "0 byte"]
        pub const _0000000000000000: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _0000000000000001: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _0000000000000010: Self = Self::new(2);

        #[doc = "3 bytes"]
        pub const _0000000000000011: Self = Self::new(3);

        #[doc = "65,535 or more bytes."]
        pub const _1111111111111111: Self = Self::new(65535);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qosdmon4_SPEC;
impl crate::sealed::RegSpec for Qosdmon4_SPEC {
    type DataType = u32;
}

#[doc = "QoS Data Transferred Amount Monitor Register 4"]
pub type Qosdmon4 = crate::RegValueT<Qosdmon4_SPEC>;

impl Qosdmon4 {
    #[doc = "QoS data transferred amount within a unit cycle"]
    #[inline(always)]
    pub fn qosdmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        qosdmon4::Qosdmon,
        qosdmon4::Qosdmon,
        Qosdmon4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            qosdmon4::Qosdmon,
            qosdmon4::Qosdmon,
            Qosdmon4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Qosdmon4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Qosdmon4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qosdmon4 {
    #[inline(always)]
    fn default() -> Qosdmon4 {
        <crate::RegValueT<Qosdmon4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qosdmon4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qosdmon_SPEC;
    pub type Qosdmon = crate::EnumBitfieldStruct<u16, Qosdmon_SPEC>;
    impl Qosdmon {
        #[doc = "0 byte"]
        pub const _0000000000000000: Self = Self::new(0);

        #[doc = "1 byte"]
        pub const _0000000000000001: Self = Self::new(1);

        #[doc = "2 bytes"]
        pub const _0000000000000010: Self = Self::new(2);

        #[doc = "3 bytes"]
        pub const _0000000000000011: Self = Self::new(3);

        #[doc = "65,535 or more bytes."]
        pub const _1111111111111111: Self = Self::new(65535);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserradd_SPEC;
impl crate::sealed::RegSpec for Buserradd_SPEC {
    type DataType = u32;
}

#[doc = "Bus Error Address Register %s"]
pub type Buserradd = crate::RegValueT<Buserradd_SPEC>;

impl Buserradd {
    #[doc = "Bus Error AddressWhen a bus error occurs, It stores an error address."]
    #[inline(always)]
    pub fn berad(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Buserradd_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Buserradd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserradd {
    #[inline(always)]
    fn default() -> Buserradd {
        <crate::RegValueT<Buserradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrrw_SPEC;
impl crate::sealed::RegSpec for Buserrrw_SPEC {
    type DataType = u8;
}

#[doc = "Bus Error RW Register %s"]
pub type Buserrrw = crate::RegValueT<Buserrrw_SPEC>;

impl Buserrrw {
    #[doc = "error access Read/Write STAtus"]
    #[inline(always)]
    pub fn rwstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrrw::Rwstat,
        buserrrw::Rwstat,
        Buserrrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrrw::Rwstat,
            buserrrw::Rwstat,
            Buserrrw_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Buserrrw_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Buserrrw_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Buserrrw {
    #[inline(always)]
    fn default() -> Buserrrw {
        <crate::RegValueT<Buserrrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwstat_SPEC;
    pub type Rwstat = crate::EnumBitfieldStruct<u8, Rwstat_SPEC>;
    impl Rwstat {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write access"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btzferradd_SPEC;
impl crate::sealed::RegSpec for Btzferradd_SPEC {
    type DataType = u32;
}

#[doc = "Bus TZF Error Address Register %s"]
pub type Btzferradd = crate::RegValueT<Btzferradd_SPEC>;

impl Btzferradd {
    #[doc = "Bus TZF Error Adress When a TZF error occurs, It stores an error address."]
    #[inline(always)]
    pub fn btzferad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Btzferradd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Btzferradd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Btzferradd {
    #[inline(always)]
    fn default() -> Btzferradd {
        <crate::RegValueT<Btzferradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btzferrrw_SPEC;
impl crate::sealed::RegSpec for Btzferrrw_SPEC {
    type DataType = u8;
}

#[doc = "Bus ZTF Error RW Register %s"]
pub type Btzferrrw = crate::RegValueT<Btzferrrw_SPEC>;

impl Btzferrrw {
    #[doc = "TrustZone filter error access Read/Write STAtus"]
    #[inline(always)]
    pub fn trwstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        btzferrrw::Trwstat,
        btzferrrw::Trwstat,
        Btzferrrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            btzferrrw::Trwstat,
            btzferrrw::Trwstat,
            Btzferrrw_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Btzferrrw_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Btzferrrw_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Btzferrrw {
    #[inline(always)]
    fn default() -> Btzferrrw {
        <crate::RegValueT<Btzferrrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod btzferrrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trwstat_SPEC;
    pub type Trwstat = crate::EnumBitfieldStruct<u8, Trwstat_SPEC>;
    impl Trwstat {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write access"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstat_SPEC;
impl crate::sealed::RegSpec for Buserrstat_SPEC {
    type DataType = u8;
}

#[doc = "Bus Error Status Register %s"]
pub type Buserrstat = crate::RegValueT<Buserrstat_SPEC>;

impl Buserrstat {
    #[doc = "Slave bus ERRor STATus"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstat::Slerrstat,
        buserrstat::Slerrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstat::Slerrstat,
            buserrstat::Slerrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave TrustZone filter ERRor STATus"]
    #[inline(always)]
    pub fn sterrstat(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        buserrstat::Sterrstat,
        buserrstat::Sterrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            buserrstat::Sterrstat,
            buserrstat::Sterrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU ERRor STATus"]
    #[inline(always)]
    pub fn mmerrstat(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrstat::Mmerrstat,
        buserrstat::Mmerrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstat::Mmerrstat,
            buserrstat::Mmerrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Illegal address access ERRor STATus"]
    #[inline(always)]
    pub fn ilerrstat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrstat::Ilerrstat,
        buserrstat::Ilerrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstat::Ilerrstat,
            buserrstat::Ilerrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Security attribution unit ERRor STATus"]
    #[inline(always)]
    pub fn mserrstat(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrstat::Mserrstat,
        buserrstat::Mserrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstat::Mserrstat,
            buserrstat::Mserrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Buserrstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Buserrstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Buserrstat {
    #[inline(always)]
    fn default() -> Buserrstat {
        <crate::RegValueT<Buserrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrstat_SPEC;
    pub type Slerrstat = crate::EnumBitfieldStruct<u8, Slerrstat_SPEC>;
    impl Slerrstat {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sterrstat_SPEC;
    pub type Sterrstat = crate::EnumBitfieldStruct<u8, Sterrstat_SPEC>;
    impl Sterrstat {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrstat_SPEC;
    pub type Mmerrstat = crate::EnumBitfieldStruct<u8, Mmerrstat_SPEC>;
    impl Mmerrstat {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrstat_SPEC;
    pub type Ilerrstat = crate::EnumBitfieldStruct<u8, Ilerrstat_SPEC>;
    impl Ilerrstat {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrstat_SPEC;
    pub type Mserrstat = crate::EnumBitfieldStruct<u8, Mserrstat_SPEC>;
    impl Mserrstat {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrclr_SPEC;
impl crate::sealed::RegSpec for Buserrclr_SPEC {
    type DataType = u8;
}

#[doc = "Bus Error Status Register %s"]
pub type Buserrclr = crate::RegValueT<Buserrclr_SPEC>;

impl Buserrclr {
    #[doc = "Slave bus ERRor CleaR"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave TrustZone filter ERRor CleaR"]
    #[inline(always)]
    pub fn sterrclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master MPU ERRor CleaR"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Illegal address access ERRor CleaR"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Security attribution unit ERRor CleaR"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Buserrclr {
    #[inline(always)]
    fn default() -> Buserrclr {
        <crate::RegValueT<Buserrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbwerrstat_SPEC;
impl crate::sealed::RegSpec for Mbwerrstat_SPEC {
    type DataType = u32;
}

#[doc = "Master Bufferable Write Error Status Register"]
pub type Mbwerrstat = crate::RegValueT<Mbwerrstat_SPEC>;

impl Mbwerrstat {
    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr0,
        mbwerrstat::Mbwerr0,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr0,
            mbwerrstat::Mbwerr0,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr1,
        mbwerrstat::Mbwerr1,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr1,
            mbwerrstat::Mbwerr1,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr8,
        mbwerrstat::Mbwerr8,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr8,
            mbwerrstat::Mbwerr8,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr16,
        mbwerrstat::Mbwerr16,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr16,
            mbwerrstat::Mbwerr16,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr17,
        mbwerrstat::Mbwerr17,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr17,
            mbwerrstat::Mbwerr17,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr18,
        mbwerrstat::Mbwerr18,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr18,
            mbwerrstat::Mbwerr18,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr19,
        mbwerrstat::Mbwerr19,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr19,
            mbwerrstat::Mbwerr19,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr20,
        mbwerrstat::Mbwerr20,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr20,
            mbwerrstat::Mbwerr20,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr21,
        mbwerrstat::Mbwerr21,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr21,
            mbwerrstat::Mbwerr21,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr22,
        mbwerrstat::Mbwerr22,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr22,
            mbwerrstat::Mbwerr22,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error"]
    #[inline(always)]
    pub fn mbwerr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr23,
        mbwerrstat::Mbwerr23,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr23,
            mbwerrstat::Mbwerr23,
            Mbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Mbwerrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Mbwerrstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mbwerrstat {
    #[inline(always)]
    fn default() -> Mbwerrstat {
        <crate::RegValueT<Mbwerrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mbwerrstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr0_SPEC;
    pub type Mbwerr0 = crate::EnumBitfieldStruct<u8, Mbwerr0_SPEC>;
    impl Mbwerr0 {
        #[doc = "No bufferable write error in Master #0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr1_SPEC;
    pub type Mbwerr1 = crate::EnumBitfieldStruct<u8, Mbwerr1_SPEC>;
    impl Mbwerr1 {
        #[doc = "No bufferable write error in Master #1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr8_SPEC;
    pub type Mbwerr8 = crate::EnumBitfieldStruct<u8, Mbwerr8_SPEC>;
    impl Mbwerr8 {
        #[doc = "No bufferable write error in Master #8"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #8"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr16_SPEC;
    pub type Mbwerr16 = crate::EnumBitfieldStruct<u8, Mbwerr16_SPEC>;
    impl Mbwerr16 {
        #[doc = "No bufferable write error in Master #16"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #16"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr17_SPEC;
    pub type Mbwerr17 = crate::EnumBitfieldStruct<u8, Mbwerr17_SPEC>;
    impl Mbwerr17 {
        #[doc = "No bufferable write error in Master #17"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #17"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr18_SPEC;
    pub type Mbwerr18 = crate::EnumBitfieldStruct<u8, Mbwerr18_SPEC>;
    impl Mbwerr18 {
        #[doc = "No bufferable write error in Master #18"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #18"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr19_SPEC;
    pub type Mbwerr19 = crate::EnumBitfieldStruct<u8, Mbwerr19_SPEC>;
    impl Mbwerr19 {
        #[doc = "No bufferable write error in Master #19"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #19"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr20_SPEC;
    pub type Mbwerr20 = crate::EnumBitfieldStruct<u8, Mbwerr20_SPEC>;
    impl Mbwerr20 {
        #[doc = "No bufferable write error in Master #20"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #20"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr21_SPEC;
    pub type Mbwerr21 = crate::EnumBitfieldStruct<u8, Mbwerr21_SPEC>;
    impl Mbwerr21 {
        #[doc = "No bufferable write error in Master #21"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #21"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr22_SPEC;
    pub type Mbwerr22 = crate::EnumBitfieldStruct<u8, Mbwerr22_SPEC>;
    impl Mbwerr22 {
        #[doc = "No bufferable write error in Master #22"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #22"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr23_SPEC;
    pub type Mbwerr23 = crate::EnumBitfieldStruct<u8, Mbwerr23_SPEC>;
    impl Mbwerr23 {
        #[doc = "No bufferable write error in Master #23"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #23"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbwerrclr_SPEC;
impl crate::sealed::RegSpec for Mbwerrclr_SPEC {
    type DataType = u32;
}

#[doc = "Master Bufferable Write Error Clear Register"]
pub type Mbwerrclr = crate::RegValueT<Mbwerrclr_SPEC>;

impl Mbwerrclr {
    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Mbwerrclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mbwerrclr {
    #[inline(always)]
    fn default() -> Mbwerrclr {
        <crate::RegValueT<Mbwerrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbwerrstat_SPEC;
impl crate::sealed::RegSpec for Sbwerrstat_SPEC {
    type DataType = u32;
}

#[doc = "Slave Bufferable Write Error Status Register"]
pub type Sbwerrstat = crate::RegValueT<Sbwerrstat_SPEC>;

impl Sbwerrstat {
    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr0,
        sbwerrstat::Sbwerr0,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr0,
            sbwerrstat::Sbwerr0,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr1,
        sbwerrstat::Sbwerr1,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr1,
            sbwerrstat::Sbwerr1,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr2,
        sbwerrstat::Sbwerr2,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr2,
            sbwerrstat::Sbwerr2,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr3,
        sbwerrstat::Sbwerr3,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr3,
            sbwerrstat::Sbwerr3,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr4,
        sbwerrstat::Sbwerr4,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr4,
            sbwerrstat::Sbwerr4,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr5,
        sbwerrstat::Sbwerr5,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr5,
            sbwerrstat::Sbwerr5,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr6,
        sbwerrstat::Sbwerr6,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr6,
            sbwerrstat::Sbwerr6,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr7,
        sbwerrstat::Sbwerr7,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr7,
            sbwerrstat::Sbwerr7,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr8,
        sbwerrstat::Sbwerr8,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr8,
            sbwerrstat::Sbwerr8,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr9,
        sbwerrstat::Sbwerr9,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr9,
            sbwerrstat::Sbwerr9,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr10,
        sbwerrstat::Sbwerr10,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr10,
            sbwerrstat::Sbwerr10,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr11,
        sbwerrstat::Sbwerr11,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr11,
            sbwerrstat::Sbwerr11,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Slave Bufferable Write Error"]
    #[inline(always)]
    pub fn sbwerr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sbwerrstat::Sbwerr12,
        sbwerrstat::Sbwerr12,
        Sbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sbwerrstat::Sbwerr12,
            sbwerrstat::Sbwerr12,
            Sbwerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Sbwerrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Sbwerrstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sbwerrstat {
    #[inline(always)]
    fn default() -> Sbwerrstat {
        <crate::RegValueT<Sbwerrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sbwerrstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr0_SPEC;
    pub type Sbwerr0 = crate::EnumBitfieldStruct<u8, Sbwerr0_SPEC>;
    impl Sbwerr0 {
        #[doc = "No bufferable write error in Slave #0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr1_SPEC;
    pub type Sbwerr1 = crate::EnumBitfieldStruct<u8, Sbwerr1_SPEC>;
    impl Sbwerr1 {
        #[doc = "No bufferable write error in Slave #1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr2_SPEC;
    pub type Sbwerr2 = crate::EnumBitfieldStruct<u8, Sbwerr2_SPEC>;
    impl Sbwerr2 {
        #[doc = "No bufferable write error in Slave #2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr3_SPEC;
    pub type Sbwerr3 = crate::EnumBitfieldStruct<u8, Sbwerr3_SPEC>;
    impl Sbwerr3 {
        #[doc = "No bufferable write error in Slave #3"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #3"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr4_SPEC;
    pub type Sbwerr4 = crate::EnumBitfieldStruct<u8, Sbwerr4_SPEC>;
    impl Sbwerr4 {
        #[doc = "No bufferable write error in Slave #4"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #4"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr5_SPEC;
    pub type Sbwerr5 = crate::EnumBitfieldStruct<u8, Sbwerr5_SPEC>;
    impl Sbwerr5 {
        #[doc = "No bufferable write error in Slave #5"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #5"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr6_SPEC;
    pub type Sbwerr6 = crate::EnumBitfieldStruct<u8, Sbwerr6_SPEC>;
    impl Sbwerr6 {
        #[doc = "No bufferable write error in Slave #6"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #6"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr7_SPEC;
    pub type Sbwerr7 = crate::EnumBitfieldStruct<u8, Sbwerr7_SPEC>;
    impl Sbwerr7 {
        #[doc = "No bufferable write error in Slave #7"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #7"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr8_SPEC;
    pub type Sbwerr8 = crate::EnumBitfieldStruct<u8, Sbwerr8_SPEC>;
    impl Sbwerr8 {
        #[doc = "No bufferable write error in Slave #8"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #8"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr9_SPEC;
    pub type Sbwerr9 = crate::EnumBitfieldStruct<u8, Sbwerr9_SPEC>;
    impl Sbwerr9 {
        #[doc = "No bufferable write error in Slave #9"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #9"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr10_SPEC;
    pub type Sbwerr10 = crate::EnumBitfieldStruct<u8, Sbwerr10_SPEC>;
    impl Sbwerr10 {
        #[doc = "No bufferable write error in Slave #10"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #10"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr11_SPEC;
    pub type Sbwerr11 = crate::EnumBitfieldStruct<u8, Sbwerr11_SPEC>;
    impl Sbwerr11 {
        #[doc = "No bufferable write error in Slave #11"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #11"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr12_SPEC;
    pub type Sbwerr12 = crate::EnumBitfieldStruct<u8, Sbwerr12_SPEC>;
    impl Sbwerr12 {
        #[doc = "No bufferable write error in Slave #12"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Slave #12"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbwerrclr_SPEC;
impl crate::sealed::RegSpec for Sbwerrclr_SPEC {
    type DataType = u32;
}

#[doc = "Slave Bufferable Write Error Clear Register"]
pub type Sbwerrclr = crate::RegValueT<Sbwerrclr_SPEC>;

impl Sbwerrclr {
    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn sbweclr12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Sbwerrclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sbwerrclr {
    #[inline(always)]
    fn default() -> Sbwerrclr {
        <crate::RegValueT<Sbwerrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
