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
#[doc = r"Peripheral Security Control Unit"]
unsafe impl ::core::marker::Send for super::Pscu {}
unsafe impl ::core::marker::Sync for super::Pscu {}
impl super::Pscu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Peripheral Security Attribution Register B"]
    #[inline(always)]
    pub const fn psarb(&self) -> &'static crate::common::Reg<self::Psarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Peripheral Security Attribution Register C"]
    #[inline(always)]
    pub const fn psarc(&self) -> &'static crate::common::Reg<self::Psarc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psarc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Peripheral Security Attribution Register D"]
    #[inline(always)]
    pub const fn psard(&self) -> &'static crate::common::Reg<self::Psard_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psard_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Peripheral Security Attribution Register E"]
    #[inline(always)]
    pub const fn psare(&self) -> &'static crate::common::Reg<self::Psare_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psare_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Module Stop Security Attribution Register"]
    #[inline(always)]
    pub const fn mssar(&self) -> &'static crate::common::Reg<self::Mssar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mssar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Peripheral Privilege Attribution Register B"]
    #[inline(always)]
    pub const fn pparb(&self) -> &'static crate::common::Reg<self::Pparb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pparb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Peripheral Privilege Attribution Register C"]
    #[inline(always)]
    pub const fn pparc(&self) -> &'static crate::common::Reg<self::Pparc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pparc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Peripheral Privilege Attribution Register D"]
    #[inline(always)]
    pub const fn ppard(&self) -> &'static crate::common::Reg<self::Ppard_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppard_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Peripheral Privilege Attribution Register E"]
    #[inline(always)]
    pub const fn ppare(&self) -> &'static crate::common::Reg<self::Ppare_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppare_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Module Stop Privilege Attribution Register"]
    #[inline(always)]
    pub const fn mspar(&self) -> &'static crate::common::Reg<self::Mspar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Code Flash Security Attribution Monitor Register A"]
    #[inline(always)]
    pub const fn cfsamona(
        &self,
    ) -> &'static crate::common::Reg<self::Cfsamona_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfsamona_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Data Flash Security Attribution Monitor Register"]
    #[inline(always)]
    pub const fn dfsamon(
        &self,
    ) -> &'static crate::common::Reg<self::Dfsamon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dfsamon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Device Lifecycle Management State Monitor Register"]
    #[inline(always)]
    pub const fn dlmmon(&self) -> &'static crate::common::Reg<self::Dlmmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dlmmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psarb_SPEC;
impl crate::sealed::RegSpec for Psarb_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register B"]
pub type Psarb = crate::RegValueT<Psarb_SPEC>;

impl Psarb {
    #[doc = "I3C Bus Interface 2 Security Attribution"]
    #[inline(always)]
    pub fn psarb4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C Bus Interface 1 Security Attribution"]
    #[inline(always)]
    pub fn psarb8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C Bus Interface 0 Security Attribution"]
    #[inline(always)]
    pub fn psarb9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Universal Serial Bus 2.0 FS Interface 0 Security Attribution"]
    #[inline(always)]
    pub fn psarb11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Universal Serial Bus 2.0 HS Interface 0 Security Attribution"]
    #[inline(always)]
    pub fn psarb12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ETHER0/EDMAC0 Controller Security Attribution"]
    #[inline(always)]
    pub fn psarb15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Octa Memory Controller Security Attribution"]
    #[inline(always)]
    pub fn psarb16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Peripheral Interface 1 Security Attribution"]
    #[inline(always)]
    pub fn psarb18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Peripheral Interface 0 Security Attribution"]
    #[inline(always)]
    pub fn psarb19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 9 Security Attribution"]
    #[inline(always)]
    pub fn psarb22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<23, 0xf, 1, 0, u8, u8, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0xf,1,0,u8,u8,Psarb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Serial Communication Interface 4 Security Attribution"]
    #[inline(always)]
    pub fn psarb27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 3 Security Attribution"]
    #[inline(always)]
    pub fn psarb28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 2 Security Attribution"]
    #[inline(always)]
    pub fn psarb29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 1 Security Attribution"]
    #[inline(always)]
    pub fn psarb30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 0 Security Attribution"]
    #[inline(always)]
    pub fn psarb31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Psarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Psarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Psarb {
    #[inline(always)]
    fn default() -> Psarb {
        <crate::RegValueT<Psarb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psarc_SPEC;
impl crate::sealed::RegSpec for Psarc_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register C"]
pub type Psarc = crate::RegValueT<Psarc_SPEC>;

impl Psarc {
    #[doc = "Clock Frequency Accuracy Measurement Circuit Security Attribution"]
    #[inline(always)]
    pub fn psarc0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Cyclic Redundancy Check Calculator Security Attribution"]
    #[inline(always)]
    pub fn psarc1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Sound Interface Enhanced (channel 1)  Security Attribution"]
    #[inline(always)]
    pub fn psarc7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Sound Interface Enhanced (channel 0)  Security Attribution"]
    #[inline(always)]
    pub fn psarc8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Secure Digital Host IF 1 Security Attribution"]
    #[inline(always)]
    pub fn psarc11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Secure Digital Host IF 0 Security Attribution"]
    #[inline(always)]
    pub fn psarc12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Data Operation Circuit Security Attribution"]
    #[inline(always)]
    pub fn psarc13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Graph-ic(GLCDC,MIPI,DRW,JPEG) Security Attribution"]
    #[inline(always)]
    pub fn psarc15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CEU Security Attribution"]
    #[inline(always)]
    pub fn psarc16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Controller Area Network with Flexible Data-Rate 1 Security Attribution"]
    #[inline(always)]
    pub fn psarc26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Controller Area Network with Flexible Data-Rate 0 Security Attribution"]
    #[inline(always)]
    pub fn psarc27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Psarc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SHIP Security Attribution"]
    #[inline(always)]
    pub fn psarc31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Psarc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Psarc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Psarc {
    #[inline(always)]
    fn default() -> Psarc {
        <crate::RegValueT<Psarc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psard_SPEC;
impl crate::sealed::RegSpec for Psard_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register D"]
pub type Psard = crate::RegValueT<Psard_SPEC>;

impl Psard {
    #[doc = "Asynchronous General Purpose Timer 1 Security Attribution"]
    #[inline(always)]
    pub fn psard4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Asynchronous General Purpose Timer 0 Security Attribution"]
    #[inline(always)]
    pub fn psard5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 3 Security Attribution"]
    #[inline(always)]
    pub fn psard11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 2 Security Attribution"]
    #[inline(always)]
    pub fn psard12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 1 Security Attribution"]
    #[inline(always)]
    pub fn psard13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 0 Security Attribution"]
    #[inline(always)]
    pub fn psard14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "12-Bit A/D 1 Converter Security Attribution"]
    #[inline(always)]
    pub fn psard15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        psard::Psard15,
        psard::Psard15,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            psard::Psard15,
            psard::Psard15,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "12-Bit A/D 0 Converter Security Attribution"]
    #[inline(always)]
    pub fn psard16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        psard::Psard16,
        psard::Psard16,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            psard::Psard16,
            psard::Psard16,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "12-Bit D/A Converter Security Attribution"]
    #[inline(always)]
    pub fn psard20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Temperature Sensor Security Attribution"]
    #[inline(always)]
    pub fn psard22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "High speed analog Comparator 1 Security Attribution"]
    #[inline(always)]
    pub fn psard27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "High speed analog Comparator 0 Security Attribution"]
    #[inline(always)]
    pub fn psard28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Psard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Psard_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Psard_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Psard {
    #[inline(always)]
    fn default() -> Psard {
        <crate::RegValueT<Psard_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psard {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard15_SPEC;
    pub type Psard15 = crate::EnumBitfieldStruct<u8, Psard15_SPEC>;
    impl Psard15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "NonSecure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard16_SPEC;
    pub type Psard16 = crate::EnumBitfieldStruct<u8, Psard16_SPEC>;
    impl Psard16 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "NonSecure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psare_SPEC;
impl crate::sealed::RegSpec for Psare_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Security Attribution Register E"]
pub type Psare = crate::RegValueT<Psare_SPEC>;

impl Psare {
    #[doc = "WDT0 Security Attribution"]
    #[inline(always)]
    pub fn psare1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        psare::Psare1,
        psare::Psare1,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            psare::Psare1,
            psare::Psare1,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Independent Watchdog Timer Security Attribution"]
    #[inline(always)]
    pub fn psare2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Real Time Clock  Security Attribution"]
    #[inline(always)]
    pub fn psare3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPT1 Security Attribution"]
    #[inline(always)]
    pub fn psare8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPT0 Security Attribution"]
    #[inline(always)]
    pub fn psare9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Psare_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "General PWM Timer channel13 Security Attribution"]
    #[inline(always)]
    pub fn psare18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel12 Security Attribution"]
    #[inline(always)]
    pub fn psare19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel11 Security Attribution"]
    #[inline(always)]
    pub fn psare20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel10 Security Attribution"]
    #[inline(always)]
    pub fn psare21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel9 Security Attribution"]
    #[inline(always)]
    pub fn psare22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel8 Security Attribution"]
    #[inline(always)]
    pub fn psare23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel7 Security Attribution"]
    #[inline(always)]
    pub fn psare24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel6 Security Attribution"]
    #[inline(always)]
    pub fn psare25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel5 Security Attribution"]
    #[inline(always)]
    pub fn psare26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel4 Security Attribution"]
    #[inline(always)]
    pub fn psare27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel3 Security Attribution"]
    #[inline(always)]
    pub fn psare28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel2 Security Attribution"]
    #[inline(always)]
    pub fn psare29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel1 Security Attribution"]
    #[inline(always)]
    pub fn psare30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel0 Security Attribution"]
    #[inline(always)]
    pub fn psare31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Psare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Psare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Psare {
    #[inline(always)]
    fn default() -> Psare {
        <crate::RegValueT<Psare_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psare {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare1_SPEC;
    pub type Psare1 = crate::EnumBitfieldStruct<u8, Psare1_SPEC>;
    impl Psare1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "NonSecure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mssar_SPEC;
impl crate::sealed::RegSpec for Mssar_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Security Attribution Register"]
pub type Mssar = crate::RegValueT<Mssar_SPEC>;

impl Mssar {
    #[doc = "SRAM0 Clock Stop Security Attribution"]
    #[inline(always)]
    pub fn mssar0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mssar::Mssar0,
        mssar::Mssar0,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mssar::Mssar0,
            mssar::Mssar0,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRAM1 Clock Stop Security Attribution"]
    #[inline(always)]
    pub fn mssar1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mssar::Mssar1,
        mssar::Mssar1,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mssar::Mssar1,
            mssar::Mssar1,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CTCM0 Security Attribution"]
    #[inline(always)]
    pub fn mssar11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        mssar::Mssar11,
        mssar::Mssar11,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            mssar::Mssar11,
            mssar::Mssar11,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "STCM0 Security Attribution"]
    #[inline(always)]
    pub fn mssar13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mssar::Mssar13,
        mssar::Mssar13,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mssar::Mssar13,
            mssar::Mssar13,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Standby RAM Clock Stop Security Attribution"]
    #[inline(always)]
    pub fn mssar15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mssar::Mssar15,
        mssar::Mssar15,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mssar::Mssar15,
            mssar::Mssar15,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMAC0/DTC0 Clock Stop Security Attribution"]
    #[inline(always)]
    pub fn mssar22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Mssar_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mssar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<23, 0xff, 1, 0, u8, u8, Mssar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0xff,1,0,u8,u8,Mssar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ELC clock stop Security Attribution"]
    #[inline(always)]
    pub fn mssar31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mssar::Mssar31,
        mssar::Mssar31,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mssar::Mssar31,
            mssar::Mssar31,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mssar {
    #[inline(always)]
    fn default() -> Mssar {
        <crate::RegValueT<Mssar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mssar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar0_SPEC;
    pub type Mssar0 = crate::EnumBitfieldStruct<u8, Mssar0_SPEC>;
    impl Mssar0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar1_SPEC;
    pub type Mssar1 = crate::EnumBitfieldStruct<u8, Mssar1_SPEC>;
    impl Mssar1 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar11_SPEC;
    pub type Mssar11 = crate::EnumBitfieldStruct<u8, Mssar11_SPEC>;
    impl Mssar11 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar13_SPEC;
    pub type Mssar13 = crate::EnumBitfieldStruct<u8, Mssar13_SPEC>;
    impl Mssar13 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar15_SPEC;
    pub type Mssar15 = crate::EnumBitfieldStruct<u8, Mssar15_SPEC>;
    impl Mssar15 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar31_SPEC;
    pub type Mssar31 = crate::EnumBitfieldStruct<u8, Mssar31_SPEC>;
    impl Mssar31 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pparb_SPEC;
impl crate::sealed::RegSpec for Pparb_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Privilege Attribution Register B"]
pub type Pparb = crate::RegValueT<Pparb_SPEC>;

impl Pparb {
    #[doc = "I3C Bus Interface 2 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C Bus Interface 1 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "I2C Bus Interface 0 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Universal Serial Bus 2.0 FS Interface 0 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Universal Serial Bus 2.0 HS Interface 0 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ETHER0/EDMAC0 Controller Privilege Attribution"]
    #[inline(always)]
    pub fn pparb15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Octa Memory Controller Privilege Attribution"]
    #[inline(always)]
    pub fn pparb16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Peripheral Interface 1 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Peripheral Interface 0 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 9 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 1111. The write value should be 1111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<23, 0xf, 1, 0, u8, u8, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0xf,1,0,u8,u8,Pparb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Serial Communication Interface 4 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 3 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 2 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 1 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Communication Interface 0 Privilege Attribution"]
    #[inline(always)]
    pub fn pparb31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Pparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pparb {
    #[inline(always)]
    fn default() -> Pparb {
        <crate::RegValueT<Pparb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pparc_SPEC;
impl crate::sealed::RegSpec for Pparc_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Privilege Attribution Register C"]
pub type Pparc = crate::RegValueT<Pparc_SPEC>;

impl Pparc {
    #[doc = "Clock Frequency Accuracy Measurement Circuit Privilege Attribution"]
    #[inline(always)]
    pub fn pparc0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Cyclic Redundancy Check Calculator Privilege Attribution"]
    #[inline(always)]
    pub fn pparc1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Sound Interface Enhanced (channel 1)  Privilege Attribution"]
    #[inline(always)]
    pub fn pparc7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Serial Sound Interface Enhanced (channel 0)  Privilege Attribution"]
    #[inline(always)]
    pub fn pparc8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Privilege Digital Host IF 1 Privilege Attribution"]
    #[inline(always)]
    pub fn pparc11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Privilege Digital Host IF 0 Privilege Attribution"]
    #[inline(always)]
    pub fn pparc12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Data Operation Circuit Privilege Attribution"]
    #[inline(always)]
    pub fn pparc13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Graph-ic(GLCDC,MIPI,DRW,JPEG) Privilege Attribution"]
    #[inline(always)]
    pub fn pparc15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CEU Privilege Attribution"]
    #[inline(always)]
    pub fn pparc16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Controller Area Network with Flexible Data-Rate 1 Privilege Attribution"]
    #[inline(always)]
    pub fn pparc26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Controller Area Network with Flexible Data-Rate 0 Privilege Attribution"]
    #[inline(always)]
    pub fn pparc27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 111. The write value should be 111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Pparc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SHIP Privilege Attribution"]
    #[inline(always)]
    pub fn pparc31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pparc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Pparc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pparc {
    #[inline(always)]
    fn default() -> Pparc {
        <crate::RegValueT<Pparc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppard_SPEC;
impl crate::sealed::RegSpec for Ppard_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Privilege Attribution Register D"]
pub type Ppard = crate::RegValueT<Ppard_SPEC>;

impl Ppard {
    #[doc = "Asynchronous General Purpose Timer 1 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Asynchronous General Purpose Timer 0 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 3 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 2 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 1 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port Output Enable for GPT Group 0 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "12-Bit A/D 1 Converter Privilege Attribution"]
    #[inline(always)]
    pub fn ppard15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ppard::Ppard15,
        ppard::Ppard15,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ppard::Ppard15,
            ppard::Ppard15,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "12-Bit A/D 0 Converter Privilege Attribution"]
    #[inline(always)]
    pub fn ppard16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ppard::Ppard16,
        ppard::Ppard16,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ppard::Ppard16,
            ppard::Ppard16,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "12-Bit D/A Converter Privilege Attribution"]
    #[inline(always)]
    pub fn ppard20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Temperature Sensor Privilege Attribution"]
    #[inline(always)]
    pub fn ppard22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "High speed analog Comparator 1 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "High speed analog Comparator 0 Privilege Attribution"]
    #[inline(always)]
    pub fn ppard28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ppard_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 111. The write value should be 111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Ppard_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Ppard_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ppard {
    #[inline(always)]
    fn default() -> Ppard {
        <crate::RegValueT<Ppard_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod ppard {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard15_SPEC;
    pub type Ppard15 = crate::EnumBitfieldStruct<u8, Ppard15_SPEC>;
    impl Ppard15 {
        #[doc = "Privilege"]
        pub const _0: Self = Self::new(0);

        #[doc = "Any Privilege"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard16_SPEC;
    pub type Ppard16 = crate::EnumBitfieldStruct<u8, Ppard16_SPEC>;
    impl Ppard16 {
        #[doc = "Privilege"]
        pub const _0: Self = Self::new(0);

        #[doc = "Any Privilege"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppare_SPEC;
impl crate::sealed::RegSpec for Ppare_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral Privilege Attribution Register E"]
pub type Ppare = crate::RegValueT<Ppare_SPEC>;

impl Ppare {
    #[doc = "Watchdog Timer0 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Independent Watchdog Timer Privilege Attribution"]
    #[inline(always)]
    pub fn ppare2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Real Time Clock  Privilege Attribution"]
    #[inline(always)]
    pub fn ppare3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPT1 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPT0 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "These bits are read as 11. The write value should be 11."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Ppare_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "General PWM Timer channel13 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel12 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel11 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel10 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel9 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel8 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel7 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel6 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel5 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel4 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel3 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel2 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel1 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "General PWM Timer channel0 Privilege Attribution"]
    #[inline(always)]
    pub fn ppare31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ppare_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ppare_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ppare {
    #[inline(always)]
    fn default() -> Ppare {
        <crate::RegValueT<Ppare_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspar_SPEC;
impl crate::sealed::RegSpec for Mspar_SPEC {
    type DataType = u32;
}

#[doc = "Module Stop Privilege Attribution Register"]
pub type Mspar = crate::RegValueT<Mspar_SPEC>;

impl Mspar {
    #[doc = "These bits are read as 111111111111111. The write value should be 111111111111111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, u16, Mspar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7fff,1,0,u16,u16,Mspar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ELC clock stop Privilege Attribution"]
    #[inline(always)]
    pub fn mspar31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mspar::Mspar31,
        mspar::Mspar31,
        Mspar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mspar::Mspar31,
            mspar::Mspar31,
            Mspar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mspar {
    #[inline(always)]
    fn default() -> Mspar {
        <crate::RegValueT<Mspar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mspar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mspar31_SPEC;
    pub type Mspar31 = crate::EnumBitfieldStruct<u8, Mspar31_SPEC>;
    impl Mspar31 {
        #[doc = "Privilege."]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Privilege"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfsamona_SPEC;
impl crate::sealed::RegSpec for Cfsamona_SPEC {
    type DataType = u32;
}

#[doc = "Code Flash Security Attribution Monitor Register A"]
pub type Cfsamona = crate::RegValueT<Cfsamona_SPEC>;

impl Cfsamona {
    #[doc = "Code Flash Secure area"]
    #[inline(always)]
    pub fn cfs2(
        self,
    ) -> crate::common::RegisterField<15, 0x1ff, 1, 0, u16, u16, Cfsamona_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1ff,1,0,u16,u16,Cfsamona_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfsamona_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfsamona_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfsamona {
    #[inline(always)]
    fn default() -> Cfsamona {
        <crate::RegValueT<Cfsamona_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsamon_SPEC;
impl crate::sealed::RegSpec for Dfsamon_SPEC {
    type DataType = u32;
}

#[doc = "Data Flash Security Attribution Monitor Register"]
pub type Dfsamon = crate::RegValueT<Dfsamon_SPEC>;

impl Dfsamon {
    #[doc = "Data flash Secure area"]
    #[inline(always)]
    pub fn dfs(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Dfsamon_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Dfsamon_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dfsamon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dfsamon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dfsamon {
    #[inline(always)]
    fn default() -> Dfsamon {
        <crate::RegValueT<Dfsamon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlmmon_SPEC;
impl crate::sealed::RegSpec for Dlmmon_SPEC {
    type DataType = u32;
}

#[doc = "Device Lifecycle Management State Monitor Register"]
pub type Dlmmon = crate::RegValueT<Dlmmon_SPEC>;

impl Dlmmon {
    #[doc = "Device Lifecycle Management State Monitor"]
    #[inline(always)]
    pub fn dlmmon(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Dlmmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Dlmmon_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits are read as 1111111111111111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dlmmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dlmmon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dlmmon {
    #[inline(always)]
    fn default() -> Dlmmon {
        <crate::RegValueT<Dlmmon_SPEC> as RegisterValue<_>>::new(4294967280)
    }
}
