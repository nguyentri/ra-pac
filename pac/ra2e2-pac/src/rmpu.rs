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
// Generated from SVD 1.40.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:37 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Renesas Memory Protection Unit"]
unsafe impl ::core::marker::Send for super::Rmpu {}
unsafe impl ::core::marker::Sync for super::Rmpu {}
impl super::Rmpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Bus Master MPU Control Register"]
    #[inline(always)]
    pub const fn mmpuctla(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuctla_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuctla_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Group A Protection of Register"]
    #[inline(always)]
    pub const fn mmpupta(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpupta_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpupta_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }

    #[doc = "Group A Region %s access control register"]
    #[inline(always)]
    pub const fn mmpuaca(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuaca0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }

    #[doc = "Group A Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusa(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusa0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }

    #[doc = "Group A Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuea(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuea0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }

    #[doc = "Slave MPU Control Register"]
    #[inline(always)]
    pub const fn smpuctl(
        &self,
    ) -> &'static crate::common::Reg<self::Smpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[doc = "Access Control Register for Memory Bus 1"]
    #[inline(always)]
    pub const fn smpumbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpumbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpumbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3088usize),
            )
        }
    }

    #[doc = "Access Control Register for Internal Peripheral Bus 9"]
    #[inline(always)]
    pub const fn smpufbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpufbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpufbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3092usize),
            )
        }
    }

    #[doc = "Access Control Register for Memory Bus 4"]
    #[inline(always)]
    pub const fn smpusram0(
        &self,
    ) -> &'static crate::common::Reg<self::Smpusram0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpusram0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3096usize),
            )
        }
    }

    #[doc = "Access Control Register for Internal Peripheral Bus 1"]
    #[inline(always)]
    pub const fn smpup0biu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpup0Biu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpup0Biu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3104usize),
            )
        }
    }

    #[doc = "Access Control Register for Internal Peripheral Bus 3"]
    #[inline(always)]
    pub const fn smpup2biu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpup2Biu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpup2Biu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3108usize),
            )
        }
    }

    #[doc = "Access Control Register for Internal Peripheral Bus 7"]
    #[inline(always)]
    pub const fn smpup6biu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpup6Biu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpup6Biu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3112usize),
            )
        }
    }

    #[doc = "Stack Pointer Monitor Operation After Detection Register"]
    #[inline(always)]
    pub const fn mspmpuoad(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpuoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpuoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3328usize),
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
                self._svd2pac_as_ptr().add(3332usize),
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
                self._svd2pac_as_ptr().add(3334usize),
            )
        }
    }

    #[doc = "Main Stack Pointer (MSP) Monitor Start Address Register"]
    #[inline(always)]
    pub const fn mspmpusa(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3336usize),
            )
        }
    }

    #[doc = "Main Stack Pointer (MSP) Monitor End Address Register"]
    #[inline(always)]
    pub const fn mspmpuea(
        &self,
    ) -> &'static crate::common::Reg<self::Mspmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3340usize),
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
                self._svd2pac_as_ptr().add(3344usize),
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
                self._svd2pac_as_ptr().add(3348usize),
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
                self._svd2pac_as_ptr().add(3350usize),
            )
        }
    }

    #[doc = "Process Stack Pointer (PSP) Monitor Start Address Register"]
    #[inline(always)]
    pub const fn pspmpusa(
        &self,
    ) -> &'static crate::common::Reg<self::Pspmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pspmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3352usize),
            )
        }
    }

    #[doc = "Process Stack Pointer (PSP) Monitor End Address Register"]
    #[inline(always)]
    pub const fn pspmpuea(
        &self,
    ) -> &'static crate::common::Reg<self::Pspmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pspmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3356usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuctla_SPEC;
impl crate::sealed::RegSpec for Mmpuctla_SPEC {
    type DataType = u16;
}

#[doc = "Bus Master MPU Control Register"]
pub type Mmpuctla = crate::RegValueT<Mmpuctla_SPEC>;

impl Mmpuctla {
    #[doc = "Master Group Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuctla::Enable,
        mmpuctla::Enable,
        Mmpuctla_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuctla::Enable,
            mmpuctla::Enable,
            Mmpuctla_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Operation After Detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuctla::Oad,
        mmpuctla::Oad,
        Mmpuctla_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuctla::Oad,
            mmpuctla::Oad,
            Mmpuctla_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuctla_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuctla_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuctla {
    #[inline(always)]
    fn default() -> Mmpuctla {
        <crate::RegValueT<Mmpuctla_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuctla {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Master group A disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Master group A enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpupta_SPEC;
impl crate::sealed::RegSpec for Mmpupta_SPEC {
    type DataType = u16;
}

#[doc = "Group A Protection of Register"]
pub type Mmpupta = crate::RegValueT<Mmpupta_SPEC>;

impl Mmpupta {
    #[doc = "Protection of Register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpupta::Protect,
        mmpupta::Protect,
        Mmpupta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpupta::Protect,
            mmpupta::Protect,
            Mmpupta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpupta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpupta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpupta {
    #[inline(always)]
    fn default() -> Mmpupta {
        <crate::RegValueT<Mmpupta_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpupta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All bus master MPU group A register writes are permitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "All bus master MPU group A register writes are protected. Reads are permitted."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuaca_SPEC;
impl crate::sealed::RegSpec for Mmpuaca_SPEC {
    type DataType = u16;
}

#[doc = "Group A Region %s access control register"]
pub type Mmpuaca = crate::RegValueT<Mmpuaca_SPEC>;

impl Mmpuaca {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuaca::Enable,
        mmpuaca::Enable,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuaca::Enable,
            mmpuaca::Enable,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read Protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuaca::Rp,
        mmpuaca::Rp,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuaca::Rp,
            mmpuaca::Rp,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuaca::Wp,
        mmpuaca::Wp,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuaca::Wp,
            mmpuaca::Wp,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuaca {
    #[inline(always)]
    fn default() -> Mmpuaca {
        <crate::RegValueT<Mmpuaca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuaca {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group A region n disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Group A region n enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusa_SPEC;
impl crate::sealed::RegSpec for Mmpusa_SPEC {
    type DataType = u32;
}

#[doc = "Group A Region %s Start Address Register"]
pub type Mmpusa = crate::RegValueT<Mmpusa_SPEC>;

impl Mmpusa {
    #[doc = "Region Start Address"]
    #[inline(always)]
    pub fn mmpusa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpusa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpusa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusa {
    #[inline(always)]
    fn default() -> Mmpusa {
        <crate::RegValueT<Mmpusa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuea_SPEC;
impl crate::sealed::RegSpec for Mmpuea_SPEC {
    type DataType = u32;
}

#[doc = "Group A Region %s End Address Register"]
pub type Mmpuea = crate::RegValueT<Mmpuea_SPEC>;

impl Mmpuea {
    #[doc = "Region End Address"]
    #[inline(always)]
    pub fn mmpuea(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpuea_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpuea_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuea {
    #[inline(always)]
    fn default() -> Mmpuea {
        <crate::RegValueT<Mmpuea_SPEC> as RegisterValue<_>>::new(3)
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
    #[doc = "Operation After Detection"]
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

    #[doc = "Protection of Register"]
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

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Smpuctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Smpuctl_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All bus slave register writes are permitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "All bus slave register writes are protected. Reads are permitted"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpumbiu_SPEC;
impl crate::sealed::RegSpec for Smpumbiu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for Memory Bus 1"]
pub type Smpumbiu = crate::RegValueT<Smpumbiu_SPEC>;

impl Smpumbiu {
    #[doc = "Master MPU Group A Read Protection"]
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

    #[doc = "Master MPU Group A Write Protection"]
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
}
impl ::core::default::Default for Smpumbiu {
    #[inline(always)]
    fn default() -> Smpumbiu {
        <crate::RegValueT<Smpumbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpumbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Memory protection read for master MPU group A disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection read for master MPU group A enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Memory protection write for master MPU group A disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection write for master MPU group A enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpufbiu_SPEC;
impl crate::sealed::RegSpec for Smpufbiu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for Internal Peripheral Bus 9"]
pub type Smpufbiu = crate::RegValueT<Smpufbiu_SPEC>;

impl Smpufbiu {
    #[doc = "CPU Read Protection"]
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

    #[doc = "CPU Write Protection"]
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

    #[doc = "Master MPU Group A Read Protection"]
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

    #[doc = "Master MPU Group A Write Protection"]
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
}
impl ::core::default::Default for Smpufbiu {
    #[inline(always)]
    fn default() -> Smpufbiu {
        <crate::RegValueT<Smpufbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpufbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "Memory protection for CPU read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "Memory protection for CPU write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Memory protection for master MPU group A read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Memory protection for master MPU group A write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A write enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpusram0_SPEC;
impl crate::sealed::RegSpec for Smpusram0_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for Memory Bus 4"]
pub type Smpusram0 = crate::RegValueT<Smpusram0_SPEC>;

impl Smpusram0 {
    #[doc = "CPU Read Protection"]
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

    #[doc = "CPU Write Protection"]
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

    #[doc = "Master MPU Group A Read Protection"]
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

    #[doc = "Master MPU Group A Write Protection"]
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
}
impl ::core::default::Default for Smpusram0 {
    #[inline(always)]
    fn default() -> Smpusram0 {
        <crate::RegValueT<Smpusram0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpusram0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "Memory protection for CPU read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "Memory protection for CPU write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Memory protection for master MPU group A read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Memory protection for master MPU group A write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A write enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpup0Biu_SPEC;
impl crate::sealed::RegSpec for Smpup0Biu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for Internal Peripheral Bus 1"]
pub type Smpup0Biu = crate::RegValueT<Smpup0Biu_SPEC>;

impl Smpup0Biu {
    #[doc = "CPU Read Protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpup0biu::Rpcpu,
        smpup0biu::Rpcpu,
        Smpup0Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpup0biu::Rpcpu,
            smpup0biu::Rpcpu,
            Smpup0Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Write Protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpup0biu::Wpcpu,
        smpup0biu::Wpcpu,
        Smpup0Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpup0biu::Wpcpu,
            smpup0biu::Wpcpu,
            Smpup0Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Group A Read Protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpup0biu::Rpgrpa,
        smpup0biu::Rpgrpa,
        Smpup0Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpup0biu::Rpgrpa,
            smpup0biu::Rpgrpa,
            Smpup0Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Group A Write Protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpup0biu::Wpgrpa,
        smpup0biu::Wpgrpa,
        Smpup0Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpup0biu::Wpgrpa,
            smpup0biu::Wpgrpa,
            Smpup0Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpup0Biu {
    #[inline(always)]
    fn default() -> Smpup0Biu {
        <crate::RegValueT<Smpup0Biu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpup0biu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "Memory protection for CPU read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "Memory protection for CPU write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Memory protection for master MPU group A read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Memory protection for master MPU group A write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A write enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpup2Biu_SPEC;
impl crate::sealed::RegSpec for Smpup2Biu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for Internal Peripheral Bus 3"]
pub type Smpup2Biu = crate::RegValueT<Smpup2Biu_SPEC>;

impl Smpup2Biu {
    #[doc = "CPU Read Protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpup2biu::Rpcpu,
        smpup2biu::Rpcpu,
        Smpup2Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpup2biu::Rpcpu,
            smpup2biu::Rpcpu,
            Smpup2Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Write Protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpup2biu::Wpcpu,
        smpup2biu::Wpcpu,
        Smpup2Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpup2biu::Wpcpu,
            smpup2biu::Wpcpu,
            Smpup2Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Group A Read Protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpup2biu::Rpgrpa,
        smpup2biu::Rpgrpa,
        Smpup2Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpup2biu::Rpgrpa,
            smpup2biu::Rpgrpa,
            Smpup2Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Group A Write Protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpup2biu::Wpgrpa,
        smpup2biu::Wpgrpa,
        Smpup2Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpup2biu::Wpgrpa,
            smpup2biu::Wpgrpa,
            Smpup2Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpup2Biu {
    #[inline(always)]
    fn default() -> Smpup2Biu {
        <crate::RegValueT<Smpup2Biu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpup2biu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "Memory protection for CPU read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "Memory protection for CPU write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for CPU write enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Memory protection for master MPU group A read disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A read enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Memory protection for master MPU group A write disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Memory protection for master MPU group A write enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpup6Biu_SPEC;
impl crate::sealed::RegSpec for Smpup6Biu_SPEC {
    type DataType = u16;
}

#[doc = "Access Control Register for Internal Peripheral Bus 7"]
pub type Smpup6Biu = crate::RegValueT<Smpup6Biu_SPEC>;

impl Smpup6Biu {
    #[doc = "CPU Read Protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpup6biu::Rpcpu,
        smpup6biu::Rpcpu,
        Smpup6Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpup6biu::Rpcpu,
            smpup6biu::Rpcpu,
            Smpup6Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Write Protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpup6biu::Wpcpu,
        smpup6biu::Wpcpu,
        Smpup6Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpup6biu::Wpcpu,
            smpup6biu::Wpcpu,
            Smpup6Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Group A Read Protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpup6biu::Rpgrpa,
        smpup6biu::Rpgrpa,
        Smpup6Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpup6biu::Rpgrpa,
            smpup6biu::Rpgrpa,
            Smpup6Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master MPU Group A Write Protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpup6biu::Wpgrpa,
        smpup6biu::Wpgrpa,
        Smpup6Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpup6biu::Wpgrpa,
            smpup6biu::Wpgrpa,
            Smpup6Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpup6Biu {
    #[inline(always)]
    fn default() -> Smpup6Biu {
        <crate::RegValueT<Smpup6Biu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpup6biu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU read of memory protection enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "CPU write of memory protection enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master MPU group A read of memory protection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Master MPU group A read of memory protection enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master MPU group A write of memory protection disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Master MPU group A write of memory protection enabled"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Operation after Detection"]
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

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mspmpuoad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mspmpuoad_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
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
}
impl ::core::default::Default for Mspmpuctl {
    #[inline(always)]
    fn default() -> Mspmpuctl {
        <crate::RegValueT<Mspmpuctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mspmpuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Stack pointer monitor is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer monitor is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Error_SPEC;
    pub type Error = crate::EnumBitfieldStruct<u8, Error_SPEC>;
    impl Error {
        #[doc = "Stack pointer has not overflowed or underflowed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer has overflowed or underflowed"]
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
    #[doc = "Protection of Register"]
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

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mspmpupt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mspmpupt_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Stack pointer monitor register writes are permitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer monitor register writes are protected. Reads are permitted"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspmpusa_SPEC;
impl crate::sealed::RegSpec for Mspmpusa_SPEC {
    type DataType = u32;
}

#[doc = "Main Stack Pointer (MSP) Monitor Start Address Register"]
pub type Mspmpusa = crate::RegValueT<Mspmpusa_SPEC>;

impl Mspmpusa {
    #[doc = "Region Start Address"]
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

#[doc = "Main Stack Pointer (MSP) Monitor End Address Register"]
pub type Mspmpuea = crate::RegValueT<Mspmpuea_SPEC>;

impl Mspmpuea {
    #[doc = "Region End Address"]
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
        <crate::RegValueT<Mspmpuea_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Operation after Detection"]
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

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Pspmpuoad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Pspmpuoad_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pspmpuctl::Error,
            pspmpuctl::Error,
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
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Stack pointer monitor is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer monitor is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Error_SPEC;
    pub type Error = crate::EnumBitfieldStruct<u8, Error_SPEC>;
    impl Error {
        #[doc = "Stack pointer has not overflowed or underflowed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer has overflowed or underflowed"]
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
    #[doc = "Protection of Register"]
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

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Pspmpupt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Pspmpupt_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Stack pointer monitor register writes are permitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stack pointer monitor register writes are protected. Reads are permitted"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pspmpusa_SPEC;
impl crate::sealed::RegSpec for Pspmpusa_SPEC {
    type DataType = u32;
}

#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register"]
pub type Pspmpusa = crate::RegValueT<Pspmpusa_SPEC>;

impl Pspmpusa {
    #[doc = "Region Start Address"]
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

#[doc = "Process Stack Pointer (PSP) Monitor End Address Register"]
pub type Pspmpuea = crate::RegValueT<Pspmpuea_SPEC>;

impl Pspmpuea {
    #[doc = "Region End Address"]
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
        <crate::RegValueT<Pspmpuea_SPEC> as RegisterValue<_>>::new(0)
    }
}
