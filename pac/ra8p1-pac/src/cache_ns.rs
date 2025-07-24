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
// Generated from SVD 1.00.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:55:39 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CACHE"]
unsafe impl ::core::marker::Send for super::CacheNs {}
unsafe impl ::core::marker::Sync for super::CacheNs {}
impl super::CacheNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "C-Cache Control Register"]
    #[inline(always)]
    pub const fn ccactl(
        &self,
    ) -> &'static crate::common::Reg<self::Ccactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "C-Cache Flush Control Register"]
    #[inline(always)]
    pub const fn ccafct(
        &self,
    ) -> &'static crate::common::Reg<self::Ccafct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccafct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "C-Cache Write Attribute"]
    #[inline(always)]
    pub const fn ccawta(
        &self,
    ) -> &'static crate::common::Reg<self::Ccawta_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccawta_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "C-Cache Error Detection Status"]
    #[inline(always)]
    pub const fn ccaedst(
        &self,
    ) -> &'static crate::common::Reg<self::Ccaedst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccaedst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "C-Cache Test Access Address"]
    #[inline(always)]
    pub const fn ccataa(
        &self,
    ) -> &'static crate::common::Reg<self::Ccataa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccataa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "C-Cache Test Access Data (DATA)"]
    #[inline(always)]
    pub const fn ccatad_data(
        &self,
    ) -> &'static crate::common::Reg<self::CcatadData_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CcatadData_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "C-Cache Test Access Data (ECC)"]
    #[inline(always)]
    pub const fn ccatad_ecc(
        &self,
    ) -> &'static crate::common::Reg<self::CcatadEcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CcatadEcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "C-Cache Test Access Data (LRU)"]
    #[inline(always)]
    pub const fn ccatad_lru(
        &self,
    ) -> &'static crate::common::Reg<self::CcatadLru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CcatadLru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "C-Cache Test Access Data (TAG)"]
    #[inline(always)]
    pub const fn ccatad_tag(
        &self,
    ) -> &'static crate::common::Reg<self::CcatadTag_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CcatadTag_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "C-Cache Test Access Data (TAGECC)"]
    #[inline(always)]
    pub const fn ccatad_tagecc(
        &self,
    ) -> &'static crate::common::Reg<self::CcatadTagecc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CcatadTagecc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "S-Cache Control Register"]
    #[inline(always)]
    pub const fn scactl(
        &self,
    ) -> &'static crate::common::Reg<self::Scactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "S-Cache Flush Control Register"]
    #[inline(always)]
    pub const fn scafct(
        &self,
    ) -> &'static crate::common::Reg<self::Scafct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scafct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "S-Cache Write Attribute"]
    #[inline(always)]
    pub const fn scawta(
        &self,
    ) -> &'static crate::common::Reg<self::Scawta_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scawta_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "S-Cache Error Detection Status"]
    #[inline(always)]
    pub const fn scaedst(
        &self,
    ) -> &'static crate::common::Reg<self::Scaedst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scaedst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "S-Cache Test Access Address"]
    #[inline(always)]
    pub const fn scataa(
        &self,
    ) -> &'static crate::common::Reg<self::Scataa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scataa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "S-Cache Test Access Data (DATA)"]
    #[inline(always)]
    pub const fn scatad_data(
        &self,
    ) -> &'static crate::common::Reg<self::ScatadData_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScatadData_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "S-Cache Test Access Data (ECC)"]
    #[inline(always)]
    pub const fn scatad_ecc(
        &self,
    ) -> &'static crate::common::Reg<self::ScatadEcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScatadEcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "S-Cache Test Access Data (LRU)"]
    #[inline(always)]
    pub const fn scatad_lru(
        &self,
    ) -> &'static crate::common::Reg<self::ScatadLru_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScatadLru_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "S-Cache Test Access Data (TAG)"]
    #[inline(always)]
    pub const fn scatad_tag(
        &self,
    ) -> &'static crate::common::Reg<self::ScatadTag_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScatadTag_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Cache Parity Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn capoad(
        &self,
    ) -> &'static crate::common::Reg<self::Capoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Capoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Cache Protection Register"]
    #[inline(always)]
    pub const fn caprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Caprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccactl_SPEC;
impl crate::sealed::RegSpec for Ccactl_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Control Register"]
pub type Ccactl = crate::RegValueT<Ccactl_SPEC>;

impl Ccactl {
    #[doc = "C-cache enable bit"]
    #[inline(always)]
    pub fn enc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccactl::Enc,
        ccactl::Enc,
        Ccactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccactl::Enc,
            ccactl::Enc,
            Ccactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-Cache flush bit"]
    #[inline(always)]
    pub fn fc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccactl::Fc,
        ccactl::Fc,
        Ccactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccactl::Fc,
            ccactl::Fc,
            Ccactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-cache write-back"]
    #[inline(always)]
    pub fn wb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ccactl::Wb,
        ccactl::Wb,
        Ccactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ccactl::Wb,
            ccactl::Wb,
            Ccactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccactl {
    #[inline(always)]
    fn default() -> Ccactl {
        <crate::RegValueT<Ccactl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccactl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enc_SPEC;
    pub type Enc = crate::EnumBitfieldStruct<u8, Enc_SPEC>;
    impl Enc {
        #[doc = "C-Cache disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-Cache enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fc_SPEC;
    pub type Fc = crate::EnumBitfieldStruct<u8, Fc_SPEC>;
    impl Fc {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-Cache line flush (all lines invalidate)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wb_SPEC;
    pub type Wb = crate::EnumBitfieldStruct<u8, Wb_SPEC>;
    impl Wb {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-Cache write-back"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccafct_SPEC;
impl crate::sealed::RegSpec for Ccafct_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Flush Control Register"]
pub type Ccafct = crate::RegValueT<Ccafct_SPEC>;

impl Ccafct {
    #[doc = "C-Cache flush bit"]
    #[inline(always)]
    pub fn fc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccafct::Fc,
        ccafct::Fc,
        Ccafct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccafct::Fc,
            ccafct::Fc,
            Ccafct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-Cache write-back"]
    #[inline(always)]
    pub fn wb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccafct::Wb,
        ccafct::Wb,
        Ccafct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccafct::Wb,
            ccafct::Wb,
            Ccafct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccafct {
    #[inline(always)]
    fn default() -> Ccafct {
        <crate::RegValueT<Ccafct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccafct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fc_SPEC;
    pub type Fc = crate::EnumBitfieldStruct<u8, Fc_SPEC>;
    impl Fc {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-Cache line flush (all lines invalidate)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wb_SPEC;
    pub type Wb = crate::EnumBitfieldStruct<u8, Wb_SPEC>;
    impl Wb {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "C-Cache write-back"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccawta_SPEC;
impl crate::sealed::RegSpec for Ccawta_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Write Attribute"]
pub type Ccawta = crate::RegValueT<Ccawta_SPEC>;

impl Ccawta {
    #[doc = "C-Cache write-through"]
    #[inline(always)]
    pub fn wt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccawta::Wt,
        ccawta::Wt,
        Ccawta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccawta::Wt,
            ccawta::Wt,
            Ccawta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-Cache write-allocation"]
    #[inline(always)]
    pub fn wa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccawta::Wa,
        ccawta::Wa,
        Ccawta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccawta::Wa,
            ccawta::Wa,
            Ccawta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccawta {
    #[inline(always)]
    fn default() -> Ccawta {
        <crate::RegValueT<Ccawta_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod ccawta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wt_SPEC;
    pub type Wt = crate::EnumBitfieldStruct<u8, Wt_SPEC>;
    impl Wt {
        #[doc = "Write-through or write-back depends on the cache attribute of the area"]
        pub const _0: Self = Self::new(0);

        #[doc = "Always write-through mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wa_SPEC;
    pub type Wa = crate::EnumBitfieldStruct<u8, Wa_SPEC>;
    impl Wa {
        #[doc = "Always disable write-allocation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write-allocation enable depends on the cache attribute of the area"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccaedst_SPEC;
impl crate::sealed::RegSpec for Ccaedst_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Error Detection Status"]
pub type Ccaedst = crate::RegValueT<Ccaedst_SPEC>;

impl Ccaedst {
    #[doc = "C-Cache data error status 0"]
    #[inline(always)]
    pub fn esd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccaedst::Esd0,
        ccaedst::Esd0,
        Ccaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccaedst::Esd0,
            ccaedst::Esd0,
            Ccaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-Cache data error status 1"]
    #[inline(always)]
    pub fn esd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccaedst::Esd1,
        ccaedst::Esd1,
        Ccaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccaedst::Esd1,
            ccaedst::Esd1,
            Ccaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-Cache tag clean line invalidate status"]
    #[inline(always)]
    pub fn estc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ccaedst::Estc,
        ccaedst::Estc,
        Ccaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ccaedst::Estc,
            ccaedst::Estc,
            Ccaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-Cache tag dirty line invalidate status"]
    #[inline(always)]
    pub fn estd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ccaedst::Estd,
        ccaedst::Estd,
        Ccaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ccaedst::Estd,
            ccaedst::Estd,
            Ccaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "C-Cache tag 2-bit error status"]
    #[inline(always)]
    pub fn est2(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccaedst::Est2,
        ccaedst::Est2,
        Ccaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccaedst::Est2,
            ccaedst::Est2,
            Ccaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccaedst {
    #[inline(always)]
    fn default() -> Ccaedst {
        <crate::RegValueT<Ccaedst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccaedst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esd0_SPEC;
    pub type Esd0 = crate::EnumBitfieldStruct<u8, Esd0_SPEC>;
    impl Esd0 {
        #[doc = "ECC 1-bit error was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC 1-bit error was detected and corrected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esd1_SPEC;
    pub type Esd1 = crate::EnumBitfieldStruct<u8, Esd1_SPEC>;
    impl Esd1 {
        #[doc = "ECC 2-bit error was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC 2-bit error was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Estc_SPEC;
    pub type Estc = crate::EnumBitfieldStruct<u8, Estc_SPEC>;
    impl Estc {
        #[doc = "Clean line was not invalidated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clean line was invalidated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Estd_SPEC;
    pub type Estd = crate::EnumBitfieldStruct<u8, Estd_SPEC>;
    impl Estd {
        #[doc = "Dirty line was not invalidated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Dirty line was invalidated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Est2_SPEC;
    pub type Est2 = crate::EnumBitfieldStruct<u8, Est2_SPEC>;
    impl Est2 {
        #[doc = "ECC 2-bit error was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC 2-bit error was detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccataa_SPEC;
impl crate::sealed::RegSpec for Ccataa_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Test Access Address"]
pub type Ccataa = crate::RegValueT<Ccataa_SPEC>;

impl Ccataa {
    #[doc = "Address offset"]
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, Ccataa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,Ccataa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Address entry"]
    #[inline(always)]
    pub fn entry(
        self,
    ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, u8, Ccataa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7f,1,0,u8,u8,Ccataa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Access target"]
    #[inline(always)]
    pub fn target(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        ccataa::Target,
        ccataa::Target,
        Ccataa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            ccataa::Target,
            ccataa::Target,
            Ccataa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read write"]
    #[inline(always)]
    pub fn rw(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        ccataa::Rw,
        ccataa::Rw,
        Ccataa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            ccataa::Rw,
            ccataa::Rw,
            Ccataa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address way"]
    #[inline(always)]
    pub fn way(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, Ccataa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,Ccataa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccataa {
    #[inline(always)]
    fn default() -> Ccataa {
        <crate::RegValueT<Ccataa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccataa {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Target_SPEC;
    pub type Target = crate::EnumBitfieldStruct<u8, Target_SPEC>;
    impl Target {
        #[doc = "Cache data read or write"]
        pub const _000: Self = Self::new(0);

        #[doc = "Data ECC code read or write"]
        pub const _001: Self = Self::new(1);

        #[doc = "Tag, V, D read or write"]
        pub const _010: Self = Self::new(2);

        #[doc = "LRU read or write"]
        pub const _011: Self = Self::new(3);

        #[doc = "Tag ECC code read or write"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rw_SPEC;
    pub type Rw = crate::EnumBitfieldStruct<u8, Rw_SPEC>;
    impl Rw {
        #[doc = "Read"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcatadData_SPEC;
impl crate::sealed::RegSpec for CcatadData_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Test Access Data (DATA)"]
pub type CcatadData = crate::RegValueT<CcatadData_SPEC>;

impl CcatadData {
    #[doc = "C-Cache test access data"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CcatadData_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CcatadData_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CcatadData {
    #[inline(always)]
    fn default() -> CcatadData {
        <crate::RegValueT<CcatadData_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcatadEcc_SPEC;
impl crate::sealed::RegSpec for CcatadEcc_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Test Access Data (ECC)"]
pub type CcatadEcc = crate::RegValueT<CcatadEcc_SPEC>;

impl CcatadEcc {
    #[doc = "C-Cache test access ECC code"]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, CcatadEcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,CcatadEcc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CcatadEcc {
    #[inline(always)]
    fn default() -> CcatadEcc {
        <crate::RegValueT<CcatadEcc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcatadLru_SPEC;
impl crate::sealed::RegSpec for CcatadLru_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Test Access Data (LRU)"]
pub type CcatadLru = crate::RegValueT<CcatadLru_SPEC>;

impl CcatadLru {
    #[doc = "C-Cache test access LRU data"]
    #[inline(always)]
    pub fn lru(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, CcatadLru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,CcatadLru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CcatadLru {
    #[inline(always)]
    fn default() -> CcatadLru {
        <crate::RegValueT<CcatadLru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcatadTag_SPEC;
impl crate::sealed::RegSpec for CcatadTag_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Test Access Data (TAG)"]
pub type CcatadTag = crate::RegValueT<CcatadTag_SPEC>;

impl CcatadTag {
    #[doc = "C-Cache test access Dirty"]
    #[inline(always)]
    pub fn d(self) -> crate::common::RegisterFieldBool<0, 1, 0, CcatadTag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CcatadTag_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "C-Cache test access Valid"]
    #[inline(always)]
    pub fn v(self) -> crate::common::RegisterFieldBool<1, 1, 0, CcatadTag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CcatadTag_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "C-Cache test access Tag data"]
    #[inline(always)]
    pub fn tag(
        self,
    ) -> crate::common::RegisterField<12, 0xfffff, 1, 0, u32, u32, CcatadTag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xfffff,1,0,u32,u32,CcatadTag_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CcatadTag {
    #[inline(always)]
    fn default() -> CcatadTag {
        <crate::RegValueT<CcatadTag_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcatadTagecc_SPEC;
impl crate::sealed::RegSpec for CcatadTagecc_SPEC {
    type DataType = u32;
}

#[doc = "C-Cache Test Access Data (TAGECC)"]
pub type CcatadTagecc = crate::RegValueT<CcatadTagecc_SPEC>;

impl CcatadTagecc {
    #[doc = "C-Cache test access Tag ECC code"]
    #[inline(always)]
    pub fn tagecc(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, CcatadTagecc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,CcatadTagecc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CcatadTagecc {
    #[inline(always)]
    fn default() -> CcatadTagecc {
        <crate::RegValueT<CcatadTagecc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scactl_SPEC;
impl crate::sealed::RegSpec for Scactl_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Control Register"]
pub type Scactl = crate::RegValueT<Scactl_SPEC>;

impl Scactl {
    #[doc = "S-Cache enable bit"]
    #[inline(always)]
    pub fn ens(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scactl::Ens,
        scactl::Ens,
        Scactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scactl::Ens,
            scactl::Ens,
            Scactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache flush bit"]
    #[inline(always)]
    pub fn fs(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        scactl::Fs,
        scactl::Fs,
        Scactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            scactl::Fs,
            scactl::Fs,
            Scactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache write-back"]
    #[inline(always)]
    pub fn wb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        scactl::Wb,
        scactl::Wb,
        Scactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            scactl::Wb,
            scactl::Wb,
            Scactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scactl {
    #[inline(always)]
    fn default() -> Scactl {
        <crate::RegValueT<Scactl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scactl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ens_SPEC;
    pub type Ens = crate::EnumBitfieldStruct<u8, Ens_SPEC>;
    impl Ens {
        #[doc = "S-Cache disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-Cache enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fs_SPEC;
    pub type Fs = crate::EnumBitfieldStruct<u8, Fs_SPEC>;
    impl Fs {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-Cache line flush (all lines invalidate)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wb_SPEC;
    pub type Wb = crate::EnumBitfieldStruct<u8, Wb_SPEC>;
    impl Wb {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-Cache write-back"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scafct_SPEC;
impl crate::sealed::RegSpec for Scafct_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Flush Control Register"]
pub type Scafct = crate::RegValueT<Scafct_SPEC>;

impl Scafct {
    #[doc = "S-Cache flush bit"]
    #[inline(always)]
    pub fn fs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scafct::Fs,
        scafct::Fs,
        Scafct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scafct::Fs,
            scafct::Fs,
            Scafct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache write-back"]
    #[inline(always)]
    pub fn wb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        scafct::Wb,
        scafct::Wb,
        Scafct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            scafct::Wb,
            scafct::Wb,
            Scafct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scafct {
    #[inline(always)]
    fn default() -> Scafct {
        <crate::RegValueT<Scafct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scafct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fs_SPEC;
    pub type Fs = crate::EnumBitfieldStruct<u8, Fs_SPEC>;
    impl Fs {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-Cache line flush (all lines invalidate)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wb_SPEC;
    pub type Wb = crate::EnumBitfieldStruct<u8, Wb_SPEC>;
    impl Wb {
        #[doc = "No action"]
        pub const _0: Self = Self::new(0);

        #[doc = "S-Cache write-back"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scawta_SPEC;
impl crate::sealed::RegSpec for Scawta_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Write Attribute"]
pub type Scawta = crate::RegValueT<Scawta_SPEC>;

impl Scawta {
    #[doc = "S-Cache write-through"]
    #[inline(always)]
    pub fn wt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scawta::Wt,
        scawta::Wt,
        Scawta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scawta::Wt,
            scawta::Wt,
            Scawta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache write-allocation"]
    #[inline(always)]
    pub fn wa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        scawta::Wa,
        scawta::Wa,
        Scawta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            scawta::Wa,
            scawta::Wa,
            Scawta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scawta {
    #[inline(always)]
    fn default() -> Scawta {
        <crate::RegValueT<Scawta_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod scawta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wt_SPEC;
    pub type Wt = crate::EnumBitfieldStruct<u8, Wt_SPEC>;
    impl Wt {
        #[doc = "Write-through or write-back depends on the cache attribute of the area"]
        pub const _0: Self = Self::new(0);

        #[doc = "Always write-through mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wa_SPEC;
    pub type Wa = crate::EnumBitfieldStruct<u8, Wa_SPEC>;
    impl Wa {
        #[doc = "Always disable write-allocation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write-allocation enable depends on the cache attribute of the area"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scaedst_SPEC;
impl crate::sealed::RegSpec for Scaedst_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Error Detection Status"]
pub type Scaedst = crate::RegValueT<Scaedst_SPEC>;

impl Scaedst {
    #[doc = "S-Cache data error status 0"]
    #[inline(always)]
    pub fn esd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scaedst::Esd0,
        scaedst::Esd0,
        Scaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scaedst::Esd0,
            scaedst::Esd0,
            Scaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache data error status 1"]
    #[inline(always)]
    pub fn esd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        scaedst::Esd1,
        scaedst::Esd1,
        Scaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            scaedst::Esd1,
            scaedst::Esd1,
            Scaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache Tag clean line invalidate status"]
    #[inline(always)]
    pub fn estc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        scaedst::Estc,
        scaedst::Estc,
        Scaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            scaedst::Estc,
            scaedst::Estc,
            Scaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache Tag dirty line invalidate status"]
    #[inline(always)]
    pub fn estd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        scaedst::Estd,
        scaedst::Estd,
        Scaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            scaedst::Estd,
            scaedst::Estd,
            Scaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "S-Cache Tag 2-bit error status"]
    #[inline(always)]
    pub fn est2(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scaedst::Est2,
        scaedst::Est2,
        Scaedst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scaedst::Est2,
            scaedst::Est2,
            Scaedst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scaedst {
    #[inline(always)]
    fn default() -> Scaedst {
        <crate::RegValueT<Scaedst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scaedst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esd0_SPEC;
    pub type Esd0 = crate::EnumBitfieldStruct<u8, Esd0_SPEC>;
    impl Esd0 {
        #[doc = "ECC 1-bit error was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC 1-bit error was detected, and corrected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esd1_SPEC;
    pub type Esd1 = crate::EnumBitfieldStruct<u8, Esd1_SPEC>;
    impl Esd1 {
        #[doc = "ECC 2-bit error was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC 2-bit error was detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Estc_SPEC;
    pub type Estc = crate::EnumBitfieldStruct<u8, Estc_SPEC>;
    impl Estc {
        #[doc = "Clean line was not invalidated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clean line was invalidated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Estd_SPEC;
    pub type Estd = crate::EnumBitfieldStruct<u8, Estd_SPEC>;
    impl Estd {
        #[doc = "Dirty line was not invalidated"]
        pub const _0: Self = Self::new(0);

        #[doc = "Dirty line was invalidated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Est2_SPEC;
    pub type Est2 = crate::EnumBitfieldStruct<u8, Est2_SPEC>;
    impl Est2 {
        #[doc = "ECC 2-bit error was not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC 2-bit error was detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scataa_SPEC;
impl crate::sealed::RegSpec for Scataa_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Test Access Address"]
pub type Scataa = crate::RegValueT<Scataa_SPEC>;

impl Scataa {
    #[doc = "Address offset"]
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, Scataa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,Scataa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Address entry"]
    #[inline(always)]
    pub fn entry(
        self,
    ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, u8, Scataa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7f,1,0,u8,u8,Scataa_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Access target"]
    #[inline(always)]
    pub fn target(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        scataa::Target,
        scataa::Target,
        Scataa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            scataa::Target,
            scataa::Target,
            Scataa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read write"]
    #[inline(always)]
    pub fn rw(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        scataa::Rw,
        scataa::Rw,
        Scataa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            scataa::Rw,
            scataa::Rw,
            Scataa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address way"]
    #[inline(always)]
    pub fn way(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, u8, Scataa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8,u8,Scataa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scataa {
    #[inline(always)]
    fn default() -> Scataa {
        <crate::RegValueT<Scataa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scataa {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Target_SPEC;
    pub type Target = crate::EnumBitfieldStruct<u8, Target_SPEC>;
    impl Target {
        #[doc = "Cache data read or write"]
        pub const _000: Self = Self::new(0);

        #[doc = "Data ECC code read or write"]
        pub const _001: Self = Self::new(1);

        #[doc = "Tag, V, D read or write"]
        pub const _010: Self = Self::new(2);

        #[doc = "LRU read or write"]
        pub const _011: Self = Self::new(3);

        #[doc = "Tag ECC code read or write"]
        pub const _100: Self = Self::new(4);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rw_SPEC;
    pub type Rw = crate::EnumBitfieldStruct<u8, Rw_SPEC>;
    impl Rw {
        #[doc = "read"]
        pub const _0: Self = Self::new(0);

        #[doc = "write"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScatadData_SPEC;
impl crate::sealed::RegSpec for ScatadData_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Test Access Data (DATA)"]
pub type ScatadData = crate::RegValueT<ScatadData_SPEC>;

impl ScatadData {
    #[doc = "S-Cache test access data"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ScatadData_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ScatadData_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ScatadData {
    #[inline(always)]
    fn default() -> ScatadData {
        <crate::RegValueT<ScatadData_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScatadEcc_SPEC;
impl crate::sealed::RegSpec for ScatadEcc_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Test Access Data (ECC)"]
pub type ScatadEcc = crate::RegValueT<ScatadEcc_SPEC>;

impl ScatadEcc {
    #[doc = "S-Cache test access ECC code"]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, ScatadEcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,ScatadEcc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ScatadEcc {
    #[inline(always)]
    fn default() -> ScatadEcc {
        <crate::RegValueT<ScatadEcc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScatadLru_SPEC;
impl crate::sealed::RegSpec for ScatadLru_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Test Access Data (LRU)"]
pub type ScatadLru = crate::RegValueT<ScatadLru_SPEC>;

impl ScatadLru {
    #[doc = "S-Cache test access LRU data"]
    #[inline(always)]
    pub fn lru(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, ScatadLru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,ScatadLru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ScatadLru {
    #[inline(always)]
    fn default() -> ScatadLru {
        <crate::RegValueT<ScatadLru_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScatadTag_SPEC;
impl crate::sealed::RegSpec for ScatadTag_SPEC {
    type DataType = u32;
}

#[doc = "S-Cache Test Access Data (TAG)"]
pub type ScatadTag = crate::RegValueT<ScatadTag_SPEC>;

impl ScatadTag {
    #[doc = "S-Cache test access Dirty"]
    #[inline(always)]
    pub fn d(self) -> crate::common::RegisterFieldBool<0, 1, 0, ScatadTag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ScatadTag_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "S-Cache test access Valid"]
    #[inline(always)]
    pub fn v(self) -> crate::common::RegisterFieldBool<1, 1, 0, ScatadTag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ScatadTag_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "S-Cache test access Tag data"]
    #[inline(always)]
    pub fn tag(
        self,
    ) -> crate::common::RegisterField<12, 0xfffff, 1, 0, u32, u32, ScatadTag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xfffff,1,0,u32,u32,ScatadTag_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ScatadTag {
    #[inline(always)]
    fn default() -> ScatadTag {
        <crate::RegValueT<ScatadTag_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capoad_SPEC;
impl crate::sealed::RegSpec for Capoad_SPEC {
    type DataType = u32;
}

#[doc = "Cache Parity Error Operation After Detection Register"]
pub type Capoad = crate::RegValueT<Capoad_SPEC>;

impl Capoad {
    #[doc = "Operation after detection bit"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        capoad::Oad,
        capoad::Oad,
        Capoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            capoad::Oad,
            capoad::Oad,
            Capoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC enable"]
    #[inline(always)]
    pub fn eccmod1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        capoad::Eccmod1,
        capoad::Eccmod1,
        Capoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            capoad::Eccmod1,
            capoad::Eccmod1,
            Capoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC correctable error information update enable"]
    #[inline(always)]
    pub fn e1stsen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        capoad::E1Stsen,
        capoad::E1Stsen,
        Capoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            capoad::E1Stsen,
            capoad::E1Stsen,
            Capoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Capoad {
    #[inline(always)]
    fn default() -> Capoad {
        <crate::RegValueT<Capoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod capoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Interrupt"]
        pub const _0: Self = Self::new(0);

        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccmod1_SPEC;
    pub type Eccmod1 = crate::EnumBitfieldStruct<u8, Eccmod1_SPEC>;
    impl Eccmod1 {
        #[doc = "Disable ECC"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable ECC"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct E1Stsen_SPEC;
    pub type E1Stsen = crate::EnumBitfieldStruct<u8, E1Stsen_SPEC>;
    impl E1Stsen {
        #[doc = "Disable updating of correctable ECC error information"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable updating of correctable ECC error information"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caprcr_SPEC;
impl crate::sealed::RegSpec for Caprcr_SPEC {
    type DataType = u32;
}

#[doc = "Cache Protection Register"]
pub type Caprcr = crate::RegValueT<Caprcr_SPEC>;

impl Caprcr {
    #[doc = "Register Write Control bit"]
    #[inline(always)]
    pub fn prcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        caprcr::Prcr,
        caprcr::Prcr,
        Caprcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            caprcr::Prcr,
            caprcr::Prcr,
            Caprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Write Key Code bit"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Caprcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Caprcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Caprcr {
    #[inline(always)]
    fn default() -> Caprcr {
        <crate::RegValueT<Caprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod caprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prcr_SPEC;
    pub type Prcr = crate::EnumBitfieldStruct<u8, Prcr_SPEC>;
    impl Prcr {
        #[doc = "Writing to the protected register is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the protected register is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
