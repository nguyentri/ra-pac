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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:19:53 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Bus Slave MPU"]
unsafe impl ::core::marker::Send for super::Smpu {}
unsafe impl ::core::marker::Sync for super::Smpu {}
impl super::Smpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn smpuctl(
        &self,
    ) -> &'static crate::common::Reg<self::Smpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smpumbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpumbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpumbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smpufbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpufbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpufbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smpusram(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Smpusram_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18usize))
        }
    }

    #[inline(always)]
    pub const fn smpupbiu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Smpupbiu_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }

    #[inline(always)]
    pub const fn smpuexbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpuexbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpuexbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smpuexbiu2(
        &self,
    ) -> &'static crate::common::Reg<self::Smpuexbiu2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpuexbiu2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpuctl_SPEC;
impl crate::sealed::RegSpec for Smpuctl_SPEC {
    type DataType = u16;
}

pub type Smpuctl = crate::RegValueT<Smpuctl_SPEC>;

impl Smpuctl {
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        smpuctl::Key,
        smpuctl::Key,
        Smpuctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            smpuctl::Key,
            smpuctl::Key,
            Smpuctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Smpuctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Smpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpuctl::Protect,
        smpuctl::Protect,
        Smpuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpuctl::Protect,
            smpuctl::Protect,
            Smpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpuctl::Oad,
        smpuctl::Oad,
        Smpuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpuctl::Oad,
            smpuctl::Oad,
            Smpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpuctl {
    #[inline(always)]
    fn default() -> Smpuctl {
        <crate::RegValueT<Smpuctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        pub const _0_X_A_5: Self = Self::new(165);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
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
pub struct Smpumbiu_SPEC;
impl crate::sealed::RegSpec for Smpumbiu_SPEC {
    type DataType = u16;
}

pub type Smpumbiu = crate::RegValueT<Smpumbiu_SPEC>;

impl Smpumbiu {
    #[inline(always)]
    pub fn wpsramhs(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        smpumbiu::Wpsramhs,
        smpumbiu::Wpsramhs,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            smpumbiu::Wpsramhs,
            smpumbiu::Wpsramhs,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpsramhs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        smpumbiu::Rpsramhs,
        smpumbiu::Rpsramhs,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            smpumbiu::Rpsramhs,
            smpumbiu::Rpsramhs,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wpfli(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        smpumbiu::Wpfli,
        smpumbiu::Wpfli,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            smpumbiu::Wpfli,
            smpumbiu::Wpfli,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpfli(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        smpumbiu::Rpfli,
        smpumbiu::Rpfli,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            smpumbiu::Rpfli,
            smpumbiu::Rpfli,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpumbiu::Wpgrpa,
        smpumbiu::Wpgrpa,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpumbiu::Wpgrpa,
            smpumbiu::Wpgrpa,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpumbiu::Rpgrpa,
        smpumbiu::Rpgrpa,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpumbiu::Rpgrpa,
            smpumbiu::Rpgrpa,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Smpumbiu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Smpumbiu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Smpumbiu {
    #[inline(always)]
    fn default() -> Smpumbiu {
        <crate::RegValueT<Smpumbiu_SPEC> as RegisterValue<_>>::new(8192)
    }
}
pub mod smpumbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpsramhs_SPEC;
    pub type Wpsramhs = crate::EnumBitfieldStruct<u8, Wpsramhs_SPEC>;
    impl Wpsramhs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpsramhs_SPEC;
    pub type Rpsramhs = crate::EnumBitfieldStruct<u8, Rpsramhs_SPEC>;
    impl Rpsramhs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpfli_SPEC;
    pub type Wpfli = crate::EnumBitfieldStruct<u8, Wpfli_SPEC>;
    impl Wpfli {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpfli_SPEC;
    pub type Rpfli = crate::EnumBitfieldStruct<u8, Rpfli_SPEC>;
    impl Rpfli {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpufbiu_SPEC;
impl crate::sealed::RegSpec for Smpufbiu_SPEC {
    type DataType = u16;
}

pub type Smpufbiu = crate::RegValueT<Smpufbiu_SPEC>;

impl Smpufbiu {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpufbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpufbiu_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpufbiu::Wpgrpa,
        smpufbiu::Wpgrpa,
        Smpufbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpufbiu::Wpgrpa,
            smpufbiu::Wpgrpa,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpufbiu::Rpgrpa,
        smpufbiu::Rpgrpa,
        Smpufbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpufbiu::Rpgrpa,
            smpufbiu::Rpgrpa,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpufbiu::Wpcpu,
        smpufbiu::Wpcpu,
        Smpufbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpufbiu::Wpcpu,
            smpufbiu::Wpcpu,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpufbiu::Rpcpu,
        smpufbiu::Rpcpu,
        Smpufbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpufbiu::Rpcpu,
            smpufbiu::Rpcpu,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpufbiu {
    #[inline(always)]
    fn default() -> Smpufbiu {
        <crate::RegValueT<Smpufbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpufbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpusram_SPEC;
impl crate::sealed::RegSpec for Smpusram_SPEC {
    type DataType = u16;
}

pub type Smpusram = crate::RegValueT<Smpusram_SPEC>;

impl Smpusram {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpusram_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpusram_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpusram::Wpgrpa,
        smpusram::Wpgrpa,
        Smpusram_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpusram::Wpgrpa,
            smpusram::Wpgrpa,
            Smpusram_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpusram::Rpgrpa,
        smpusram::Rpgrpa,
        Smpusram_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpusram::Rpgrpa,
            smpusram::Rpgrpa,
            Smpusram_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpusram::Wpcpu,
        smpusram::Wpcpu,
        Smpusram_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpusram::Wpcpu,
            smpusram::Wpcpu,
            Smpusram_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpusram::Rpcpu,
        smpusram::Rpcpu,
        Smpusram_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpusram::Rpcpu,
            smpusram::Rpcpu,
            Smpusram_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpusram {
    #[inline(always)]
    fn default() -> Smpusram {
        <crate::RegValueT<Smpusram_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpusram {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpupbiu_SPEC;
impl crate::sealed::RegSpec for Smpupbiu_SPEC {
    type DataType = u16;
}

pub type Smpupbiu = crate::RegValueT<Smpupbiu_SPEC>;

impl Smpupbiu {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpupbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpupbiu_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpupbiu::Wpgrpa,
        smpupbiu::Wpgrpa,
        Smpupbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpupbiu::Wpgrpa,
            smpupbiu::Wpgrpa,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpupbiu::Rpgrpa,
        smpupbiu::Rpgrpa,
        Smpupbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpupbiu::Rpgrpa,
            smpupbiu::Rpgrpa,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpupbiu::Wpcpu,
        smpupbiu::Wpcpu,
        Smpupbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpupbiu::Wpcpu,
            smpupbiu::Wpcpu,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpupbiu::Rpcpu,
        smpupbiu::Rpcpu,
        Smpupbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpupbiu::Rpcpu,
            smpupbiu::Rpcpu,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpupbiu {
    #[inline(always)]
    fn default() -> Smpupbiu {
        <crate::RegValueT<Smpupbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpupbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpuexbiu_SPEC;
impl crate::sealed::RegSpec for Smpuexbiu_SPEC {
    type DataType = u16;
}

pub type Smpuexbiu = crate::RegValueT<Smpuexbiu_SPEC>;

impl Smpuexbiu {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpuexbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpuexbiu_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wp_grpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpuexbiu::WpGrpa,
        smpuexbiu::WpGrpa,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpuexbiu::WpGrpa,
            smpuexbiu::WpGrpa,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rp_grpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpuexbiu::RpGrpa,
        smpuexbiu::RpGrpa,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpuexbiu::RpGrpa,
            smpuexbiu::RpGrpa,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wp_cpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpuexbiu::WpCpu,
        smpuexbiu::WpCpu,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpuexbiu::WpCpu,
            smpuexbiu::WpCpu,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rp_cpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpuexbiu::RpCpu,
        smpuexbiu::RpCpu,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpuexbiu::RpCpu,
            smpuexbiu::RpCpu,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpuexbiu {
    #[inline(always)]
    fn default() -> Smpuexbiu {
        <crate::RegValueT<Smpuexbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpuexbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct WpGrpa_SPEC;
    pub type WpGrpa = crate::EnumBitfieldStruct<u8, WpGrpa_SPEC>;
    impl WpGrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RpGrpa_SPEC;
    pub type RpGrpa = crate::EnumBitfieldStruct<u8, RpGrpa_SPEC>;
    impl RpGrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct WpCpu_SPEC;
    pub type WpCpu = crate::EnumBitfieldStruct<u8, WpCpu_SPEC>;
    impl WpCpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RpCpu_SPEC;
    pub type RpCpu = crate::EnumBitfieldStruct<u8, RpCpu_SPEC>;
    impl RpCpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpuexbiu2_SPEC;
impl crate::sealed::RegSpec for Smpuexbiu2_SPEC {
    type DataType = u16;
}

pub type Smpuexbiu2 = crate::RegValueT<Smpuexbiu2_SPEC>;

impl Smpuexbiu2 {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpuexbiu2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpuexbiu2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wp_grpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpuexbiu2::WpGrpa,
        smpuexbiu2::WpGrpa,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpuexbiu2::WpGrpa,
            smpuexbiu2::WpGrpa,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rp_grpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpuexbiu2::RpGrpa,
        smpuexbiu2::RpGrpa,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpuexbiu2::RpGrpa,
            smpuexbiu2::RpGrpa,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wp_cpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpuexbiu2::WpCpu,
        smpuexbiu2::WpCpu,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpuexbiu2::WpCpu,
            smpuexbiu2::WpCpu,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rp_cpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpuexbiu2::RpCpu,
        smpuexbiu2::RpCpu,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpuexbiu2::RpCpu,
            smpuexbiu2::RpCpu,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpuexbiu2 {
    #[inline(always)]
    fn default() -> Smpuexbiu2 {
        <crate::RegValueT<Smpuexbiu2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpuexbiu2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct WpGrpa_SPEC;
    pub type WpGrpa = crate::EnumBitfieldStruct<u8, WpGrpa_SPEC>;
    impl WpGrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RpGrpa_SPEC;
    pub type RpGrpa = crate::EnumBitfieldStruct<u8, RpGrpa_SPEC>;
    impl RpGrpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct WpCpu_SPEC;
    pub type WpCpu = crate::EnumBitfieldStruct<u8, WpCpu_SPEC>;
    impl WpCpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RpCpu_SPEC;
    pub type RpCpu = crate::EnumBitfieldStruct<u8, RpCpu_SPEC>;
    impl RpCpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
