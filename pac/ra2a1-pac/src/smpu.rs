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
// Generated from SVD 1.1, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:31 +0000

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

    #[doc = "Slave MPU Control Register"]
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

    #[doc = "Access Control Register for MBIU"]
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

    #[doc = "Access Control Register for FBIU"]
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

    #[doc = "Access Control Register for SRAM"]
    #[inline(always)]
    pub const fn smpusram0(
        &self,
    ) -> &'static crate::common::Reg<self::Smpusram0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpusram0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Access Control Register for P%sBIU"]
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
    pub const fn smpup0biu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpupbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpupbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
    #[inline(always)]
    pub const fn smpup2biu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpupbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpupbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x24usize),
            )
        }
    }
    #[inline(always)]
    pub const fn smpup6biu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpupbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpupbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
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

#[doc = "Slave MPU Control Register"]
pub type Smpuctl = crate::RegValueT<Smpuctl_SPEC>;

impl Smpuctl {
    #[doc = "Key Code This bit is used to enable or disable writing of the PROTECT and OAD bit."]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Smpuctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Smpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Protection of register"]
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

    #[doc = "Operation after detection"]
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
        #[doc = "Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Slave register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "All Bus Slave register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpumbiu_SPEC;
impl crate::sealed::RegSpec for Smpumbiu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for MBIU"]
pub type Smpumbiu = crate::RegValueT<Smpumbiu_SPEC>;

impl Smpumbiu {
    #[doc = "Master Group A Write protection"]
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

    #[doc = "Master Group A Read protection"]
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

    #[doc = "These bits are read as 00. The write value should be 00."]
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
        <crate::RegValueT<Smpumbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpumbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A write of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A read of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpufbiu_SPEC;
impl crate::sealed::RegSpec for Smpufbiu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for FBIU"]
pub type Smpufbiu = crate::RegValueT<Smpufbiu_SPEC>;

impl Smpufbiu {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpufbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpufbiu_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Group A Write protection"]
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

    #[doc = "Master Group A Read protection"]
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

    #[doc = "CPU Write protection"]
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

    #[doc = "CPU Read protection"]
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
        #[doc = "Master group A write of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A write of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A read of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU write of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU read of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpusram0_SPEC;
impl crate::sealed::RegSpec for Smpusram0_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for SRAM"]
pub type Smpusram0 = crate::RegValueT<Smpusram0_SPEC>;

impl Smpusram0 {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpusram0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpusram0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpusram0::Wpgrpa,
        smpusram0::Wpgrpa,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpusram0::Wpgrpa,
            smpusram0::Wpgrpa,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpusram0::Rpgrpa,
        smpusram0::Rpgrpa,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpusram0::Rpgrpa,
            smpusram0::Rpgrpa,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpusram0::Wpcpu,
        smpusram0::Wpcpu,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpusram0::Wpcpu,
            smpusram0::Wpcpu,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpusram0::Rpcpu,
        smpusram0::Rpcpu,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpusram0::Rpcpu,
            smpusram0::Rpcpu,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpusram0 {
    #[inline(always)]
    fn default() -> Smpusram0 {
        <crate::RegValueT<Smpusram0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpusram0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A write of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A read of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU write of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU read of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpupbiu_SPEC;
impl crate::sealed::RegSpec for Smpupbiu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for P%sBIU"]
pub type Smpupbiu = crate::RegValueT<Smpupbiu_SPEC>;

impl Smpupbiu {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Smpupbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Smpupbiu_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Master Group A Write protection"]
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

    #[doc = "Master Group A Read protection"]
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

    #[doc = "CPU Write protection"]
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

    #[doc = "CPU Read protection"]
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
        #[doc = "Master group A write of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A write of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A read of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU write of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU read of memory protection is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
