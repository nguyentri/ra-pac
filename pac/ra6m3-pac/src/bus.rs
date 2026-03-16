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
#[doc = r"BUS Control"]
unsafe impl ::core::marker::Send for super::Bus {}
unsafe impl ::core::marker::Sync for super::Bus {}
impl super::Bus {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
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

    #[doc = "Bus Error Address Register %s"]
    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        11,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1800usize))
        }
    }
    #[inline(always)]
    pub const fn bus1erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1800usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus2erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1810usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus3erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1820usize),
            )
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
    #[inline(always)]
    pub const fn bus10erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1890usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus11erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Buserradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x18a0usize),
            )
        }
    }

    #[doc = "Bus Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        11,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1804usize))
        }
    }
    #[inline(always)]
    pub const fn bus1errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus2errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus3errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus4errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus5errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus6errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1854usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus7errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1864usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus8errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1874usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus9errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1884usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus10errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1894usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus11errstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x18a4usize),
            )
        }
    }

    #[doc = "Master Bus Control Register SYS"]
    #[inline(always)]
    pub const fn busmcntsys(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcntsys_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcntsys_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4104usize),
            )
        }
    }

    #[doc = "Master Bus Control Register DMA"]
    #[inline(always)]
    pub const fn busmcntdma(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcntdma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcntdma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4108usize),
            )
        }
    }

    #[doc = "Master Bus Control Register %s"]
    #[inline(always)]
    pub const fn busmcnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1010usize))
        }
    }
    #[inline(always)]
    pub const fn busmcntedm(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1010usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busmcntgpx(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1014usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register MBIU"]
    #[inline(always)]
    pub const fn busscntmbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntmbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntmbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4360usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register %s"]
    #[inline(always)]
    pub const fn busscnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Busscnt_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1130usize))
        }
    }
    #[inline(always)]
    pub const fn busscntfbu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busscntext(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busscntext2(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busscntgpx(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x113cusize),
            )
        }
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
}
impl ::core::default::Default for Cs0Cr {
    #[inline(always)]
    fn default() -> Cs0Cr {
        <crate::RegValueT<Cs0Cr_SPEC> as RegisterValue<_>>::new(33)
    }
}
pub mod cs0cr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpxen_SPEC;
    pub type Mpxen = crate::EnumBitfieldStruct<u8, Mpxen_SPEC>;
    impl Mpxen {
        #[doc = "Separate bus interface is selected for area n"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        #[doc = "Little Endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big Endian"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        #[doc = "A 16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "An 8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disable operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable operation"]
        pub const _1: Self = Self::new(1);
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
}
impl ::core::default::Default for Cscr {
    #[inline(always)]
    fn default() -> Cscr {
        <crate::RegValueT<Cscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpxen_SPEC;
    pub type Mpxen = crate::EnumBitfieldStruct<u8, Mpxen_SPEC>;
    impl Mpxen {
        #[doc = "Separate bus interface is selected for area n"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area n. (n = 0 to 7)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        #[doc = "Little Endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big Endian"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        #[doc = "A 16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "An 8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disable operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable operation"]
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
}
impl ::core::default::Default for Csrec {
    #[inline(always)]
    fn default() -> Csrec {
        <crate::RegValueT<Csrec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod csrec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrcv_SPEC;
    pub type Wrcv = crate::EnumBitfieldStruct<u8, Wrcv_SPEC>;
    impl Wrcv {
        #[doc = "No recovery cycle is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrcv_SPEC;
    pub type Rrcv = crate::EnumBitfieldStruct<u8, Rrcv_SPEC>;
    impl Rrcv {
        #[doc = "No recovery cycle is inserted."]
        pub const _0_X_0: Self = Self::new(0);
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
}
impl ::core::default::Default for Csrecen {
    #[inline(always)]
    fn default() -> Csrecen {
        <crate::RegValueT<Csrecen_SPEC> as RegisterValue<_>>::new(15934)
    }
}
pub mod csrecen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm7_SPEC;
    pub type Rcvenm7 = crate::EnumBitfieldStruct<u8, Rcvenm7_SPEC>;
    impl Rcvenm7 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm6_SPEC;
    pub type Rcvenm6 = crate::EnumBitfieldStruct<u8, Rcvenm6_SPEC>;
    impl Rcvenm6 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm5_SPEC;
    pub type Rcvenm5 = crate::EnumBitfieldStruct<u8, Rcvenm5_SPEC>;
    impl Rcvenm5 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm4_SPEC;
    pub type Rcvenm4 = crate::EnumBitfieldStruct<u8, Rcvenm4_SPEC>;
    impl Rcvenm4 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm3_SPEC;
    pub type Rcvenm3 = crate::EnumBitfieldStruct<u8, Rcvenm3_SPEC>;
    impl Rcvenm3 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm2_SPEC;
    pub type Rcvenm2 = crate::EnumBitfieldStruct<u8, Rcvenm2_SPEC>;
    impl Rcvenm2 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm1_SPEC;
    pub type Rcvenm1 = crate::EnumBitfieldStruct<u8, Rcvenm1_SPEC>;
    impl Rcvenm1 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm0_SPEC;
    pub type Rcvenm0 = crate::EnumBitfieldStruct<u8, Rcvenm0_SPEC>;
    impl Rcvenm0 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven7_SPEC;
    pub type Rcven7 = crate::EnumBitfieldStruct<u8, Rcven7_SPEC>;
    impl Rcven7 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven6_SPEC;
    pub type Rcven6 = crate::EnumBitfieldStruct<u8, Rcven6_SPEC>;
    impl Rcven6 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven5_SPEC;
    pub type Rcven5 = crate::EnumBitfieldStruct<u8, Rcven5_SPEC>;
    impl Rcven5 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven4_SPEC;
    pub type Rcven4 = crate::EnumBitfieldStruct<u8, Rcven4_SPEC>;
    impl Rcven4 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven3_SPEC;
    pub type Rcven3 = crate::EnumBitfieldStruct<u8, Rcven3_SPEC>;
    impl Rcven3 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven2_SPEC;
    pub type Rcven2 = crate::EnumBitfieldStruct<u8, Rcven2_SPEC>;
    impl Rcven2 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven1_SPEC;
    pub type Rcven1 = crate::EnumBitfieldStruct<u8, Rcven1_SPEC>;
    impl Rcven1 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcven0_SPEC;
    pub type Rcven0 = crate::EnumBitfieldStruct<u8, Rcven0_SPEC>;
    impl Rcven0 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
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
}
impl ::core::default::Default for Csmod {
    #[inline(always)]
    fn default() -> Csmod {
        <crate::RegValueT<Csmod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod csmod {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prmod_SPEC;
    pub type Prmod = crate::EnumBitfieldStruct<u8, Prmod_SPEC>;
    impl Prmod {
        #[doc = "Normal access compatible mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "External data read continuous assertion mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwenb_SPEC;
    pub type Pwenb = crate::EnumBitfieldStruct<u8, Pwenb_SPEC>;
    impl Pwenb {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prenb_SPEC;
    pub type Prenb = crate::EnumBitfieldStruct<u8, Prenb_SPEC>;
    impl Prenb {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewenb_SPEC;
    pub type Ewenb = crate::EnumBitfieldStruct<u8, Ewenb_SPEC>;
    impl Ewenb {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrmod_SPEC;
    pub type Wrmod = crate::EnumBitfieldStruct<u8, Wrmod_SPEC>;
    impl Wrmod {
        #[doc = "Byte strobe mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single write strobe mode"]
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
}
impl ::core::default::Default for Cswcr1 {
    #[inline(always)]
    fn default() -> Cswcr1 {
        <crate::RegValueT<Cswcr1_SPEC> as RegisterValue<_>>::new(117901063)
    }
}
pub mod cswcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csrwait_SPEC;
    pub type Csrwait = crate::EnumBitfieldStruct<u8, Csrwait_SPEC>;
    impl Csrwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cswwait_SPEC;
    pub type Cswwait = crate::EnumBitfieldStruct<u8, Cswwait_SPEC>;
    impl Cswwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csprwait_SPEC;
    pub type Csprwait = crate::EnumBitfieldStruct<u8, Csprwait_SPEC>;
    impl Csprwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cspwwait_SPEC;
    pub type Cspwwait = crate::EnumBitfieldStruct<u8, Cspwwait_SPEC>;
    impl Cspwwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
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

    #[doc = "Write-Access CS Extension Cycle Select"]
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

    #[doc = "Read-Access CS Extension Cycle Select"]
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
}
impl ::core::default::Default for Cswcr2 {
    #[inline(always)]
    fn default() -> Cswcr2 {
        <crate::RegValueT<Cswcr2_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod cswcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cson_SPEC;
    pub type Cson = crate::EnumBitfieldStruct<u8, Cson_SPEC>;
    impl Cson {
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
    pub struct Wron_SPEC;
    pub type Wron = crate::EnumBitfieldStruct<u8, Wron_SPEC>;
    impl Wron {
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
    pub struct Await_SPEC;
    pub type Await = crate::EnumBitfieldStruct<u8, Await_SPEC>;
    impl Await {
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
    pub struct Cswoff_SPEC;
    pub type Cswoff = crate::EnumBitfieldStruct<u8, Cswoff_SPEC>;
    impl Cswoff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csroff_SPEC;
    pub type Csroff = crate::EnumBitfieldStruct<u8, Csroff_SPEC>;
    impl Csroff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);
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
}
impl ::core::default::Default for Sdccr {
    #[inline(always)]
    fn default() -> Sdccr {
        <crate::RegValueT<Sdccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        #[doc = "A 16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "An 8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
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
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable."]
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
    #[doc = "Auto-Refresh Cycle/ Self-Refresh Clearing Cycle Count Setting. ( REFW+1 Cycles )"]
    #[inline(always)]
    pub fn refw(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Sdrfcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Sdrfcr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[doc = "Initialization Precharge Cycle Count (PRC+3 cycles)"]
    #[inline(always)]
    pub fn prc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Sdir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Sdir_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Initialization Auto-Refresh Interval (ARFI+3 cycles )"]
    #[inline(always)]
    pub fn arfi(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Sdir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Sdir_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Row Active Interval"]
    #[inline(always)]
    pub fn ras(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        sdtr::Ras,
        sdtr::Ras,
        Sdtr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            sdtr::Ras,
            sdtr::Ras,
            Sdtr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Row Column Latency ( RCD+1  cycles )"]
    #[inline(always)]
    pub fn rcd(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Sdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Row Precharge Interval ( RP+1  cycles )"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, Sdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Recovery Interval"]
    #[inline(always)]
    pub fn wr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sdtr::Wr, sdtr::Wr, Sdtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,sdtr::Wr,sdtr::Wr,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SDRAMC Column Latency"]
    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, sdtr::Cl, sdtr::Cl, Sdtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,sdtr::Cl,sdtr::Cl,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Ras_SPEC;
    pub type Ras = crate::EnumBitfieldStruct<u8, Ras_SPEC>;
    impl Ras {
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
}
impl ::core::default::Default for Sdsr {
    #[inline(always)]
    fn default() -> Sdsr {
        <crate::RegValueT<Sdsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srfst_SPEC;
    pub type Srfst = crate::EnumBitfieldStruct<u8, Srfst_SPEC>;
    impl Srfst {
        #[doc = "Transition/recovery not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transition/recovery in progress"]
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
    pub struct Mrsst_SPEC;
    pub type Mrsst = crate::EnumBitfieldStruct<u8, Mrsst_SPEC>;
    impl Mrsst {
        #[doc = "Mode register setting not in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Mode register setting in progress"]
        pub const _1: Self = Self::new(1);
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
pub struct Buserrstat_SPEC;
impl crate::sealed::RegSpec for Buserrstat_SPEC {
    type DataType = u8;
}

#[doc = "Bus Error Status Register %s"]
pub type Buserrstat = crate::RegValueT<Buserrstat_SPEC>;

impl Buserrstat {
    #[doc = "Bus Error StatusWhen bus error assert, error flag occurs."]
    #[inline(always)]
    pub fn errstat(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        buserrstat::Errstat,
        buserrstat::Errstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            buserrstat::Errstat,
            buserrstat::Errstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Error access statusThe status at the time of the error"]
    #[inline(always)]
    pub fn accstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstat::Accstat,
        buserrstat::Accstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstat::Accstat,
            buserrstat::Accstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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
    pub struct Errstat_SPEC;
    pub type Errstat = crate::EnumBitfieldStruct<u8, Errstat_SPEC>;
    impl Errstat {
        #[doc = "No bus error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Accstat_SPEC;
    pub type Accstat = crate::EnumBitfieldStruct<u8, Accstat_SPEC>;
    impl Accstat {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write Access"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcntsys_SPEC;
impl crate::sealed::RegSpec for Busmcntsys_SPEC {
    type DataType = u16;
}

#[doc = "Master Bus Control Register SYS"]
pub type Busmcntsys = crate::RegValueT<Busmcntsys_SPEC>;

impl Busmcntsys {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        busmcntsys::Ieres,
        busmcntsys::Ieres,
        Busmcntsys_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            busmcntsys::Ieres,
            busmcntsys::Ieres,
            Busmcntsys_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busmcntsys {
    #[inline(always)]
    fn default() -> Busmcntsys {
        <crate::RegValueT<Busmcntsys_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcntsys {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "Bus error will be reported."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error will not be reported."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcntdma_SPEC;
impl crate::sealed::RegSpec for Busmcntdma_SPEC {
    type DataType = u16;
}

#[doc = "Master Bus Control Register DMA"]
pub type Busmcntdma = crate::RegValueT<Busmcntdma_SPEC>;

impl Busmcntdma {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        busmcntdma::Ieres,
        busmcntdma::Ieres,
        Busmcntdma_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            busmcntdma::Ieres,
            busmcntdma::Ieres,
            Busmcntdma_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busmcntdma {
    #[inline(always)]
    fn default() -> Busmcntdma {
        <crate::RegValueT<Busmcntdma_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcntdma {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "Bus error will be reported."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error will not be reported."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcnt_SPEC;
impl crate::sealed::RegSpec for Busmcnt_SPEC {
    type DataType = u16;
}

#[doc = "Master Bus Control Register %s"]
pub type Busmcnt = crate::RegValueT<Busmcnt_SPEC>;

impl Busmcnt {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        busmcnt::Ieres,
        busmcnt::Ieres,
        Busmcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            busmcnt::Ieres,
            busmcnt::Ieres,
            Busmcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busmcnt {
    #[inline(always)]
    fn default() -> Busmcnt {
        <crate::RegValueT<Busmcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "Bus error will be reported."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error will not be reported."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntmbiu_SPEC;
impl crate::sealed::RegSpec for Busscntmbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register MBIU"]
pub type Busscntmbiu = crate::RegValueT<Busscntmbiu_SPEC>;

impl Busscntmbiu {
    #[doc = "Early Write ResponseWhether the next write request is accepted or not until a response for the write transaction comes back."]
    #[inline(always)]
    pub fn ewres(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        busscntmbiu::Ewres,
        busscntmbiu::Ewres,
        Busscntmbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            busscntmbiu::Ewres,
            busscntmbiu::Ewres,
            Busscntmbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntmbiu::Arbmet,
        busscntmbiu::Arbmet,
        Busscntmbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntmbiu::Arbmet,
            busscntmbiu::Arbmet,
            Busscntmbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntmbiu {
    #[inline(always)]
    fn default() -> Busscntmbiu {
        <crate::RegValueT<Busscntmbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntmbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewres_SPEC;
    pub type Ewres = crate::EnumBitfieldStruct<u8, Ewres_SPEC>;
    impl Ewres {
        #[doc = "Not accepted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Accepted but error response is ignored."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnt_SPEC;
impl crate::sealed::RegSpec for Busscnt_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register %s"]
pub type Busscnt = crate::RegValueT<Busscnt_SPEC>;

impl Busscnt {
    #[doc = "Early Write ResponseWhether the next write request is accepted or not until a response for the write transaction comes back."]
    #[inline(always)]
    pub fn ewres(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        busscnt::Ewres,
        busscnt::Ewres,
        Busscnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            busscnt::Ewres,
            busscnt::Ewres,
            Busscnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscnt::Arbmet,
        busscnt::Arbmet,
        Busscnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscnt::Arbmet,
            busscnt::Arbmet,
            Busscnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscnt {
    #[inline(always)]
    fn default() -> Busscnt {
        <crate::RegValueT<Busscnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewres_SPEC;
    pub type Ewres = crate::EnumBitfieldStruct<u8, Ewres_SPEC>;
    impl Ewres {
        #[doc = "Not accepted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Accepted but error response is ignored."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
    }
}
