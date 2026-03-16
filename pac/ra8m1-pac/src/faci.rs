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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

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

    #[doc = "Flash Counter Data Register 0"]
    #[inline(always)]
    pub const fn fcntdatar0(
        &self,
    ) -> &'static crate::common::Reg<self::Fcntdatar0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fcntdatar0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Flash Counter Data Register 1"]
    #[inline(always)]
    pub const fn fcntdatar1(
        &self,
    ) -> &'static crate::common::Reg<self::Fcntdatar1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fcntdatar1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Flash Configuration Update Transfer Control Register"]
    #[inline(always)]
    pub const fn fctrcntr(
        &self,
    ) -> &'static crate::common::Reg<self::Fctrcntr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fctrcntr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Flash Configuration Update Transfer List Select Register"]
    #[inline(always)]
    pub const fn fctrlsr(
        &self,
    ) -> &'static crate::common::Reg<self::Fctrlsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fctrlsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Flash Configuration Update Transfer Address Register"]
    #[inline(always)]
    pub const fn fctraddr(
        &self,
    ) -> &'static crate::common::Reg<self::Fctraddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fctraddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Flash Configuration Update Transfer Status Register"]
    #[inline(always)]
    pub const fn fctrstatr(
        &self,
    ) -> &'static crate::common::Reg<self::Fctrstatr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fctrstatr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Flash Block Protection"]
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

    #[doc = "Flash Block Protection for secure Register"]
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

    #[doc = "Flash Sequencer Set-Up Initialization Register"]
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

    #[doc = "Data Flash Blank Check Control Register"]
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

    #[doc = "Data Flash Blank Check Control Register"]
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

    #[doc = "Flash Start-Up Area Select Monitor Register"]
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

    #[doc = "Flash Start-Up Area Control Register"]
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
    #[doc = "Data Flash Access Error"]
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

    #[doc = "Command Lock"]
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Fastat_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Fastat_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Code Flash Access Error"]
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
        #[doc = "No data flash access error has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Data flash access error has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlk_SPEC;
    pub type Cmdlk = crate::EnumBitfieldStruct<u8, Cmdlk_SPEC>;
    impl Cmdlk {
        #[doc = "Flash sequencer is not in \"Command Lock\" state."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash sequencer is in \"Command Lock\" state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfae_SPEC;
    pub type Cfae = crate::EnumBitfieldStruct<u8, Cfae_SPEC>;
    impl Cfae {
        #[doc = "No code flash access error has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Code flash access error has occurred."]
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
    #[doc = "Data Flash Access Error Interrupt Enable"]
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Faeint_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Faeint_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Code Flash Access Error Interrupt Enable"]
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
        #[doc = "Does not generate \"intflerr\" interrupt request when DFAE = \"1\"."]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates \"intflerr\" interrupt request when DFAE = \"1\"."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlkie_SPEC;
    pub type Cmdlkie = crate::EnumBitfieldStruct<u8, Cmdlkie_SPEC>;
    impl Cmdlkie {
        #[doc = "Does not generate \"intflerr\" interrupt request when CMDLK = \"1\"."]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates \"intflerr\" interrupt request when CMDLK = \"1\"."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfaeie_SPEC;
    pub type Cfaeie = crate::EnumBitfieldStruct<u8, Cfaeie_SPEC>;
    impl Cfaeie {
        #[doc = "Does not generate \"intflerr\" interrupt request when CFAE = \"1\"."]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates \"intflerr\" interrupt request when CFAE = \"1\"."]
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
    #[doc = "FRDY Interrupt Enable"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Frdyie_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Frdyie_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Does not generate \"intflend\" interrupt request when FRDY is changed from \"0\" to \"1\"."]
        pub const _0: Self = Self::new(0);

        #[doc = "Generates \"intflend\" interrupt request when FRDY is changed from \"0\" to \"1\"."]
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

impl Fsaddr {
    #[doc = "Start Address of Flash Sequencer Command Target AreaThese bits can be written when FRDY bit of FSTATR register is \"1\". Writing to these bits in FRDY = \"0\" is ignored."]
    #[inline(always)]
    pub fn fsaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fsaddr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fsaddr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
    #[doc = "End Address of Flash Sequencer Command Target AreaSpecifies end address of target area in \"Blank Check\" command.These bits can be written when FRDY bit of FSTATR register is \"1\". Writing to these bits in FRDY = \"0\" is ignored."]
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

    #[doc = "fac_cpstat pin Monitor"]
    #[inline(always)]
    pub fn cpstat(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fmeprot::Cpstat,
        fmeprot::Cpstat,
        Fmeprot_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fmeprot::Cpstat,
            fmeprot::Cpstat,
            Fmeprot_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Fmeprot_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Fmeprot_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Key CodeThese bits enable or disable CEPROT bit modification. The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        fmeprot::Key,
        fmeprot::Key,
        Fmeprot_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            fmeprot::Key,
            fmeprot::Key,
            Fmeprot_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fmeprot {
    #[inline(always)]
    fn default() -> Fmeprot {
        <crate::RegValueT<Fmeprot_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fmeprot {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceprot_SPEC;
    pub type Ceprot = crate::EnumBitfieldStruct<u8, Ceprot_SPEC>;
    impl Ceprot {
        #[doc = "FENTRYC bit is not protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "FENTRYC bit is protected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpstat_SPEC;
    pub type Cpstat = crate::EnumBitfieldStruct<u8, Cpstat_SPEC>;
    impl Cpstat {
        #[doc = "Protection by CEPROT bit is enabled. (\"fac_cpstat\" pin = \"0\")"]
        pub const _0: Self = Self::new(0);

        #[doc = "Protection by CEPROT bit is disabled. (\"fac_cpstat\" pin = \"1\")"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the other bits in this register is enabled."]
        pub const _0_X_D_9: Self = Self::new(217);
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Fcntselr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Fcntselr_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Fcntdatar0_SPEC;
impl crate::sealed::RegSpec for Fcntdatar0_SPEC {
    type DataType = u32;
}

#[doc = "Flash Counter Data Register 0"]
pub type Fcntdatar0 = crate::RegValueT<Fcntdatar0_SPEC>;

impl Fcntdatar0 {
    #[doc = "Counter Read Data"]
    #[inline(always)]
    pub fn cntrdat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Fcntdatar0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fcntdatar0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcntdatar0 {
    #[inline(always)]
    fn default() -> Fcntdatar0 {
        <crate::RegValueT<Fcntdatar0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcntdatar1_SPEC;
impl crate::sealed::RegSpec for Fcntdatar1_SPEC {
    type DataType = u32;
}

#[doc = "Flash Counter Data Register 1"]
pub type Fcntdatar1 = crate::RegValueT<Fcntdatar1_SPEC>;

impl Fcntdatar1 {
    #[doc = "Counter Read Data"]
    #[inline(always)]
    pub fn cntrdat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Fcntdatar1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fcntdatar1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcntdatar1 {
    #[inline(always)]
    fn default() -> Fcntdatar1 {
        <crate::RegValueT<Fcntdatar1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrcntr_SPEC;
impl crate::sealed::RegSpec for Fctrcntr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Configuration Update Transfer Control Register"]
pub type Fctrcntr = crate::RegValueT<Fctrcntr_SPEC>;

impl Fctrcntr {
    #[doc = "Transfer Start Trigger"]
    #[inline(always)]
    pub fn trtrg(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fctrcntr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fctrcntr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Transfer Information Select"]
    #[inline(always)]
    pub fn trsel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fctrcntr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fctrcntr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Fctrcntr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Fctrcntr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fctrcntr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fctrcntr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fctrcntr {
    #[inline(always)]
    fn default() -> Fctrcntr {
        <crate::RegValueT<Fctrcntr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrlsr_SPEC;
impl crate::sealed::RegSpec for Fctrlsr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Configuration Update Transfer List Select Register"]
pub type Fctrlsr = crate::RegValueT<Fctrlsr_SPEC>;

impl Fctrlsr {
    #[doc = "Configuration Update Transfer List"]
    #[inline(always)]
    pub fn trlist(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fctrlsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fctrlsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Fctrlsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Fctrlsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fctrlsr {
    #[inline(always)]
    fn default() -> Fctrlsr {
        <crate::RegValueT<Fctrlsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctraddr_SPEC;
impl crate::sealed::RegSpec for Fctraddr_SPEC {
    type DataType = u32;
}

#[doc = "Flash Configuration Update Transfer Address Register"]
pub type Fctraddr = crate::RegValueT<Fctraddr_SPEC>;

impl Fctraddr {
    #[doc = "Configuration Update Transfer Source Address"]
    #[inline(always)]
    pub fn ctra(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fctraddr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Fctraddr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fctraddr {
    #[inline(always)]
    fn default() -> Fctraddr {
        <crate::RegValueT<Fctraddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrstatr_SPEC;
impl crate::sealed::RegSpec for Fctrstatr_SPEC {
    type DataType = u8;
}

#[doc = "Flash Configuration Update Transfer Status Register"]
pub type Fctrstatr = crate::RegValueT<Fctrstatr_SPEC>;

impl Fctrstatr {
    #[doc = "Transfer Busy Status"]
    #[inline(always)]
    pub fn trbusy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Fctrstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fctrstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Transfer Address Setting Status"]
    #[inline(always)]
    pub fn trad(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Fctrstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fctrstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Transfer Mode Setting Status"]
    #[inline(always)]
    pub fn trmd(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Fctrstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fctrstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Fctrstatr_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Fctrstatr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fctrstatr {
    #[inline(always)]
    fn default() -> Fctrstatr {
        <crate::RegValueT<Fctrstatr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot0_SPEC;
impl crate::sealed::RegSpec for Fbprot0_SPEC {
    type DataType = u16;
}

#[doc = "Flash Block Protection"]
pub type Fbprot0 = crate::RegValueT<Fbprot0_SPEC>;

impl Fbprot0 {
    #[doc = "Block Protection for non-secure CancelDisables block protect for non-secure function"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Fbprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Fbprot0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Key CodeThese bits enable or disable BPCN0 bit modification. The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        fbprot0::Key,
        fbprot0::Key,
        Fbprot0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            fbprot0::Key,
            fbprot0::Key,
            Fbprot0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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
        #[doc = "User area is protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "User area is not protected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the other bits in this register is enabled."]
        pub const _0_X_78: Self = Self::new(120);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot1_SPEC;
impl crate::sealed::RegSpec for Fbprot1_SPEC {
    type DataType = u16;
}

#[doc = "Flash Block Protection for secure Register"]
pub type Fbprot1 = crate::RegValueT<Fbprot1_SPEC>;

impl Fbprot1 {
    #[doc = "Block Protection for secure CancelDisables block protection for secure"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Fbprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Fbprot1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Key CodeThese bits enable or disable BPCN1 bit modification. The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        fbprot1::Key,
        fbprot1::Key,
        Fbprot1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            fbprot1::Key,
            fbprot1::Key,
            Fbprot1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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
        #[doc = "User area is protected."]
        pub const _0: Self = Self::new(0);

        #[doc = "User area is not protected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the other bits in this register is enabled."]
        pub const _0_X_B_1: Self = Self::new(177);
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
    #[doc = "\"fhve\" Error"]
    #[inline(always)]
    pub fn fhveerr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fstatr::Fhveerr,
        fstatr::Fhveerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fstatr::Fhveerr,
            fstatr::Fhveerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Programming-Suspended Status"]
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

    #[doc = "Erasure-Suspended Status"]
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

    #[doc = "Data Buffer Full"]
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

    #[doc = "Suspend Ready"]
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

    #[doc = "Programming Error"]
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

    #[doc = "Erasure Error"]
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

    #[doc = "Illegal Command Error"]
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

    #[doc = "Flash Ready"]
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

    #[doc = "TrustZone Filter ErrorWhen this bit is \"1\", flash sequencer enters \"Command Lock\" state."]
    #[inline(always)]
    pub fn tzferr(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Fstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Other ErrorWhen this bit is \"1\", flash sequencer enters \"Command Lock\" state."]
    #[inline(always)]
    pub fn oterr(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Fstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Security ErrorWhen this bit is \"1\", flash sequencer enters \"Command Lock\" state."]
    #[inline(always)]
    pub fn secerr(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Fstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FENTRY Setting ErrorWhen this bit is \"1\", flash sequencer enters \"Command Lock\" state."]
    #[inline(always)]
    pub fn feseterr(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Fstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Fstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Illegal Command ErrorWhen this bit is \"1\", flash sequencer enters \"Command Lock\" state."]
    #[inline(always)]
    pub fn ilgcomerr(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Fstatr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Fstatr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Fstatr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Fstatr_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Fhveerr_SPEC;
    pub type Fhveerr = crate::EnumBitfieldStruct<u8, Fhveerr_SPEC>;
    impl Fhveerr {
        #[doc = "No error has been detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has been detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgspd_SPEC;
    pub type Prgspd = crate::EnumBitfieldStruct<u8, Prgspd_SPEC>;
    impl Prgspd {
        #[doc = "Flash sequencer is in status other than the below mentioned."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash sequencer is in programming suspension process or programming-suspended status."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ersspd_SPEC;
    pub type Ersspd = crate::EnumBitfieldStruct<u8, Ersspd_SPEC>;
    impl Ersspd {
        #[doc = "Flash sequencer is in status other than the below mentioned."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash sequencer is in erasure suspension process or erasure-suspended status."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbfull_SPEC;
    pub type Dbfull = crate::EnumBitfieldStruct<u8, Dbfull_SPEC>;
    impl Dbfull {
        #[doc = "Data Buffer is not full"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Susrdy_SPEC;
    pub type Susrdy = crate::EnumBitfieldStruct<u8, Susrdy_SPEC>;
    impl Susrdy {
        #[doc = "Flash sequencer cannot accept \"Program/Erase Suspend\" command."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash sequencer can accept \"Program/Erase Suspend\" command."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr_SPEC;
    pub type Prgerr = crate::EnumBitfieldStruct<u8, Prgerr_SPEC>;
    impl Prgerr {
        #[doc = "Programming has been completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during programming"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erserr_SPEC;
    pub type Erserr = crate::EnumBitfieldStruct<u8, Erserr_SPEC>;
    impl Erserr {
        #[doc = "Erasure processing has been completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during erasure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilglerr_SPEC;
    pub type Ilglerr = crate::EnumBitfieldStruct<u8, Ilglerr_SPEC>;
    impl Ilglerr {
        #[doc = "Flash sequencer has not detected any illegal command or illegal flash memory access."]
        pub const _0: Self = Self::new(0);

        #[doc = "Flash sequencer has detected an illegal command or illegal flash memory access"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "\"Program\", \"DMA Program\", \"Erase\", \"Program\" or \"Erase\" command suspension, \"Forced Stop\", \"Blank Check\", \"Config Program\", \"Config Clear\", \"Lock Bit Program\", \"Lock Bit Read\", or \"OTP Program\" is processing."]
        pub const _0: Self = Self::new(0);

        #[doc = "None of the above is in progress."]
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
    #[doc = "Code Flash P/E Mode EntryThese bits can be written when FRDY bit in FSTATR register is \"1\". Writing to this bit in FRDY = \"0\" is ignored.Writing to these bits is enabled only when this register is accessed in 16-bit size and H\'AA is written to KEY bits"]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Fentryr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Fentryr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Flash P/E Mode EntryThese bits can be written when FRDY bit in FSTATR register is \"1\". Writing to this bit in FRDY = \"0\" is ignored.Writing to these bits is enabled only when this register is accessed in 16-bit size and H\'AA is written to KEY bits."]
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

    #[doc = "KEY Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        fentryr::Key,
        fentryr::Key,
        Fentryr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            fentryr::Key,
            fentryr::Key,
            Fentryr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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
        #[doc = "Code flash is in \"Read Mode\""]
        pub const _0: Self = Self::new(0);

        #[doc = "Code flash is in \"P/E Mode\""]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentryd_SPEC;
    pub type Fentryd = crate::EnumBitfieldStruct<u8, Fentryd_SPEC>;
    impl Fentryd {
        #[doc = "Data flash is in \"Read Mode\""]
        pub const _0: Self = Self::new(0);

        #[doc = "Data flash is in \"P/E Mode\""]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the other bits in this register is enabled."]
        pub const _0_X_AA: Self = Self::new(170);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuinitr_SPEC;
impl crate::sealed::RegSpec for Fsuinitr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Sequencer Set-Up Initialization Register"]
pub type Fsuinitr = crate::RegValueT<Fsuinitr_SPEC>;

impl Fsuinitr {
    #[doc = "Set-up InitializationThis bit can be written when FRDY bit of FSTATR register is \"1\". Writing to this bit in FRDY = \"0\" is ignored.Writing to these bits is enabled only when this register is accessed in 16-bit size and H\'2D is written to KEY bits."]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Fsuinitr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Fsuinitr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "KEY Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        fsuinitr::Key,
        fsuinitr::Key,
        Fsuinitr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            fsuinitr::Key,
            fsuinitr::Key,
            Fsuinitr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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
        #[doc = "Set-up registers keep its\' value."]
        pub const _0: Self = Self::new(0);

        #[doc = "Set-up registers are initialized."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the other bits in this register is enabled."]
        pub const _0_X_2_D: Self = Self::new(45);
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
    #[doc = "Previous Command Register"]
    #[inline(always)]
    pub fn pcmdr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fcmdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fcmdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Register"]
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

#[doc = "Data Flash Blank Check Control Register"]
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

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Fbccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Fbccnt_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Blank check is executed from smaller address to larger address. (Incremental mode)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Blank check is executed from larger address to smaller address. (Decremental mode)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbcstat_SPEC;
impl crate::sealed::RegSpec for Fbcstat_SPEC {
    type DataType = u8;
}

#[doc = "Data Flash Blank Check Control Register"]
pub type Fbcstat = crate::RegValueT<Fbcstat_SPEC>;

impl Fbcstat {
    #[doc = "Blank Check Status Bit"]
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

    #[doc = "These bits are read as 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Fbcstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Fbcstat_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Fill Check Status Bit"]
    #[inline(always)]
    pub fn fcst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fbcstat::Fcst,
        fbcstat::Fcst,
        Fbcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fbcstat::Fcst,
            fbcstat::Fcst,
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
        #[doc = "The target area is erased (blank)."]
        pub const _0: Self = Self::new(0);

        #[doc = "The target area is filled with 0s and/or 1s."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fcst_SPEC;
    pub type Fcst = crate::EnumBitfieldStruct<u8, Fcst_SPEC>;
    impl Fcst {
        #[doc = "The target area is filled with 0s and/or 1s."]
        pub const _0: Self = Self::new(0);

        #[doc = "The target area is blank. (not programmed)"]
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
    #[doc = "Programmed Area Start AddressNOTE: Indicates address of the first programmed data which is found in \"Blank Check\" command execution."]
    #[inline(always)]
    pub fn psadr(
        self,
    ) -> crate::common::RegisterField<0, 0x1ffff, 1, 0, u32, u32, Fpsaddr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ffff,1,0,u32,u32,Fpsaddr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, Fpsaddr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,Fpsaddr_SPEC,crate::common::R>::from_register(self,0)
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

#[doc = "Flash Start-Up Area Select Monitor Register"]
pub type Fsuasmon = crate::RegValueT<Fsuasmon_SPEC>;

impl Fsuasmon {
    #[doc = "Protection Flag of programing the Access Window, Boot Flag and Temporary Boot Swap Control and \"Config Clear\" command execution"]
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

    #[doc = "These bits are read as 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, u16, Fsuasmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7fff,1,0,u16,u16,Fsuasmon_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Flag of Start-Up area select for Boot Swap"]
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

        #[doc = "Non-protected state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btflg_SPEC;
    pub type Btflg = crate::EnumBitfieldStruct<u8, Btflg_SPEC>;
    impl Btflg {
        #[doc = "The start-up area is the alternate area (sector 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The start-up area is the default area (sector 0)"]
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
    #[doc = "Erasure-Suspended Mode"]
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

    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, Fcpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,Fcpsr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Suspension-priority mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Erasure-priority mode"]
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
    #[doc = "Flash Sequencer Processing Clock FrequencyThese bits can be written when FRDY bit in FSTATR register is \"1\". Writing to this bit in FRDY = \"0\" is ignored.Writing to these bits is enabled only when this register is accessed in 16-bit size and H\'1E is written to KEY bits."]
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fpckar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fpckar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "KEY Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        fpckar::Key,
        fpckar::Key,
        Fpckar_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            fpckar::Key,
            fpckar::Key,
            Fpckar_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fpckar {
    #[inline(always)]
    fn default() -> Fpckar {
        <crate::RegValueT<Fpckar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fpckar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the other bits in this register is enabled."]
        pub const _0_X_1_E: Self = Self::new(30);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuacr_SPEC;
impl crate::sealed::RegSpec for Fsuacr_SPEC {
    type DataType = u16;
}

#[doc = "Flash Start-Up Area Control Register"]
pub type Fsuacr = crate::RegValueT<Fsuacr_SPEC>;

impl Fsuacr {
    #[doc = "Start Up Area SelectThese bits can be written when FRDY bit in FSTATR register is \"1\". Writing to this bit in FRDY = \"0\" is ignored.Writing to these bits is enabled only when this register is accessed in 16-bit size and H\'66 is written to KEY bits."]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Fsuacr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Fsuacr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "KEY CodeThese bits enable or disable SAS bits modification. The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        fsuacr::Key,
        fsuacr::Key,
        Fsuacr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            fsuacr::Key,
            fsuacr::Key,
            Fsuacr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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
        #[doc = "The start-up area is temporarily switched to the default area (sector 0) regardless of the BTFLG bit. When a reset is generated after setting, the start-up area is selected according to the BTFLG bit."]
        pub const _10: Self = Self::new(2);

        #[doc = "The start-up area is temporarily switched to the alternate area (sector 1) regardless of the BTFLG bit. When a reset is generated after setting, the start-up area is selected according to the BTFLG bit."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the other bits in this register is enabled."]
        pub const _0_X_66: Self = Self::new(102);
    }
}
