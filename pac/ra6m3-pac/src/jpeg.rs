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
#[doc = r"JPEG Codec"]
unsafe impl ::core::marker::Send for super::Jpeg {}
unsafe impl ::core::marker::Sync for super::Jpeg {}
impl super::Jpeg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "JPEG Code Mode Register"]
    #[inline(always)]
    pub const fn jcmod(&self) -> &'static crate::common::Reg<self::Jcmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "JPEG Code Command Register"]
    #[inline(always)]
    pub const fn jccmd(&self) -> &'static crate::common::Reg<self::Jccmd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Jccmd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "JPEG Code Quantization Table Number Register"]
    #[inline(always)]
    pub const fn jcqtn(&self) -> &'static crate::common::Reg<self::Jcqtn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcqtn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "JPEG Code Huffman Table Number Register"]
    #[inline(always)]
    pub const fn jchtn(&self) -> &'static crate::common::Reg<self::Jchtn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jchtn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "JPEG Code DRI Upper Register"]
    #[inline(always)]
    pub const fn jcdriu(
        &self,
    ) -> &'static crate::common::Reg<self::Jcdriu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcdriu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "JPEG Code DRI Lower Register"]
    #[inline(always)]
    pub const fn jcdrid(
        &self,
    ) -> &'static crate::common::Reg<self::Jcdrid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcdrid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "JPEG Code Vertical Size Upper Register"]
    #[inline(always)]
    pub const fn jcvszu(
        &self,
    ) -> &'static crate::common::Reg<self::Jcvszu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcvszu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "JPEG Code Vertical Size Lower Register"]
    #[inline(always)]
    pub const fn jcvszd(
        &self,
    ) -> &'static crate::common::Reg<self::Jcvszd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcvszd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "JPEG Code Horizontal Size Upper Register"]
    #[inline(always)]
    pub const fn jchszu(
        &self,
    ) -> &'static crate::common::Reg<self::Jchszu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jchszu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "JPEG Coded Horizontal Size Lower Register"]
    #[inline(always)]
    pub const fn jchszd(
        &self,
    ) -> &'static crate::common::Reg<self::Jchszd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jchszd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "JPEG Code Data Count Upper Register"]
    #[inline(always)]
    pub const fn jcdtcu(&self) -> &'static crate::common::Reg<self::Jcdtcu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcdtcu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "JPEG Code Data Count Middle Register"]
    #[inline(always)]
    pub const fn jcdtcm(&self) -> &'static crate::common::Reg<self::Jcdtcm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcdtcm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "JPEG Code Data Count Lower Register"]
    #[inline(always)]
    pub const fn jcdtcd(&self) -> &'static crate::common::Reg<self::Jcdtcd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcdtcd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "JPEG Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn jinte0(
        &self,
    ) -> &'static crate::common::Reg<self::Jinte0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jinte0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "JPEG Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn jints0(
        &self,
    ) -> &'static crate::common::Reg<self::Jints0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jints0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[doc = "JPEG Code Decode Error Register"]
    #[inline(always)]
    pub const fn jcderr(
        &self,
    ) -> &'static crate::common::Reg<self::Jcderr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcderr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "JPEG Code Reset Register"]
    #[inline(always)]
    pub const fn jcrst(&self) -> &'static crate::common::Reg<self::Jcrst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcrst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[doc = "JPEG Interface Compression Control Register"]
    #[inline(always)]
    pub const fn jifecnt(
        &self,
    ) -> &'static crate::common::Reg<self::Jifecnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifecnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "JPEG Interface Compression Source Address Register"]
    #[inline(always)]
    pub const fn jifesa(
        &self,
    ) -> &'static crate::common::Reg<self::Jifesa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifesa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "JPEG Interface Compression Line Offset Register"]
    #[inline(always)]
    pub const fn jifesofst(
        &self,
    ) -> &'static crate::common::Reg<self::Jifesofst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifesofst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "JPEG Interface Compression Destination Address Register"]
    #[inline(always)]
    pub const fn jifeda(
        &self,
    ) -> &'static crate::common::Reg<self::Jifeda_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifeda_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "JPEG Interface Compression Source Line Count Register"]
    #[inline(always)]
    pub const fn jifeslc(
        &self,
    ) -> &'static crate::common::Reg<self::Jifeslc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifeslc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "JPEG Interface Decompression Control Register"]
    #[inline(always)]
    pub const fn jifdcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "JPEG Interface Decompression Source Address Register"]
    #[inline(always)]
    pub const fn jifdsa(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdsa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdsa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "JPEG Interface Decompression Line Offset Register"]
    #[inline(always)]
    pub const fn jifddofst(
        &self,
    ) -> &'static crate::common::Reg<self::Jifddofst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifddofst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "JPEG Interface Decompression Destination Address Register"]
    #[inline(always)]
    pub const fn jifdda(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdda_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdda_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "JPEG Interface Decompression Source Data Count Register"]
    #[inline(always)]
    pub const fn jifdsdc(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdsdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdsdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "JPEG Interface Decompression Destination Line Count Register"]
    #[inline(always)]
    pub const fn jifddlc(
        &self,
    ) -> &'static crate::common::Reg<self::Jifddlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifddlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "JPEG Interface Decompression alpha Set Register"]
    #[inline(always)]
    pub const fn jifdadt(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdadt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdadt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "JPEG Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn jinte1(
        &self,
    ) -> &'static crate::common::Reg<self::Jinte1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jinte1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "JPEG Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn jints1(
        &self,
    ) -> &'static crate::common::Reg<self::Jints1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jints1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcmod_SPEC;
impl crate::sealed::RegSpec for Jcmod_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Mode Register"]
pub type Jcmod = crate::RegValueT<Jcmod_SPEC>;

impl Jcmod {
    #[doc = "Compression/Decompression Set Note: When changing between processing for compression and for decompression, be sure to reset this module in advance by setting the JCUSRST bit in the software reset control register 2 (SWRSTCR2) of the power-downmodes."]
    #[inline(always)]
    pub fn dsp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        jcmod::Dsp,
        jcmod::Dsp,
        Jcmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            jcmod::Dsp,
            jcmod::Dsp,
            Jcmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pixel FormatNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn redu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        jcmod::Redu,
        jcmod::Redu,
        Jcmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            jcmod::Redu,
            jcmod::Redu,
            Jcmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcmod {
    #[inline(always)]
    fn default() -> Jcmod {
        <crate::RegValueT<Jcmod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jcmod {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsp_SPEC;
    pub type Dsp = crate::EnumBitfieldStruct<u8, Dsp_SPEC>;
    impl Dsp {
        #[doc = "Compression process"]
        pub const _0: Self = Self::new(0);

        #[doc = "Decompression process"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Redu_SPEC;
    pub type Redu = crate::EnumBitfieldStruct<u8, Redu_SPEC>;
    impl Redu {
        #[doc = "YCbCr422(Compression) / YCbCr422(Decompression)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Setting prohibited(Compression) / YCbCr444(Decompression)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Setting prohibited(Compression) / YCbCr411/\\[Decompression\\]"]
        pub const _110: Self = Self::new(6);

        #[doc = "Setting prohibited(Compression) / YCbCr420/\\[Decompression\\]"]
        pub const _010: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jccmd_SPEC;
impl crate::sealed::RegSpec for Jccmd_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Command Register"]
pub type Jccmd = crate::RegValueT<Jccmd_SPEC>;

impl Jccmd {
    #[doc = "Bus Reset. NOTE: When this module is in operation, the bus reset command should not be issued."]
    #[inline(always)]
    pub fn brst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        jccmd::Brst,
        jccmd::Brst,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            jccmd::Brst,
            jccmd::Brst,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Interrupt Request Clear Command This bit is valid only for the interrupt sources corresponding to bits INS6, INS5, and INS3 in JINTS0. To clear an interrupt request, set this bit to 1"]
    #[inline(always)]
    pub fn jend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        jccmd::Jend,
        jccmd::Jend,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            jccmd::Jend,
            jccmd::Jend,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "JPEG Core Process Stop Clear CommandTo clear the process-stopped state caused by requests to read the image size and pixel format (enabled by the INT3 bit in JINTE0), set this bit to 1."]
    #[inline(always)]
    pub fn jrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        jccmd::Jrst,
        jccmd::Jrst,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            jccmd::Jrst,
            jccmd::Jrst,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "JPEG Core Process Start CommandTo start JPEG core processing, set this bit to 1. Do not write this bit to 1 again while this module is in operation."]
    #[inline(always)]
    pub fn jsrt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jccmd::Jsrt,
        jccmd::Jsrt,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jccmd::Jsrt,
            jccmd::Jsrt,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jccmd {
    #[inline(always)]
    fn default() -> Jccmd {
        <crate::RegValueT<Jccmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jccmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brst_SPEC;
    pub type Brst = crate::EnumBitfieldStruct<u8, Brst_SPEC>;
    impl Brst {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Resets the JCDTCU, JCDTCM, JCDTCD, JCDERR and JCRST registers."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jend_SPEC;
    pub type Jend = crate::EnumBitfieldStruct<u8, Jend_SPEC>;
    impl Jend {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear all bits in JINTE0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jrst_SPEC;
    pub type Jrst = crate::EnumBitfieldStruct<u8, Jrst_SPEC>;
    impl Jrst {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the process-stopped state caused by requests to read the image size and pixel format(enabled by the INT3 bit in JINTE0)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jsrt_SPEC;
    pub type Jsrt = crate::EnumBitfieldStruct<u8, Jsrt_SPEC>;
    impl Jsrt {
        #[doc = "No effect."]
        pub const _0: Self = Self::new(0);

        #[doc = "Start JPEG core processing"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcqtn_SPEC;
impl crate::sealed::RegSpec for Jcqtn_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Quantization Table Number Register"]
pub type Jcqtn = crate::RegValueT<Jcqtn_SPEC>;

impl Jcqtn {
    #[doc = "Quantization table number for the third color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn qt3(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        jcqtn::Qt3,
        jcqtn::Qt3,
        Jcqtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            jcqtn::Qt3,
            jcqtn::Qt3,
            Jcqtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Quantization table number for the second color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn qt2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        jcqtn::Qt2,
        jcqtn::Qt2,
        Jcqtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            jcqtn::Qt2,
            jcqtn::Qt2,
            Jcqtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Quantization table number for the first color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn qt1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        jcqtn::Qt1,
        jcqtn::Qt1,
        Jcqtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            jcqtn::Qt1,
            jcqtn::Qt1,
            Jcqtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcqtn {
    #[inline(always)]
    fn default() -> Jcqtn {
        <crate::RegValueT<Jcqtn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jcqtn {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qt3_SPEC;
    pub type Qt3 = crate::EnumBitfieldStruct<u8, Qt3_SPEC>;
    impl Qt3 {
        #[doc = "Use quantization table No.0 (JCQTBL0) as the third color component."]
        pub const _00: Self = Self::new(0);

        #[doc = "Use quantization table No.1 (JCQTBL1) as the third color component."]
        pub const _01: Self = Self::new(1);

        #[doc = "Use quantization table No.2 (JCQTBL2) as the third color component."]
        pub const _10: Self = Self::new(2);

        #[doc = "Use quantization table No.3 (JCQTBL3) as the third color component."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qt2_SPEC;
    pub type Qt2 = crate::EnumBitfieldStruct<u8, Qt2_SPEC>;
    impl Qt2 {
        #[doc = "Use quantization table No.0 (JCQTBL0) as the second color component."]
        pub const _00: Self = Self::new(0);

        #[doc = "Use quantization table No.1 (JCQTBL1) as the second color component."]
        pub const _01: Self = Self::new(1);

        #[doc = "Use quantization table No.2 (JCQTBL2) as the second color component."]
        pub const _10: Self = Self::new(2);

        #[doc = "Use quantization table No.3 (JCQTBL3) as the second color component."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qt1_SPEC;
    pub type Qt1 = crate::EnumBitfieldStruct<u8, Qt1_SPEC>;
    impl Qt1 {
        #[doc = "Use quantization table No.0 (JCQTBL0) as the first color component."]
        pub const _00: Self = Self::new(0);

        #[doc = "Use quantization table No.1 (JCQTBL1) as the first color component."]
        pub const _01: Self = Self::new(1);

        #[doc = "Use quantization table No.2 (JCQTBL2) as the first color component."]
        pub const _10: Self = Self::new(2);

        #[doc = "Use quantization table No.3 (JCQTBL3) as the first color component."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jchtn_SPEC;
impl crate::sealed::RegSpec for Jchtn_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Huffman Table Number Register"]
pub type Jchtn = crate::RegValueT<Jchtn_SPEC>;

impl Jchtn {
    #[doc = "Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hta3(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        jchtn::Hta3,
        jchtn::Hta3,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            jchtn::Hta3,
            jchtn::Hta3,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Huffman table number (DC) for the third color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn htd3(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        jchtn::Htd3,
        jchtn::Htd3,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            jchtn::Htd3,
            jchtn::Htd3,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hta2(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        jchtn::Hta2,
        jchtn::Hta2,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            jchtn::Hta2,
            jchtn::Hta2,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Huffman table number (DC) for the second color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn htd2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        jchtn::Htd2,
        jchtn::Htd2,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            jchtn::Htd2,
            jchtn::Htd2,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hta1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        jchtn::Hta1,
        jchtn::Hta1,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            jchtn::Hta1,
            jchtn::Hta1,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Huffman table number (DC) for the first color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn htd1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jchtn::Htd1,
        jchtn::Htd1,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jchtn::Htd1,
            jchtn::Htd1,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jchtn {
    #[inline(always)]
    fn default() -> Jchtn {
        <crate::RegValueT<Jchtn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jchtn {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hta3_SPEC;
    pub type Hta3 = crate::EnumBitfieldStruct<u8, Hta3_SPEC>;
    impl Hta3 {
        #[doc = "AC Huffman table 0(HTD3=0)/Setting prohibited(HTD3=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "AC Huffman table 1(HTD3=1)/Setting prohibited(HTD3=0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htd3_SPEC;
    pub type Htd3 = crate::EnumBitfieldStruct<u8, Htd3_SPEC>;
    impl Htd3 {
        #[doc = "DC Huffman table 0(HTA3=0)/Setting prohibited(HTA3=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "DC Huffman table 1(HTA3=1)/Setting prohibited(HTA3=0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hta2_SPEC;
    pub type Hta2 = crate::EnumBitfieldStruct<u8, Hta2_SPEC>;
    impl Hta2 {
        #[doc = "AC Huffman table 0(HTD2=0)/Setting prohibited(HTD2=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "AC Huffman table 1(HTD2=1)/Setting prohibited(HTD2=0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htd2_SPEC;
    pub type Htd2 = crate::EnumBitfieldStruct<u8, Htd2_SPEC>;
    impl Htd2 {
        #[doc = "DC Huffman table 0(HTA2=0)/Setting prohibited(HTA2=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "DC Huffman table 1(HTA2=1)/Setting prohibited(HTA2=0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hta1_SPEC;
    pub type Hta1 = crate::EnumBitfieldStruct<u8, Hta1_SPEC>;
    impl Hta1 {
        #[doc = "AC Huffman table 0(HTD1=0)/Setting prohibited(HTD1=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "AC Huffman table 1(HTD1=1)/Setting prohibited(HTD1=0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htd1_SPEC;
    pub type Htd1 = crate::EnumBitfieldStruct<u8, Htd1_SPEC>;
    impl Htd1 {
        #[doc = "DC Huffman table 0(HTA1=0)/Setting prohibited(HTA1=1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "DC Huffman table 1(HTA1=1)/Setting prohibited(HTA1=0)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdriu_SPEC;
impl crate::sealed::RegSpec for Jcdriu_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code DRI Upper Register"]
pub type Jcdriu = crate::RegValueT<Jcdriu_SPEC>;

impl Jcdriu {
    #[doc = "Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn driu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdriu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdriu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdriu {
    #[inline(always)]
    fn default() -> Jcdriu {
        <crate::RegValueT<Jcdriu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdrid_SPEC;
impl crate::sealed::RegSpec for Jcdrid_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code DRI Lower Register"]
pub type Jcdrid = crate::RegValueT<Jcdrid_SPEC>;

impl Jcdrid {
    #[doc = "Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn drid(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdrid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdrid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdrid {
    #[inline(always)]
    fn default() -> Jcdrid {
        <crate::RegValueT<Jcdrid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcvszu_SPEC;
impl crate::sealed::RegSpec for Jcvszu_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Vertical Size Upper Register"]
pub type Jcvszu = crate::RegValueT<Jcvszu_SPEC>;

impl Jcvszu {
    #[doc = "Upper Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn vszu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcvszu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcvszu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcvszu {
    #[inline(always)]
    fn default() -> Jcvszu {
        <crate::RegValueT<Jcvszu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcvszd_SPEC;
impl crate::sealed::RegSpec for Jcvszd_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Vertical Size Lower Register"]
pub type Jcvszd = crate::RegValueT<Jcvszd_SPEC>;

impl Jcvszd {
    #[doc = "Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn vszd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcvszd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcvszd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcvszd {
    #[inline(always)]
    fn default() -> Jcvszd {
        <crate::RegValueT<Jcvszd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jchszu_SPEC;
impl crate::sealed::RegSpec for Jchszu_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Horizontal Size Upper Register"]
pub type Jchszu = crate::RegValueT<Jchszu_SPEC>;

impl Jchszu {
    #[doc = "Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hszu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jchszu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jchszu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jchszu {
    #[inline(always)]
    fn default() -> Jchszu {
        <crate::RegValueT<Jchszu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jchszd_SPEC;
impl crate::sealed::RegSpec for Jchszd_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Coded Horizontal Size Lower Register"]
pub type Jchszd = crate::RegValueT<Jchszd_SPEC>;

impl Jchszd {
    #[doc = "Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn hszd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jchszd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jchszd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jchszd {
    #[inline(always)]
    fn default() -> Jchszd {
        <crate::RegValueT<Jchszd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdtcu_SPEC;
impl crate::sealed::RegSpec for Jcdtcu_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Data Count Upper Register"]
pub type Jcdtcu = crate::RegValueT<Jcdtcu_SPEC>;

impl Jcdtcu {
    #[doc = "Upper bytes of the counted amount of data to be compressed The values of this register are reset before compression starts.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn dcu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdtcu_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdtcu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdtcu {
    #[inline(always)]
    fn default() -> Jcdtcu {
        <crate::RegValueT<Jcdtcu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdtcm_SPEC;
impl crate::sealed::RegSpec for Jcdtcm_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Data Count Middle Register"]
pub type Jcdtcm = crate::RegValueT<Jcdtcm_SPEC>;

impl Jcdtcm {
    #[doc = "Middle bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts. NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn dcm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdtcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdtcm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdtcm {
    #[inline(always)]
    fn default() -> Jcdtcm {
        <crate::RegValueT<Jcdtcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdtcd_SPEC;
impl crate::sealed::RegSpec for Jcdtcd_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Data Count Lower Register"]
pub type Jcdtcd = crate::RegValueT<Jcdtcd_SPEC>;

impl Jcdtcd {
    #[doc = "Lower bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn dcd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdtcd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdtcd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdtcd {
    #[inline(always)]
    fn default() -> Jcdtcd {
        <crate::RegValueT<Jcdtcd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jinte0_SPEC;
impl crate::sealed::RegSpec for Jinte0_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Interrupt Enable Register 0"]
pub type Jinte0 = crate::RegValueT<Jinte0_SPEC>;

impl Jinte0 {
    #[doc = "This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    pub fn int7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        jinte0::Int7,
        jinte0::Int7,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            jinte0::Int7,
            jinte0::Int7,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    pub fn int6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jinte0::Int6,
        jinte0::Int6,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jinte0::Int6,
            jinte0::Int6,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    pub fn int5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        jinte0::Int5,
        jinte0::Int5,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            jinte0::Int5,
            jinte0::Int5,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data."]
    #[inline(always)]
    pub fn int3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        jinte0::Int3,
        jinte0::Int3,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            jinte0::Int3,
            jinte0::Int3,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jinte0 {
    #[inline(always)]
    fn default() -> Jinte0 {
        <crate::RegValueT<Jinte0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jinte0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int7_SPEC;
    pub type Int7 = crate::EnumBitfieldStruct<u8, Int7_SPEC>;
    impl Int7 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int6_SPEC;
    pub type Int6 = crate::EnumBitfieldStruct<u8, Int6_SPEC>;
    impl Int6 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int5_SPEC;
    pub type Int5 = crate::EnumBitfieldStruct<u8, Int5_SPEC>;
    impl Int5 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int3_SPEC;
    pub type Int3 = crate::EnumBitfieldStruct<u8, Int3_SPEC>;
    impl Int3 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jints0_SPEC;
impl crate::sealed::RegSpec for Jints0_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Interrupt Status Register 0"]
pub type Jints0 = crate::RegValueT<Jints0_SPEC>;

impl Jints0 {
    #[doc = "This bit is set to 1 when this module completes compression process normally."]
    #[inline(always)]
    pub fn ins6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Jints0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Jints0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is set to 1 when a compressed data error occurs."]
    #[inline(always)]
    pub fn ins5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Jints0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jints0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is set to 1 when the image size and pixel format can be read. When an interrupt occurs, this module stops processing and the state is indicated by the JCRST register. To make this module resume processing, set the JPEG core process stop clear command bit (JRST) in JCCMD."]
    #[inline(always)]
    pub fn ins3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Jints0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Jints0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Jints0 {
    #[inline(always)]
    fn default() -> Jints0 {
        <crate::RegValueT<Jints0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcderr_SPEC;
impl crate::sealed::RegSpec for Jcderr_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Decode Error Register"]
pub type Jcderr = crate::RegValueT<Jcderr_SPEC>;

impl Jcderr {
    #[doc = "Error Code  (See tables )Identify the type of the error which has occurred in the compressed data analysis for decompression."]
    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        jcderr::Err,
        jcderr::Err,
        Jcderr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            jcderr::Err,
            jcderr::Err,
            Jcderr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcderr {
    #[inline(always)]
    fn default() -> Jcderr {
        <crate::RegValueT<Jcderr_SPEC> as RegisterValue<_>>::new(10)
    }
}
pub mod jcderr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err_SPEC;
    pub type Err = crate::EnumBitfieldStruct<u8, Err_SPEC>;
    impl Err {
        #[doc = "Normal(Decompression error codes)/Normal(Segment error codes)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "SOI not detected(Decompression error codes)"]
        pub const _0001: Self = Self::new(1);

        #[doc = "SOF1 to SOFF detected(Decompression error codes)"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Unprovided pixel format detected(Decompression error codes)"]
        pub const _0011: Self = Self::new(3);

        #[doc = "SOF accuracy error(Decompression error codes)"]
        pub const _0100: Self = Self::new(4);

        #[doc = "DQT accuracy error(Decompression error codes)"]
        pub const _0101: Self = Self::new(5);

        #[doc = "Component error 1(Decompression error codes)"]
        pub const _0110: Self = Self::new(6);

        #[doc = "Component error 2(Decompression error codes)"]
        pub const _0111: Self = Self::new(7);

        #[doc = "SOF0, DQT, and DHT not detected when SOS detected(Decompression error codes)"]
        pub const _1000: Self = Self::new(8);

        #[doc = "SOS not detected(Decompression error codes)"]
        pub const _1001: Self = Self::new(9);

        #[doc = "EOI not detected (default)(Decompression error codes)"]
        pub const _1010: Self = Self::new(10);

        #[doc = "Restart interval data number error detected(Decompression error codes)/Restart interval data number error(Segment error codes)"]
        pub const _1011: Self = Self::new(11);

        #[doc = "Image size error detected(Decompression error codes)/Image size error(Segment error codes)"]
        pub const _1100: Self = Self::new(12);

        #[doc = "Last MCU data number error detected(Decompression error codes)/Last MCU data number error(Segment error codes)"]
        pub const _1101: Self = Self::new(13);

        #[doc = "Block data number error detected(Decompression error codes)/Block data number error(Segment error codes)"]
        pub const _1110: Self = Self::new(14);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcrst_SPEC;
impl crate::sealed::RegSpec for Jcrst_SPEC {
    type DataType = u8;
}

#[doc = "JPEG Code Reset Register"]
pub type Jcrst = crate::RegValueT<Jcrst_SPEC>;

impl Jcrst {
    #[doc = "Operating State"]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jcrst::Rst,
        jcrst::Rst,
        Jcrst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jcrst::Rst,
            jcrst::Rst,
            Jcrst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcrst {
    #[inline(always)]
    fn default() -> Jcrst {
        <crate::RegValueT<Jcrst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jcrst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "State other than below"]
        pub const _0: Self = Self::new(0);

        #[doc = "Suspended state caused by interrupt sources of JINTE0"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifecnt_SPEC;
impl crate::sealed::RegSpec for Jifecnt_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Compression Control Register"]
pub type Jifecnt = crate::RegValueT<Jifecnt_SPEC>;

impl Jifecnt {
    #[doc = "Byte/Halfword/Word Swap Output coded data in compression is swapped."]
    #[inline(always)]
    pub fn joutswap(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        jifecnt::Joutswap,
        jifecnt::Joutswap,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            jifecnt::Joutswap,
            jifecnt::Joutswap,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
    #[inline(always)]
    pub fn dinrini(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jifecnt::Dinrini,
        jifecnt::Dinrini,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jifecnt::Dinrini,
            jifecnt::Dinrini,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input Image Data Lines Resume Command This bit is valid only when the count mode for stopping the input of image data lines is on. Setting this bit to 1 resumes reading input image data. This bit is always read as 0."]
    #[inline(always)]
    pub fn dinrcmd(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Jifecnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jifecnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Count Mode Setting for Stopping Input Image Data Lines"]
    #[inline(always)]
    pub fn dinlc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        jifecnt::Dinlc,
        jifecnt::Dinlc,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            jifecnt::Dinlc,
            jifecnt::Dinlc,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Byte/Halfword Swap"]
    #[inline(always)]
    pub fn dinswap(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        jifecnt::Dinswap,
        jifecnt::Dinswap,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            jifecnt::Dinswap,
            jifecnt::Dinswap,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jifecnt {
    #[inline(always)]
    fn default() -> Jifecnt {
        <crate::RegValueT<Jifecnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jifecnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Joutswap_SPEC;
    pub type Joutswap = crate::EnumBitfieldStruct<u8, Joutswap_SPEC>;
    impl Joutswap {
        #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
        pub const _000: Self = Self::new(0);

        #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
        pub const _001: Self = Self::new(1);

        #[doc = "(3) (4) (1) (2) (7) (8) (5) (6)  \\[Halfword swap\\]"]
        pub const _010: Self = Self::new(2);

        #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
        pub const _011: Self = Self::new(3);

        #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
        pub const _100: Self = Self::new(4);

        #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
        pub const _101: Self = Self::new(5);

        #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
        pub const _110: Self = Self::new(6);

        #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Word - byte swap\\]"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dinrini_SPEC;
    pub type Dinrini = crate::EnumBitfieldStruct<u8, Dinrini_SPEC>;
    impl Dinrini {
        #[doc = "The transfer address is not initialized when the input of image data lines is restarted"]
        pub const _0: Self = Self::new(0);

        #[doc = "The transfer address is initialized when the input of image data lines is restarted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dinlc_SPEC;
    pub type Dinlc = crate::EnumBitfieldStruct<u8, Dinlc_SPEC>;
    impl Dinlc {
        #[doc = "Count mode for stopping the input of image data lines is off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Count mode for stopping the input of image data lines is on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dinswap_SPEC;
    pub type Dinswap = crate::EnumBitfieldStruct<u8, Dinswap_SPEC>;
    impl Dinswap {
        #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
        pub const _000: Self = Self::new(0);

        #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
        pub const _001: Self = Self::new(1);

        #[doc = "(3) (4) (1) (2) (7) (8) (5) (6)  \\[Halfword swap\\]"]
        pub const _010: Self = Self::new(2);

        #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
        pub const _011: Self = Self::new(3);

        #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
        pub const _100: Self = Self::new(4);

        #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
        pub const _101: Self = Self::new(5);

        #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
        pub const _110: Self = Self::new(6);

        #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifesa_SPEC;
impl crate::sealed::RegSpec for Jifesa_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Compression Source Address Register"]
pub type Jifesa = crate::RegValueT<Jifesa_SPEC>;

impl Jifesa {
    #[doc = "Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn esa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifesa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifesa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifesa {
    #[inline(always)]
    fn default() -> Jifesa {
        <crate::RegValueT<Jifesa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifesofst_SPEC;
impl crate::sealed::RegSpec for Jifesofst_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Compression Line Offset Register"]
pub type Jifesofst = crate::RegValueT<Jifesofst_SPEC>;

impl Jifesofst {
    #[doc = "Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn esmw(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Jifesofst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Jifesofst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifesofst {
    #[inline(always)]
    fn default() -> Jifesofst {
        <crate::RegValueT<Jifesofst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifeda_SPEC;
impl crate::sealed::RegSpec for Jifeda_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Compression Destination Address Register"]
pub type Jifeda = crate::RegValueT<Jifeda_SPEC>;

impl Jifeda {
    #[doc = "Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn eda(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifeda_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifeda_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifeda {
    #[inline(always)]
    fn default() -> Jifeda {
        <crate::RegValueT<Jifeda_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifeslc_SPEC;
impl crate::sealed::RegSpec for Jifeslc_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Compression Source Line Count Register"]
pub type Jifeslc = crate::RegValueT<Jifeslc_SPEC>;

impl Jifeslc {
    #[doc = "Number of Input Image Data Lines to be Read (in 8-line units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn lines(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Jifeslc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Jifeslc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifeslc {
    #[inline(always)]
    fn default() -> Jifeslc {
        <crate::RegValueT<Jifeslc_SPEC> as RegisterValue<_>>::new(4294508536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdcnt_SPEC;
impl crate::sealed::RegSpec for Jifdcnt_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Decompression Control Register"]
pub type Jifdcnt = crate::RegValueT<Jifdcnt_SPEC>;

impl Jifdcnt {
    #[doc = "Vertical SubsamplingSubsamples vertical output image data."]
    #[inline(always)]
    pub fn vinter(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        jifdcnt::Vinter,
        jifdcnt::Vinter,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            jifdcnt::Vinter,
            jifdcnt::Vinter,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Horizontal Subsampling Subsamples horizontal output image data."]
    #[inline(always)]
    pub fn hinter(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        jifdcnt::Hinter,
        jifdcnt::Hinter,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            jifdcnt::Hinter,
            jifdcnt::Hinter,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Specifies output image data pixel format."]
    #[inline(always)]
    pub fn opf(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        jifdcnt::Opf,
        jifdcnt::Opf,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            jifdcnt::Opf,
            jifdcnt::Opf,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Initialization when Input Coded Data is Resumed This bit is only valid when the count mode for stopping the input of coded data is on. Set this bit before writing 1 to the data resume command bit."]
    #[inline(always)]
    pub fn jinrini(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        jifdcnt::Jinrini,
        jifdcnt::Jinrini,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            jifdcnt::Jinrini,
            jifdcnt::Jinrini,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input Coded Data Resume CommandThis bit is valid only when the count mode for stopping the input of coded data is on. Setting this bit to 1 resumes reading input coded data. This bit is always read as 0."]
    #[inline(always)]
    pub fn jinrcmd(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Jifdcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Jifdcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Count Mode Setting for Stopping Input Coded Data"]
    #[inline(always)]
    pub fn jinc(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        jifdcnt::Jinc,
        jifdcnt::Jinc,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            jifdcnt::Jinc,
            jifdcnt::Jinc,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Byte/Word/Longword Swap Input coded data in decompression is swapped."]
    #[inline(always)]
    pub fn jinswap(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        jifdcnt::Jinswap,
        jifdcnt::Jinswap,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            jifdcnt::Jinswap,
            jifdcnt::Jinswap,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Initialization when Resuming Output of Image Data Lines This bit is only valid when the count mode for stopping the output of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
    #[inline(always)]
    pub fn doutrini(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jifdcnt::Doutrini,
        jifdcnt::Doutrini,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jifdcnt::Doutrini,
            jifdcnt::Doutrini,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Output Image Data Lines Resume Command This bit is valid only when the count mode for stopping the output of image data lines is on. Setting this bit to 1 resumes writing image data. This bit is always read as 0."]
    #[inline(always)]
    pub fn doutrcmd(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Jifdcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jifdcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Count Mode for Stopping Output Image Data Lines"]
    #[inline(always)]
    pub fn doutlc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        jifdcnt::Doutlc,
        jifdcnt::Doutlc,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            jifdcnt::Doutlc,
            jifdcnt::Doutlc,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Byte/Word Swap Output image data in decompression is swapped."]
    #[inline(always)]
    pub fn doutswap(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        jifdcnt::Doutswap,
        jifdcnt::Doutswap,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            jifdcnt::Doutswap,
            jifdcnt::Doutswap,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jifdcnt {
    #[inline(always)]
    fn default() -> Jifdcnt {
        <crate::RegValueT<Jifdcnt_SPEC> as RegisterValue<_>>::new(16777216)
    }
}
pub mod jifdcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vinter_SPEC;
    pub type Vinter = crate::EnumBitfieldStruct<u8, Vinter_SPEC>;
    impl Vinter {
        #[doc = "No subsampling"]
        pub const _00: Self = Self::new(0);

        #[doc = "Subsamples output data into 1/2."]
        pub const _01: Self = Self::new(1);

        #[doc = "Subsamples output data into 1/4."]
        pub const _10: Self = Self::new(2);

        #[doc = "Subsamples output data into 1/8."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hinter_SPEC;
    pub type Hinter = crate::EnumBitfieldStruct<u8, Hinter_SPEC>;
    impl Hinter {
        #[doc = "No subsampling"]
        pub const _00: Self = Self::new(0);

        #[doc = "Subsamples output data into 1/2."]
        pub const _01: Self = Self::new(1);

        #[doc = "Subsamples output data into 1/4."]
        pub const _10: Self = Self::new(2);

        #[doc = "Subsamples output data into 1/8."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opf_SPEC;
    pub type Opf = crate::EnumBitfieldStruct<u8, Opf_SPEC>;
    impl Opf {
        #[doc = "ARGB8888"]
        pub const _01: Self = Self::new(1);

        #[doc = "RGB565"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinrini_SPEC;
    pub type Jinrini = crate::EnumBitfieldStruct<u8, Jinrini_SPEC>;
    impl Jinrini {
        #[doc = "The transfer address is not initialized when the input of coded data is restarted."]
        pub const _0: Self = Self::new(0);

        #[doc = "The transfer address is initialized when the input of coded data is restarted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinc_SPEC;
    pub type Jinc = crate::EnumBitfieldStruct<u8, Jinc_SPEC>;
    impl Jinc {
        #[doc = "Count mode for stopping the input of coded data is off."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count mode for stopping the input of coded data is on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinswap_SPEC;
    pub type Jinswap = crate::EnumBitfieldStruct<u8, Jinswap_SPEC>;
    impl Jinswap {
        #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
        pub const _000: Self = Self::new(0);

        #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
        pub const _001: Self = Self::new(1);

        #[doc = "(3) (4) (1) (2) (7) (8) (5) (6)  \\[Halfword swap\\]"]
        pub const _010: Self = Self::new(2);

        #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
        pub const _011: Self = Self::new(3);

        #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
        pub const _100: Self = Self::new(4);

        #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
        pub const _101: Self = Self::new(5);

        #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word -Halfword swap\\]"]
        pub const _110: Self = Self::new(6);

        #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Doutrini_SPEC;
    pub type Doutrini = crate::EnumBitfieldStruct<u8, Doutrini_SPEC>;
    impl Doutrini {
        #[doc = "The transfer address is not initialized when the output of lines of image data is restarted."]
        pub const _0: Self = Self::new(0);

        #[doc = "The transfer address is initialized when the output of lines of image data is restarted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Doutlc_SPEC;
    pub type Doutlc = crate::EnumBitfieldStruct<u8, Doutlc_SPEC>;
    impl Doutlc {
        #[doc = "Count mode for stopping the output of image data lines is off."]
        pub const _0: Self = Self::new(0);

        #[doc = "Count mode for stopping the output of image data lines is on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Doutswap_SPEC;
    pub type Doutswap = crate::EnumBitfieldStruct<u8, Doutswap_SPEC>;
    impl Doutswap {
        #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
        pub const _000: Self = Self::new(0);

        #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
        pub const _001: Self = Self::new(1);

        #[doc = "(3) (4) (1) (2) (7) (8) (5) (6)  \\[Halfword swap\\]"]
        pub const _010: Self = Self::new(2);

        #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
        pub const _011: Self = Self::new(3);

        #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
        pub const _100: Self = Self::new(4);

        #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
        pub const _101: Self = Self::new(5);

        #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
        pub const _110: Self = Self::new(6);

        #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdsa_SPEC;
impl crate::sealed::RegSpec for Jifdsa_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Decompression Source Address Register"]
pub type Jifdsa = crate::RegValueT<Jifdsa_SPEC>;

impl Jifdsa {
    #[doc = "Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn dsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifdsa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifdsa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdsa {
    #[inline(always)]
    fn default() -> Jifdsa {
        <crate::RegValueT<Jifdsa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifddofst_SPEC;
impl crate::sealed::RegSpec for Jifddofst_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Decompression Line Offset Register"]
pub type Jifddofst = crate::RegValueT<Jifddofst_SPEC>;

impl Jifddofst {
    #[doc = "Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn ddmw(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Jifddofst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Jifddofst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifddofst {
    #[inline(always)]
    fn default() -> Jifddofst {
        <crate::RegValueT<Jifddofst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdda_SPEC;
impl crate::sealed::RegSpec for Jifdda_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Decompression Destination Address Register"]
pub type Jifdda = crate::RegValueT<Jifdda_SPEC>;

impl Jifdda {
    #[doc = "Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn dda(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifdda_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifdda_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdda {
    #[inline(always)]
    fn default() -> Jifdda {
        <crate::RegValueT<Jifdda_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdsdc_SPEC;
impl crate::sealed::RegSpec for Jifdsdc_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Decompression Source Data Count Register"]
pub type Jifdsdc = crate::RegValueT<Jifdsdc_SPEC>;

impl Jifdsdc {
    #[doc = "Amount of Input Coded Data to be Read (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn jdatas(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Jifdsdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Jifdsdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdsdc {
    #[inline(always)]
    fn default() -> Jifdsdc {
        <crate::RegValueT<Jifdsdc_SPEC> as RegisterValue<_>>::new(4294508536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifddlc_SPEC;
impl crate::sealed::RegSpec for Jifddlc_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Decompression Destination Line Count Register"]
pub type Jifddlc = crate::RegValueT<Jifddlc_SPEC>;

impl Jifddlc {
    #[doc = "Number of Input Image Lines to Be ReadThe lower three bits should be set to 0. These bits are read as0.Number of input image data lines to be read, in 8-line units."]
    #[inline(always)]
    pub fn lines(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Jifddlc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Jifddlc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifddlc {
    #[inline(always)]
    fn default() -> Jifddlc {
        <crate::RegValueT<Jifddlc_SPEC> as RegisterValue<_>>::new(4294508536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdadt_SPEC;
impl crate::sealed::RegSpec for Jifdadt_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interface Decompression alpha Set Register"]
pub type Jifdadt = crate::RegValueT<Jifdadt_SPEC>;

impl Jifdadt {
    #[doc = "Setting of the alpha value for output in ARGB8888 format."]
    #[inline(always)]
    pub fn alpha(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jifdadt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jifdadt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdadt {
    #[inline(always)]
    fn default() -> Jifdadt {
        <crate::RegValueT<Jifdadt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jinte1_SPEC;
impl crate::sealed::RegSpec for Jinte1_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interrupt Enable Register 1"]
pub type Jinte1 = crate::RegValueT<Jinte1_SPEC>;

impl Jinte1 {
    #[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn cbten(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jinte1::Cbten,
        jinte1::Cbten,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jinte1::Cbten,
            jinte1::Cbten,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn dinlen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        jinte1::Dinlen,
        jinte1::Dinlen,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            jinte1::Dinlen,
            jinte1::Dinlen,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn dbten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        jinte1::Dbten,
        jinte1::Dbten,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            jinte1::Dbten,
            jinte1::Dbten,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1."]
    #[inline(always)]
    pub fn jinen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        jinte1::Jinen,
        jinte1::Jinen,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            jinte1::Jinen,
            jinte1::Jinen,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1"]
    #[inline(always)]
    pub fn doutlen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jinte1::Doutlen,
        jinte1::Doutlen,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jinte1::Doutlen,
            jinte1::Doutlen,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jinte1 {
    #[inline(always)]
    fn default() -> Jinte1 {
        <crate::RegValueT<Jinte1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jinte1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cbten_SPEC;
    pub type Cbten = crate::EnumBitfieldStruct<u8, Cbten_SPEC>;
    impl Cbten {
        #[doc = "Disables an interrupt request."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an interrupt request."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dinlen_SPEC;
    pub type Dinlen = crate::EnumBitfieldStruct<u8, Dinlen_SPEC>;
    impl Dinlen {
        #[doc = "Disables an interrupt request."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an interrupt request."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbten_SPEC;
    pub type Dbten = crate::EnumBitfieldStruct<u8, Dbten_SPEC>;
    impl Dbten {
        #[doc = "Disables an interrupt request."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an interrupt request."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinen_SPEC;
    pub type Jinen = crate::EnumBitfieldStruct<u8, Jinen_SPEC>;
    impl Jinen {
        #[doc = "Disables an interrupt request."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an interrupt request."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Doutlen_SPEC;
    pub type Doutlen = crate::EnumBitfieldStruct<u8, Doutlen_SPEC>;
    impl Doutlen {
        #[doc = "Disables an interrupt request."]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables an interrupt request."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jints1_SPEC;
impl crate::sealed::RegSpec for Jints1_SPEC {
    type DataType = u32;
}

#[doc = "JPEG Interrupt Status Register 1"]
pub type Jints1 = crate::RegValueT<Jints1_SPEC>;

impl Jints1 {
    #[doc = "This bit is set to 1 when the last output coded data is written in compression."]
    #[inline(always)]
    pub fn cbtf(self) -> crate::common::RegisterFieldBool<6, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is set to 1 when the number of input image data lines indicated by JIFESLC is read in compression.  This bit is valid only when the DINLC bit in JIFECNT is set to 1."]
    #[inline(always)]
    pub fn dinlf(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is set to 1 when the last output image data is written in decompression."]
    #[inline(always)]
    pub fn dbtf(self) -> crate::common::RegisterFieldBool<2, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "This bit is set to 1 when the amount of input coded data indicated by JIFDSDC is read in decompression.  This bit is valid only when the JINC bit in JIFDCNT is set to 1."]
    #[inline(always)]
    pub fn jinf(self) -> crate::common::RegisterFieldBool<1, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "In decompression, this bit is set to 1 when the number of lines of output image data indicated by JIFDDLC have been written. This bit is only valid when the DOUTLC bit in JIFDCNT is set to 1."]
    #[inline(always)]
    pub fn doutlf(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Jints1 {
    #[inline(always)]
    fn default() -> Jints1 {
        <crate::RegValueT<Jints1_SPEC> as RegisterValue<_>>::new(0)
    }
}
