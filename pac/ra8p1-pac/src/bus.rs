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
#[doc = r"Bus Control"]
unsafe impl ::core::marker::Send for super::Bus {}
unsafe impl ::core::marker::Sync for super::Bus {}
impl super::Bus {
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

    #[doc = "Master Security Attribution Operation After Detection Register"]
    #[inline(always)]
    pub const fn msaoad(
        &self,
    ) -> &'static crate::common::Reg<self::Msaoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msaoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4112usize),
            )
        }
    }

    #[doc = "Master Security Attribution Protect Register"]
    #[inline(always)]
    pub const fn msapt(&self) -> &'static crate::common::Reg<self::Msapt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msapt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

    #[doc = "Bus Master Arbitration Control Register"]
    #[inline(always)]
    pub const fn busmabtgraphbi(
        &self,
    ) -> &'static crate::common::Reg<self::Busmabtgraphbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmabtgraphbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 1"]
    #[inline(always)]
    pub const fn bussabt1mrc0bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt1Mrc0Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt1Mrc0Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4608usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0mre0bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Mre0Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Mre0Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4616usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0s0bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0S0Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0S0Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4616usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0s1bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0S1Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0S1Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4616usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0s2bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0S2Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0S2Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4616usize),
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
                self._svd2pac_as_ptr().add(4664usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0ospi0bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Ospi0Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Ospi0Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4664usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0ospi1bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Ospi1Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Ospi1Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4664usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0s3bi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0S3Bi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0S3Bi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4664usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0cpu0sahbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Cpu0Sahbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Cpu0Sahbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4712usize),
            )
        }
    }

    #[doc = "Bus Slave Arbitration Control Register 0"]
    #[inline(always)]
    pub const fn bussabt0cpu1tcmbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Cpu1Tcmbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Cpu1Tcmbi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4712usize),
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
    pub const fn bussabt0pbbi(
        &self,
    ) -> &'static crate::common::Reg<self::Bussabt0Pbbi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussabt0Pbbi_SPEC, crate::common::RW>::from_ptr(
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
                self._svd2pac_as_ptr().add(4744usize),
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
                self._svd2pac_as_ptr().add(4744usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatcpu0(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatcpu0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatcpu0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6656usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrcpu0(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrcpu0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrcpu0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6660usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqencpu0(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqencpu0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqencpu0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6664usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatcpu1(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatcpu1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatcpu1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6672usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrcpu1(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrcpu1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrcpu1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6676usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqencpu1(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqencpu1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqencpu1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6680usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatdmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatdmac0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatdmac0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6688usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrdmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrdmac0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrdmac0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6692usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqendmac0(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqendmac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqendmac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6696usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatdmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatdmac1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatdmac1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6704usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrdmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrdmac1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrdmac1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6708usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqendmac1(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqendmac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqendmac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6712usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatnpu(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatnpu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatnpu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6720usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrnpu(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrnpu_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrnpu_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6724usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqennpu(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqennpu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqennpu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6728usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatedmac_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatedmac_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6736usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclredmac(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclredmac_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclredmac_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6740usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqenedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqenedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqenedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6744usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatglcdc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatglcdc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6752usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrglcdc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrglcdc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6756usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqenglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqenglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqenglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6760usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstattdrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstattdrw_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstattdrw_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6768usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrtdrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrtdrw_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrtdrw_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6772usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqentdrw(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqentdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqentdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6776usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatmipi0(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatmipi0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatmipi0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6784usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrmipi0(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrmipi0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrmipi0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6788usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqenmipi0(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqenmipi0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqenmipi0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6792usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatmipi1(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatmipi1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatmipi1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6800usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrmipi1(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrmipi1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrmipi1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6804usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqenmipi1(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqenmipi1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqenmipi1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6808usize),
            )
        }
    }

    #[doc = "BUS Error Status Register"]
    #[inline(always)]
    pub const fn buserrstatceu(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstatceu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstatceu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6816usize),
            )
        }
    }

    #[doc = "BUS Error Clear Register"]
    #[inline(always)]
    pub const fn buserrclrceu(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclrceu_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Buserrclrceu_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6820usize),
            )
        }
    }

    #[doc = "BUS Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn busirqenceu(
        &self,
    ) -> &'static crate::common::Reg<self::Busirqenceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busirqenceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6824usize),
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
    ) -> &'static crate::common::Reg<self::Mbwerrclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Mbwerrclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(6920usize),
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
        <crate::RegValueT<Cs0Cr_SPEC> as RegisterValue<_>>::new(0)
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

        #[doc = "32-bit bus space"]
        pub const _01: Self = Self::new(1);

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

        #[doc = "32-bit bus space"]
        pub const _01: Self = Self::new(1);

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
        #[doc = "Little endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big endian"]
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

    #[doc = "Row Precharge Interval"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, sdtr::Rp, sdtr::Rp, Sdtr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,sdtr::Rp,sdtr::Rp,Sdtr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Setting prohibited"]
        pub const _000: Self = Self::new(0);

        #[doc = "1 cycle"]
        pub const _001: Self = Self::new(1);

        #[doc = "2 cycles"]
        pub const _010: Self = Self::new(2);

        #[doc = "3 cycles"]
        pub const _011: Self = Self::new(3);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
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

        #[doc = "8 cycles"]
        pub const _111: Self = Self::new(7);
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
pub struct Msaoad_SPEC;
impl crate::sealed::RegSpec for Msaoad_SPEC {
    type DataType = u16;
}

#[doc = "Master Security Attribution Operation After Detection Register"]
pub type Msaoad = crate::RegValueT<Msaoad_SPEC>;

impl Msaoad {
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        msaoad::Oad,
        msaoad::Oad,
        Msaoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            msaoad::Oad,
            msaoad::Oad,
            Msaoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Msaoad_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Msaoad_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Msaoad {
    #[inline(always)]
    fn default() -> Msaoad {
        <crate::RegValueT<Msaoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msaoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "IRQ"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msapt_SPEC;
impl crate::sealed::RegSpec for Msapt_SPEC {
    type DataType = u16;
}

#[doc = "Master Security Attribution Protect Register"]
pub type Msapt = crate::RegValueT<Msapt_SPEC>;

impl Msapt {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        msapt::Protect,
        msapt::Protect,
        Msapt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            msapt::Protect,
            msapt::Protect,
            Msapt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Msapt_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Msapt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Msapt {
    #[inline(always)]
    fn default() -> Msapt {
        <crate::RegValueT<Msapt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msapt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MSAOAD register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "MSAOAD register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmabtgraphbi_SPEC;
impl crate::sealed::RegSpec for Busmabtgraphbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Master Arbitration Control Register"]
pub type Busmabtgraphbi = crate::RegValueT<Busmabtgraphbi_SPEC>;

impl Busmabtgraphbi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busmabtgraphbi::Arbs,
        busmabtgraphbi::Arbs,
        Busmabtgraphbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busmabtgraphbi::Arbs,
            busmabtgraphbi::Arbs,
            Busmabtgraphbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busmabtgraphbi {
    #[inline(always)]
    fn default() -> Busmabtgraphbi {
        <crate::RegValueT<Busmabtgraphbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmabtgraphbi {

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
pub struct Bussabt1Mrc0Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt1Mrc0Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 1"]
pub type Bussabt1Mrc0Bi = crate::RegValueT<Bussabt1Mrc0Bi_SPEC>;

impl Bussabt1Mrc0Bi {
    #[doc = "Arbitration Select"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        bussabt1mrc0bi::Arbs,
        bussabt1mrc0bi::Arbs,
        Bussabt1Mrc0Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            bussabt1mrc0bi::Arbs,
            bussabt1mrc0bi::Arbs,
            Bussabt1Mrc0Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt1Mrc0Bi {
    #[inline(always)]
    fn default() -> Bussabt1Mrc0Bi {
        <crate::RegValueT<Bussabt1Mrc0Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt1mrc0bi {

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
pub struct Bussabt0Mre0Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Mre0Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Mre0Bi = crate::RegValueT<Bussabt0Mre0Bi_SPEC>;

impl Bussabt0Mre0Bi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0mre0bi::Arbs,
        bussabt0mre0bi::Arbs,
        Bussabt0Mre0Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0mre0bi::Arbs,
            bussabt0mre0bi::Arbs,
            Bussabt0Mre0Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Mre0Bi {
    #[inline(always)]
    fn default() -> Bussabt0Mre0Bi {
        <crate::RegValueT<Bussabt0Mre0Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0mre0bi {

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
pub struct Bussabt0S0Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt0S0Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0S0Bi = crate::RegValueT<Bussabt0S0Bi_SPEC>;

impl Bussabt0S0Bi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0s0bi::Arbs,
        bussabt0s0bi::Arbs,
        Bussabt0S0Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0s0bi::Arbs,
            bussabt0s0bi::Arbs,
            Bussabt0S0Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0S0Bi {
    #[inline(always)]
    fn default() -> Bussabt0S0Bi {
        <crate::RegValueT<Bussabt0S0Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0s0bi {

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
pub struct Bussabt0S1Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt0S1Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0S1Bi = crate::RegValueT<Bussabt0S1Bi_SPEC>;

impl Bussabt0S1Bi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0s1bi::Arbs,
        bussabt0s1bi::Arbs,
        Bussabt0S1Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0s1bi::Arbs,
            bussabt0s1bi::Arbs,
            Bussabt0S1Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0S1Bi {
    #[inline(always)]
    fn default() -> Bussabt0S1Bi {
        <crate::RegValueT<Bussabt0S1Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0s1bi {

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
pub struct Bussabt0S2Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt0S2Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0S2Bi = crate::RegValueT<Bussabt0S2Bi_SPEC>;

impl Bussabt0S2Bi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0s2bi::Arbs,
        bussabt0s2bi::Arbs,
        Bussabt0S2Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0s2bi::Arbs,
            bussabt0s2bi::Arbs,
            Bussabt0S2Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0S2Bi {
    #[inline(always)]
    fn default() -> Bussabt0S2Bi {
        <crate::RegValueT<Bussabt0S2Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0s2bi {

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
    #[doc = "Arbitration Select for <slave>"]
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
pub struct Bussabt0Ospi0Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Ospi0Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Ospi0Bi = crate::RegValueT<Bussabt0Ospi0Bi_SPEC>;

impl Bussabt0Ospi0Bi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0ospi0bi::Arbs,
        bussabt0ospi0bi::Arbs,
        Bussabt0Ospi0Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0ospi0bi::Arbs,
            bussabt0ospi0bi::Arbs,
            Bussabt0Ospi0Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Ospi0Bi {
    #[inline(always)]
    fn default() -> Bussabt0Ospi0Bi {
        <crate::RegValueT<Bussabt0Ospi0Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0ospi0bi {

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
pub struct Bussabt0Ospi1Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Ospi1Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Ospi1Bi = crate::RegValueT<Bussabt0Ospi1Bi_SPEC>;

impl Bussabt0Ospi1Bi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0ospi1bi::Arbs,
        bussabt0ospi1bi::Arbs,
        Bussabt0Ospi1Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0ospi1bi::Arbs,
            bussabt0ospi1bi::Arbs,
            Bussabt0Ospi1Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Ospi1Bi {
    #[inline(always)]
    fn default() -> Bussabt0Ospi1Bi {
        <crate::RegValueT<Bussabt0Ospi1Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0ospi1bi {

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
pub struct Bussabt0S3Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt0S3Bi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0S3Bi = crate::RegValueT<Bussabt0S3Bi_SPEC>;

impl Bussabt0S3Bi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0s3bi::Arbs,
        bussabt0s3bi::Arbs,
        Bussabt0S3Bi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0s3bi::Arbs,
            bussabt0s3bi::Arbs,
            Bussabt0S3Bi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0S3Bi {
    #[inline(always)]
    fn default() -> Bussabt0S3Bi {
        <crate::RegValueT<Bussabt0S3Bi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0s3bi {

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
pub struct Bussabt0Cpu0Sahbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Cpu0Sahbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Cpu0Sahbi = crate::RegValueT<Bussabt0Cpu0Sahbi_SPEC>;

impl Bussabt0Cpu0Sahbi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0cpu0sahbi::Arbs,
        bussabt0cpu0sahbi::Arbs,
        Bussabt0Cpu0Sahbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0cpu0sahbi::Arbs,
            bussabt0cpu0sahbi::Arbs,
            Bussabt0Cpu0Sahbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Cpu0Sahbi {
    #[inline(always)]
    fn default() -> Bussabt0Cpu0Sahbi {
        <crate::RegValueT<Bussabt0Cpu0Sahbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0cpu0sahbi {

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
pub struct Bussabt0Cpu1Tcmbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Cpu1Tcmbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Cpu1Tcmbi = crate::RegValueT<Bussabt0Cpu1Tcmbi_SPEC>;

impl Bussabt0Cpu1Tcmbi {
    #[doc = "Arbitration Select for <slave>"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bussabt0cpu1tcmbi::Arbs,
        bussabt0cpu1tcmbi::Arbs,
        Bussabt0Cpu1Tcmbi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bussabt0cpu1tcmbi::Arbs,
            bussabt0cpu1tcmbi::Arbs,
            Bussabt0Cpu1Tcmbi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bussabt0Cpu1Tcmbi {
    #[inline(always)]
    fn default() -> Bussabt0Cpu1Tcmbi {
        <crate::RegValueT<Bussabt0Cpu1Tcmbi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussabt0cpu1tcmbi {

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
    #[doc = "Arbitration Select for <slave>"]
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
pub struct Bussabt0Pbbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pbbi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Pbbi = crate::RegValueT<Bussabt0Pbbi_SPEC>;

impl Bussabt0Pbbi {
    #[doc = "Arbitration Select for <slave>"]
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
pub struct Bussabt0Pibi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pibi_SPEC {
    type DataType = u32;
}

#[doc = "Bus Slave Arbitration Control Register 0"]
pub type Bussabt0Pibi = crate::RegValueT<Bussabt0Pibi_SPEC>;

impl Bussabt0Pibi {
    #[doc = "Arbitration Select for <slave>"]
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
    #[doc = "Arbitration Select for <slave>"]
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
pub struct Buserrstatcpu0_SPEC;
impl crate::sealed::RegSpec for Buserrstatcpu0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatcpu0 = crate::RegValueT<Buserrstatcpu0_SPEC>;

impl Buserrstatcpu0 {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatcpu0::Slerrstat,
        buserrstatcpu0::Slerrstat,
        Buserrstatcpu0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatcpu0::Slerrstat,
            buserrstatcpu0::Slerrstat,
            Buserrstatcpu0_SPEC,
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
        buserrstatcpu0::Mmerrstat,
        buserrstatcpu0::Mmerrstat,
        Buserrstatcpu0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatcpu0::Mmerrstat,
            buserrstatcpu0::Mmerrstat,
            Buserrstatcpu0_SPEC,
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
        buserrstatcpu0::Ilerrstat,
        buserrstatcpu0::Ilerrstat,
        Buserrstatcpu0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatcpu0::Ilerrstat,
            buserrstatcpu0::Ilerrstat,
            Buserrstatcpu0_SPEC,
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
        buserrstatcpu0::Mserrstat,
        buserrstatcpu0::Mserrstat,
        Buserrstatcpu0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatcpu0::Mserrstat,
            buserrstatcpu0::Mserrstat,
            Buserrstatcpu0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatcpu0 {
    #[inline(always)]
    fn default() -> Buserrstatcpu0 {
        <crate::RegValueT<Buserrstatcpu0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatcpu0 {

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
pub struct Buserrclrcpu0_SPEC;
impl crate::sealed::RegSpec for Buserrclrcpu0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrcpu0 = crate::RegValueT<Buserrclrcpu0_SPEC>;

impl Buserrclrcpu0 {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrcpu0::Slerrclr,
        buserrclrcpu0::Slerrclr,
        Buserrclrcpu0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrcpu0::Slerrclr,
            buserrclrcpu0::Slerrclr,
            Buserrclrcpu0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrcpu0::Mmerrclr,
        buserrclrcpu0::Mmerrclr,
        Buserrclrcpu0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrcpu0::Mmerrclr,
            buserrclrcpu0::Mmerrclr,
            Buserrclrcpu0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrcpu0::Ilerrclr,
        buserrclrcpu0::Ilerrclr,
        Buserrclrcpu0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrcpu0::Ilerrclr,
            buserrclrcpu0::Ilerrclr,
            Buserrclrcpu0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrcpu0::Mserrclr,
        buserrclrcpu0::Mserrclr,
        Buserrclrcpu0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrcpu0::Mserrclr,
            buserrclrcpu0::Mserrclr,
            Buserrclrcpu0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrcpu0 {
    #[inline(always)]
    fn default() -> Buserrclrcpu0 {
        <crate::RegValueT<Buserrclrcpu0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrcpu0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqencpu0_SPEC;
impl crate::sealed::RegSpec for Busirqencpu0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqencpu0 = crate::RegValueT<Busirqencpu0_SPEC>;

impl Busirqencpu0 {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqencpu0::En,
        busirqencpu0::En,
        Busirqencpu0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqencpu0::En,
            busirqencpu0::En,
            Busirqencpu0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqencpu0 {
    #[inline(always)]
    fn default() -> Busirqencpu0 {
        <crate::RegValueT<Busirqencpu0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqencpu0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatcpu1_SPEC;
impl crate::sealed::RegSpec for Buserrstatcpu1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatcpu1 = crate::RegValueT<Buserrstatcpu1_SPEC>;

impl Buserrstatcpu1 {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatcpu1::Slerrstat,
        buserrstatcpu1::Slerrstat,
        Buserrstatcpu1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatcpu1::Slerrstat,
            buserrstatcpu1::Slerrstat,
            Buserrstatcpu1_SPEC,
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
        buserrstatcpu1::Mmerrstat,
        buserrstatcpu1::Mmerrstat,
        Buserrstatcpu1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatcpu1::Mmerrstat,
            buserrstatcpu1::Mmerrstat,
            Buserrstatcpu1_SPEC,
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
        buserrstatcpu1::Ilerrstat,
        buserrstatcpu1::Ilerrstat,
        Buserrstatcpu1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatcpu1::Ilerrstat,
            buserrstatcpu1::Ilerrstat,
            Buserrstatcpu1_SPEC,
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
        buserrstatcpu1::Mserrstat,
        buserrstatcpu1::Mserrstat,
        Buserrstatcpu1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatcpu1::Mserrstat,
            buserrstatcpu1::Mserrstat,
            Buserrstatcpu1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatcpu1 {
    #[inline(always)]
    fn default() -> Buserrstatcpu1 {
        <crate::RegValueT<Buserrstatcpu1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatcpu1 {

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
pub struct Buserrclrcpu1_SPEC;
impl crate::sealed::RegSpec for Buserrclrcpu1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrcpu1 = crate::RegValueT<Buserrclrcpu1_SPEC>;

impl Buserrclrcpu1 {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrcpu1::Slerrclr,
        buserrclrcpu1::Slerrclr,
        Buserrclrcpu1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrcpu1::Slerrclr,
            buserrclrcpu1::Slerrclr,
            Buserrclrcpu1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrcpu1::Mmerrclr,
        buserrclrcpu1::Mmerrclr,
        Buserrclrcpu1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrcpu1::Mmerrclr,
            buserrclrcpu1::Mmerrclr,
            Buserrclrcpu1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrcpu1::Ilerrclr,
        buserrclrcpu1::Ilerrclr,
        Buserrclrcpu1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrcpu1::Ilerrclr,
            buserrclrcpu1::Ilerrclr,
            Buserrclrcpu1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrcpu1::Mserrclr,
        buserrclrcpu1::Mserrclr,
        Buserrclrcpu1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrcpu1::Mserrclr,
            buserrclrcpu1::Mserrclr,
            Buserrclrcpu1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrcpu1 {
    #[inline(always)]
    fn default() -> Buserrclrcpu1 {
        <crate::RegValueT<Buserrclrcpu1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrcpu1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqencpu1_SPEC;
impl crate::sealed::RegSpec for Busirqencpu1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqencpu1 = crate::RegValueT<Busirqencpu1_SPEC>;

impl Busirqencpu1 {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqencpu1::En,
        busirqencpu1::En,
        Busirqencpu1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqencpu1::En,
            busirqencpu1::En,
            Busirqencpu1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqencpu1 {
    #[inline(always)]
    fn default() -> Busirqencpu1 {
        <crate::RegValueT<Busirqencpu1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqencpu1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatdmac0_SPEC;
impl crate::sealed::RegSpec for Buserrstatdmac0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatdmac0 = crate::RegValueT<Buserrstatdmac0_SPEC>;

impl Buserrstatdmac0 {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatdmac0::Slerrstat,
        buserrstatdmac0::Slerrstat,
        Buserrstatdmac0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatdmac0::Slerrstat,
            buserrstatdmac0::Slerrstat,
            Buserrstatdmac0_SPEC,
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
        buserrstatdmac0::Mmerrstat,
        buserrstatdmac0::Mmerrstat,
        Buserrstatdmac0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatdmac0::Mmerrstat,
            buserrstatdmac0::Mmerrstat,
            Buserrstatdmac0_SPEC,
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
        buserrstatdmac0::Ilerrstat,
        buserrstatdmac0::Ilerrstat,
        Buserrstatdmac0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatdmac0::Ilerrstat,
            buserrstatdmac0::Ilerrstat,
            Buserrstatdmac0_SPEC,
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
        buserrstatdmac0::Mserrstat,
        buserrstatdmac0::Mserrstat,
        Buserrstatdmac0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatdmac0::Mserrstat,
            buserrstatdmac0::Mserrstat,
            Buserrstatdmac0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatdmac0 {
    #[inline(always)]
    fn default() -> Buserrstatdmac0 {
        <crate::RegValueT<Buserrstatdmac0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatdmac0 {

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
pub struct Buserrclrdmac0_SPEC;
impl crate::sealed::RegSpec for Buserrclrdmac0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrdmac0 = crate::RegValueT<Buserrclrdmac0_SPEC>;

impl Buserrclrdmac0 {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrdmac0::Slerrclr,
        buserrclrdmac0::Slerrclr,
        Buserrclrdmac0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrdmac0::Slerrclr,
            buserrclrdmac0::Slerrclr,
            Buserrclrdmac0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrdmac0::Mmerrclr,
        buserrclrdmac0::Mmerrclr,
        Buserrclrdmac0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrdmac0::Mmerrclr,
            buserrclrdmac0::Mmerrclr,
            Buserrclrdmac0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrdmac0::Ilerrclr,
        buserrclrdmac0::Ilerrclr,
        Buserrclrdmac0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrdmac0::Ilerrclr,
            buserrclrdmac0::Ilerrclr,
            Buserrclrdmac0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrdmac0::Mserrclr,
        buserrclrdmac0::Mserrclr,
        Buserrclrdmac0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrdmac0::Mserrclr,
            buserrclrdmac0::Mserrclr,
            Buserrclrdmac0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrdmac0 {
    #[inline(always)]
    fn default() -> Buserrclrdmac0 {
        <crate::RegValueT<Buserrclrdmac0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrdmac0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqendmac0_SPEC;
impl crate::sealed::RegSpec for Busirqendmac0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqendmac0 = crate::RegValueT<Busirqendmac0_SPEC>;

impl Busirqendmac0 {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqendmac0::En,
        busirqendmac0::En,
        Busirqendmac0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqendmac0::En,
            busirqendmac0::En,
            Busirqendmac0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqendmac0 {
    #[inline(always)]
    fn default() -> Busirqendmac0 {
        <crate::RegValueT<Busirqendmac0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqendmac0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatdmac1_SPEC;
impl crate::sealed::RegSpec for Buserrstatdmac1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatdmac1 = crate::RegValueT<Buserrstatdmac1_SPEC>;

impl Buserrstatdmac1 {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatdmac1::Slerrstat,
        buserrstatdmac1::Slerrstat,
        Buserrstatdmac1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatdmac1::Slerrstat,
            buserrstatdmac1::Slerrstat,
            Buserrstatdmac1_SPEC,
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
        buserrstatdmac1::Mmerrstat,
        buserrstatdmac1::Mmerrstat,
        Buserrstatdmac1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatdmac1::Mmerrstat,
            buserrstatdmac1::Mmerrstat,
            Buserrstatdmac1_SPEC,
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
        buserrstatdmac1::Ilerrstat,
        buserrstatdmac1::Ilerrstat,
        Buserrstatdmac1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatdmac1::Ilerrstat,
            buserrstatdmac1::Ilerrstat,
            Buserrstatdmac1_SPEC,
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
        buserrstatdmac1::Mserrstat,
        buserrstatdmac1::Mserrstat,
        Buserrstatdmac1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatdmac1::Mserrstat,
            buserrstatdmac1::Mserrstat,
            Buserrstatdmac1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatdmac1 {
    #[inline(always)]
    fn default() -> Buserrstatdmac1 {
        <crate::RegValueT<Buserrstatdmac1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatdmac1 {

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
pub struct Buserrclrdmac1_SPEC;
impl crate::sealed::RegSpec for Buserrclrdmac1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrdmac1 = crate::RegValueT<Buserrclrdmac1_SPEC>;

impl Buserrclrdmac1 {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrdmac1::Slerrclr,
        buserrclrdmac1::Slerrclr,
        Buserrclrdmac1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrdmac1::Slerrclr,
            buserrclrdmac1::Slerrclr,
            Buserrclrdmac1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrdmac1::Mmerrclr,
        buserrclrdmac1::Mmerrclr,
        Buserrclrdmac1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrdmac1::Mmerrclr,
            buserrclrdmac1::Mmerrclr,
            Buserrclrdmac1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrdmac1::Ilerrclr,
        buserrclrdmac1::Ilerrclr,
        Buserrclrdmac1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrdmac1::Ilerrclr,
            buserrclrdmac1::Ilerrclr,
            Buserrclrdmac1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrdmac1::Mserrclr,
        buserrclrdmac1::Mserrclr,
        Buserrclrdmac1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrdmac1::Mserrclr,
            buserrclrdmac1::Mserrclr,
            Buserrclrdmac1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrdmac1 {
    #[inline(always)]
    fn default() -> Buserrclrdmac1 {
        <crate::RegValueT<Buserrclrdmac1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrdmac1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqendmac1_SPEC;
impl crate::sealed::RegSpec for Busirqendmac1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqendmac1 = crate::RegValueT<Busirqendmac1_SPEC>;

impl Busirqendmac1 {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqendmac1::En,
        busirqendmac1::En,
        Busirqendmac1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqendmac1::En,
            busirqendmac1::En,
            Busirqendmac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqendmac1 {
    #[inline(always)]
    fn default() -> Busirqendmac1 {
        <crate::RegValueT<Busirqendmac1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqendmac1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatnpu_SPEC;
impl crate::sealed::RegSpec for Buserrstatnpu_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatnpu = crate::RegValueT<Buserrstatnpu_SPEC>;

impl Buserrstatnpu {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatnpu::Slerrstat,
        buserrstatnpu::Slerrstat,
        Buserrstatnpu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatnpu::Slerrstat,
            buserrstatnpu::Slerrstat,
            Buserrstatnpu_SPEC,
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
        buserrstatnpu::Mmerrstat,
        buserrstatnpu::Mmerrstat,
        Buserrstatnpu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatnpu::Mmerrstat,
            buserrstatnpu::Mmerrstat,
            Buserrstatnpu_SPEC,
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
        buserrstatnpu::Ilerrstat,
        buserrstatnpu::Ilerrstat,
        Buserrstatnpu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatnpu::Ilerrstat,
            buserrstatnpu::Ilerrstat,
            Buserrstatnpu_SPEC,
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
        buserrstatnpu::Mserrstat,
        buserrstatnpu::Mserrstat,
        Buserrstatnpu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatnpu::Mserrstat,
            buserrstatnpu::Mserrstat,
            Buserrstatnpu_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatnpu {
    #[inline(always)]
    fn default() -> Buserrstatnpu {
        <crate::RegValueT<Buserrstatnpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatnpu {

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
pub struct Buserrclrnpu_SPEC;
impl crate::sealed::RegSpec for Buserrclrnpu_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrnpu = crate::RegValueT<Buserrclrnpu_SPEC>;

impl Buserrclrnpu {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrnpu::Slerrclr,
        buserrclrnpu::Slerrclr,
        Buserrclrnpu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrnpu::Slerrclr,
            buserrclrnpu::Slerrclr,
            Buserrclrnpu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrnpu::Mmerrclr,
        buserrclrnpu::Mmerrclr,
        Buserrclrnpu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrnpu::Mmerrclr,
            buserrclrnpu::Mmerrclr,
            Buserrclrnpu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrnpu::Ilerrclr,
        buserrclrnpu::Ilerrclr,
        Buserrclrnpu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrnpu::Ilerrclr,
            buserrclrnpu::Ilerrclr,
            Buserrclrnpu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrnpu::Mserrclr,
        buserrclrnpu::Mserrclr,
        Buserrclrnpu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrnpu::Mserrclr,
            buserrclrnpu::Mserrclr,
            Buserrclrnpu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrnpu {
    #[inline(always)]
    fn default() -> Buserrclrnpu {
        <crate::RegValueT<Buserrclrnpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrnpu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqennpu_SPEC;
impl crate::sealed::RegSpec for Busirqennpu_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqennpu = crate::RegValueT<Busirqennpu_SPEC>;

impl Busirqennpu {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqennpu::En,
        busirqennpu::En,
        Busirqennpu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqennpu::En,
            busirqennpu::En,
            Busirqennpu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqennpu {
    #[inline(always)]
    fn default() -> Busirqennpu {
        <crate::RegValueT<Busirqennpu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqennpu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatedmac_SPEC;
impl crate::sealed::RegSpec for Buserrstatedmac_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatedmac = crate::RegValueT<Buserrstatedmac_SPEC>;

impl Buserrstatedmac {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatedmac::Slerrstat,
        buserrstatedmac::Slerrstat,
        Buserrstatedmac_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatedmac::Slerrstat,
            buserrstatedmac::Slerrstat,
            Buserrstatedmac_SPEC,
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
        buserrstatedmac::Mmerrstat,
        buserrstatedmac::Mmerrstat,
        Buserrstatedmac_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatedmac::Mmerrstat,
            buserrstatedmac::Mmerrstat,
            Buserrstatedmac_SPEC,
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
        buserrstatedmac::Ilerrstat,
        buserrstatedmac::Ilerrstat,
        Buserrstatedmac_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatedmac::Ilerrstat,
            buserrstatedmac::Ilerrstat,
            Buserrstatedmac_SPEC,
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
        buserrstatedmac::Mserrstat,
        buserrstatedmac::Mserrstat,
        Buserrstatedmac_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatedmac::Mserrstat,
            buserrstatedmac::Mserrstat,
            Buserrstatedmac_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatedmac {
    #[inline(always)]
    fn default() -> Buserrstatedmac {
        <crate::RegValueT<Buserrstatedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatedmac {

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
pub struct Buserrclredmac_SPEC;
impl crate::sealed::RegSpec for Buserrclredmac_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclredmac = crate::RegValueT<Buserrclredmac_SPEC>;

impl Buserrclredmac {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclredmac::Slerrclr,
        buserrclredmac::Slerrclr,
        Buserrclredmac_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclredmac::Slerrclr,
            buserrclredmac::Slerrclr,
            Buserrclredmac_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclredmac::Mmerrclr,
        buserrclredmac::Mmerrclr,
        Buserrclredmac_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclredmac::Mmerrclr,
            buserrclredmac::Mmerrclr,
            Buserrclredmac_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclredmac::Ilerrclr,
        buserrclredmac::Ilerrclr,
        Buserrclredmac_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclredmac::Ilerrclr,
            buserrclredmac::Ilerrclr,
            Buserrclredmac_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclredmac::Mserrclr,
        buserrclredmac::Mserrclr,
        Buserrclredmac_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclredmac::Mserrclr,
            buserrclredmac::Mserrclr,
            Buserrclredmac_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclredmac {
    #[inline(always)]
    fn default() -> Buserrclredmac {
        <crate::RegValueT<Buserrclredmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclredmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqenedmac_SPEC;
impl crate::sealed::RegSpec for Busirqenedmac_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqenedmac = crate::RegValueT<Busirqenedmac_SPEC>;

impl Busirqenedmac {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqenedmac::En,
        busirqenedmac::En,
        Busirqenedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqenedmac::En,
            busirqenedmac::En,
            Busirqenedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqenedmac {
    #[inline(always)]
    fn default() -> Busirqenedmac {
        <crate::RegValueT<Busirqenedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqenedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatglcdc_SPEC;
impl crate::sealed::RegSpec for Buserrstatglcdc_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatglcdc = crate::RegValueT<Buserrstatglcdc_SPEC>;

impl Buserrstatglcdc {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatglcdc::Slerrstat,
        buserrstatglcdc::Slerrstat,
        Buserrstatglcdc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatglcdc::Slerrstat,
            buserrstatglcdc::Slerrstat,
            Buserrstatglcdc_SPEC,
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
        buserrstatglcdc::Mmerrstat,
        buserrstatglcdc::Mmerrstat,
        Buserrstatglcdc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatglcdc::Mmerrstat,
            buserrstatglcdc::Mmerrstat,
            Buserrstatglcdc_SPEC,
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
        buserrstatglcdc::Ilerrstat,
        buserrstatglcdc::Ilerrstat,
        Buserrstatglcdc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatglcdc::Ilerrstat,
            buserrstatglcdc::Ilerrstat,
            Buserrstatglcdc_SPEC,
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
        buserrstatglcdc::Mserrstat,
        buserrstatglcdc::Mserrstat,
        Buserrstatglcdc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatglcdc::Mserrstat,
            buserrstatglcdc::Mserrstat,
            Buserrstatglcdc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatglcdc {
    #[inline(always)]
    fn default() -> Buserrstatglcdc {
        <crate::RegValueT<Buserrstatglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatglcdc {

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
pub struct Buserrclrglcdc_SPEC;
impl crate::sealed::RegSpec for Buserrclrglcdc_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrglcdc = crate::RegValueT<Buserrclrglcdc_SPEC>;

impl Buserrclrglcdc {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrglcdc::Slerrclr,
        buserrclrglcdc::Slerrclr,
        Buserrclrglcdc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrglcdc::Slerrclr,
            buserrclrglcdc::Slerrclr,
            Buserrclrglcdc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrglcdc::Mmerrclr,
        buserrclrglcdc::Mmerrclr,
        Buserrclrglcdc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrglcdc::Mmerrclr,
            buserrclrglcdc::Mmerrclr,
            Buserrclrglcdc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrglcdc::Ilerrclr,
        buserrclrglcdc::Ilerrclr,
        Buserrclrglcdc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrglcdc::Ilerrclr,
            buserrclrglcdc::Ilerrclr,
            Buserrclrglcdc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrglcdc::Mserrclr,
        buserrclrglcdc::Mserrclr,
        Buserrclrglcdc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrglcdc::Mserrclr,
            buserrclrglcdc::Mserrclr,
            Buserrclrglcdc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrglcdc {
    #[inline(always)]
    fn default() -> Buserrclrglcdc {
        <crate::RegValueT<Buserrclrglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqenglcdc_SPEC;
impl crate::sealed::RegSpec for Busirqenglcdc_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqenglcdc = crate::RegValueT<Busirqenglcdc_SPEC>;

impl Busirqenglcdc {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqenglcdc::En,
        busirqenglcdc::En,
        Busirqenglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqenglcdc::En,
            busirqenglcdc::En,
            Busirqenglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqenglcdc {
    #[inline(always)]
    fn default() -> Busirqenglcdc {
        <crate::RegValueT<Busirqenglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqenglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstattdrw_SPEC;
impl crate::sealed::RegSpec for Buserrstattdrw_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstattdrw = crate::RegValueT<Buserrstattdrw_SPEC>;

impl Buserrstattdrw {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstattdrw::Slerrstat,
        buserrstattdrw::Slerrstat,
        Buserrstattdrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstattdrw::Slerrstat,
            buserrstattdrw::Slerrstat,
            Buserrstattdrw_SPEC,
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
        buserrstattdrw::Mmerrstat,
        buserrstattdrw::Mmerrstat,
        Buserrstattdrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstattdrw::Mmerrstat,
            buserrstattdrw::Mmerrstat,
            Buserrstattdrw_SPEC,
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
        buserrstattdrw::Ilerrstat,
        buserrstattdrw::Ilerrstat,
        Buserrstattdrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstattdrw::Ilerrstat,
            buserrstattdrw::Ilerrstat,
            Buserrstattdrw_SPEC,
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
        buserrstattdrw::Mserrstat,
        buserrstattdrw::Mserrstat,
        Buserrstattdrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstattdrw::Mserrstat,
            buserrstattdrw::Mserrstat,
            Buserrstattdrw_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstattdrw {
    #[inline(always)]
    fn default() -> Buserrstattdrw {
        <crate::RegValueT<Buserrstattdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstattdrw {

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
pub struct Buserrclrtdrw_SPEC;
impl crate::sealed::RegSpec for Buserrclrtdrw_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrtdrw = crate::RegValueT<Buserrclrtdrw_SPEC>;

impl Buserrclrtdrw {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrtdrw::Slerrclr,
        buserrclrtdrw::Slerrclr,
        Buserrclrtdrw_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrtdrw::Slerrclr,
            buserrclrtdrw::Slerrclr,
            Buserrclrtdrw_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrtdrw::Mmerrclr,
        buserrclrtdrw::Mmerrclr,
        Buserrclrtdrw_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrtdrw::Mmerrclr,
            buserrclrtdrw::Mmerrclr,
            Buserrclrtdrw_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrtdrw::Ilerrclr,
        buserrclrtdrw::Ilerrclr,
        Buserrclrtdrw_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrtdrw::Ilerrclr,
            buserrclrtdrw::Ilerrclr,
            Buserrclrtdrw_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrtdrw::Mserrclr,
        buserrclrtdrw::Mserrclr,
        Buserrclrtdrw_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrtdrw::Mserrclr,
            buserrclrtdrw::Mserrclr,
            Buserrclrtdrw_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrtdrw {
    #[inline(always)]
    fn default() -> Buserrclrtdrw {
        <crate::RegValueT<Buserrclrtdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrtdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqentdrw_SPEC;
impl crate::sealed::RegSpec for Busirqentdrw_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqentdrw = crate::RegValueT<Busirqentdrw_SPEC>;

impl Busirqentdrw {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqentdrw::En,
        busirqentdrw::En,
        Busirqentdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqentdrw::En,
            busirqentdrw::En,
            Busirqentdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqentdrw {
    #[inline(always)]
    fn default() -> Busirqentdrw {
        <crate::RegValueT<Busirqentdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqentdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatmipi0_SPEC;
impl crate::sealed::RegSpec for Buserrstatmipi0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatmipi0 = crate::RegValueT<Buserrstatmipi0_SPEC>;

impl Buserrstatmipi0 {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatmipi0::Slerrstat,
        buserrstatmipi0::Slerrstat,
        Buserrstatmipi0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatmipi0::Slerrstat,
            buserrstatmipi0::Slerrstat,
            Buserrstatmipi0_SPEC,
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
        buserrstatmipi0::Mmerrstat,
        buserrstatmipi0::Mmerrstat,
        Buserrstatmipi0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatmipi0::Mmerrstat,
            buserrstatmipi0::Mmerrstat,
            Buserrstatmipi0_SPEC,
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
        buserrstatmipi0::Ilerrstat,
        buserrstatmipi0::Ilerrstat,
        Buserrstatmipi0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatmipi0::Ilerrstat,
            buserrstatmipi0::Ilerrstat,
            Buserrstatmipi0_SPEC,
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
        buserrstatmipi0::Mserrstat,
        buserrstatmipi0::Mserrstat,
        Buserrstatmipi0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatmipi0::Mserrstat,
            buserrstatmipi0::Mserrstat,
            Buserrstatmipi0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatmipi0 {
    #[inline(always)]
    fn default() -> Buserrstatmipi0 {
        <crate::RegValueT<Buserrstatmipi0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatmipi0 {

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
pub struct Buserrclrmipi0_SPEC;
impl crate::sealed::RegSpec for Buserrclrmipi0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrmipi0 = crate::RegValueT<Buserrclrmipi0_SPEC>;

impl Buserrclrmipi0 {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrmipi0::Slerrclr,
        buserrclrmipi0::Slerrclr,
        Buserrclrmipi0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrmipi0::Slerrclr,
            buserrclrmipi0::Slerrclr,
            Buserrclrmipi0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrmipi0::Mmerrclr,
        buserrclrmipi0::Mmerrclr,
        Buserrclrmipi0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrmipi0::Mmerrclr,
            buserrclrmipi0::Mmerrclr,
            Buserrclrmipi0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrmipi0::Ilerrclr,
        buserrclrmipi0::Ilerrclr,
        Buserrclrmipi0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrmipi0::Ilerrclr,
            buserrclrmipi0::Ilerrclr,
            Buserrclrmipi0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrmipi0::Mserrclr,
        buserrclrmipi0::Mserrclr,
        Buserrclrmipi0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrmipi0::Mserrclr,
            buserrclrmipi0::Mserrclr,
            Buserrclrmipi0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrmipi0 {
    #[inline(always)]
    fn default() -> Buserrclrmipi0 {
        <crate::RegValueT<Buserrclrmipi0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrmipi0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqenmipi0_SPEC;
impl crate::sealed::RegSpec for Busirqenmipi0_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqenmipi0 = crate::RegValueT<Busirqenmipi0_SPEC>;

impl Busirqenmipi0 {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqenmipi0::En,
        busirqenmipi0::En,
        Busirqenmipi0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqenmipi0::En,
            busirqenmipi0::En,
            Busirqenmipi0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqenmipi0 {
    #[inline(always)]
    fn default() -> Busirqenmipi0 {
        <crate::RegValueT<Busirqenmipi0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqenmipi0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatmipi1_SPEC;
impl crate::sealed::RegSpec for Buserrstatmipi1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatmipi1 = crate::RegValueT<Buserrstatmipi1_SPEC>;

impl Buserrstatmipi1 {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatmipi1::Slerrstat,
        buserrstatmipi1::Slerrstat,
        Buserrstatmipi1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatmipi1::Slerrstat,
            buserrstatmipi1::Slerrstat,
            Buserrstatmipi1_SPEC,
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
        buserrstatmipi1::Mmerrstat,
        buserrstatmipi1::Mmerrstat,
        Buserrstatmipi1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatmipi1::Mmerrstat,
            buserrstatmipi1::Mmerrstat,
            Buserrstatmipi1_SPEC,
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
        buserrstatmipi1::Ilerrstat,
        buserrstatmipi1::Ilerrstat,
        Buserrstatmipi1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatmipi1::Ilerrstat,
            buserrstatmipi1::Ilerrstat,
            Buserrstatmipi1_SPEC,
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
        buserrstatmipi1::Mserrstat,
        buserrstatmipi1::Mserrstat,
        Buserrstatmipi1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatmipi1::Mserrstat,
            buserrstatmipi1::Mserrstat,
            Buserrstatmipi1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatmipi1 {
    #[inline(always)]
    fn default() -> Buserrstatmipi1 {
        <crate::RegValueT<Buserrstatmipi1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatmipi1 {

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
pub struct Buserrclrmipi1_SPEC;
impl crate::sealed::RegSpec for Buserrclrmipi1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrmipi1 = crate::RegValueT<Buserrclrmipi1_SPEC>;

impl Buserrclrmipi1 {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrmipi1::Slerrclr,
        buserrclrmipi1::Slerrclr,
        Buserrclrmipi1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrmipi1::Slerrclr,
            buserrclrmipi1::Slerrclr,
            Buserrclrmipi1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrmipi1::Mmerrclr,
        buserrclrmipi1::Mmerrclr,
        Buserrclrmipi1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrmipi1::Mmerrclr,
            buserrclrmipi1::Mmerrclr,
            Buserrclrmipi1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrmipi1::Ilerrclr,
        buserrclrmipi1::Ilerrclr,
        Buserrclrmipi1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrmipi1::Ilerrclr,
            buserrclrmipi1::Ilerrclr,
            Buserrclrmipi1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrmipi1::Mserrclr,
        buserrclrmipi1::Mserrclr,
        Buserrclrmipi1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrmipi1::Mserrclr,
            buserrclrmipi1::Mserrclr,
            Buserrclrmipi1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrmipi1 {
    #[inline(always)]
    fn default() -> Buserrclrmipi1 {
        <crate::RegValueT<Buserrclrmipi1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrmipi1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqenmipi1_SPEC;
impl crate::sealed::RegSpec for Busirqenmipi1_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqenmipi1 = crate::RegValueT<Busirqenmipi1_SPEC>;

impl Busirqenmipi1 {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqenmipi1::En,
        busirqenmipi1::En,
        Busirqenmipi1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqenmipi1::En,
            busirqenmipi1::En,
            Busirqenmipi1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqenmipi1 {
    #[inline(always)]
    fn default() -> Busirqenmipi1 {
        <crate::RegValueT<Busirqenmipi1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqenmipi1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstatceu_SPEC;
impl crate::sealed::RegSpec for Buserrstatceu_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Status Register"]
pub type Buserrstatceu = crate::RegValueT<Buserrstatceu_SPEC>;

impl Buserrstatceu {
    #[doc = "Slave Bus Error Status"]
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstatceu::Slerrstat,
        buserrstatceu::Slerrstat,
        Buserrstatceu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstatceu::Slerrstat,
            buserrstatceu::Slerrstat,
            Buserrstatceu_SPEC,
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
        buserrstatceu::Mmerrstat,
        buserrstatceu::Mmerrstat,
        Buserrstatceu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstatceu::Mmerrstat,
            buserrstatceu::Mmerrstat,
            Buserrstatceu_SPEC,
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
        buserrstatceu::Ilerrstat,
        buserrstatceu::Ilerrstat,
        Buserrstatceu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstatceu::Ilerrstat,
            buserrstatceu::Ilerrstat,
            Buserrstatceu_SPEC,
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
        buserrstatceu::Mserrstat,
        buserrstatceu::Mserrstat,
        Buserrstatceu_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrstatceu::Mserrstat,
            buserrstatceu::Mserrstat,
            Buserrstatceu_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstatceu {
    #[inline(always)]
    fn default() -> Buserrstatceu {
        <crate::RegValueT<Buserrstatceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstatceu {

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
pub struct Buserrclrceu_SPEC;
impl crate::sealed::RegSpec for Buserrclrceu_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register"]
pub type Buserrclrceu = crate::RegValueT<Buserrclrceu_SPEC>;

impl Buserrclrceu {
    #[doc = "Slave Bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrclrceu::Slerrclr,
        buserrclrceu::Slerrclr,
        Buserrclrceu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrclrceu::Slerrclr,
            buserrclrceu::Slerrclr,
            Buserrclrceu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Error Clear"]
    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrclrceu::Mmerrclr,
        buserrclrceu::Mmerrclr,
        Buserrclrceu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrclrceu::Mmerrclr,
            buserrclrceu::Mmerrclr,
            Buserrclrceu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Address Access Error Clear"]
    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrclrceu::Ilerrclr,
        buserrclrceu::Ilerrclr,
        Buserrclrceu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrclrceu::Ilerrclr,
            buserrclrceu::Ilerrclr,
            Buserrclrceu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Security Attribution Unit Error Clear"]
    #[inline(always)]
    pub fn mserrclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        buserrclrceu::Mserrclr,
        buserrclrceu::Mserrclr,
        Buserrclrceu_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            buserrclrceu::Mserrclr,
            buserrclrceu::Mserrclr,
            Buserrclrceu_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrclrceu {
    #[inline(always)]
    fn default() -> Buserrclrceu {
        <crate::RegValueT<Buserrclrceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrclrceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrclr_SPEC;
    pub type Slerrclr = crate::EnumBitfieldStruct<u8, Slerrclr_SPEC>;
    impl Slerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.SLERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrclr_SPEC;
    pub type Mmerrclr = crate::EnumBitfieldStruct<u8, Mmerrclr_SPEC>;
    impl Mmerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MMERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrclr_SPEC;
    pub type Ilerrclr = crate::EnumBitfieldStruct<u8, Ilerrclr_SPEC>;
    impl Ilerrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.ILERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrclr_SPEC;
    pub type Mserrclr = crate::EnumBitfieldStruct<u8, Mserrclr_SPEC>;
    impl Mserrclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the BUSERRSTAT<Master Name>.MSERRSTAT flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busirqenceu_SPEC;
impl crate::sealed::RegSpec for Busirqenceu_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Interrupt Enable Register"]
pub type Busirqenceu = crate::RegValueT<Busirqenceu_SPEC>;

impl Busirqenceu {
    #[doc = "Interrupt enable after bus error detected"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busirqenceu::En,
        busirqenceu::En,
        Busirqenceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busirqenceu::En,
            busirqenceu::En,
            Busirqenceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busirqenceu {
    #[inline(always)]
    fn default() -> Busirqenceu {
        <crate::RegValueT<Busirqenceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busirqenceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
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
    pub fn mbwerr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mbwerrstat::Mbwerr9,
        mbwerrstat::Mbwerr9,
        Mbwerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mbwerrstat::Mbwerr9,
            mbwerrstat::Mbwerr9,
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
    pub struct Mbwerr9_SPEC;
    pub type Mbwerr9 = crate::EnumBitfieldStruct<u8, Mbwerr9_SPEC>;
    impl Mbwerr9 {
        #[doc = "No bufferable write error in Master #9"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bufferable write error occurs in Master #9"]
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
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr0,
        mbwerrclr::Mbweclr0,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr0,
            mbwerrclr::Mbweclr0,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr1,
        mbwerrclr::Mbweclr1,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr1,
            mbwerrclr::Mbweclr1,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr8,
        mbwerrclr::Mbweclr8,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr8,
            mbwerrclr::Mbweclr8,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr9,
        mbwerrclr::Mbweclr9,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr9,
            mbwerrclr::Mbweclr9,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr16,
        mbwerrclr::Mbweclr16,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr16,
            mbwerrclr::Mbweclr16,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr17,
        mbwerrclr::Mbweclr17,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr17,
            mbwerrclr::Mbweclr17,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr18,
        mbwerrclr::Mbweclr18,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr18,
            mbwerrclr::Mbweclr18,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr19,
        mbwerrclr::Mbweclr19,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr19,
            mbwerrclr::Mbweclr19,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr20,
        mbwerrclr::Mbweclr20,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr20,
            mbwerrclr::Mbweclr20,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Master Bufferable Write Error Clear"]
    #[inline(always)]
    pub fn mbweclr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mbwerrclr::Mbweclr21,
        mbwerrclr::Mbweclr21,
        Mbwerrclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mbwerrclr::Mbweclr21,
            mbwerrclr::Mbweclr21,
            Mbwerrclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mbwerrclr {
    #[inline(always)]
    fn default() -> Mbwerrclr {
        <crate::RegValueT<Mbwerrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mbwerrclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr0_SPEC;
    pub type Mbweclr0 = crate::EnumBitfieldStruct<u8, Mbweclr0_SPEC>;
    impl Mbweclr0 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR0 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr1_SPEC;
    pub type Mbweclr1 = crate::EnumBitfieldStruct<u8, Mbweclr1_SPEC>;
    impl Mbweclr1 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR1 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr8_SPEC;
    pub type Mbweclr8 = crate::EnumBitfieldStruct<u8, Mbweclr8_SPEC>;
    impl Mbweclr8 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR8 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr9_SPEC;
    pub type Mbweclr9 = crate::EnumBitfieldStruct<u8, Mbweclr9_SPEC>;
    impl Mbweclr9 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR9 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr16_SPEC;
    pub type Mbweclr16 = crate::EnumBitfieldStruct<u8, Mbweclr16_SPEC>;
    impl Mbweclr16 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the MBWERRSTAT.MBWERR16 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr17_SPEC;
    pub type Mbweclr17 = crate::EnumBitfieldStruct<u8, Mbweclr17_SPEC>;
    impl Mbweclr17 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR17 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr18_SPEC;
    pub type Mbweclr18 = crate::EnumBitfieldStruct<u8, Mbweclr18_SPEC>;
    impl Mbweclr18 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR18 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr19_SPEC;
    pub type Mbweclr19 = crate::EnumBitfieldStruct<u8, Mbweclr19_SPEC>;
    impl Mbweclr19 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR19 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr20_SPEC;
    pub type Mbweclr20 = crate::EnumBitfieldStruct<u8, Mbweclr20_SPEC>;
    impl Mbweclr20 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR20 flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbweclr21_SPEC;
    pub type Mbweclr21 = crate::EnumBitfieldStruct<u8, Mbweclr21_SPEC>;
    impl Mbweclr21 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the MBWERRSTAT.MBWERR21 flag"]
        pub const _1: Self = Self::new(1);
    }
}
