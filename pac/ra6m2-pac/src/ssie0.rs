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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:16 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Sound Interface Enhanced (SSIE)"]
unsafe impl ::core::marker::Send for super::Ssie0 {}
unsafe impl ::core::marker::Sync for super::Ssie0 {}
impl super::Ssie0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ssicr(&self) -> &'static crate::common::Reg<self::Ssicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn ssisr(&self) -> &'static crate::common::Reg<self::Ssisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn ssifcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssifcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssifcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "FIFO Status Register"]
    #[inline(always)]
    pub const fn ssifsr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssifsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssifsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Transmit FIFO Data Register"]
    #[inline(always)]
    pub const fn ssiftdr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssiftdr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ssiftdr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Receive FIFO Data Register"]
    #[inline(always)]
    pub const fn ssifrdr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssifrdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssifrdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Audio Format Register"]
    #[inline(always)]
    pub const fn ssiofr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssiofr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssiofr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Status Control Register"]
    #[inline(always)]
    pub const fn ssiscr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssiscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssiscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssicr_SPEC;
impl crate::sealed::RegSpec for Ssicr_SPEC {
    type DataType = u32;
}

#[doc = "Control Register"]
pub type Ssicr = crate::RegValueT<Ssicr_SPEC>;

impl Ssicr {
    #[doc = "Oversampling Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ssicr::Cks,
        ssicr::Cks,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ssicr::Cks,
            ssicr::Cks,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn tuien(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ssicr::Tuien,
        ssicr::Tuien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ssicr::Tuien,
            ssicr::Tuien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toien(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ssicr::Toien,
        ssicr::Toien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ssicr::Toien,
            ssicr::Toien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn ruien(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ssicr::Ruien,
        ssicr::Ruien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ssicr::Ruien,
            ssicr::Ruien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn roien(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ssicr::Roien,
        ssicr::Roien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ssicr::Roien,
            ssicr::Roien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Idle Mode Interrupt Enable"]
    #[inline(always)]
    pub fn iien(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ssicr::Iien,
        ssicr::Iien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ssicr::Iien,
            ssicr::Iien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Channels"]
    #[inline(always)]
    pub fn chnl(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        ssicr::Chnl,
        ssicr::Chnl,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            ssicr::Chnl,
            ssicr::Chnl,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Data Word Length"]
    #[inline(always)]
    pub fn dwl(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        ssicr::Dwl,
        ssicr::Dwl,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            ssicr::Dwl,
            ssicr::Dwl,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "System Word LengthSet the system word length to the bit clock frequency/2 fs."]
    #[inline(always)]
    pub fn swl(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        ssicr::Swl,
        ssicr::Swl,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            ssicr::Swl,
            ssicr::Swl,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial WS Direction NOTE: Only the following settings are allowed: (SCKD, SWSD) = (0, 0) and (1, 1). Other settings are prohibited."]
    #[inline(always)]
    pub fn swsd(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ssicr::Swsd,
        ssicr::Swsd,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ssicr::Swsd,
            ssicr::Swsd,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Bit Clock Polarity"]
    #[inline(always)]
    pub fn sckp(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ssicr::Sckp,
        ssicr::Sckp,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ssicr::Sckp,
            ssicr::Sckp,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial WS Polarity"]
    #[inline(always)]
    pub fn swsp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ssicr::Swsp,
        ssicr::Swsp,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ssicr::Swsp,
            ssicr::Swsp,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Padding Polarity"]
    #[inline(always)]
    pub fn spdp(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ssicr::Spdp,
        ssicr::Spdp,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ssicr::Spdp,
            ssicr::Spdp,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Data Alignment"]
    #[inline(always)]
    pub fn sdta(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ssicr::Sdta,
        ssicr::Sdta,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ssicr::Sdta,
            ssicr::Sdta,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Parallel Data Alignment"]
    #[inline(always)]
    pub fn pdta(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ssicr::Pdta,
        ssicr::Pdta,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ssicr::Pdta,
            ssicr::Pdta,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Data Delay"]
    #[inline(always)]
    pub fn del(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ssicr::Del,
        ssicr::Del,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ssicr::Del,
            ssicr::Del,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Serial Oversampling Clock Division Ratio"]
    #[inline(always)]
    pub fn ckdv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        ssicr::Ckdv,
        ssicr::Ckdv,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            ssicr::Ckdv,
            ssicr::Ckdv,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mute EnableNOTE: When this module is muted, the value of outputting serial data is rewritten to 0 but data transmission is not stopped. Write dummy data to the SSIFTDR not to generate a transmit underflow because the number of data in the transmit FIFO is decreasing."]
    #[inline(always)]
    pub fn muen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssicr::Muen,
        ssicr::Muen,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssicr::Muen,
            ssicr::Muen,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn ten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssicr::Ten,
        ssicr::Ten,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssicr::Ten,
            ssicr::Ten,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn ren(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssicr::Ren,
        ssicr::Ren,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssicr::Ren,
            ssicr::Ren,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssicr {
    #[inline(always)]
    fn default() -> Ssicr {
        <crate::RegValueT<Ssicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "AUDIO_CLK input"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tuien_SPEC;
    pub type Tuien = crate::EnumBitfieldStruct<u8, Tuien_SPEC>;
    impl Tuien {
        #[doc = "Disables an underflow interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an underflow interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toien_SPEC;
    pub type Toien = crate::EnumBitfieldStruct<u8, Toien_SPEC>;
    impl Toien {
        #[doc = "Disables an overflow interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an overflow interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ruien_SPEC;
    pub type Ruien = crate::EnumBitfieldStruct<u8, Ruien_SPEC>;
    impl Ruien {
        #[doc = "Disables an underflow interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an underflow interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Roien_SPEC;
    pub type Roien = crate::EnumBitfieldStruct<u8, Roien_SPEC>;
    impl Roien {
        #[doc = "Disables an overflow interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an overflow interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iien_SPEC;
    pub type Iien = crate::EnumBitfieldStruct<u8, Iien_SPEC>;
    impl Iien {
        #[doc = "Disables an idle mode interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an idle mode interrupt."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chnl_SPEC;
    pub type Chnl = crate::EnumBitfieldStruct<u8, Chnl_SPEC>;
    impl Chnl {
        #[doc = "One channel"]
        pub const _00: Self = Self::new(0);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dwl_SPEC;
    pub type Dwl = crate::EnumBitfieldStruct<u8, Dwl_SPEC>;
    impl Dwl {
        #[doc = "8 bits"]
        pub const _000: Self = Self::new(0);

        #[doc = "16 bits"]
        pub const _001: Self = Self::new(1);

        #[doc = "18 bits"]
        pub const _010: Self = Self::new(2);

        #[doc = "20 bits"]
        pub const _011: Self = Self::new(3);

        #[doc = "22 bits"]
        pub const _100: Self = Self::new(4);

        #[doc = "24 bits"]
        pub const _101: Self = Self::new(5);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swl_SPEC;
    pub type Swl = crate::EnumBitfieldStruct<u8, Swl_SPEC>;
    impl Swl {
        #[doc = "8 bits (serial bit clock frequency = 16fs )"]
        pub const _000: Self = Self::new(0);

        #[doc = "16 bits (serial bit clock frequency = 32fs )"]
        pub const _001: Self = Self::new(1);

        #[doc = "24 bits (serial bit clock frequency = 48fs )"]
        pub const _010: Self = Self::new(2);

        #[doc = "32 bits (serial bit clock frequency = 64fs )"]
        pub const _011: Self = Self::new(3);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swsd_SPEC;
    pub type Swsd = crate::EnumBitfieldStruct<u8, Swsd_SPEC>;
    impl Swsd {
        #[doc = "Serial word select is input, slave mode."]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial word select is output, master mode."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckp_SPEC;
    pub type Sckp = crate::EnumBitfieldStruct<u8, Sckp_SPEC>;
    impl Sckp {
        #[doc = "SSIWS and SSIDATA change at the SSISCK falling edge (sampled at the SCK rising edge)."]
        pub const _0: Self = Self::new(0);

        #[doc = "SSIWS and SSIDATA change at the SSISCK rising edge (sampled at the SCK falling edge)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swsp_SPEC;
    pub type Swsp = crate::EnumBitfieldStruct<u8, Swsp_SPEC>;
    impl Swsp {
        #[doc = "SSIWS is low for 1st channel, high for 2nd channel."]
        pub const _0: Self = Self::new(0);

        #[doc = "SSIWS is high for 1st channel, low for 2nd channel."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spdp_SPEC;
    pub type Spdp = crate::EnumBitfieldStruct<u8, Spdp_SPEC>;
    impl Spdp {
        #[doc = "Padding bits are low."]
        pub const _0: Self = Self::new(0);

        #[doc = "Padding bits are high."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdta_SPEC;
    pub type Sdta = crate::EnumBitfieldStruct<u8, Sdta_SPEC>;
    impl Sdta {
        #[doc = "Transmitting and receiving in the order of serial data and padding bits"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmitting and receiving in the order of padding bits and serial data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdta_SPEC;
    pub type Pdta = crate::EnumBitfieldStruct<u8, Pdta_SPEC>;
    impl Pdta {
        #[doc = "The lower bits of parallel data (SSITDR, SSIRDR) are transferred prior to the upper bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is left-aligned.(When data word length is 18, 20, 22, or 24 bits)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The upper bits of parallel data (SSITDR, SSIRDR) are transferred prior to the lower bits.(When data word length is 8 or 16 bits) / Parallel data (SSITDR, SSIRDR) is right-aligned.(When data word length is 18, 20, 22, or 24 bits)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Del_SPEC;
    pub type Del = crate::EnumBitfieldStruct<u8, Del_SPEC>;
    impl Del {
        #[doc = "1 clock cycle delay between SSIWS and SSIDATA"]
        pub const _0: Self = Self::new(0);

        #[doc = "No delay between SSIWS and SSIDATA"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckdv_SPEC;
    pub type Ckdv = crate::EnumBitfieldStruct<u8, Ckdv_SPEC>;
    impl Ckdv {
        #[doc = "CLK"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "CLK/2"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "CLK/4"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "CLK/8"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "CLK/16"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "CLK/32"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "CLK/64"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "CLK/128"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "CLK/6"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "CLK/12 (These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "CLK/24"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "CLK/48(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "CLK/96(These bits are only settable for channel 0. Setting these bits in the register for channel 1 is prohibited.)"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Muen_SPEC;
    pub type Muen = crate::EnumBitfieldStruct<u8, Muen_SPEC>;
    impl Muen {
        #[doc = "This module is not muted."]
        pub const _0: Self = Self::new(0);

        #[doc = "This module is muted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ten_SPEC;
    pub type Ten = crate::EnumBitfieldStruct<u8, Ten_SPEC>;
    impl Ten {
        #[doc = "Disables the transmit operation."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables the transmit operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ren_SPEC;
    pub type Ren = crate::EnumBitfieldStruct<u8, Ren_SPEC>;
    impl Ren {
        #[doc = "Disables the receive operation."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables the receive operation."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssisr_SPEC;
impl crate::sealed::RegSpec for Ssisr_SPEC {
    type DataType = u32;
}

#[doc = "Status Register"]
pub type Ssisr = crate::RegValueT<Ssisr_SPEC>;

impl Ssisr {
    #[doc = "Transmit Underflow Error Interrupt Status Flag NOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn tuirq(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ssisr::Tuirq,
        ssisr::Tuirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ssisr::Tuirq,
            ssisr::Tuirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Overflow Error Interrupt Status Flag NOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn toirq(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ssisr::Toirq,
        ssisr::Toirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ssisr::Toirq,
            ssisr::Toirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Underflow Error Interrupt Status Flag NOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn ruirq(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ssisr::Ruirq,
        ssisr::Ruirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ssisr::Ruirq,
            ssisr::Ruirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Overflow Error Interrupt Status Flag NOTE: Writable only to clear the flag. Confirm the value is 1 and then write 0."]
    #[inline(always)]
    pub fn roirq(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ssisr::Roirq,
        ssisr::Roirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ssisr::Roirq,
            ssisr::Roirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Idle Mode Interrupt Status Flag"]
    #[inline(always)]
    pub fn iirq(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ssisr::Iirq,
        ssisr::Iirq,
        Ssisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ssisr::Iirq,
            ssisr::Iirq,
            Ssisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Channel Number"]
    #[inline(always)]
    pub fn tchno(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Ssisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Ssisr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit Serial Word Number"]
    #[inline(always)]
    pub fn tswno(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ssisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ssisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Receive Channel Number.These bits are read as 00b."]
    #[inline(always)]
    pub fn rchno(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Ssisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Ssisr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Serial Word Number"]
    #[inline(always)]
    pub fn rswno(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ssisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ssisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Idle Mode Status Flag"]
    #[inline(always)]
    pub fn idst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssisr::Idst,
        ssisr::Idst,
        Ssisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssisr::Idst,
            ssisr::Idst,
            Ssisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssisr {
    #[inline(always)]
    fn default() -> Ssisr {
        <crate::RegValueT<Ssisr_SPEC> as RegisterValue<_>>::new(33554451)
    }
}
pub mod ssisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tuirq_SPEC;
    pub type Tuirq = crate::EnumBitfieldStruct<u8, Tuirq_SPEC>;
    impl Tuirq {
        #[doc = "No transmit underflow has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A transmit underflow has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toirq_SPEC;
    pub type Toirq = crate::EnumBitfieldStruct<u8, Toirq_SPEC>;
    impl Toirq {
        #[doc = "No transmit overflow has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A transmit overflow has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ruirq_SPEC;
    pub type Ruirq = crate::EnumBitfieldStruct<u8, Ruirq_SPEC>;
    impl Ruirq {
        #[doc = "No receive underflow has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A receive underflow has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Roirq_SPEC;
    pub type Roirq = crate::EnumBitfieldStruct<u8, Roirq_SPEC>;
    impl Roirq {
        #[doc = "No receive overflow has occurred."]
        pub const _0: Self = Self::new(0);

        #[doc = "A receive overflow has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iirq_SPEC;
    pub type Iirq = crate::EnumBitfieldStruct<u8, Iirq_SPEC>;
    impl Iirq {
        #[doc = "This module is not in idle state."]
        pub const _0: Self = Self::new(0);

        #[doc = "This module is in idle state."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idst_SPEC;
    pub type Idst = crate::EnumBitfieldStruct<u8, Idst_SPEC>;
    impl Idst {
        #[doc = "Serial bus is operating."]
        pub const _0: Self = Self::new(0);

        #[doc = "The current communication is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifcr_SPEC;
impl crate::sealed::RegSpec for Ssifcr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Control Register"]
pub type Ssifcr = crate::RegValueT<Ssifcr_SPEC>;

impl Ssifcr {
    #[doc = "Oversampling Clock Enable"]
    #[inline(always)]
    pub fn aucke(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ssifcr::Aucke,
        ssifcr::Aucke,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ssifcr::Aucke,
            ssifcr::Aucke,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SSI soft ware reset"]
    #[inline(always)]
    pub fn ssirst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ssifcr::Ssirst,
        ssifcr::Ssirst,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ssifcr::Ssirst,
            ssifcr::Ssirst,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Data Trigger Number NOTE: The values in parenthesis are the number of empty stages in SSIFTDR at which the TDE flag is set."]
    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ssifcr::Ttrg,
        ssifcr::Ttrg,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ssifcr::Ttrg,
            ssifcr::Ttrg,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Trigger Number"]
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ssifcr::Rtrg,
        ssifcr::Rtrg,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ssifcr::Rtrg,
            ssifcr::Rtrg,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Interrupt Enable NOTE: TXI can be cleared by clearing either the TDE flag (see the description of the TDE bit for details) or TIE bit."]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssifcr::Tie,
        ssifcr::Tie,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssifcr::Tie,
            ssifcr::Tie,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Interrupt Enable NOTE: RXI can be cleared by clearing either the RDF flag (see the description of the RDF bit for details) or RIE bit."]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssifcr::Rie,
        ssifcr::Rie,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssifcr::Rie,
            ssifcr::Rie,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit FIFO Data Register Reset"]
    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssifcr::Tfrst,
        ssifcr::Tfrst,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssifcr::Tfrst,
            ssifcr::Tfrst,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive FIFO Data Register Reset"]
    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssifcr::Rfrst,
        ssifcr::Rfrst,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssifcr::Rfrst,
            ssifcr::Rfrst,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssifcr {
    #[inline(always)]
    fn default() -> Ssifcr {
        <crate::RegValueT<Ssifcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssifcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aucke_SPEC;
    pub type Aucke = crate::EnumBitfieldStruct<u8, Aucke_SPEC>;
    impl Aucke {
        #[doc = "The oversampling clock is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The oversampling clock is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssirst_SPEC;
    pub type Ssirst = crate::EnumBitfieldStruct<u8, Ssirst_SPEC>;
    impl Ssirst {
        #[doc = "Clears the SSI software reset."]
        pub const _0: Self = Self::new(0);

        #[doc = "initiates the SSI software reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttrg_SPEC;
    pub type Ttrg = crate::EnumBitfieldStruct<u8, Ttrg_SPEC>;
    impl Ttrg {
        #[doc = "7 (1)"]
        pub const _00: Self = Self::new(0);

        #[doc = "6 (2)"]
        pub const _01: Self = Self::new(1);

        #[doc = "4 (4)"]
        pub const _10: Self = Self::new(2);

        #[doc = "2 (6)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtrg_SPEC;
    pub type Rtrg = crate::EnumBitfieldStruct<u8, Rtrg_SPEC>;
    impl Rtrg {
        #[doc = "1"]
        pub const _00: Self = Self::new(0);

        #[doc = "2"]
        pub const _01: Self = Self::new(1);

        #[doc = "4"]
        pub const _10: Self = Self::new(2);

        #[doc = "6"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "Transmit data empty interrupt (TXI) request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit data empty interrupt (TXI) request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Receive data full interrupt (RXI) request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive data full interrupt (RXI) request is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        #[doc = "Clears the transmit data FIFO reset."]
        pub const _0: Self = Self::new(0);

        #[doc = "Initiates the transmit data FIFO reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        #[doc = "Clears the receive data FIFO reset."]
        pub const _0: Self = Self::new(0);

        #[doc = "Initiates the receive data FIFO reset."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifsr_SPEC;
impl crate::sealed::RegSpec for Ssifsr_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Status Register"]
pub type Ssifsr = crate::RegValueT<Ssifsr_SPEC>;

impl Ssifsr {
    #[doc = "Transmit Data Indicate Flag(Indicates the number of data units stored in SSIFTDR)"]
    #[inline(always)]
    pub fn tdc(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Ssifsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Ssifsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit Data Empty Flag NOTE: Since the SSIFTDR register is a 32-byte FIFO register, the maximum number of bytes that can be written to it while the TDE flag is 1 is 8 - TDC\\[3:0\\]. If writing data to the SSIFTDR register is continued after all the data is written, writing will be invalid and an overflow occurs."]
    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ssifsr::Tde,
        ssifsr::Tde,
        Ssifsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ssifsr::Tde,
            ssifsr::Tde,
            Ssifsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Indicate Flag(Indicates the number of data units stored in SSIFRDR)"]
    #[inline(always)]
    pub fn rdc(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Ssifsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Ssifsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Receive Data Full Flag NOTE:  Since the SSIFRDR register is a 32-byte FIFO register, the maximum number of data bytes that can be read from it while the RDF flag is 1 is indicated in the RDC\\[3:0\\] flags. If reading data from the SSIFRDR register is continued after all the data is read, undefined values will be read."]
    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssifsr::Rdf,
        ssifsr::Rdf,
        Ssifsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssifsr::Rdf,
            ssifsr::Rdf,
            Ssifsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssifsr {
    #[inline(always)]
    fn default() -> Ssifsr {
        <crate::RegValueT<Ssifsr_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod ssifsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        #[doc = "Number of data bytes for transmission in SSIFTDR is greater than the set transmit trigger number."]
        pub const _0: Self = Self::new(0);

        #[doc = "Number of data bytes for transmission in SSIFTDR is equal to or less than the set transmit trigger number."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "Number of received data bytes in SSIFRDR is less than the set receive trigger number."]
        pub const _0: Self = Self::new(0);

        #[doc = "Number of received data bytes in SSIFRDR is equal to or greater than the set receive trigger number."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiftdr_SPEC;
impl crate::sealed::RegSpec for Ssiftdr_SPEC {
    type DataType = u32;
}

#[doc = "Transmit FIFO Data Register"]
pub type Ssiftdr = crate::RegValueT<Ssiftdr_SPEC>;

impl Ssiftdr {
    #[doc = "SSIFTDR is a write-only FIFO register consisting of eight stages of 32-bit registers for storing data to be serially transmitted.  NOTE: that when the SSIFTDR register is full of data (32 bytes), the next data cannot be written to it. If writing is attempted, it will be ignored and an overflow occurs."]
    #[inline(always)]
    pub fn ssiftdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ssiftdr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ssiftdr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssiftdr {
    #[inline(always)]
    fn default() -> Ssiftdr {
        <crate::RegValueT<Ssiftdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifrdr_SPEC;
impl crate::sealed::RegSpec for Ssifrdr_SPEC {
    type DataType = u32;
}

#[doc = "Receive FIFO Data Register"]
pub type Ssifrdr = crate::RegValueT<Ssifrdr_SPEC>;

impl Ssifrdr {
    #[doc = "SSIFRDR is a read-only FIFO register consisting of eight stages of 32-bit registers for storing serially received data."]
    #[inline(always)]
    pub fn ssifrdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ssifrdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ssifrdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssifrdr {
    #[inline(always)]
    fn default() -> Ssifrdr {
        <crate::RegValueT<Ssifrdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiofr_SPEC;
impl crate::sealed::RegSpec for Ssiofr_SPEC {
    type DataType = u32;
}

#[doc = "Audio Format Register"]
pub type Ssiofr = crate::RegValueT<Ssiofr_SPEC>;

impl Ssiofr {
    #[doc = "Whether to Enable Stopping BCK Output When SSIE is in Idle Status"]
    #[inline(always)]
    pub fn bckastp(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ssiofr::Bckastp,
        ssiofr::Bckastp,
        Ssiofr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ssiofr::Bckastp,
            ssiofr::Bckastp,
            Ssiofr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Whether to Enable LRCK/FS Continuation"]
    #[inline(always)]
    pub fn lrcont(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ssiofr::Lrcont,
        ssiofr::Lrcont,
        Ssiofr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ssiofr::Lrcont,
            ssiofr::Lrcont,
            Ssiofr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Audio Format Select"]
    #[inline(always)]
    pub fn omod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ssiofr::Omod,
        ssiofr::Omod,
        Ssiofr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ssiofr::Omod,
            ssiofr::Omod,
            Ssiofr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssiofr {
    #[inline(always)]
    fn default() -> Ssiofr {
        <crate::RegValueT<Ssiofr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssiofr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bckastp_SPEC;
    pub type Bckastp = crate::EnumBitfieldStruct<u8, Bckastp_SPEC>;
    impl Bckastp {
        #[doc = "Always outputs BCK to the SSIBCK pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "Automatically controls output of BCK to the SSIBCK pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrcont_SPEC;
    pub type Lrcont = crate::EnumBitfieldStruct<u8, Lrcont_SPEC>;
    impl Lrcont {
        #[doc = "Disables LRCK/FS continuation."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables LRCK/FS continuation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Omod_SPEC;
    pub type Omod = crate::EnumBitfieldStruct<u8, Omod_SPEC>;
    impl Omod {
        #[doc = "I2S format"]
        pub const _00: Self = Self::new(0);

        #[doc = "TDM format"]
        pub const _01: Self = Self::new(1);

        #[doc = "Monaural format"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiscr_SPEC;
impl crate::sealed::RegSpec for Ssiscr_SPEC {
    type DataType = u32;
}

#[doc = "Status Control Register"]
pub type Ssiscr = crate::RegValueT<Ssiscr_SPEC>;

impl Ssiscr {
    #[doc = "TDE Setting Condition Select"]
    #[inline(always)]
    pub fn tdes(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1f,
        1,
        0,
        ssiscr::Tdes,
        ssiscr::Tdes,
        Ssiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1f,
            1,
            0,
            ssiscr::Tdes,
            ssiscr::Tdes,
            Ssiscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RDF Setting Condition Select"]
    #[inline(always)]
    pub fn rdfs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        ssiscr::Rdfs,
        ssiscr::Rdfs,
        Ssiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            ssiscr::Rdfs,
            ssiscr::Rdfs,
            Ssiscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssiscr {
    #[inline(always)]
    fn default() -> Ssiscr {
        <crate::RegValueT<Ssiscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssiscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdes_SPEC;
    pub type Tdes = crate::EnumBitfieldStruct<u8, Tdes_SPEC>;
    impl Tdes {
        #[doc = "SSIFTDR has one stage or more free space"]
        pub const _00000: Self = Self::new(0);

        #[doc = "SSIFTDR has two stages or more free space (snip)"]
        pub const _00001: Self = Self::new(1);

        #[doc = "SSIFTDR has thirty-one stages or more free space"]
        pub const _11110: Self = Self::new(30);

        #[doc = "SSIFTDR has thirty-two stages or more free space."]
        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdfs_SPEC;
    pub type Rdfs = crate::EnumBitfieldStruct<u8, Rdfs_SPEC>;
    impl Rdfs {
        #[doc = "SSIFRDR has one stage or more data size"]
        pub const _00000: Self = Self::new(0);

        #[doc = "SSIFRDR has two stages or more data size (snip)"]
        pub const _00001: Self = Self::new(1);

        #[doc = "SSIFRDR has thirty-one stages or more data size"]
        pub const _11110: Self = Self::new(30);

        #[doc = "SSIFRDR has thirty-two stages or more data size."]
        pub const _11111: Self = Self::new(31);
    }
}
