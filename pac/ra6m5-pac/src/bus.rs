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
// Generated from SVD 1.41.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:06 +0000

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

    #[doc = "CS%s Control Register"]
    #[inline(always)]
    pub const fn cscr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cscr_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x802usize))
        }
    }
    #[inline(always)]
    pub const fn cs0cr(&self) -> &'static crate::common::Reg<self::Cscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x802usize),
            )
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

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscntfhbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntfhbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntfhbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscntflbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntflbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntflbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4356usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscnts0biu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnts0Biu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnts0Biu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4368usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscntpsbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntpsbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntpsbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4384usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscntplbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntplbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntplbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4400usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscntphbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntphbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntphbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4404usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscnteqbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnteqbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnteqbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4416usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscnteobiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnteobiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnteobiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4420usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register"]
    #[inline(always)]
    pub const fn busscntecbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntecbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntecbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4424usize),
            )
        }
    }

    #[doc = "BUS Error Address Register"]
    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        4,
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

    #[doc = "BUS Error Read Write Register"]
    #[inline(always)]
    pub const fn buserrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrrw_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1804usize))
        }
    }
    #[inline(always)]
    pub const fn bus1errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1804usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus2errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1814usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus3errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1824usize),
            )
        }
    }
    #[inline(always)]
    pub const fn bus4errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1834usize),
            )
        }
    }

    #[doc = "BUS TZF Error Address Register"]
    #[inline(always)]
    pub const fn btzferradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferradd_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1900usize))
        }
    }
    #[inline(always)]
    pub const fn btzf1erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1900usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf2erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1910usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf3erradd(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferradd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Btzferradd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1920usize),
            )
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

    #[doc = "BUS TZF Error Read Write Register"]
    #[inline(always)]
    pub const fn btzferrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferrrw_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1904usize))
        }
    }
    #[inline(always)]
    pub const fn btzf1errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1904usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf2errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1914usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf3errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1924usize),
            )
        }
    }
    #[inline(always)]
    pub const fn btzf4errrw(
        &self,
    ) -> &'static crate::common::Reg<self::Btzferrrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Btzferrrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1934usize),
            )
        }
    }

    #[doc = "BUS Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        4,
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

    #[doc = "BUS Error Clear Register %s"]
    #[inline(always)]
    pub const fn buserrclr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW>,
        4,
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

    #[doc = "DMAC/DTC Error Status Register"]
    #[inline(always)]
    pub const fn dmacdtcerrstat(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacdtcerrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dmacdtcerrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6692usize),
            )
        }
    }

    #[doc = "DMAC/DTC Error Clear Register"]
    #[inline(always)]
    pub const fn dmacdtcerrclr(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacdtcerrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacdtcerrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6700usize),
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
        #[doc = "External wait is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "External wait is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prenb_SPEC;
    pub type Prenb = crate::EnumBitfieldStruct<u8, Prenb_SPEC>;
    impl Prenb {
        #[doc = "Page read access is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Page read access is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pwenb_SPEC;
    pub type Pwenb = crate::EnumBitfieldStruct<u8, Pwenb_SPEC>;
    impl Pwenb {
        #[doc = "Page write access is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Page write access is enabled."]
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

#[doc = "CS%s Wait Control Register 2"]
pub type Cswcr2 = crate::RegValueT<Cswcr2_SPEC>;

impl Cswcr2 {
    #[doc = "Read-Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn csroff(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Cswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write-Access CS Extension Cycle Select"]
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
        #[doc = "Operation is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        #[doc = "A 16-bit bus space is selected."]
        pub const _00: Self = Self::new(0);

        #[doc = "An 8-bit bus space is selected."]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        #[doc = "Little-endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big-endian"]
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

        #[doc = "RRCV\\[3:0\\] clock cycles are inserted for read recovery."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrcv_SPEC;
    pub type Wrcv = crate::EnumBitfieldStruct<u8, Wrcv_SPEC>;
    impl Wrcv {
        #[doc = "No recovery cycle is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "WRCV\\[3:0\\] clock cycles are inserted for write recovery."]
        pub const OTHERS: Self = Self::new(0);
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
    pub struct Rcven2_SPEC;
    pub type Rcven2 = crate::EnumBitfieldStruct<u8, Rcven2_SPEC>;
    impl Rcven2 {
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
    pub struct Rcven4_SPEC;
    pub type Rcven4 = crate::EnumBitfieldStruct<u8, Rcven4_SPEC>;
    impl Rcven4 {
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
    pub struct Rcven6_SPEC;
    pub type Rcven6 = crate::EnumBitfieldStruct<u8, Rcven6_SPEC>;
    impl Rcven6 {
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
    pub struct Rcvenm0_SPEC;
    pub type Rcvenm0 = crate::EnumBitfieldStruct<u8, Rcvenm0_SPEC>;
    impl Rcvenm0 {
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
    pub struct Rcvenm2_SPEC;
    pub type Rcvenm2 = crate::EnumBitfieldStruct<u8, Rcvenm2_SPEC>;
    impl Rcvenm2 {
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
    pub struct Rcvenm4_SPEC;
    pub type Rcvenm4 = crate::EnumBitfieldStruct<u8, Rcvenm4_SPEC>;
    impl Rcvenm4 {
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
    pub struct Rcvenm6_SPEC;
    pub type Rcvenm6 = crate::EnumBitfieldStruct<u8, Rcvenm6_SPEC>;
    impl Rcvenm6 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvenm7_SPEC;
    pub type Rcvenm7 = crate::EnumBitfieldStruct<u8, Rcvenm7_SPEC>;
    impl Rcvenm7 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfhbiu_SPEC;
impl crate::sealed::RegSpec for Busscntfhbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscntfhbiu = crate::RegValueT<Busscntfhbiu_SPEC>;

impl Busscntfhbiu {
    #[doc = "Arbitration Select for three masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscntfhbiu::Arbs,
        busscntfhbiu::Arbs,
        Busscntfhbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscntfhbiu::Arbs,
            busscntfhbiu::Arbs,
            Busscntfhbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntfhbiu {
    #[inline(always)]
    fn default() -> Busscntfhbiu {
        <crate::RegValueT<Busscntfhbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntfhbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "EDMAC > DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "(EDMAC ↔ DMAC/DTC) > CPU"]
        pub const _10: Self = Self::new(2);

        #[doc = "(EDMAC ↔ DMAC/DTC) ↔ CPU"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntflbiu_SPEC;
impl crate::sealed::RegSpec for Busscntflbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscntflbiu = crate::RegValueT<Busscntflbiu_SPEC>;

impl Busscntflbiu {
    #[doc = "Arbitration Select for three masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscntflbiu::Arbs,
        busscntflbiu::Arbs,
        Busscntflbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscntflbiu::Arbs,
            busscntflbiu::Arbs,
            Busscntflbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntflbiu {
    #[inline(always)]
    fn default() -> Busscntflbiu {
        <crate::RegValueT<Busscntflbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntflbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "EDMAC > DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "(EDMAC ↔ DMAC/DTC) > CPU"]
        pub const _10: Self = Self::new(2);

        #[doc = "(EDMAC ↔ DMAC/DTC) ↔ CPU"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnts0Biu_SPEC;
impl crate::sealed::RegSpec for Busscnts0Biu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscnts0Biu = crate::RegValueT<Busscnts0Biu_SPEC>;

impl Busscnts0Biu {
    #[doc = "Arbitration Select for three masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscnts0biu::Arbs,
        busscnts0biu::Arbs,
        Busscnts0Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscnts0biu::Arbs,
            busscnts0biu::Arbs,
            Busscnts0Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscnts0Biu {
    #[inline(always)]
    fn default() -> Busscnts0Biu {
        <crate::RegValueT<Busscnts0Biu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscnts0biu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "EDMAC > DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "(EDMAC ↔ DMAC/DTC) > CPU"]
        pub const _10: Self = Self::new(2);

        #[doc = "(EDMAC ↔ DMAC/DTC) ↔ CPU"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntpsbiu_SPEC;
impl crate::sealed::RegSpec for Busscntpsbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscntpsbiu = crate::RegValueT<Busscntpsbiu_SPEC>;

impl Busscntpsbiu {
    #[doc = "Arbitration Select for two masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busscntpsbiu::Arbs,
        busscntpsbiu::Arbs,
        Busscntpsbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busscntpsbiu::Arbs,
            busscntpsbiu::Arbs,
            Busscntpsbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntpsbiu {
    #[inline(always)]
    fn default() -> Busscntpsbiu {
        <crate::RegValueT<Busscntpsbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntpsbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "DMAC/DTC > CPU"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC/DTC ↔ CPU"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntplbiu_SPEC;
impl crate::sealed::RegSpec for Busscntplbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscntplbiu = crate::RegValueT<Busscntplbiu_SPEC>;

impl Busscntplbiu {
    #[doc = "Arbitration Select for two masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busscntplbiu::Arbs,
        busscntplbiu::Arbs,
        Busscntplbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busscntplbiu::Arbs,
            busscntplbiu::Arbs,
            Busscntplbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntplbiu {
    #[inline(always)]
    fn default() -> Busscntplbiu {
        <crate::RegValueT<Busscntplbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntplbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "DMAC/DTC > CPU"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC/DTC ↔ CPU"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntphbiu_SPEC;
impl crate::sealed::RegSpec for Busscntphbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscntphbiu = crate::RegValueT<Busscntphbiu_SPEC>;

impl Busscntphbiu {
    #[doc = "Arbitration Select for two masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busscntphbiu::Arbs,
        busscntphbiu::Arbs,
        Busscntphbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busscntphbiu::Arbs,
            busscntphbiu::Arbs,
            Busscntphbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntphbiu {
    #[inline(always)]
    fn default() -> Busscntphbiu {
        <crate::RegValueT<Busscntphbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntphbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "DMAC/DTC > CPU"]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC/DTC ↔ CPU"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnteqbiu_SPEC;
impl crate::sealed::RegSpec for Busscnteqbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscnteqbiu = crate::RegValueT<Busscnteqbiu_SPEC>;

impl Busscnteqbiu {
    #[doc = "Arbitration Select for three masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscnteqbiu::Arbs,
        busscnteqbiu::Arbs,
        Busscnteqbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscnteqbiu::Arbs,
            busscnteqbiu::Arbs,
            Busscnteqbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscnteqbiu {
    #[inline(always)]
    fn default() -> Busscnteqbiu {
        <crate::RegValueT<Busscnteqbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscnteqbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "EDMAC > DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "(EDMAC ↔ DMAC/DTC) > CPU"]
        pub const _10: Self = Self::new(2);

        #[doc = "(EDMAC ↔ DMAC/DTC) ↔ CPU"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnteobiu_SPEC;
impl crate::sealed::RegSpec for Busscnteobiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscnteobiu = crate::RegValueT<Busscnteobiu_SPEC>;

impl Busscnteobiu {
    #[doc = "Arbitration Select for three masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscnteobiu::Arbs,
        busscnteobiu::Arbs,
        Busscnteobiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscnteobiu::Arbs,
            busscnteobiu::Arbs,
            Busscnteobiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscnteobiu {
    #[inline(always)]
    fn default() -> Busscnteobiu {
        <crate::RegValueT<Busscnteobiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscnteobiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "EDMAC > DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "(EDMAC ↔ DMAC/DTC) > CPU"]
        pub const _10: Self = Self::new(2);

        #[doc = "(EDMAC ↔ DMAC/DTC) ↔ CPU"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntecbiu_SPEC;
impl crate::sealed::RegSpec for Busscntecbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscntecbiu = crate::RegValueT<Busscntecbiu_SPEC>;

impl Busscntecbiu {
    #[doc = "Arbitration Select for three masters"]
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscntecbiu::Arbs,
        busscntecbiu::Arbs,
        Busscntecbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscntecbiu::Arbs,
            busscntecbiu::Arbs,
            Busscntecbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntecbiu {
    #[inline(always)]
    fn default() -> Busscntecbiu {
        <crate::RegValueT<Busscntecbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntecbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        #[doc = "EDMAC > DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "(EDMAC ↔ DMAC/DTC) > CPU"]
        pub const _10: Self = Self::new(2);

        #[doc = "(EDMAC ↔ DMAC/DTC) ↔ CPU"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserradd_SPEC;
impl crate::sealed::RegSpec for Buserradd_SPEC {
    type DataType = u32;
}

#[doc = "BUS Error Address Register"]
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
pub struct Buserrrw_SPEC;
impl crate::sealed::RegSpec for Buserrrw_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Read Write Register"]
pub type Buserrrw = crate::RegValueT<Buserrrw_SPEC>;

impl Buserrrw {
    #[doc = "Error Access Read/Write Status"]
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
pub struct Btzferradd_SPEC;
impl crate::sealed::RegSpec for Btzferradd_SPEC {
    type DataType = u32;
}

#[doc = "BUS TZF Error Address Register"]
pub type Btzferradd = crate::RegValueT<Btzferradd_SPEC>;

impl Btzferradd {
    #[doc = "Bus TrustZone Filter Error Address"]
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

#[doc = "BUS TZF Error Read Write Register"]
pub type Btzferrrw = crate::RegValueT<Btzferrrw_SPEC>;

impl Btzferrrw {
    #[doc = "TrustZone filter error access Read/Write Status"]
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

#[doc = "BUS Error Status Register %s"]
pub type Buserrstat = crate::RegValueT<Buserrstat_SPEC>;

impl Buserrstat {
    #[doc = "Slave bus Error Status"]
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

    #[doc = "Slave TrustZone filter Error Status"]
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

    #[doc = "Illegal address access Error Status"]
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrclr_SPEC;
impl crate::sealed::RegSpec for Buserrclr_SPEC {
    type DataType = u8;
}

#[doc = "BUS Error Clear Register %s"]
pub type Buserrclr = crate::RegValueT<Buserrclr_SPEC>;

impl Buserrclr {
    #[doc = "Slave bus Error Clear"]
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slave TrustZone filter Error Clear"]
    #[inline(always)]
    pub fn sterrclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for Buserrclr {
    #[inline(always)]
    fn default() -> Buserrclr {
        <crate::RegValueT<Buserrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacdtcerrstat_SPEC;
impl crate::sealed::RegSpec for Dmacdtcerrstat_SPEC {
    type DataType = u8;
}

#[doc = "DMAC/DTC Error Status Register"]
pub type Dmacdtcerrstat = crate::RegValueT<Dmacdtcerrstat_SPEC>;

impl Dmacdtcerrstat {
    #[doc = "Master TrustZone Filter Error Status"]
    #[inline(always)]
    pub fn mterrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacdtcerrstat::Mterrstat,
        dmacdtcerrstat::Mterrstat,
        Dmacdtcerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacdtcerrstat::Mterrstat,
            dmacdtcerrstat::Mterrstat,
            Dmacdtcerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmacdtcerrstat {
    #[inline(always)]
    fn default() -> Dmacdtcerrstat {
        <crate::RegValueT<Dmacdtcerrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmacdtcerrstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mterrstat_SPEC;
    pub type Mterrstat = crate::EnumBitfieldStruct<u8, Mterrstat_SPEC>;
    impl Mterrstat {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacdtcerrclr_SPEC;
impl crate::sealed::RegSpec for Dmacdtcerrclr_SPEC {
    type DataType = u8;
}

#[doc = "DMAC/DTC Error Clear Register"]
pub type Dmacdtcerrclr = crate::RegValueT<Dmacdtcerrclr_SPEC>;

impl Dmacdtcerrclr {
    #[doc = "Master TrustZone filter Error Clear"]
    #[inline(always)]
    pub fn mterrclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dmacdtcerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dmacdtcerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmacdtcerrclr {
    #[inline(always)]
    fn default() -> Dmacdtcerrclr {
        <crate::RegValueT<Dmacdtcerrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
