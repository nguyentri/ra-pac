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
#[doc = r"DMA Controller for EPTPC"]
unsafe impl ::core::marker::Send for super::Ptpedmac {}
unsafe impl ::core::marker::Sync for super::Ptpedmac {}
impl super::Ptpedmac {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "PTPEDMAC Mode Register"]
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

    #[doc = "PTP/EDMAC Status Register"]
    #[inline(always)]
    pub const fn eesr(&self) -> &'static crate::common::Reg<self::Eesr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eesr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "PTP/EDMAC Status Interrupt Enable Register"]
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

    #[doc = "Transmit FIFO Threshold Register"]
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

#[doc = "PTPEDMAC Mode Register"]
pub type Edmr = crate::RegValueT<Edmr_SPEC>;

impl Edmr {
    #[doc = "Big Endian Mode/Little Endian ModeNOTE: This setting applies to data for the transmit/receive buffer. It does not apply to transmit/receive descriptors and registers."]
    #[inline(always)]
    pub fn de(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, edmr::De, edmr::De, Edmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,edmr::De,edmr::De,Edmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit/Receive DescriptorLength"]
    #[inline(always)]
    pub fn dl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, edmr::Dl, edmr::Dl, Edmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,edmr::Dl,edmr::Dl,Edmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Reset (The read value is 0.)"]
    #[inline(always)]
    pub fn swr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        edmr::Swr,
        edmr::Swr,
        Edmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            edmr::Swr,
            edmr::Swr,
            Edmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    pub struct De_SPEC;
    pub type De = crate::EnumBitfieldStruct<u8, De_SPEC>;
    impl De {
        #[doc = "Big endian mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Little endian mode"]
        pub const _1: Self = Self::new(1);
    }
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

        #[doc = "16 bytes"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swr_SPEC;
    pub type Swr = crate::EnumBitfieldStruct<u8, Swr_SPEC>;
    impl Swr {
        #[doc = "no effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "the corresponding channels of the EDMAC and ETHERC are reset.  Registers TDLAR, RDLAR, RMFCR, TFUCR, and RFOCR are not reset."]
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
    #[doc = "Transmit Request (The read value is 0.)"]
    #[inline(always)]
    pub fn tr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        edtrr::Tr,
        edtrr::Tr,
        Edtrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            edtrr::Tr,
            edtrr::Tr,
            Edtrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Edtrr {
    #[inline(always)]
    fn default() -> Edtrr {
        <crate::RegValueT<Edtrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod edtrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tr_SPEC;
    pub type Tr = crate::EnumBitfieldStruct<u8, Tr_SPEC>;
    impl Tr {
        #[doc = "no effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "When 1 is written, the EDMAC reads the corresponding descriptor and transmits frames where the TD0.TACT bit is 1. The TR bit becomes 0 after all the valid frames are transmitted."]
        pub const _1: Self = Self::new(1);
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
        #[doc = "Receive function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive descriptor is read, and the receive function is enabled."]
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

impl Tdlar {
    #[doc = "The start address of the transmit descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\\[1:0\\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b"]
    #[inline(always)]
    pub fn tdlar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tdlar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tdlar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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

impl Rdlar {
    #[doc = "The start address of the receive descriptor list is set. Set the start address according to the descriptor length selected by the EDMR.DL\\[1:0\\] bits.16-byte boundary: Lower 4 bits = 0000b32-byte boundary: Lower 5 bits = 00000b64-byte boundary: Lower 6 bits = 000000b"]
    #[inline(always)]
    pub fn rdlar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rdlar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rdlar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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

#[doc = "PTP/EDMAC Status Register"]
pub type Eesr = crate::RegValueT<Eesr_SPEC>;

impl Eesr {
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

    #[doc = "MAC Address Mismatch Flag"]
    #[inline(always)]
    pub fn mace(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eesr::Mace,
        eesr::Mace,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eesr::Mace,
            eesr::Mace,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP v2 Packet Flag"]
    #[inline(always)]
    pub fn pver(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eesr::Pver,
        eesr::Pver,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eesr::Pver,
            eesr::Pver,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP v2 Message Type Flag"]
    #[inline(always)]
    pub fn r#type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        eesr::Type,
        eesr::Type,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            eesr::Type,
            eesr::Type,
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
    pub struct Twb_SPEC;
    pub type Twb = crate::EnumBitfieldStruct<u8, Twb_SPEC>;
    impl Twb {
        #[doc = "Write-back has not been completed, or transmission has not been requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "Write-back to the transmit descriptor has been completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabt_SPEC;
    pub type Tabt = crate::EnumBitfieldStruct<u8, Tabt_SPEC>;
    impl Tabt {
        #[doc = "Frame transmission has not been aborted or transmission has not been requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame transmission has been aborted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfcof_SPEC;
    pub type Rfcof = crate::EnumBitfieldStruct<u8, Rfcof_SPEC>;
    impl Rfcof {
        #[doc = "Receive frame counter has not overflowed."]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive frame counter has overflowed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ade_SPEC;
    pub type Ade = crate::EnumBitfieldStruct<u8, Ade_SPEC>;
    impl Ade {
        #[doc = "Invalid memory address has not been detected (normal operation)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Invalid memory address has been detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tc_SPEC;
    pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
    impl Tc {
        #[doc = "Transfer has not been completed, or transfer has not been requested."]
        pub const _0: Self = Self::new(0);

        #[doc = "All frames indicated by the transmit descriptor have been completely transferred to the transmit FIFO."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        #[doc = "The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "The EDMAC detects that the transmit descriptor valid bit (TD0.TACT) is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfuf_SPEC;
    pub type Tfuf = crate::EnumBitfieldStruct<u8, Tfuf_SPEC>;
    impl Tfuf {
        #[doc = "Underflow has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fr_SPEC;
    pub type Fr = crate::EnumBitfieldStruct<u8, Fr_SPEC>;
    impl Fr {
        #[doc = "Frame has not been received."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame has been received. The receive descriptor has been updated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rde_SPEC;
    pub type Rde = crate::EnumBitfieldStruct<u8, Rde_SPEC>;
    impl Rde {
        #[doc = "The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "The EDMAC detects that the receive descriptor valid bit (RD0.RACT) is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfof_SPEC;
    pub type Rfof = crate::EnumBitfieldStruct<u8, Rfof_SPEC>;
    impl Rfof {
        #[doc = "Overflow has not occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mace_SPEC;
    pub type Mace = crate::EnumBitfieldStruct<u8, Mace_SPEC>;
    impl Mace {
        #[doc = "The source MAC address of transmit frame data matches the set value."]
        pub const _0: Self = Self::new(0);

        #[doc = "The source MAC address of transmit frame data does not match the set value."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pver_SPEC;
    pub type Pver = crate::EnumBitfieldStruct<u8, Pver_SPEC>;
    impl Pver {
        #[doc = "The current packet is not a PTP v2 packet."]
        pub const _0: Self = Self::new(0);

        #[doc = "The current packet is a PTP v2 packet."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Type_SPEC;
    pub type Type = crate::EnumBitfieldStruct<u8, Type_SPEC>;
    impl Type {
        #[doc = "Sync"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Delay_Req"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Pdelay_Req"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Pdelay_Resp"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Follow_Up"]
        pub const _1000: Self = Self::new(8);

        #[doc = "Delay_Resp"]
        pub const _1001: Self = Self::new(9);

        #[doc = "Pdelay_Resp_Follow_Up"]
        pub const _1010: Self = Self::new(10);

        #[doc = "Announce"]
        pub const _1011: Self = Self::new(11);

        #[doc = "Signaling"]
        pub const _1100: Self = Self::new(12);

        #[doc = "Management"]
        pub const _1101: Self = Self::new(13);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eesipr_SPEC;
impl crate::sealed::RegSpec for Eesipr_SPEC {
    type DataType = u32;
}

#[doc = "PTP/EDMAC Status Interrupt Enable Register"]
pub type Eesipr = crate::RegValueT<Eesipr_SPEC>;

impl Eesipr {
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

    #[doc = "MAC Address Mismatch Interrupt Request Enable"]
    #[inline(always)]
    pub fn maceip(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eesipr::Maceip,
        eesipr::Maceip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eesipr::Maceip,
            eesipr::Maceip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PTP v2 Packet Receive Interrupt Request Enable"]
    #[inline(always)]
    pub fn pverip(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eesipr::Pverip,
        eesipr::Pverip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eesipr::Pverip,
            eesipr::Pverip,
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
    pub struct Twbip_SPEC;
    pub type Twbip = crate::EnumBitfieldStruct<u8, Twbip_SPEC>;
    impl Twbip {
        #[doc = "Write-back complete interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Write-back complete interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtip_SPEC;
    pub type Tabtip = crate::EnumBitfieldStruct<u8, Tabtip_SPEC>;
    impl Tabtip {
        #[doc = "Transmit abort detect interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit abort detect interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfcofip_SPEC;
    pub type Rfcofip = crate::EnumBitfieldStruct<u8, Rfcofip_SPEC>;
    impl Rfcofip {
        #[doc = "Receive frame counter overflow interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive frame counter overflow interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adeip_SPEC;
    pub type Adeip = crate::EnumBitfieldStruct<u8, Adeip_SPEC>;
    impl Adeip {
        #[doc = "Address error interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Address error interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcip_SPEC;
    pub type Tcip = crate::EnumBitfieldStruct<u8, Tcip_SPEC>;
    impl Tcip {
        #[doc = "Frame transmission complete interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame transmission complete interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdeip_SPEC;
    pub type Tdeip = crate::EnumBitfieldStruct<u8, Tdeip_SPEC>;
    impl Tdeip {
        #[doc = "Transmit descriptor empty interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit descriptor empty interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfufip_SPEC;
    pub type Tfufip = crate::EnumBitfieldStruct<u8, Tfufip_SPEC>;
    impl Tfufip {
        #[doc = "Underflow interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Underflow interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frip_SPEC;
    pub type Frip = crate::EnumBitfieldStruct<u8, Frip_SPEC>;
    impl Frip {
        #[doc = "Frame receive interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Frame receive interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdeip_SPEC;
    pub type Rdeip = crate::EnumBitfieldStruct<u8, Rdeip_SPEC>;
    impl Rdeip {
        #[doc = "Receive descriptor empty interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive descriptor empty interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfofip_SPEC;
    pub type Rfofip = crate::EnumBitfieldStruct<u8, Rfofip_SPEC>;
    impl Rfofip {
        #[doc = "Overflow interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Overflow interrupt request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Maceip_SPEC;
    pub type Maceip = crate::EnumBitfieldStruct<u8, Maceip_SPEC>;
    impl Maceip {
        #[doc = "This bit disables an interrupt request generated when the source MAC address of transmit frame data does not match the set value."]
        pub const _0: Self = Self::new(0);

        #[doc = "This bit enables an interrupt request generated when the source MAC address of transmit frame data does not match the set value."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pverip_SPEC;
    pub type Pverip = crate::EnumBitfieldStruct<u8, Pverip_SPEC>;
    impl Pverip {
        #[doc = "PTP v2 packet receive interrupt request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "PTP v2 packet receive interrupt request is enabled."]
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
    #[doc = "Missed-Frame CounterThese bits indicate the number of frames that are discarded and not transferred to the receive buffer during reception."]
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
    #[doc = "Transmit FIFO Threshold00Dh to 200h: The threshold is the set value multiplied by 4.  Example:     00Dh: 52 bytes     040h: 256 bytes     100h: 1024 bytes     200h: 2048 bytes"]
    #[inline(always)]
    pub fn tft(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        tftr::Tft,
        tftr::Tft,
        Tftr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tftr::Tft,
            tftr::Tft,
            Tftr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tftr {
    #[inline(always)]
    fn default() -> Tftr {
        <crate::RegValueT<Tftr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tftr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tft_SPEC;
    pub type Tft = crate::EnumBitfieldStruct<u8, Tft_SPEC>;
    impl Tft {
        #[doc = "Store and forward mode"]
        pub const _0_X_000: Self = Self::new(0);

        #[doc = "The threshold is the set value multiplied by 4. (001h to 00Ch and 201h to 7FFh: Setting prohibited)"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}

#[doc = "Transmit FIFO Threshold Register"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Receive FIFO Depth"]
    #[inline(always)]
    pub fn tfd(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, fdr::Tfd, fdr::Tfd, Fdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,fdr::Tfd,fdr::Tfd,Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO Depth"]
    #[inline(always)]
    pub fn rfd(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, fdr::Rfd, fdr::Rfd, Fdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,fdr::Rfd,fdr::Rfd,Fdr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Tfd_SPEC;
    pub type Tfd = crate::EnumBitfieldStruct<u8, Tfd_SPEC>;
    impl Tfd {
        #[doc = "4096 bytes"]
        pub const _01111: Self = Self::new(15);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfd_SPEC;
    pub type Rfd = crate::EnumBitfieldStruct<u8, Rfd_SPEC>;
    impl Rfd {
        #[doc = "2048 bytes"]
        pub const _00111: Self = Self::new(7);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
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
        #[doc = "EDRRR.RR bit (receive request bit) is set to 0 when one frame has been received."]
        pub const _0: Self = Self::new(0);

        #[doc = "EDRRR.RR bit (receive request bit) is not set to 0 when one frame has been received."]
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
    #[doc = "Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh."]
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
    #[doc = "Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh."]
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
        #[doc = "The ETn_EXOUT pin outputs low."]
        pub const _0: Self = Self::new(0);

        #[doc = "The ETn_EXOUT pin outputs high."]
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
    #[doc = "Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)"]
    #[inline(always)]
    pub fn rffo(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        fcftr::Rffo,
        fcftr::Rffo,
        Fcftr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            fcftr::Rffo,
            fcftr::Rffo,
            Fcftr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO Data PAUSE Output Threshold(When  (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)"]
    #[inline(always)]
    pub fn rfdo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        fcftr::Rfdo,
        fcftr::Rfdo,
        Fcftr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            fcftr::Rfdo,
            fcftr::Rfdo,
            Fcftr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcftr {
    #[inline(always)]
    fn default() -> Fcftr {
        <crate::RegValueT<Fcftr_SPEC> as RegisterValue<_>>::new(458759)
    }
}
pub mod fcftr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffo_SPEC;
    pub type Rffo = crate::EnumBitfieldStruct<u8, Rffo_SPEC>;
    impl Rffo {
        #[doc = "When 2 receive frames have been stored in the receive FIFO."]
        pub const _000: Self = Self::new(0);

        #[doc = "When 4 receive frames have been stored in the receive FIFO."]
        pub const _001: Self = Self::new(1);

        #[doc = "When 6 receive frames have been stored in the receive FIFO."]
        pub const _010: Self = Self::new(2);

        #[doc = "When 8 receive frames have been stored in the receive FIFO."]
        pub const _011: Self = Self::new(3);

        #[doc = "When 10 receive frames have been stored in the receive FIFO."]
        pub const _100: Self = Self::new(4);

        #[doc = "When 12 receive frames have been stored in the receive FIFO."]
        pub const _101: Self = Self::new(5);

        #[doc = "When 14 receive frames have been stored in the receive FIFO."]
        pub const _110: Self = Self::new(6);

        #[doc = "When 16 receive frames have been stored in the receive FIFO."]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdo_SPEC;
    pub type Rfdo = crate::EnumBitfieldStruct<u8, Rfdo_SPEC>;
    impl Rfdo {
        #[doc = "When  224 ( 256 - 32) bytes of data is stored in the receive FIFO."]
        pub const _000: Self = Self::new(0);

        #[doc = "When  480 ( 512 - 32) bytes of data is stored in the receive FIFO."]
        pub const _001: Self = Self::new(1);

        #[doc = "When  736 ( 768 - 32) bytes of data is stored in the receive FIFO."]
        pub const _010: Self = Self::new(2);

        #[doc = "When  992 (1024 - 32) bytes of data is stored in the receive FIFO."]
        pub const _011: Self = Self::new(3);

        #[doc = "When 1248 (1280 - 32) bytes of data is stored in the receive FIFO."]
        pub const _100: Self = Self::new(4);

        #[doc = "When 1504 (1536 - 32) bytes of data is stored in the receive FIFO."]
        pub const _101: Self = Self::new(5);

        #[doc = "When 1760 (1792 - 32) bytes of data is stored in the receive FIFO."]
        pub const _110: Self = Self::new(6);

        #[doc = "When 2016 (2048 - 32) bytes of data is stored in the receive FIFO."]
        pub const _111: Self = Self::new(7);
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

    #[doc = "Padding Slot"]
    #[inline(always)]
    pub fn padr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        rpadir::Padr,
        rpadir::Padr,
        Rpadir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            rpadir::Padr,
            rpadir::Padr,
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
        #[doc = "No padding is inserted."]
        pub const _00: Self = Self::new(0);

        #[doc = "1 byte is inserted."]
        pub const _01: Self = Self::new(1);

        #[doc = "2 bytes are inserted."]
        pub const _10: Self = Self::new(2);

        #[doc = "3 bytes are inserted."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padr_SPEC;
    pub type Padr = crate::EnumBitfieldStruct<u8, Padr_SPEC>;
    impl Padr {
        #[doc = "Padding is inserted at the head of received data."]
        pub const _00_H: Self = Self::new(0);

        #[doc = "Padding is inserted between the (PADR)th byte and (PADR+1)th byte of received data."]
        pub const OTHERS: Self = Self::new(0);
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

    #[doc = "Transmit Interrupt EnableSet the EESR.TWB flag to 1 in the mode selected by the TIM bit to notify an interrupt."]
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
}
impl ::core::default::Default for Trimd {
    #[inline(always)]
    fn default() -> Trimd {
        <crate::RegValueT<Trimd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trimd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tim_SPEC;
    pub type Tim = crate::EnumBitfieldStruct<u8, Tim_SPEC>;
    impl Tim {
        #[doc = "Transmission complete interrupt mode: An interrupt occurs when a frame has been transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Write-back complete interrupt mode: An interrupt occurs when write-back to the transmit descriptor has been completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tis_SPEC;
    pub type Tis = crate::EnumBitfieldStruct<u8, Tis_SPEC>;
    impl Tis {
        #[doc = "Transmit Interrupt is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit Interrupt is enabled."]
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

impl Rbwar {
    #[doc = "Receive Buffer Write Address RegisterThe RBWAR register indicates the last address that the EDMAC has written data to when writing to the receive buffer.Refer to the address indicated by the RBWAR register to recognize which address in the receive buffer the EDMAC is writing data to. Note that the address that the EDMAC is outputting to the receive buffer may not match the read value of the RBWAR register during data reception."]
    #[inline(always)]
    pub fn rbwar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rbwar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rbwar_SPEC,crate::common::R>::from_register(self,0)
    }
}
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

impl Rdfar {
    #[doc = "Receive Descriptor Fetch Address RegisterThe RDFAR register indicates the start address of the last fetched receive descriptor when the EDMAC fetches descriptor information from the receive descriptor.Refer to the address indicated by the RDFAR register to recognize which receive descriptor information the EDMAC is using for the current processing. Note that the address of the receive descriptor that the EDMAC fetches may not match the read value of the RDFAR register during data reception."]
    #[inline(always)]
    pub fn rdfar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rdfar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rdfar_SPEC,crate::common::R>::from_register(self,0)
    }
}
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

impl Tbrar {
    #[doc = "Transmit Buffer Read Address RegisterThe TBRAR register indicates the last address that the EDMAC has read data from when reading data from the transmit buffer.Refer to the address indicated by the TBRAR register to recognize which address in the transmit buffer the EDMAC is reading from. Note that the address that the EDMAC is outputting to the transmit buffer may not match the read value of the TBRAR register."]
    #[inline(always)]
    pub fn tbrar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tbrar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tbrar_SPEC,crate::common::R>::from_register(self,0)
    }
}
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

impl Tdfar {
    #[doc = "Transmit Descriptor Fetch Address RegisterThe TDFAR register indicates the start address of the last fetched transmit descriptor when the EDMAC fetches descriptor information from the transmit descriptor.Refer to the address indicated by the TDFAR register to recognize which transmit descriptor information the EDMAC is using for the current processing. Note that the address of the transmit descriptor that the EDMAC fetches may not match the read value of the TDFAR register."]
    #[inline(always)]
    pub fn tdfar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tdfar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tdfar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdfar {
    #[inline(always)]
    fn default() -> Tdfar {
        <crate::RegValueT<Tdfar_SPEC> as RegisterValue<_>>::new(0)
    }
}
