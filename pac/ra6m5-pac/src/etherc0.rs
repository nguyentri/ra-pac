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
// Generated from SVD 1.41.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:52:06 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Ethernet Controller Channel 0"]
unsafe impl ::core::marker::Send for super::Etherc0 {}
unsafe impl ::core::marker::Sync for super::Etherc0 {}
impl super::Etherc0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "ETHERC Mode Register"]
    #[inline(always)]
    pub const fn ecmr(&self) -> &'static crate::common::Reg<self::Ecmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Receive Frame Maximum Length Register"]
    #[inline(always)]
    pub const fn rflr(&self) -> &'static crate::common::Reg<self::Rflr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rflr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "ETHERC Status Register"]
    #[inline(always)]
    pub const fn ecsr(&self) -> &'static crate::common::Reg<self::Ecsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "ETHERC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ecsipr(
        &self,
    ) -> &'static crate::common::Reg<self::Ecsipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecsipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "PHY Interface Register"]
    #[inline(always)]
    pub const fn pir(&self) -> &'static crate::common::Reg<self::Pir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "PHY Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &'static crate::common::Reg<self::Psr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Psr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Random Number Generation Counter Upper Limit Setting Register"]
    #[inline(always)]
    pub const fn rdmlr(&self) -> &'static crate::common::Reg<self::Rdmlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdmlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Interpacket Gap Register"]
    #[inline(always)]
    pub const fn ipgr(&self) -> &'static crate::common::Reg<self::Ipgr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipgr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Automatic PAUSE Frame Register"]
    #[inline(always)]
    pub const fn apr(&self) -> &'static crate::common::Reg<self::Apr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Apr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Manual PAUSE Frame Register"]
    #[inline(always)]
    pub const fn mpr(&self) -> &'static crate::common::Reg<self::Mpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Received PAUSE Frame Counter"]
    #[inline(always)]
    pub const fn rfcf(&self) -> &'static crate::common::Reg<self::Rfcf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rfcf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "PAUSE Frame Retransmit Count Setting Register"]
    #[inline(always)]
    pub const fn tpauser(
        &self,
    ) -> &'static crate::common::Reg<self::Tpauser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpauser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "PAUSE Frame Retransmit Counter"]
    #[inline(always)]
    pub const fn tpausecr(
        &self,
    ) -> &'static crate::common::Reg<self::Tpausecr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tpausecr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Broadcast Frame Receive Count Setting Register"]
    #[inline(always)]
    pub const fn bcfrr(&self) -> &'static crate::common::Reg<self::Bcfrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcfrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "MAC Address Upper Bit Register"]
    #[inline(always)]
    pub const fn mahr(&self) -> &'static crate::common::Reg<self::Mahr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mahr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "MAC Address Lower Bit Register"]
    #[inline(always)]
    pub const fn malr(&self) -> &'static crate::common::Reg<self::Malr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Malr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "Transmit Retry Over Counter Register"]
    #[inline(always)]
    pub const fn trocr(&self) -> &'static crate::common::Reg<self::Trocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Late Collision Detect Counter Register"]
    #[inline(always)]
    pub const fn cdcr(&self) -> &'static crate::common::Reg<self::Cdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "Lost Carrier Counter Register"]
    #[inline(always)]
    pub const fn lccr(&self) -> &'static crate::common::Reg<self::Lccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "Carrier Not Detect Counter Register"]
    #[inline(always)]
    pub const fn cndcr(&self) -> &'static crate::common::Reg<self::Cndcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cndcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[doc = "CRC Error Frame Receive Counter Register"]
    #[inline(always)]
    pub const fn cefcr(&self) -> &'static crate::common::Reg<self::Cefcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cefcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "Frame Receive Error Counter Register"]
    #[inline(always)]
    pub const fn frecr(&self) -> &'static crate::common::Reg<self::Frecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Frecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[doc = "Too-Short Frame Receive Counter Register"]
    #[inline(always)]
    pub const fn tsfrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Tsfrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tsfrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[doc = "Too-Long Frame Receive Counter Register"]
    #[inline(always)]
    pub const fn tlfrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Tlfrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tlfrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "Received Alignment Error Frame Counter Register"]
    #[inline(always)]
    pub const fn rfcr(&self) -> &'static crate::common::Reg<self::Rfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "Multicast Address Frame Receive Counter Register"]
    #[inline(always)]
    pub const fn mafcr(&self) -> &'static crate::common::Reg<self::Mafcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mafcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmr_SPEC;
impl crate::sealed::RegSpec for Ecmr_SPEC {
    type DataType = u32;
}

#[doc = "ETHERC Mode Register"]
pub type Ecmr = crate::RegValueT<Ecmr_SPEC>;

impl Ecmr {
    #[doc = "Promiscuous Mode"]
    #[inline(always)]
    pub fn prm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecmr::Prm,
        ecmr::Prm,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecmr::Prm,
            ecmr::Prm,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Duplex Mode"]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ecmr::Dm, ecmr::Dm, Ecmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ecmr::Dm,ecmr::Dm,Ecmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bit Rate"]
    #[inline(always)]
    pub fn rtm(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecmr::Rtm,
        ecmr::Rtm,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecmr::Rtm,
            ecmr::Rtm,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Internal Loopback Mode"]
    #[inline(always)]
    pub fn ilb(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ecmr::Ilb,
        ecmr::Ilb,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ecmr::Ilb,
            ecmr::Ilb,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmission Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ecmr::Te, ecmr::Te, Ecmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ecmr::Te,ecmr::Te,Ecmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reception Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ecmr::Re, ecmr::Re, Ecmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ecmr::Re,ecmr::Re,Ecmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn mpde(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ecmr::Mpde,
        ecmr::Mpde,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ecmr::Mpde,
            ecmr::Mpde,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CRC Error Frame Receive Mode"]
    #[inline(always)]
    pub fn prcef(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ecmr::Prcef,
        ecmr::Prcef,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ecmr::Prcef,
            ecmr::Prcef,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Flow Control Operating Mode"]
    #[inline(always)]
    pub fn txf(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ecmr::Txf,
        ecmr::Txf,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ecmr::Txf,
            ecmr::Txf,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Flow Control Operating Mode"]
    #[inline(always)]
    pub fn rxf(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ecmr::Rxf,
        ecmr::Rxf,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ecmr::Rxf,
            ecmr::Rxf,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PAUSE Frame Receive Mode"]
    #[inline(always)]
    pub fn pfr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ecmr::Pfr,
        ecmr::Pfr,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ecmr::Pfr,
            ecmr::Pfr,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "0 Time PAUSE Frame Enable"]
    #[inline(always)]
    pub fn zpf(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ecmr::Zpf,
        ecmr::Zpf,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ecmr::Zpf,
            ecmr::Zpf,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PAUSE Frame Transmit"]
    #[inline(always)]
    pub fn tpc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ecmr::Tpc,
        ecmr::Tpc,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ecmr::Tpc,
            ecmr::Tpc,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecmr {
    #[inline(always)]
    fn default() -> Ecmr {
        <crate::RegValueT<Ecmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prm_SPEC;
    pub type Prm = crate::EnumBitfieldStruct<u8, Prm_SPEC>;
    impl Prm {
        #[doc = "Disable promiscuous mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable promiscuous mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dm_SPEC;
    pub type Dm = crate::EnumBitfieldStruct<u8, Dm_SPEC>;
    impl Dm {
        #[doc = "Half-duplex mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Full-duplex mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtm_SPEC;
    pub type Rtm = crate::EnumBitfieldStruct<u8, Rtm_SPEC>;
    impl Rtm {
        #[doc = "10 Mbps"]
        pub const _0: Self = Self::new(0);

        #[doc = "100 Mbps."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilb_SPEC;
    pub type Ilb = crate::EnumBitfieldStruct<u8, Ilb_SPEC>;
    impl Ilb {
        #[doc = "Perform normal data transmission or reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Loop data back in the ETHERC when full-duplex mode is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Disable transmit function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit function."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Disable receive function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable receive function."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpde_SPEC;
    pub type Mpde = crate::EnumBitfieldStruct<u8, Mpde_SPEC>;
    impl Mpde {
        #[doc = "Disable Magic Packet detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable Magic Packet detection."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prcef_SPEC;
    pub type Prcef = crate::EnumBitfieldStruct<u8, Prcef_SPEC>;
    impl Prcef {
        #[doc = "Notify EDMAC of a CRC error"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not notify EDMAC of a CRC error."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txf_SPEC;
    pub type Txf = crate::EnumBitfieldStruct<u8, Txf_SPEC>;
    impl Txf {
        #[doc = "Disable automatic PAUSE frame transmission (PAUSE frame is not automatically transmitted)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable automatic PAUSE frame transmission (PAUSE frame is automatically transmitted as required)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxf_SPEC;
    pub type Rxf = crate::EnumBitfieldStruct<u8, Rxf_SPEC>;
    impl Rxf {
        #[doc = "Disable PAUSE frame detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable PAUSE frame detection."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfr_SPEC;
    pub type Pfr = crate::EnumBitfieldStruct<u8, Pfr_SPEC>;
    impl Pfr {
        #[doc = "Do not transfer PAUSE frame to the EDMAC"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transfer PAUSE frame to the EDMAC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zpf_SPEC;
    pub type Zpf = crate::EnumBitfieldStruct<u8, Zpf_SPEC>;
    impl Zpf {
        #[doc = "Do not use PAUSE frames that containing a pause_time parameter of 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use PAUSE frames that containing a pause_time parameter of 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpc_SPEC;
    pub type Tpc = crate::EnumBitfieldStruct<u8, Tpc_SPEC>;
    impl Tpc {
        #[doc = "Transmit PAUSE frame even during a PAUSE period"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not transmit PAUSE frame during a PAUSE period."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rflr_SPEC;
impl crate::sealed::RegSpec for Rflr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Frame Maximum Length Register"]
pub type Rflr = crate::RegValueT<Rflr_SPEC>;

impl Rflr {
    #[doc = "Receive Frame Maximum Length"]
    #[inline(always)]
    pub fn rfl(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Rflr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Rflr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rflr {
    #[inline(always)]
    fn default() -> Rflr {
        <crate::RegValueT<Rflr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecsr_SPEC;
impl crate::sealed::RegSpec for Ecsr_SPEC {
    type DataType = u32;
}

#[doc = "ETHERC Status Register"]
pub type Ecsr = crate::RegValueT<Ecsr_SPEC>;

impl Ecsr {
    #[doc = "False Carrier Detect Flag"]
    #[inline(always)]
    pub fn icd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecsr::Icd,
        ecsr::Icd,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecsr::Icd,
            ecsr::Icd,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Magic Packet Detect Flag"]
    #[inline(always)]
    pub fn mpd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ecsr::Mpd,
        ecsr::Mpd,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ecsr::Mpd,
            ecsr::Mpd,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Link Signal Change Flag"]
    #[inline(always)]
    pub fn lchng(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecsr::Lchng,
        ecsr::Lchng,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecsr::Lchng,
            ecsr::Lchng,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PAUSE Frame Retransmit Over Flag"]
    #[inline(always)]
    pub fn psrto(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ecsr::Psrto,
        ecsr::Psrto,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ecsr::Psrto,
            ecsr::Psrto,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Continuous Broadcast Frame Reception Flag"]
    #[inline(always)]
    pub fn bfr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ecsr::Bfr,
        ecsr::Bfr,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ecsr::Bfr,
            ecsr::Bfr,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecsr {
    #[inline(always)]
    fn default() -> Ecsr {
        <crate::RegValueT<Ecsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icd_SPEC;
    pub type Icd = crate::EnumBitfieldStruct<u8, Icd_SPEC>;
    impl Icd {
        #[doc = "PHY-LSI has not detected a false carrier on the line"]
        pub const _0: Self = Self::new(0);

        #[doc = "PHY-LSI detected a false carrier on the line."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpd_SPEC;
    pub type Mpd = crate::EnumBitfieldStruct<u8, Mpd_SPEC>;
    impl Mpd {
        #[doc = "Magic Packet not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Magic Packet detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lchng_SPEC;
    pub type Lchng = crate::EnumBitfieldStruct<u8, Lchng_SPEC>;
    impl Lchng {
        #[doc = "Change in the ET0_LINKSTA signal not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Change in the ET0_LINKSTA signal detected (high to low, or low to high)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psrto_SPEC;
    pub type Psrto = crate::EnumBitfieldStruct<u8, Psrto_SPEC>;
    impl Psrto {
        #[doc = "PAUSE frame retransmit count has not reached the upper limit"]
        pub const _0: Self = Self::new(0);

        #[doc = "PAUSE frame retransmit count reached the upper limit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfr_SPEC;
    pub type Bfr = crate::EnumBitfieldStruct<u8, Bfr_SPEC>;
    impl Bfr {
        #[doc = "Continuous reception of broadcast frames not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Continuous reception of broadcast frames detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecsipr_SPEC;
impl crate::sealed::RegSpec for Ecsipr_SPEC {
    type DataType = u32;
}

#[doc = "ETHERC Interrupt Enable Register"]
pub type Ecsipr = crate::RegValueT<Ecsipr_SPEC>;

impl Ecsipr {
    #[doc = "False Carrier Detect Interrupt Enable"]
    #[inline(always)]
    pub fn icdip(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecsipr::Icdip,
        ecsipr::Icdip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecsipr::Icdip,
            ecsipr::Icdip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Magic Packet Detect Interrupt Enable"]
    #[inline(always)]
    pub fn mpdip(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ecsipr::Mpdip,
        ecsipr::Mpdip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ecsipr::Mpdip,
            ecsipr::Mpdip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LINK Signal Change Interrupt Enable"]
    #[inline(always)]
    pub fn lchngip(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecsipr::Lchngip,
        ecsipr::Lchngip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecsipr::Lchngip,
            ecsipr::Lchngip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PAUSE Frame Retransmit Over Interrupt Enable"]
    #[inline(always)]
    pub fn psrtoip(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ecsipr::Psrtoip,
        ecsipr::Psrtoip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ecsipr::Psrtoip,
            ecsipr::Psrtoip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Continuous Broadcast Frame Reception Interrupt Enable"]
    #[inline(always)]
    pub fn bfsipr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ecsipr::Bfsipr,
        ecsipr::Bfsipr,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ecsipr::Bfsipr,
            ecsipr::Bfsipr,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecsipr {
    #[inline(always)]
    fn default() -> Ecsipr {
        <crate::RegValueT<Ecsipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecsipr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icdip_SPEC;
    pub type Icdip = crate::EnumBitfieldStruct<u8, Icdip_SPEC>;
    impl Icdip {
        #[doc = "Disable interrupt notification"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt notification."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpdip_SPEC;
    pub type Mpdip = crate::EnumBitfieldStruct<u8, Mpdip_SPEC>;
    impl Mpdip {
        #[doc = "Disable interrupt notification"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt notification."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lchngip_SPEC;
    pub type Lchngip = crate::EnumBitfieldStruct<u8, Lchngip_SPEC>;
    impl Lchngip {
        #[doc = "Disable interrupt notification"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt notification."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psrtoip_SPEC;
    pub type Psrtoip = crate::EnumBitfieldStruct<u8, Psrtoip_SPEC>;
    impl Psrtoip {
        #[doc = "Disable interrupt notification"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt notification."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfsipr_SPEC;
    pub type Bfsipr = crate::EnumBitfieldStruct<u8, Bfsipr_SPEC>;
    impl Bfsipr {
        #[doc = "Disable interrupt notification"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt notification."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pir_SPEC;
impl crate::sealed::RegSpec for Pir_SPEC {
    type DataType = u32;
}

#[doc = "PHY Interface Register"]
pub type Pir = crate::RegValueT<Pir_SPEC>;

impl Pir {
    #[doc = "MII/RMII Management Data Clock"]
    #[inline(always)]
    pub fn mdc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MII/RMII Management Mode"]
    #[inline(always)]
    pub fn mmd(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pir::Mmd, pir::Mmd, Pir_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pir::Mmd,pir::Mmd,Pir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MII/RMII Management Data-Out"]
    #[inline(always)]
    pub fn mdo(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "MII/RMII Management Data-In"]
    #[inline(always)]
    pub fn mdi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pir {
    #[inline(always)]
    fn default() -> Pir {
        <crate::RegValueT<Pir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pir {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmd_SPEC;
    pub type Mmd = crate::EnumBitfieldStruct<u8, Mmd_SPEC>;
    impl Mmd {
        #[doc = "Read"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr_SPEC;
impl crate::sealed::RegSpec for Psr_SPEC {
    type DataType = u32;
}

#[doc = "PHY Status Register"]
pub type Psr = crate::RegValueT<Psr_SPEC>;

impl Psr {
    #[doc = "ET0_LINKSTA Pin Status Flag"]
    #[inline(always)]
    pub fn lmon(self) -> crate::common::RegisterFieldBool<0, 1, 0, Psr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Psr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        <crate::RegValueT<Psr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdmlr_SPEC;
impl crate::sealed::RegSpec for Rdmlr_SPEC {
    type DataType = u32;
}

#[doc = "Random Number Generation Counter Upper Limit Setting Register"]
pub type Rdmlr = crate::RegValueT<Rdmlr_SPEC>;

impl Rdmlr {
    #[doc = "Random Number Generation Counter"]
    #[inline(always)]
    pub fn rmd(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Rdmlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,Rdmlr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdmlr {
    #[inline(always)]
    fn default() -> Rdmlr {
        <crate::RegValueT<Rdmlr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipgr_SPEC;
impl crate::sealed::RegSpec for Ipgr_SPEC {
    type DataType = u32;
}

#[doc = "Interpacket Gap Register"]
pub type Ipgr = crate::RegValueT<Ipgr_SPEC>;

impl Ipgr {
    #[inline(always)]
    pub fn ipg(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Ipgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Ipgr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipgr {
    #[inline(always)]
    fn default() -> Ipgr {
        <crate::RegValueT<Ipgr_SPEC> as RegisterValue<_>>::new(20)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apr_SPEC;
impl crate::sealed::RegSpec for Apr_SPEC {
    type DataType = u32;
}

#[doc = "Automatic PAUSE Frame Register"]
pub type Apr = crate::RegValueT<Apr_SPEC>;

impl Apr {
    #[doc = "Automatic PAUSE Time Setting"]
    #[inline(always)]
    pub fn ap(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Apr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Apr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Apr {
    #[inline(always)]
    fn default() -> Apr {
        <crate::RegValueT<Apr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpr_SPEC;
impl crate::sealed::RegSpec for Mpr_SPEC {
    type DataType = u32;
}

#[doc = "Manual PAUSE Frame Register"]
pub type Mpr = crate::RegValueT<Mpr_SPEC>;

impl Mpr {
    #[doc = "Manual PAUSE Time Setting"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpr {
    #[inline(always)]
    fn default() -> Mpr {
        <crate::RegValueT<Mpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcf_SPEC;
impl crate::sealed::RegSpec for Rfcf_SPEC {
    type DataType = u32;
}

#[doc = "Received PAUSE Frame Counter"]
pub type Rfcf = crate::RegValueT<Rfcf_SPEC>;

impl Rfcf {
    #[doc = "Received PAUSE Frame Count"]
    #[inline(always)]
    pub fn rpause(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rfcf_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rfcf_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfcf {
    #[inline(always)]
    fn default() -> Rfcf {
        <crate::RegValueT<Rfcf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpauser_SPEC;
impl crate::sealed::RegSpec for Tpauser_SPEC {
    type DataType = u32;
}

#[doc = "PAUSE Frame Retransmit Count Setting Register"]
pub type Tpauser = crate::RegValueT<Tpauser_SPEC>;

impl Tpauser {
    #[inline(always)]
    pub fn tpause(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tpauser_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tpauser_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpauser {
    #[inline(always)]
    fn default() -> Tpauser {
        <crate::RegValueT<Tpauser_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpausecr_SPEC;
impl crate::sealed::RegSpec for Tpausecr_SPEC {
    type DataType = u32;
}

#[doc = "PAUSE Frame Retransmit Counter"]
pub type Tpausecr = crate::RegValueT<Tpausecr_SPEC>;

impl Tpausecr {
    #[doc = "PAUSE Frame Retransmit Count"]
    #[inline(always)]
    pub fn txp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Tpausecr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Tpausecr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpausecr {
    #[inline(always)]
    fn default() -> Tpausecr {
        <crate::RegValueT<Tpausecr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcfrr_SPEC;
impl crate::sealed::RegSpec for Bcfrr_SPEC {
    type DataType = u32;
}

#[doc = "Broadcast Frame Receive Count Setting Register"]
pub type Bcfrr = crate::RegValueT<Bcfrr_SPEC>;

impl Bcfrr {
    #[inline(always)]
    pub fn bcf(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Bcfrr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Bcfrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcfrr {
    #[inline(always)]
    fn default() -> Bcfrr {
        <crate::RegValueT<Bcfrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mahr_SPEC;
impl crate::sealed::RegSpec for Mahr_SPEC {
    type DataType = u32;
}

#[doc = "MAC Address Upper Bit Register"]
pub type Mahr = crate::RegValueT<Mahr_SPEC>;

impl Mahr {
    #[doc = "MAC Address Upper Bit"]
    #[inline(always)]
    pub fn mahr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mahr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mahr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mahr {
    #[inline(always)]
    fn default() -> Mahr {
        <crate::RegValueT<Mahr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Malr_SPEC;
impl crate::sealed::RegSpec for Malr_SPEC {
    type DataType = u32;
}

#[doc = "MAC Address Lower Bit Register"]
pub type Malr = crate::RegValueT<Malr_SPEC>;

impl Malr {
    #[doc = "MAC Address Lower Bit"]
    #[inline(always)]
    pub fn malr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Malr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Malr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Malr {
    #[inline(always)]
    fn default() -> Malr {
        <crate::RegValueT<Malr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trocr_SPEC;
impl crate::sealed::RegSpec for Trocr_SPEC {
    type DataType = u32;
}

#[doc = "Transmit Retry Over Counter Register"]
pub type Trocr = crate::RegValueT<Trocr_SPEC>;

impl Trocr {
    #[doc = "Transmit Retry Over Counter"]
    #[inline(always)]
    pub fn trocr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Trocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Trocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Trocr {
    #[inline(always)]
    fn default() -> Trocr {
        <crate::RegValueT<Trocr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdcr_SPEC;
impl crate::sealed::RegSpec for Cdcr_SPEC {
    type DataType = u32;
}

#[doc = "Late Collision Detect Counter Register"]
pub type Cdcr = crate::RegValueT<Cdcr_SPEC>;

impl Cdcr {
    #[doc = "Late Collision Detect Counter"]
    #[inline(always)]
    pub fn cdcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdcr {
    #[inline(always)]
    fn default() -> Cdcr {
        <crate::RegValueT<Cdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lccr_SPEC;
impl crate::sealed::RegSpec for Lccr_SPEC {
    type DataType = u32;
}

#[doc = "Lost Carrier Counter Register"]
pub type Lccr = crate::RegValueT<Lccr_SPEC>;

impl Lccr {
    #[doc = "Lost Carrier Counter"]
    #[inline(always)]
    pub fn lccr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lccr {
    #[inline(always)]
    fn default() -> Lccr {
        <crate::RegValueT<Lccr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndcr_SPEC;
impl crate::sealed::RegSpec for Cndcr_SPEC {
    type DataType = u32;
}

#[doc = "Carrier Not Detect Counter Register"]
pub type Cndcr = crate::RegValueT<Cndcr_SPEC>;

impl Cndcr {
    #[doc = "Carrier Not Detect Counter"]
    #[inline(always)]
    pub fn cndcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cndcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cndcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cndcr {
    #[inline(always)]
    fn default() -> Cndcr {
        <crate::RegValueT<Cndcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cefcr_SPEC;
impl crate::sealed::RegSpec for Cefcr_SPEC {
    type DataType = u32;
}

#[doc = "CRC Error Frame Receive Counter Register"]
pub type Cefcr = crate::RegValueT<Cefcr_SPEC>;

impl Cefcr {
    #[doc = "CRC Error Frame Receive Counter"]
    #[inline(always)]
    pub fn cefcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cefcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cefcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cefcr {
    #[inline(always)]
    fn default() -> Cefcr {
        <crate::RegValueT<Cefcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frecr_SPEC;
impl crate::sealed::RegSpec for Frecr_SPEC {
    type DataType = u32;
}

#[doc = "Frame Receive Error Counter Register"]
pub type Frecr = crate::RegValueT<Frecr_SPEC>;

impl Frecr {
    #[doc = "Frame Receive Error Counter"]
    #[inline(always)]
    pub fn frecr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Frecr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Frecr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Frecr {
    #[inline(always)]
    fn default() -> Frecr {
        <crate::RegValueT<Frecr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsfrcr_SPEC;
impl crate::sealed::RegSpec for Tsfrcr_SPEC {
    type DataType = u32;
}

#[doc = "Too-Short Frame Receive Counter Register"]
pub type Tsfrcr = crate::RegValueT<Tsfrcr_SPEC>;

impl Tsfrcr {
    #[doc = "Too-Short Frame Receive Counter"]
    #[inline(always)]
    pub fn tsfrcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tsfrcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tsfrcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tsfrcr {
    #[inline(always)]
    fn default() -> Tsfrcr {
        <crate::RegValueT<Tsfrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlfrcr_SPEC;
impl crate::sealed::RegSpec for Tlfrcr_SPEC {
    type DataType = u32;
}

#[doc = "Too-Long Frame Receive Counter Register"]
pub type Tlfrcr = crate::RegValueT<Tlfrcr_SPEC>;

impl Tlfrcr {
    #[doc = "Too-Long Frame Receive Counter"]
    #[inline(always)]
    pub fn tlfrcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tlfrcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tlfrcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tlfrcr {
    #[inline(always)]
    fn default() -> Tlfrcr {
        <crate::RegValueT<Tlfrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcr_SPEC;
impl crate::sealed::RegSpec for Rfcr_SPEC {
    type DataType = u32;
}

#[doc = "Received Alignment Error Frame Counter Register"]
pub type Rfcr = crate::RegValueT<Rfcr_SPEC>;

impl Rfcr {
    #[doc = "Received Alignment Error Frame Counter"]
    #[inline(always)]
    pub fn rfcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfcr {
    #[inline(always)]
    fn default() -> Rfcr {
        <crate::RegValueT<Rfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mafcr_SPEC;
impl crate::sealed::RegSpec for Mafcr_SPEC {
    type DataType = u32;
}

#[doc = "Multicast Address Frame Receive Counter Register"]
pub type Mafcr = crate::RegValueT<Mafcr_SPEC>;

impl Mafcr {
    #[doc = "Multicast Address Frame Receive Counter"]
    #[inline(always)]
    pub fn mafcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mafcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mafcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mafcr {
    #[inline(always)]
    fn default() -> Mafcr {
        <crate::RegValueT<Mafcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
