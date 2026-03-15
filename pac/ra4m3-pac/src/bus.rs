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
// Generated from SVD 1.40.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:07:45 +0000

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

    #[doc = "BUS Error Address Register"]
    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        3,
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

    #[doc = "BUS Error Read Write Register"]
    #[inline(always)]
    pub const fn buserrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrrw_SPEC, crate::common::RW>,
        3,
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

    #[doc = "BUS TZF Error Address Register"]
    #[inline(always)]
    pub const fn btzferradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferradd_SPEC, crate::common::R>,
        3,
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

    #[doc = "BUS TZF Error Read Write Register"]
    #[inline(always)]
    pub const fn btzferrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferrrw_SPEC, crate::common::RW>,
        3,
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

    #[doc = "BUS Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        3,
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

    #[doc = "BUS Error Clear Register %s"]
    #[inline(always)]
    pub const fn buserrclr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW>,
        3,
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
pub struct Busscntfhbiu_SPEC;
impl crate::sealed::RegSpec for Busscntfhbiu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register"]
pub type Busscntfhbiu = crate::RegValueT<Busscntfhbiu_SPEC>;

impl Busscntfhbiu {
    #[doc = "Arbitration Select for two masters"]
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
        #[doc = "DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "DMAC/DTC ↔ CPU"]
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
    #[doc = "Arbitration Select for two masters"]
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
        #[doc = "DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "DMAC/DTC ↔ CPU"]
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
    #[doc = "Arbitration Select for two masters"]
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
        #[doc = "DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "DMAC/DTC ↔ CPU"]
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
    #[doc = "Arbitration Select for two masters"]
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
        #[doc = "DMAC/DTC > CPU"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);

        #[doc = "DMAC/DTC ↔ CPU"]
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
