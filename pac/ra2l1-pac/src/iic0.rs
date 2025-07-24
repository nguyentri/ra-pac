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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:47:04 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Inter-Integrated Circuit 0"]
unsafe impl ::core::marker::Send for super::Iic0 {}
unsafe impl ::core::marker::Sync for super::Iic0 {}
impl super::Iic0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "I2C Bus Control Register 1"]
    #[inline(always)]
    pub const fn iccr1(&self) -> &'static crate::common::Reg<self::Iccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "I2C Bus Control Register 2"]
    #[inline(always)]
    pub const fn iccr2(&self) -> &'static crate::common::Reg<self::Iccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "I2C Bus Mode Register 1"]
    #[inline(always)]
    pub const fn icmr1(&self) -> &'static crate::common::Reg<self::Icmr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icmr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "I2C Bus Mode Register 2"]
    #[inline(always)]
    pub const fn icmr2(&self) -> &'static crate::common::Reg<self::Icmr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icmr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "I2C Bus Mode Register 3"]
    #[inline(always)]
    pub const fn icmr3(&self) -> &'static crate::common::Reg<self::Icmr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icmr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "I2C Bus Function Enable Register"]
    #[inline(always)]
    pub const fn icfer(&self) -> &'static crate::common::Reg<self::Icfer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icfer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "I2C Bus Status Enable Register"]
    #[inline(always)]
    pub const fn icser(&self) -> &'static crate::common::Reg<self::Icser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "I2C Bus Interrupt Enable Register"]
    #[inline(always)]
    pub const fn icier(&self) -> &'static crate::common::Reg<self::Icier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "I2C Bus Status Register 1"]
    #[inline(always)]
    pub const fn icsr1(&self) -> &'static crate::common::Reg<self::Icsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "I2C Bus Status Register 2"]
    #[inline(always)]
    pub const fn icsr2(&self) -> &'static crate::common::Reg<self::Icsr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icsr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "Slave Address Register Ly"]
    #[inline(always)]
    pub const fn sarl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sarl_SPEC, crate::common::RW>,
        3,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xausize))
        }
    }
    #[inline(always)]
    pub const fn sarl0(&self) -> &'static crate::common::Reg<self::Sarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xausize),
            )
        }
    }
    #[inline(always)]
    pub const fn sarl1(&self) -> &'static crate::common::Reg<self::Sarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn sarl2(&self) -> &'static crate::common::Reg<self::Sarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xeusize),
            )
        }
    }

    #[doc = "Slave Address Register Uy"]
    #[inline(always)]
    pub const fn saru(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Saru_SPEC, crate::common::RW>,
        3,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xbusize))
        }
    }
    #[inline(always)]
    pub const fn saru0(&self) -> &'static crate::common::Reg<self::Saru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Saru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xbusize),
            )
        }
    }
    #[inline(always)]
    pub const fn saru1(&self) -> &'static crate::common::Reg<self::Saru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Saru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdusize),
            )
        }
    }
    #[inline(always)]
    pub const fn saru2(&self) -> &'static crate::common::Reg<self::Saru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Saru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xfusize),
            )
        }
    }

    #[doc = "I2C Bus Bit Rate Low-Level Register"]
    #[inline(always)]
    pub const fn icbrl(&self) -> &'static crate::common::Reg<self::Icbrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icbrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "I2C Bus Bit Rate High-Level Register"]
    #[inline(always)]
    pub const fn icbrh(&self) -> &'static crate::common::Reg<self::Icbrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icbrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[doc = "I2C Bus Transmit Data Register"]
    #[inline(always)]
    pub const fn icdrt(&self) -> &'static crate::common::Reg<self::Icdrt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icdrt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "I2C Bus Receive Data Register"]
    #[inline(always)]
    pub const fn icdrr(&self) -> &'static crate::common::Reg<self::Icdrr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Icdrr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(19usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iccr1_SPEC;
impl crate::sealed::RegSpec for Iccr1_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Control Register 1"]
pub type Iccr1 = crate::RegValueT<Iccr1_SPEC>;

impl Iccr1 {
    #[doc = "SDA Line Monitor"]
    #[inline(always)]
    pub fn sdai(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iccr1::Sdai,
        iccr1::Sdai,
        Iccr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iccr1::Sdai,
            iccr1::Sdai,
            Iccr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SCL Line Monitor"]
    #[inline(always)]
    pub fn scli(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iccr1::Scli,
        iccr1::Scli,
        Iccr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iccr1::Scli,
            iccr1::Scli,
            Iccr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Control/Monitor"]
    #[inline(always)]
    pub fn sdao(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iccr1::Sdao,
        iccr1::Sdao,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iccr1::Sdao,
            iccr1::Sdao,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Output Control/Monitor"]
    #[inline(always)]
    pub fn sclo(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iccr1::Sclo,
        iccr1::Sclo,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iccr1::Sclo,
            iccr1::Sclo,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCLO/SDAO Write Protect"]
    #[inline(always)]
    pub fn sowp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        iccr1::Sowp,
        iccr1::Sowp,
        Iccr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            iccr1::Sowp,
            iccr1::Sowp,
            Iccr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn clo(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iccr1::Clo,
        iccr1::Clo,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iccr1::Clo,
            iccr1::Clo,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Bus Interface Internal Reset"]
    #[inline(always)]
    pub fn iicrst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iccr1::Iicrst,
        iccr1::Iicrst,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iccr1::Iicrst,
            iccr1::Iicrst,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "I2C Bus Interface Enable"]
    #[inline(always)]
    pub fn ice(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iccr1::Ice,
        iccr1::Ice,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iccr1::Ice,
            iccr1::Ice,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iccr1 {
    #[inline(always)]
    fn default() -> Iccr1 {
        <crate::RegValueT<Iccr1_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod iccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdai_SPEC;
    pub type Sdai = crate::EnumBitfieldStruct<u8, Sdai_SPEC>;
    impl Sdai {
        #[doc = "SDAn line is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "SDAn line is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scli_SPEC;
    pub type Scli = crate::EnumBitfieldStruct<u8, Scli_SPEC>;
    impl Scli {
        #[doc = "SCLn line is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "SCLn line is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdao_SPEC;
    pub type Sdao = crate::EnumBitfieldStruct<u8, Sdao_SPEC>;
    impl Sdao {
        #[doc = "Read: IIC drives SDAn pin low Write: IIC drives SDAn pin low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read: IIC releases SDAn pin Write: IIC releases SDAn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sclo_SPEC;
    pub type Sclo = crate::EnumBitfieldStruct<u8, Sclo_SPEC>;
    impl Sclo {
        #[doc = "Read: IIC drives SCLn pin low Write: IIC drives SCLn pin low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read: IIC releases SCLn pin Write: IIC releases SCLn pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sowp_SPEC;
    pub type Sowp = crate::EnumBitfieldStruct<u8, Sowp_SPEC>;
    impl Sowp {
        #[doc = "Write enable SCLO and SDAO bits"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protect SCLO and SDAO bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clo_SPEC;
    pub type Clo = crate::EnumBitfieldStruct<u8, Clo_SPEC>;
    impl Clo {
        #[doc = "Do not output extra SCL clock cycle (default)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output extra SCL clock cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrst_SPEC;
    pub type Iicrst = crate::EnumBitfieldStruct<u8, Iicrst_SPEC>;
    impl Iicrst {
        #[doc = "Release IIC reset or internal reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Initiate IIC reset or internal reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ice_SPEC;
    pub type Ice = crate::EnumBitfieldStruct<u8, Ice_SPEC>;
    impl Ice {
        #[doc = "Disable (SCLn and SDAn pins in inactive state)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable (SCLn and SDAn pins in active state)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iccr2_SPEC;
impl crate::sealed::RegSpec for Iccr2_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Control Register 2"]
pub type Iccr2 = crate::RegValueT<Iccr2_SPEC>;

impl Iccr2 {
    #[doc = "Start Condition Issuance Request"]
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iccr2::St,
        iccr2::St,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iccr2::St,
            iccr2::St,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Restart Condition Issuance Request"]
    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iccr2::Rs,
        iccr2::Rs,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iccr2::Rs,
            iccr2::Rs,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Issuance Request"]
    #[inline(always)]
    pub fn sp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iccr2::Sp,
        iccr2::Sp,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iccr2::Sp,
            iccr2::Sp,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit/Receive Mode"]
    #[inline(always)]
    pub fn trs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iccr2::Trs,
        iccr2::Trs,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iccr2::Trs,
            iccr2::Trs,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master/Slave Mode"]
    #[inline(always)]
    pub fn mst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iccr2::Mst,
        iccr2::Mst,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iccr2::Mst,
            iccr2::Mst,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Bus Busy Detection Flag"]
    #[inline(always)]
    pub fn bbsy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iccr2::Bbsy,
        iccr2::Bbsy,
        Iccr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iccr2::Bbsy,
            iccr2::Bbsy,
            Iccr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iccr2 {
    #[inline(always)]
    fn default() -> Iccr2 {
        <crate::RegValueT<Iccr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "Do not issue a start condition request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Issue a start condition request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        #[doc = "Do not issue a restart condition request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Issue a restart condition request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sp_SPEC;
    pub type Sp = crate::EnumBitfieldStruct<u8, Sp_SPEC>;
    impl Sp {
        #[doc = "Do not issue a stop condition request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Issue a stop condition request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trs_SPEC;
    pub type Trs = crate::EnumBitfieldStruct<u8, Trs_SPEC>;
    impl Trs {
        #[doc = "Receive mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mst_SPEC;
    pub type Mst = crate::EnumBitfieldStruct<u8, Mst_SPEC>;
    impl Mst {
        #[doc = "Slave mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Master mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bbsy_SPEC;
    pub type Bbsy = crate::EnumBitfieldStruct<u8, Bbsy_SPEC>;
    impl Bbsy {
        #[doc = "I2C bus released (bus free state)"]
        pub const _0: Self = Self::new(0);

        #[doc = "I2C bus occupied (bus busy state)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icmr1_SPEC;
impl crate::sealed::RegSpec for Icmr1_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Mode Register 1"]
pub type Icmr1 = crate::RegValueT<Icmr1_SPEC>;

impl Icmr1 {
    #[doc = "Bit Counter"]
    #[inline(always)]
    pub fn bc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        icmr1::Bc,
        icmr1::Bc,
        Icmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            icmr1::Bc,
            icmr1::Bc,
            Icmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BC Write Protect"]
    #[inline(always)]
    pub fn bcwp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icmr1::Bcwp,
        icmr1::Bcwp,
        Icmr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icmr1::Bcwp,
            icmr1::Bcwp,
            Icmr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Internal Reference Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Icmr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Icmr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "MST/TRS Write Protect"]
    #[inline(always)]
    pub fn mtwp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icmr1::Mtwp,
        icmr1::Mtwp,
        Icmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icmr1::Mtwp,
            icmr1::Mtwp,
            Icmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icmr1 {
    #[inline(always)]
    fn default() -> Icmr1 {
        <crate::RegValueT<Icmr1_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod icmr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bc_SPEC;
    pub type Bc = crate::EnumBitfieldStruct<u8, Bc_SPEC>;
    impl Bc {
        #[doc = "9 bits"]
        pub const _000: Self = Self::new(0);

        #[doc = "2 bits"]
        pub const _001: Self = Self::new(1);

        #[doc = "3 bits"]
        pub const _010: Self = Self::new(2);

        #[doc = "4 bits"]
        pub const _011: Self = Self::new(3);

        #[doc = "5 bits"]
        pub const _100: Self = Self::new(4);

        #[doc = "6 bits"]
        pub const _101: Self = Self::new(5);

        #[doc = "7 bits"]
        pub const _110: Self = Self::new(6);

        #[doc = "8 bits"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcwp_SPEC;
    pub type Bcwp = crate::EnumBitfieldStruct<u8, Bcwp_SPEC>;
    impl Bcwp {
        #[doc = "Write enable BC\\[2:0\\] bits"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protect BC\\[2:0\\] bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtwp_SPEC;
    pub type Mtwp = crate::EnumBitfieldStruct<u8, Mtwp_SPEC>;
    impl Mtwp {
        #[doc = "Write protect MST and TRS bits in ICCR2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enable MST and TRS bits in ICCR2"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icmr2_SPEC;
impl crate::sealed::RegSpec for Icmr2_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Mode Register 2"]
pub type Icmr2 = crate::RegValueT<Icmr2_SPEC>;

impl Icmr2 {
    #[doc = "Timeout Detection Time Select"]
    #[inline(always)]
    pub fn tmos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icmr2::Tmos,
        icmr2::Tmos,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icmr2::Tmos,
            icmr2::Tmos,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout L Count Control"]
    #[inline(always)]
    pub fn tmol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icmr2::Tmol,
        icmr2::Tmol,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icmr2::Tmol,
            icmr2::Tmol,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timeout H Count Control"]
    #[inline(always)]
    pub fn tmoh(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icmr2::Tmoh,
        icmr2::Tmoh,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icmr2::Tmoh,
            icmr2::Tmoh,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Delay Counter"]
    #[inline(always)]
    pub fn sddl(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        icmr2::Sddl,
        icmr2::Sddl,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            icmr2::Sddl,
            icmr2::Sddl,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SDA Output Delay Clock Source Select"]
    #[inline(always)]
    pub fn dlcs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icmr2::Dlcs,
        icmr2::Dlcs,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icmr2::Dlcs,
            icmr2::Dlcs,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icmr2 {
    #[inline(always)]
    fn default() -> Icmr2 {
        <crate::RegValueT<Icmr2_SPEC> as RegisterValue<_>>::new(6)
    }
}
pub mod icmr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmos_SPEC;
    pub type Tmos = crate::EnumBitfieldStruct<u8, Tmos_SPEC>;
    impl Tmos {
        #[doc = "Select long mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select short mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmol_SPEC;
    pub type Tmol = crate::EnumBitfieldStruct<u8, Tmol_SPEC>;
    impl Tmol {
        #[doc = "Disable count while SCLn line is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable count while SCLn line is low"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoh_SPEC;
    pub type Tmoh = crate::EnumBitfieldStruct<u8, Tmoh_SPEC>;
    impl Tmoh {
        #[doc = "Disable count while SCLn line is high"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable count while SCLn line is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sddl_SPEC;
    pub type Sddl = crate::EnumBitfieldStruct<u8, Sddl_SPEC>;
    impl Sddl {
        #[doc = "No output delay"]
        pub const _000: Self = Self::new(0);

        #[doc = "1 IIC-phi cycle (When ICMR2.DLCS = 0 (IIC-phi)) 1 or 2 IIC-phi cycles (When ICMR2.DLCS = 1 (IIC-phi/2))"]
        pub const _001: Self = Self::new(1);

        #[doc = "2 IIC-phi cycles (When ICMR2.DLCS = 0 (IIC-phi)) 3 or 4 IIC-phi cycles (When ICMR2.DLCS = 1 (IIC-phi/2))"]
        pub const _010: Self = Self::new(2);

        #[doc = "3 IIC-phi cycles (When ICMR2.DLCS = 0 (IIC-phi)) 5 or 6 IIC-phi cycles (When ICMR2.DLCS = 1 (IIC-phi/2))"]
        pub const _011: Self = Self::new(3);

        #[doc = "4 IIC-phi cycles (When ICMR2.DLCS = 0 (IIC-phi)) 7 or 8 IIC-phi cycles (When ICMR2.DLCS = 1 (IIC-phi/2))"]
        pub const _100: Self = Self::new(4);

        #[doc = "5 IIC-phi cycles (When ICMR2.DLCS = 0 (IIC-phi)) 9 or 10 IIC-phi cycles (When ICMR2.DLCS = 1 (IIC-phi/2))"]
        pub const _101: Self = Self::new(5);

        #[doc = "6 IIC-phi cycles (When ICMR2.DLCS = 0 (IIC-phi)) 11 or 12 IIC-phi cycles (When ICMR2.DLCS = 1 (IIC-phi/2))"]
        pub const _110: Self = Self::new(6);

        #[doc = "7 IIC-phi cycles (When ICMR2.DLCS = 0 (IIC-phi)) 13 or 14 IIC-phi cycles (When ICMR2.DLCS = 1 (IIC-phi/2))"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlcs_SPEC;
    pub type Dlcs = crate::EnumBitfieldStruct<u8, Dlcs_SPEC>;
    impl Dlcs {
        #[doc = "Select internal reference clock (IIC-phi) as the clock source for SDA output delay counter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select internal reference clock divided by 2 (IIC-phi/2) as the clock source for SDA output delay counter"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icmr3_SPEC;
impl crate::sealed::RegSpec for Icmr3_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Mode Register 3"]
pub type Icmr3 = crate::RegValueT<Icmr3_SPEC>;

impl Icmr3 {
    #[doc = "Noise Filter Stage Select"]
    #[inline(always)]
    pub fn nf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        icmr3::Nf,
        icmr3::Nf,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            icmr3::Nf,
            icmr3::Nf,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Acknowledge"]
    #[inline(always)]
    pub fn ackbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icmr3::Ackbr,
        icmr3::Ackbr,
        Icmr3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icmr3::Ackbr,
            icmr3::Ackbr,
            Icmr3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Acknowledge"]
    #[inline(always)]
    pub fn ackbt(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icmr3::Ackbt,
        icmr3::Ackbt,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icmr3::Ackbt,
            icmr3::Ackbt,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ACKBT Write Protect"]
    #[inline(always)]
    pub fn ackwp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icmr3::Ackwp,
        icmr3::Ackwp,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icmr3::Ackwp,
            icmr3::Ackwp,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RDRF Flag Set Timing Select"]
    #[inline(always)]
    pub fn rdrfs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icmr3::Rdrfs,
        icmr3::Rdrfs,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icmr3::Rdrfs,
            icmr3::Rdrfs,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Low-hold is released by reading ICDRR."]
    #[inline(always)]
    pub fn wait(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icmr3::Wait,
        icmr3::Wait,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icmr3::Wait,
            icmr3::Wait,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SMBus/I2C Bus Select"]
    #[inline(always)]
    pub fn smbs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icmr3::Smbs,
        icmr3::Smbs,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icmr3::Smbs,
            icmr3::Smbs,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icmr3 {
    #[inline(always)]
    fn default() -> Icmr3 {
        <crate::RegValueT<Icmr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icmr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nf_SPEC;
    pub type Nf = crate::EnumBitfieldStruct<u8, Nf_SPEC>;
    impl Nf {
        #[doc = "Filter out noise of up to 1 IIC-phi cycle (single-stage filter)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Filter out noise of up to 2 IIC-phi cycles (2-stage filter)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Filter out noise of up to 3 IIC-phi cycles (3-stage filter)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Filter out noise of up to 4 IIC-phi cycles (4-stage filter)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackbr_SPEC;
    pub type Ackbr = crate::EnumBitfieldStruct<u8, Ackbr_SPEC>;
    impl Ackbr {
        #[doc = "0 received as the acknowledge bit (ACK reception)"]
        pub const _0: Self = Self::new(0);

        #[doc = "1 received as the acknowledge bit (NACK reception)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackbt_SPEC;
    pub type Ackbt = crate::EnumBitfieldStruct<u8, Ackbt_SPEC>;
    impl Ackbt {
        #[doc = "Send 0 as the acknowledge bit (ACK transmission)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Send 1 as the acknowledge bit (NACK transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackwp_SPEC;
    pub type Ackwp = crate::EnumBitfieldStruct<u8, Ackwp_SPEC>;
    impl Ackwp {
        #[doc = "Write protect ACKBT bit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write enable ACKBT bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrfs_SPEC;
    pub type Rdrfs = crate::EnumBitfieldStruct<u8, Rdrfs_SPEC>;
    impl Rdrfs {
        #[doc = "Set the RDRF flag on the rising edge of the 9th SCL clock cycle. The SCLn line is not held low on the falling edge of the 8th clock cycle."]
        pub const _0: Self = Self::new(0);

        #[doc = "Set the RDRF flag on the rising edge of the 8th SCL clock cycle. The SCLn line is held low on the falling edge of the 8th clock cycle."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wait_SPEC;
    pub type Wait = crate::EnumBitfieldStruct<u8, Wait_SPEC>;
    impl Wait {
        #[doc = "No wait (The SCLn line is not held low during the period between the 9th clock cycle and the 1st clock cycle.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Wait (The SCLn line is held low during the period between the 9th clock cycle and the 1st clock cycle.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smbs_SPEC;
    pub type Smbs = crate::EnumBitfieldStruct<u8, Smbs_SPEC>;
    impl Smbs {
        #[doc = "Select I2C Bus"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select SMBus"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icfer_SPEC;
impl crate::sealed::RegSpec for Icfer_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Function Enable Register"]
pub type Icfer = crate::RegValueT<Icfer_SPEC>;

impl Icfer {
    #[doc = "Timeout Function Enable"]
    #[inline(always)]
    pub fn tmoe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icfer::Tmoe,
        icfer::Tmoe,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icfer::Tmoe,
            icfer::Tmoe,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icfer::Male,
        icfer::Male,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icfer::Male,
            icfer::Male,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icfer::Nale,
        icfer::Nale,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icfer::Nale,
            icfer::Nale,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icfer::Sale,
        icfer::Sale,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icfer::Sale,
            icfer::Sale,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    pub fn nacke(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icfer::Nacke,
        icfer::Nacke,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icfer::Nacke,
            icfer::Nacke,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn nfe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icfer::Nfe,
        icfer::Nfe,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icfer::Nfe,
            icfer::Nfe,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scle(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icfer::Scle,
        icfer::Scle,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icfer::Scle,
            icfer::Scle,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icfer {
    #[inline(always)]
    fn default() -> Icfer {
        <crate::RegValueT<Icfer_SPEC> as RegisterValue<_>>::new(114)
    }
}
pub mod icfer {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoe_SPEC;
    pub type Tmoe = crate::EnumBitfieldStruct<u8, Tmoe_SPEC>;
    impl Tmoe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Male_SPEC;
    pub type Male = crate::EnumBitfieldStruct<u8, Male_SPEC>;
    impl Male {
        #[doc = "Disable the arbitration-lost detection function and disable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable the arbitration-lost detection function and enable automatic clearing of the MST and TRS bits in ICCR2 when arbitration is lost"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nale_SPEC;
    pub type Nale = crate::EnumBitfieldStruct<u8, Nale_SPEC>;
    impl Nale {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sale_SPEC;
    pub type Sale = crate::EnumBitfieldStruct<u8, Sale_SPEC>;
    impl Sale {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nacke_SPEC;
    pub type Nacke = crate::EnumBitfieldStruct<u8, Nacke_SPEC>;
    impl Nacke {
        #[doc = "Do not suspend transfer operation during NACK reception (disable transfer suspension)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Suspend transfer operation during NACK reception (enable transfer suspension)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfe_SPEC;
    pub type Nfe = crate::EnumBitfieldStruct<u8, Nfe_SPEC>;
    impl Nfe {
        #[doc = "Do not use the digital noise filter circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the digital noise filter circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scle_SPEC;
    pub type Scle = crate::EnumBitfieldStruct<u8, Scle_SPEC>;
    impl Scle {
        #[doc = "Do not use the SCL synchronous circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Use the SCL synchronous circuit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icser_SPEC;
impl crate::sealed::RegSpec for Icser_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Status Enable Register"]
pub type Icser = crate::RegValueT<Icser_SPEC>;

impl Icser {
    #[doc = "Slave Address Register 0 Enable"]
    #[inline(always)]
    pub fn sar0e(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icser::Sar0E,
        icser::Sar0E,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icser::Sar0E,
            icser::Sar0E,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Register 1 Enable"]
    #[inline(always)]
    pub fn sar1e(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icser::Sar1E,
        icser::Sar1E,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icser::Sar1E,
            icser::Sar1E,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address Register 2 Enable"]
    #[inline(always)]
    pub fn sar2e(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icser::Sar2E,
        icser::Sar2E,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icser::Sar2E,
            icser::Sar2E,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General Call Address Enable"]
    #[inline(always)]
    pub fn gcae(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icser::Gcae,
        icser::Gcae,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icser::Gcae,
            icser::Gcae,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device-ID Address Detection Enable"]
    #[inline(always)]
    pub fn dide(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icser::Dide,
        icser::Dide,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icser::Dide,
            icser::Dide,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Host Address Enable"]
    #[inline(always)]
    pub fn hoae(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icser::Hoae,
        icser::Hoae,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icser::Hoae,
            icser::Hoae,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icser {
    #[inline(always)]
    fn default() -> Icser {
        <crate::RegValueT<Icser_SPEC> as RegisterValue<_>>::new(9)
    }
}
pub mod icser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar0E_SPEC;
    pub type Sar0E = crate::EnumBitfieldStruct<u8, Sar0E_SPEC>;
    impl Sar0E {
        #[doc = "Disable slave address in SARL0 and SARU0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable slave address in SARL0 and SARU0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar1E_SPEC;
    pub type Sar1E = crate::EnumBitfieldStruct<u8, Sar1E_SPEC>;
    impl Sar1E {
        #[doc = "Disable slave address in SARL1 and SARU1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable slave address in SARL1 and SARU1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar2E_SPEC;
    pub type Sar2E = crate::EnumBitfieldStruct<u8, Sar2E_SPEC>;
    impl Sar2E {
        #[doc = "Disable slave address in SARL2 and SARU2"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable slave address in SARL2 and SARU2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gcae_SPEC;
    pub type Gcae = crate::EnumBitfieldStruct<u8, Gcae_SPEC>;
    impl Gcae {
        #[doc = "Disable general call address detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable general call address detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dide_SPEC;
    pub type Dide = crate::EnumBitfieldStruct<u8, Dide_SPEC>;
    impl Dide {
        #[doc = "Disable device-ID address detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable device-ID address detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoae_SPEC;
    pub type Hoae = crate::EnumBitfieldStruct<u8, Hoae_SPEC>;
    impl Hoae {
        #[doc = "Disable host address detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable host address detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icier_SPEC;
impl crate::sealed::RegSpec for Icier_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Interrupt Enable Register"]
pub type Icier = crate::RegValueT<Icier_SPEC>;

impl Icier {
    #[doc = "Timeout Interrupt Request Enable"]
    #[inline(always)]
    pub fn tmoie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icier::Tmoie,
        icier::Tmoie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icier::Tmoie,
            icier::Tmoie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration-Lost Interrupt Request Enable"]
    #[inline(always)]
    pub fn alie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icier::Alie,
        icier::Alie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icier::Alie,
            icier::Alie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn stie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icier::Stie,
        icier::Stie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icier::Stie,
            icier::Stie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn spie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icier::Spie,
        icier::Spie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icier::Spie,
            icier::Spie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Reception Interrupt Request Enable"]
    #[inline(always)]
    pub fn nakie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icier::Nakie,
        icier::Nakie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icier::Nakie,
            icier::Nakie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Full Interrupt Request Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icier::Rie,
        icier::Rie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icier::Rie,
            icier::Rie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Interrupt Request Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icier::Teie,
        icier::Teie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icier::Teie,
            icier::Teie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Data Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icier::Tie,
        icier::Tie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icier::Tie,
            icier::Tie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icier {
    #[inline(always)]
    fn default() -> Icier {
        <crate::RegValueT<Icier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoie_SPEC;
    pub type Tmoie = crate::EnumBitfieldStruct<u8, Tmoie_SPEC>;
    impl Tmoie {
        #[doc = "Disable timeout interrupt (TMOI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable timeout interrupt (TMOI) request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alie_SPEC;
    pub type Alie = crate::EnumBitfieldStruct<u8, Alie_SPEC>;
    impl Alie {
        #[doc = "Disable arbitration-lost interrupt (ALI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable arbitration-lost interrupt (ALI) request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stie_SPEC;
    pub type Stie = crate::EnumBitfieldStruct<u8, Stie_SPEC>;
    impl Stie {
        #[doc = "Disable start condition detection interrupt (STI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable start condition detection interrupt (STI) request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spie_SPEC;
    pub type Spie = crate::EnumBitfieldStruct<u8, Spie_SPEC>;
    impl Spie {
        #[doc = "Disable stop condition detection interrupt (SPI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable stop condition detection interrupt (SPI) request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nakie_SPEC;
    pub type Nakie = crate::EnumBitfieldStruct<u8, Nakie_SPEC>;
    impl Nakie {
        #[doc = "Disable NACK reception interrupt (NAKI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable NACK reception interrupt (NAKI) request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable receive data full interrupt (IICn_RXI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable receive data full interrupt (IICn_RXI) request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "Disable transmit end interrupt (IICn_TEI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit end interrupt (IICn_TEI) request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "Disable transmit data empty interrupt (IICn_TXI) request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transmit data empty interrupt (IICn_TXI) request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr1_SPEC;
impl crate::sealed::RegSpec for Icsr1_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Status Register 1"]
pub type Icsr1 = crate::RegValueT<Icsr1_SPEC>;

impl Icsr1 {
    #[doc = "Slave Address 0 Detection Flag"]
    #[inline(always)]
    pub fn aas0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icsr1::Aas0,
        icsr1::Aas0,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icsr1::Aas0,
            icsr1::Aas0,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address 1 Detection Flag"]
    #[inline(always)]
    pub fn aas1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icsr1::Aas1,
        icsr1::Aas1,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icsr1::Aas1,
            icsr1::Aas1,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Slave Address 2 Detection Flag"]
    #[inline(always)]
    pub fn aas2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icsr1::Aas2,
        icsr1::Aas2,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icsr1::Aas2,
            icsr1::Aas2,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "General Call Address Detection Flag"]
    #[inline(always)]
    pub fn gca(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icsr1::Gca,
        icsr1::Gca,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icsr1::Gca,
            icsr1::Gca,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device-ID Address Detection Flag"]
    #[inline(always)]
    pub fn did(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icsr1::Did,
        icsr1::Did,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icsr1::Did,
            icsr1::Did,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Host Address Detection Flag"]
    #[inline(always)]
    pub fn hoa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icsr1::Hoa,
        icsr1::Hoa,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icsr1::Hoa,
            icsr1::Hoa,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icsr1 {
    #[inline(always)]
    fn default() -> Icsr1 {
        <crate::RegValueT<Icsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas0_SPEC;
    pub type Aas0 = crate::EnumBitfieldStruct<u8, Aas0_SPEC>;
    impl Aas0 {
        #[doc = "Slave address 0 not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address 0 detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas1_SPEC;
    pub type Aas1 = crate::EnumBitfieldStruct<u8, Aas1_SPEC>;
    impl Aas1 {
        #[doc = "Slave address 1 not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address 1 detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas2_SPEC;
    pub type Aas2 = crate::EnumBitfieldStruct<u8, Aas2_SPEC>;
    impl Aas2 {
        #[doc = "Slave address 2 not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address 2 detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gca_SPEC;
    pub type Gca = crate::EnumBitfieldStruct<u8, Gca_SPEC>;
    impl Gca {
        #[doc = "General call address not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "General call address detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Did_SPEC;
    pub type Did = crate::EnumBitfieldStruct<u8, Did_SPEC>;
    impl Did {
        #[doc = "Device-ID command not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Device-ID command detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoa_SPEC;
    pub type Hoa = crate::EnumBitfieldStruct<u8, Hoa_SPEC>;
    impl Hoa {
        #[doc = "Host address not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Host address detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr2_SPEC;
impl crate::sealed::RegSpec for Icsr2_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Status Register 2"]
pub type Icsr2 = crate::RegValueT<Icsr2_SPEC>;

impl Icsr2 {
    #[doc = "Timeout Detection Flag"]
    #[inline(always)]
    pub fn tmof(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icsr2::Tmof,
        icsr2::Tmof,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icsr2::Tmof,
            icsr2::Tmof,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Arbitration-Lost Flag"]
    #[inline(always)]
    pub fn al(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icsr2::Al,
        icsr2::Al,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icsr2::Al,
            icsr2::Al,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Start Condition Detection Flag"]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icsr2::Start,
        icsr2::Start,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icsr2::Start,
            icsr2::Start,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Stop Condition Detection Flag"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icsr2::Stop,
        icsr2::Stop,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icsr2::Stop,
            icsr2::Stop,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NACK Detection Flag"]
    #[inline(always)]
    pub fn nackf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icsr2::Nackf,
        icsr2::Nackf,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icsr2::Nackf,
            icsr2::Nackf,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icsr2::Rdrf,
        icsr2::Rdrf,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icsr2::Rdrf,
            icsr2::Rdrf,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icsr2::Tend,
        icsr2::Tend,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icsr2::Tend,
            icsr2::Tend,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icsr2::Tdre,
        icsr2::Tdre,
        Icsr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icsr2::Tdre,
            icsr2::Tdre,
            Icsr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icsr2 {
    #[inline(always)]
    fn default() -> Icsr2 {
        <crate::RegValueT<Icsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icsr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmof_SPEC;
    pub type Tmof = crate::EnumBitfieldStruct<u8, Tmof_SPEC>;
    impl Tmof {
        #[doc = "Timeout not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Timeout detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Al_SPEC;
    pub type Al = crate::EnumBitfieldStruct<u8, Al_SPEC>;
    impl Al {
        #[doc = "Arbitration not lost"]
        pub const _0: Self = Self::new(0);

        #[doc = "Arbitration lost"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Start condition not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Start condition detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "Stop condition not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop condition detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackf_SPEC;
    pub type Nackf = crate::EnumBitfieldStruct<u8, Nackf_SPEC>;
    impl Nackf {
        #[doc = "NACK not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "ICDRR contains no receive data"]
        pub const _0: Self = Self::new(0);

        #[doc = "ICDRR contains receive data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "Data being transmitted"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data transmit complete"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "ICDRT contains transmit data"]
        pub const _0: Self = Self::new(0);

        #[doc = "ICDRT contains no transmit data"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sarl_SPEC;
impl crate::sealed::RegSpec for Sarl_SPEC {
    type DataType = u8;
}

#[doc = "Slave Address Register Ly"]
pub type Sarl = crate::RegValueT<Sarl_SPEC>;

impl Sarl {
    #[doc = "10-bit Address LSB"]
    #[inline(always)]
    pub fn sva0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sarl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sarl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "7-bit Address/10-bit Address Lower Bits"]
    #[inline(always)]
    pub fn sva(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sarl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sarl {
    #[inline(always)]
    fn default() -> Sarl {
        <crate::RegValueT<Sarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saru_SPEC;
impl crate::sealed::RegSpec for Saru_SPEC {
    type DataType = u8;
}

#[doc = "Slave Address Register Uy"]
pub type Saru = crate::RegValueT<Saru_SPEC>;

impl Saru {
    #[doc = "7-bit/10-bit Address Format Select"]
    #[inline(always)]
    pub fn fs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, saru::Fs, saru::Fs, Saru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,saru::Fs,saru::Fs,Saru_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "10-bit Address Upper Bits"]
    #[inline(always)]
    pub fn sva(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Saru_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Saru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Saru {
    #[inline(always)]
    fn default() -> Saru {
        <crate::RegValueT<Saru_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod saru {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fs_SPEC;
    pub type Fs = crate::EnumBitfieldStruct<u8, Fs_SPEC>;
    impl Fs {
        #[doc = "Select 7-bit address format"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select 10-bit address format"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icbrl_SPEC;
impl crate::sealed::RegSpec for Icbrl_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Bit Rate Low-Level Register"]
pub type Icbrl = crate::RegValueT<Icbrl_SPEC>;

impl Icbrl {
    #[doc = "Bit Rate Low-Level Period"]
    #[inline(always)]
    pub fn brl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Icbrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Icbrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icbrl {
    #[inline(always)]
    fn default() -> Icbrl {
        <crate::RegValueT<Icbrl_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icbrh_SPEC;
impl crate::sealed::RegSpec for Icbrh_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Bit Rate High-Level Register"]
pub type Icbrh = crate::RegValueT<Icbrh_SPEC>;

impl Icbrh {
    #[doc = "Bit Rate High-Level Period"]
    #[inline(always)]
    pub fn brh(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Icbrh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Icbrh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icbrh {
    #[inline(always)]
    fn default() -> Icbrh {
        <crate::RegValueT<Icbrh_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icdrt_SPEC;
impl crate::sealed::RegSpec for Icdrt_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Transmit Data Register"]
pub type Icdrt = crate::RegValueT<Icdrt_SPEC>;

impl NoBitfieldReg<Icdrt_SPEC> for Icdrt {}
impl ::core::default::Default for Icdrt {
    #[inline(always)]
    fn default() -> Icdrt {
        <crate::RegValueT<Icdrt_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icdrr_SPEC;
impl crate::sealed::RegSpec for Icdrr_SPEC {
    type DataType = u8;
}

#[doc = "I2C Bus Receive Data Register"]
pub type Icdrr = crate::RegValueT<Icdrr_SPEC>;

impl NoBitfieldReg<Icdrr_SPEC> for Icdrr {}
impl ::core::default::Default for Icdrr {
    #[inline(always)]
    fn default() -> Icdrr {
        <crate::RegValueT<Icdrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
