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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:32 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Direct memory access controller 0"]
unsafe impl ::core::marker::Send for super::Dmac0 {}
unsafe impl ::core::marker::Sync for super::Dmac0 {}
impl super::Dmac0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DMA Source Address Register"]
    #[inline(always)]
    pub const fn dmsar(&self) -> &'static crate::common::Reg<self::Dmsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DMA Destination Address Register"]
    #[inline(always)]
    pub const fn dmdar(&self) -> &'static crate::common::Reg<self::Dmdar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmdar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DMA Transfer Count Register"]
    #[inline(always)]
    pub const fn dmcra(&self) -> &'static crate::common::Reg<self::Dmcra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmcra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "DMA Block Transfer Count Register"]
    #[inline(always)]
    pub const fn dmcrb(&self) -> &'static crate::common::Reg<self::Dmcrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmcrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "DMA Transfer Mode Register"]
    #[inline(always)]
    pub const fn dmtmd(&self) -> &'static crate::common::Reg<self::Dmtmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmtmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DMA Interrupt Setting Register"]
    #[inline(always)]
    pub const fn dmint(&self) -> &'static crate::common::Reg<self::Dmint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19usize),
            )
        }
    }

    #[doc = "DMA Address Mode Register"]
    #[inline(always)]
    pub const fn dmamd(&self) -> &'static crate::common::Reg<self::Dmamd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmamd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "DMA Offset Register"]
    #[inline(always)]
    pub const fn dmofr(&self) -> &'static crate::common::Reg<self::Dmofr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmofr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "DMA Transfer Enable Register"]
    #[inline(always)]
    pub const fn dmcnt(&self) -> &'static crate::common::Reg<self::Dmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "DMA Software Start Register"]
    #[inline(always)]
    pub const fn dmreq(&self) -> &'static crate::common::Reg<self::Dmreq_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmreq_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }

    #[doc = "DMAC Module Activation Register"]
    #[inline(always)]
    pub const fn dmsts(&self) -> &'static crate::common::Reg<self::Dmsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsar_SPEC;
impl crate::sealed::RegSpec for Dmsar_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address Register"]
pub type Dmsar = crate::RegValueT<Dmsar_SPEC>;

impl Dmsar {
    #[doc = "Specifies the transfer source start address."]
    #[inline(always)]
    pub fn dmsar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dmsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dmsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmsar {
    #[inline(always)]
    fn default() -> Dmsar {
        <crate::RegValueT<Dmsar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdar_SPEC;
impl crate::sealed::RegSpec for Dmdar_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address Register"]
pub type Dmdar = crate::RegValueT<Dmdar_SPEC>;

impl Dmdar {
    #[doc = "Specifies the transfer destination start address."]
    #[inline(always)]
    pub fn dmdar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dmdar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dmdar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmdar {
    #[inline(always)]
    fn default() -> Dmdar {
        <crate::RegValueT<Dmdar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcra_SPEC;
impl crate::sealed::RegSpec for Dmcra_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Count Register"]
pub type Dmcra = crate::RegValueT<Dmcra_SPEC>;

impl Dmcra {
    #[doc = "Upper bits of transfer count"]
    #[inline(always)]
    pub fn dmcrah(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Dmcra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Dmcra_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Lower bits of transfer count"]
    #[inline(always)]
    pub fn dmcral(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dmcra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dmcra_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmcra {
    #[inline(always)]
    fn default() -> Dmcra {
        <crate::RegValueT<Dmcra_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcrb_SPEC;
impl crate::sealed::RegSpec for Dmcrb_SPEC {
    type DataType = u16;
}

#[doc = "DMA Block Transfer Count Register"]
pub type Dmcrb = crate::RegValueT<Dmcrb_SPEC>;

impl Dmcrb {
    #[doc = "Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    pub fn dmcrb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        dmcrb::Dmcrb,
        dmcrb::Dmcrb,
        Dmcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            dmcrb::Dmcrb,
            dmcrb::Dmcrb,
            Dmcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmcrb {
    #[inline(always)]
    fn default() -> Dmcrb {
        <crate::RegValueT<Dmcrb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmcrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmcrb_SPEC;
    pub type Dmcrb = crate::EnumBitfieldStruct<u8, Dmcrb_SPEC>;
    impl Dmcrb {
        #[doc = "65,536 blocks"]
        pub const _0000: Self = Self::new(0);

        #[doc = "DMCRB blocks"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmtmd_SPEC;
impl crate::sealed::RegSpec for Dmtmd_SPEC {
    type DataType = u16;
}

#[doc = "DMA Transfer Mode Register"]
pub type Dmtmd = crate::RegValueT<Dmtmd_SPEC>;

impl Dmtmd {
    #[doc = "Transfer Mode Select"]
    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        dmtmd::Md,
        dmtmd::Md,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            dmtmd::Md,
            dmtmd::Md,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Repeat Area Select"]
    #[inline(always)]
    pub fn dts(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        dmtmd::Dts,
        dmtmd::Dts,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            dmtmd::Dts,
            dmtmd::Dts,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Data Size Select"]
    #[inline(always)]
    pub fn sz(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        dmtmd::Sz,
        dmtmd::Sz,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            dmtmd::Sz,
            dmtmd::Sz,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Request Source Select"]
    #[inline(always)]
    pub fn dctg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dmtmd::Dctg,
        dmtmd::Dctg,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dmtmd::Dctg,
            dmtmd::Dctg,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmtmd {
    #[inline(always)]
    fn default() -> Dmtmd {
        <crate::RegValueT<Dmtmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmtmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        #[doc = "Normal transfer"]
        pub const _00: Self = Self::new(0);

        #[doc = "Repeat transfer"]
        pub const _01: Self = Self::new(1);

        #[doc = "Block transfer"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dts_SPEC;
    pub type Dts = crate::EnumBitfieldStruct<u8, Dts_SPEC>;
    impl Dts {
        #[doc = "Specify destination as the repeat area or block area"]
        pub const _00: Self = Self::new(0);

        #[doc = "Specify source as the repeat area or block area"]
        pub const _01: Self = Self::new(1);

        #[doc = "Do not specify repeat area or block area"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sz_SPEC;
    pub type Sz = crate::EnumBitfieldStruct<u8, Sz_SPEC>;
    impl Sz {
        #[doc = "8 bits"]
        pub const _00: Self = Self::new(0);

        #[doc = "16 bits"]
        pub const _01: Self = Self::new(1);

        #[doc = "32 bits"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dctg_SPEC;
    pub type Dctg = crate::EnumBitfieldStruct<u8, Dctg_SPEC>;
    impl Dctg {
        #[doc = "Software"]
        pub const _00: Self = Self::new(0);

        #[doc = "Interrupts from peripheral modules or external interrupt input pins"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmint_SPEC;
impl crate::sealed::RegSpec for Dmint_SPEC {
    type DataType = u8;
}

#[doc = "DMA Interrupt Setting Register"]
pub type Dmint = crate::RegValueT<Dmint_SPEC>;

impl Dmint {
    #[doc = "Transfer End Interrupt Enable"]
    #[inline(always)]
    pub fn dtie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmint::Dtie,
        dmint::Dtie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmint::Dtie,
            dmint::Dtie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Escape End Interrupt Enable"]
    #[inline(always)]
    pub fn esie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmint::Esie,
        dmint::Esie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmint::Esie,
            dmint::Esie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Repeat Size End Interrupt Enable"]
    #[inline(always)]
    pub fn rptie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmint::Rptie,
        dmint::Rptie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmint::Rptie,
            dmint::Rptie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Source Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sarie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmint::Sarie,
        dmint::Sarie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmint::Sarie,
            dmint::Sarie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Destination Address Extended Repeat Area Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn darie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmint::Darie,
        dmint::Darie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmint::Darie,
            dmint::Darie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmint {
    #[inline(always)]
    fn default() -> Dmint {
        <crate::RegValueT<Dmint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtie_SPEC;
    pub type Dtie = crate::EnumBitfieldStruct<u8, Dtie_SPEC>;
    impl Dtie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esie_SPEC;
    pub type Esie = crate::EnumBitfieldStruct<u8, Esie_SPEC>;
    impl Esie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rptie_SPEC;
    pub type Rptie = crate::EnumBitfieldStruct<u8, Rptie_SPEC>;
    impl Rptie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sarie_SPEC;
    pub type Sarie = crate::EnumBitfieldStruct<u8, Sarie_SPEC>;
    impl Sarie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Darie_SPEC;
    pub type Darie = crate::EnumBitfieldStruct<u8, Darie_SPEC>;
    impl Darie {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmamd_SPEC;
impl crate::sealed::RegSpec for Dmamd_SPEC {
    type DataType = u16;
}

#[doc = "DMA Address Mode Register"]
pub type Dmamd = crate::RegValueT<Dmamd_SPEC>;

impl Dmamd {
    #[doc = "Source Address Update Mode"]
    #[inline(always)]
    pub fn sm(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        dmamd::Sm,
        dmamd::Sm,
        Dmamd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            dmamd::Sm,
            dmamd::Sm,
            Dmamd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
    #[inline(always)]
    pub fn sara(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Dmamd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Dmamd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Address Update Mode"]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        dmamd::Dm,
        dmamd::Dm,
        Dmamd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            dmamd::Dm,
            dmamd::Dm,
            Dmamd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
    #[inline(always)]
    pub fn dara(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Dmamd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Dmamd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmamd {
    #[inline(always)]
    fn default() -> Dmamd {
        <crate::RegValueT<Dmamd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmamd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sm_SPEC;
    pub type Sm = crate::EnumBitfieldStruct<u8, Sm_SPEC>;
    impl Sm {
        #[doc = "Fixed address"]
        pub const _00: Self = Self::new(0);

        #[doc = "Offset addition"]
        pub const _01: Self = Self::new(1);

        #[doc = "Incremented address"]
        pub const _10: Self = Self::new(2);

        #[doc = "Decremented address."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dm_SPEC;
    pub type Dm = crate::EnumBitfieldStruct<u8, Dm_SPEC>;
    impl Dm {
        #[doc = "Fixed address"]
        pub const _00: Self = Self::new(0);

        #[doc = "Offset addition"]
        pub const _01: Self = Self::new(1);

        #[doc = "Incremented address"]
        pub const _10: Self = Self::new(2);

        #[doc = "Decremented address."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmofr_SPEC;
impl crate::sealed::RegSpec for Dmofr_SPEC {
    type DataType = u32;
}

#[doc = "DMA Offset Register"]
pub type Dmofr = crate::RegValueT<Dmofr_SPEC>;

impl Dmofr {
    #[doc = "Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    pub fn dmofr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dmofr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dmofr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmofr {
    #[inline(always)]
    fn default() -> Dmofr {
        <crate::RegValueT<Dmofr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcnt_SPEC;
impl crate::sealed::RegSpec for Dmcnt_SPEC {
    type DataType = u8;
}

#[doc = "DMA Transfer Enable Register"]
pub type Dmcnt = crate::RegValueT<Dmcnt_SPEC>;

impl Dmcnt {
    #[doc = "DMA Transfer Enable"]
    #[inline(always)]
    pub fn dte(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmcnt::Dte,
        dmcnt::Dte,
        Dmcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmcnt::Dte,
            dmcnt::Dte,
            Dmcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmcnt {
    #[inline(always)]
    fn default() -> Dmcnt {
        <crate::RegValueT<Dmcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dte_SPEC;
    pub type Dte = crate::EnumBitfieldStruct<u8, Dte_SPEC>;
    impl Dte {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmreq_SPEC;
impl crate::sealed::RegSpec for Dmreq_SPEC {
    type DataType = u8;
}

#[doc = "DMA Software Start Register"]
pub type Dmreq = crate::RegValueT<Dmreq_SPEC>;

impl Dmreq {
    #[doc = "DMA Software Start Bit Auto Clear Select"]
    #[inline(always)]
    pub fn clrs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmreq::Clrs,
        dmreq::Clrs,
        Dmreq_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmreq::Clrs,
            dmreq::Clrs,
            Dmreq_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMA Software Start"]
    #[inline(always)]
    pub fn swreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmreq::Swreq,
        dmreq::Swreq,
        Dmreq_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmreq::Swreq,
            dmreq::Swreq,
            Dmreq_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmreq {
    #[inline(always)]
    fn default() -> Dmreq {
        <crate::RegValueT<Dmreq_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmreq {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrs_SPEC;
    pub type Clrs = crate::EnumBitfieldStruct<u8, Clrs_SPEC>;
    impl Clrs {
        #[doc = "Clear SWREQ bit after DMA transfer is started by software"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not clear SWREQ bit after DMA transfer is started by software"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swreq_SPEC;
    pub type Swreq = crate::EnumBitfieldStruct<u8, Swreq_SPEC>;
    impl Swreq {
        #[doc = "Do not request DMA transfer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Request DMA transfer."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsts_SPEC;
impl crate::sealed::RegSpec for Dmsts_SPEC {
    type DataType = u8;
}

#[doc = "DMAC Module Activation Register"]
pub type Dmsts = crate::RegValueT<Dmsts_SPEC>;

impl Dmsts {
    #[doc = "DMA Active Flag"]
    #[inline(always)]
    pub fn act(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmsts::Act,
        dmsts::Act,
        Dmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmsts::Act,
            dmsts::Act,
            Dmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transfer End Interrupt Flag"]
    #[inline(always)]
    pub fn dtif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmsts::Dtif,
        dmsts::Dtif,
        Dmsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmsts::Dtif,
            dmsts::Dtif,
            Dmsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Escape End Interrupt Flag"]
    #[inline(always)]
    pub fn esif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmsts::Esif,
        dmsts::Esif,
        Dmsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmsts::Esif,
            dmsts::Esif,
            Dmsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmsts {
    #[inline(always)]
    fn default() -> Dmsts {
        <crate::RegValueT<Dmsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Act_SPEC;
    pub type Act = crate::EnumBitfieldStruct<u8, Act_SPEC>;
    impl Act {
        #[doc = "DMAC operation is suspended."]
        pub const _0: Self = Self::new(0);

        #[doc = "DMAC is operating."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtif_SPEC;
    pub type Dtif = crate::EnumBitfieldStruct<u8, Dtif_SPEC>;
    impl Dtif {
        #[doc = "No interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esif_SPEC;
    pub type Esif = crate::EnumBitfieldStruct<u8, Esif_SPEC>;
    impl Esif {
        #[doc = "A transfer escape end interrupt has not been generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "A transfer escape end interrupt has been generated."]
        pub const _1: Self = Self::new(1);
    }
}
