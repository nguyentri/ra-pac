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
// Generated from SVD 1.00.01, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:22:22 +0000

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

    #[inline(always)]
    pub const fn cs0cr(&self) -> &'static crate::common::Reg<self::Cs0Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs0Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2050usize),
            )
        }
    }

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
    pub const fn busoad(
        &self,
    ) -> &'static crate::common::Reg<self::Busoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn msapt(&self) -> &'static crate::common::Reg<self::Msapt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msapt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn buserrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrrw_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1804usize))
        }
    }

    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1830usize))
        }
    }

    #[inline(always)]
    pub const fn bmsaerradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bmsaerradd_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1900usize))
        }
    }

    #[inline(always)]
    pub const fn bmsaerrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Bmsaerrrw_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1904usize))
        }
    }

    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Buserrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6800usize),
            )
        }
    }

    #[inline(always)]
    pub const fn buserrclr(
        &self,
    ) -> &'static crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buserrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6808usize),
            )
        }
    }

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
pub struct Cs0Cr_SPEC;
impl crate::sealed::RegSpec for Cs0Cr_SPEC {
    type DataType = u16;
}

pub type Cs0Cr = crate::RegValueT<Cs0Cr_SPEC>;

impl Cs0Cr {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        pub const _00: Self = Self::new(0);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpxen_SPEC;
    pub type Mpxen = crate::EnumBitfieldStruct<u8, Mpxen_SPEC>;
    impl Mpxen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cscr_SPEC;
impl crate::sealed::RegSpec for Cscr_SPEC {
    type DataType = u16;
}

pub type Cscr = crate::RegValueT<Cscr_SPEC>;

impl Cscr {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsize_SPEC;
    pub type Bsize = crate::EnumBitfieldStruct<u8, Bsize_SPEC>;
    impl Bsize {
        pub const _00: Self = Self::new(0);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Emode_SPEC;
    pub type Emode = crate::EnumBitfieldStruct<u8, Emode_SPEC>;
    impl Emode {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpxen_SPEC;
    pub type Mpxen = crate::EnumBitfieldStruct<u8, Mpxen_SPEC>;
    impl Mpxen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busoad_SPEC;
impl crate::sealed::RegSpec for Busoad_SPEC {
    type DataType = u16;
}

pub type Busoad = crate::RegValueT<Busoad_SPEC>;

impl Busoad {
    #[inline(always)]
    pub fn ilerroad(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Busoad_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Busoad_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn slerroad(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Busoad_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Busoad_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

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

pub type Busoadpt = crate::RegValueT<Busoadpt_SPEC>;

impl Busoadpt {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msaoad_SPEC;
impl crate::sealed::RegSpec for Msaoad_SPEC {
    type DataType = u16;
}

pub type Msaoad = crate::RegValueT<Msaoad_SPEC>;

impl Msaoad {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msapt_SPEC;
impl crate::sealed::RegSpec for Msapt_SPEC {
    type DataType = u16;
}

pub type Msapt = crate::RegValueT<Msapt_SPEC>;

impl Msapt {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt1Fhbi_SPEC;
impl crate::sealed::RegSpec for Bussabt1Fhbi_SPEC {
    type DataType = u32;
}

pub type Bussabt1Fhbi = crate::RegValueT<Bussabt1Fhbi_SPEC>;

impl Bussabt1Fhbi {
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
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Flbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Flbi_SPEC {
    type DataType = u32;
}

pub type Bussabt0Flbi = crate::RegValueT<Bussabt0Flbi_SPEC>;

impl Bussabt0Flbi {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt1S1Bi_SPEC;
impl crate::sealed::RegSpec for Bussabt1S1Bi_SPEC {
    type DataType = u32;
}

pub type Bussabt1S1Bi = crate::RegValueT<Bussabt1S1Bi_SPEC>;

impl Bussabt1S1Bi {
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
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Stbysbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Stbysbi_SPEC {
    type DataType = u32;
}

pub type Bussabt0Stbysbi = crate::RegValueT<Bussabt0Stbysbi_SPEC>;

impl Bussabt0Stbysbi {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Eobi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Eobi_SPEC {
    type DataType = u32;
}

pub type Bussabt0Eobi = crate::RegValueT<Bussabt0Eobi_SPEC>;

impl Bussabt0Eobi {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Pbbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pbbi_SPEC {
    type DataType = u32;
}

pub type Bussabt0Pbbi = crate::RegValueT<Bussabt0Pbbi_SPEC>;

impl Bussabt0Pbbi {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Pabi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pabi_SPEC {
    type DataType = u32;
}

pub type Bussabt0Pabi = crate::RegValueT<Bussabt0Pabi_SPEC>;

impl Bussabt0Pabi {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Pibi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Pibi_SPEC {
    type DataType = u32;
}

pub type Bussabt0Pibi = crate::RegValueT<Bussabt0Pibi_SPEC>;

impl Bussabt0Pibi {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussabt0Psbi_SPEC;
impl crate::sealed::RegSpec for Bussabt0Psbi_SPEC {
    type DataType = u32;
}

pub type Bussabt0Psbi = crate::RegValueT<Bussabt0Psbi_SPEC>;

impl Bussabt0Psbi {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busdivbyp_SPEC;
impl crate::sealed::RegSpec for Busdivbyp_SPEC {
    type DataType = u32;
}

pub type Busdivbyp = crate::RegValueT<Busdivbyp_SPEC>;

impl Busdivbyp {
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
    pub struct Edmabpe_SPEC;
    pub type Edmabpe = crate::EnumBitfieldStruct<u8, Edmabpe_SPEC>;
    impl Edmabpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpu0Sbpe_SPEC;
    pub type Cpu0Sbpe = crate::EnumBitfieldStruct<u8, Cpu0Sbpe_SPEC>;
    impl Cpu0Sbpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrrw_SPEC;
impl crate::sealed::RegSpec for Buserrrw_SPEC {
    type DataType = u8;
}

pub type Buserrrw = crate::RegValueT<Buserrrw_SPEC>;

impl Buserrrw {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserradd_SPEC;
impl crate::sealed::RegSpec for Buserradd_SPEC {
    type DataType = u32;
}

pub type Buserradd = crate::RegValueT<Buserradd_SPEC>;

impl Buserradd {
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

pub type Bmsaerradd = crate::RegValueT<Bmsaerradd_SPEC>;

impl Bmsaerradd {
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

pub type Bmsaerrrw = crate::RegValueT<Bmsaerrrw_SPEC>;

impl Bmsaerrrw {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstat_SPEC;
impl crate::sealed::RegSpec for Buserrstat_SPEC {
    type DataType = u8;
}

pub type Buserrstat = crate::RegValueT<Buserrstat_SPEC>;

impl Buserrstat {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrstat_SPEC;
    pub type Mmerrstat = crate::EnumBitfieldStruct<u8, Mmerrstat_SPEC>;
    impl Mmerrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrstat_SPEC;
    pub type Ilerrstat = crate::EnumBitfieldStruct<u8, Ilerrstat_SPEC>;
    impl Ilerrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mserrstat_SPEC;
    pub type Mserrstat = crate::EnumBitfieldStruct<u8, Mserrstat_SPEC>;
    impl Mserrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrclr_SPEC;
impl crate::sealed::RegSpec for Buserrclr_SPEC {
    type DataType = u8;
}

pub type Buserrclr = crate::RegValueT<Buserrclr_SPEC>;

impl Buserrclr {
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Mbwerrstat = crate::RegValueT<Mbwerrstat_SPEC>;

impl Mbwerrstat {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr1_SPEC;
    pub type Mbwerr1 = crate::EnumBitfieldStruct<u8, Mbwerr1_SPEC>;
    impl Mbwerr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr8_SPEC;
    pub type Mbwerr8 = crate::EnumBitfieldStruct<u8, Mbwerr8_SPEC>;
    impl Mbwerr8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr16_SPEC;
    pub type Mbwerr16 = crate::EnumBitfieldStruct<u8, Mbwerr16_SPEC>;
    impl Mbwerr16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbwerr21_SPEC;
    pub type Mbwerr21 = crate::EnumBitfieldStruct<u8, Mbwerr21_SPEC>;
    impl Mbwerr21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbwerrclr_SPEC;
impl crate::sealed::RegSpec for Mbwerrclr_SPEC {
    type DataType = u32;
}

pub type Mbwerrclr = crate::RegValueT<Mbwerrclr_SPEC>;

impl Mbwerrclr {
    #[inline(always)]
    pub fn mbweclr0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mbweclr1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mbweclr8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mbweclr16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mbweclr21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Mbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Mbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Sbwerrstat = crate::RegValueT<Sbwerrstat_SPEC>;

impl Sbwerrstat {
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr1_SPEC;
    pub type Sbwerr1 = crate::EnumBitfieldStruct<u8, Sbwerr1_SPEC>;
    impl Sbwerr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr2_SPEC;
    pub type Sbwerr2 = crate::EnumBitfieldStruct<u8, Sbwerr2_SPEC>;
    impl Sbwerr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr4_SPEC;
    pub type Sbwerr4 = crate::EnumBitfieldStruct<u8, Sbwerr4_SPEC>;
    impl Sbwerr4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr5_SPEC;
    pub type Sbwerr5 = crate::EnumBitfieldStruct<u8, Sbwerr5_SPEC>;
    impl Sbwerr5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr7_SPEC;
    pub type Sbwerr7 = crate::EnumBitfieldStruct<u8, Sbwerr7_SPEC>;
    impl Sbwerr7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr8_SPEC;
    pub type Sbwerr8 = crate::EnumBitfieldStruct<u8, Sbwerr8_SPEC>;
    impl Sbwerr8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr9_SPEC;
    pub type Sbwerr9 = crate::EnumBitfieldStruct<u8, Sbwerr9_SPEC>;
    impl Sbwerr9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr10_SPEC;
    pub type Sbwerr10 = crate::EnumBitfieldStruct<u8, Sbwerr10_SPEC>;
    impl Sbwerr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr11_SPEC;
    pub type Sbwerr11 = crate::EnumBitfieldStruct<u8, Sbwerr11_SPEC>;
    impl Sbwerr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbwerr12_SPEC;
    pub type Sbwerr12 = crate::EnumBitfieldStruct<u8, Sbwerr12_SPEC>;
    impl Sbwerr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbwerrclr_SPEC;
impl crate::sealed::RegSpec for Sbwerrclr_SPEC {
    type DataType = u32;
}

pub type Sbwerrclr = crate::RegValueT<Sbwerrclr_SPEC>;

impl Sbwerrclr {
    #[inline(always)]
    pub fn sbweclr0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbweclr11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Sbwerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Sbwerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
