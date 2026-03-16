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
// Generated from SVD 1.30.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:13:42 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DMA Controller for the Ethernet Controller Channel 0"]
unsafe impl ::core::marker::Send for super::Edmac0 {}
unsafe impl ::core::marker::Sync for super::Edmac0 {}
impl super::Edmac0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "EDMAC Mode Register"]
    #[inline(always)]
    pub const fn edmr(&self) -> &'static crate::common::Reg<self::Edmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Edmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "EDMAC Transmit Request Register"]
    #[inline(always)]
    pub const fn edtrr(&self) -> &'static crate::common::Reg<self::Edtrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Edtrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "EDMAC Receive Request Register"]
    #[inline(always)]
    pub const fn edrrr(&self) -> &'static crate::common::Reg<self::Edrrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Edrrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Transmit Descriptor List Start Address Register"]
    #[inline(always)]
    pub const fn tdlar(&self) -> &'static crate::common::Reg<self::Tdlar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdlar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Receive Descriptor List Start Address Register"]
    #[inline(always)]
    pub const fn rdlar(&self) -> &'static crate::common::Reg<self::Rdlar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdlar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "ETHERC/EDMAC Status Register"]
    #[inline(always)]
    pub const fn eesr(&self) -> &'static crate::common::Reg<self::Eesr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eesr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "ETHERC/EDMAC Status Interrupt Enable Register"]
    #[inline(always)]
    pub const fn eesipr(
        &self,
    ) -> &'static crate::common::Reg<self::Eesipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eesipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "ETHERC/EDMAC Transmit/Receive Status Copy Enable Register"]
    #[inline(always)]
    pub const fn trscer(
        &self,
    ) -> &'static crate::common::Reg<self::Trscer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trscer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Missed-Frame Counter Register"]
    #[inline(always)]
    pub const fn rmfcr(&self) -> &'static crate::common::Reg<self::Rmfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Transmit FIFO Threshold Register"]
    #[inline(always)]
    pub const fn tftr(&self) -> &'static crate::common::Reg<self::Tftr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tftr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "FIFO Depth Register"]
    #[inline(always)]
    pub const fn fdr(&self) -> &'static crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Receive Method Control Register"]
    #[inline(always)]
    pub const fn rmcr(&self) -> &'static crate::common::Reg<self::Rmcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Transmit FIFO Underflow Counter"]
    #[inline(always)]
    pub const fn tfucr(&self) -> &'static crate::common::Reg<self::Tfucr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tfucr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Receive FIFO Overflow Counter"]
    #[inline(always)]
    pub const fn rfocr(&self) -> &'static crate::common::Reg<self::Rfocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Independent Output Signal Setting Register"]
    #[inline(always)]
    pub const fn iosr(&self) -> &'static crate::common::Reg<self::Iosr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iosr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Flow Control Start FIFO Threshold Setting Register"]
    #[inline(always)]
    pub const fn fcftr(&self) -> &'static crate::common::Reg<self::Fcftr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcftr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Receive Data Padding Insert Register"]
    #[inline(always)]
    pub const fn rpadir(
        &self,
    ) -> &'static crate::common::Reg<self::Rpadir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rpadir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Transmit Interrupt Setting Register"]
    #[inline(always)]
    pub const fn trimd(&self) -> &'static crate::common::Reg<self::Trimd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trimd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Receive Buffer Write Address Register"]
    #[inline(always)]
    pub const fn rbwar(&self) -> &'static crate::common::Reg<self::Rbwar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rbwar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "Receive Descriptor Fetch Address Register"]
    #[inline(always)]
    pub const fn rdfar(&self) -> &'static crate::common::Reg<self::Rdfar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdfar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "Transmit Buffer Read Address Register"]
    #[inline(always)]
    pub const fn tbrar(&self) -> &'static crate::common::Reg<self::Tbrar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tbrar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "Transmit Descriptor Fetch Address Register"]
    #[inline(always)]
    pub const fn tdfar(&self) -> &'static crate::common::Reg<self::Tdfar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tdfar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edmr_SPEC;
impl crate::sealed::RegSpec for Edmr_SPEC {
    type DataType = u32;
}

#[doc = "EDMAC Mode Register"]
pub type Edmr = crate::RegValueT<Edmr_SPEC>;

impl Edmr {
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn swr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Edmr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Edmr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Transmit/Receive Descriptor Length"]
    #[inline(always)]
    pub fn dl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, edmr::Dl, edmr::Dl, Edmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,edmr::Dl,edmr::Dl,Edmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Big Endian Mode/Little Endian Mode"]
    #[inline(always)]
    pub fn de(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, edmr::De, edmr::De, Edmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,edmr::De,edmr::De,Edmr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Edmr {
    #[inline(always)]
    fn default() -> Edmr {
        <crate::RegValueT<Edmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod edmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl_SPEC;
    pub type Dl = crate::EnumBitfieldStruct<u8, Dl_SPEC>;
    impl Dl {
        #[doc = "16 bytes"]
        pub const _00: Self = Self::new(0);

        #[doc = "32 bytes"]
        pub const _01: Self = Self::new(1);

        #[doc = "64 bytes"]
        pub const _10: Self = Self::new(2);

        #[doc = "16 bytes."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct De_SPEC;
    pub type De = crate::EnumBitfieldStruct<u8, De_SPEC>;
    impl De {
        #[doc = "Big endian mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Little endian mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edtrr_SPEC;
impl crate::sealed::RegSpec for Edtrr_SPEC {
    type DataType = u32;
}

#[doc = "EDMAC Transmit Request Register"]
pub type Edtrr = crate::RegValueT<Edtrr_SPEC>;

impl Edtrr {
    #[doc = "Transmit Request"]
    #[inline(always)]
    pub fn tr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Edtrr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Edtrr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Edtrr {
    #[inline(always)]
    fn default() -> Edtrr {
        <crate::RegValueT<Edtrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edrrr_SPEC;
impl crate::sealed::RegSpec for Edrrr_SPEC {
    type DataType = u32;
}

#[doc = "EDMAC Receive Request Register"]
pub type Edrrr = crate::RegValueT<Edrrr_SPEC>;

impl Edrrr {
    #[doc = "Receive Request"]
    #[inline(always)]
    pub fn rr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        edrrr::Rr,
        edrrr::Rr,
        Edrrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            edrrr::Rr,
            edrrr::Rr,
            Edrrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Edrrr {
    #[inline(always)]
    fn default() -> Edrrr {
        <crate::RegValueT<Edrrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod edrrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rr_SPEC;
    pub type Rr = crate::EnumBitfieldStruct<u8, Rr_SPEC>;
    impl Rr {
        #[doc = "Disable the receive function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read receive descriptor and enable the receive function."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdlar_SPEC;
impl crate::sealed::RegSpec for Tdlar_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Descriptor List Start Address Register"]
pub type Tdlar = crate::RegValueT<Tdlar_SPEC>;

impl NoBitfieldReg<Tdlar_SPEC> for Tdlar {}
impl ::core::default::Default for Tdlar {
    #[inline(always)]
    fn default() -> Tdlar {
        <crate::RegValueT<Tdlar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdlar_SPEC;
impl crate::sealed::RegSpec for Rdlar_SPEC {
    type DataType = u32;
}

#[doc = "Receive Descriptor List Start Address Register"]
pub type Rdlar = crate::RegValueT<Rdlar_SPEC>;

impl NoBitfieldReg<Rdlar_SPEC> for Rdlar {}
impl ::core::default::Default for Rdlar {
    #[inline(always)]
    fn default() -> Rdlar {
        <crate::RegValueT<Rdlar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eesr_SPEC;
impl crate::sealed::RegSpec for Eesr_SPEC {
    type DataType = u32;
}

#[doc = "ETHERC/EDMAC Status Register"]
pub type Eesr = crate::RegValueT<Eesr_SPEC>;

impl Eesr {
    #[doc = "CRC Error Flag"]
    #[inline(always)]
    pub fn cerf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eesr::Cerf,
        eesr::Cerf,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eesr::Cerf,
            eesr::Cerf,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY-LSI Receive Error Flag"]
    #[inline(always)]
    pub fn pre(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eesr::Pre,
        eesr::Pre,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eesr::Pre,
            eesr::Pre,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame-Too-Short Error Flag"]
    #[inline(always)]
    pub fn rtsf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eesr::Rtsf,
        eesr::Rtsf,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eesr::Rtsf,
            eesr::Rtsf,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame-Too-Long Error Flag"]
    #[inline(always)]
    pub fn rtlf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eesr::Rtlf,
        eesr::Rtlf,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eesr::Rtlf,
            eesr::Rtlf,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alignment Error Flag"]
    #[inline(always)]
    pub fn rrf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eesr::Rrf,
        eesr::Rrf,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eesr::Rrf,
            eesr::Rrf,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Address Frame Receive Flag"]
    #[inline(always)]
    pub fn rmaf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eesr::Rmaf,
        eesr::Rmaf,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eesr::Rmaf,
            eesr::Rmaf,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Retry Over Flag"]
    #[inline(always)]
    pub fn tro(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eesr::Tro,
        eesr::Tro,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eesr::Tro,
            eesr::Tro,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Late Collision Detect Flag"]
    #[inline(always)]
    pub fn cd(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, eesr::Cd, eesr::Cd, Eesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,eesr::Cd,eesr::Cd,Eesr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Loss of Carrier Detect Flag"]
    #[inline(always)]
    pub fn dlc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        eesr::Dlc,
        eesr::Dlc,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            eesr::Dlc,
            eesr::Dlc,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Carrier Not Detect Flag"]
    #[inline(always)]
    pub fn cnd(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        eesr::Cnd,
        eesr::Cnd,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            eesr::Cnd,
            eesr::Cnd,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        eesr::Rfof,
        eesr::Rfof,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            eesr::Rfof,
            eesr::Rfof,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Descriptor Empty Flag"]
    #[inline(always)]
    pub fn rde(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        eesr::Rde,
        eesr::Rde,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            eesr::Rde,
            eesr::Rde,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Receive Flag"]
    #[inline(always)]
    pub fn fr(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, eesr::Fr, eesr::Fr, Eesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            eesr::Fr,
            eesr::Fr,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        eesr::Tfuf,
        eesr::Tfuf,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            eesr::Tfuf,
            eesr::Tfuf,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Descriptor Empty Flag"]
    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        eesr::Tde,
        eesr::Tde,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            eesr::Tde,
            eesr::Tde,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Transfer Complete Flag"]
    #[inline(always)]
    pub fn tc(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, eesr::Tc, eesr::Tc, Eesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            eesr::Tc,
            eesr::Tc,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHERC Status Register Source Flag"]
    #[inline(always)]
    pub fn eci(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        eesr::Eci,
        eesr::Eci,
        Eesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            eesr::Eci,
            eesr::Eci,
            Eesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Address Error Flag"]
    #[inline(always)]
    pub fn ade(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        eesr::Ade,
        eesr::Ade,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            eesr::Ade,
            eesr::Ade,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn rfcof(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        eesr::Rfcof,
        eesr::Rfcof,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            eesr::Rfcof,
            eesr::Rfcof,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Abort Detect Flag"]
    #[inline(always)]
    pub fn rabt(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        eesr::Rabt,
        eesr::Rabt,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            eesr::Rabt,
            eesr::Rabt,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Abort Detect Flag"]
    #[inline(always)]
    pub fn tabt(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        eesr::Tabt,
        eesr::Tabt,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            eesr::Tabt,
            eesr::Tabt,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write-Back Complete Flag"]
    #[inline(always)]
    pub fn twb(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        eesr::Twb,
        eesr::Twb,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            eesr::Twb,
            eesr::Twb,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eesr {
    #[inline(always)]
    fn default() -> Eesr {
        <crate::RegValueT<Eesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerf_SPEC;
    pub type Cerf = crate::EnumBitfieldStruct<u8, Cerf_SPEC>;
    impl Cerf {
        #[doc = "CRC error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "CRC error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pre_SPEC;
    pub type Pre = crate::EnumBitfieldStruct<u8, Pre_SPEC>;
    impl Pre {
        #[doc = "PHY-LSI receive error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "PHY-LSI receive error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtsf_SPEC;
    pub type Rtsf = crate::EnumBitfieldStruct<u8, Rtsf_SPEC>;
    impl Rtsf {
        #[doc = "Frame-too-short error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame-too-short error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtlf_SPEC;
    pub type Rtlf = crate::EnumBitfieldStruct<u8, Rtlf_SPEC>;
    impl Rtlf {
        #[doc = "Frame-too-long error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame-too-long error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrf_SPEC;
    pub type Rrf = crate::EnumBitfieldStruct<u8, Rrf_SPEC>;
    impl Rrf {
        #[doc = "Alignment error not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Alignment error detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmaf_SPEC;
    pub type Rmaf = crate::EnumBitfieldStruct<u8, Rmaf_SPEC>;
    impl Rmaf {
        #[doc = "Multicast address frame not received"]
        pub const _0: Self = Self::new(0);

        #[doc = "Multicast address frame received."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tro_SPEC;
    pub type Tro = crate::EnumBitfieldStruct<u8, Tro_SPEC>;
    impl Tro {
        #[doc = "Transmit retry-over condition not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit retry-over condition detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cd_SPEC;
    pub type Cd = crate::EnumBitfieldStruct<u8, Cd_SPEC>;
    impl Cd {
        #[doc = "Late collision not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Late collision detected during frame transmission."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlc_SPEC;
    pub type Dlc = crate::EnumBitfieldStruct<u8, Dlc_SPEC>;
    impl Dlc {
        #[doc = "Loss of carrier not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loss of carrier detected during frame transmission."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cnd_SPEC;
    pub type Cnd = crate::EnumBitfieldStruct<u8, Cnd_SPEC>;
    impl Cnd {
        #[doc = "Carrier detected when transmission started"]
        pub const _0: Self = Self::new(0);

        #[doc = "Carrier not detected during preamble transmission."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfof_SPEC;
    pub type Rfof = crate::EnumBitfieldStruct<u8, Rfof_SPEC>;
    impl Rfof {
        #[doc = "No overflow occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rde_SPEC;
    pub type Rde = crate::EnumBitfieldStruct<u8, Rde_SPEC>;
    impl Rde {
        #[doc = "EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fr_SPEC;
    pub type Fr = crate::EnumBitfieldStruct<u8, Fr_SPEC>;
    impl Fr {
        #[doc = "Frame not received"]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame received and update of the receive descriptor is complete."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfuf_SPEC;
    pub type Tfuf = crate::EnumBitfieldStruct<u8, Tfuf_SPEC>;
    impl Tfuf {
        #[doc = "No underflow occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        #[doc = "EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tc_SPEC;
    pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
    impl Tc {
        #[doc = "Transfer not complete or no transfer requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "All frames indicated in the transmit descriptor were completely transferred to the transmit FIFO."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eci_SPEC;
    pub type Eci = crate::EnumBitfieldStruct<u8, Eci_SPEC>;
    impl Eci {
        #[doc = "ETHERC status interrupt source not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ETHERC status interrupt source detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ade_SPEC;
    pub type Ade = crate::EnumBitfieldStruct<u8, Ade_SPEC>;
    impl Ade {
        #[doc = "Invalid memory address not detected (normal operation)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invalid memory address detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfcof_SPEC;
    pub type Rfcof = crate::EnumBitfieldStruct<u8, Rfcof_SPEC>;
    impl Rfcof {
        #[doc = "Receive frame counter did not overflow"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive frame counter overflowed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rabt_SPEC;
    pub type Rabt = crate::EnumBitfieldStruct<u8, Rabt_SPEC>;
    impl Rabt {
        #[doc = "Frame reception not aborted or no reception requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame reception aborted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabt_SPEC;
    pub type Tabt = crate::EnumBitfieldStruct<u8, Tabt_SPEC>;
    impl Tabt {
        #[doc = "Frame transmission not aborted or no transmission requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame transmission aborted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Twb_SPEC;
    pub type Twb = crate::EnumBitfieldStruct<u8, Twb_SPEC>;
    impl Twb {
        #[doc = "Write-back not complete or no transmission requested"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write-back to the transmit descriptor completed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eesipr_SPEC;
impl crate::sealed::RegSpec for Eesipr_SPEC {
    type DataType = u32;
}

#[doc = "ETHERC/EDMAC Status Interrupt Enable Register"]
pub type Eesipr = crate::RegValueT<Eesipr_SPEC>;

impl Eesipr {
    #[doc = "CRC Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn cerfip(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eesipr::Cerfip,
        eesipr::Cerfip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eesipr::Cerfip,
            eesipr::Cerfip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY-LSI Receive Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn preip(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eesipr::Preip,
        eesipr::Preip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eesipr::Preip,
            eesipr::Preip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame-Too-Short Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn rtsfip(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eesipr::Rtsfip,
        eesipr::Rtsfip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eesipr::Rtsfip,
            eesipr::Rtsfip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame-Too-Long Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn rtlfip(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eesipr::Rtlfip,
        eesipr::Rtlfip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eesipr::Rtlfip,
            eesipr::Rtlfip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Alignment Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn rrfip(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eesipr::Rrfip,
        eesipr::Rrfip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eesipr::Rrfip,
            eesipr::Rrfip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Multicast Address Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    pub fn rmafip(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eesipr::Rmafip,
        eesipr::Rmafip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eesipr::Rmafip,
            eesipr::Rmafip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Retry Over Interrupt Request Enable"]
    #[inline(always)]
    pub fn troip(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eesipr::Troip,
        eesipr::Troip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eesipr::Troip,
            eesipr::Troip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Late Collision Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn cdip(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        eesipr::Cdip,
        eesipr::Cdip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            eesipr::Cdip,
            eesipr::Cdip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Loss of Carrier Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn dlcip(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        eesipr::Dlcip,
        eesipr::Dlcip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            eesipr::Dlcip,
            eesipr::Dlcip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Carrier Not Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn cndip(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        eesipr::Cndip,
        eesipr::Cndip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            eesipr::Cndip,
            eesipr::Cndip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn rfofip(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        eesipr::Rfofip,
        eesipr::Rfofip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            eesipr::Rfofip,
            eesipr::Rfofip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn rdeip(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        eesipr::Rdeip,
        eesipr::Rdeip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            eesipr::Rdeip,
            eesipr::Rdeip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Receive Interrupt Request Enable"]
    #[inline(always)]
    pub fn frip(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        eesipr::Frip,
        eesipr::Frip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            eesipr::Frip,
            eesipr::Frip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO Underflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn tfufip(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        eesipr::Tfufip,
        eesipr::Tfufip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            eesipr::Tfufip,
            eesipr::Tfufip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Descriptor Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn tdeip(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        eesipr::Tdeip,
        eesipr::Tdeip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            eesipr::Tdeip,
            eesipr::Tdeip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Transfer Complete Interrupt Request Enable"]
    #[inline(always)]
    pub fn tcip(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        eesipr::Tcip,
        eesipr::Tcip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            eesipr::Tcip,
            eesipr::Tcip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ETHERC Status Register Source Interrupt Request Enable"]
    #[inline(always)]
    pub fn eciip(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        eesipr::Eciip,
        eesipr::Eciip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            eesipr::Eciip,
            eesipr::Eciip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn adeip(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        eesipr::Adeip,
        eesipr::Adeip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            eesipr::Adeip,
            eesipr::Adeip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Frame Counter Overflow Interrupt Request Enable"]
    #[inline(always)]
    pub fn rfcofip(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        eesipr::Rfcofip,
        eesipr::Rfcofip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            eesipr::Rfcofip,
            eesipr::Rfcofip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn rabtip(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        eesipr::Rabtip,
        eesipr::Rabtip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            eesipr::Rabtip,
            eesipr::Rabtip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Abort Detect Interrupt Request Enable"]
    #[inline(always)]
    pub fn tabtip(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        eesipr::Tabtip,
        eesipr::Tabtip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            eesipr::Tabtip,
            eesipr::Tabtip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write-Back Complete Interrupt Request Enable"]
    #[inline(always)]
    pub fn twbip(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        eesipr::Twbip,
        eesipr::Twbip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            eesipr::Twbip,
            eesipr::Twbip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eesipr {
    #[inline(always)]
    fn default() -> Eesipr {
        <crate::RegValueT<Eesipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eesipr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerfip_SPEC;
    pub type Cerfip = crate::EnumBitfieldStruct<u8, Cerfip_SPEC>;
    impl Cerfip {
        #[doc = "Disable CRC error interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable CRC error interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Preip_SPEC;
    pub type Preip = crate::EnumBitfieldStruct<u8, Preip_SPEC>;
    impl Preip {
        #[doc = "Disable PHY-LSI receive error interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable PHY-LSI receive error interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtsfip_SPEC;
    pub type Rtsfip = crate::EnumBitfieldStruct<u8, Rtsfip_SPEC>;
    impl Rtsfip {
        #[doc = "Disable frame-too-short error interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable frame-too-short error interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtlfip_SPEC;
    pub type Rtlfip = crate::EnumBitfieldStruct<u8, Rtlfip_SPEC>;
    impl Rtlfip {
        #[doc = "Disable frame-too-long error interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable frame-too-long error interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrfip_SPEC;
    pub type Rrfip = crate::EnumBitfieldStruct<u8, Rrfip_SPEC>;
    impl Rrfip {
        #[doc = "Disable alignment error interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable alignment error interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmafip_SPEC;
    pub type Rmafip = crate::EnumBitfieldStruct<u8, Rmafip_SPEC>;
    impl Rmafip {
        #[doc = "Disable multicast address frame receive interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable multicast address frame receive interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Troip_SPEC;
    pub type Troip = crate::EnumBitfieldStruct<u8, Troip_SPEC>;
    impl Troip {
        #[doc = "Disable transmit retry over interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit retry over interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdip_SPEC;
    pub type Cdip = crate::EnumBitfieldStruct<u8, Cdip_SPEC>;
    impl Cdip {
        #[doc = "Disable late collision detected interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable late collision detected interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlcip_SPEC;
    pub type Dlcip = crate::EnumBitfieldStruct<u8, Dlcip_SPEC>;
    impl Dlcip {
        #[doc = "Disable loss of carrier detected interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable loss of carrier detected interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cndip_SPEC;
    pub type Cndip = crate::EnumBitfieldStruct<u8, Cndip_SPEC>;
    impl Cndip {
        #[doc = "Disable carrier not detected interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable carrier not detected interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfofip_SPEC;
    pub type Rfofip = crate::EnumBitfieldStruct<u8, Rfofip_SPEC>;
    impl Rfofip {
        #[doc = "Disable overflow interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable overflow interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdeip_SPEC;
    pub type Rdeip = crate::EnumBitfieldStruct<u8, Rdeip_SPEC>;
    impl Rdeip {
        #[doc = "Disable receive descriptor empty interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable receive descriptor empty interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frip_SPEC;
    pub type Frip = crate::EnumBitfieldStruct<u8, Frip_SPEC>;
    impl Frip {
        #[doc = "Disable frame reception interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable frame reception interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfufip_SPEC;
    pub type Tfufip = crate::EnumBitfieldStruct<u8, Tfufip_SPEC>;
    impl Tfufip {
        #[doc = "Disable underflow interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable underflow interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdeip_SPEC;
    pub type Tdeip = crate::EnumBitfieldStruct<u8, Tdeip_SPEC>;
    impl Tdeip {
        #[doc = "Disable transmit descriptor empty interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit descriptor empty interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcip_SPEC;
    pub type Tcip = crate::EnumBitfieldStruct<u8, Tcip_SPEC>;
    impl Tcip {
        #[doc = "Disable frame transmission complete interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable frame transmission complete interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eciip_SPEC;
    pub type Eciip = crate::EnumBitfieldStruct<u8, Eciip_SPEC>;
    impl Eciip {
        #[doc = "Disable ETHERC status interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ETHERC status interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adeip_SPEC;
    pub type Adeip = crate::EnumBitfieldStruct<u8, Adeip_SPEC>;
    impl Adeip {
        #[doc = "Disable address error interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable address error interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfcofip_SPEC;
    pub type Rfcofip = crate::EnumBitfieldStruct<u8, Rfcofip_SPEC>;
    impl Rfcofip {
        #[doc = "Disable receive frame counter overflow interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable receive frame counter overflow interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rabtip_SPEC;
    pub type Rabtip = crate::EnumBitfieldStruct<u8, Rabtip_SPEC>;
    impl Rabtip {
        #[doc = "Disable receive abort detected interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable receive abort detected interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtip_SPEC;
    pub type Tabtip = crate::EnumBitfieldStruct<u8, Tabtip_SPEC>;
    impl Tabtip {
        #[doc = "Disable transmit abort detected interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit abort detected interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Twbip_SPEC;
    pub type Twbip = crate::EnumBitfieldStruct<u8, Twbip_SPEC>;
    impl Twbip {
        #[doc = "Disable write-back complete interrupt requests"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable write-back complete interrupt requests."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trscer_SPEC;
impl crate::sealed::RegSpec for Trscer_SPEC {
    type DataType = u32;
}

#[doc = "ETHERC/EDMAC Transmit/Receive Status Copy Enable Register"]
pub type Trscer = crate::RegValueT<Trscer_SPEC>;

impl Trscer {
    #[doc = "RRF Flag Copy Enable"]
    #[inline(always)]
    pub fn rrfce(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        trscer::Rrfce,
        trscer::Rrfce,
        Trscer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            trscer::Rrfce,
            trscer::Rrfce,
            Trscer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RMAF Flag Copy Enable"]
    #[inline(always)]
    pub fn rmafce(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        trscer::Rmafce,
        trscer::Rmafce,
        Trscer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            trscer::Rmafce,
            trscer::Rmafce,
            Trscer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trscer {
    #[inline(always)]
    fn default() -> Trscer {
        <crate::RegValueT<Trscer_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trscer {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rrfce_SPEC;
    pub type Rrfce = crate::EnumBitfieldStruct<u8, Rrfce_SPEC>;
    impl Rrfce {
        #[doc = "Reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not reflect the EESR.RRF flag status in the RD0.RFE bit of the receive descriptor."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmafce_SPEC;
    pub type Rmafce = crate::EnumBitfieldStruct<u8, Rmafce_SPEC>;
    impl Rmafce {
        #[doc = "Reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not reflect the EESR.RMAF flag status in the RD0.RFE bit of the receive descriptor."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmfcr_SPEC;
impl crate::sealed::RegSpec for Rmfcr_SPEC {
    type DataType = u32;
}

#[doc = "Missed-Frame Counter Register"]
pub type Rmfcr = crate::RegValueT<Rmfcr_SPEC>;

impl Rmfcr {
    #[doc = "Missed-Frame Counter"]
    #[inline(always)]
    pub fn mfc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Rmfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Rmfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmfcr {
    #[inline(always)]
    fn default() -> Rmfcr {
        <crate::RegValueT<Rmfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftr_SPEC;
impl crate::sealed::RegSpec for Tftr_SPEC {
    type DataType = u32;
}

#[doc = "Transmit FIFO Threshold Register"]
pub type Tftr = crate::RegValueT<Tftr_SPEC>;

impl Tftr {
    #[doc = "Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn tft(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Tftr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Tftr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tftr {
    #[inline(always)]
    fn default() -> Tftr {
        <crate::RegValueT<Tftr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Depth Register"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Receive FIFO Depth"]
    #[inline(always)]
    pub fn rfd(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, fdr::Rfd, fdr::Rfd, Fdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,fdr::Rfd,fdr::Rfd,Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Depth"]
    #[inline(always)]
    pub fn tfd(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, fdr::Tfd, fdr::Tfd, Fdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,fdr::Tfd,fdr::Tfd,Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfd_SPEC;
    pub type Rfd = crate::EnumBitfieldStruct<u8, Rfd_SPEC>;
    impl Rfd {
        #[doc = "4096 bytes"]
        pub const _0_X_0_F: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfd_SPEC;
    pub type Tfd = crate::EnumBitfieldStruct<u8, Tfd_SPEC>;
    impl Tfd {
        #[doc = "2048 bytes"]
        pub const _0_X_07: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmcr_SPEC;
impl crate::sealed::RegSpec for Rmcr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Method Control Register"]
pub type Rmcr = crate::RegValueT<Rmcr_SPEC>;

impl Rmcr {
    #[doc = "Receive Request Reset"]
    #[inline(always)]
    pub fn rnr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rmcr::Rnr,
        rmcr::Rnr,
        Rmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rmcr::Rnr,
            rmcr::Rnr,
            Rmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rmcr {
    #[inline(always)]
    fn default() -> Rmcr {
        <crate::RegValueT<Rmcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rnr_SPEC;
    pub type Rnr = crate::EnumBitfieldStruct<u8, Rnr_SPEC>;
    impl Rnr {
        #[doc = "EDRRR.RR bit (receive request bit) is cleared to 0 when one frame is received"]
        pub const _0: Self = Self::new(0);

        #[doc = "EDRRR.RR bit (receive request bit) is not cleared to 0 when one frame is received."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfucr_SPEC;
impl crate::sealed::RegSpec for Tfucr_SPEC {
    type DataType = u32;
}

#[doc = "Transmit FIFO Underflow Counter"]
pub type Tfucr = crate::RegValueT<Tfucr_SPEC>;

impl Tfucr {
    #[doc = "Transmit FIFO Underflow Count"]
    #[inline(always)]
    pub fn under(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tfucr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tfucr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tfucr {
    #[inline(always)]
    fn default() -> Tfucr {
        <crate::RegValueT<Tfucr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfocr_SPEC;
impl crate::sealed::RegSpec for Rfocr_SPEC {
    type DataType = u32;
}

#[doc = "Receive FIFO Overflow Counter"]
pub type Rfocr = crate::RegValueT<Rfocr_SPEC>;

impl Rfocr {
    #[doc = "Receive FIFO Overflow Count"]
    #[inline(always)]
    pub fn over(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Rfocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Rfocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfocr {
    #[inline(always)]
    fn default() -> Rfocr {
        <crate::RegValueT<Rfocr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iosr_SPEC;
impl crate::sealed::RegSpec for Iosr_SPEC {
    type DataType = u32;
}

#[doc = "Independent Output Signal Setting Register"]
pub type Iosr = crate::RegValueT<Iosr_SPEC>;

impl Iosr {
    #[doc = "External Loopback Mode"]
    #[inline(always)]
    pub fn elb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iosr::Elb,
        iosr::Elb,
        Iosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iosr::Elb,
            iosr::Elb,
            Iosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iosr {
    #[inline(always)]
    fn default() -> Iosr {
        <crate::RegValueT<Iosr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iosr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elb_SPEC;
    pub type Elb = crate::EnumBitfieldStruct<u8, Elb_SPEC>;
    impl Elb {
        #[doc = "Output low on the ET0_EXOUT pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high on the ET0_EXOUT pin."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcftr_SPEC;
impl crate::sealed::RegSpec for Fcftr_SPEC {
    type DataType = u32;
}

#[doc = "Flow Control Start FIFO Threshold Setting Register"]
pub type Fcftr = crate::RegValueT<Fcftr_SPEC>;

impl Fcftr {
    #[doc = "Receive FIFO Data PAUSE Output Threshold"]
    #[inline(always)]
    pub fn rfdo(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Fcftr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Fcftr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive FIFO Frame PAUSE Output Threshold"]
    #[inline(always)]
    pub fn rffo(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Fcftr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Fcftr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcftr {
    #[inline(always)]
    fn default() -> Fcftr {
        <crate::RegValueT<Fcftr_SPEC> as RegisterValue<_>>::new(458759)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpadir_SPEC;
impl crate::sealed::RegSpec for Rpadir_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data Padding Insert Register"]
pub type Rpadir = crate::RegValueT<Rpadir_SPEC>;

impl Rpadir {
    #[doc = "Padding Slot"]
    #[inline(always)]
    pub fn padr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Rpadir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Rpadir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Padding Size"]
    #[inline(always)]
    pub fn pads(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        rpadir::Pads,
        rpadir::Pads,
        Rpadir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            rpadir::Pads,
            rpadir::Pads,
            Rpadir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rpadir {
    #[inline(always)]
    fn default() -> Rpadir {
        <crate::RegValueT<Rpadir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rpadir {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pads_SPEC;
    pub type Pads = crate::EnumBitfieldStruct<u8, Pads_SPEC>;
    impl Pads {
        #[doc = "Do not insert padding"]
        pub const _00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trimd_SPEC;
impl crate::sealed::RegSpec for Trimd_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Interrupt Setting Register"]
pub type Trimd = crate::RegValueT<Trimd_SPEC>;

impl Trimd {
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        trimd::Tis,
        trimd::Tis,
        Trimd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            trimd::Tis,
            trimd::Tis,
            Trimd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Interrupt Mode"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        trimd::Tim,
        trimd::Tim,
        Trimd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            trimd::Tim,
            trimd::Tim,
            Trimd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trimd {
    #[inline(always)]
    fn default() -> Trimd {
        <crate::RegValueT<Trimd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trimd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tis_SPEC;
    pub type Tis = crate::EnumBitfieldStruct<u8, Tis_SPEC>;
    impl Tis {
        #[doc = "Disable transmit interrupts"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit Interrupts."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tim_SPEC;
    pub type Tim = crate::EnumBitfieldStruct<u8, Tim_SPEC>;
    impl Tim {
        #[doc = "Select transmission complete interrupt mode, where an interrupt occurs when a frame is transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select write-back complete interrupt mode, where an interrupt occurs when write-back to the transmit descriptor is complete while the TWBI bit is 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbwar_SPEC;
impl crate::sealed::RegSpec for Rbwar_SPEC {
    type DataType = u32;
}

#[doc = "Receive Buffer Write Address Register"]
pub type Rbwar = crate::RegValueT<Rbwar_SPEC>;

impl NoBitfieldReg<Rbwar_SPEC> for Rbwar {}
impl ::core::default::Default for Rbwar {
    #[inline(always)]
    fn default() -> Rbwar {
        <crate::RegValueT<Rbwar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdfar_SPEC;
impl crate::sealed::RegSpec for Rdfar_SPEC {
    type DataType = u32;
}

#[doc = "Receive Descriptor Fetch Address Register"]
pub type Rdfar = crate::RegValueT<Rdfar_SPEC>;

impl NoBitfieldReg<Rdfar_SPEC> for Rdfar {}
impl ::core::default::Default for Rdfar {
    #[inline(always)]
    fn default() -> Rdfar {
        <crate::RegValueT<Rdfar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbrar_SPEC;
impl crate::sealed::RegSpec for Tbrar_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Buffer Read Address Register"]
pub type Tbrar = crate::RegValueT<Tbrar_SPEC>;

impl NoBitfieldReg<Tbrar_SPEC> for Tbrar {}
impl ::core::default::Default for Tbrar {
    #[inline(always)]
    fn default() -> Tbrar {
        <crate::RegValueT<Tbrar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdfar_SPEC;
impl crate::sealed::RegSpec for Tdfar_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Descriptor Fetch Address Register"]
pub type Tdfar = crate::RegValueT<Tdfar_SPEC>;

impl NoBitfieldReg<Tdfar_SPEC> for Tdfar {}
impl ::core::default::Default for Tdfar {
    #[inline(always)]
    fn default() -> Tdfar {
        <crate::RegValueT<Tdfar_SPEC> as RegisterValue<_>>::new(0)
    }
}
