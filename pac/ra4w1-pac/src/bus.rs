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
// Generated from SVD 1.0, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:19 +0000

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
        4,
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

    #[doc = "CS%s Wait Control Register 1"]
    #[inline(always)]
    pub const fn cswcr1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cswcr1_SPEC, crate::common::RW>,
        4,
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

    #[doc = "CS%s Wait Control Register 2"]
    #[inline(always)]
    pub const fn cswcr2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cswcr2_SPEC, crate::common::RW>,
        4,
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
        4,
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

    #[doc = "CS%s Control Register"]
    #[inline(always)]
    pub const fn cscr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cscr_SPEC, crate::common::RW>,
        3,
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

    #[doc = "Master Bus Control Register %s"]
    #[inline(always)]
    pub const fn busmcnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }
    #[inline(always)]
    pub const fn busmcntm4i(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1000usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busmcntm4d(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1004usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busmcntsys(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1008usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busmcntdma(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100cusize),
            )
        }
    }

    #[doc = "Slave Bus Control Register FLI"]
    #[inline(always)]
    pub const fn busscntfli(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntfli_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntfli_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register P6B"]
    #[inline(always)]
    pub const fn busscntp6b(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntp6B_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntp6B_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4392usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register %s"]
    #[inline(always)]
    pub const fn busscnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Busscnt_SPEC, crate::common::RW>,
        3,
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

    #[doc = "Bus Error Address Register %s"]
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

    #[doc = "Bus Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        4,
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
    pub fn prdmod(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        csmod::Prdmod,
        csmod::Prdmod,
        Csmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            csmod::Prdmod,
            csmod::Prdmod,
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Csmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Csmod_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Prdmod_SPEC;
    pub type Prdmod = crate::EnumBitfieldStruct<u8, Prdmod_SPEC>;
    impl Prdmod {
        #[doc = "Normal access compatible mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "External data read continuous assertion mode"]
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
    pub struct Prenb_SPEC;
    pub type Prenb = crate::EnumBitfieldStruct<u8, Prenb_SPEC>;
    impl Prenb {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Cswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Cswcr1_SPEC,crate::common::RW>::from_register(self,0)
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

        #[doc = "Wait with a length of CSRWAIT clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cswwait_SPEC;
    pub type Cswwait = crate::EnumBitfieldStruct<u8, Cswwait_SPEC>;
    impl Cswwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_00: Self = Self::new(0);

        #[doc = "Wait with a length of CSWWAIT clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csprwait_SPEC;
    pub type Csprwait = crate::EnumBitfieldStruct<u8, Csprwait_SPEC>;
    impl Csprwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of CSPRWAIT clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cspwwait_SPEC;
    pub type Cspwwait = crate::EnumBitfieldStruct<u8, Cspwwait_SPEC>;
    impl Cspwwait {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of CSPWWAIT clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Cswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cswcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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

        #[doc = "Wait with a length of CSON clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdon_SPEC;
    pub type Wdon = crate::EnumBitfieldStruct<u8, Wdon_SPEC>;
    impl Wdon {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of WDON clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wron_SPEC;
    pub type Wron = crate::EnumBitfieldStruct<u8, Wron_SPEC>;
    impl Wron {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of WRON clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdon_SPEC;
    pub type Rdon = crate::EnumBitfieldStruct<u8, Rdon_SPEC>;
    impl Rdon {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of RDON clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Await_SPEC;
    pub type Await = crate::EnumBitfieldStruct<u8, Await_SPEC>;
    impl Await {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of AWAIT clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdoff_SPEC;
    pub type Wdoff = crate::EnumBitfieldStruct<u8, Wdoff_SPEC>;
    impl Wdoff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of WDOFF clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cswoff_SPEC;
    pub type Cswoff = crate::EnumBitfieldStruct<u8, Cswoff_SPEC>;
    impl Cswoff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of CSWOFF clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csroff_SPEC;
    pub type Csroff = crate::EnumBitfieldStruct<u8, Csroff_SPEC>;
    impl Csroff {
        #[doc = "No wait is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Wait with a length of CSROFF clock cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Cs0Cr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Cs0Cr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Separate bus interface is selected for area 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area 0."]
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
        #[doc = "16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Csrec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Csrec_SPEC,crate::common::RW>::from_register(self,0)
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

        #[doc = "WRCV recovery cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrcv_SPEC;
    pub type Rrcv = crate::EnumBitfieldStruct<u8, Rrcv_SPEC>;
    impl Rrcv {
        #[doc = "No recovery cycle is inserted."]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "RRCV recovery cycle is inserted."]
        pub const OTHERS: Self = Self::new(0);
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Cscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Cscr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Separate bus interface is selected for area 0."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address/data multiplexed I/O interface is selected for area 0."]
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
        #[doc = "16-bit bus space"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "8-bit bus space"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exenb_SPEC;
    pub type Exenb = crate::EnumBitfieldStruct<u8, Exenb_SPEC>;
    impl Exenb {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
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
    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    pub fn recvenm7(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        csrecen::Recvenm7,
        csrecen::Recvenm7,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            csrecen::Recvenm7,
            csrecen::Recvenm7,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    pub fn recvenm6(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        csrecen::Recvenm6,
        csrecen::Recvenm6,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            csrecen::Recvenm6,
            csrecen::Recvenm6,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    pub fn recvenm5(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        csrecen::Recvenm5,
        csrecen::Recvenm5,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            csrecen::Recvenm5,
            csrecen::Recvenm5,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    pub fn recvenm4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        csrecen::Recvenm4,
        csrecen::Recvenm4,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            csrecen::Recvenm4,
            csrecen::Recvenm4,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    pub fn recvenm3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        csrecen::Recvenm3,
        csrecen::Recvenm3,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            csrecen::Recvenm3,
            csrecen::Recvenm3,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    pub fn recvenm2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        csrecen::Recvenm2,
        csrecen::Recvenm2,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            csrecen::Recvenm2,
            csrecen::Recvenm2,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    pub fn recvenm1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        csrecen::Recvenm1,
        csrecen::Recvenm1,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            csrecen::Recvenm1,
            csrecen::Recvenm1,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multiplexed Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    pub fn recvenm0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        csrecen::Recvenm0,
        csrecen::Recvenm0,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            csrecen::Recvenm0,
            csrecen::Recvenm0,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 7"]
    #[inline(always)]
    pub fn recven7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        csrecen::Recven7,
        csrecen::Recven7,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            csrecen::Recven7,
            csrecen::Recven7,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 6"]
    #[inline(always)]
    pub fn recven6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        csrecen::Recven6,
        csrecen::Recven6,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            csrecen::Recven6,
            csrecen::Recven6,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 5"]
    #[inline(always)]
    pub fn recven5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        csrecen::Recven5,
        csrecen::Recven5,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            csrecen::Recven5,
            csrecen::Recven5,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 4"]
    #[inline(always)]
    pub fn recven4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        csrecen::Recven4,
        csrecen::Recven4,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            csrecen::Recven4,
            csrecen::Recven4,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 3"]
    #[inline(always)]
    pub fn recven3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        csrecen::Recven3,
        csrecen::Recven3,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            csrecen::Recven3,
            csrecen::Recven3,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 2"]
    #[inline(always)]
    pub fn recven2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        csrecen::Recven2,
        csrecen::Recven2,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            csrecen::Recven2,
            csrecen::Recven2,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 1"]
    #[inline(always)]
    pub fn recven1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        csrecen::Recven1,
        csrecen::Recven1,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            csrecen::Recven1,
            csrecen::Recven1,
            Csrecen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Separate Bus Recovery Cycle Insertion Enable 0"]
    #[inline(always)]
    pub fn recven0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        csrecen::Recven0,
        csrecen::Recven0,
        Csrecen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            csrecen::Recven0,
            csrecen::Recven0,
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
    pub struct Recvenm7_SPEC;
    pub type Recvenm7 = crate::EnumBitfieldStruct<u8, Recvenm7_SPEC>;
    impl Recvenm7 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recvenm6_SPEC;
    pub type Recvenm6 = crate::EnumBitfieldStruct<u8, Recvenm6_SPEC>;
    impl Recvenm6 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recvenm5_SPEC;
    pub type Recvenm5 = crate::EnumBitfieldStruct<u8, Recvenm5_SPEC>;
    impl Recvenm5 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recvenm4_SPEC;
    pub type Recvenm4 = crate::EnumBitfieldStruct<u8, Recvenm4_SPEC>;
    impl Recvenm4 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recvenm3_SPEC;
    pub type Recvenm3 = crate::EnumBitfieldStruct<u8, Recvenm3_SPEC>;
    impl Recvenm3 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recvenm2_SPEC;
    pub type Recvenm2 = crate::EnumBitfieldStruct<u8, Recvenm2_SPEC>;
    impl Recvenm2 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recvenm1_SPEC;
    pub type Recvenm1 = crate::EnumBitfieldStruct<u8, Recvenm1_SPEC>;
    impl Recvenm1 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recvenm0_SPEC;
    pub type Recvenm0 = crate::EnumBitfieldStruct<u8, Recvenm0_SPEC>;
    impl Recvenm0 {
        #[doc = "Recovery cycle insertion is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Recovery cycle insertion is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven7_SPEC;
    pub type Recven7 = crate::EnumBitfieldStruct<u8, Recven7_SPEC>;
    impl Recven7 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven6_SPEC;
    pub type Recven6 = crate::EnumBitfieldStruct<u8, Recven6_SPEC>;
    impl Recven6 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven5_SPEC;
    pub type Recven5 = crate::EnumBitfieldStruct<u8, Recven5_SPEC>;
    impl Recven5 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven4_SPEC;
    pub type Recven4 = crate::EnumBitfieldStruct<u8, Recven4_SPEC>;
    impl Recven4 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven3_SPEC;
    pub type Recven3 = crate::EnumBitfieldStruct<u8, Recven3_SPEC>;
    impl Recven3 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven2_SPEC;
    pub type Recven2 = crate::EnumBitfieldStruct<u8, Recven2_SPEC>;
    impl Recven2 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven1_SPEC;
    pub type Recven1 = crate::EnumBitfieldStruct<u8, Recven1_SPEC>;
    impl Recven1 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recven0_SPEC;
    pub type Recven0 = crate::EnumBitfieldStruct<u8, Recven0_SPEC>;
    impl Recven0 {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
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

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Busmcnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Busmcnt_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "A bus error is reported"]
        pub const _0: Self = Self::new(0);

        #[doc = "A bus error is not reported."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfli_SPEC;
impl crate::sealed::RegSpec for Busscntfli_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register FLI"]
pub type Busscntfli = crate::RegValueT<Busscntfli_SPEC>;

impl Busscntfli {
    #[doc = "Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntfli::Arbmet,
        busscntfli::Arbmet,
        Busscntfli_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntfli::Arbmet,
            busscntfli::Arbmet,
            Busscntfli_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Busscntfli_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Busscntfli_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntfli {
    #[inline(always)]
    fn default() -> Busscntfli {
        <crate::RegValueT<Busscntfli_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntfli {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntp6B_SPEC;
impl crate::sealed::RegSpec for Busscntp6B_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register P6B"]
pub type Busscntp6B = crate::RegValueT<Busscntp6B_SPEC>;

impl Busscntp6B {
    #[doc = "Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntp6b::Arbmet,
        busscntp6b::Arbmet,
        Busscntp6B_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntp6b::Arbmet,
            busscntp6b::Arbmet,
            Busscntp6B_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Busscntp6B_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Busscntp6B_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntp6B {
    #[inline(always)]
    fn default() -> Busscntp6B {
        <crate::RegValueT<Busscntp6B_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntp6b {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Busscnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Busscnt_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
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

    #[doc = "These bits are read as 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Buserrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Buserrstat_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Error Access StatusThe status at the time of the error"]
    #[inline(always)]
    pub fn accstst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstat::Accstst,
        buserrstat::Accstst,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstat::Accstst,
            buserrstat::Accstst,
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

        #[doc = "Bus error occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Accstst_SPEC;
    pub type Accstst = crate::EnumBitfieldStruct<u8, Accstst_SPEC>;
    impl Accstst {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write Access"]
        pub const _1: Self = Self::new(1);
    }
}
