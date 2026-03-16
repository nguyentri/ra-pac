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
// Generated from SVD 1.10.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:13 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Array Unit 0"]
unsafe impl ::core::marker::Send for super::Sau0 {}
unsafe impl ::core::marker::Sync for super::Sau0 {}
impl super::Sau0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Serial Data Register 0%s"]
    #[inline(always)]
    pub const fn sdr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdr0_SPEC, crate::common::RW>,
        4,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn sdr00(&self) -> &'static crate::common::Reg<self::Sdr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdr01(&self) -> &'static crate::common::Reg<self::Sdr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdr02(&self) -> &'static crate::common::Reg<self::Sdr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn sdr03(&self) -> &'static crate::common::Reg<self::Sdr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x6usize),
            )
        }
    }

    #[doc = "Serial Status Register 00"]
    #[inline(always)]
    pub const fn ssr00(&self) -> &'static crate::common::Reg<self::Ssr00_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssr00_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Serial Status Register 01"]
    #[inline(always)]
    pub const fn ssr01(&self) -> &'static crate::common::Reg<self::Ssr01_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssr01_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }

    #[doc = "Serial Status Register 02"]
    #[inline(always)]
    pub const fn ssr02(&self) -> &'static crate::common::Reg<self::Ssr02_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssr02_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Serial Status Register 03"]
    #[inline(always)]
    pub const fn ssr03(&self) -> &'static crate::common::Reg<self::Ssr03_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssr03_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(262usize),
            )
        }
    }

    #[doc = "Serial Flag Clear Trigger Register 00"]
    #[inline(always)]
    pub const fn sir00(&self) -> &'static crate::common::Reg<self::Sir00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sir00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Serial Flag Clear Trigger Register 01"]
    #[inline(always)]
    pub const fn sir01(&self) -> &'static crate::common::Reg<self::Sir01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sir01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(266usize),
            )
        }
    }

    #[doc = "Serial Flag Clear Trigger Register 02"]
    #[inline(always)]
    pub const fn sir02(&self) -> &'static crate::common::Reg<self::Sir02_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sir02_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "Serial Flag Clear Trigger Register 03"]
    #[inline(always)]
    pub const fn sir03(&self) -> &'static crate::common::Reg<self::Sir03_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sir03_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(270usize),
            )
        }
    }

    #[doc = "Serial Mode Register 00"]
    #[inline(always)]
    pub const fn smr00(&self) -> &'static crate::common::Reg<self::Smr00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Serial Mode Register 01"]
    #[inline(always)]
    pub const fn smr01(&self) -> &'static crate::common::Reg<self::Smr01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(274usize),
            )
        }
    }

    #[doc = "Serial Mode Register 02"]
    #[inline(always)]
    pub const fn smr02(&self) -> &'static crate::common::Reg<self::Smr02_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr02_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "Serial Mode Register 03"]
    #[inline(always)]
    pub const fn smr03(&self) -> &'static crate::common::Reg<self::Smr03_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr03_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(278usize),
            )
        }
    }

    #[doc = "Serial Communication Operation Setting Register 00"]
    #[inline(always)]
    pub const fn scr00(&self) -> &'static crate::common::Reg<self::Scr00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[doc = "Serial Communication Operation Setting Register 01"]
    #[inline(always)]
    pub const fn scr01(&self) -> &'static crate::common::Reg<self::Scr01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(282usize),
            )
        }
    }

    #[doc = "Serial Communication Operation Setting Register 02"]
    #[inline(always)]
    pub const fn scr02(&self) -> &'static crate::common::Reg<self::Scr02_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr02_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[doc = "Serial Communication Operation Setting Register 03"]
    #[inline(always)]
    pub const fn scr03(&self) -> &'static crate::common::Reg<self::Scr03_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr03_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(286usize),
            )
        }
    }

    #[doc = "Serial Channel Enable Status Register 0"]
    #[inline(always)]
    pub const fn se0(&self) -> &'static crate::common::Reg<self::Se0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Se0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Serial Channel Start Register 0"]
    #[inline(always)]
    pub const fn ss0(&self) -> &'static crate::common::Reg<self::Ss0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ss0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(290usize),
            )
        }
    }

    #[doc = "Serial Channel Stop Register 0"]
    #[inline(always)]
    pub const fn st0(&self) -> &'static crate::common::Reg<self::St0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::St0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "Serial Clock Select Register 0"]
    #[inline(always)]
    pub const fn sps0(&self) -> &'static crate::common::Reg<self::Sps0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sps0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(294usize),
            )
        }
    }

    #[doc = "Serial Output Register 0"]
    #[inline(always)]
    pub const fn so0(&self) -> &'static crate::common::Reg<self::So0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::So0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[doc = "Serial Output Enable Register 0"]
    #[inline(always)]
    pub const fn soe0(&self) -> &'static crate::common::Reg<self::Soe0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Soe0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(298usize),
            )
        }
    }

    #[doc = "Serial Output Level Register 0"]
    #[inline(always)]
    pub const fn sol0(&self) -> &'static crate::common::Reg<self::Sol0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sol0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "Serial Standby Control Register 0"]
    #[inline(always)]
    pub const fn ssc0(&self) -> &'static crate::common::Reg<self::Ssc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdr0_SPEC;
impl crate::sealed::RegSpec for Sdr0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Data Register 0%s"]
pub type Sdr0 = crate::RegValueT<Sdr0_SPEC>;

impl Sdr0 {
    #[doc = "Data Buffer for Transmit and Receive"]
    #[inline(always)]
    pub fn dat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Sdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Sdr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Clock Setting by Dividing the Operation Clock"]
    #[inline(always)]
    pub fn stclk(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Sdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Sdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdr0 {
    #[inline(always)]
    fn default() -> Sdr0 {
        <crate::RegValueT<Sdr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr00_SPEC;
impl crate::sealed::RegSpec for Ssr00_SPEC {
    type DataType = u16;
}

#[doc = "Serial Status Register 00"]
pub type Ssr00 = crate::RegValueT<Ssr00_SPEC>;

impl Ssr00 {
    #[doc = "Overrun Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr00::Ovf,
        ssr00::Ovf,
        Ssr00_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr00::Ovf,
            ssr00::Ovf,
            Ssr00_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity or ACK Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn pef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr00::Pef,
        ssr00::Pef,
        Ssr00_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr00::Pef,
            ssr00::Pef,
            Ssr00_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of the Buffer Register for Channel n"]
    #[inline(always)]
    pub fn bff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr00::Bff,
        ssr00::Bff,
        Ssr00_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr00::Bff,
            ssr00::Bff,
            Ssr00_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of Communications for Channel n"]
    #[inline(always)]
    pub fn tsf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr00::Tsf,
        ssr00::Tsf,
        Ssr00_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr00::Tsf,
            ssr00::Tsf,
            Ssr00_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr00 {
    #[inline(always)]
    fn default() -> Ssr00 {
        <crate::RegValueT<Ssr00_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssr00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pef_SPEC;
    pub type Pef = crate::EnumBitfieldStruct<u8, Pef_SPEC>;
    impl Pef {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurs (during UART reception) or ACK is not detected (during I2C transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bff_SPEC;
    pub type Bff = crate::EnumBitfieldStruct<u8, Bff_SPEC>;
    impl Bff {
        #[doc = "Valid data is not stored in the SDRmn register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid data is stored in the SDRmn register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsf_SPEC;
    pub type Tsf = crate::EnumBitfieldStruct<u8, Tsf_SPEC>;
    impl Tsf {
        #[doc = "Communication is stopped or suspended"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication is in progress"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr01_SPEC;
impl crate::sealed::RegSpec for Ssr01_SPEC {
    type DataType = u16;
}

#[doc = "Serial Status Register 01"]
pub type Ssr01 = crate::RegValueT<Ssr01_SPEC>;

impl Ssr01 {
    #[doc = "Overrun Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr01::Ovf,
        ssr01::Ovf,
        Ssr01_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr01::Ovf,
            ssr01::Ovf,
            Ssr01_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity or ACK Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn pef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr01::Pef,
        ssr01::Pef,
        Ssr01_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr01::Pef,
            ssr01::Pef,
            Ssr01_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framing Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn fef(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr01::Fef,
        ssr01::Fef,
        Ssr01_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr01::Fef,
            ssr01::Fef,
            Ssr01_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of the Buffer Register for Channel n"]
    #[inline(always)]
    pub fn bff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr01::Bff,
        ssr01::Bff,
        Ssr01_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr01::Bff,
            ssr01::Bff,
            Ssr01_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of Communications for Channel n"]
    #[inline(always)]
    pub fn tsf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr01::Tsf,
        ssr01::Tsf,
        Ssr01_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr01::Tsf,
            ssr01::Tsf,
            Ssr01_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr01 {
    #[inline(always)]
    fn default() -> Ssr01 {
        <crate::RegValueT<Ssr01_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssr01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pef_SPEC;
    pub type Pef = crate::EnumBitfieldStruct<u8, Pef_SPEC>;
    impl Pef {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurs (during UART reception) or ACK is not detected (during I2C transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fef_SPEC;
    pub type Fef = crate::EnumBitfieldStruct<u8, Fef_SPEC>;
    impl Fef {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs (during UART reception)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bff_SPEC;
    pub type Bff = crate::EnumBitfieldStruct<u8, Bff_SPEC>;
    impl Bff {
        #[doc = "Valid data is not stored in the SDRmn register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid data is stored in the SDRmn register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsf_SPEC;
    pub type Tsf = crate::EnumBitfieldStruct<u8, Tsf_SPEC>;
    impl Tsf {
        #[doc = "Communication is stopped or suspended"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication is in progress"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr02_SPEC;
impl crate::sealed::RegSpec for Ssr02_SPEC {
    type DataType = u16;
}

#[doc = "Serial Status Register 02"]
pub type Ssr02 = crate::RegValueT<Ssr02_SPEC>;

impl Ssr02 {
    #[doc = "Overrun Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr02::Ovf,
        ssr02::Ovf,
        Ssr02_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr02::Ovf,
            ssr02::Ovf,
            Ssr02_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity or ACK Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn pef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr02::Pef,
        ssr02::Pef,
        Ssr02_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr02::Pef,
            ssr02::Pef,
            Ssr02_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of the Buffer Register for Channel n"]
    #[inline(always)]
    pub fn bff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr02::Bff,
        ssr02::Bff,
        Ssr02_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr02::Bff,
            ssr02::Bff,
            Ssr02_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of Communications for Channel n"]
    #[inline(always)]
    pub fn tsf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr02::Tsf,
        ssr02::Tsf,
        Ssr02_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr02::Tsf,
            ssr02::Tsf,
            Ssr02_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr02 {
    #[inline(always)]
    fn default() -> Ssr02 {
        <crate::RegValueT<Ssr02_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssr02 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pef_SPEC;
    pub type Pef = crate::EnumBitfieldStruct<u8, Pef_SPEC>;
    impl Pef {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurs (during UART reception) or ACK is not detected (during I2C transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bff_SPEC;
    pub type Bff = crate::EnumBitfieldStruct<u8, Bff_SPEC>;
    impl Bff {
        #[doc = "Valid data is not stored in the SDRmn register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid data is stored in the SDRmn register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsf_SPEC;
    pub type Tsf = crate::EnumBitfieldStruct<u8, Tsf_SPEC>;
    impl Tsf {
        #[doc = "Communication is stopped or suspended"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication is in progress"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr03_SPEC;
impl crate::sealed::RegSpec for Ssr03_SPEC {
    type DataType = u16;
}

#[doc = "Serial Status Register 03"]
pub type Ssr03 = crate::RegValueT<Ssr03_SPEC>;

impl Ssr03 {
    #[doc = "Overrun Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr03::Ovf,
        ssr03::Ovf,
        Ssr03_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr03::Ovf,
            ssr03::Ovf,
            Ssr03_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Parity or ACK Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn pef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr03::Pef,
        ssr03::Pef,
        Ssr03_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr03::Pef,
            ssr03::Pef,
            Ssr03_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Framing Error Detection Flag of Channel n"]
    #[inline(always)]
    pub fn fef(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr03::Fef,
        ssr03::Fef,
        Ssr03_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr03::Fef,
            ssr03::Fef,
            Ssr03_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of the Buffer Register for Channel n"]
    #[inline(always)]
    pub fn bff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr03::Bff,
        ssr03::Bff,
        Ssr03_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr03::Bff,
            ssr03::Bff,
            Ssr03_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag Indicating the State of Communications for Channel n"]
    #[inline(always)]
    pub fn tsf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr03::Tsf,
        ssr03::Tsf,
        Ssr03_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr03::Tsf,
            ssr03::Tsf,
            Ssr03_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr03 {
    #[inline(always)]
    fn default() -> Ssr03 {
        <crate::RegValueT<Ssr03_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssr03 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pef_SPEC;
    pub type Pef = crate::EnumBitfieldStruct<u8, Pef_SPEC>;
    impl Pef {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "Parity error occurs (during UART reception) or ACK is not detected (during I2C transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fef_SPEC;
    pub type Fef = crate::EnumBitfieldStruct<u8, Fef_SPEC>;
    impl Fef {
        #[doc = "No error occurs"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurs (during UART reception)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bff_SPEC;
    pub type Bff = crate::EnumBitfieldStruct<u8, Bff_SPEC>;
    impl Bff {
        #[doc = "Valid data is not stored in the SDRmn register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid data is stored in the SDRmn register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsf_SPEC;
    pub type Tsf = crate::EnumBitfieldStruct<u8, Tsf_SPEC>;
    impl Tsf {
        #[doc = "Communication is stopped or suspended"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication is in progress"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir00_SPEC;
impl crate::sealed::RegSpec for Sir00_SPEC {
    type DataType = u16;
}

#[doc = "Serial Flag Clear Trigger Register 00"]
pub type Sir00 = crate::RegValueT<Sir00_SPEC>;

impl Sir00 {
    #[doc = "Clear Trigger of Overrun Error Flag of Channel n"]
    #[inline(always)]
    pub fn ovct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sir00::Ovct,
        sir00::Ovct,
        Sir00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sir00::Ovct,
            sir00::Ovct,
            Sir00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clear Trigger of Parity Error Flag of Channel n"]
    #[inline(always)]
    pub fn pect(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sir00::Pect,
        sir00::Pect,
        Sir00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sir00::Pect,
            sir00::Pect,
            Sir00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sir00 {
    #[inline(always)]
    fn default() -> Sir00 {
        <crate::RegValueT<Sir00_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sir00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovct_SPEC;
    pub type Ovct = crate::EnumBitfieldStruct<u8, Ovct_SPEC>;
    impl Ovct {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the OVF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pect_SPEC;
    pub type Pect = crate::EnumBitfieldStruct<u8, Pect_SPEC>;
    impl Pect {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the PEF bit of the SSRmn register to 0."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir01_SPEC;
impl crate::sealed::RegSpec for Sir01_SPEC {
    type DataType = u16;
}

#[doc = "Serial Flag Clear Trigger Register 01"]
pub type Sir01 = crate::RegValueT<Sir01_SPEC>;

impl Sir01 {
    #[doc = "Clear Trigger of Overrun Error Flag of Channel n"]
    #[inline(always)]
    pub fn ovct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sir01::Ovct,
        sir01::Ovct,
        Sir01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sir01::Ovct,
            sir01::Ovct,
            Sir01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clear Trigger of Parity Error Flag of Channel n"]
    #[inline(always)]
    pub fn pect(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sir01::Pect,
        sir01::Pect,
        Sir01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sir01::Pect,
            sir01::Pect,
            Sir01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clear Trigger of Framing Error Flag of Channel n"]
    #[inline(always)]
    pub fn fect(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sir01::Fect,
        sir01::Fect,
        Sir01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sir01::Fect,
            sir01::Fect,
            Sir01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sir01 {
    #[inline(always)]
    fn default() -> Sir01 {
        <crate::RegValueT<Sir01_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sir01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovct_SPEC;
    pub type Ovct = crate::EnumBitfieldStruct<u8, Ovct_SPEC>;
    impl Ovct {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the OVF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pect_SPEC;
    pub type Pect = crate::EnumBitfieldStruct<u8, Pect_SPEC>;
    impl Pect {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the PEF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fect_SPEC;
    pub type Fect = crate::EnumBitfieldStruct<u8, Fect_SPEC>;
    impl Fect {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the FEF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir02_SPEC;
impl crate::sealed::RegSpec for Sir02_SPEC {
    type DataType = u16;
}

#[doc = "Serial Flag Clear Trigger Register 02"]
pub type Sir02 = crate::RegValueT<Sir02_SPEC>;

impl Sir02 {
    #[doc = "Clear Trigger of Overrun Error Flag of Channel n"]
    #[inline(always)]
    pub fn ovct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sir02::Ovct,
        sir02::Ovct,
        Sir02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sir02::Ovct,
            sir02::Ovct,
            Sir02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clear Trigger of Parity Error Flag of Channel n"]
    #[inline(always)]
    pub fn pect(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sir02::Pect,
        sir02::Pect,
        Sir02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sir02::Pect,
            sir02::Pect,
            Sir02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sir02 {
    #[inline(always)]
    fn default() -> Sir02 {
        <crate::RegValueT<Sir02_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sir02 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovct_SPEC;
    pub type Ovct = crate::EnumBitfieldStruct<u8, Ovct_SPEC>;
    impl Ovct {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the OVF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pect_SPEC;
    pub type Pect = crate::EnumBitfieldStruct<u8, Pect_SPEC>;
    impl Pect {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the PEF bit of the SSRmn register to 0."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir03_SPEC;
impl crate::sealed::RegSpec for Sir03_SPEC {
    type DataType = u16;
}

#[doc = "Serial Flag Clear Trigger Register 03"]
pub type Sir03 = crate::RegValueT<Sir03_SPEC>;

impl Sir03 {
    #[doc = "Clear Trigger of Overrun Error Flag of Channel n"]
    #[inline(always)]
    pub fn ovct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sir03::Ovct,
        sir03::Ovct,
        Sir03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sir03::Ovct,
            sir03::Ovct,
            Sir03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clear Trigger of Parity Error Flag of Channel n"]
    #[inline(always)]
    pub fn pect(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sir03::Pect,
        sir03::Pect,
        Sir03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sir03::Pect,
            sir03::Pect,
            Sir03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Clear Trigger of Framing Error Flag of Channel n"]
    #[inline(always)]
    pub fn fect(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sir03::Fect,
        sir03::Fect,
        Sir03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sir03::Fect,
            sir03::Fect,
            Sir03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sir03 {
    #[inline(always)]
    fn default() -> Sir03 {
        <crate::RegValueT<Sir03_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sir03 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovct_SPEC;
    pub type Ovct = crate::EnumBitfieldStruct<u8, Ovct_SPEC>;
    impl Ovct {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the OVF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pect_SPEC;
    pub type Pect = crate::EnumBitfieldStruct<u8, Pect_SPEC>;
    impl Pect {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the PEF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fect_SPEC;
    pub type Fect = crate::EnumBitfieldStruct<u8, Fect_SPEC>;
    impl Fect {
        #[doc = "Not cleared"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the FEF bit of the SSRmn register to 0"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr00_SPEC;
impl crate::sealed::RegSpec for Smr00_SPEC {
    type DataType = u16;
}

#[doc = "Serial Mode Register 00"]
pub type Smr00 = crate::RegValueT<Smr00_SPEC>;

impl Smr00 {
    #[doc = "Selection of Channel n Interrupt Source"]
    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smr00::Md0,
        smr00::Md0,
        Smr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smr00::Md0,
            smr00::Md0,
            Smr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Channel n Operation Mode"]
    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        smr00::Md1,
        smr00::Md1,
        Smr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            smr00::Md1,
            smr00::Md1,
            Smr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Transfer Clock (fTCLK) of Channel n"]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        smr00::Ccs,
        smr00::Ccs,
        Smr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            smr00::Ccs,
            smr00::Ccs,
            Smr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Operation Clock (fMCK) of Channel n"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        smr00::Cks,
        smr00::Cks,
        Smr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            smr00::Cks,
            smr00::Cks,
            Smr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smr00 {
    #[inline(always)]
    fn default() -> Smr00 {
        <crate::RegValueT<Smr00_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod smr00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md0_SPEC;
    pub type Md0 = crate::EnumBitfieldStruct<u8, Md0_SPEC>;
    impl Md0 {
        #[doc = "Transfer end interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer empty interrupt (Occurs when data is transferred from the SDRmn register to the shift register.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md1_SPEC;
    pub type Md1 = crate::EnumBitfieldStruct<u8, Md1_SPEC>;
    impl Md1 {
        #[doc = "Simplified SPI mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "UART mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Simplified I2C mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        #[doc = "Divided operation clock fMCK specified by the CKS bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock input fSCK from the SCKp pin (slave transfer in simplified SPI mode)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "Operation clock CKm0 set by the SPSm register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation clock CKm1 set by the SPSm register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr01_SPEC;
impl crate::sealed::RegSpec for Smr01_SPEC {
    type DataType = u16;
}

#[doc = "Serial Mode Register 01"]
pub type Smr01 = crate::RegValueT<Smr01_SPEC>;

impl Smr01 {
    #[doc = "Selection of Channel n Interrupt Source"]
    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smr01::Md0,
        smr01::Md0,
        Smr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smr01::Md0,
            smr01::Md0,
            Smr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Channel n Operation Mode"]
    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        smr01::Md1,
        smr01::Md1,
        Smr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            smr01::Md1,
            smr01::Md1,
            Smr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Controls Inversion of Level of Channel n Receive Data in UART Mode"]
    #[inline(always)]
    pub fn sis0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        smr01::Sis0,
        smr01::Sis0,
        Smr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            smr01::Sis0,
            smr01::Sis0,
            Smr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Start Trigger Source"]
    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        smr01::Sts,
        smr01::Sts,
        Smr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            smr01::Sts,
            smr01::Sts,
            Smr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Transfer Clock (fTCLK) of Channel n"]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        smr01::Ccs,
        smr01::Ccs,
        Smr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            smr01::Ccs,
            smr01::Ccs,
            Smr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Operation Clock (fMCK) of Channel n"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        smr01::Cks,
        smr01::Cks,
        Smr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            smr01::Cks,
            smr01::Cks,
            Smr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smr01 {
    #[inline(always)]
    fn default() -> Smr01 {
        <crate::RegValueT<Smr01_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod smr01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md0_SPEC;
    pub type Md0 = crate::EnumBitfieldStruct<u8, Md0_SPEC>;
    impl Md0 {
        #[doc = "Transfer end interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer empty interrupt (Occurs when data is transferred from the SDRmn register to the shift register.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md1_SPEC;
    pub type Md1 = crate::EnumBitfieldStruct<u8, Md1_SPEC>;
    impl Md1 {
        #[doc = "Simplified SPI mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "UART mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Simplified I2C mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sis0_SPEC;
    pub type Sis0 = crate::EnumBitfieldStruct<u8, Sis0_SPEC>;
    impl Sis0 {
        #[doc = "Falling edge is detected as the start bit. The input communication data is captured as is."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge is detected as the start bit. The input communication data is inverted and captured."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        #[doc = "Only software trigger is valid (selected for simplified SPI, UART transmission, and simplified I2C)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid edge of the RXDq pin (selected for UART reception)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        #[doc = "Divided operation clock fMCK specified by the CKS bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock input fSCK from the SCKp pin (slave transfer in simplified SPI mode)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "Operation clock CKm0 set by the SPSm register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation clock CKm1 set by the SPSm register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr02_SPEC;
impl crate::sealed::RegSpec for Smr02_SPEC {
    type DataType = u16;
}

#[doc = "Serial Mode Register 02"]
pub type Smr02 = crate::RegValueT<Smr02_SPEC>;

impl Smr02 {
    #[doc = "Selection of Channel n Interrupt Source"]
    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smr02::Md0,
        smr02::Md0,
        Smr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smr02::Md0,
            smr02::Md0,
            Smr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Channel n Operation Mode"]
    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        smr02::Md1,
        smr02::Md1,
        Smr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            smr02::Md1,
            smr02::Md1,
            Smr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Transfer Clock (fTCLK) of Channel n"]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        smr02::Ccs,
        smr02::Ccs,
        Smr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            smr02::Ccs,
            smr02::Ccs,
            Smr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Operation Clock (fMCK) of Channel n"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        smr02::Cks,
        smr02::Cks,
        Smr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            smr02::Cks,
            smr02::Cks,
            Smr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smr02 {
    #[inline(always)]
    fn default() -> Smr02 {
        <crate::RegValueT<Smr02_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod smr02 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md0_SPEC;
    pub type Md0 = crate::EnumBitfieldStruct<u8, Md0_SPEC>;
    impl Md0 {
        #[doc = "Transfer end interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer empty interrupt (Occurs when data is transferred from the SDRmn register to the shift register.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md1_SPEC;
    pub type Md1 = crate::EnumBitfieldStruct<u8, Md1_SPEC>;
    impl Md1 {
        #[doc = "Simplified SPI mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "UART mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Simplified I2C mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        #[doc = "Divided operation clock fMCK specified by the CKS bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock input fSCK from the SCKp pin (slave transfer in simplified SPI mode)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "Operation clock CKm0 set by the SPSm register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation clock CKm1 set by the SPSm register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr03_SPEC;
impl crate::sealed::RegSpec for Smr03_SPEC {
    type DataType = u16;
}

#[doc = "Serial Mode Register 03"]
pub type Smr03 = crate::RegValueT<Smr03_SPEC>;

impl Smr03 {
    #[doc = "Selection of Channel n Interrupt Source"]
    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smr03::Md0,
        smr03::Md0,
        Smr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smr03::Md0,
            smr03::Md0,
            Smr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Channel n Operation Mode"]
    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        smr03::Md1,
        smr03::Md1,
        Smr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            smr03::Md1,
            smr03::Md1,
            Smr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Controls Inversion of Level of Channel n Receive Data in UART Mode"]
    #[inline(always)]
    pub fn sis0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        smr03::Sis0,
        smr03::Sis0,
        Smr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            smr03::Sis0,
            smr03::Sis0,
            Smr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Start Trigger Source"]
    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        smr03::Sts,
        smr03::Sts,
        Smr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            smr03::Sts,
            smr03::Sts,
            Smr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Transfer Clock (fTCLK) of Channel n"]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        smr03::Ccs,
        smr03::Ccs,
        Smr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            smr03::Ccs,
            smr03::Ccs,
            Smr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Operation Clock (fMCK) of Channel n"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        smr03::Cks,
        smr03::Cks,
        Smr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            smr03::Cks,
            smr03::Cks,
            Smr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smr03 {
    #[inline(always)]
    fn default() -> Smr03 {
        <crate::RegValueT<Smr03_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod smr03 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md0_SPEC;
    pub type Md0 = crate::EnumBitfieldStruct<u8, Md0_SPEC>;
    impl Md0 {
        #[doc = "Transfer end interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer empty interrupt (Occurs when data is transferred from the SDRmn register to the shift register.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md1_SPEC;
    pub type Md1 = crate::EnumBitfieldStruct<u8, Md1_SPEC>;
    impl Md1 {
        #[doc = "Simplified SPI mode"]
        pub const _00: Self = Self::new(0);

        #[doc = "UART mode"]
        pub const _01: Self = Self::new(1);

        #[doc = "Simplified I2C mode"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sis0_SPEC;
    pub type Sis0 = crate::EnumBitfieldStruct<u8, Sis0_SPEC>;
    impl Sis0 {
        #[doc = "Falling edge is detected as the start bit. The input communication data is captured as is."]
        pub const _0: Self = Self::new(0);

        #[doc = "Rising edge is detected as the start bit. The input communication data is inverted and captured."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        #[doc = "Only software trigger is valid (selected for simplified SPI, UART transmission, and simplified I2C)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Valid edge of the RXDq pin (selected for UART reception)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        #[doc = "Divided operation clock fMCK specified by the CKS bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock input fSCK from the SCKp pin (slave transfer in simplified SPI mode)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "Operation clock CKm0 set by the SPSm register"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation clock CKm1 set by the SPSm register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr00_SPEC;
impl crate::sealed::RegSpec for Scr00_SPEC {
    type DataType = u16;
}

#[doc = "Serial Communication Operation Setting Register 00"]
pub type Scr00 = crate::RegValueT<Scr00_SPEC>;

impl Scr00 {
    #[doc = "Setting of Data Length in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dls(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        scr00::Dls,
        scr00::Dls,
        Scr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            scr00::Dls,
            scr00::Dls,
            Scr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Stop Bit in UART Mode"]
    #[inline(always)]
    pub fn slc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        scr00::Slc,
        scr00::Slc,
        Scr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            scr00::Slc,
            scr00::Slc,
            Scr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data Transfer Sequence in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr00::Dir,
        scr00::Dir,
        Scr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr00::Dir,
            scr00::Dir,
            Scr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Parity Bit in UART Mode"]
    #[inline(always)]
    pub fn ptc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        scr00::Ptc,
        scr00::Ptc,
        Scr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            scr00::Ptc,
            scr00::Ptc,
            Scr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data and Clock Phase in Simplified SPI Mode"]
    #[inline(always)]
    pub fn dcp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        scr00::Dcp,
        scr00::Dcp,
        Scr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            scr00::Dcp,
            scr00::Dcp,
            Scr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Channel 0 Operation Mode"]
    #[inline(always)]
    pub fn trxe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        scr00::Trxe,
        scr00::Trxe,
        Scr00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            scr00::Trxe,
            scr00::Trxe,
            Scr00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scr00 {
    #[inline(always)]
    fn default() -> Scr00 {
        <crate::RegValueT<Scr00_SPEC> as RegisterValue<_>>::new(135)
    }
}
pub mod scr00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dls_SPEC;
    pub type Dls = crate::EnumBitfieldStruct<u8, Dls_SPEC>;
    impl Dls {
        #[doc = "Setting prohibited"]
        pub const _00: Self = Self::new(0);

        #[doc = "9-bit data length (stored in bits 0 to 8 of the SDRm0 register) (settable in UART mode only)"]
        pub const _01: Self = Self::new(1);

        #[doc = "7-bit data length (stored in bits 0 to 6 of the SDRm0 register)"]
        pub const _10: Self = Self::new(2);

        #[doc = "8-bit data length (stored in bits 0 to 7 of the SDRm0 register)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slc_SPEC;
    pub type Slc = crate::EnumBitfieldStruct<u8, Slc_SPEC>;
    impl Slc {
        #[doc = "No stop bit"]
        pub const _00: Self = Self::new(0);

        #[doc = "Stop bit length = 1 bit"]
        pub const _01: Self = Self::new(1);

        #[doc = "Stop bit length = 2 bits"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Inputs or outputs data with MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "Inputs or outputs data with LSB first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptc_SPEC;
    pub type Ptc = crate::EnumBitfieldStruct<u8, Ptc_SPEC>;
    impl Ptc {
        #[doc = "Transmission: Does not output the parity bit Reception: Receives without parity"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmission: Outputs 0 parity Reception: No parity determination"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission: Outputs even parity Reception: Determines as even parity"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission: Outputs odd parity Reception: Determines as odd parity"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcp_SPEC;
    pub type Dcp = crate::EnumBitfieldStruct<u8, Dcp_SPEC>;
    impl Dcp {
        #[doc = "Type1 (SCK: inverted, Input timing: rising edge)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Type2 (SCK: non-inverted, Input timing: falling edge)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Type3 (SCK: inverted, Input timing: falling edge)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Type4 (SCK: non-inverted, Input timing: rising edge)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trxe_SPEC;
    pub type Trxe = crate::EnumBitfieldStruct<u8, Trxe_SPEC>;
    impl Trxe {
        #[doc = "Disable communication"]
        pub const _00: Self = Self::new(0);

        #[doc = "Reception only"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission only"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission and reception"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr01_SPEC;
impl crate::sealed::RegSpec for Scr01_SPEC {
    type DataType = u16;
}

#[doc = "Serial Communication Operation Setting Register 01"]
pub type Scr01 = crate::RegValueT<Scr01_SPEC>;

impl Scr01 {
    #[doc = "Setting of Data Length in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dls(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        scr01::Dls,
        scr01::Dls,
        Scr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            scr01::Dls,
            scr01::Dls,
            Scr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Stop Bit in UART Mode"]
    #[inline(always)]
    pub fn slc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scr01::Slc,
        scr01::Slc,
        Scr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scr01::Slc,
            scr01::Slc,
            Scr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data Transfer Sequence in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr01::Dir,
        scr01::Dir,
        Scr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr01::Dir,
            scr01::Dir,
            Scr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Parity Bit in UART Mode"]
    #[inline(always)]
    pub fn ptc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        scr01::Ptc,
        scr01::Ptc,
        Scr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            scr01::Ptc,
            scr01::Ptc,
            Scr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask Control of Error Interrupt Signal SAU0_UART_ERRI0 (m = 0), SAU1_UART_ERRI2 (m = 1)"]
    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        scr01::Eoc,
        scr01::Eoc,
        Scr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            scr01::Eoc,
            scr01::Eoc,
            Scr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data and Clock Phase in Simplified SPI Mode"]
    #[inline(always)]
    pub fn dcp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        scr01::Dcp,
        scr01::Dcp,
        Scr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            scr01::Dcp,
            scr01::Dcp,
            Scr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Channel 1 Operation Mode"]
    #[inline(always)]
    pub fn trxe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        scr01::Trxe,
        scr01::Trxe,
        Scr01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            scr01::Trxe,
            scr01::Trxe,
            Scr01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scr01 {
    #[inline(always)]
    fn default() -> Scr01 {
        <crate::RegValueT<Scr01_SPEC> as RegisterValue<_>>::new(135)
    }
}
pub mod scr01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dls_SPEC;
    pub type Dls = crate::EnumBitfieldStruct<u8, Dls_SPEC>;
    impl Dls {
        #[doc = "Setting prohibited"]
        pub const _00: Self = Self::new(0);

        #[doc = "9-bit data length (stored in the DAT\\[8:0\\] bits of the SDRm1 register) (settable in UART mode only)"]
        pub const _01: Self = Self::new(1);

        #[doc = "7-bit data length (stored in the DAT\\[6:0\\] bits of the SDRm1 register)"]
        pub const _10: Self = Self::new(2);

        #[doc = "8-bit data length (stored in the DAT\\[7:0\\] bits of the SDRm1 register)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slc_SPEC;
    pub type Slc = crate::EnumBitfieldStruct<u8, Slc_SPEC>;
    impl Slc {
        #[doc = "No stop bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop bit length = 1 bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Inputs or outputs data with MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "Inputs or outputs data with LSB first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptc_SPEC;
    pub type Ptc = crate::EnumBitfieldStruct<u8, Ptc_SPEC>;
    impl Ptc {
        #[doc = "Transmission: Does not output the parity bit Reception: Receives without parity"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmission: Outputs 0 parity Reception: No parity judgment"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission: Outputs even parity Reception: Determines as even parity"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission: Outputs odd parity Reception: Determines as odd parity"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoc_SPEC;
    pub type Eoc = crate::EnumBitfieldStruct<u8, Eoc_SPEC>;
    impl Eoc {
        #[doc = "Disables generation of error interrupt SAU0_UART_ERRI0 (m = 0), SAU1_UART_ERRI2 (m = 1) (SAUm_UART_RXIq is generated)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables generation of error interrupt SAU0_UART_ERRI0 (m = 0), SAU1_UART_ERRI2 (m = 1) (SAUm_UART_RXIq is not generated if an error occurs)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcp_SPEC;
    pub type Dcp = crate::EnumBitfieldStruct<u8, Dcp_SPEC>;
    impl Dcp {
        #[doc = "Type1 (SCK: inverted, Input timing: rising edge)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Type2 (SCK: non-inverted, Input timing: falling edge)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Type3 (SCK: inverted, Input timing: falling edge)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Type4 (SCK: non-inverted, Input timing: rising edge)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trxe_SPEC;
    pub type Trxe = crate::EnumBitfieldStruct<u8, Trxe_SPEC>;
    impl Trxe {
        #[doc = "Disable communication"]
        pub const _00: Self = Self::new(0);

        #[doc = "Reception only"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission only"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission and reception"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr02_SPEC;
impl crate::sealed::RegSpec for Scr02_SPEC {
    type DataType = u16;
}

#[doc = "Serial Communication Operation Setting Register 02"]
pub type Scr02 = crate::RegValueT<Scr02_SPEC>;

impl Scr02 {
    #[doc = "Setting of Data Length in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dls(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scr02::Dls,
        scr02::Dls,
        Scr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scr02::Dls,
            scr02::Dls,
            Scr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Stop Bit in UART Mode"]
    #[inline(always)]
    pub fn slc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        scr02::Slc,
        scr02::Slc,
        Scr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            scr02::Slc,
            scr02::Slc,
            Scr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data Transfer Sequence in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr02::Dir,
        scr02::Dir,
        Scr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr02::Dir,
            scr02::Dir,
            Scr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Parity Bit in UART Mode"]
    #[inline(always)]
    pub fn ptc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        scr02::Ptc,
        scr02::Ptc,
        Scr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            scr02::Ptc,
            scr02::Ptc,
            Scr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data and Clock Phase in Simplified SPI Mode"]
    #[inline(always)]
    pub fn dcp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        scr02::Dcp,
        scr02::Dcp,
        Scr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            scr02::Dcp,
            scr02::Dcp,
            Scr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Channel 2 Operation Mode"]
    #[inline(always)]
    pub fn trxe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        scr02::Trxe,
        scr02::Trxe,
        Scr02_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            scr02::Trxe,
            scr02::Trxe,
            Scr02_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scr02 {
    #[inline(always)]
    fn default() -> Scr02 {
        <crate::RegValueT<Scr02_SPEC> as RegisterValue<_>>::new(135)
    }
}
pub mod scr02 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dls_SPEC;
    pub type Dls = crate::EnumBitfieldStruct<u8, Dls_SPEC>;
    impl Dls {
        #[doc = "7-bit data length (stored in the DAT\\[6:0\\] bits of the SDR02 register)"]
        pub const _0: Self = Self::new(0);

        #[doc = "8-bit data length (stored in the DAT\\[7:0\\] bits of the SDR02 register)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slc_SPEC;
    pub type Slc = crate::EnumBitfieldStruct<u8, Slc_SPEC>;
    impl Slc {
        #[doc = "No stop bit"]
        pub const _00: Self = Self::new(0);

        #[doc = "Stop bit length = 1 bit"]
        pub const _01: Self = Self::new(1);

        #[doc = "Stop bit length = 2 bits"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Inputs or outputs data with MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "Inputs or outputs data with LSB first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptc_SPEC;
    pub type Ptc = crate::EnumBitfieldStruct<u8, Ptc_SPEC>;
    impl Ptc {
        #[doc = "Transmission: Does not output the parity bit Reception: Receives without parity"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmission: Outputs 0 parity Reception: No parity judgment"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission: Outputs even parity Reception: Determines as even parity"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission: Outputs odd parity Reception: Determines as odd parity"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcp_SPEC;
    pub type Dcp = crate::EnumBitfieldStruct<u8, Dcp_SPEC>;
    impl Dcp {
        #[doc = "Type1 (SCK: inverted, Input timing: rising edge)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Type2 (SCK: non-inverted, Input timing: falling edge)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Type3 (SCK: inverted, Input timing: falling edge)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Type4 (SCK: non-inverted, Input timing: rising edge)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trxe_SPEC;
    pub type Trxe = crate::EnumBitfieldStruct<u8, Trxe_SPEC>;
    impl Trxe {
        #[doc = "Disables communication"]
        pub const _00: Self = Self::new(0);

        #[doc = "Reception only"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission only"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission and reception"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr03_SPEC;
impl crate::sealed::RegSpec for Scr03_SPEC {
    type DataType = u16;
}

#[doc = "Serial Communication Operation Setting Register 03"]
pub type Scr03 = crate::RegValueT<Scr03_SPEC>;

impl Scr03 {
    #[doc = "Setting of Data Length in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dls(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scr03::Dls,
        scr03::Dls,
        Scr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scr03::Dls,
            scr03::Dls,
            Scr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Stop Bit in UART Mode"]
    #[inline(always)]
    pub fn slc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scr03::Slc,
        scr03::Slc,
        Scr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scr03::Slc,
            scr03::Slc,
            Scr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data Transfer Sequence in Simplified SPI and UART Modes"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr03::Dir,
        scr03::Dir,
        Scr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr03::Dir,
            scr03::Dir,
            Scr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Parity Bit in UART Mode"]
    #[inline(always)]
    pub fn ptc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        scr03::Ptc,
        scr03::Ptc,
        Scr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            scr03::Ptc,
            scr03::Ptc,
            Scr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Mask Control of Error Interrupt Signal SAU0_UART_ERRI1"]
    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        scr03::Eoc,
        scr03::Eoc,
        Scr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            scr03::Eoc,
            scr03::Eoc,
            Scr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Data and Clock Phase in Simplified SPI Mode"]
    #[inline(always)]
    pub fn dcp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        scr03::Dcp,
        scr03::Dcp,
        Scr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            scr03::Dcp,
            scr03::Dcp,
            Scr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setting of Operation Mode of Channel 3"]
    #[inline(always)]
    pub fn trxe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        scr03::Trxe,
        scr03::Trxe,
        Scr03_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            scr03::Trxe,
            scr03::Trxe,
            Scr03_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scr03 {
    #[inline(always)]
    fn default() -> Scr03 {
        <crate::RegValueT<Scr03_SPEC> as RegisterValue<_>>::new(135)
    }
}
pub mod scr03 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dls_SPEC;
    pub type Dls = crate::EnumBitfieldStruct<u8, Dls_SPEC>;
    impl Dls {
        #[doc = "7-bit data length (stored in the DAT\\[6:0\\] bits of the SDR03 register)"]
        pub const _0: Self = Self::new(0);

        #[doc = "8-bit data length (stored in the DAT\\[7:0\\] bits of the SDR03 register)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slc_SPEC;
    pub type Slc = crate::EnumBitfieldStruct<u8, Slc_SPEC>;
    impl Slc {
        #[doc = "No stop bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop bit length = 1 bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Inputs or outputs data with MSB first"]
        pub const _0: Self = Self::new(0);

        #[doc = "Inputs or outputs data with LSB first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptc_SPEC;
    pub type Ptc = crate::EnumBitfieldStruct<u8, Ptc_SPEC>;
    impl Ptc {
        #[doc = "Transmission: Does not output the parity bit Reception: Receives without parity"]
        pub const _00: Self = Self::new(0);

        #[doc = "Transmission: Outputs 0 parity Reception: No parity determination"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission: Outputs even parity Reception: Determines as even parity"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission: Outputs odd parity Reception: Determines as odd parity"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoc_SPEC;
    pub type Eoc = crate::EnumBitfieldStruct<u8, Eoc_SPEC>;
    impl Eoc {
        #[doc = "Disables generation of error interrupt SAU0_UART_ERRI1 (SAU0_UART_RXI1 is generated)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables generation of error interrupt SAU0_UART_ERRI1 (SAU0_UART_RXI1 is not generated if an error occurs)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcp_SPEC;
    pub type Dcp = crate::EnumBitfieldStruct<u8, Dcp_SPEC>;
    impl Dcp {
        #[doc = "Type1 (SCK: inverted, Input timing: rising edge)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Type2 (SCK: non-inverted, Input timing: falling edge)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Type3 (SCK: inverted, Input timing: falling edge)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Type4 (SCK: non-inverted, Input timing: rising edge)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trxe_SPEC;
    pub type Trxe = crate::EnumBitfieldStruct<u8, Trxe_SPEC>;
    impl Trxe {
        #[doc = "Disable communication"]
        pub const _00: Self = Self::new(0);

        #[doc = "Reception only"]
        pub const _01: Self = Self::new(1);

        #[doc = "Transmission only"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transmission and reception"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Se0_SPEC;
impl crate::sealed::RegSpec for Se0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Channel Enable Status Register 0"]
pub type Se0 = crate::RegValueT<Se0_SPEC>;

impl Se0 {
    #[doc = "Indication of whether Operation of Channel n is Enabled or Stopped."]
    #[inline(always)]
    pub fn se(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, se0::Se, se0::Se, Se0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xf,1,0,se0::Se,se0::Se,Se0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Se0 {
    #[inline(always)]
    fn default() -> Se0 {
        <crate::RegValueT<Se0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod se0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Se_SPEC;
    pub type Se = crate::EnumBitfieldStruct<u8, Se_SPEC>;
    impl Se {
        #[doc = "Operation stops"]
        pub const _0: Self = Self::new(0);

        #[doc = "Operation is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ss0_SPEC;
impl crate::sealed::RegSpec for Ss0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Channel Start Register 0"]
pub type Ss0 = crate::RegValueT<Ss0_SPEC>;

impl Ss0 {
    #[doc = "Operation Start Trigger of Channel n"]
    #[inline(always)]
    pub fn ss(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, ss0::Ss, ss0::Ss, Ss0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,ss0::Ss,ss0::Ss,Ss0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ss0 {
    #[inline(always)]
    fn default() -> Ss0 {
        <crate::RegValueT<Ss0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ss0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ss_SPEC;
    pub type Ss = crate::EnumBitfieldStruct<u8, Ss_SPEC>;
    impl Ss {
        #[doc = "No trigger operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set the SE0.SE\\[n\\] bit to 1 to place the channel in communications waiting state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct St0_SPEC;
impl crate::sealed::RegSpec for St0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Channel Stop Register 0"]
pub type St0 = crate::RegValueT<St0_SPEC>;

impl St0 {
    #[doc = "Operation Stop Trigger of Channel n"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, st0::St, st0::St, St0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,st0::St,st0::St,St0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for St0 {
    #[inline(always)]
    fn default() -> St0 {
        <crate::RegValueT<St0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod st0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "No trigger operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the SE0.SE\\[n\\] bit to 0 and stops the communication operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sps0_SPEC;
impl crate::sealed::RegSpec for Sps0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Clock Select Register 0"]
pub type Sps0 = crate::RegValueT<Sps0_SPEC>;

impl Sps0 {
    #[doc = "Selection of Operation Clock (CKm0)"]
    #[inline(always)]
    pub fn prs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        sps0::Prs0,
        sps0::Prs0,
        Sps0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            sps0::Prs0,
            sps0::Prs0,
            Sps0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of Operation Clock (CKm1)"]
    #[inline(always)]
    pub fn prs1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        sps0::Prs1,
        sps0::Prs1,
        Sps0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            sps0::Prs1,
            sps0::Prs1,
            Sps0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sps0 {
    #[inline(always)]
    fn default() -> Sps0 {
        <crate::RegValueT<Sps0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sps0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs0_SPEC;
    pub type Prs0 = crate::EnumBitfieldStruct<u8, Prs0_SPEC>;
    impl Prs0 {
        #[doc = "PCLKB"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "PCLKB/2"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "PCLKB/22"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "PCLKB/23"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "PCLKB/24"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "PCLKB/25"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "PCLKB/26"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "PCLKB/27"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "PCLKB/28"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "PCLKB/29"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "PCLKB/210"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "PCLKB/211"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "PCLKB/212"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "PCLKB/213"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "PCLKB/214"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "PCLKB/215"]
        pub const _0_X_F: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs1_SPEC;
    pub type Prs1 = crate::EnumBitfieldStruct<u8, Prs1_SPEC>;
    impl Prs1 {
        #[doc = "PCLKB"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "PCLKB/2"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "PCLKB/22"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "PCLKB/23"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "PCLKB/24"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "PCLKB/25"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "PCLKB/26"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "PCLKB/27"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "PCLKB/28"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "PCLKB/29"]
        pub const _0_X_9: Self = Self::new(9);

        #[doc = "PCLKB/210"]
        pub const _0_X_A: Self = Self::new(10);

        #[doc = "PCLKB/211"]
        pub const _0_X_B: Self = Self::new(11);

        #[doc = "PCLKB/212"]
        pub const _0_X_C: Self = Self::new(12);

        #[doc = "PCLKB/213"]
        pub const _0_X_D: Self = Self::new(13);

        #[doc = "PCLKB/214"]
        pub const _0_X_E: Self = Self::new(14);

        #[doc = "PCLKB/215"]
        pub const _0_X_F: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct So0_SPEC;
impl crate::sealed::RegSpec for So0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Output Register 0"]
pub type So0 = crate::RegValueT<So0_SPEC>;

impl So0 {
    #[doc = "Serial Data Output of Channel n"]
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, so0::So, so0::So, So0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,so0::So,so0::So,So0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Serial Clock Output of Channel n"]
    #[inline(always)]
    pub fn cko(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, so0::Cko, so0::Cko, So0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,so0::Cko,so0::Cko,So0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for So0 {
    #[inline(always)]
    fn default() -> So0 {
        <crate::RegValueT<So0_SPEC> as RegisterValue<_>>::new(3855)
    }
}
pub mod so0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct So_SPEC;
    pub type So = crate::EnumBitfieldStruct<u8, So_SPEC>;
    impl So {
        #[doc = "Serial data output value is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial data output value is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cko_SPEC;
    pub type Cko = crate::EnumBitfieldStruct<u8, Cko_SPEC>;
    impl Cko {
        #[doc = "Serial clock output value is 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Serial clock output value is 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soe0_SPEC;
impl crate::sealed::RegSpec for Soe0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Output Enable Register 0"]
pub type Soe0 = crate::RegValueT<Soe0_SPEC>;

impl Soe0 {
    #[doc = "Serial Output Enable or Stop of Channel n"]
    #[inline(always)]
    pub fn soe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        soe0::Soe,
        soe0::Soe,
        Soe0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            soe0::Soe,
            soe0::Soe,
            Soe0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Soe0 {
    #[inline(always)]
    fn default() -> Soe0 {
        <crate::RegValueT<Soe0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod soe0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soe_SPEC;
    pub type Soe = crate::EnumBitfieldStruct<u8, Soe_SPEC>;
    impl Soe {
        #[doc = "Stops output by serial communication operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables output by serial communication operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sol0_SPEC;
impl crate::sealed::RegSpec for Sol0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Output Level Register 0"]
pub type Sol0 = crate::RegValueT<Sol0_SPEC>;

impl Sol0 {
    #[doc = "Selects Inversion of the Level of the Transmit Data of Channel 0 in UART Mode"]
    #[inline(always)]
    pub fn sol0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sol0::Sol0,
        sol0::Sol0,
        Sol0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sol0::Sol0,
            sol0::Sol0,
            Sol0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selects Inversion of the Level of the Transmit Data of Channel 2 in UART Mode"]
    #[inline(always)]
    pub fn sol2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sol0::Sol2,
        sol0::Sol2,
        Sol0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sol0::Sol2,
            sol0::Sol2,
            Sol0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sol0 {
    #[inline(always)]
    fn default() -> Sol0 {
        <crate::RegValueT<Sol0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sol0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sol0_SPEC;
    pub type Sol0 = crate::EnumBitfieldStruct<u8, Sol0_SPEC>;
    impl Sol0 {
        #[doc = "Communication data is output as is"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication data is inverted and output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sol2_SPEC;
    pub type Sol2 = crate::EnumBitfieldStruct<u8, Sol2_SPEC>;
    impl Sol2 {
        #[doc = "Communication data is output as is"]
        pub const _0: Self = Self::new(0);

        #[doc = "Communication data is inverted and output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssc0_SPEC;
impl crate::sealed::RegSpec for Ssc0_SPEC {
    type DataType = u16;
}

#[doc = "Serial Standby Control Register 0"]
pub type Ssc0 = crate::RegValueT<Ssc0_SPEC>;

impl Ssc0 {
    #[doc = "Setting of the Snooze Mode"]
    #[inline(always)]
    pub fn swc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssc0::Swc,
        ssc0::Swc,
        Ssc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssc0::Swc,
            ssc0::Swc,
            Ssc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selection of whether to Enable or Disable the Generation of Communication Error Interrupts in the Snooze Mode"]
    #[inline(always)]
    pub fn ssec(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssc0::Ssec,
        ssc0::Ssec,
        Ssc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssc0::Ssec,
            ssc0::Ssec,
            Ssc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssc0 {
    #[inline(always)]
    fn default() -> Ssc0 {
        <crate::RegValueT<Ssc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swc_SPEC;
    pub type Swc = crate::EnumBitfieldStruct<u8, Swc_SPEC>;
    impl Swc {
        #[doc = "Do not use the Snooze mode function"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the Snooze mode function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssec_SPEC;
    pub type Ssec = crate::EnumBitfieldStruct<u8, Ssec_SPEC>;
    impl Ssec {
        #[doc = "Enable the generation of error interrupts SAU0_UART_ERRI0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable the generation of error interrupts SAU0_UART_ERRI0"]
        pub const _1: Self = Self::new(1);
    }
}
