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
#[doc = r"BUS Control"]
unsafe impl ::core::marker::Send for super::Bus {}
unsafe impl ::core::marker::Sync for super::Bus {}
impl super::Bus {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Master Bus Control Register SYS"]
    #[inline(always)]
    pub const fn busmcntsys(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcntsys_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcntsys_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4104usize),
            )
        }
    }

    #[doc = "Master Bus Control Register DMA"]
    #[inline(always)]
    pub const fn busmcntdma(
        &self,
    ) -> &'static crate::common::Reg<self::Busmcntdma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busmcntdma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4108usize),
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

    #[doc = "Slave Bus Control Register RAM0"]
    #[inline(always)]
    pub const fn busscntram0(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntram0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntram0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4364usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register %s"]
    #[inline(always)]
    pub const fn busscnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Busscnt_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1114usize))
        }
    }
    #[inline(always)]
    pub const fn busscntp0b(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn busscntp2b(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1118usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register P4B"]
    #[inline(always)]
    pub const fn busscntp4b(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntp4B_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntp4B_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4384usize),
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

    #[doc = "Slave Bus Control Register FBU"]
    #[inline(always)]
    pub const fn busscntfbu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntfbu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntfbu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4400usize),
            )
        }
    }

    #[doc = "Bus Error Address Register %s"]
    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1820usize))
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
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1824usize))
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
pub struct Busmcntsys_SPEC;
impl crate::sealed::RegSpec for Busmcntsys_SPEC {
    type DataType = u16;
}

#[doc = "Master Bus Control Register SYS"]
pub type Busmcntsys = crate::RegValueT<Busmcntsys_SPEC>;

impl Busmcntsys {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        busmcntsys::Ieres,
        busmcntsys::Ieres,
        Busmcntsys_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            busmcntsys::Ieres,
            busmcntsys::Ieres,
            Busmcntsys_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Busmcntsys_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Busmcntsys_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busmcntsys {
    #[inline(always)]
    fn default() -> Busmcntsys {
        <crate::RegValueT<Busmcntsys_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcntsys {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "Bus error will be reported."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error will not be reported."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcntdma_SPEC;
impl crate::sealed::RegSpec for Busmcntdma_SPEC {
    type DataType = u16;
}

#[doc = "Master Bus Control Register DMA"]
pub type Busmcntdma = crate::RegValueT<Busmcntdma_SPEC>;

impl Busmcntdma {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        busmcntdma::Ieres,
        busmcntdma::Ieres,
        Busmcntdma_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            busmcntdma::Ieres,
            busmcntdma::Ieres,
            Busmcntdma_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Busmcntdma_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Busmcntdma_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busmcntdma {
    #[inline(always)]
    fn default() -> Busmcntdma {
        <crate::RegValueT<Busmcntdma_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcntdma {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "Bus error will be reported."]
        pub const _0: Self = Self::new(0);

        #[doc = "Bus error will not be reported."]
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
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntram0_SPEC;
impl crate::sealed::RegSpec for Busscntram0_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register RAM0"]
pub type Busscntram0 = crate::RegValueT<Busscntram0_SPEC>;

impl Busscntram0 {
    #[doc = "Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntram0::Arbmet,
        busscntram0::Arbmet,
        Busscntram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntram0::Arbmet,
            busscntram0::Arbmet,
            Busscntram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Busscntram0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Busscntram0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntram0 {
    #[inline(always)]
    fn default() -> Busscntram0 {
        <crate::RegValueT<Busscntram0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntram0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
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
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntp4B_SPEC;
impl crate::sealed::RegSpec for Busscntp4B_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register P4B"]
pub type Busscntp4B = crate::RegValueT<Busscntp4B_SPEC>;

impl Busscntp4B {
    #[doc = "Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntp4b::Arbmet,
        busscntp4b::Arbmet,
        Busscntp4B_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntp4b::Arbmet,
            busscntp4b::Arbmet,
            Busscntp4B_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Busscntp4B_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Busscntp4B_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntp4B {
    #[inline(always)]
    fn default() -> Busscntp4B {
        <crate::RegValueT<Busscntp4B_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntp4b {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
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
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfbu_SPEC;
impl crate::sealed::RegSpec for Busscntfbu_SPEC {
    type DataType = u16;
}

#[doc = "Slave Bus Control Register FBU"]
pub type Busscntfbu = crate::RegValueT<Busscntfbu_SPEC>;

impl Busscntfbu {
    #[doc = "Arbitration MethodSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntfbu::Arbmet,
        busscntfbu::Arbmet,
        Busscntfbu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntfbu::Arbmet,
            busscntfbu::Arbmet,
            Busscntfbu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Busscntfbu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Busscntfbu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntfbu {
    #[inline(always)]
    fn default() -> Busscntfbu {
        <crate::RegValueT<Busscntfbu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntfbu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);

        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
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

    #[doc = "Error access statusThe status at the time of the error"]
    #[inline(always)]
    pub fn accstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstat::Accstat,
        buserrstat::Accstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstat::Accstat,
            buserrstat::Accstat,
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

        #[doc = "Bus error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Accstat_SPEC;
    pub type Accstat = crate::EnumBitfieldStruct<u8, Accstat_SPEC>;
    impl Accstat {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write Access"]
        pub const _1: Self = Self::new(1);
    }
}
