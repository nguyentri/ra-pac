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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:54:26 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Bus Control"]
unsafe impl ::core::marker::Send for super::BusNs {}
unsafe impl ::core::marker::Sync for super::BusNs {}
impl super::BusNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CS%s Mode Register (n = 0 to 7)"]
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

    #[doc = "CS%s Wait Control Register 1 (n = 0 to 7)"]
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

    #[doc = "CS%s Wait Control Register 2 (n = 0 to 7)"]
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

    #[doc = "CS%s Recovery Cycle Register (n = 0 to 7)"]
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

    #[doc = "BUS Operation After Detection Register"]
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

    #[doc = "BUS Operation After Detection Protect Register"]
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

    #[doc = "Bus Slave Arbitration Control Register 1(x = FHBI, S0BI, S1BI)"]
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

    #[doc = "Bus Slave Arbitration Control Register 0"]
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

    #[doc = "Bus Slave Arbitration Control Register 1(x = FHBI, S0BI, S1BI)"]
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

    #[doc = "Bus Slave Arbitration Control Register 1(x = FHBI, S0BI, S1BI)"]
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

    #[doc = "Bus Slave Arbitration Control Register 0"]
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

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0ecbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Ecbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Ecbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4688usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0eobi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Eobi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Eobi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4696usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
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

    #[doc = "Bus Slave Arbitration Control Register 0"]
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

    #[doc = "Bus Slave Arbitration Control Register 0"]
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

    #[doc = "Bus Slave Arbitration Control Register 0"]
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

    #[doc = "BUS Error Read Write (n = 6 to 9)"]
    #[inline(always)]
    pub const fn buserrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrrw_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1824usize))
        }
    }
    #[inline(always)]
    pub const fn bus6errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus7errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1834usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus8errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1844usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus9errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1854usize),
            )
        }
    }

    #[doc = "BUS Error Address Register (n = 6 to 9)"]
    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1850usize))
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

    #[doc = "Bus Master Security Attribution Unit Error Address"]
    #[inline(always)]
    pub const fn bmsaerradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bmsaerradd_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1920usize))
        }
    }
    #[inline(always)]
    pub const fn bmsa6erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1920usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bmsa7erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1930usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bmsa8erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1940usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bmsa9erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1950usize),
            )
        }
    }

    #[doc = "BUS Master Security Attribution Unit Error Read Write (n = 6 to 9)"]
    #[inline(always)]
    pub const fn bmsaerrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bmsaerrrw_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1924usize))
        }
    }
    #[inline(always)]
    pub const fn bmsa6errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1924usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bmsa7errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1934usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bmsa8errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1944usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bmsa9errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Bmsaerrrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bmsaerrrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1954usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a50usize))
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

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a58usize))
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

#[doc = "CS%s Mode Register (n = 0 to 7)"]
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

        #[doc = "Single-write strobe mode"]
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

#[doc = "CS%s Wait Control Register 1 (n = 0 to 7)"]
pub type Cswcr1 = crate::RegValueT<Cswcr1_SPEC>;

impl Cswcr1 {
    #[doc = "Page Write Cycle Wait Select"]
    #[inline(always)]
    pub fn cspwwait(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Cswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Cswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Page Read Cycle Wait Select"]
    #[inline(always)]
    pub fn csprwait(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Cswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Cswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Normal Write Cycle Wait Select"]
    #[inline(always)]
    pub fn cswwait(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Cswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Cswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Normal Read Cycle Wait Select"]
    #[inline(always)]
    pub fn csrwait(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Cswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Cswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cswcr1 {
    #[inline(always)]
    fn default() -> Cswcr1 {
        <crate::RegValueT<Cswcr1_SPEC> as RegisterValue<_>>::new(117901063)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cswcr2_SPEC;
impl crate::sealed::RegSpec for Cswcr2_SPEC {
    type DataType = u32;
}

#[doc = "CS%s Wait Control Register 2 (n = 0 to 7)"]
pub type Cswcr2 = crate::RegValueT<Cswcr2_SPEC>;

impl Cswcr2 {
    #[doc = "Read Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn csroff(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn cswoff(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Data Output Extension Cycle Select"]
    #[inline(always)]
    pub fn wdoff(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Address Cycle Wait Select"]
    #[inline(always)]
    pub fn r#await(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RD Assert Wait Select"]
    #[inline(always)]
    pub fn rdon(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "WR Assert Wait Select"]
    #[inline(always)]
    pub fn wron(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Data Output Wait Select"]
    #[inline(always)]
    pub fn wdon(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CS Assert Wait Select"]
    #[inline(always)]
    pub fn cson(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cswcr2 {
    #[inline(always)]
    fn default() -> Cswcr2 {
        <crate::RegValueT<Cswcr2_SPEC> as RegisterValue<_>>::new(7)
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
}
impl ::core::default::Default for Cs0Cr {
    #[inline(always)]
    fn default() -> Cs0Cr {
        <crate::RegValueT<Cs0Cr_SPEC> as RegisterValue<_>>::new(1)
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

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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
        #[doc = "Separate bus interface is selected for area n."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area n."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csrec_SPEC;
impl crate::sealed::RegSpec for Csrec_SPEC {
    type DataType = u16;
}

#[doc = "CS%s Recovery Cycle Register (n = 0 to 7)"]
pub type Csrec = crate::RegValueT<Csrec_SPEC>;

impl Csrec {
    #[doc = "Read Recovery"]
    #[inline(always)]
    pub fn rrcv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Csrec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Csrec_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Recovery"]
    #[inline(always)]
    pub fn wrcv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Csrec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Csrec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Csrec {
    #[inline(always)]
    fn default() -> Csrec {
        <crate::RegValueT<Csrec_SPEC> as RegisterValue<_>>::new(0)
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

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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
        #[doc = "Separate bus interface is selected for area n."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area n."]
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

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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
        #[doc = "Endian order of SDRAM address space is the same as the endian order of the operating mode."]
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
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Sdrfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Sdrfcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Auto-Refresh Cycle/Self-Refresh Clearing Cycle Count Setting"]
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

        #[doc = "Start initialization sequence"]
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
    #[doc = "Initialization Auto-Refresh Interval"]
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
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Sdir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Sdir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Initialization Precharge Cycle Count"]
    #[inline(always)]
    pub fn prc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Sdir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Sdir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdir {
    #[inline(always)]
    fn default() -> Sdir {
        <crate::RegValueT<Sdir_SPEC> as RegisterValue<_>>::new(16)
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
    #[doc = "SDRAMC Column Latency"]
    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Sdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Recovery Interval"]
    #[inline(always)]
    pub fn wr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sdtr::Wr, sdtr::Wr, Sdtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,sdtr::Wr,sdtr::Wr,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Row Precharge Interval"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, Sdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Row Column Latency"]
    #[inline(always)]
    pub fn rcd(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        sdtr::Rcd,
        sdtr::Rcd,
        Sdtr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            sdtr::Rcd,
            sdtr::Rcd,
            Sdtr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Row Active Interval"]
    #[inline(always)]
    pub fn rai(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Sdtr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Wr_SPEC;
    pub type Wr = crate::EnumBitfieldStruct<u8, Wr_SPEC>;
    impl Wr {
        #[doc = "1 cycle"]
        pub const _0: Self = Self::new(0);

        #[doc = "2 cycles"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcd_SPEC;
    pub type Rcd = crate::EnumBitfieldStruct<u8, Rcd_SPEC>;
    impl Rcd {
        #[doc = "1 cycle"]
        pub const _00: Self = Self::new(0);

        #[doc = "2 cycles"]
        pub const _01: Self = Self::new(1);

        #[doc = "3 cycles"]
        pub const _10: Self = Self::new(2);

        #[doc = "4 cycles"]
        pub const _11: Self = Self::new(3);
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
    #[doc = "Mode Register Setting"]
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

#[doc = "BUS Operation After Detection Register"]
pub type Busoad = crate::RegValueT<Busoad_SPEC>;

impl Busoad {
    #[doc = "Illegal address access error operation after detection"]
    #[inline(always)]
    pub fn ilerroad(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Busoad_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Busoad_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave bus error operation after detection"]
    #[inline(always)]
    pub fn slerroad(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Busoad_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Busoad_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Bufferable write error operation after detection"]
    #[inline(always)]
    pub fn bwerroad(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Busoad_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Busoad_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Busoad {
    #[inline(always)]
    fn default() -> Busoad {
        <crate::RegValueT<Busoad_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busoadpt_SPEC;
impl crate::sealed::RegSpec for Busoadpt_SPEC {
    type DataType = u16;
}

#[doc = "BUS Operation After Detection Protect Register"]
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
    #[doc = "Arbitration Select for GDSSBI"]
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

#[doc = "Bus Slave Arbitration Control Register 1(x = FHBI, S0BI, S1BI)"]
pub type Bussabt1Fhbi = crate::RegValueT<Bussabt1Fhbi_SPEC>;

impl Bussabt1Fhbi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Flbi = crate::RegValueT<Bussabt0Flbi_SPEC>;

impl Bussabt0Flbi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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

#[doc = "Bus Slave Arbitration Control Register 1(x = FHBI, S0BI, S1BI)"]
pub type Bussabt1S0Bi = crate::RegValueT<Bussabt1S0Bi_SPEC>;

impl Bussabt1S0Bi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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

#[doc = "Bus Slave Arbitration Control Register 1(x = FHBI, S0BI, S1BI)"]
pub type Bussabt1S1Bi = crate::RegValueT<Bussabt1S1Bi_SPEC>;

impl Bussabt1S1Bi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Stbysbi = crate::RegValueT<Bussabt0Stbysbi_SPEC>;

impl Bussabt0Stbysbi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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
pub struct Bussabt0Ecbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Ecbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Ecbi = crate::RegValueT<Bussabt0Ecbi_SPEC>;

impl Bussabt0Ecbi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0ecbi::Arbs,
        bussabt0ecbi::Arbs,
        Bussabt0Ecbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0ecbi::Arbs,
            bussabt0ecbi::Arbs,
            Bussabt0Ecbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Ecbi {
    #[inline(always)]
    fn default() -> Bussabt0Ecbi {
        <crate::RegValueT<Bussabt0Ecbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0ecbi {

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
pub struct Bussabt0Eobi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Eobi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Eobi = crate::RegValueT<Bussabt0Eobi_SPEC>;

impl Bussabt0Eobi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0eobi::Arbs,
        bussabt0eobi::Arbs,
        Bussabt0Eobi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0eobi::Arbs,
            bussabt0eobi::Arbs,
            Bussabt0Eobi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Eobi {
    #[inline(always)]
    fn default() -> Bussabt0Eobi {
        <crate::RegValueT<Bussabt0Eobi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0eobi {

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
pub struct Bussabt0Pbbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pbbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Pbbi = crate::RegValueT<Bussabt0Pbbi_SPEC>;

impl Bussabt0Pbbi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Pabi = crate::RegValueT<Bussabt0Pabi_SPEC>;

impl Bussabt0Pabi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Pibi = crate::RegValueT<Bussabt0Pibi_SPEC>;

impl Bussabt0Pibi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Psbi = crate::RegValueT<Bussabt0Psbi_SPEC>;

impl Bussabt0Psbi {
    #[doc = "Arbitration Select for &lt;slave&gt;"]
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
}
impl ::core::default::Default for Busdivbyp {
    #[inline(always)]
    fn default() -> Busdivbyp {
        <crate::RegValueT<Busdivbyp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busdivbyp {

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
pub struct Buserrrw_SPEC;
impl crate::sealed::RegSpec for Buserrrw_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Read Write (n = 6 to 9)"]
pub type Buserrrw = crate::RegValueT<Buserrrw_SPEC>;

impl Buserrrw {
    #[doc = "Error access Read/Write Status"]
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
pub struct Buserradd_SPEC;
impl crate::sealed::RegSpec for Buserradd_SPEC {
    type DataType = u32;
}

#[doc = "BUS Error Address Register (n = 6 to 9)"]
pub type Buserradd = crate::RegValueT<Buserradd_SPEC>;

impl Buserradd {
    #[doc = "Bus Error Address"]
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
pub struct Bmsaerradd_SPEC;
impl crate::sealed::RegSpec for Bmsaerradd_SPEC {
    type DataType = u32;
}

#[doc = "Bus Master Security Attribution Unit Error Address"]
pub type Bmsaerradd = crate::RegValueT<Bmsaerradd_SPEC>;

impl Bmsaerradd {
    #[doc = "Bus Master Security Attribution Unit Error Address"]
    #[inline(always)]
    pub fn mserad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Bmsaerradd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Bmsaerradd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bmsaerradd {
    #[inline(always)]
    fn default() -> Bmsaerradd {
        <crate::RegValueT<Bmsaerradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmsaerrrw_SPEC;
impl crate::sealed::RegSpec for Bmsaerrrw_SPEC {
    type DataType = u8;
}

#[doc = "BUS Master Security Attribution Unit Error Read Write (n = 6 to 9)"]
pub type Bmsaerrrw = crate::RegValueT<Bmsaerrrw_SPEC>;

impl Bmsaerrrw {
    #[doc = "Master Security Attribution Unit error access Read/Write Status"]
    #[inline(always)]
    pub fn msarwstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bmsaerrrw::Msarwstat,
        bmsaerrrw::Msarwstat,
        Bmsaerrrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bmsaerrrw::Msarwstat,
            bmsaerrrw::Msarwstat,
            Bmsaerrrw_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bmsaerrrw {
    #[inline(always)]
    fn default() -> Bmsaerrrw {
        <crate::RegValueT<Bmsaerrrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bmsaerrrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msarwstat_SPEC;
    pub type Msarwstat = crate::EnumBitfieldStruct<u8, Msarwstat_SPEC>;
    impl Msarwstat {
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

#[doc = "BUS Error Status Register"]
pub type Buserrstat = crate::RegValueT<Buserrstat_SPEC>;

impl Buserrstat {
    #[doc = "Slave Bus Error Status"]
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

    #[doc = "Master MPU Error Status"]
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

    #[doc = "Illegal Address Access Error Status"]
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

    #[doc = "Master Security Attribution Unit Error Status"]
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

#[doc = "BUS Error Clear Register"]
pub type Buserrclr = crate::RegValueT<Buserrclr_SPEC>;

impl Buserrclr {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn mbweclr23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for Sbwerrclr {
    #[inline(always)]
    fn default() -> Sbwerrclr {
        <crate::RegValueT<Sbwerrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
