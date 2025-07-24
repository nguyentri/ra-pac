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
// Generated from SVD 1.0, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CPU Stack Pointer Monitor"]
unsafe impl ::core::marker::Send for super::Spmon {}
unsafe impl ::core::marker::Sync for super::Spmon {}
impl super::Spmon {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Stack Pointer Monitor Operation After Detection Register"]
    #[inline(always)]
    pub const fn mspmpuoad(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpuoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpuoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Stack Pointer Monitor Access Control Register"]
    #[inline(always)]
    pub const fn mspmpuctl(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Stack Pointer Monitor Protection Register"]
    #[inline(always)]
    pub const fn mspmpupt(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpupt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpupt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Main Stack Pointer Monitor Start Address Register"]
    #[inline(always)]
    pub const fn mspmpusa(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Main Stack Pointer Monitor End Address Register"]
    #[inline(always)]
    pub const fn mspmpuea(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Stack Pointer Monitor Operation After Detection Register"]
    #[inline(always)]
    pub const fn pspmpuoad(
        &self,
    ) -> &'static crate::common::Reg<self::Pspmpuoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pspmpuoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Stack Pointer Monitor Access Control Register"]
    #[inline(always)]
    pub const fn pspmpuctl(
        &self,
    ) -> &'static crate::common::Reg<self::Pspmpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pspmpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Stack Pointer Monitor Protection Register"]
    #[inline(always)]
    pub const fn pspmpupt(
        &self,
    ) -> &'static crate::common::Reg<self::Pspmpupt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pspmpupt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Process Stack Pointer Monitor Start Address Register"]
    #[inline(always)]
    pub const fn pspmpusa(
        &self,
    ) -> &'static crate::common::Reg<self::Pspmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pspmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Process Stack Pointer Monitor End Address Register"]
    #[inline(always)]
    pub const fn pspmpuea(
        &self,
    ) -> &'static crate::common::Reg<self::Pspmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pspmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpuoad_SPEC;
impl crate::sealed::RegSpec for Mspmpuoad_SPEC {
    type DataType = u16;
}

#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub type Mspmpuoad = crate::RegValueT<Mspmpuoad_SPEC>;

impl Mspmpuoad {
    #[doc = "Write Keyword \nThe data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        mspmpuoad::Key,
        mspmpuoad::Key,
        Mspmpuoad_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            mspmpuoad::Key,
            mspmpuoad::Key,
            Mspmpuoad_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mspmpuoad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mspmpuoad_SPEC,crate::common::RW>::from_register(self,0)
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
        mspmpuoad::Oad,
        mspmpuoad::Oad,
        Mspmpuoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mspmpuoad::Oad,
            mspmpuoad::Oad,
            Mspmpuoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mspmpuoad {
    #[inline(always)]
    fn default() -> Mspmpuoad {
        <crate::RegValueT<Mspmpuoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mspmpuoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);

        #[doc = "Writing to the OAD bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);

        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpuctl_SPEC;
impl crate::sealed::RegSpec for Mspmpuctl_SPEC {
    type DataType = u16;
}

#[doc = "Stack Pointer Monitor Access Control Register"]
pub type Mspmpuctl = crate::RegValueT<Mspmpuctl_SPEC>;

impl Mspmpuctl {
    #[doc = "Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub fn error(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mspmpuctl::Error,
        mspmpuctl::Error,
        Mspmpuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mspmpuctl::Error,
            mspmpuctl::Error,
            Mspmpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mspmpuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mspmpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mspmpuctl::Enable,
        mspmpuctl::Enable,
        Mspmpuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mspmpuctl::Enable,
            mspmpuctl::Enable,
            Mspmpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mspmpuctl {
    #[inline(always)]
    fn default() -> Mspmpuctl {
        <crate::RegValueT<Mspmpuctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mspmpuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Error_SPEC;
    pub type Error = crate::EnumBitfieldStruct<u8, Error_SPEC>;
    impl Error {
        #[doc = "No stack pointer overflow or underflow occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer overflow or underflow occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Disable stack pointer monitor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable stack pointer monitor"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpupt_SPEC;
impl crate::sealed::RegSpec for Mspmpupt_SPEC {
    type DataType = u16;
}

#[doc = "Stack Pointer Monitor Protection Register"]
pub type Mspmpupt = crate::RegValueT<Mspmpupt_SPEC>;

impl Mspmpupt {
    #[doc = "Key Code\n Write data is not saved."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        mspmpupt::Key,
        mspmpupt::Key,
        Mspmpupt_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            mspmpupt::Key,
            mspmpupt::Key,
            Mspmpupt_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mspmpupt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mspmpupt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mspmpupt::Protect,
        mspmpupt::Protect,
        Mspmpupt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mspmpupt::Protect,
            mspmpupt::Protect,
            Mspmpupt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mspmpupt {
    #[inline(always)]
    fn default() -> Mspmpupt {
        <crate::RegValueT<Mspmpupt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mspmpupt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);

        #[doc = "Writing to the PROTECT bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Stack pointer monitor register writes are permitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer monitor register writes are protected. Reads are permitted."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpusa_SPEC;
impl crate::sealed::RegSpec for Mspmpusa_SPEC {
    type DataType = u32;
}

#[doc = "Main Stack Pointer Monitor Start Address Register"]
pub type Mspmpusa = crate::RegValueT<Mspmpusa_SPEC>;

impl Mspmpusa {
    #[doc = "Region Start Address : \nAddress where the region starts, for use in region determination.\nNOTE: Range: 0x1FF00000-0x200FFFFC\n         The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mspmpusa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mspmpusa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Mspmpusa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mspmpusa {
    #[inline(always)]
    fn default() -> Mspmpusa {
        <crate::RegValueT<Mspmpusa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpuea_SPEC;
impl crate::sealed::RegSpec for Mspmpuea_SPEC {
    type DataType = u32;
}

#[doc = "Main Stack Pointer Monitor End Address Register"]
pub type Mspmpuea = crate::RegValueT<Mspmpuea_SPEC>;

impl Mspmpuea {
    #[doc = "Region End Address : \nAddress where the region ends, for use in region determination.\nNOTE: Range: 0x1FF00003-0x200FFFFF\n         The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mspmpuea(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mspmpuea_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Mspmpuea_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mspmpuea {
    #[inline(always)]
    fn default() -> Mspmpuea {
        <crate::RegValueT<Mspmpuea_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpuoad_SPEC;
impl crate::sealed::RegSpec for Pspmpuoad_SPEC {
    type DataType = u16;
}

#[doc = "Stack Pointer Monitor Operation After Detection Register"]
pub type Pspmpuoad = crate::RegValueT<Pspmpuoad_SPEC>;

impl Pspmpuoad {
    #[doc = "Write Keyword \nThe data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        pspmpuoad::Key,
        pspmpuoad::Key,
        Pspmpuoad_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            pspmpuoad::Key,
            pspmpuoad::Key,
            Pspmpuoad_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Pspmpuoad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Pspmpuoad_SPEC,crate::common::RW>::from_register(self,0)
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
        pspmpuoad::Oad,
        pspmpuoad::Oad,
        Pspmpuoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pspmpuoad::Oad,
            pspmpuoad::Oad,
            Pspmpuoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pspmpuoad {
    #[inline(always)]
    fn default() -> Pspmpuoad {
        <crate::RegValueT<Pspmpuoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pspmpuoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);

        #[doc = "Writing to the OAD bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);

        #[doc = "Non-maskable interrupt."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpuctl_SPEC;
impl crate::sealed::RegSpec for Pspmpuctl_SPEC {
    type DataType = u16;
}

#[doc = "Stack Pointer Monitor Access Control Register"]
pub type Pspmpuctl = crate::RegValueT<Pspmpuctl_SPEC>;

impl Pspmpuctl {
    #[doc = "Stack Pointer Monitor Error Flag"]
    #[inline(always)]
    pub fn error(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pspmpuctl::Error,
        pspmpuctl::Error,
        Pspmpuctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pspmpuctl::Error,
            pspmpuctl::Error,
            Pspmpuctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Pspmpuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Pspmpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Stack Pointer Monitor Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pspmpuctl::Enable,
        pspmpuctl::Enable,
        Pspmpuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pspmpuctl::Enable,
            pspmpuctl::Enable,
            Pspmpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pspmpuctl {
    #[inline(always)]
    fn default() -> Pspmpuctl {
        <crate::RegValueT<Pspmpuctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pspmpuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Error_SPEC;
    pub type Error = crate::EnumBitfieldStruct<u8, Error_SPEC>;
    impl Error {
        #[doc = "No stack pointer overflow or underflow occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer overflow or underflow occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Disable stack pointer monitor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable stack pointer monitor"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpupt_SPEC;
impl crate::sealed::RegSpec for Pspmpupt_SPEC {
    type DataType = u16;
}

#[doc = "Stack Pointer Monitor Protection Register"]
pub type Pspmpupt = crate::RegValueT<Pspmpupt_SPEC>;

impl Pspmpupt {
    #[doc = "Key Code\n Write data is not saved."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        pspmpupt::Key,
        pspmpupt::Key,
        Pspmpupt_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            pspmpupt::Key,
            pspmpupt::Key,
            Pspmpupt_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Pspmpupt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Pspmpupt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pspmpupt::Protect,
        pspmpupt::Protect,
        Pspmpupt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pspmpupt::Protect,
            pspmpupt::Protect,
            Pspmpupt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pspmpupt {
    #[inline(always)]
    fn default() -> Pspmpupt {
        <crate::RegValueT<Pspmpupt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pspmpupt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);

        #[doc = "Writing to the PROTECT bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Stack pointer monitor register writes are permitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer monitor register writes are protected. Reads are permitted."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpusa_SPEC;
impl crate::sealed::RegSpec for Pspmpusa_SPEC {
    type DataType = u32;
}

#[doc = "Process Stack Pointer Monitor Start Address Register"]
pub type Pspmpusa = crate::RegValueT<Pspmpusa_SPEC>;

impl Pspmpusa {
    #[doc = "Region Start Address : \nAddress where the region starts, for use in region determination.\nNOTE: Range: 0x1FF00000-0x200FFFFC\n         The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn pspmpusa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Pspmpusa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Pspmpusa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pspmpusa {
    #[inline(always)]
    fn default() -> Pspmpusa {
        <crate::RegValueT<Pspmpusa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpuea_SPEC;
impl crate::sealed::RegSpec for Pspmpuea_SPEC {
    type DataType = u32;
}

#[doc = "Process Stack Pointer Monitor End Address Register"]
pub type Pspmpuea = crate::RegValueT<Pspmpuea_SPEC>;

impl Pspmpuea {
    #[doc = "Region End Address : \nAddress where the region ends, for use in region determination.\nNOTE: Range: 0x1FF00003-0x200FFFFF\n         The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn pspmpuea(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Pspmpuea_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Pspmpuea_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pspmpuea {
    #[inline(always)]
    fn default() -> Pspmpuea {
        <crate::RegValueT<Pspmpuea_SPEC> as RegisterValue<_>>::new(3)
    }
}
