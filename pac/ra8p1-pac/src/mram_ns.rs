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
#[doc = r"MS_NS System Register area"]
unsafe impl ::core::marker::Send for super::MramNs {}
unsafe impl ::core::marker::Sync for super::MramNs {}
impl super::MramNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Code MRAM Prefetch Buffer Enable Register"]
    #[inline(always)]
    pub const fn mrcpfb(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcpfb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcpfb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Code MRAM Frequency Notifications Register"]
    #[inline(always)]
    pub const fn mrcfreq(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcfreq_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcfreq_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Extra MRAM Frequency Notifications Register"]
    #[inline(always)]
    pub const fn mrefreq(
        &self,
    ) -> &'static crate::common::Reg<self::Mrefreq_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrefreq_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Code MRAM ECC Decoder Control Register"]
    #[inline(always)]
    pub const fn mrcdecc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcdecc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcdecc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Code MRAM Read Access Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mrcraeint(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcraeint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcraeint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Code MRAM Read Access Error Status Register"]
    #[inline(always)]
    pub const fn mrcraes(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcraes_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcraes_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Code MRAM TED Error Address Register"]
    #[inline(always)]
    pub const fn mrcrtea(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcrtea_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrcrtea_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Code MRAM DEC Error Address Register"]
    #[inline(always)]
    pub const fn mrcrdea(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcrdea_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrcrdea_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Extra MRAM Read Access Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mreraeint(
        &self,
    ) -> &'static crate::common::Reg<self::Mreraeint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mreraeint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Extra MRAM Read Access Error Status Register"]
    #[inline(always)]
    pub const fn mreraes(
        &self,
    ) -> &'static crate::common::Reg<self::Mreraes_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mreraes_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Extra MRAM TED Error Address Register"]
    #[inline(always)]
    pub const fn mrertea(
        &self,
    ) -> &'static crate::common::Reg<self::Mrertea_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrertea_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Extra MRAM DEC Error Address Register"]
    #[inline(always)]
    pub const fn mrerdea(
        &self,
    ) -> &'static crate::common::Reg<self::Mrerdea_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrerdea_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "MRAM Security Attribution Register"]
    #[inline(always)]
    pub const fn msar(&self) -> &'static crate::common::Reg<self::Msar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Extra MRAM Zeroization Status Register"]
    #[inline(always)]
    pub const fn mrezs(&self) -> &'static crate::common::Reg<self::Mrezs_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrezs_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "Extra MRAM Zeroization Control Register"]
    #[inline(always)]
    pub const fn mrezc(&self) -> &'static crate::common::Reg<self::Mrezc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrezc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "Extra MRAM Access Status Register"]
    #[inline(always)]
    pub const fn mastat(
        &self,
    ) -> &'static crate::common::Reg<self::Mastat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mastat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8208usize),
            )
        }
    }

    #[doc = "Extra MRAM Access Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mpaeint(
        &self,
    ) -> &'static crate::common::Reg<self::Mpaeint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpaeint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8212usize),
            )
        }
    }

    #[doc = "Extra MRAM Ready Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mrdyie(
        &self,
    ) -> &'static crate::common::Reg<self::Mrdyie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrdyie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8216usize),
            )
        }
    }

    #[doc = "MACI Command Start Address Register"]
    #[inline(always)]
    pub const fn msaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Msaddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msaddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8240usize),
            )
        }
    }

    #[doc = "MRAM Counter Select Register"]
    #[inline(always)]
    pub const fn mcntselr(
        &self,
    ) -> &'static crate::common::Reg<self::Mcntselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mcntselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8264usize),
            )
        }
    }

    #[doc = "MRAM Counter Data Register (n = 0, 1)"]
    #[inline(always)]
    pub const fn mcntdtr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mcntdtr_SPEC, crate::common::R>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204cusize))
        }
    }
    #[inline(always)]
    pub const fn mcntdtr0(
        &self,
    ) -> &'static crate::common::Reg<self::Mcntdtr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mcntdtr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x204cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn mcntdtr1(
        &self,
    ) -> &'static crate::common::Reg<self::Mcntdtr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mcntdtr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x2050usize),
            )
        }
    }

    #[doc = "MRAM Configuration Update Transfer Control Register"]
    #[inline(always)]
    pub const fn mctrcntr(
        &self,
    ) -> &'static crate::common::Reg<self::Mctrcntr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mctrcntr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8288usize),
            )
        }
    }

    #[doc = "MRAM Configuration Update Transfer List Select Register"]
    #[inline(always)]
    pub const fn mctrlsr(
        &self,
    ) -> &'static crate::common::Reg<self::Mctrlsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mctrlsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8292usize),
            )
        }
    }

    #[doc = "MRAM Configuration Update Transfer Status Register"]
    #[inline(always)]
    pub const fn mctrstatr(
        &self,
    ) -> &'static crate::common::Reg<self::Mctrstatr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mctrstatr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8300usize),
            )
        }
    }

    #[doc = "Extra MRAM Status Register"]
    #[inline(always)]
    pub const fn mstatr(&self) -> &'static crate::common::Reg<self::Mstatr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mstatr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8320usize),
            )
        }
    }

    #[doc = "Extra MRAM Program Mode Entry Register"]
    #[inline(always)]
    pub const fn mentryr(
        &self,
    ) -> &'static crate::common::Reg<self::Mentryr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mentryr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8324usize),
            )
        }
    }

    #[doc = "Extra MRAM Sequencer Setup Initialization Register"]
    #[inline(always)]
    pub const fn msuinitr(
        &self,
    ) -> &'static crate::common::Reg<self::Msuinitr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msuinitr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8332usize),
            )
        }
    }

    #[doc = "MACI Command Register"]
    #[inline(always)]
    pub const fn mcmdr(&self) -> &'static crate::common::Reg<self::Mcmdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mcmdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8352usize),
            )
        }
    }

    #[doc = "MRAM Startup Area Select Monitor Register"]
    #[inline(always)]
    pub const fn msuasmon(
        &self,
    ) -> &'static crate::common::Reg<self::Msuasmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Msuasmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8412usize),
            )
        }
    }

    #[doc = "MRAM Startup Area Control Register"]
    #[inline(always)]
    pub const fn msuacr(
        &self,
    ) -> &'static crate::common::Reg<self::Msuacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msuacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8424usize),
            )
        }
    }

    #[doc = "MRAM Program Speed Control Register"]
    #[inline(always)]
    pub const fn mrpsc(&self) -> &'static crate::common::Reg<self::Mrpsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrpsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10240usize),
            )
        }
    }

    #[doc = "Code MRAM Program Control Register"]
    #[inline(always)]
    pub const fn mrcpc0(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcpc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcpc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12288usize),
            )
        }
    }

    #[doc = "Code MRAM Program Control Register for Secure"]
    #[inline(always)]
    pub const fn mrcpc1(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcpc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcpc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12292usize),
            )
        }
    }

    #[doc = "Code MRAM Block Protection Register"]
    #[inline(always)]
    pub const fn mrcbprot0(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcbprot0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcbprot0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12296usize),
            )
        }
    }

    #[doc = "Code MRAM Block Protection Register for Secure"]
    #[inline(always)]
    pub const fn mrcbprot1(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcbprot1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcbprot1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12300usize),
            )
        }
    }

    #[doc = "Code MRAM Program Status Register"]
    #[inline(always)]
    pub const fn mrcps(&self) -> &'static crate::common::Reg<self::Mrcps_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcps_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12304usize),
            )
        }
    }

    #[doc = "Code MRAM Program Access Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mrcpaeint(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcpaeint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcpaeint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12308usize),
            )
        }
    }

    #[doc = "Code MRAM Program Error Address Register"]
    #[inline(always)]
    pub const fn mrcpea(&self) -> &'static crate::common::Reg<self::Mrcpea_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrcpea_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12312usize),
            )
        }
    }

    #[doc = "Code MRAM Flush Register"]
    #[inline(always)]
    pub const fn mrcflr(
        &self,
    ) -> &'static crate::common::Reg<self::Mrcflr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrcflr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12336usize),
            )
        }
    }

    #[doc = "Code MRAM ECC Encoder Control Register"]
    #[inline(always)]
    pub const fn mrceecc(
        &self,
    ) -> &'static crate::common::Reg<self::Mrceecc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mrceecc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14340usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcpfb_SPEC;
impl crate::sealed::RegSpec for Mrcpfb_SPEC {
    type DataType = u8;
}

#[doc = "Code MRAM Prefetch Buffer Enable Register"]
pub type Mrcpfb = crate::RegValueT<Mrcpfb_SPEC>;

impl Mrcpfb {
    #[doc = "Code MRAM Prefetch Buffer Enable"]
    #[inline(always)]
    pub fn mpfben(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcpfb::Mpfben,
        mrcpfb::Mpfben,
        Mrcpfb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcpfb::Mpfben,
            mrcpfb::Mpfben,
            Mrcpfb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrcpfb {
    #[inline(always)]
    fn default() -> Mrcpfb {
        <crate::RegValueT<Mrcpfb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcpfb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpfben_SPEC;
    pub type Mpfben = crate::EnumBitfieldStruct<u8, Mpfben_SPEC>;
    impl Mpfben {
        #[doc = "Prefetch buffer disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Prefetch buffer enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcfreq_SPEC;
impl crate::sealed::RegSpec for Mrcfreq_SPEC {
    type DataType = u32;
}

#[doc = "Code MRAM Frequency Notifications Register"]
pub type Mrcfreq = crate::RegValueT<Mrcfreq_SPEC>;

impl Mrcfreq {
    #[doc = "Code MRAM Frequency Notifications"]
    #[inline(always)]
    pub fn mrcmhz(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Mrcfreq_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Mrcfreq_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Mrcfreq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Mrcfreq_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcfreq {
    #[inline(always)]
    fn default() -> Mrcfreq {
        <crate::RegValueT<Mrcfreq_SPEC> as RegisterValue<_>>::new(10)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrefreq_SPEC;
impl crate::sealed::RegSpec for Mrefreq_SPEC {
    type DataType = u32;
}

#[doc = "Extra MRAM Frequency Notifications Register"]
pub type Mrefreq = crate::RegValueT<Mrefreq_SPEC>;

impl Mrefreq {
    #[doc = "Extra MRAM Frequency Notifications"]
    #[inline(always)]
    pub fn mremhz(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mrefreq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mrefreq_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Mrefreq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Mrefreq_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrefreq {
    #[inline(always)]
    fn default() -> Mrefreq {
        <crate::RegValueT<Mrefreq_SPEC> as RegisterValue<_>>::new(10)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcdecc_SPEC;
impl crate::sealed::RegSpec for Mrcdecc_SPEC {
    type DataType = u16;
}

#[doc = "Code MRAM ECC Decoder Control Register"]
pub type Mrcdecc = crate::RegValueT<Mrcdecc_SPEC>;

impl Mrcdecc {
    #[doc = "Code MRAM ECC Decoder Disable"]
    #[inline(always)]
    pub fn decdisc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcdecc::Decdisc,
        mrcdecc::Decdisc,
        Mrcdecc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcdecc::Decdisc,
            mrcdecc::Decdisc,
            Mrcdecc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code MRAM ECC Bits Select"]
    #[inline(always)]
    pub fn eccselc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mrcdecc::Eccselc,
        mrcdecc::Eccselc,
        Mrcdecc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mrcdecc::Eccselc,
            mrcdecc::Eccselc,
            Mrcdecc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrcdecc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrcdecc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcdecc {
    #[inline(always)]
    fn default() -> Mrcdecc {
        <crate::RegValueT<Mrcdecc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcdecc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Decdisc_SPEC;
    pub type Decdisc = crate::EnumBitfieldStruct<u8, Decdisc_SPEC>;
    impl Decdisc {
        #[doc = "ECC decoder enable"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC decoder disable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccselc_SPEC;
    pub type Eccselc = crate::EnumBitfieldStruct<u8, Eccselc_SPEC>;
    impl Eccselc {
        #[doc = "ECC bits unselect"]
        pub const _0: Self = Self::new(0);

        #[doc = "ECC bits select"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcraeint_SPEC;
impl crate::sealed::RegSpec for Mrcraeint_SPEC {
    type DataType = u8;
}

#[doc = "Code MRAM Read Access Error Interrupt Enable Register"]
pub type Mrcraeint = crate::RegValueT<Mrcraeint_SPEC>;

impl Mrcraeint {
    #[doc = "Code MRAM DEC Error Interrupt Enable"]
    #[inline(always)]
    pub fn intenbdc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcraeint::Intenbdc,
        mrcraeint::Intenbdc,
        Mrcraeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcraeint::Intenbdc,
            mrcraeint::Intenbdc,
            Mrcraeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code MRAM TED Error Interrupt Enable"]
    #[inline(always)]
    pub fn intenbtc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mrcraeint::Intenbtc,
        mrcraeint::Intenbtc,
        Mrcraeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mrcraeint::Intenbtc,
            mrcraeint::Intenbtc,
            Mrcraeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrcraeint {
    #[inline(always)]
    fn default() -> Mrcraeint {
        <crate::RegValueT<Mrcraeint_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod mrcraeint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intenbdc_SPEC;
    pub type Intenbdc = crate::EnumBitfieldStruct<u8, Intenbdc_SPEC>;
    impl Intenbdc {
        #[doc = "Generation of an MRAM_MRCRD interrupt request is disabled when MRCRAES.DECERRC is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_MRCRD interrupt request is enabled when MRCRAES.DECERRC is set to 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intenbtc_SPEC;
    pub type Intenbtc = crate::EnumBitfieldStruct<u8, Intenbtc_SPEC>;
    impl Intenbtc {
        #[doc = "Generation of an MRAM_MRCRD interrupt request is disabled when MRCRAES.TEDERRC is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_MRCRD interrupt request is enabled when MRCRAES.TEDERRC is set to 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcraes_SPEC;
impl crate::sealed::RegSpec for Mrcraes_SPEC {
    type DataType = u8;
}

#[doc = "Code MRAM Read Access Error Status Register"]
pub type Mrcraes = crate::RegValueT<Mrcraes_SPEC>;

impl Mrcraes {
    #[doc = "Code MRAM DEC Error Detected"]
    #[inline(always)]
    pub fn decerrc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcraes::Decerrc,
        mrcraes::Decerrc,
        Mrcraes_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcraes::Decerrc,
            mrcraes::Decerrc,
            Mrcraes_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code MRAM TED Error Detected"]
    #[inline(always)]
    pub fn tederrc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mrcraes::Tederrc,
        mrcraes::Tederrc,
        Mrcraes_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mrcraes::Tederrc,
            mrcraes::Tederrc,
            Mrcraes_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrcraes {
    #[inline(always)]
    fn default() -> Mrcraes {
        <crate::RegValueT<Mrcraes_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcraes {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Decerrc_SPEC;
    pub type Decerrc = crate::EnumBitfieldStruct<u8, Decerrc_SPEC>;
    impl Decerrc {
        #[doc = "DEC error undetected"]
        pub const _0: Self = Self::new(0);

        #[doc = "DEC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tederrc_SPEC;
    pub type Tederrc = crate::EnumBitfieldStruct<u8, Tederrc_SPEC>;
    impl Tederrc {
        #[doc = "TED error undetected"]
        pub const _0: Self = Self::new(0);

        #[doc = "TED error detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcrtea_SPEC;
impl crate::sealed::RegSpec for Mrcrtea_SPEC {
    type DataType = u32;
}

#[doc = "Code MRAM TED Error Address Register"]
pub type Mrcrtea = crate::RegValueT<Mrcrtea_SPEC>;

impl Mrcrtea {
    #[doc = "Code MRAM Read Access TED Error Address"]
    #[inline(always)]
    pub fn mrcrtea(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Mrcrtea_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,Mrcrtea_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcrtea {
    #[inline(always)]
    fn default() -> Mrcrtea {
        <crate::RegValueT<Mrcrtea_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcrdea_SPEC;
impl crate::sealed::RegSpec for Mrcrdea_SPEC {
    type DataType = u32;
}

#[doc = "Code MRAM DEC Error Address Register"]
pub type Mrcrdea = crate::RegValueT<Mrcrdea_SPEC>;

impl Mrcrdea {
    #[doc = "Code MRAM Read Access DEC Error Address"]
    #[inline(always)]
    pub fn mrcrdea(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Mrcrdea_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,Mrcrdea_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcrdea {
    #[inline(always)]
    fn default() -> Mrcrdea {
        <crate::RegValueT<Mrcrdea_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mreraeint_SPEC;
impl crate::sealed::RegSpec for Mreraeint_SPEC {
    type DataType = u8;
}

#[doc = "Extra MRAM Read Access Error Interrupt Enable Register"]
pub type Mreraeint = crate::RegValueT<Mreraeint_SPEC>;

impl Mreraeint {
    #[doc = "Code MRAM DEC Error Interrupt Enable"]
    #[inline(always)]
    pub fn intenbde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mreraeint::Intenbde,
        mreraeint::Intenbde,
        Mreraeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mreraeint::Intenbde,
            mreraeint::Intenbde,
            Mreraeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code MRAM TED Error Interrupt Enable"]
    #[inline(always)]
    pub fn intenbte(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mreraeint::Intenbte,
        mreraeint::Intenbte,
        Mreraeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mreraeint::Intenbte,
            mreraeint::Intenbte,
            Mreraeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mreraeint {
    #[inline(always)]
    fn default() -> Mreraeint {
        <crate::RegValueT<Mreraeint_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod mreraeint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intenbde_SPEC;
    pub type Intenbde = crate::EnumBitfieldStruct<u8, Intenbde_SPEC>;
    impl Intenbde {
        #[doc = "Generation of an MRAM_MRERD interrupt request is disabled when MERAES.DECERRE is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_MRERD interrupt request is enabled when MERAES.DECERRE is set to 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intenbte_SPEC;
    pub type Intenbte = crate::EnumBitfieldStruct<u8, Intenbte_SPEC>;
    impl Intenbte {
        #[doc = "Generation of an MRAM_MRERD interrupt request is disabled when MERAES.TEDERRE is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_MRERD interrupt request is enabled when MERAES.TEDERRE is set to 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mreraes_SPEC;
impl crate::sealed::RegSpec for Mreraes_SPEC {
    type DataType = u8;
}

#[doc = "Extra MRAM Read Access Error Status Register"]
pub type Mreraes = crate::RegValueT<Mreraes_SPEC>;

impl Mreraes {
    #[doc = "Extra MRAM DEC Error Detected"]
    #[inline(always)]
    pub fn decerre(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mreraes::Decerre,
        mreraes::Decerre,
        Mreraes_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mreraes::Decerre,
            mreraes::Decerre,
            Mreraes_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Extra MRAM TED Error Detected"]
    #[inline(always)]
    pub fn tederre(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mreraes::Tederre,
        mreraes::Tederre,
        Mreraes_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mreraes::Tederre,
            mreraes::Tederre,
            Mreraes_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mreraes {
    #[inline(always)]
    fn default() -> Mreraes {
        <crate::RegValueT<Mreraes_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mreraes {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Decerre_SPEC;
    pub type Decerre = crate::EnumBitfieldStruct<u8, Decerre_SPEC>;
    impl Decerre {
        #[doc = "DEC error undetected"]
        pub const _0: Self = Self::new(0);

        #[doc = "DEC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tederre_SPEC;
    pub type Tederre = crate::EnumBitfieldStruct<u8, Tederre_SPEC>;
    impl Tederre {
        #[doc = "TED error undetected"]
        pub const _0: Self = Self::new(0);

        #[doc = "TED error detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrertea_SPEC;
impl crate::sealed::RegSpec for Mrertea_SPEC {
    type DataType = u32;
}

#[doc = "Extra MRAM TED Error Address Register"]
pub type Mrertea = crate::RegValueT<Mrertea_SPEC>;

impl Mrertea {
    #[doc = "Extra MRAM Read Access TED Error Address"]
    #[inline(always)]
    pub fn mrertea(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Mrertea_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xfffffff,1,0,u32,u32,Mrertea_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrertea {
    #[inline(always)]
    fn default() -> Mrertea {
        <crate::RegValueT<Mrertea_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrerdea_SPEC;
impl crate::sealed::RegSpec for Mrerdea_SPEC {
    type DataType = u32;
}

#[doc = "Extra MRAM DEC Error Address Register"]
pub type Mrerdea = crate::RegValueT<Mrerdea_SPEC>;

impl Mrerdea {
    #[doc = "Extra MRAM Read Access DEC Error Address"]
    #[inline(always)]
    pub fn mrerdea(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Mrerdea_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xfffffff,1,0,u32,u32,Mrerdea_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrerdea {
    #[inline(always)]
    fn default() -> Mrerdea {
        <crate::RegValueT<Mrerdea_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msar_SPEC;
impl crate::sealed::RegSpec for Msar_SPEC {
    type DataType = u16;
}

#[doc = "MRAM Security Attribution Register"]
pub type Msar = crate::RegValueT<Msar_SPEC>;

impl Msar {
    #[doc = "Extra MRAM ECC Register Security Attribution"]
    #[inline(always)]
    pub fn mreeccsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        msar::Mreeccsa,
        msar::Mreeccsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            msar::Mreeccsa,
            msar::Mreeccsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MREFREQ Register Security Attribution"]
    #[inline(always)]
    pub fn mrefreqsa(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        msar::Mrefreqsa,
        msar::Mrefreqsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            msar::Mrefreqsa,
            msar::Mrefreqsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code MRAM ECC Register Security Attribution"]
    #[inline(always)]
    pub fn mrceccsa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        msar::Mrceccsa,
        msar::Mrceccsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            msar::Mrceccsa,
            msar::Mrceccsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MRCFREQ Register Security Attribution"]
    #[inline(always)]
    pub fn mrcfreqsa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        msar::Mrcfreqsa,
        msar::Mrcfreqsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            msar::Mrcfreqsa,
            msar::Mrcfreqsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MRCPFB Register Security Attribution"]
    #[inline(always)]
    pub fn mpfbensa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        msar::Mpfbensa,
        msar::Mpfbensa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            msar::Mpfbensa,
            msar::Mpfbensa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MACI command Issuing Security Attribution"]
    #[inline(always)]
    pub fn macicmisa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        msar::Macicmisa,
        msar::Macicmisa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            msar::Macicmisa,
            msar::Macicmisa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MACI Command Registers Security Attribution"]
    #[inline(always)]
    pub fn macicmrsa(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        msar::Macicmrsa,
        msar::Macicmrsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            msar::Macicmrsa,
            msar::Macicmrsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "MACI Transfer Security Attribution"]
    #[inline(always)]
    pub fn macitrsa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        msar::Macitrsa,
        msar::Macitrsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            msar::Macitrsa,
            msar::Macitrsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Code MRAM Program Register Security Attribution"]
    #[inline(always)]
    pub fn mrcpsa(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        msar::Mrcpsa,
        msar::Mrcpsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            msar::Mrcpsa,
            msar::Mrcpsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EPSEQ Area Register Security Attribution"]
    #[inline(always)]
    pub fn mrepseqsa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        msar::Mrepseqsa,
        msar::Mrepseqsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            msar::Mrepseqsa,
            msar::Mrepseqsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPSEQ Area Register Security Attribution"]
    #[inline(always)]
    pub fn mrcpseqsa(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        msar::Mrcpseqsa,
        msar::Mrcpseqsa,
        Msar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            msar::Mrcpseqsa,
            msar::Mrcpseqsa,
            Msar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Msar {
    #[inline(always)]
    fn default() -> Msar {
        <crate::RegValueT<Msar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mreeccsa_SPEC;
    pub type Mreeccsa = crate::EnumBitfieldStruct<u8, Mreeccsa_SPEC>;
    impl Mreeccsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrefreqsa_SPEC;
    pub type Mrefreqsa = crate::EnumBitfieldStruct<u8, Mrefreqsa_SPEC>;
    impl Mrefreqsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrceccsa_SPEC;
    pub type Mrceccsa = crate::EnumBitfieldStruct<u8, Mrceccsa_SPEC>;
    impl Mrceccsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcfreqsa_SPEC;
    pub type Mrcfreqsa = crate::EnumBitfieldStruct<u8, Mrcfreqsa_SPEC>;
    impl Mrcfreqsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpfbensa_SPEC;
    pub type Mpfbensa = crate::EnumBitfieldStruct<u8, Mpfbensa_SPEC>;
    impl Mpfbensa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macicmisa_SPEC;
    pub type Macicmisa = crate::EnumBitfieldStruct<u8, Macicmisa_SPEC>;
    impl Macicmisa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macicmrsa_SPEC;
    pub type Macicmrsa = crate::EnumBitfieldStruct<u8, Macicmrsa_SPEC>;
    impl Macicmrsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macitrsa_SPEC;
    pub type Macitrsa = crate::EnumBitfieldStruct<u8, Macitrsa_SPEC>;
    impl Macitrsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcpsa_SPEC;
    pub type Mrcpsa = crate::EnumBitfieldStruct<u8, Mrcpsa_SPEC>;
    impl Mrcpsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrepseqsa_SPEC;
    pub type Mrepseqsa = crate::EnumBitfieldStruct<u8, Mrepseqsa_SPEC>;
    impl Mrepseqsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcpseqsa_SPEC;
    pub type Mrcpseqsa = crate::EnumBitfieldStruct<u8, Mrcpseqsa_SPEC>;
    impl Mrcpseqsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-Secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrezs_SPEC;
impl crate::sealed::RegSpec for Mrezs_SPEC {
    type DataType = u8;
}

#[doc = "Extra MRAM Zeroization Status Register"]
pub type Mrezs = crate::RegValueT<Mrezs_SPEC>;

impl Mrezs {
    #[doc = "W-HUK Zero Flag Status"]
    #[inline(always)]
    pub fn whukzf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrezs::Whukzf,
        mrezs::Whukzf,
        Mrezs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrezs::Whukzf,
            mrezs::Whukzf,
            Mrezs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "W-HUK Zeroization Executing Status"]
    #[inline(always)]
    pub fn whukexe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mrezs::Whukexe,
        mrezs::Whukexe,
        Mrezs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mrezs::Whukexe,
            mrezs::Whukexe,
            Mrezs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrezs {
    #[inline(always)]
    fn default() -> Mrezs {
        <crate::RegValueT<Mrezs_SPEC> as RegisterValue<_>>::new(252)
    }
}
pub mod mrezs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Whukzf_SPEC;
    pub type Whukzf = crate::EnumBitfieldStruct<u8, Whukzf_SPEC>;
    impl Whukzf {
        #[doc = "W-HUK is zeroized"]
        pub const _0: Self = Self::new(0);

        #[doc = "W-HUK is not zeroized"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Whukexe_SPEC;
    pub type Whukexe = crate::EnumBitfieldStruct<u8, Whukexe_SPEC>;
    impl Whukexe {
        #[doc = "W-HUK zeroization is not executing"]
        pub const _0: Self = Self::new(0);

        #[doc = "W-HUK zeroization is executing"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrezc_SPEC;
impl crate::sealed::RegSpec for Mrezc_SPEC {
    type DataType = u16;
}

#[doc = "Extra MRAM Zeroization Control Register"]
pub type Mrezc = crate::RegValueT<Mrezc_SPEC>;

impl Mrezc {
    #[doc = "W-HUK Zeroization Execute"]
    #[inline(always)]
    pub fn whukze(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        mrezc::Whukze,
        mrezc::Whukze,
        Mrezc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            mrezc::Whukze,
            mrezc::Whukze,
            Mrezc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrezc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrezc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrezc {
    #[inline(always)]
    fn default() -> Mrezc {
        <crate::RegValueT<Mrezc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrezc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Whukze_SPEC;
    pub type Whukze = crate::EnumBitfieldStruct<u8, Whukze_SPEC>;
    impl Whukze {
        #[doc = "W-HUK zeroization not executes"]
        pub const XX_0: Self = Self::new(0);

        #[doc = "W-HUK zeroization executes"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mastat_SPEC;
impl crate::sealed::RegSpec for Mastat_SPEC {
    type DataType = u8;
}

#[doc = "Extra MRAM Access Status Register"]
pub type Mastat = crate::RegValueT<Mastat_SPEC>;

impl Mastat {
    #[doc = "Extra MRAM Access Violation Flag"]
    #[inline(always)]
    pub fn mreae(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mastat::Mreae,
        mastat::Mreae,
        Mastat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mastat::Mreae,
            mastat::Mreae,
            Mastat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Lock Flag"]
    #[inline(always)]
    pub fn cmdlk(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mastat::Cmdlk,
        mastat::Cmdlk,
        Mastat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mastat::Cmdlk,
            mastat::Cmdlk,
            Mastat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mastat {
    #[inline(always)]
    fn default() -> Mastat {
        <crate::RegValueT<Mastat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mastat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mreae_SPEC;
    pub type Mreae = crate::EnumBitfieldStruct<u8, Mreae_SPEC>;
    impl Mreae {
        #[doc = "No extra MRAM access violation has occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "An extra MRAM access violation has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlk_SPEC;
    pub type Cmdlk = crate::EnumBitfieldStruct<u8, Cmdlk_SPEC>;
    impl Cmdlk {
        #[doc = "The MRAM sequencer is not in the command-locked state"]
        pub const _0: Self = Self::new(0);

        #[doc = "The MRAM sequencer is in the command-locked state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpaeint_SPEC;
impl crate::sealed::RegSpec for Mpaeint_SPEC {
    type DataType = u8;
}

#[doc = "Extra MRAM Access Error Interrupt Enable Register"]
pub type Mpaeint = crate::RegValueT<Mpaeint_SPEC>;

impl Mpaeint {
    #[doc = "Extra MRAM Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn mreaeie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mpaeint::Mreaeie,
        mpaeint::Mreaeie,
        Mpaeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mpaeint::Mreaeie,
            mpaeint::Mreaeie,
            Mpaeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command Lock Interrupt Enable"]
    #[inline(always)]
    pub fn cmdlkie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mpaeint::Cmdlkie,
        mpaeint::Cmdlkie,
        Mpaeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mpaeint::Cmdlkie,
            mpaeint::Cmdlkie,
            Mpaeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mpaeint {
    #[inline(always)]
    fn default() -> Mpaeint {
        <crate::RegValueT<Mpaeint_SPEC> as RegisterValue<_>>::new(24)
    }
}
pub mod mpaeint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mreaeie_SPEC;
    pub type Mreaeie = crate::EnumBitfieldStruct<u8, Mreaeie_SPEC>;
    impl Mreaeie {
        #[doc = "Generation of an MRAM_MREPR interrupt request is disabled when MASTAT.MREAE is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_MREPR interrupt request is enabled when MASTAT.MREAE is set to 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlkie_SPEC;
    pub type Cmdlkie = crate::EnumBitfieldStruct<u8, Cmdlkie_SPEC>;
    impl Cmdlkie {
        #[doc = "Generation of an MRAM_MREPR interrupt request is disabled when MASTAT.CMDLK is set to 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_MREPR interrupt request is enabled when MASTAT.CMDLK is set to 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdyie_SPEC;
impl crate::sealed::RegSpec for Mrdyie_SPEC {
    type DataType = u8;
}

#[doc = "Extra MRAM Ready Interrupt Enable Register"]
pub type Mrdyie = crate::RegValueT<Mrdyie_SPEC>;

impl Mrdyie {
    #[doc = "Extra MRAM Ready Interrupt Enable"]
    #[inline(always)]
    pub fn mrdyie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrdyie::Mrdyie,
        mrdyie::Mrdyie,
        Mrdyie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrdyie::Mrdyie,
            mrdyie::Mrdyie,
            Mrdyie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrdyie {
    #[inline(always)]
    fn default() -> Mrdyie {
        <crate::RegValueT<Mrdyie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrdyie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrdyie_SPEC;
    pub type Mrdyie = crate::EnumBitfieldStruct<u8, Mrdyie_SPEC>;
    impl Mrdyie {
        #[doc = "Generation of an MRAM_ENDOFPE interrupt request is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_ENDOFPE interrupt request is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msaddr_SPEC;
impl crate::sealed::RegSpec for Msaddr_SPEC {
    type DataType = u32;
}

#[doc = "MACI Command Start Address Register"]
pub type Msaddr = crate::RegValueT<Msaddr_SPEC>;

impl Msaddr {
    #[doc = "Start Address for MACI Command Processing"]
    #[inline(always)]
    pub fn msaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Msaddr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Msaddr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Msaddr {
    #[inline(always)]
    fn default() -> Msaddr {
        <crate::RegValueT<Msaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcntselr_SPEC;
impl crate::sealed::RegSpec for Mcntselr_SPEC {
    type DataType = u8;
}

#[doc = "MRAM Counter Select Register"]
pub type Mcntselr = crate::RegValueT<Mcntselr_SPEC>;

impl Mcntselr {
    #[doc = "Counter select"]
    #[inline(always)]
    pub fn cntsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        mcntselr::Cntsel,
        mcntselr::Cntsel,
        Mcntselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            mcntselr::Cntsel,
            mcntselr::Cntsel,
            Mcntselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mcntselr {
    #[inline(always)]
    fn default() -> Mcntselr {
        <crate::RegValueT<Mcntselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcntselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntsel_SPEC;
    pub type Cntsel = crate::EnumBitfieldStruct<u8, Cntsel_SPEC>;
    impl Cntsel {
        #[doc = "Anti-rollback counter is not selected."]
        pub const _000: Self = Self::new(0);

        #[doc = "ARC_SEC is selected. Counter size is 64 bits."]
        pub const _001: Self = Self::new(1);

        #[doc = "ARC_OEMBL is selected. Counter size is 64 bits."]
        pub const _010: Self = Self::new(2);

        #[doc = "Anti-rollback counter is not selected."]
        pub const _011: Self = Self::new(3);

        #[doc = "ARC_NSEC is selected. Counter configuration is 64 or 256 bits. See , , and for details of the counter select condition."]
        pub const _1_XX: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcntdtr_SPEC;
impl crate::sealed::RegSpec for Mcntdtr_SPEC {
    type DataType = u32;
}

#[doc = "MRAM Counter Data Register (n = 0, 1)"]
pub type Mcntdtr = crate::RegValueT<Mcntdtr_SPEC>;

impl Mcntdtr {
    #[doc = "Counter Read Data"]
    #[inline(always)]
    pub fn cntrdat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mcntdtr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mcntdtr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mcntdtr {
    #[inline(always)]
    fn default() -> Mcntdtr {
        <crate::RegValueT<Mcntdtr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrcntr_SPEC;
impl crate::sealed::RegSpec for Mctrcntr_SPEC {
    type DataType = u16;
}

#[doc = "MRAM Configuration Update Transfer Control Register"]
pub type Mctrcntr = crate::RegValueT<Mctrcntr_SPEC>;

impl Mctrcntr {
    #[doc = "Transfer Start Trigger"]
    #[inline(always)]
    pub fn trtrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mctrcntr::Trtrg,
        mctrcntr::Trtrg,
        Mctrcntr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mctrcntr::Trtrg,
            mctrcntr::Trtrg,
            Mctrcntr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mctrcntr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mctrcntr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mctrcntr {
    #[inline(always)]
    fn default() -> Mctrcntr {
        <crate::RegValueT<Mctrcntr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mctrcntr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trtrg_SPEC;
    pub type Trtrg = crate::EnumBitfieldStruct<u8, Trtrg_SPEC>;
    impl Trtrg {
        #[doc = "Configuration update transfer processing does not start"]
        pub const _0: Self = Self::new(0);

        #[doc = "Configuration update transfer processing starts"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrlsr_SPEC;
impl crate::sealed::RegSpec for Mctrlsr_SPEC {
    type DataType = u8;
}

#[doc = "MRAM Configuration Update Transfer List Select Register"]
pub type Mctrlsr = crate::RegValueT<Mctrlsr_SPEC>;

impl Mctrlsr {
    #[doc = "Configuration Update Transfer List"]
    #[inline(always)]
    pub fn trlist(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        mctrlsr::Trlist,
        mctrlsr::Trlist,
        Mctrlsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            mctrlsr::Trlist,
            mctrlsr::Trlist,
            Mctrlsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mctrlsr {
    #[inline(always)]
    fn default() -> Mctrlsr {
        <crate::RegValueT<Mctrlsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mctrlsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trlist_SPEC;
    pub type Trlist = crate::EnumBitfieldStruct<u8, Trlist_SPEC>;
    impl Trlist {
        #[doc = "Transfer list 1 is selected"]
        pub const _001: Self = Self::new(1);

        #[doc = "Transfer list 1 is not selected"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrstatr_SPEC;
impl crate::sealed::RegSpec for Mctrstatr_SPEC {
    type DataType = u8;
}

#[doc = "MRAM Configuration Update Transfer Status Register"]
pub type Mctrstatr = crate::RegValueT<Mctrstatr_SPEC>;

impl Mctrstatr {
    #[doc = "Transfer Busy Status"]
    #[inline(always)]
    pub fn trbusy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mctrstatr::Trbusy,
        mctrstatr::Trbusy,
        Mctrstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mctrstatr::Trbusy,
            mctrstatr::Trbusy,
            Mctrstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Mode Setting Status"]
    #[inline(always)]
    pub fn trmd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mctrstatr::Trmd,
        mctrstatr::Trmd,
        Mctrstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mctrstatr::Trmd,
            mctrstatr::Trmd,
            Mctrstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mctrstatr {
    #[inline(always)]
    fn default() -> Mctrstatr {
        <crate::RegValueT<Mctrstatr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mctrstatr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trbusy_SPEC;
    pub type Trbusy = crate::EnumBitfieldStruct<u8, Trbusy_SPEC>;
    impl Trbusy {
        #[doc = "Configuration update transfer is not processing"]
        pub const _0: Self = Self::new(0);

        #[doc = "Configuration update transfer is processing"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmd_SPEC;
    pub type Trmd = crate::EnumBitfieldStruct<u8, Trmd_SPEC>;
    impl Trmd {
        #[doc = "MENTRYR register is a data transferable setting"]
        pub const _0: Self = Self::new(0);

        #[doc = "MENTRYR register is not a data transferable setting"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstatr_SPEC;
impl crate::sealed::RegSpec for Mstatr_SPEC {
    type DataType = u32;
}

#[doc = "Extra MRAM Status Register"]
pub type Mstatr = crate::RegValueT<Mstatr_SPEC>;

impl Mstatr {
    #[doc = "Configuration Set Error Flag"]
    #[inline(always)]
    pub fn cfgseterr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mstatr::Cfgseterr,
        mstatr::Cfgseterr,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mstatr::Cfgseterr,
            mstatr::Cfgseterr,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Programming Error Flag"]
    #[inline(always)]
    pub fn prgerr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mstatr::Prgerr,
        mstatr::Prgerr,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mstatr::Prgerr,
            mstatr::Prgerr,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Command Error Flag"]
    #[inline(always)]
    pub fn ilglerr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mstatr::Ilglerr,
        mstatr::Ilglerr,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mstatr::Ilglerr,
            mstatr::Ilglerr,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Extra MRAM Ready Flag"]
    #[inline(always)]
    pub fn mrdy(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mstatr::Mrdy,
        mstatr::Mrdy,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mstatr::Mrdy,
            mstatr::Mrdy,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "TrustZone Filter Error"]
    #[inline(always)]
    pub fn tzferr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mstatr::Tzferr,
        mstatr::Tzferr,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mstatr::Tzferr,
            mstatr::Tzferr,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Other Error"]
    #[inline(always)]
    pub fn oterr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mstatr::Oterr,
        mstatr::Oterr,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mstatr::Oterr,
            mstatr::Oterr,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Security Error"]
    #[inline(always)]
    pub fn secerr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mstatr::Secerr,
        mstatr::Secerr,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mstatr::Secerr,
            mstatr::Secerr,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Illegal Command Error"]
    #[inline(always)]
    pub fn ilgcomerr(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mstatr::Ilgcomerr,
        mstatr::Ilgcomerr,
        Mstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mstatr::Ilgcomerr,
            mstatr::Ilgcomerr,
            Mstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstatr {
    #[inline(always)]
    fn default() -> Mstatr {
        <crate::RegValueT<Mstatr_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod mstatr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfgseterr_SPEC;
    pub type Cfgseterr = crate::EnumBitfieldStruct<u8, Cfgseterr_SPEC>;
    impl Cfgseterr {
        #[doc = "Configuration set has been completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during configuration set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr_SPEC;
    pub type Prgerr = crate::EnumBitfieldStruct<u8, Prgerr_SPEC>;
    impl Prgerr {
        #[doc = "Programming has been completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during programming"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilglerr_SPEC;
    pub type Ilglerr = crate::EnumBitfieldStruct<u8, Ilglerr_SPEC>;
    impl Ilglerr {
        #[doc = "The extra MRAM sequencer has not detected an illegal MACI command or illegal extra MRAM access"]
        pub const _0: Self = Self::new(0);

        #[doc = "The extra MRAM sequencer has detected an illegal MACI command or illegal extra MRAM access"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrdy_SPEC;
    pub type Mrdy = crate::EnumBitfieldStruct<u8, Mrdy_SPEC>;
    impl Mrdy {
        #[doc = "Program, Configuration set, Increment Counter, or Read Counter command processing is in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "None of the above is in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tzferr_SPEC;
    pub type Tzferr = crate::EnumBitfieldStruct<u8, Tzferr_SPEC>;
    impl Tzferr {
        #[doc = "A TrustZone filter error has not been detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A TrustZone filter error has been detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oterr_SPEC;
    pub type Oterr = crate::EnumBitfieldStruct<u8, Oterr_SPEC>;
    impl Oterr {
        #[doc = "An error has not been detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has been detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secerr_SPEC;
    pub type Secerr = crate::EnumBitfieldStruct<u8, Secerr_SPEC>;
    impl Secerr {
        #[doc = "A write protection error against MSUASMON.FSPR bit has not been detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "A write protection error against MSUASMON.FSPR bit has been detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilgcomerr_SPEC;
    pub type Ilgcomerr = crate::EnumBitfieldStruct<u8, Ilgcomerr_SPEC>;
    impl Ilgcomerr {
        #[doc = "An illegal MACI command error has not been detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "An illegal MACI command error has been detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mentryr_SPEC;
impl crate::sealed::RegSpec for Mentryr_SPEC {
    type DataType = u16;
}

#[doc = "Extra MRAM Program Mode Entry Register"]
pub type Mentryr = crate::RegValueT<Mentryr_SPEC>;

impl Mentryr {
    #[doc = "Extra MRAM Program Mode Entry"]
    #[inline(always)]
    pub fn mentry(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mentryr::Mentry,
        mentryr::Mentry,
        Mentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mentryr::Mentry,
            mentryr::Mentry,
            Mentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mentryr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mentryr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mentryr {
    #[inline(always)]
    fn default() -> Mentryr {
        <crate::RegValueT<Mentryr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mentryr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mentry_SPEC;
    pub type Mentry = crate::EnumBitfieldStruct<u8, Mentry_SPEC>;
    impl Mentry {
        #[doc = "Extra MRAM is in read mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Extra MRAM is in program mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msuinitr_SPEC;
impl crate::sealed::RegSpec for Msuinitr_SPEC {
    type DataType = u16;
}

#[doc = "Extra MRAM Sequencer Setup Initialization Register"]
pub type Msuinitr = crate::RegValueT<Msuinitr_SPEC>;

impl Msuinitr {
    #[doc = "Setup Initialization"]
    #[inline(always)]
    pub fn suinit(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        msuinitr::Suinit,
        msuinitr::Suinit,
        Msuinitr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            msuinitr::Suinit,
            msuinitr::Suinit,
            Msuinitr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Msuinitr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Msuinitr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Msuinitr {
    #[inline(always)]
    fn default() -> Msuinitr {
        <crate::RegValueT<Msuinitr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msuinitr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suinit_SPEC;
    pub type Suinit = crate::EnumBitfieldStruct<u8, Suinit_SPEC>;
    impl Suinit {
        #[doc = "The MSADDR, MENTRYR, and MCNTSELR extra MRAM sequencer setup registers keep their current values"]
        pub const _0: Self = Self::new(0);

        #[doc = "The MSADDR, MENTRYR, and MCNTSELR extra MRAM sequencer setup registers are initialized"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcmdr_SPEC;
impl crate::sealed::RegSpec for Mcmdr_SPEC {
    type DataType = u16;
}

#[doc = "MACI Command Register"]
pub type Mcmdr = crate::RegValueT<Mcmdr_SPEC>;

impl Mcmdr {
    #[doc = "Pre-command Flag"]
    #[inline(always)]
    pub fn pcmdr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mcmdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mcmdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Flag"]
    #[inline(always)]
    pub fn cmdr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mcmdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mcmdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mcmdr {
    #[inline(always)]
    fn default() -> Mcmdr {
        <crate::RegValueT<Mcmdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msuasmon_SPEC;
impl crate::sealed::RegSpec for Msuasmon_SPEC {
    type DataType = u32;
}

#[doc = "MRAM Startup Area Select Monitor Register"]
pub type Msuasmon = crate::RegValueT<Msuasmon_SPEC>;

impl Msuasmon {
    #[doc = "Protection flag of programming to set boot flag and startup area control"]
    #[inline(always)]
    pub fn fspr(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        msuasmon::Fspr,
        msuasmon::Fspr,
        Msuasmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            msuasmon::Fspr,
            msuasmon::Fspr,
            Msuasmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Size of startup area select for boot swap"]
    #[inline(always)]
    pub fn btsize(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        msuasmon::Btsize,
        msuasmon::Btsize,
        Msuasmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            msuasmon::Btsize,
            msuasmon::Btsize,
            Msuasmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Flag of startup area select for boot swap"]
    #[inline(always)]
    pub fn btflg(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        msuasmon::Btflg,
        msuasmon::Btflg,
        Msuasmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            msuasmon::Btflg,
            msuasmon::Btflg,
            Msuasmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Msuasmon {
    #[inline(always)]
    fn default() -> Msuasmon {
        <crate::RegValueT<Msuasmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msuasmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fspr_SPEC;
    pub type Fspr = crate::EnumBitfieldStruct<u8, Fspr_SPEC>;
    impl Fspr {
        #[doc = "Protected state"]
        pub const _0: Self = Self::new(0);

        #[doc = "Non-protected state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btsize_SPEC;
    pub type Btsize = crate::EnumBitfieldStruct<u8, Btsize_SPEC>;
    impl Btsize {
        #[doc = "Reserved"]
        pub const _00: Self = Self::new(0);

        #[doc = "Startup area size is set to 32 KB"]
        pub const _01: Self = Self::new(1);

        #[doc = "Startup area size is set to 16 KB"]
        pub const _10: Self = Self::new(2);

        #[doc = "Startup area size is set to 8 KB"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btflg_SPEC;
    pub type Btflg = crate::EnumBitfieldStruct<u8, Btflg_SPEC>;
    impl Btflg {
        #[doc = "The startup area is the alternate block (block 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The startup area is the default block (block 0)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msuacr_SPEC;
impl crate::sealed::RegSpec for Msuacr_SPEC {
    type DataType = u16;
}

#[doc = "MRAM Startup Area Control Register"]
pub type Msuacr = crate::RegValueT<Msuacr_SPEC>;

impl Msuacr {
    #[doc = "Startup Area Select"]
    #[inline(always)]
    pub fn sas(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        msuacr::Sas,
        msuacr::Sas,
        Msuacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            msuacr::Sas,
            msuacr::Sas,
            Msuacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Msuacr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Msuacr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Msuacr {
    #[inline(always)]
    fn default() -> Msuacr {
        <crate::RegValueT<Msuacr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msuacr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sas_SPEC;
    pub type Sas = crate::EnumBitfieldStruct<u8, Sas_SPEC>;
    impl Sas {
        #[doc = "Startup area is selected by BTFLG bit"]
        pub const _0_X: Self = Self::new(0);

        #[doc = "Startup area is temporarily switched to the default area"]
        pub const _10: Self = Self::new(2);

        #[doc = "Startup area is temporarily switched to the alternate area"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrpsc_SPEC;
impl crate::sealed::RegSpec for Mrpsc_SPEC {
    type DataType = u8;
}

#[doc = "MRAM Program Speed Control Register"]
pub type Mrpsc = crate::RegValueT<Mrpsc_SPEC>;

impl Mrpsc {
    #[doc = "MRAM High-Speed Program Mode Enable"]
    #[inline(always)]
    pub fn mhspen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrpsc::Mhspen,
        mrpsc::Mhspen,
        Mrpsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrpsc::Mhspen,
            mrpsc::Mhspen,
            Mrpsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrpsc {
    #[inline(always)]
    fn default() -> Mrpsc {
        <crate::RegValueT<Mrpsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrpsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mhspen_SPEC;
    pub type Mhspen = crate::EnumBitfieldStruct<u8, Mhspen_SPEC>;
    impl Mhspen {
        #[doc = "Normal program mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "High-speed program mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcpc0_SPEC;
impl crate::sealed::RegSpec for Mrcpc0_SPEC {
    type DataType = u16;
}

#[doc = "Code MRAM Program Control Register"]
pub type Mrcpc0 = crate::RegValueT<Mrcpc0_SPEC>;

impl Mrcpc0 {
    #[doc = "Code MRAM Program Enable for Non-secure"]
    #[inline(always)]
    pub fn mrcpnen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcpc0::Mrcpnen,
        mrcpc0::Mrcpnen,
        Mrcpc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcpc0::Mrcpnen,
            mrcpc0::Mrcpnen,
            Mrcpc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrcpc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrcpc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcpc0 {
    #[inline(always)]
    fn default() -> Mrcpc0 {
        <crate::RegValueT<Mrcpc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcpc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcpnen_SPEC;
    pub type Mrcpnen = crate::EnumBitfieldStruct<u8, Mrcpnen_SPEC>;
    impl Mrcpnen {
        #[doc = "Code MRAM non-secure aliases are not programmable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Code MRAM non-secure aliases are programmable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcpc1_SPEC;
impl crate::sealed::RegSpec for Mrcpc1_SPEC {
    type DataType = u16;
}

#[doc = "Code MRAM Program Control Register for Secure"]
pub type Mrcpc1 = crate::RegValueT<Mrcpc1_SPEC>;

impl Mrcpc1 {
    #[doc = "Code MRAM Program Enable for Secure"]
    #[inline(always)]
    pub fn mrcpsen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcpc1::Mrcpsen,
        mrcpc1::Mrcpsen,
        Mrcpc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcpc1::Mrcpsen,
            mrcpc1::Mrcpsen,
            Mrcpc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrcpc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrcpc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcpc1 {
    #[inline(always)]
    fn default() -> Mrcpc1 {
        <crate::RegValueT<Mrcpc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcpc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcpsen_SPEC;
    pub type Mrcpsen = crate::EnumBitfieldStruct<u8, Mrcpsen_SPEC>;
    impl Mrcpsen {
        #[doc = "Code MRAM secure aliases are not programmable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Code MRAM secure aliases are programmable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcbprot0_SPEC;
impl crate::sealed::RegSpec for Mrcbprot0_SPEC {
    type DataType = u16;
}

#[doc = "Code MRAM Block Protection Register"]
pub type Mrcbprot0 = crate::RegValueT<Mrcbprot0_SPEC>;

impl Mrcbprot0 {
    #[doc = "Code MRAM Block Protection Cancel for Non-secure"]
    #[inline(always)]
    pub fn bpcn0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcbprot0::Bpcn0,
        mrcbprot0::Bpcn0,
        Mrcbprot0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcbprot0::Bpcn0,
            mrcbprot0::Bpcn0,
            Mrcbprot0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrcbprot0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrcbprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcbprot0 {
    #[inline(always)]
    fn default() -> Mrcbprot0 {
        <crate::RegValueT<Mrcbprot0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcbprot0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn0_SPEC;
    pub type Bpcn0 = crate::EnumBitfieldStruct<u8, Bpcn0_SPEC>;
    impl Bpcn0 {
        #[doc = "Code MRAM block protection is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Code MRAM block protection is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcbprot1_SPEC;
impl crate::sealed::RegSpec for Mrcbprot1_SPEC {
    type DataType = u16;
}

#[doc = "Code MRAM Block Protection Register for Secure"]
pub type Mrcbprot1 = crate::RegValueT<Mrcbprot1_SPEC>;

impl Mrcbprot1 {
    #[doc = "Code MRAM Block Protection Cancel for Secure"]
    #[inline(always)]
    pub fn bpcn1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcbprot1::Bpcn1,
        mrcbprot1::Bpcn1,
        Mrcbprot1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcbprot1::Bpcn1,
            mrcbprot1::Bpcn1,
            Mrcbprot1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrcbprot1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrcbprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcbprot1 {
    #[inline(always)]
    fn default() -> Mrcbprot1 {
        <crate::RegValueT<Mrcbprot1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcbprot1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn1_SPEC;
    pub type Bpcn1 = crate::EnumBitfieldStruct<u8, Bpcn1_SPEC>;
    impl Bpcn1 {
        #[doc = "Code MRAM block protection is enabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Code MRAM block protection is disabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcps_SPEC;
impl crate::sealed::RegSpec for Mrcps_SPEC {
    type DataType = u8;
}

#[doc = "Code MRAM Program Status Register"]
pub type Mrcps = crate::RegValueT<Mrcps_SPEC>;

impl Mrcps {
    #[doc = "Programming Error"]
    #[inline(always)]
    pub fn prgerrc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcps::Prgerrc,
        mrcps::Prgerrc,
        Mrcps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcps::Prgerrc,
            mrcps::Prgerrc,
            Mrcps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECC Error"]
    #[inline(always)]
    pub fn eccerrc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mrcps::Eccerrc,
        mrcps::Eccerrc,
        Mrcps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mrcps::Eccerrc,
            mrcps::Eccerrc,
            Mrcps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Address Buffer Empty"]
    #[inline(always)]
    pub fn abufemp(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mrcps::Abufemp,
        mrcps::Abufemp,
        Mrcps_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mrcps::Abufemp,
            mrcps::Abufemp,
            Mrcps_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Address Buffer Full"]
    #[inline(always)]
    pub fn abuffull(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mrcps::Abuffull,
        mrcps::Abuffull,
        Mrcps_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mrcps::Abuffull,
            mrcps::Abuffull,
            Mrcps_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Code MRAM Program Busy"]
    #[inline(always)]
    pub fn prgbsyc(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mrcps::Prgbsyc,
        mrcps::Prgbsyc,
        Mrcps_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mrcps::Prgbsyc,
            mrcps::Prgbsyc,
            Mrcps_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrcps {
    #[inline(always)]
    fn default() -> Mrcps {
        <crate::RegValueT<Mrcps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcps {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerrc_SPEC;
    pub type Prgerrc = crate::EnumBitfieldStruct<u8, Prgerrc_SPEC>;
    impl Prgerrc {
        #[doc = "Programming has been completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during programming"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccerrc_SPEC;
    pub type Eccerrc = crate::EnumBitfieldStruct<u8, Eccerrc_SPEC>;
    impl Eccerrc {
        #[doc = "Programming has been completed successfully"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error has occurred during programming"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abufemp_SPEC;
    pub type Abufemp = crate::EnumBitfieldStruct<u8, Abufemp_SPEC>;
    impl Abufemp {
        #[doc = "Address buffer is not empty, and code MRAM write data can flush"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address buffer is empty, and code MRAM write data cannot flush"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abuffull_SPEC;
    pub type Abuffull = crate::EnumBitfieldStruct<u8, Abuffull_SPEC>;
    impl Abuffull {
        #[doc = "Address buffer is not full, and code MRAM write transaction is acceptable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Address buffer is full, and code MRAM write transaction is not acceptable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgbsyc_SPEC;
    pub type Prgbsyc = crate::EnumBitfieldStruct<u8, Prgbsyc_SPEC>;
    impl Prgbsyc {
        #[doc = "Code MRAM is not in program status"]
        pub const _0: Self = Self::new(0);

        #[doc = "Code MRAM is in program status"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcpaeint_SPEC;
impl crate::sealed::RegSpec for Mrcpaeint_SPEC {
    type DataType = u8;
}

#[doc = "Code MRAM Program Access Error Interrupt Enable Register"]
pub type Mrcpaeint = crate::RegValueT<Mrcpaeint_SPEC>;

impl Mrcpaeint {
    #[doc = "Code MRAM Program Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn mrcaeie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mrcpaeint::Mrcaeie,
        mrcpaeint::Mrcaeie,
        Mrcpaeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mrcpaeint::Mrcaeie,
            mrcpaeint::Mrcaeie,
            Mrcpaeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mrcpaeint {
    #[inline(always)]
    fn default() -> Mrcpaeint {
        <crate::RegValueT<Mrcpaeint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcpaeint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcaeie_SPEC;
    pub type Mrcaeie = crate::EnumBitfieldStruct<u8, Mrcaeie_SPEC>;
    impl Mrcaeie {
        #[doc = "Generation of an MRAM_MRCPR interrupt request is disabled when the ECCERRC or PRGERRC bits are 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generation of an MRAM_MRCPR interrupt request is enabled when the ECCERRC or PRGERRC bits are 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcpea_SPEC;
impl crate::sealed::RegSpec for Mrcpea_SPEC {
    type DataType = u32;
}

#[doc = "Code MRAM Program Error Address Register"]
pub type Mrcpea = crate::RegValueT<Mrcpea_SPEC>;

impl Mrcpea {
    #[doc = "Code MRAM Program Error Address"]
    #[inline(always)]
    pub fn mcpea(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Mrcpea_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,Mrcpea_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcpea {
    #[inline(always)]
    fn default() -> Mrcpea {
        <crate::RegValueT<Mrcpea_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcflr_SPEC;
impl crate::sealed::RegSpec for Mrcflr_SPEC {
    type DataType = u16;
}

#[doc = "Code MRAM Flush Register"]
pub type Mrcflr = crate::RegValueT<Mrcflr_SPEC>;

impl Mrcflr {
    #[doc = "Flush Write Data Buffer for Code MRAM"]
    #[inline(always)]
    pub fn mrcfl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrcflr::Mrcfl,
        mrcflr::Mrcfl,
        Mrcflr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrcflr::Mrcfl,
            mrcflr::Mrcfl,
            Mrcflr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrcflr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrcflr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrcflr {
    #[inline(always)]
    fn default() -> Mrcflr {
        <crate::RegValueT<Mrcflr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrcflr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrcfl_SPEC;
    pub type Mrcfl = crate::EnumBitfieldStruct<u8, Mrcfl_SPEC>;
    impl Mrcfl {
        #[doc = "Not execute to flush program data buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Execute to flush program data buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrceecc_SPEC;
impl crate::sealed::RegSpec for Mrceecc_SPEC {
    type DataType = u16;
}

#[doc = "Code MRAM ECC Encoder Control Register"]
pub type Mrceecc = crate::RegValueT<Mrceecc_SPEC>;

impl Mrceecc {
    #[doc = "Code MRAM ECC Encoder Outputs Bypass Enable"]
    #[inline(always)]
    pub fn eccbypc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mrceecc::Eccbypc,
        mrceecc::Eccbypc,
        Mrceecc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mrceecc::Eccbypc,
            mrceecc::Eccbypc,
            Mrceecc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mrceecc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mrceecc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mrceecc {
    #[inline(always)]
    fn default() -> Mrceecc {
        <crate::RegValueT<Mrceecc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mrceecc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccbypc_SPEC;
    pub type Eccbypc = crate::EnumBitfieldStruct<u8, Eccbypc_SPEC>;
    impl Eccbypc {
        #[doc = "ECC encoder output is programmed to the ECC bits of the code MRAM"]
        pub const _0: Self = Self::new(0);

        #[doc = "Bits \\[144:128\\] and bits \\[16:0\\] of written data are programmed to the ECC bits of the code MRAM"]
        pub const _1: Self = Self::new(1);
    }
}
