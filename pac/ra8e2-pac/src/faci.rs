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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:54:26 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Flash Application Command Interface"]
unsafe impl ::core::marker::Send for super::Faci {}
unsafe impl ::core::marker::Sync for super::Faci {}
impl super::Faci {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Flash Access Status Register"]
    #[inline(always)]
    pub const fn fastat(
        &self,
    ) -> &'static crate::common::Reg<self::Fastat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fastat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Flash Access Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn faeint(
        &self,
    ) -> &'static crate::common::Reg<self::Faeint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Faeint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Flash Ready Interrupt Enable Register"]
    #[inline(always)]
    pub const fn frdyie(
        &self,
    ) -> &'static crate::common::Reg<self::Frdyie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Frdyie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "FACI Command Start Address Register"]
    #[inline(always)]
    pub const fn fsaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Fsaddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsaddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "FACI Command End Address Register"]
    #[inline(always)]
    pub const fn feaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Feaddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Feaddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Flash P/E Mode Entry Protection Register"]
    #[inline(always)]
    pub const fn fmeprot(
        &self,
    ) -> &'static crate::common::Reg<self::Fmeprot_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fmeprot_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Flash Counter Select Register"]
    #[inline(always)]
    pub const fn fcntselr(
        &self,
    ) -> &'static crate::common::Reg<self::Fcntselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcntselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Flash Counter Data Register %s"]
    #[inline(always)]
    pub const fn fcntdatar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fcntdatar_SPEC, crate::common::R>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4cusize))
        }
    }
    #[inline(always)]
    pub const fn fcntdatar0(
        &self,
    ) -> &'static crate::common::Reg<self::Fcntdatar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fcntdatar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x4cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn fcntdatar1(
        &self,
    ) -> &'static crate::common::Reg<self::Fcntdatar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fcntdatar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x50usize),
            )
        }
    }

    #[doc = "Flash Block Protection Register"]
    #[inline(always)]
    pub const fn fbprot0(
        &self,
    ) -> &'static crate::common::Reg<self::Fbprot0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbprot0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Flash Block Protection for Secure Register"]
    #[inline(always)]
    pub const fn fbprot1(
        &self,
    ) -> &'static crate::common::Reg<self::Fbprot1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbprot1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Flash Status Register"]
    #[inline(always)]
    pub const fn fstatr(&self) -> &'static crate::common::Reg<self::Fstatr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fstatr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Flash P/E Mode Entry Register"]
    #[inline(always)]
    pub const fn fentryr(
        &self,
    ) -> &'static crate::common::Reg<self::Fentryr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fentryr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Flash Sequencer Setup Initialization Register"]
    #[inline(always)]
    pub const fn fsuinitr(
        &self,
    ) -> &'static crate::common::Reg<self::Fsuinitr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsuinitr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "FACI Command Register"]
    #[inline(always)]
    pub const fn fcmdr(&self) -> &'static crate::common::Reg<self::Fcmdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fcmdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Blank Check Control Register"]
    #[inline(always)]
    pub const fn fbccnt(
        &self,
    ) -> &'static crate::common::Reg<self::Fbccnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbccnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Blank Check Status Register"]
    #[inline(always)]
    pub const fn fbcstat(
        &self,
    ) -> &'static crate::common::Reg<self::Fbcstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fbcstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "Data Flash Programming Start Address Register"]
    #[inline(always)]
    pub const fn fpsaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Fpsaddr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fpsaddr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "Flash Startup Area Select Monitor Register"]
    #[inline(always)]
    pub const fn fsuasmon(
        &self,
    ) -> &'static crate::common::Reg<self::Fsuasmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fsuasmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[doc = "Flash Sequencer Processing Switching Register"]
    #[inline(always)]
    pub const fn fcpsr(&self) -> &'static crate::common::Reg<self::Fcpsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcpsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "Flash Sequencer Processing Clock Notification Register"]
    #[inline(always)]
    pub const fn fpckar(
        &self,
    ) -> &'static crate::common::Reg<self::Fpckar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fpckar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "Flash Startup Area Control Register"]
    #[inline(always)]
    pub const fn fsuacr(
        &self,
    ) -> &'static crate::common::Reg<self::Fsuacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsuacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fastat_SPEC;
impl crate::sealed::RegSpec for Fastat_SPEC {
    type DataType = u8;
}

#[doc = "Flash Access Status Register"]
pub type Fastat = crate::RegValueT<Fastat_SPEC>;

impl Fastat {
    #[doc = "Data Flash Memory Access Violation Flag"]
    #[inline(always)]
    pub fn dfae(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fastat::Dfae,
        fastat::Dfae,
        Fastat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fastat::Dfae,
            fastat::Dfae,
            Fastat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Lock Flag"]
    #[inline(always)]
    pub fn cmdlk(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fastat::Cmdlk,
        fastat::Cmdlk,
        Fastat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fastat::Cmdlk,
            fastat::Cmdlk,
            Fastat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Code Flash Memory Access Violation Flag"]
    #[inline(always)]
    pub fn cfae(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fastat::Cfae,
        fastat::Cfae,
        Fastat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fastat::Cfae,
            fastat::Cfae,
            Fastat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fastat {
    #[inline(always)]
    fn default() -> Fastat {
        <crate::RegValueT<Fastat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fastat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfae_SPEC;
    pub type Dfae = crate::EnumBitfieldStruct<u8, Dfae_SPEC>;
    impl Dfae {
        #[doc = "No data flash memory access violation has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A data flash memory access violation has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlk_SPEC;
    pub type Cmdlk = crate::EnumBitfieldStruct<u8, Cmdlk_SPEC>;
    impl Cmdlk {
        #[doc = "The flash sequencer is not in the command-locked state"]
        pub const _0: Self = Self::new(0);

        #[doc = "The flash sequencer is in the command-locked state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfae_SPEC;
    pub type Cfae = crate::EnumBitfieldStruct<u8, Cfae_SPEC>;
    impl Cfae {
        #[doc = "No code flash memory access violation has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "A code flash memory access violation has occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Faeint_SPEC;
impl crate::sealed::RegSpec for Faeint_SPEC {
    type DataType = u8;
}

#[doc = "Flash Access Error Interrupt Enable Register"]
pub type Faeint = crate::RegValueT<Faeint_SPEC>;

impl Faeint {
    #[doc = "Data Flash Memory Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn dfaeie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        faeint::Dfaeie,
        faeint::Dfaeie,
        Faeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            faeint::Dfaeie,
            faeint::Dfaeie,
            Faeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Lock Interrupt Enable"]
    #[inline(always)]
    pub fn cmdlkie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        faeint::Cmdlkie,
        faeint::Cmdlkie,
        Faeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            faeint::Cmdlkie,
            faeint::Cmdlkie,
            Faeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code Flash Memory Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn cfaeie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        faeint::Cfaeie,
        faeint::Cfaeie,
        Faeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            faeint::Cfaeie,
            faeint::Cfaeie,
            Faeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Faeint {
    #[inline(always)]
    fn default() -> Faeint {
        <crate::RegValueT<Faeint_SPEC> as RegisterValue<_>>::new(152)
    }
}
pub mod faeint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfaeie_SPEC;
    pub type Dfaeie = crate::EnumBitfieldStruct<u8, Dfaeie_SPEC>;
    impl Dfaeie {
        #[doc = "Generation of an FIFERR interrupt request is disabled when FASTAT.DFAE is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an FIFERR interrupt request is enabled when FASTAT.DFAE is set to 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlkie_SPEC;
    pub type Cmdlkie = crate::EnumBitfieldStruct<u8, Cmdlkie_SPEC>;
    impl Cmdlkie {
        #[doc = "Generation of an FIFERR interrupt request is disabled when FASTAT.CMDLK is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an FIFERR interrupt request is enabled when FASTAT.CMDLK is set to 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfaeie_SPEC;
    pub type Cfaeie = crate::EnumBitfieldStruct<u8, Cfaeie_SPEC>;
    impl Cfaeie {
        #[doc = "Generation of an FIFERR interrupt request is disabled when FASTAT.CFAE is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an FIFERR interrupt request is enabled when FASTAT.CFAE is set to 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdyie_SPEC;
impl crate::sealed::RegSpec for Frdyie_SPEC {
    type DataType = u8;
}

#[doc = "Flash Ready Interrupt Enable Register"]
pub type Frdyie = crate::RegValueT<Frdyie_SPEC>;

impl Frdyie {
    #[doc = "Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdyie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        frdyie::Frdyie,
        frdyie::Frdyie,
        Frdyie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            frdyie::Frdyie,
            frdyie::Frdyie,
            Frdyie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Frdyie {
    #[inline(always)]
    fn default() -> Frdyie {
        <crate::RegValueT<Frdyie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdyie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdyie_SPEC;
    pub type Frdyie = crate::EnumBitfieldStruct<u8, Frdyie_SPEC>;
    impl Frdyie {
        #[doc = "Generation of an FRDY interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an FRDY interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsaddr_SPEC;
impl crate::sealed::RegSpec for Fsaddr_SPEC {
    type DataType = u32;
}

#[doc = "FACI Command Start Address Register"]
pub type Fsaddr = crate::RegValueT<Fsaddr_SPEC>;

impl NoBitfieldReg<Fsaddr_SPEC> for Fsaddr {}
impl ::core::default::Default for Fsaddr {
    #[inline(always)]
    fn default() -> Fsaddr {
        <crate::RegValueT<Fsaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feaddr_SPEC;
impl crate::sealed::RegSpec for Feaddr_SPEC {
    type DataType = u32;
}

#[doc = "FACI Command End Address Register"]
pub type Feaddr = crate::RegValueT<Feaddr_SPEC>;

impl Feaddr {
    #[doc = "End Address for FACI Command Processing"]
    #[inline(always)]
    pub fn feaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Feaddr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Feaddr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Feaddr {
    #[inline(always)]
    fn default() -> Feaddr {
        <crate::RegValueT<Feaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmeprot_SPEC;
impl crate::sealed::RegSpec for Fmeprot_SPEC {
    type DataType = u16;
}

#[doc = "Flash P/E Mode Entry Protection Register"]
pub type Fmeprot = crate::RegValueT<Fmeprot_SPEC>;

impl Fmeprot {
    #[doc = "Code Flash P/E Mode Entry Protection"]
    #[inline(always)]
    pub fn ceprot(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fmeprot::Ceprot,
        fmeprot::Ceprot,
        Fmeprot_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fmeprot::Ceprot,
            fmeprot::Ceprot,
            Fmeprot_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fmeprot_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fmeprot_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fmeprot {
    #[inline(always)]
    fn default() -> Fmeprot {
        <crate::RegValueT<Fmeprot_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod fmeprot {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceprot_SPEC;
    pub type Ceprot = crate::EnumBitfieldStruct<u8, Ceprot_SPEC>;
    impl Ceprot {
        #[doc = "FENTRYC bit is not protected"]
        pub const _0: Self = Self::new(0);

        #[doc = "FENTRYC bit is protected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcntselr_SPEC;
impl crate::sealed::RegSpec for Fcntselr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Counter Select Register"]
pub type Fcntselr = crate::RegValueT<Fcntselr_SPEC>;

impl Fcntselr {
    #[doc = "Counter Select"]
    #[inline(always)]
    pub fn cntsel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fcntselr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fcntselr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcntselr {
    #[inline(always)]
    fn default() -> Fcntselr {
        <crate::RegValueT<Fcntselr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcntdatar_SPEC;
impl crate::sealed::RegSpec for Fcntdatar_SPEC {
    type DataType = u32;
}

#[doc = "Flash Counter Data Register %s"]
pub type Fcntdatar = crate::RegValueT<Fcntdatar_SPEC>;

impl Fcntdatar {
    #[doc = "Counter Read Data"]
    #[inline(always)]
    pub fn cntrdat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fcntdatar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fcntdatar_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcntdatar {
    #[inline(always)]
    fn default() -> Fcntdatar {
        <crate::RegValueT<Fcntdatar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot0_SPEC;
impl crate::sealed::RegSpec for Fbprot0_SPEC {
    type DataType = u16;
}

#[doc = "Flash Block Protection Register"]
pub type Fbprot0 = crate::RegValueT<Fbprot0_SPEC>;

impl Fbprot0 {
    #[doc = "Block Protection for Non-secure Cancel"]
    #[inline(always)]
    pub fn bpcn0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbprot0::Bpcn0,
        fbprot0::Bpcn0,
        Fbprot0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbprot0::Bpcn0,
            fbprot0::Bpcn0,
            Fbprot0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fbprot0_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fbprot0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbprot0 {
    #[inline(always)]
    fn default() -> Fbprot0 {
        <crate::RegValueT<Fbprot0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbprot0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn0_SPEC;
    pub type Bpcn0 = crate::EnumBitfieldStruct<u8, Bpcn0_SPEC>;
    impl Bpcn0 {
        #[doc = "Block protection is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Block protection is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot1_SPEC;
impl crate::sealed::RegSpec for Fbprot1_SPEC {
    type DataType = u16;
}

#[doc = "Flash Block Protection for Secure Register"]
pub type Fbprot1 = crate::RegValueT<Fbprot1_SPEC>;

impl Fbprot1 {
    #[doc = "Block Protection for Secure Cancel"]
    #[inline(always)]
    pub fn bpcn1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbprot1::Bpcn1,
        fbprot1::Bpcn1,
        Fbprot1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbprot1::Bpcn1,
            fbprot1::Bpcn1,
            Fbprot1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fbprot1_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fbprot1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbprot1 {
    #[inline(always)]
    fn default() -> Fbprot1 {
        <crate::RegValueT<Fbprot1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbprot1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn1_SPEC;
    pub type Bpcn1 = crate::EnumBitfieldStruct<u8, Bpcn1_SPEC>;
    impl Bpcn1 {
        #[doc = "Block protection is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Block protection is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstatr_SPEC;
impl crate::sealed::RegSpec for Fstatr_SPEC {
    type DataType = u32;
}

#[doc = "Flash Status Register"]
pub type Fstatr = crate::RegValueT<Fstatr_SPEC>;

impl Fstatr {
    #[doc = "Flash Write/Erase Protect Error Flag"]
    #[inline(always)]
    pub fn flweerr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fstatr::Flweerr,
        fstatr::Flweerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fstatr::Flweerr,
            fstatr::Flweerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Programming Suspend Status Flag"]
    #[inline(always)]
    pub fn prgspd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fstatr::Prgspd,
        fstatr::Prgspd,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fstatr::Prgspd,
            fstatr::Prgspd,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Erasure Suspend Status Flag"]
    #[inline(always)]
    pub fn ersspd(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fstatr::Ersspd,
        fstatr::Ersspd,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fstatr::Ersspd,
            fstatr::Ersspd,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Data Buffer Full Flag"]
    #[inline(always)]
    pub fn dbfull(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fstatr::Dbfull,
        fstatr::Dbfull,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fstatr::Dbfull,
            fstatr::Dbfull,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Suspend Ready Flag"]
    #[inline(always)]
    pub fn susrdy(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fstatr::Susrdy,
        fstatr::Susrdy,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fstatr::Susrdy,
            fstatr::Susrdy,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Programming Error Flag"]
    #[inline(always)]
    pub fn prgerr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fstatr::Prgerr,
        fstatr::Prgerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fstatr::Prgerr,
            fstatr::Prgerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Erasure Error Flag"]
    #[inline(always)]
    pub fn erserr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        fstatr::Erserr,
        fstatr::Erserr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            fstatr::Erserr,
            fstatr::Erserr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Command Error Flag"]
    #[inline(always)]
    pub fn ilglerr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        fstatr::Ilglerr,
        fstatr::Ilglerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            fstatr::Ilglerr,
            fstatr::Ilglerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flash Ready Flag"]
    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fstatr::Frdy,
        fstatr::Frdy,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fstatr::Frdy,
            fstatr::Frdy,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TrustZone Filter Error"]
    #[inline(always)]
    pub fn tzferr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        fstatr::Tzferr,
        fstatr::Tzferr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            fstatr::Tzferr,
            fstatr::Tzferr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Other Error"]
    #[inline(always)]
    pub fn oterr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fstatr::Oterr,
        fstatr::Oterr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fstatr::Oterr,
            fstatr::Oterr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Security Error"]
    #[inline(always)]
    pub fn secerr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fstatr::Secerr,
        fstatr::Secerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fstatr::Secerr,
            fstatr::Secerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "FENTRY Setting Error"]
    #[inline(always)]
    pub fn feseterr(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        fstatr::Feseterr,
        fstatr::Feseterr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            fstatr::Feseterr,
            fstatr::Feseterr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Command Error"]
    #[inline(always)]
    pub fn ilgcomerr(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fstatr::Ilgcomerr,
        fstatr::Ilgcomerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fstatr::Ilgcomerr,
            fstatr::Ilgcomerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fstatr {
    #[inline(always)]
    fn default() -> Fstatr {
        <crate::RegValueT<Fstatr_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod fstatr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flweerr_SPEC;
    pub type Flweerr = crate::EnumBitfieldStruct<u8, Flweerr_SPEC>;
    impl Flweerr {
        #[doc = "An error has not occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgspd_SPEC;
    pub type Prgspd = crate::EnumBitfieldStruct<u8, Prgspd_SPEC>;
    impl Prgspd {
        #[doc = "The flash sequencer is not in the programming suspension processing state or programming suspended state"]
        pub const _0: Self = Self::new(0);

        #[doc = "The flash sequencer is in the programming suspension processing state or programming suspended state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ersspd_SPEC;
    pub type Ersspd = crate::EnumBitfieldStruct<u8, Ersspd_SPEC>;
    impl Ersspd {
        #[doc = "The flash sequencer is not in the erasure suspension processing state or the erasure suspended state"]
        pub const _0: Self = Self::new(0);

        #[doc = "The flash sequencer is in the erasure suspension processing state or the erasure suspended state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbfull_SPEC;
    pub type Dbfull = crate::EnumBitfieldStruct<u8, Dbfull_SPEC>;
    impl Dbfull {
        #[doc = "The data buffer is empty"]
        pub const _0: Self = Self::new(0);

        #[doc = "The data buffer is full."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Susrdy_SPEC;
    pub type Susrdy = crate::EnumBitfieldStruct<u8, Susrdy_SPEC>;
    impl Susrdy {
        #[doc = "The flash sequencer cannot receive P/E suspend commands"]
        pub const _0: Self = Self::new(0);

        #[doc = "The flash sequencer can receive P/E suspend commands."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr_SPEC;
    pub type Prgerr = crate::EnumBitfieldStruct<u8, Prgerr_SPEC>;
    impl Prgerr {
        #[doc = "Programming has completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during programming."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erserr_SPEC;
    pub type Erserr = crate::EnumBitfieldStruct<u8, Erserr_SPEC>;
    impl Erserr {
        #[doc = "Erasure has completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during erasure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilglerr_SPEC;
    pub type Ilglerr = crate::EnumBitfieldStruct<u8, Ilglerr_SPEC>;
    impl Ilglerr {
        #[doc = "The flash sequencer has not detected an illegal FACI command or illegal flash memory access"]
        pub const _0: Self = Self::new(0);

        #[doc = "The flash sequencer has detected an illegal FACI command or illegal flash memory access."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "Program, Block Erase, Multi Block Erase, P/E suspend, P/E resume, Forced Stop, Blank Check, Configuration set, Increment counter, Refresh counter, or Read counter command processing is in progress."]
        pub const _0: Self = Self::new(0);

        #[doc = "None of the above is in progress."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tzferr_SPEC;
    pub type Tzferr = crate::EnumBitfieldStruct<u8, Tzferr_SPEC>;
    impl Tzferr {
        #[doc = "A TrustZone filter error has not been detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A TrustZone filter error has been detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oterr_SPEC;
    pub type Oterr = crate::EnumBitfieldStruct<u8, Oterr_SPEC>;
    impl Oterr {
        #[doc = "An error has not been detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has been detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secerr_SPEC;
    pub type Secerr = crate::EnumBitfieldStruct<u8, Secerr_SPEC>;
    impl Secerr {
        #[doc = "A write protection error against MSUASMON.FSPR bit has not been detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A write protection error against MSUASMON.FSPR bit has been detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Feseterr_SPEC;
    pub type Feseterr = crate::EnumBitfieldStruct<u8, Feseterr_SPEC>;
    impl Feseterr {
        #[doc = "A setting error in the FENTRYR register has not been detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "A setting error in the FENTRYR register has been detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilgcomerr_SPEC;
    pub type Ilgcomerr = crate::EnumBitfieldStruct<u8, Ilgcomerr_SPEC>;
    impl Ilgcomerr {
        #[doc = "An illegal FACI command error has not been detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "An illegal FACI command error has been detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fentryr_SPEC;
impl crate::sealed::RegSpec for Fentryr_SPEC {
    type DataType = u16;
}

#[doc = "Flash P/E Mode Entry Register"]
pub type Fentryr = crate::RegValueT<Fentryr_SPEC>;

impl Fentryr {
    #[doc = "Code Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fentryr::Fentryc,
        fentryr::Fentryc,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fentryr::Fentryc,
            fentryr::Fentryc,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fentryr::Fentryd,
        fentryr::Fentryd,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fentryr::Fentryd,
            fentryr::Fentryd,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fentryr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fentryr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fentryr {
    #[inline(always)]
    fn default() -> Fentryr {
        <crate::RegValueT<Fentryr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fentryr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentryc_SPEC;
    pub type Fentryc = crate::EnumBitfieldStruct<u8, Fentryc_SPEC>;
    impl Fentryc {
        #[doc = "Code flash is in read mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Code flash is in P/E mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentryd_SPEC;
    pub type Fentryd = crate::EnumBitfieldStruct<u8, Fentryd_SPEC>;
    impl Fentryd {
        #[doc = "Data flash is in read mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data flash is in P/E mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuinitr_SPEC;
impl crate::sealed::RegSpec for Fsuinitr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Sequencer Setup Initialization Register"]
pub type Fsuinitr = crate::RegValueT<Fsuinitr_SPEC>;

impl Fsuinitr {
    #[doc = "Set-Up Initialization"]
    #[inline(always)]
    pub fn suinit(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fsuinitr::Suinit,
        fsuinitr::Suinit,
        Fsuinitr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fsuinitr::Suinit,
            fsuinitr::Suinit,
            Fsuinitr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fsuinitr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fsuinitr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsuinitr {
    #[inline(always)]
    fn default() -> Fsuinitr {
        <crate::RegValueT<Fsuinitr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsuinitr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suinit_SPEC;
    pub type Suinit = crate::EnumBitfieldStruct<u8, Suinit_SPEC>;
    impl Suinit {
        #[doc = "The FSADDR, FEADDR, FBPROT0, FBPROT1, FENTRYR, FBCCNT, FCPSR, and FCNTSELR flash sequencer setup registers keep their current values"]
        pub const _0: Self = Self::new(0);

        #[doc = "The FSADDR, FEADDR, FBPROT0, FBRPOT1, FENTRYR, FBCCNT, FCPSR, and FCNTSELR flash sequencer setup registers are initialized."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcmdr_SPEC;
impl crate::sealed::RegSpec for Fcmdr_SPEC {
    type DataType = u16;
}

#[doc = "FACI Command Register"]
pub type Fcmdr = crate::RegValueT<Fcmdr_SPEC>;

impl Fcmdr {
    #[doc = "Pre-command Flag"]
    #[inline(always)]
    pub fn pcmdr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fcmdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fcmdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Flag"]
    #[inline(always)]
    pub fn cmdr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fcmdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fcmdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcmdr {
    #[inline(always)]
    fn default() -> Fcmdr {
        <crate::RegValueT<Fcmdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbccnt_SPEC;
impl crate::sealed::RegSpec for Fbccnt_SPEC {
    type DataType = u8;
}

#[doc = "Blank Check Control Register"]
pub type Fbccnt = crate::RegValueT<Fbccnt_SPEC>;

impl Fbccnt {
    #[doc = "Blank Check Direction"]
    #[inline(always)]
    pub fn bcdir(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbccnt::Bcdir,
        fbccnt::Bcdir,
        Fbccnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbccnt::Bcdir,
            fbccnt::Bcdir,
            Fbccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fbccnt {
    #[inline(always)]
    fn default() -> Fbccnt {
        <crate::RegValueT<Fbccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbccnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdir_SPEC;
    pub type Bcdir = crate::EnumBitfieldStruct<u8, Bcdir_SPEC>;
    impl Bcdir {
        #[doc = "Blank checking is executed from the lower addresses to the higher addresses (incremental mode)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Blank checking is executed from the higher addresses to the lower addresses (decremental mode)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbcstat_SPEC;
impl crate::sealed::RegSpec for Fbcstat_SPEC {
    type DataType = u8;
}

#[doc = "Blank Check Status Register"]
pub type Fbcstat = crate::RegValueT<Fbcstat_SPEC>;

impl Fbcstat {
    #[doc = "Blank Check Status Flag"]
    #[inline(always)]
    pub fn bcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbcstat::Bcst,
        fbcstat::Bcst,
        Fbcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbcstat::Bcst,
            fbcstat::Bcst,
            Fbcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fbcstat {
    #[inline(always)]
    fn default() -> Fbcstat {
        <crate::RegValueT<Fbcstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbcstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcst_SPEC;
    pub type Bcst = crate::EnumBitfieldStruct<u8, Bcst_SPEC>;
    impl Bcst {
        #[doc = "The target area is in the non-programmed state, that is, the area has been erased but has not yet been reprogrammed"]
        pub const _0: Self = Self::new(0);

        #[doc = "The target area has been programmed with 0s or 1s."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpsaddr_SPEC;
impl crate::sealed::RegSpec for Fpsaddr_SPEC {
    type DataType = u32;
}

#[doc = "Data Flash Programming Start Address Register"]
pub type Fpsaddr = crate::RegValueT<Fpsaddr_SPEC>;

impl Fpsaddr {
    #[doc = "Programmed Area Start Address"]
    #[inline(always)]
    pub fn psadr(
        self,
    ) -> crate::common::RegisterField<0, 0x1ffff, 1, 0, u32, u32, Fpsaddr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ffff,1,0,u32,u32,Fpsaddr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fpsaddr {
    #[inline(always)]
    fn default() -> Fpsaddr {
        <crate::RegValueT<Fpsaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuasmon_SPEC;
impl crate::sealed::RegSpec for Fsuasmon_SPEC {
    type DataType = u32;
}

#[doc = "Flash Startup Area Select Monitor Register"]
pub type Fsuasmon = crate::RegValueT<Fsuasmon_SPEC>;

impl Fsuasmon {
    #[doc = "Protection Programming Flag to set Boot Flag and Startup Area Control"]
    #[inline(always)]
    pub fn fspr(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fsuasmon::Fspr,
        fsuasmon::Fspr,
        Fsuasmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fsuasmon::Fspr,
            fsuasmon::Fspr,
            Fsuasmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag of Startup Area Select for Boot Swap"]
    #[inline(always)]
    pub fn btflg(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        fsuasmon::Btflg,
        fsuasmon::Btflg,
        Fsuasmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            fsuasmon::Btflg,
            fsuasmon::Btflg,
            Fsuasmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fsuasmon {
    #[inline(always)]
    fn default() -> Fsuasmon {
        <crate::RegValueT<Fsuasmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsuasmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fspr_SPEC;
    pub type Fspr = crate::EnumBitfieldStruct<u8, Fspr_SPEC>;
    impl Fspr {
        #[doc = "Protected state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-protected state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btflg_SPEC;
    pub type Btflg = crate::EnumBitfieldStruct<u8, Btflg_SPEC>;
    impl Btflg {
        #[doc = "The startup area is the alternate block (block 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The startup area is the default block (block 0)."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcpsr_SPEC;
impl crate::sealed::RegSpec for Fcpsr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Sequencer Processing Switching Register"]
pub type Fcpsr = crate::RegValueT<Fcpsr_SPEC>;

impl Fcpsr {
    #[doc = "Erasure Suspend Mode"]
    #[inline(always)]
    pub fn esuspmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fcpsr::Esuspmd,
        fcpsr::Esuspmd,
        Fcpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcpsr::Esuspmd,
            fcpsr::Esuspmd,
            Fcpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcpsr {
    #[inline(always)]
    fn default() -> Fcpsr {
        <crate::RegValueT<Fcpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esuspmd_SPEC;
    pub type Esuspmd = crate::EnumBitfieldStruct<u8, Esuspmd_SPEC>;
    impl Esuspmd {
        #[doc = "Suspension priority mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Erasure priority mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpckar_SPEC;
impl crate::sealed::RegSpec for Fpckar_SPEC {
    type DataType = u16;
}

#[doc = "Flash Sequencer Processing Clock Notification Register"]
pub type Fpckar = crate::RegValueT<Fpckar_SPEC>;

impl Fpckar {
    #[doc = "Flash Sequencer Operating Clock Notification"]
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fpckar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fpckar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fpckar_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fpckar_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fpckar {
    #[inline(always)]
    fn default() -> Fpckar {
        <crate::RegValueT<Fpckar_SPEC> as RegisterValue<_>>::new(60)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuacr_SPEC;
impl crate::sealed::RegSpec for Fsuacr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Startup Area Control Register"]
pub type Fsuacr = crate::RegValueT<Fsuacr_SPEC>;

impl Fsuacr {
    #[doc = "Startup Area Select"]
    #[inline(always)]
    pub fn sas(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fsuacr::Sas,
        fsuacr::Sas,
        Fsuacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fsuacr::Sas,
            fsuacr::Sas,
            Fsuacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fsuacr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fsuacr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsuacr {
    #[inline(always)]
    fn default() -> Fsuacr {
        <crate::RegValueT<Fsuacr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsuacr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sas_SPEC;
    pub type Sas = crate::EnumBitfieldStruct<u8, Sas_SPEC>;
    impl Sas {
        #[doc = "Startup area is selected by BTFLG bit"]
        pub const _00: Self = Self::new(0);

        #[doc = "Startup area is selected by BTFLG bit"]
        pub const _01: Self = Self::new(1);

        #[doc = "Startup area is temporarily switched to the default area (block 0)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Startup area is temporarily switched to the alternate area (block 1)."]
        pub const _11: Self = Self::new(3);
    }
}
