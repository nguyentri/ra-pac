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
// Generated from SVD 1.0, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:09:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Segment LCD Controller/Driver"]
unsafe impl ::core::marker::Send for super::Slcdc {}
unsafe impl ::core::marker::Sync for super::Slcdc {}
impl super::Slcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "LCD Mode Register 0"]
    #[inline(always)]
    pub const fn lcdm0(&self) -> &'static crate::common::Reg<self::Lcdm0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdm0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "LCD Mode Register 1"]
    #[inline(always)]
    pub const fn lcdm1(&self) -> &'static crate::common::Reg<self::Lcdm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "LCD Clock Control Register 0"]
    #[inline(always)]
    pub const fn lcdc0(&self) -> &'static crate::common::Reg<self::Lcdc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "LCD Boost Level Control Register"]
    #[inline(always)]
    pub const fn vlcd(&self) -> &'static crate::common::Reg<self::Vlcd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vlcd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "LCD Display Data Register %s"]
    #[inline(always)]
    pub const fn seg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Seg_SPEC, crate::common::RW>,
        54,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn seg0(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg1(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x101usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg2(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x102usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg3(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x103usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg4(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg5(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x105usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg6(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x106usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg7(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x107usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg8(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg9(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x109usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg10(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg11(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10busize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg12(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg13(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg14(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg15(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg16(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg17(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x111usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg18(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x112usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg19(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x113usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg20(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg21(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x115usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg22(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x116usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg23(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x117usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg24(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg25(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x119usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg26(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg27(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11busize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg28(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg29(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg30(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg31(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x11fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg32(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg33(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x121usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg34(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x122usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg35(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x123usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg36(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg37(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x125usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg38(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x126usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg39(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x127usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg40(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg41(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x129usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg42(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg43(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12busize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg44(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg45(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12dusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg46(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg47(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x12fusize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg48(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg49(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x131usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg50(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x132usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg51(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x133usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg52(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn seg53(&self) -> &'static crate::common::Reg<self::Seg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Seg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x135usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm0_SPEC;
impl crate::sealed::RegSpec for Lcdm0_SPEC {
    type DataType = u8;
}

#[doc = "LCD Mode Register 0"]
pub type Lcdm0 = crate::RegValueT<Lcdm0_SPEC>;

impl Lcdm0 {
    #[doc = "LCD drive voltage generator selection"]
    #[inline(always)]
    pub fn mdset(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        lcdm0::Mdset,
        lcdm0::Mdset,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            lcdm0::Mdset,
            lcdm0::Mdset,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD display waveform selection"]
    #[inline(always)]
    pub fn lwave(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lcdm0::Lwave,
        lcdm0::Lwave,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lcdm0::Lwave,
            lcdm0::Lwave,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Time Slice of LCD Display Select"]
    #[inline(always)]
    pub fn ldty(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        lcdm0::Ldty,
        lcdm0::Ldty,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            lcdm0::Ldty,
            lcdm0::Ldty,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD Display Bias Method Select"]
    #[inline(always)]
    pub fn lbas(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lcdm0::Lbas,
        lcdm0::Lbas,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lcdm0::Lbas,
            lcdm0::Lbas,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdm0 {
    #[inline(always)]
    fn default() -> Lcdm0 {
        <crate::RegValueT<Lcdm0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdm0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mdset_SPEC;
    pub type Mdset = crate::EnumBitfieldStruct<u8, Mdset_SPEC>;
    impl Mdset {
        #[doc = "External resistance division method"]
        pub const _00: Self = Self::new(0);

        #[doc = "Internal voltage boosting method"]
        pub const _01: Self = Self::new(1);

        #[doc = "Capacitor split method"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lwave_SPEC;
    pub type Lwave = crate::EnumBitfieldStruct<u8, Lwave_SPEC>;
    impl Lwave {
        #[doc = "Waveform A"]
        pub const _0: Self = Self::new(0);

        #[doc = "Waveform B"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldty_SPEC;
    pub type Ldty = crate::EnumBitfieldStruct<u8, Ldty_SPEC>;
    impl Ldty {
        #[doc = "Static"]
        pub const _000: Self = Self::new(0);

        #[doc = "2-time slice"]
        pub const _001: Self = Self::new(1);

        #[doc = "3-time slice"]
        pub const _010: Self = Self::new(2);

        #[doc = "4-time slice"]
        pub const _011: Self = Self::new(3);

        #[doc = "8-time slice"]
        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lbas_SPEC;
    pub type Lbas = crate::EnumBitfieldStruct<u8, Lbas_SPEC>;
    impl Lbas {
        #[doc = "1/2 bias method"]
        pub const _00: Self = Self::new(0);

        #[doc = "1/3 bias method"]
        pub const _01: Self = Self::new(1);

        #[doc = "1/4 bias method"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm1_SPEC;
impl crate::sealed::RegSpec for Lcdm1_SPEC {
    type DataType = u8;
}

#[doc = "LCD Mode Register 1"]
pub type Lcdm1 = crate::RegValueT<Lcdm1_SPEC>;

impl Lcdm1 {
    #[doc = "LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn lcdon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lcdm1::Lcdon,
        lcdm1::Lcdon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lcdm1::Lcdon,
            lcdm1::Lcdon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn scoc(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lcdm1::Scoc,
        lcdm1::Scoc,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lcdm1::Scoc,
            lcdm1::Scoc,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Voltage boost circuit or capacitor split circuit operation enable/disable"]
    #[inline(always)]
    pub fn vlcon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lcdm1::Vlcon,
        lcdm1::Vlcon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lcdm1::Vlcon,
            lcdm1::Vlcon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Display data area control"]
    #[inline(always)]
    pub fn blon(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        lcdm1::Blon,
        lcdm1::Blon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            lcdm1::Blon,
            lcdm1::Blon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Display data area control"]
    #[inline(always)]
    pub fn lcdsel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        lcdm1::Lcdsel,
        lcdm1::Lcdsel,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            lcdm1::Lcdsel,
            lcdm1::Lcdsel,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Lcdm1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Lcdm1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage Boosting Pin Initial Value Switching Control"]
    #[inline(always)]
    pub fn lcdvlm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lcdm1::Lcdvlm,
        lcdm1::Lcdvlm,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lcdm1::Lcdvlm,
            lcdm1::Lcdvlm,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdm1 {
    #[inline(always)]
    fn default() -> Lcdm1 {
        <crate::RegValueT<Lcdm1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdm1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdon_SPEC;
    pub type Lcdon = crate::EnumBitfieldStruct<u8, Lcdon_SPEC>;
    impl Lcdon {
        #[doc = "Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scoc_SPEC;
    pub type Scoc = crate::EnumBitfieldStruct<u8, Scoc_SPEC>;
    impl Scoc {
        #[doc = "Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlcon_SPEC;
    pub type Vlcon = crate::EnumBitfieldStruct<u8, Vlcon_SPEC>;
    impl Vlcon {
        #[doc = "Stops voltage boost circuit or capacitor split circuit operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables voltage boost circuit or capacitor split circuit operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blon_SPEC;
    pub type Blon = crate::EnumBitfieldStruct<u8, Blon_SPEC>;
    impl Blon {
        #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdsel_SPEC;
    pub type Lcdsel = crate::EnumBitfieldStruct<u8, Lcdsel_SPEC>;
    impl Lcdsel {
        #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdvlm_SPEC;
    pub type Lcdvlm = crate::EnumBitfieldStruct<u8, Lcdvlm_SPEC>;
    impl Lcdvlm {
        #[doc = "Set when VDD >= 2.7 V"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set when VDD <= 4.2 V"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdc0_SPEC;
impl crate::sealed::RegSpec for Lcdc0_SPEC {
    type DataType = u8;
}

#[doc = "LCD Clock Control Register 0"]
pub type Lcdc0 = crate::RegValueT<Lcdc0_SPEC>;

impl Lcdc0 {
    #[doc = "LCD clock (LCDCL)"]
    #[inline(always)]
    pub fn lcdc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        lcdc0::Lcdc,
        lcdc0::Lcdc,
        Lcdc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            lcdc0::Lcdc,
            lcdc0::Lcdc,
            Lcdc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdc0 {
    #[inline(always)]
    fn default() -> Lcdc0 {
        <crate::RegValueT<Lcdc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdc_SPEC;
    pub type Lcdc = crate::EnumBitfieldStruct<u8, Lcdc_SPEC>;
    impl Lcdc {
        #[doc = "(Sub clock)/22  or  (LOCO clock)/22"]
        pub const _000001: Self = Self::new(1);

        #[doc = "(Sub clock)/23  or  (LOCO clock)/23"]
        pub const _000010: Self = Self::new(2);

        #[doc = "(Sub clock)/24  or  (LOCO clock)/24"]
        pub const _000011: Self = Self::new(3);

        #[doc = "(Sub clock)/25  or  (LOCO clock)/25"]
        pub const _000100: Self = Self::new(4);

        #[doc = "(Sub clock)/26  or  (LOCO clock)/26"]
        pub const _000101: Self = Self::new(5);

        #[doc = "(Sub clock)/27  or  (LOCO clock)/27"]
        pub const _000110: Self = Self::new(6);

        #[doc = "(Sub clock)/28  or  (LOCO clock)/28"]
        pub const _000111: Self = Self::new(7);

        #[doc = "(Sub clock)/29  or  (LOCO clock)/29"]
        pub const _001000: Self = Self::new(8);

        #[doc = "(Sub clock)/210 or  (LOCO clock)/210"]
        pub const _001001: Self = Self::new(9);

        #[doc = "(Main clock)/28 or  (HOCO clock)/28"]
        pub const _010001: Self = Self::new(17);

        #[doc = "(Main clock)/29 or  (HOCO clock)/29"]
        pub const _010010: Self = Self::new(18);

        #[doc = "(Main clock)/210 or  (HOCO clock)/210"]
        pub const _010011: Self = Self::new(19);

        #[doc = "(Main clock)/211 or  (HOCO clock)/211"]
        pub const _010100: Self = Self::new(20);

        #[doc = "(Main clock)/212 or  (HOCO clock)/212"]
        pub const _010101: Self = Self::new(21);

        #[doc = "(Main clock)/213 or  (HOCO clock)/213"]
        pub const _010110: Self = Self::new(22);

        #[doc = "(Main clock)/214 or  (HOCO clock)/214"]
        pub const _010111: Self = Self::new(23);

        #[doc = "(Main clock)/215 or  (HOCO clock)/215"]
        pub const _011000: Self = Self::new(24);

        #[doc = "(Main clock)/216 or  (HOCO clock)/216"]
        pub const _011001: Self = Self::new(25);

        #[doc = "(Main clock)/217 or  (HOCO clock)/217"]
        pub const _011010: Self = Self::new(26);

        #[doc = "(Main clock)/218 or  (HOCO clock)/218"]
        pub const _011011: Self = Self::new(27);

        #[doc = "(Main clock)/219 or  (HOCO clock)/219"]
        pub const _101011: Self = Self::new(43);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vlcd_SPEC;
impl crate::sealed::RegSpec for Vlcd_SPEC {
    type DataType = u8;
}

#[doc = "LCD Boost Level Control Register"]
pub type Vlcd = crate::RegValueT<Vlcd_SPEC>;

impl Vlcd {
    #[doc = "Reference Voltage(Contrast Adjustment) Select"]
    #[inline(always)]
    pub fn vlcd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        vlcd::Vlcd,
        vlcd::Vlcd,
        Vlcd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            vlcd::Vlcd,
            vlcd::Vlcd,
            Vlcd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vlcd {
    #[inline(always)]
    fn default() -> Vlcd {
        <crate::RegValueT<Vlcd_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod vlcd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlcd_SPEC;
    pub type Vlcd = crate::EnumBitfieldStruct<u8, Vlcd_SPEC>;
    impl Vlcd {
        #[doc = "Reference voltageselection(contrast adjustment):  1.00 V (default)  VL4 voltage:  3.00 V(1/3 bias method)/4.00 V(1/4 bias method)"]
        pub const _00100: Self = Self::new(4);

        #[doc = "Reference voltageselection(contrast adjustment):  1.05 V    VL4 voltage:  3.15 V(1/3 bias method)/4.20 V(1/4 bias method)"]
        pub const _00101: Self = Self::new(5);

        #[doc = "Reference voltageselection(contrast adjustment):  1.10 V    VL4 voltage:  3.30 V(1/3 bias method)/4.40 V(1/4 bias method)"]
        pub const _00110: Self = Self::new(6);

        #[doc = "Reference voltageselection(contrast adjustment):  1.15 V   VL4 voltage:  3.45 V(1/3 bias method)/4.60 V(1/4 bias method)"]
        pub const _00111: Self = Self::new(7);

        #[doc = "Reference voltageselection(contrast adjustment):  1.20 V    VL4 voltage:  3.60 V(1/3 bias method)/4.80 V(1/4 bias method)"]
        pub const _01000: Self = Self::new(8);

        #[doc = "Reference voltageselection(contrast adjustment):  1.25 V    VL4 voltage:  3.75 V(1/3 bias method)/5.00 V(1/4 bias method)"]
        pub const _01001: Self = Self::new(9);

        #[doc = "Reference voltageselection(contrast adjustment):  1.30 V    VL4 voltage:  3.90 V(1/3 bias method)/5.20 V(1/4 bias method)"]
        pub const _01010: Self = Self::new(10);

        #[doc = "Reference voltageselection(contrast adjustment):  1.35 V    VL4 voltage:  4.05 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _01011: Self = Self::new(11);

        #[doc = "Reference voltageselection(contrast adjustment):  1.40 V    VL4 voltage:  4.20 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _01100: Self = Self::new(12);

        #[doc = "Reference voltageselection(contrast adjustment):  1.45 V    VL4 voltage:  4.35 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _01101: Self = Self::new(13);

        #[doc = "Reference voltageselection(contrast adjustment):  1.50 V    VL4 voltage:  4.50 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _01110: Self = Self::new(14);

        #[doc = "Reference voltageselection(contrast adjustment):  1.55 V    VL4 voltage:  4.65 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _01111: Self = Self::new(15);

        #[doc = "Reference voltageselection(contrast adjustment):  1.60 V   VL4 voltage:  4.80 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _10000: Self = Self::new(16);

        #[doc = "Reference voltageselection(contrast adjustment):  1.65 V   VL4 voltage:  4.95 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _10001: Self = Self::new(17);

        #[doc = "Reference voltageselection(contrast adjustment):  1.70 V   VL4 voltage:  5.10 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _10010: Self = Self::new(18);

        #[doc = "Reference voltageselection(contrast adjustment):  1.75 V   VL4 voltage:  5.25 V(1/3 bias method)/Setting prohibited(1/4 bias method)"]
        pub const _10011: Self = Self::new(19);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seg_SPEC;
impl crate::sealed::RegSpec for Seg_SPEC {
    type DataType = u8;
}

#[doc = "LCD Display Data Register %s"]
pub type Seg = crate::RegValueT<Seg_SPEC>;

impl Seg {
    #[doc = "LCD Display Data"]
    #[inline(always)]
    pub fn seg(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Seg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Seg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Seg {
    #[inline(always)]
    fn default() -> Seg {
        <crate::RegValueT<Seg_SPEC> as RegisterValue<_>>::new(0)
    }
}
