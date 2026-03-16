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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:12:21 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Inter-Integrated Circuit 1"]
unsafe impl ::core::marker::Send for super::Iic1 {}
unsafe impl ::core::marker::Sync for super::Iic1 {}
impl super::Iic1 {
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

    #[doc = "Slave Address Register L%s"]
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

    #[doc = "Slave Address Register U%s"]
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

    #[doc = "I2C Bus Interface Internal ResetNote:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information)."]
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
}
impl ::core::default::Default for Iccr1 {
    #[inline(always)]
    fn default() -> Iccr1 {
        <crate::RegValueT<Iccr1_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod iccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ice_SPEC;
    pub type Ice = crate::EnumBitfieldStruct<u8, Ice_SPEC>;
    impl Ice {
        #[doc = "Disable (SCLn and SDAn pins in inactive state)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable (SCLn and SDAn pins in active state)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrst_SPEC;
    pub type Iicrst = crate::EnumBitfieldStruct<u8, Iicrst_SPEC>;
    impl Iicrst {
        #[doc = "Releases the RIIC reset or internal reset."]
        pub const _0: Self = Self::new(0);

        #[doc = "Initiates the RIIC reset or internal reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clo_SPEC;
    pub type Clo = crate::EnumBitfieldStruct<u8, Clo_SPEC>;
    impl Clo {
        #[doc = "Does not output an extra SCL clock cycle."]
        pub const _0: Self = Self::new(0);

        #[doc = "Outputs an extra SCL clock cycle."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sowp_SPEC;
    pub type Sowp = crate::EnumBitfieldStruct<u8, Sowp_SPEC>;
    impl Sowp {
        #[doc = "Enables a value to be written in SCLO bit and SDAO bit."]
        pub const _0: Self = Self::new(0);

        #[doc = "Disables a value to be written in SCLO bit and SDAO bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sclo_SPEC;
    pub type Sclo = crate::EnumBitfieldStruct<u8, Sclo_SPEC>;
    impl Sclo {
        #[doc = "(Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low."]
        pub const _0: Self = Self::new(0);

        #[doc = "(Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdao_SPEC;
    pub type Sdao = crate::EnumBitfieldStruct<u8, Sdao_SPEC>;
    impl Sdao {
        #[doc = "(Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low."]
        pub const _0: Self = Self::new(0);

        #[doc = "(Read)The RIIC has released the SDAn pin./  (Write)The RIIC releases the SDAn pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scli_SPEC;
    pub type Scli = crate::EnumBitfieldStruct<u8, Scli_SPEC>;
    impl Scli {
        #[doc = "SCLn line is low."]
        pub const _0: Self = Self::new(0);

        #[doc = "SCLn line is high."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdai_SPEC;
    pub type Sdai = crate::EnumBitfieldStruct<u8, Sdai_SPEC>;
    impl Sdai {
        #[doc = "SDAn line is low."]
        pub const _0: Self = Self::new(0);

        #[doc = "SDAn line is high."]
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

    #[doc = "Stop Condition Issuance RequestNote: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state).Note: Do not set the SP bit to 1 while a restart condition is being issued."]
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

    #[doc = "Restart Condition Issuance RequestNote: Do not set the RS bit to 1 while issuing a stop condition."]
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

    #[doc = "Start Condition Issuance RequestSet the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
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
}
impl ::core::default::Default for Iccr2 {
    #[inline(always)]
    fn default() -> Iccr2 {
        <crate::RegValueT<Iccr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bbsy_SPEC;
    pub type Bbsy = crate::EnumBitfieldStruct<u8, Bbsy_SPEC>;
    impl Bbsy {
        #[doc = "The I2C bus is released (bus free state)."]
        pub const _0: Self = Self::new(0);

        #[doc = "The I2C bus is occupied (bus busy state)."]
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
    pub struct Trs_SPEC;
    pub type Trs = crate::EnumBitfieldStruct<u8, Trs_SPEC>;
    impl Trs {
        #[doc = "Receive mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sp_SPEC;
    pub type Sp = crate::EnumBitfieldStruct<u8, Sp_SPEC>;
    impl Sp {
        #[doc = "Does not request to issue a stop condition."]
        pub const _0: Self = Self::new(0);

        #[doc = "Requests to issue a stop condition."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        #[doc = "Does not request to issue a restart condition."]
        pub const _0: Self = Self::new(0);

        #[doc = "Requests to issue a restart condition."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        #[doc = "Does not request to issue a start condition."]
        pub const _0: Self = Self::new(0);

        #[doc = "Requests to issue a start condition."]
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

    #[doc = "Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        icmr1::Cks,
        icmr1::Cks,
        Icmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            icmr1::Cks,
            icmr1::Cks,
            Icmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BC Write Protect(This bit is read as 1.)"]
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
}
impl ::core::default::Default for Icmr1 {
    #[inline(always)]
    fn default() -> Icmr1 {
        <crate::RegValueT<Icmr1_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod icmr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtwp_SPEC;
    pub type Mtwp = crate::EnumBitfieldStruct<u8, Mtwp_SPEC>;
    impl Mtwp {
        #[doc = "Disables writing to the MST and TRS bits in ICCR2."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables writing to the MST and TRS bits in ICCR2."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLKB/1 clock"]
        pub const _000: Self = Self::new(0);

        #[doc = "PCLKB/2 clock"]
        pub const _001: Self = Self::new(1);

        #[doc = "PCLKB/4 clock"]
        pub const _010: Self = Self::new(2);

        #[doc = "PCLKB/8 clock"]
        pub const _011: Self = Self::new(3);

        #[doc = "PCLKB/16 clock"]
        pub const _100: Self = Self::new(4);

        #[doc = "PCLKB/32 clock"]
        pub const _101: Self = Self::new(5);

        #[doc = "PCLKB/64 clock"]
        pub const _110: Self = Self::new(6);

        #[doc = "PCLKB/128 clock"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcwp_SPEC;
    pub type Bcwp = crate::EnumBitfieldStruct<u8, Bcwp_SPEC>;
    impl Bcwp {
        #[doc = "Enables a value to be written in the BC\\[2:0\\] bits."]
        pub const _0: Self = Self::new(0);

        #[doc = "Disables a value to be written in the BC\\[2:0\\] bits."]
        pub const _1: Self = Self::new(1);
    }
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
    #[doc = "SDA Output Delay Clock Source Selection"]
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

    #[doc = "Timeout Detection Time Selection"]
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
}
impl ::core::default::Default for Icmr2 {
    #[inline(always)]
    fn default() -> Icmr2 {
        <crate::RegValueT<Icmr2_SPEC> as RegisterValue<_>>::new(6)
    }
}
pub mod icmr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlcs_SPEC;
    pub type Dlcs = crate::EnumBitfieldStruct<u8, Dlcs_SPEC>;
    impl Dlcs {
        #[doc = "The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
        pub const _0: Self = Self::new(0);

        #[doc = "The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sddl_SPEC;
    pub type Sddl = crate::EnumBitfieldStruct<u8, Sddl_SPEC>;
    impl Sddl {
        #[doc = "No output delay"]
        pub const _000: Self = Self::new(0);

        #[doc = "1 fIIC cycle  (ICMR2.DLCS=0) / 1  or 2 fIIC cycles (ICMR2.DLCS=1)"]
        pub const _001: Self = Self::new(1);

        #[doc = "2 fIIC cycles (ICMR2.DLCS=0) /  3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
        pub const _010: Self = Self::new(2);

        #[doc = "3 fIIC cycles (ICMR2.DLCS=0) /  5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
        pub const _011: Self = Self::new(3);

        #[doc = "4 fIIC cycles (ICMR2.DLCS=0) /  7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
        pub const _100: Self = Self::new(4);

        #[doc = "5 fIIC cycles (ICMR2.DLCS=0) /  9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
        pub const _101: Self = Self::new(5);

        #[doc = "6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
        pub const _110: Self = Self::new(6);

        #[doc = "7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoh_SPEC;
    pub type Tmoh = crate::EnumBitfieldStruct<u8, Tmoh_SPEC>;
    impl Tmoh {
        #[doc = "Count is disabled while the SCLn line is at a high level."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count is enabled while the SCLn line is at a high level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmol_SPEC;
    pub type Tmol = crate::EnumBitfieldStruct<u8, Tmol_SPEC>;
    impl Tmol {
        #[doc = "Count is disabled while the SCLn line is at a low level."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count is enabled while the SCLn line is at a low level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmos_SPEC;
    pub type Tmos = crate::EnumBitfieldStruct<u8, Tmos_SPEC>;
    impl Tmos {
        #[doc = "Long mode is selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Short mode is selected."]
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
    #[doc = "SMBus/I2C Bus Selection"]
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

    #[doc = "WAITNote: When the value of the WAIT bit is to be read, be sure to read the ICDRR beforehand."]
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

    #[doc = "RDRF Flag Set Timing Selection"]
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

    #[doc = "Noise Filter Stage Selection"]
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
}
impl ::core::default::Default for Icmr3 {
    #[inline(always)]
    fn default() -> Icmr3 {
        <crate::RegValueT<Icmr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icmr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smbs_SPEC;
    pub type Smbs = crate::EnumBitfieldStruct<u8, Smbs_SPEC>;
    impl Smbs {
        #[doc = "The I2C bus is selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The SMBus is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wait_SPEC;
    pub type Wait = crate::EnumBitfieldStruct<u8, Wait_SPEC>;
    impl Wait {
        #[doc = "No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "WAIT (The period between ninth clock cycle and first clock cycle is held low.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrfs_SPEC;
    pub type Rdrfs = crate::EnumBitfieldStruct<u8, Rdrfs_SPEC>;
    impl Rdrfs {
        #[doc = "The RDRF flag is set at the rising edge of the ninth SCL clock cycle. (The SCLn line is not held low at the falling edge of the eighth clock cycle.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The RDRF flag is set at the rising edge of the eighth SCL clock cycle. (The SCLn line is held low at the falling edge of the eighth clock cycle.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackwp_SPEC;
    pub type Ackwp = crate::EnumBitfieldStruct<u8, Ackwp_SPEC>;
    impl Ackwp {
        #[doc = "Modification of the ACKBT bit is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Modification of the ACKBT bit is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackbt_SPEC;
    pub type Ackbt = crate::EnumBitfieldStruct<u8, Ackbt_SPEC>;
    impl Ackbt {
        #[doc = "A 0 is sent as the acknowledge bit (ACK transmission)."]
        pub const _0: Self = Self::new(0);

        #[doc = "A 1 is sent as the acknowledge bit (NACK transmission)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackbr_SPEC;
    pub type Ackbr = crate::EnumBitfieldStruct<u8, Ackbr_SPEC>;
    impl Ackbr {
        #[doc = "A 0 is received as the acknowledge bit (ACK reception)."]
        pub const _0: Self = Self::new(0);

        #[doc = "A 1 is received as the acknowledge bit (NACK reception)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nf_SPEC;
    pub type Nf = crate::EnumBitfieldStruct<u8, Nf_SPEC>;
    impl Nf {
        #[doc = "Noise of up to one fIIC cycle is filtered out (single-stage filter)."]
        pub const _00: Self = Self::new(0);

        #[doc = "Noise of up to two fIIC cycles is filtered out (2-stage filter)."]
        pub const _01: Self = Self::new(1);

        #[doc = "Noise of up to three fIIC cycles is filtered out (3-stage filter)."]
        pub const _10: Self = Self::new(2);

        #[doc = "Noise of up to four fIIC cycles is filtered out (4-stage filter)"]
        pub const _11: Self = Self::new(3);
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
    #[doc = "Fast-mode Plus Enable"]
    #[inline(always)]
    pub fn fmpe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icfer::Fmpe,
        icfer::Fmpe,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icfer::Fmpe,
            icfer::Fmpe,
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
}
impl ::core::default::Default for Icfer {
    #[inline(always)]
    fn default() -> Icfer {
        <crate::RegValueT<Icfer_SPEC> as RegisterValue<_>>::new(114)
    }
}
pub mod icfer {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmpe_SPEC;
    pub type Fmpe = crate::EnumBitfieldStruct<u8, Fmpe_SPEC>;
    impl Fmpe {
        #[doc = "No Fm+ slope control circuit is used for the SCLn pin and SDAn pin."]
        pub const _0: Self = Self::new(0);

        #[doc = "An Fm+ slope control circuit is used for the SCLn pin and SDAn pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scle_SPEC;
    pub type Scle = crate::EnumBitfieldStruct<u8, Scle_SPEC>;
    impl Scle {
        #[doc = "No SCL synchronous circuit is used."]
        pub const _0: Self = Self::new(0);

        #[doc = "An SCL synchronous circuit is used."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfe_SPEC;
    pub type Nfe = crate::EnumBitfieldStruct<u8, Nfe_SPEC>;
    impl Nfe {
        #[doc = "No digital noise filter circuit is used."]
        pub const _0: Self = Self::new(0);

        #[doc = "A digital noise filter circuit is used."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nacke_SPEC;
    pub type Nacke = crate::EnumBitfieldStruct<u8, Nacke_SPEC>;
    impl Nacke {
        #[doc = "Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sale_SPEC;
    pub type Sale = crate::EnumBitfieldStruct<u8, Sale_SPEC>;
    impl Sale {
        #[doc = "Slave arbitration-lost detection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave arbitration-lost detection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nale_SPEC;
    pub type Nale = crate::EnumBitfieldStruct<u8, Nale_SPEC>;
    impl Nale {
        #[doc = "NACK transmission arbitration-lost detection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK transmission arbitration-lost detection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Male_SPEC;
    pub type Male = crate::EnumBitfieldStruct<u8, Male_SPEC>;
    impl Male {
        #[doc = "Master arbitration-lost detection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Master arbitration-lost detection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoe_SPEC;
    pub type Tmoe = crate::EnumBitfieldStruct<u8, Tmoe_SPEC>;
    impl Tmoe {
        #[doc = "The timeout function is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "The timeout function is enabled."]
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
}
impl ::core::default::Default for Icser {
    #[inline(always)]
    fn default() -> Icser {
        <crate::RegValueT<Icser_SPEC> as RegisterValue<_>>::new(9)
    }
}
pub mod icser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoae_SPEC;
    pub type Hoae = crate::EnumBitfieldStruct<u8, Hoae_SPEC>;
    impl Hoae {
        #[doc = "Host address detection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Host address detection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dide_SPEC;
    pub type Dide = crate::EnumBitfieldStruct<u8, Dide_SPEC>;
    impl Dide {
        #[doc = "Device-ID address detection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Device-ID address detection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gcae_SPEC;
    pub type Gcae = crate::EnumBitfieldStruct<u8, Gcae_SPEC>;
    impl Gcae {
        #[doc = "General call address detection is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "General call address detection is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar2E_SPEC;
    pub type Sar2E = crate::EnumBitfieldStruct<u8, Sar2E_SPEC>;
    impl Sar2E {
        #[doc = "Slave address in SARL2 and SARU2 is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address in SARL2 and SARU2 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar1E_SPEC;
    pub type Sar1E = crate::EnumBitfieldStruct<u8, Sar1E_SPEC>;
    impl Sar1E {
        #[doc = "Slave address in SARL1 and SARU1 is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address in SARL1 and SARU1 is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar0E_SPEC;
    pub type Sar0E = crate::EnumBitfieldStruct<u8, Sar0E_SPEC>;
    impl Sar0E {
        #[doc = "Slave address in SARL0 and SARU0 is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address in SARL0 and SARU0 is enabled."]
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

    #[doc = "Transmit End Interrupt Request  Enable"]
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
}
impl ::core::default::Default for Icier {
    #[inline(always)]
    fn default() -> Icier {
        <crate::RegValueT<Icier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "Transmit data empty interrupt request (TXI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit data empty interrupt request (TXI) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "Transmit end interrupt request (TEI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit end interrupt request (TEI) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Receive data full interrupt request (RXI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Receive data full interrupt request (RXI) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nakie_SPEC;
    pub type Nakie = crate::EnumBitfieldStruct<u8, Nakie_SPEC>;
    impl Nakie {
        #[doc = "NACK reception interrupt request (NAKI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK reception interrupt request (NAKI) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spie_SPEC;
    pub type Spie = crate::EnumBitfieldStruct<u8, Spie_SPEC>;
    impl Spie {
        #[doc = "Stop condition detection interrupt request (SPI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop condition detection interrupt request (SPI) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stie_SPEC;
    pub type Stie = crate::EnumBitfieldStruct<u8, Stie_SPEC>;
    impl Stie {
        #[doc = "Start condition detection interrupt request (STI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start condition detection interrupt request (STI) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alie_SPEC;
    pub type Alie = crate::EnumBitfieldStruct<u8, Alie_SPEC>;
    impl Alie {
        #[doc = "Arbitration-lost interrupt request (ALI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Arbitration-lost interrupt request (ALI) is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoie_SPEC;
    pub type Tmoie = crate::EnumBitfieldStruct<u8, Tmoie_SPEC>;
    impl Tmoie {
        #[doc = "Timeout interrupt request (TMOI) is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Timeout interrupt request (TMOI) is enabled."]
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
}
impl ::core::default::Default for Icsr1 {
    #[inline(always)]
    fn default() -> Icsr1 {
        <crate::RegValueT<Icsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoa_SPEC;
    pub type Hoa = crate::EnumBitfieldStruct<u8, Hoa_SPEC>;
    impl Hoa {
        #[doc = "Host address is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Host address is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Did_SPEC;
    pub type Did = crate::EnumBitfieldStruct<u8, Did_SPEC>;
    impl Did {
        #[doc = "Device-ID command is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Device-ID command is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gca_SPEC;
    pub type Gca = crate::EnumBitfieldStruct<u8, Gca_SPEC>;
    impl Gca {
        #[doc = "General call address is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "General call address is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas2_SPEC;
    pub type Aas2 = crate::EnumBitfieldStruct<u8, Aas2_SPEC>;
    impl Aas2 {
        #[doc = "Slave address 2 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address 2 is detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas1_SPEC;
    pub type Aas1 = crate::EnumBitfieldStruct<u8, Aas1_SPEC>;
    impl Aas1 {
        #[doc = "Slave address 1 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address 1 is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas0_SPEC;
    pub type Aas0 = crate::EnumBitfieldStruct<u8, Aas0_SPEC>;
    impl Aas0 {
        #[doc = "Slave address 0 is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Slave address 0 is detected."]
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
}
impl ::core::default::Default for Icsr2 {
    #[inline(always)]
    fn default() -> Icsr2 {
        <crate::RegValueT<Icsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icsr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "ICDRT contains transmit data."]
        pub const _0: Self = Self::new(0);

        #[doc = "ICDRT contains no transmit data."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "Data is being transmitted."]
        pub const _0: Self = Self::new(0);

        #[doc = "Data has been transmitted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "ICDRR contains no receive data."]
        pub const _0: Self = Self::new(0);

        #[doc = "ICDRR contains receive data."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackf_SPEC;
    pub type Nackf = crate::EnumBitfieldStruct<u8, Nackf_SPEC>;
    impl Nackf {
        #[doc = "NACK is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "NACK is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "Stop condition is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop condition is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        #[doc = "Start condition is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start condition is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Al_SPEC;
    pub type Al = crate::EnumBitfieldStruct<u8, Al_SPEC>;
    impl Al {
        #[doc = "Arbitration is not lost."]
        pub const _0: Self = Self::new(0);

        #[doc = "Arbitration is lost."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmof_SPEC;
    pub type Tmof = crate::EnumBitfieldStruct<u8, Tmof_SPEC>;
    impl Tmof {
        #[doc = "Timeout is not detected."]
        pub const _0: Self = Self::new(0);

        #[doc = "Timeout is detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sarl_SPEC;
impl crate::sealed::RegSpec for Sarl_SPEC {
    type DataType = u8;
}

#[doc = "Slave Address Register L%s"]
pub type Sarl = crate::RegValueT<Sarl_SPEC>;

impl Sarl {
    #[doc = "A slave address is set.7-Bit Address = SVA\\[7:1\\] 10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\] }"]
    #[inline(always)]
    pub fn sva(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sarl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sarl_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "Slave Address Register U%s"]
pub type Saru = crate::RegValueT<Saru_SPEC>;

impl Saru {
    #[doc = "10-Bit Address(bit9)"]
    #[inline(always)]
    pub fn sva9(self) -> crate::common::RegisterFieldBool<2, 1, 0, Saru_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Saru_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "10-Bit Address(bit8)"]
    #[inline(always)]
    pub fn sva8(self) -> crate::common::RegisterFieldBool<1, 1, 0, Saru_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Saru_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "7-Bit/10-Bit Address Format Selection"]
    #[inline(always)]
    pub fn fs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, saru::Fs, saru::Fs, Saru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,saru::Fs,saru::Fs,Saru_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "The 7-bit address format is selected."]
        pub const _0: Self = Self::new(0);

        #[doc = "The 10-bit address format is selected."]
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
    #[doc = "Bit Rate Low-Level Period(Low-level period of SCL clock)"]
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
    #[doc = "Bit Rate High-Level Period(High-level period of SCL clock)"]
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

impl Icdrt {
    #[doc = "8-bit read-write register that stores transmit data."]
    #[inline(always)]
    pub fn icdrt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Icdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Icdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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

impl Icdrr {
    #[doc = "8-bit register that stores the received data"]
    #[inline(always)]
    pub fn icdrr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Icdrr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Icdrr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Icdrr {
    #[inline(always)]
    fn default() -> Icdrr {
        <crate::RegValueT<Icdrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
